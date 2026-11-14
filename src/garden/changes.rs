//! Visibility-scoped immutable history feed with bounded snapshot pages.
use super::*;
use crate::garden::contracts::{Change, ChangesRequest, ChangesResponse};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Position {
    sequence: u64,
    id: String,
    checkpoint: bool,
}

#[derive(Debug)]
struct Head {
    reference: ExternalReference,
    sequence: u64,
}

impl GardenStore {
    /// Read immutable committed changes. A completed page returns a checkpoint
    /// cursor that can poll future commits until its original lease expires.
    /// # Errors
    /// Fails closed on stale authority, generation, cursor or lease expiry.
    pub fn changes(&self, auth: &Auth, request: &ChangesRequest) -> Result<ChangesResponse> {
        GardenRequest::parse(
            GardenTool::Changes,
            &serde_json::to_value(request).map_err(storage_error)?,
        )?;
        // Authorize before clock bookkeeping, then recheck in the serving snapshot.
        {
            let tx = self.db.begin_read().map_err(storage_error)?;
            principal_read(&tx, auth, GardenAction::Read)?;
        }
        pager::observe_clock(self)?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let principal = principal_read(&tx, auth, GardenAction::Read)?;
        if request.cursor.is_some()
            && (request.generation != state.generation
                || request.workspace_id != state.workspace_id)
        {
            return Err(pager::restart());
        }
        envelope(&state, &request.workspace_id, &request.generation)?;
        let digest = contracts::hash(&format!("changes:{}", request.limit));
        let binding = pager::Binding {
            state: &state,
            principal: &principal,
            auth,
            kind: "changes",
            digest: &digest,
        };
        let mut lease = if let Some(cursor) = &request.cursor {
            pager::load(self, &tx, &binding, cursor)?
        } else {
            pager::Lease::new(
                self,
                &binding,
                json!({"sequence":0,"id":"","checkpoint":false}),
                state.sequence,
            )?
        };
        let mut position: Position =
            serde_json::from_value(lease.payload.clone()).map_err(storage_error)?;
        if position.checkpoint {
            lease.cutoff = state.sequence;
            position.checkpoint = false;
        }
        let limit = request.limit as usize;
        let mut changes = Vec::new();
        let mut scan_truncated = false;
        let mut reads = 100_000_usize;
        match &principal.garden {
            GardenAccess::Allowlist { objects } => {
                // Enumerate only owner-authorized IDs. Hidden global events do not
                // consume page work, alter coverage or advance disclosed positions.
                let mut heads = Vec::new();
                let mut work = 0_usize;
                for reference in objects.iter().collect::<BTreeSet<_>>() {
                    if let Some(change) = next_authorized(
                        &tx,
                        &principal,
                        reference,
                        (position.sequence, &position.id),
                        lease.cutoff,
                        &mut work,
                        &mut reads,
                    )? {
                        heads.push(change);
                    }
                }
                while changes.len() <= limit && !heads.is_empty() {
                    let index = heads
                        .iter()
                        .enumerate()
                        .min_by(|(_, a), (_, b)| {
                            (a.sequence, &a.reference.id).cmp(&(b.sequence, &b.reference.id))
                        })
                        .map(|(index, _)| index)
                        .ok_or_else(work_limit)?;
                    let next = heads.swap_remove(index);
                    if changes.len() < limit
                        && let Some(change) = next_authorized(
                            &tx,
                            &principal,
                            &next.reference,
                            (next.sequence, &next.reference.id),
                            lease.cutoff,
                            &mut work,
                            &mut reads,
                        )?
                    {
                        heads.push(change);
                    }
                    let record = history::at_sequence(&tx, &next.reference.id, next.sequence)?
                        .ok_or_else(|| storage_error("missing feed head revision"))?;
                    let change =
                        disclose(&tx, &principal, record, &mut reads)?.ok_or_else(|| {
                            storage_error("feed head visibility changed within snapshot")
                        })?;
                    changes.push(change);
                }
            }
            GardenAccess::All {} => {
                let mut after_sequence = position.sequence;
                let mut after_id = position.id.clone();
                let mut work = 0;
                loop {
                    let events =
                        indexes::events(&tx, after_sequence, &after_id, lease.cutoff, 128)?;
                    if events.is_empty() {
                        break;
                    }
                    for (sequence, id, _) in events {
                        after_sequence = sequence;
                        after_id = id.clone();
                        work += 1;
                        let record = history::at_sequence(&tx, &id, sequence)?
                            .ok_or_else(|| storage_error("missing event revision"))?;
                        if let Some(change) = disclose(&tx, &principal, record, &mut reads)? {
                            changes.push(change);
                        }
                        if changes.len() > limit || work >= 10_000 {
                            break;
                        }
                    }
                    if changes.len() > limit {
                        break;
                    }
                    if work >= 10_000 {
                        scan_truncated = true;
                        position.sequence = after_sequence;
                        position.id = after_id;
                        break;
                    }
                }
            }
        }
        let mut more = changes.len() > limit || scan_truncated;
        changes.truncate(limit);
        // Leave room for envelope, coverage and opaque cursor metadata. A byte
        // boundary continues after the last returned event, never the last read.
        let mut bytes = 0_usize;
        let mut byte_boundary = None;
        let mut payload_omitted = false;
        for (index, change) in changes.iter_mut().enumerate() {
            let mut size = encode(change)?.len();
            // This wrapper has the same record nesting as the complete wire
            // response, including the array and change object above the record.
            let exceeds_envelope = match bounded_json(&json!({"changes":[&*change]})) {
                Ok(()) => false,
                Err(error) if error.code == GardenErrorCode::LimitExceeded => true,
                Err(error) => return Err(error),
            };
            let oversized = size > MAX_JSON_BYTES - 4096 || exceeds_envelope;
            if oversized {
                // Records accepted by the canonical writer can exceed the feed's
                // envelope reserve. Preserve the event so consumers can advance
                // and fetch its payload separately through exact get.
                change.record = None;
                size = encode(change)?.len();
            }
            bytes += size;
            if bytes > MAX_JSON_BYTES - 4096 {
                byte_boundary = Some(index);
                break;
            }
            payload_omitted |= oversized;
        }
        if let Some(index) = byte_boundary {
            changes.truncate(index);
            more = true;
        }
        if more {
            if (!scan_truncated || byte_boundary.is_some())
                && let Some(last) = changes.last()
            {
                position.sequence = last.sequence;
                position.id = last.reference.id.clone();
            }
        } else {
            position.sequence = lease.cutoff;
            position.id = "\u{10ffff}".into();
            position.checkpoint = true;
        }
        lease.payload = serde_json::to_value(position).map_err(storage_error)?;
        let cursor = pager::save(self, auth, &lease)?;
        let response = ChangesResponse {
            version: 2,
            workspace_id: state.workspace_id,
            generation: state.generation,
            changes,
            coverage: Coverage {
                complete: !more && !payload_omitted,
                truncated: more || payload_omitted,
                reason: (more || payload_omitted).then(|| {
                    if payload_omitted {
                        "event_payload_omitted".into()
                    } else if byte_boundary.is_some() {
                        "response_byte_limit".into()
                    } else {
                        "page_limit".into()
                    }
                }),
                canonical_watermark: lease.cutoff,
                index_watermark: lease.cutoff,
                search_mode: SearchMode::Exact,
                model: None,
                profile: None,
            },
            page: lease.page(Some(cursor)),
        };
        pager::revalidate_at(self, auth, &lease, state.sequence)?;
        GardenResponse::parse(
            GardenTool::Changes,
            &serde_json::to_value(&response).map_err(storage_error)?,
        )?;
        Ok(response)
    }
}

