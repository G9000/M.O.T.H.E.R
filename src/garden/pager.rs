//! Bounded private snapshot leases. Cursor contents carry no client-supplied claims.
use super::*;
use chrono::Duration;
use redb::ReadableTableMetadata;
use serde_json::Value;

const LEASES: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_page_leases");
const EXPIRY: TableDefinition<(i64, &str), u8> = TableDefinition::new("garden_page_expiry");
const TOKENS: TableDefinition<&str, &str> = TableDefinition::new("garden_page_tokens");
const SNAPSHOT_DATA: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_page_snapshot_data");
const SNAPSHOT_EXPIRY: TableDefinition<(i64, &str), u8> =
    TableDefinition::new("garden_page_snapshot_expiry");
const MAX_LEASES: u64 = 4096;
const MAX_SNAPSHOTS: u64 = 512;

pub(super) struct Binding<'a> {
    pub state: &'a State,
    pub principal: &'a Principal,
    pub auth: &'a Auth,
    pub kind: &'a str,
    pub digest: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Lease {
    pub snapshot_id: String,
    pub workspace_id: String,
    pub generation: String,
    pub actor: String,
    pub principal_hash: String,
    pub visibility_epoch: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub cutoff: u64,
    pub kind: String,
    pub digest: String,
    pub payload: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    position_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    candidate_digest: Option<String>,
}

pub(super) fn initialize(tx: &redb::WriteTransaction) -> Result<()> {
    tx.open_table(LEASES).map_err(storage_error)?;
    tx.open_table(EXPIRY).map_err(storage_error)?;
    tx.open_table(TOKENS).map_err(storage_error)?;
    tx.open_table(SNAPSHOT_DATA).map_err(storage_error)?;
    tx.open_table(SNAPSHOT_EXPIRY).map_err(storage_error)?;
    tx.open_table(META)
        .map_err(storage_error)?
        .insert("pager_initialized", b"2".as_slice())
        .map_err(storage_error)?;
    Ok(())
}
pub(super) fn needs_initialize(db: &Database) -> Result<bool> {
    let tx = db.begin_read().map_err(storage_error)?;
    Ok(tx
        .open_table(META)
        .map_err(storage_error)?
        .get("pager_initialized")
        .map_err(storage_error)?
        .is_none_or(|version| version.value() != b"2"))
}
pub(super) fn restart() -> GardenError {
    failure(
        GardenErrorCode::RestartRequired,
        "Cursor lease expired or no longer matches this request; restart retrieval",
    )
}
fn principal_hash(principal: &Principal) -> Result<String> {
    Ok(contracts::hash(&canonical_json(principal)?))
}

/// Persist the observed wall-clock high water, including rejected expired requests.
pub(super) fn observe_clock(store: &GardenStore) -> Result<()> {
    let tx = store.db.begin_write().map_err(storage_error)?;
    // Sample inside the serialized writer gate. An earlier observation waiting
    // behind another query is not evidence that the owner's clock moved back.
    let now = store.clock.now();
    {
        let mut meta = tx.open_table(META).map_err(storage_error)?;
        if let Some(previous) = meta.get("pager_clock").map_err(storage_error)? {
            let previous: DateTime<Utc> = decode(previous.value())?;
            if now < previous {
                return Err(restart());
            }
        }
        meta.insert("pager_clock", encode(&now)?.as_slice())
            .map_err(storage_error)?;
    }
    tx.commit().map_err(storage_error)?;
    Ok(())
}

