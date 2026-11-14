//! Canonical, permission-first indexed candidates and bounded directed traversal.
use super::pager::Lease;
use super::*;
use crate::garden::indexes::CandidateFilter;
use redb::ReadTransaction;
use std::collections::VecDeque;

const MAX_CANDIDATES: usize = 1024;
const DEFAULT_WORK: usize = 4096;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Candidate {
    reference: ExternalReference,
    revision: u64,
    scores: ComponentScores,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QueryPage {
    candidates: Vec<Candidate>,
    position: usize,
    coverage: Coverage,
}

/// Read an immutable revision after checking its entire current and historic footprint.
/// Hidden footprints are rejected before they consume public work/edge budgets.
pub(super) fn authorized_with_budget(
    tx: &ReadTransaction,
    principal: &Principal,
    id: &str,
    cutoff: u64,
    active_now: bool,
    reads: &mut usize,
) -> Result<Option<GardenRecord>> {
    charge(reads)?;
    let Some(record) = history::at_sequence(tx, id, cutoff)? else {
        return Ok(None);
    };
    let mut pending = vec![ExternalReference {
        kind: record.draft.kind(),
        id: id.into(),
    }];
    let mut visited = BTreeSet::new();
    let current = tx.open_table(CURRENT).map_err(storage_error)?;
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    while let Some(reference) = pending.pop() {
        if !principal.garden.allows(&reference) {
            return Ok(None);
        }
        if !visited.insert(reference.clone()) {
            continue;
        }
        if visited.len() > MAX_TRANSACTION_OBJECTS {
            return Err(work_limit());
        }
        charge(reads)?;
        let Some(now) = current.get(reference.id.as_str()).map_err(storage_error)? else {
            return Ok(None);
        };
        let now: GardenRecord = decode(now.value())?;
        if now.draft.kind() != reference.kind
            || active_now && now.metadata.lifecycle != Lifecycle::Active
        {
            return Ok(None);
        }
        let current_footprint = footprint(&now);
        if principal
            .garden
            .require_visible(&[current_footprint])
            .is_err()
        {
            return Ok(None);
        }
        pending.extend(
            now.draft
                .references()
                .into_iter()
                .filter_map(existing_reference),
        );
        charge(reads)?;
        if let Some(old) = history::at_sequence(tx, &reference.id, cutoff)? {
            if old.draft.kind() != reference.kind
                || principal
                    .garden
                    .require_visible(&[footprint(&old)])
                    .is_err()
            {
                return Ok(None);
            }
            pending.extend(
                old.draft
                    .references()
                    .into_iter()
                    .filter_map(existing_reference),
            );
        }
        if let Some(target) = replacements
            .get(reference.id.as_str())
            .map_err(storage_error)?
        {
            pending.push(ExternalReference {
                kind: reference.kind,
                id: target.value().into(),
            });
        }
        if pending.len() + visited.len() > MAX_TRANSACTION_OBJECTS {
            return Err(work_limit());
        }
    }
    Ok(Some(record))
}
fn charge(reads: &mut usize) -> Result<()> {
    *reads = reads.checked_sub(1).ok_or_else(|| {
        failure(
            GardenErrorCode::LimitExceeded,
            "Query authorization policy work limit exceeded",
        )
    })?;
    Ok(())
}
fn existing_reference(reference: &RecordRef) -> Option<ExternalReference> {
    match reference {
        RecordRef::Existing { kind, id } => Some(ExternalReference {
            kind: *kind,
            id: id.clone(),
        }),
        RecordRef::Item { .. } => None,
    }
}

impl GardenStore {
    /// Query bounded canonical candidates with a stable, current-authority-checked snapshot.
    /// # Errors
    /// Rejects invalid grants, inputs, expired or mismatched cursors, and unavailable storage.
    pub fn query(&self, auth: &Auth, request: &QueryRequest) -> Result<QueryResponse> {
        GardenRequest::parse(
            GardenTool::Query,
            &serde_json::to_value(request).map_err(storage_error)?,
        )?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let principal = principal_read(&tx, auth, GardenAction::Read)?;
        if request.cursor.is_some()
            && (state.workspace_id != request.workspace_id
                || state.generation != request.generation)
        {
            return Err(pager::restart());
        }
        envelope(&state, &request.workspace_id, &request.generation)?;
        pager::observe_clock(self)?;
        let mut normalized = request.clone();
        normalized.cursor = None;
        normalized.ids.sort();
        normalized.ids.dedup();
        normalized.kinds.sort();
        normalized.kinds.dedup();
        let digest = contracts::hash(&canonical_json(&normalized)?);
        let binding = pager::Binding {
            state: &state,
            principal: &principal,
            auth,
            kind: "query",
            digest: &digest,
        };
        let mut lease = if let Some(token) = &request.cursor {
            pager::load(self, &tx, &binding, token)?
        } else {
            let cutoff = if let Some(known) = &request.known_at {
                history::sequence_at(&tx, known)?
            } else {
                state.sequence
            };
            let page = materialize(&tx, &principal, request, cutoff, state.sequence)?;
            Lease::new(
                self,
                &binding,
                serde_json::to_value(page).map_err(storage_error)?,
                cutoff,
            )?
        };
        let mut page: QueryPage =
            serde_json::from_value(lease.payload.clone()).map_err(storage_error)?;
        let mut reads = 100_000;
        let mut response = QueryResponse {
            version: 2,
            workspace_id: state.workspace_id.clone(),
            generation: state.generation.clone(),
            hits: Vec::new(),
            coverage: page.coverage.clone(),
            page: lease.page(Some("0".repeat(64))),
        };
        while page.position < page.candidates.len() && response.hits.len() < request.limit as usize
        {
            let candidate = &page.candidates[page.position];
            let Some(record) = authorized_with_budget(
                &tx,
                &principal,
                &candidate.reference.id,
                lease.cutoff,
                request.known_at.is_none(),
                &mut reads,
            )?
            else {
                page.position += 1;
                continue;
            };
            if record.metadata.revision != candidate.revision
                || record.draft.kind() != candidate.reference.kind
                || record.metadata.lifecycle != Lifecycle::Active
            {
                page.position += 1;
                continue;
            }
            let sources = match &record.draft {
                GardenDraft::Assertion { evidence, .. } => {
                    evidence.iter().filter_map(existing_reference).collect()
                }
                _ => Vec::new(),
            };
            response.hits.push(QueryHit {
                record,
                sources,
                scores: candidate.scores.clone(),
            });
            if bounded_json(&serde_json::to_value(&response).map_err(storage_error)?).is_err() {
                response.hits.pop();
                response.coverage.complete = false;
                response.coverage.truncated = true;
                response.coverage.reason = Some("response_byte_limit".into());
                if response.hits.is_empty() {
                    return Err(failure(
                        GardenErrorCode::LimitExceeded,
                        "A query hit exceeds the response byte limit",
                    ));
                }
                break;
            }
            page.position += 1;
        }
        response.page = lease.page(None);
        if page.position < page.candidates.len() {
            lease.payload = serde_json::to_value(page).map_err(storage_error)?;
            response.page = lease.page(Some(pager::save(self, auth, &lease)?));
        }
        GardenResponse::parse(
            GardenTool::Query,
            &serde_json::to_value(&response).map_err(storage_error)?,
        )?;
        pager::revalidate_at(self, auth, &lease, state.sequence)?;
        Ok(response)
    }
}

fn matches_filters(record: &GardenRecord, request: &QueryRequest) -> Result<bool> {
    if record.metadata.lifecycle != Lifecycle::Active
        || !request.ids.is_empty()
            && !request.ids.iter().any(|reference| {
                reference.id == record.metadata.id && reference.kind == record.draft.kind()
            })
        || !request.kinds.is_empty() && !request.kinds.contains(&record.draft.kind())
    {
        return Ok(false);
    }
    if let Some(entity_type) = &request.entity_type
        && !matches!(&record.draft,GardenDraft::Entity{entity_type:value,..} if value==entity_type)
    {
        return Ok(false);
    }
    if let Some(predicate) = &request.predicate
        && !(request.traversal.is_some() && record.draft.kind() == GardenKind::Entity
            || matches!(&record.draft,GardenDraft::Assertion{predicate:value,..} if value==predicate))
    {
        return Ok(false);
    }
    if let Some(property) = &request.property {
        let extensions = match &record.draft {
            GardenDraft::Entity { extensions, .. }
            | GardenDraft::Assertion { extensions, .. }
            | GardenDraft::Evidence { extensions, .. } => extensions,
        };
        let extension_match = extensions.get(&property.name).is_some_and(|extension| {
            serde_json::to_value(&property.value).is_ok_and(|value| value == extension.payload)
        });
        let assertion_match = matches!(&record.draft,GardenDraft::Assertion{predicate,object:AssertionObject::Value{value},..} if predicate==&property.name && value==&property.value);
        if !extension_match && !assertion_match {
            return Ok(false);
        }
    }
    if let Some(at) = &request.valid_at
        && !(request.traversal.is_some() && record.draft.kind() == GardenKind::Entity
            || history::valid_at(record, at)?)
    {
        return Ok(false);
    }
    Ok(true)
}

fn driver(request: &QueryRequest) -> CandidateFilter {
    if let Some(property) = &request.property {
        return CandidateFilter::Property {
            namespace: property.name.clone(),
            value: property.value.clone(),
        };
    }
    if let Some(entity_type) = &request.entity_type {
        return CandidateFilter::EntityType(entity_type.clone());
    }
    if let Some(predicate) = &request.predicate {
        return CandidateFilter::Predicate(predicate.clone());
    }
    if let Some(text) = &request.text {
        return CandidateFilter::Literal(text.clone());
    }
    if let Some(at) = &request.valid_at {
        return CandidateFilter::ValidAt(at.clone());
    }
    if request.kinds.len() == 1 {
        return CandidateFilter::Kind(request.kinds[0]);
    }
    CandidateFilter::All
}

fn materialize(
    tx: &ReadTransaction,
    principal: &Principal,
    request: &QueryRequest,
    cutoff: u64,
    canonical: u64,
) -> Result<QueryPage> {
    let search = search::coverage(tx, request, principal)?;
    let mut coverage = Coverage {
        complete: search.reason.is_none(),
        truncated: false,
        reason: search.reason,
        canonical_watermark: canonical,
        index_watermark: if request.vector.is_none() {
            canonical
        } else {
            search.index_watermark.min(canonical)
        },
        search_mode: search.mode,
        model: search.model,
        profile: search.profile,
    };
    let work_budget = request
        .traversal
        .as_ref()
        .map_or(DEFAULT_WORK, |traversal| traversal.work_budget as usize);
    let mut work = 0;
    let mut reads = 100_000;
    let mut candidates = BTreeMap::new();
    let mut add = |record: GardenRecord, coverage: &mut Coverage| -> Result<()> {
        if !matches_filters(&record, request)? {
            return Ok(());
        }
        if search::vector_available_for(tx, &record, request)?
            && let Some(vector) = &request.vector
        {
            coverage.search_mode = if request.text.is_some() {
                SearchMode::Hybrid
            } else {
                SearchMode::Vector
            };
            coverage.model = Some(vector.model.clone());
            coverage.profile = Some(vector.profile.clone());
            coverage.complete = false;
            coverage.reason = Some("vector_index_partial".into());
            coverage.index_watermark = 0;
        }
        if let Some(scores) = search::score(tx, &record, request)? {
            if candidates.len() >= MAX_CANDIDATES && !candidates.contains_key(&record.metadata.id) {
                truncate(coverage, "snapshot_candidate_limit");
                return Ok(());
            }
            candidates.insert(
                record.metadata.id.clone(),
                Candidate {
                    reference: ExternalReference {
                        kind: record.draft.kind(),
                        id: record.metadata.id.clone(),
                    },
                    revision: record.metadata.revision,
                    scores,
                },
            );
        }
        Ok(())
    };
    if let Some(traversal) = &request.traversal {
        // Restricted enumeration starts from the owner allowlist once. Build a
        // private adjacency map so each frontier node does not rescan that list.
        let restricted = restricted_adjacency(
            tx,
            principal,
            cutoff,
            request.known_at.is_none(),
            &mut reads,
        )?;
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        let mut edges = BTreeSet::new();
        for start in &traversal.start {
            if !principal.garden.allows(start) {
                continue;
            }
            if let Some(record) = authorized_with_budget(
                tx,
                principal,
                &start.id,
                cutoff,
                request.known_at.is_none(),
                &mut reads,
            )? {
                if record.draft.kind() != start.kind
                    || record.metadata.lifecycle != Lifecycle::Active
                {
                    continue;
                }
                queue.push_back((start.clone(), 0));
                visited.insert(start.clone());
            }
        }
        while let Some((node, hops)) = queue.pop_front() {
            let (ids, index_truncated) = if let Some((outgoing, incoming)) = &restricted {
                let mut ids = BTreeSet::new();
                if traversal.direction != Direction::Incoming {
                    ids.extend(outgoing.get(&node).into_iter().flatten().cloned());
                }
                if traversal.direction != Direction::Outgoing {
                    ids.extend(incoming.get(&node).into_iter().flatten().cloned());
                }
                (ids.into_iter().collect(), false)
            } else {
                adjacent_ids(tx, &node, traversal.direction, cutoff, work_budget + 1)?
            };
            if index_truncated {
                truncate(&mut coverage, "index_work_limit")
            }
            for id in ids {
                let Some(record) = authorized_with_budget(
                    tx,
                    principal,
                    &id,
                    cutoff,
                    request.known_at.is_none(),
                    &mut reads,
                )?
                else {
                    continue;
                };
                let Some(neighbors) = neighbors(&record, &node, traversal.direction) else {
                    continue;
                };
                if request.predicate.as_ref().is_some_and(|filter|!matches!(&record.draft,GardenDraft::Assertion{predicate,..} if predicate==filter)) {
                    continue;
                }
                if let Some(at) = &request.valid_at
                    && !history::valid_at(&record, at)?
                {
                    continue;
                }
                if record.metadata.lifecycle != Lifecycle::Active || !edges.insert(id) {
                    continue;
                }
                if hops >= traversal.max_hops {
                    truncate(&mut coverage, "hop_limit");
                    continue;
                }
                if edges.len() > traversal.max_edges as usize {
                    truncate(&mut coverage, "edge_limit");
                    break;
                }
                if work >= work_budget {
                    truncate(&mut coverage, "work_limit");
                    break;
                }
                work += 1;
                add(record, &mut coverage)?;
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        if work >= work_budget {
                            truncate(&mut coverage, "work_limit");
                            break;
                        }
                        if let Some(record) = authorized_with_budget(
                            tx,
                            principal,
                            &neighbor.id,
                            cutoff,
                            request.known_at.is_none(),
                            &mut reads,
                        )? {
                            work += 1;
                            add(record, &mut coverage)?;
                            queue.push_back((neighbor, hops + 1));
                        }
                    }
                }
            }
            if work >= work_budget || edges.len() > traversal.max_edges as usize {
                if work >= work_budget && !queue.is_empty() {
                    truncate(&mut coverage, "work_limit")
                }
                break;
            }
        }
    } else {
        let (ids, index_truncated) =
            candidate_ids(tx, principal, request, cutoff, work_budget + 1)?;
        if index_truncated {
            truncate(&mut coverage, "index_work_limit")
        }
        for id in ids {
            let Some(record) = authorized_with_budget(
                tx,
                principal,
                &id,
                cutoff,
                request.known_at.is_none(),
                &mut reads,
            )?
            else {
                continue;
            };
            if work >= work_budget {
                truncate(&mut coverage, "work_limit");
                break;
            }
            work += 1;
            add(record, &mut coverage)?;
        }
    }
    Ok(QueryPage {
        candidates: candidates.into_values().collect(),
        position: 0,
        coverage,
    })
}
fn truncate(coverage: &mut Coverage, reason: &str) {
    coverage.complete = false;
    coverage.truncated = true;
    coverage.reason = Some(reason.into());
}