fn disclose(
    tx: &redb::ReadTransaction,
    principal: &Principal,
    record: GardenRecord,
    reads: &mut usize,
) -> Result<Option<Change>> {
    let Some(authorized) = query::authorized_with_budget(
        tx,
        principal,
        &record.metadata.id,
        record.metadata.commit_sequence,
        false,
        reads,
    )?
    else {
        return Ok(None);
    };
    if authorized != record {
        return Err(storage_error("event revision mismatch"));
    }
    // Historical active payloads must not revive currently inactive content.
    if record.metadata.lifecycle == Lifecycle::Active
        && query::authorized_with_budget(
            tx,
            principal,
            &record.metadata.id,
            record.metadata.commit_sequence,
            true,
            reads,
        )?
        .is_none()
    {
        return Ok(None);
    }
    Ok(Some(Change {
        reference: ExternalReference {
            kind: record.draft.kind(),
            id: record.metadata.id.clone(),
        },
        revision: record.metadata.revision,
        sequence: record.metadata.commit_sequence,
        lifecycle: record.metadata.lifecycle,
        record: (record.metadata.lifecycle == Lifecycle::Active).then_some(record),
    }))
}

// One authorized head per allowlisted ID bounds retained memory. Hidden history
// never consumes successful-page budgets. A hard resource ceiling fails with a
// generic error instead of exposing hidden-dependent successful coverage/counts.
fn next_authorized(
    tx: &redb::ReadTransaction,
    principal: &Principal,
    reference: &ExternalReference,
    after: (u64, &str),
    cutoff: u64,
    work: &mut usize,
    reads: &mut usize,
) -> Result<Option<Head>> {
    let table = tx.open_table(history::HISTORY).map_err(storage_error)?;
    for entry in table
        .range((reference.id.as_str(), after.0)..=(reference.id.as_str(), cutoff))
        .map_err(storage_error)?
    {
        let (_, bytes) = entry.map_err(storage_error)?;
        let record: GardenRecord = decode(bytes.value())?;
        if record.metadata.commit_sequence == after.0 && record.metadata.id.as_str() <= after.1 {
            continue;
        }
        *work += 1;
        if *work > 100_000 {
            return Err(work_limit());
        }
        if record.draft.kind() != reference.kind {
            continue;
        }
        if let Some(change) = disclose(tx, principal, record, reads)? {
            return Ok(Some(Head {
                reference: change.reference,
                sequence: change.sequence,
            }));
        }
    }
    Ok(None)
}