impl Lease {
    pub(super) fn new(
        store: &GardenStore,
        binding: &Binding<'_>,
        payload: Value,
        cutoff: u64,
    ) -> Result<Self> {
        let Binding {
            state,
            principal,
            auth,
            kind,
            digest,
        } = binding;
        let issued_at = store.clock.now();
        let expires_at = issued_at
            .checked_add_signed(Duration::minutes(5))
            .ok_or_else(restart)?;
        Ok(Self {
            snapshot_id: Uuid::new_v4().to_string(),
            workspace_id: state.workspace_id.clone(),
            generation: state.generation.clone(),
            actor: auth.client_id.clone(),
            principal_hash: principal_hash(principal)?,
            visibility_epoch: epoch(state, principal),
            issued_at,
            expires_at,
            cutoff,
            kind: (*kind).into(),
            digest: (*digest).into(),
            payload,
            position_key: None,
            candidate_digest: None,
        })
    }
    fn check(
        &self,
        store: &GardenStore,
        state: &State,
        principal: &Principal,
        auth: &Auth,
        kind: &str,
        digest: &str,
    ) -> Result<()> {
        let now = store.clock.now();
        if self.workspace_id != state.workspace_id
            || self.generation != state.generation
            || self.actor != auth.client_id
            || self.principal_hash != principal_hash(principal)?
            || self.visibility_epoch != epoch(state, principal)
            || self.kind != kind
            || self.digest != digest
            || now < self.issued_at
            || now >= self.expires_at
            || self.cutoff > state.sequence
        {
            return Err(restart());
        }
        Ok(())
    }
    pub(super) fn page(&self, next_cursor: Option<String>) -> PageState {
        PageState {
            next_cursor,
            visibility_epoch: self.visibility_epoch.clone(),
            snapshot_id: self.snapshot_id.clone(),
            snapshot_expires_at: self.expires_at.to_rfc3339_opts(SecondsFormat::Nanos, true),
        }
    }
}

pub(super) fn load(
    store: &GardenStore,
    tx: &redb::ReadTransaction,
    binding: &Binding<'_>,
    token: &str,
) -> Result<Lease> {
    if token.len() != 64 || !token.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(restart());
    }
    let table = tx.open_table(LEASES).map_err(storage_error)?;
    let row = table
        .get(contracts::hash(token).as_str())
        .map_err(storage_error)?
        .ok_or_else(restart)?;
    let mut lease: Lease = decode(row.value())?;
    lease.check(
        store,
        binding.state,
        binding.principal,
        binding.auth,
        binding.kind,
        binding.digest,
    )?;
    if lease.kind == "query" && lease.payload.get("candidates").is_none() {
        let data = tx.open_table(SNAPSHOT_DATA).map_err(storage_error)?;
        let bytes = data
            .get(lease.snapshot_id.as_str())
            .map_err(storage_error)?
            .ok_or_else(restart)?;
        let candidates: Value = decode(bytes.value())?;
        if !candidates.is_array()
            || lease.candidate_digest.as_deref()
                != Some(contracts::hash(&canonical_json(&candidates)?).as_str())
        {
            return Err(storage_error("Snapshot candidate digest mismatch"));
        }
        lease
            .payload
            .as_object_mut()
            .ok_or_else(|| storage_error("Invalid snapshot payload"))?
            .insert("candidates".into(), candidates);
    }
    lease.position_key = None;
    lease.candidate_digest = None;
    Ok(lease)
}

/// Reject a response if canonical changes raced its visibility checks. A later
/// sequential page may still use the lease's older immutable cutoff normally.
pub(super) fn revalidate_at(
    store: &GardenStore,
    auth: &Auth,
    lease: &Lease,
    served_sequence: u64,
) -> Result<()> {
    let tx = store.db.begin_read().map_err(storage_error)?;
    let state = state_read(&tx)?;
    let principal = principal_read(&tx, auth, GardenAction::Read)?;
    lease.check(store, &state, &principal, auth, &lease.kind, &lease.digest)?;
    if state.sequence != served_sequence {
        return Err(restart());
    }
    Ok(())
}