fn candidate_ids(
    tx: &ReadTransaction,
    principal: &Principal,
    request: &QueryRequest,
    cutoff: u64,
    limit: usize,
) -> Result<(Vec<String>, bool)> {
    if !request.ids.is_empty() {
        return Ok((
            request
                .ids
                .iter()
                .filter(|reference| principal.garden.allows(reference))
                .map(|reference| reference.id.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            false,
        ));
    }
    if let GardenAccess::Allowlist { objects } = &principal.garden {
        return Ok((
            objects
                .iter()
                .map(|reference| reference.id.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            false,
        ));
    }
    let page = indexes::candidates(tx, &driver(request), cutoff, None, limit)?;
    let mut ids: BTreeSet<_> = page.ids.into_iter().collect();
    let mut truncated = page.truncated;
    if let Some(vector) = &request.vector {
        let (vectors, cut) = search::vector_candidates(tx, vector, limit)?;
        truncated |= cut;
        ids.extend(vectors.into_iter().map(|reference| reference.id));
    }
    Ok((ids.into_iter().collect(), truncated))
}
type Adjacency = BTreeMap<ExternalReference, BTreeSet<String>>;
fn restricted_adjacency(
    tx: &ReadTransaction,
    principal: &Principal,
    cutoff: u64,
    active_now: bool,
    reads: &mut usize,
) -> Result<Option<(Adjacency, Adjacency)>> {
    let GardenAccess::Allowlist { objects } = &principal.garden else {
        return Ok(None);
    };
    let mut outgoing = Adjacency::new();
    let mut incoming = Adjacency::new();
    for reference in objects
        .iter()
        .filter(|reference| reference.kind == GardenKind::Assertion)
        .collect::<BTreeSet<_>>()
    {
        let Some(record) =
            authorized_with_budget(tx, principal, &reference.id, cutoff, active_now, reads)?
        else {
            continue;
        };
        if record.metadata.lifecycle != Lifecycle::Active {
            continue;
        }
        let GardenDraft::Assertion {
            subject, object, ..
        } = &record.draft
        else {
            continue;
        };
        if let Some(subject) = existing_reference(subject) {
            outgoing
                .entry(subject)
                .or_default()
                .insert(reference.id.clone());
        }
        if let AssertionObject::EntityRef { entity_ref } = object
            && let Some(object) = existing_reference(entity_ref)
        {
            incoming
                .entry(object)
                .or_default()
                .insert(reference.id.clone());
        }
    }
    Ok(Some((outgoing, incoming)))
}
fn adjacent_ids(
    tx: &ReadTransaction,
    node: &ExternalReference,
    direction: Direction,
    cutoff: u64,
    limit: usize,
) -> Result<(Vec<String>, bool)> {
    let page = indexes::candidates(
        tx,
        &CandidateFilter::Adjacent {
            reference: node.clone(),
            direction,
        },
        cutoff,
        None,
        limit,
    )?;
    Ok((page.ids, page.truncated))
}
fn neighbors(
    record: &GardenRecord,
    node: &ExternalReference,
    direction: Direction,
) -> Option<Vec<ExternalReference>> {
    let GardenDraft::Assertion {
        subject, object, ..
    } = &record.draft
    else {
        return None;
    };
    let subject = existing_reference(subject)?;
    let object = match object {
        AssertionObject::EntityRef { entity_ref } => existing_reference(entity_ref),
        AssertionObject::Value { .. } => None,
    };
    let mut neighbors = Vec::new();
    let mut matched = false;
    if direction != Direction::Incoming && subject == *node {
        matched = true;
        if let Some(object) = &object {
            neighbors.push(object.clone())
        }
    }
    if direction != Direction::Outgoing && object.as_ref() == Some(node) {
        matched = true;
        neighbors.push(subject)
    }
    matched.then_some(neighbors)
}