/// Save an immutable next-page position without extending its original lease deadline.
pub(super) fn save(store: &GardenStore, auth: &Auth, lease: &Lease) -> Result<String> {
    let position_key = contracts::hash(&canonical_json(lease)?);
    let mut compact = lease.clone();
    compact.position_key = Some(position_key.clone());
    let candidates = if lease.kind == "query" {
        compact
            .payload
            .as_object_mut()
            .and_then(|payload| payload.remove("candidates"))
    } else {
        None
    };
    let data = if let Some(candidates) = candidates {
        if !candidates.is_array() {
            return Err(storage_error("Invalid snapshot candidates"));
        }
        compact.candidate_digest = Some(contracts::hash(&canonical_json(&candidates)?));
        Some(encode(&candidates)?)
    } else {
        None
    };
    let bytes = encode(&compact)?;
    if bytes.len() > MAX_JSON_BYTES
        || data
            .as_ref()
            .is_some_and(|bytes| bytes.len() > MAX_JSON_BYTES)
    {
        return Err(failure(
            GardenErrorCode::LimitExceeded,
            "Snapshot lease candidate byte limit exceeded",
        ));
    }
    let mut tx = store.db.begin_write().map_err(storage_error)?;
    tx.set_durability(Durability::Immediate)
        .map_err(storage_error)?;
    let state = state_write(&tx)?;
    let principal = principal_write(&tx, auth, GardenAction::Read)?;
    lease.check(store, &state, &principal, auth, &lease.kind, &lease.digest)?;
    let now = store.clock.now().timestamp();
    {
        let mut rows = tx.open_table(LEASES).map_err(storage_error)?;
        let mut expiry = tx.open_table(EXPIRY).map_err(storage_error)?;
        let mut tokens = tx.open_table(TOKENS).map_err(storage_error)?;
        let mut snapshots = tx.open_table(SNAPSHOT_DATA).map_err(storage_error)?;
        let mut snapshot_expiry = tx.open_table(SNAPSHOT_EXPIRY).map_err(storage_error)?;
        if let Some(token) = tokens.get(position_key.as_str()).map_err(storage_error)? {
            return Ok(token.value().to_owned());
        }
        let expired = expiry
            .range(..=(now, "\u{10ffff}"))
            .map_err(storage_error)?
            .take(64)
            .map(|entry| {
                let (k, _) = entry.map_err(storage_error)?;
                Ok((k.value().0, k.value().1.to_owned()))
            })
            .collect::<Result<Vec<_>>>()?;
        for (time, key) in expired {
            expiry.remove((time, key.as_str())).map_err(storage_error)?;
            if let Some(value) = rows.remove(key.as_str()).map_err(storage_error)? {
                let old: Lease = decode(value.value())?;
                let old_key = match old.position_key.as_ref() {
                    Some(key) => key.clone(),
                    None => contracts::hash(&canonical_json(&old)?),
                };
                tokens.remove(old_key.as_str()).map_err(storage_error)?;
            }
        }
        let expired_snapshots = snapshot_expiry
            .range(..=(now, "\u{10ffff}"))
            .map_err(storage_error)?
            .take(64)
            .map(|entry| {
                let (key, _) = entry.map_err(storage_error)?;
                Ok((key.value().0, key.value().1.to_owned()))
            })
            .collect::<Result<Vec<_>>>()?;
        for (time, id) in expired_snapshots {
            snapshot_expiry
                .remove((time, id.as_str()))
                .map_err(storage_error)?;
            snapshots.remove(id.as_str()).map_err(storage_error)?;
        }
        if rows.len().map_err(storage_error)? >= MAX_LEASES {
            return Err(failure(
                GardenErrorCode::LimitExceeded,
                "Snapshot lease capacity reached; retry after expiry",
            ));
        }
        if let Some(data) = &data {
            let exists = {
                let existing = snapshots
                    .get(lease.snapshot_id.as_str())
                    .map_err(storage_error)?;
                if let Some(existing) = existing {
                    if existing.value() != data.as_slice() {
                        return Err(storage_error("Snapshot candidates changed"));
                    }
                    true
                } else {
                    false
                }
            };
            if !exists {
                if snapshots.len().map_err(storage_error)? >= MAX_SNAPSHOTS {
                    return Err(failure(
                        GardenErrorCode::LimitExceeded,
                        "Snapshot candidate capacity reached; retry after expiry",
                    ));
                }
                snapshots
                    .insert(lease.snapshot_id.as_str(), data.as_slice())
                    .map_err(storage_error)?;
                snapshot_expiry
                    .insert(
                        (lease.expires_at.timestamp() + 1, lease.snapshot_id.as_str()),
                        1,
                    )
                    .map_err(storage_error)?;
            }
        }
        let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
        let key = contracts::hash(&token);
        rows.insert(key.as_str(), bytes.as_slice())
            .map_err(storage_error)?;
        expiry
            .insert((lease.expires_at.timestamp() + 1, key.as_str()), 1)
            .map_err(storage_error)?;
        tokens
            .insert(position_key.as_str(), token.as_str())
            .map_err(storage_error)?;
        drop(tokens);
        drop(snapshots);
        drop(snapshot_expiry);
        drop(expiry);
        drop(rows);
        tx.commit().map_err(storage_error)?;
        Ok(token)
    }
}
