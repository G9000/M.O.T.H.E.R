//! Independent transactional graph storage; no live-store migration or MCP dispatch.
use super::{
    access::{GardenAccess, GardenAction, GardenObjectVisibility},
    contracts::*,
    history, indexes,
};
use crate::{
    access::{AccessMode, ClientAccess},
    contracts::{self, Auth},
};
use chrono::{DateTime, SecondsFormat, Utc};
use rand::RngCore;
use redb::{Database, Durability, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    path::Path,
    sync::Arc,
};
use subtle::ConstantTimeEq;
use uuid::Uuid;

pub(crate) type Result<T> = std::result::Result<T, GardenError>;
pub(crate) const CURRENT: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_current");
pub(crate) const META: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_metadata");
pub(crate) const RECEIPTS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_receipts");
const PRINCIPALS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_principals");
pub(crate) const REPLACEMENTS: TableDefinition<&str, &str> =
    TableDefinition::new("garden_replacements");
pub(crate) const REPLACEMENT_INCOMING: TableDefinition<(&str, &str), u8> =
    TableDefinition::new("garden_replacement_incoming");
#[path = "backup_audit.rs"]
pub(crate) mod backup_audit;
#[path = "changes.rs"]
pub(crate) mod changes;
#[path = "compat.rs"]
pub(crate) mod compat;
#[path = "inbox.rs"]
pub(crate) mod inbox;
#[path = "pager.rs"]
pub(crate) mod pager;
#[path = "query.rs"]
pub(crate) mod query;
#[path = "search.rs"]
pub(crate) mod search;

const MAX_TRANSACTION_OBJECTS: usize = 10_000;

pub(crate) fn failure(code: GardenErrorCode, message: &str) -> GardenError {
    GardenError {
        code,
        message: message.into(),
        restart_required: code == GardenErrorCode::RestartRequired,
    }
}
pub(crate) fn storage_error(_error: impl fmt::Display) -> GardenError {
    failure(
        GardenErrorCode::StoreUnavailable,
        "Garden storage unavailable; if a write was attempted its outcome may be uncertain. Reconcile the same operation UUID.",
    )
}
fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec(value).map_err(storage_error)
}
fn decode<T: for<'a> Deserialize<'a>>(value: &[u8]) -> Result<T> {
    serde_json::from_slice(value).map_err(storage_error)
}
fn not_found() -> GardenError {
    failure(GardenErrorCode::NotFound, "Graph object does not exist")
}

/// Trusted runtime clock, installed by a constructor rather than by a request.
pub trait Clock: Send + Sync + fmt::Debug {
    fn now(&self) -> DateTime<Utc>;
}
/// System UTC clock used by ordinary graph stores.
#[derive(Debug)]
pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// Persistence barriers available only to trusted library instrumentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitStage {
    BeforeWrite,
    Staged,
    BeforeCommit,
    Committed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    format: u32,
    workspace_id: String,
    generation: String,
    sequence: u64,
    last_time: Option<String>,
    epoch: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Principal {
    token_hash: String,
    scopes: Vec<String>,
    access: ClientAccess,
    garden: GardenAccess,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredOperation {
    payload_hash: String,
    request: BatchRequest,
    subject: Option<String>,
    receipt: OperationReceipt,
    records: Vec<GardenRecord>,
}

/// New graph runtime with graph-native credentials, transactions and immutable receipts.
pub struct GardenStore {
    db: Database,
    clock: Arc<dyn Clock>,
}
impl fmt::Debug for GardenStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GardenStore").finish_non_exhaustive()
    }
}
impl GardenStore {
    /// Exclusively initialize a new private graph file with explicit workspace identity.
    /// # Errors
    /// Fails if the destination exists or identity, permissions or storage are invalid.
    pub fn initialize(path: impl AsRef<Path>, workspace: &str, generation: &str) -> Result<Self> {
        Self::initialize_with_clock(path, workspace, generation, Arc::new(SystemClock))
    }
    /// Initialize a graph with a trusted deterministic clock.
    /// # Errors
    /// Fails under the same conditions as [`Self::initialize`].
    pub fn initialize_with_clock(
        path: impl AsRef<Path>,
        workspace: &str,
        generation: &str,
        clock: Arc<dyn Clock>,
    ) -> Result<Self> {
        validate_identity(workspace, generation)?;
        let file = crate::config::create_private_file(path.as_ref()).map_err(storage_error)?;
        let backend = redb::backends::FileBackend::new(file).map_err(storage_error)?;
        let db = Database::builder()
            .create_with_backend(backend)
            .map_err(storage_error)?;
        Self::initialize_database(db, workspace, generation, clock)
    }
    /// Open an existing graph, rejecting missing files and foreign storage formats.
    /// # Errors
    /// Fails for missing, invalid, locked or unsupported databases.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_with_clock(path, Arc::new(SystemClock))
    }
    /// Open an existing graph with a trusted runtime clock.
    /// # Errors
    /// Fails under the same conditions as [`Self::open`].
    pub fn open_with_clock(path: impl AsRef<Path>, clock: Arc<dyn Clock>) -> Result<Self> {
        let db = Database::open(path).map_err(storage_error)?;
        Self::existing(db, clock)
    }
    /// Initialize a synthetic backend for persistence qualification.
    #[doc(hidden)]
    pub fn initialize_with_backend(
        backend: impl redb::StorageBackend,
        workspace: &str,
        generation: &str,
    ) -> Result<Self> {
        validate_identity(workspace, generation)?;
        let db = Database::builder()
            .create_with_backend(backend)
            .map_err(storage_error)?;
        Self::initialize_database(db, workspace, generation, Arc::new(SystemClock))
    }
    /// Open a synthetic backend, requiring existing Garden metadata.
    #[doc(hidden)]
    pub fn open_with_backend(backend: impl redb::StorageBackend) -> Result<Self> {
        let db = Database::builder()
            .create_with_backend(backend)
            .map_err(storage_error)?;
        Self::existing(db, Arc::new(SystemClock))
    }
    fn initialize_database(
        db: Database,
        workspace: &str,
        generation: &str,
        clock: Arc<dyn Clock>,
    ) -> Result<Self> {
        let mut tx = db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        {
            let mut meta = tx.open_table(META).map_err(storage_error)?;
            if meta.get("state").map_err(storage_error)?.is_some() {
                return Err(failure(
                    GardenErrorCode::Conflict,
                    "Graph already initialized",
                ));
            }
            let state = State {
                format: 1,
                workspace_id: workspace.into(),
                generation: generation.into(),
                sequence: 0,
                last_time: None,
                epoch: 1,
            };
            meta.insert("state", encode(&state)?.as_slice())
                .map_err(storage_error)?;
        }
        tx.open_table(CURRENT).map_err(storage_error)?;
        tx.open_table(PRINCIPALS).map_err(storage_error)?;
        tx.open_table(RECEIPTS).map_err(storage_error)?;
        tx.open_table(REPLACEMENTS).map_err(storage_error)?;
        tx.open_table(REPLACEMENT_INCOMING).map_err(storage_error)?;
        history::initialize(&tx)?;
        indexes::initialize(&tx)?;
        inbox::initialize(&tx)?;
        pager::initialize(&tx)?;
        tx.commit().map_err(storage_error)?;
        Ok(Self { db, clock })
    }
    fn existing(db: Database, clock: Arc<dyn Clock>) -> Result<Self> {
        let (needs_inbox, needs_query) = {
            let tx = db.begin_read().map_err(storage_error)?;
            let table = tx.open_table(META).map_err(storage_error)?;
            let state: State = decode(
                table
                    .get("state")
                    .map_err(storage_error)?
                    .ok_or_else(|| {
                        failure(GardenErrorCode::StoreUnavailable, "Not a Garden database")
                    })?
                    .value(),
            )?;
            validate_state(&state)?;
            tx.open_table(CURRENT).map_err(storage_error)?;
            tx.open_table(PRINCIPALS).map_err(storage_error)?;
            tx.open_table(RECEIPTS).map_err(storage_error)?;
            tx.open_table(REPLACEMENTS).map_err(storage_error)?;
            tx.open_table(REPLACEMENT_INCOMING).map_err(storage_error)?;
            let needs_inbox = table
                .get("inbox_registry_initialized")
                .map_err(storage_error)?
                .is_none();
            let needs_query = table
                .get("query_indexes_v1")
                .map_err(storage_error)?
                .is_none();
            (needs_inbox, needs_query)
        };
        if needs_inbox || needs_query || pager::needs_initialize(&db)? {
            let mut tx = db.begin_write().map_err(storage_error)?;
            tx.set_durability(Durability::Immediate)
                .map_err(storage_error)?;
            inbox::initialize(&tx)?;
            pager::initialize(&tx)?;
            if needs_query {
                indexes::upgrade(&tx)?;
            }
            tx.commit().map_err(storage_error)?;
        }
        Ok(Self { db, clock })
    }
    /// Owner-only credential provisioning in the same writer used by graph mutations.
    /// # Errors
    /// Rejects invalid scopes, invalid policies and shared write grants.
    pub fn grant(
        &self,
        client: &str,
        scopes: &[String],
        access: &ClientAccess,
        garden: &GardenAccess,
    ) -> Result<Auth> {
        let (auth, principal) = prepare_grant(client, scopes, access, garden)?;
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        tx.open_table(PRINCIPALS)
            .map_err(storage_error)?
            .insert(client, encode(&principal)?.as_slice())
            .map_err(storage_error)?;
        bump_epoch(&tx)?;
        tx.commit().map_err(storage_error)?;
        Ok(auth)
    }
    /// Revoke a graph credential atomically with respect to pending graph writers.
    /// # Errors
    /// Returns storage errors without changing canonical graph sequence numbers.
    pub fn revoke(&self, client: &str) -> Result<()> {
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        tx.open_table(PRINCIPALS)
            .map_err(storage_error)?
            .remove(client)
            .map_err(storage_error)?;
        bump_epoch(&tx)?;
        tx.commit().map_err(storage_error)
    }
    /// Apply one bounded graph batch atomically, preserving its original operation outcome.
    /// # Errors
    /// Rejects unauthorized, reused, stale, dangling or invalid lifecycle mutations.
    /// Storage errors may represent an uncertain commit; reconcile the same operation UUID.
    pub fn apply(&self, auth: &Auth, request: &BatchRequest) -> Result<OutcomeResponse> {
        self.apply_with_hook(auth, request, |_| {})
    }
    /// Apply with trusted persistence barrier instrumentation.
    #[doc(hidden)]
    pub fn apply_with_hook(
        &self,
        auth: &Auth,
        request: &BatchRequest,
        mut hook: impl FnMut(CommitStage),
    ) -> Result<OutcomeResponse> {
        GardenRequest::parse(
            GardenTool::Apply,
            &serde_json::to_value(request).map_err(storage_error)?,
        )?;
        let hash = contracts::hash(&canonical_json(request)?);
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let state = state_write(&tx)?;
        envelope(&state, &request.workspace_id, &request.generation)?;
        let principal = principal_write(&tx, auth, GardenAction::Write)?;
        let key = operation_key(&request.generation, &auth.client_id, &request.operation_id);
        let stored = tx
            .open_table(RECEIPTS)
            .map_err(storage_error)?
            .get(key.as_str())
            .map_err(storage_error)?
            .map(|v| decode::<StoredOperation>(v.value()))
            .transpose()?;
        if let Some(stored) = stored {
            if stored.subject != principal.access.subject {
                return Err(not_found());
            }
            if stored.payload_hash != hash || stored.request != *request {
                return Err(failure(
                    GardenErrorCode::OperationReused,
                    "Operation UUID was used for a different payload",
                ));
            }
            authorize_operation(&principal, &stored)?;
            let (current, replacements) = context_write(&tx, &principal, operation_seeds(&stored))?;
            return outcome(&state, &principal, &stored, &current, &replacements);
        }
        inbox::check_operation(
            &tx,
            &request.generation,
            &request.operation_id,
            &auth.client_id,
            "apply",
        )?;
        hook(CommitStage::BeforeWrite);
        let prepared =
            self.prepare_batch(&tx, request, &principal, state, None, &auth.client_id)?;
        inbox::claim_operation(
            &tx,
            &request.generation,
            &request.operation_id,
            &auth.client_id,
            "apply",
        )?;
        persist_batch(&tx, &key, &prepared)?;
        hook(CommitStage::Staged);
        hook(CommitStage::BeforeCommit);
        tx.commit().map_err(storage_error)?;
        hook(CommitStage::Committed);
        Ok(prepared.response)
    }
    fn prepare_batch(
        &self,
        tx: &redb::WriteTransaction,
        request: &BatchRequest,
        principal: &Principal,
        mut state: State,
        review: Option<&inbox::ReviewAttribution>,
        actor: &str,
    ) -> Result<PreparedBatch> {
        authorize_request(principal, request)?;
        let mut seeds = request_seeds(request);
        for item in &request.items {
            if matches!(item.mutation, Mutation::Retract | Mutation::Supersede) {
                let reference = item.id.as_ref().ok_or_else(not_found)?;
                seeds.extend(indexes::dependents(
                    tx,
                    reference,
                    MAX_TRANSACTION_OBJECTS,
                    |reference| require_allowed(principal, reference),
                )?);
                let incoming = tx.open_table(REPLACEMENT_INCOMING).map_err(storage_error)?;
                for entry in incoming
                    .range((reference.id.as_str(), "")..=(reference.id.as_str(), "\u{10ffff}"))
                    .map_err(storage_error)?
                {
                    let (key, _) = entry.map_err(storage_error)?;
                    require_allowed(
                        principal,
                        &ExternalReference {
                            kind: reference.kind,
                            id: key.value().1.into(),
                        },
                    )?;
                    seeds.push(key.value().1.to_string());
                    if seeds.len() > MAX_TRANSACTION_OBJECTS {
                        return Err(work_limit());
                    }
                }
            }
        }
        let (original, mut replacements) = context_write(tx, principal, seeds)?;
        let mut current = original.clone();
        let mut resolved = BTreeMap::new();
        for item in &request.items {
            let reference = match &item.id {
                Some(id) => id.clone(),
                None => ExternalReference {
                    kind: item.draft.as_ref().ok_or_else(not_found)?.kind(),
                    id: Uuid::new_v4().to_string(),
                },
            };
            resolved.insert(item.item_key.clone(), reference);
        }
        state.sequence = state
            .sequence
            .checked_add(1)
            .filter(|s| *s <= MAX_SAFE_INTEGER)
            .ok_or_else(|| failure(GardenErrorCode::LimitExceeded, "Graph sequence exhausted"))?;
        let mut time = self.clock.now();
        if let Some(previous) = &state.last_time {
            let previous = DateTime::parse_from_rfc3339(previous)
                .map_err(storage_error)?
                .with_timezone(&Utc);
            if time <= previous {
                time = previous
                    .checked_add_signed(chrono::Duration::nanoseconds(1))
                    .ok_or_else(|| storage_error("clock exhausted"))?;
            }
        }
        let timestamp = time.to_rfc3339_opts(SecondsFormat::Nanos, true);
        state.last_time = Some(timestamp.clone());
        let mut records = Vec::new();
        let mut results = Vec::new();
        for item in &request.items {
            let reference = resolved.get(&item.item_key).ok_or_else(not_found)?;
            let previous = original.get(&reference.id);
            if item.mutation != Mutation::Create {
                let previous = previous
                    .filter(|r| r.draft.kind() == reference.kind)
                    .ok_or_else(not_found)?;
                visible(principal, previous, &original, &replacements)?;
                if previous.metadata.revision != item.expected_revision {
                    return Err(failure(
                        GardenErrorCode::Conflict,
                        "Expected graph revision is stale",
                    ));
                }
                if previous.metadata.lifecycle != Lifecycle::Active {
                    return Err(failure(
                        GardenErrorCode::Conflict,
                        "Retired graph records cannot be changed",
                    ));
                }
            }
            let mut draft = match &item.draft {
                Some(draft) => draft.clone(),
                None => previous.ok_or_else(not_found)?.draft.clone(),
            };
            resolve_draft(&mut draft, &resolved)?;
            if let Some(previous) = previous {
                merge_extensions(&previous.draft, &mut draft);
            }
            draft.validate()?;
            let revision = previous.map_or(1, |r| r.metadata.revision + 1);
            let lifecycle = match item.mutation {
                Mutation::Create | Mutation::Update => Lifecycle::Active,
                Mutation::Retract => Lifecycle::Retracted,
                Mutation::Supersede => Lifecycle::Superseded,
            };
            if let Some(replacement) = &item.replacement {
                let replacement = resolve_ref(replacement, &resolved)?;
                if replacement.id == reference.id {
                    return Err(failure(
                        GardenErrorCode::InvalidEndpoint,
                        "Self supersession is invalid",
                    ));
                }
                replacements.insert(reference.id.clone(), replacement.id);
            }
            let record = GardenRecord {
                metadata: RevisionMetadata {
                    id: reference.id.clone(),
                    revision,
                    previous_revision: previous.map(|r| r.metadata.revision),
                    commit_sequence: state.sequence,
                    transaction_time: timestamp.clone(),
                    actor: actor.to_string(),
                    contributor: previous
                        .map_or_else(|| actor.to_string(), |r| r.metadata.contributor.clone()),
                    approver: previous.and_then(|r| r.metadata.approver.clone()),
                    lifecycle,
                },
                draft,
            };
            let mut record = record;
            if let Some(review) = review {
                record.metadata.actor = inbox::OWNER.into();
                record.metadata.approver = Some(inbox::OWNER.into());
                if previous.is_none() {
                    record.metadata.contributor = if review
                        .original
                        .items
                        .iter()
                        .any(|old| old.item_key == item.item_key)
                    {
                        review.proposer.clone()
                    } else {
                        inbox::OWNER.into()
                    };
                }
            }
            record.validate()?;
            // Immutable history must remain readable even after watermarks reach their longest encoding.
            record_response(&state, principal, record.clone(), MAX_SAFE_INTEGER)?;
            current.insert(reference.id.clone(), record.clone());
            records.push(record);
            results.push(ItemResult {
                item_key: item.item_key.clone(),
                reference: reference.clone(),
                revision,
                owner_edited: review.is_some_and(|review| {
                    review
                        .original
                        .items
                        .iter()
                        .find(|old| old.item_key == item.item_key)
                        .is_some_and(|old| old != item)
                }),
                owner_added: review.is_some_and(|review| {
                    !review
                        .original
                        .items
                        .iter()
                        .any(|old| old.item_key == item.item_key)
                }),
            });
        }
        let created: BTreeSet<_> = request
            .items
            .iter()
            .filter(|i| i.mutation == Mutation::Create)
            .filter_map(|i| resolved.get(&i.item_key))
            .cloned()
            .collect();
        for record in &records {
            visible_for_write(principal, record, &current, &created)?;
        }
        for item in &request.items {
            if let Some(replacement) = &item.replacement {
                let reference = resolve_ref(replacement, &resolved)?;
                let target = current.get(&reference.id).ok_or_else(not_found)?;
                visible_for_write(principal, target, &current, &created)?;
            }
        }
        validate_final(&current, &replacements)?;
        let receipt = OperationReceipt {
            operation_id: request.operation_id.clone(),
            operation_generation: request.generation.clone(),
            actor: actor.to_string(),
            status: if review.is_some() {
                OutcomeStatus::Approved
            } else {
                OutcomeStatus::Applied
            },
            proposal_id: review.map(|review| review.proposal_id.clone()),
            committed_at: Some(timestamp),
            commit_sequence: Some(state.sequence),
            results,
        };
        receipt.validate()?;
        let stored = StoredOperation {
            payload_hash: contracts::hash(&canonical_json(request)?),
            request: request.clone(),
            subject: principal.access.subject.clone(),
            receipt,
            records,
        };
        let response = outcome(&state, principal, &stored, &current, &replacements)?;
        // Validate the complete wire result before any canonical side effect is committed.
        GardenResponse::parse(
            GardenTool::GetReceipt,
            &serde_json::to_value(&response).map_err(storage_error)?,
        )?;
        Ok(PreparedBatch {
            state,
            original,
            replacements,
            stored,
            response,
        })
    }
    /// Read a currently authorized immutable operation outcome.
    /// # Errors
    /// Requires the original actor, subject, generation, scope and payload visibility.
    pub fn receipt(&self, auth: &Auth, request: &OutcomeRequest) -> Result<OutcomeResponse> {
        GardenRequest::parse(
            GardenTool::GetReceipt,
            &serde_json::to_value(request).map_err(storage_error)?,
        )?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        envelope(&state, &request.workspace_id, &request.generation)?;
        if request.operation_generation != state.generation {
            return Err(failure(
                GardenErrorCode::GenerationMismatch,
                "Operation belongs to another generation",
            ));
        }
        if inbox::is_proposal_operation(
            &tx,
            &request.operation_generation,
            &request.operation_id,
            &auth.client_id,
        )? {
            return inbox::proposal_outcome_read(&tx, &state, auth, request, false);
        }
        let principal = principal_read(&tx, auth, GardenAction::Write)?;
        let key = operation_key(
            &request.operation_generation,
            &auth.client_id,
            &request.operation_id,
        );
        let table = tx.open_table(RECEIPTS).map_err(storage_error)?;
        let operation = table
            .get(key.as_str())
            .map_err(storage_error)?
            .ok_or_else(not_found)?;
        let operation: StoredOperation = decode(operation.value())?;
        if operation.subject != principal.access.subject {
            return Err(not_found());
        }
        authorize_operation(&principal, &operation)?;
        let (current, replacements) = context_read(&tx, &principal, operation_seeds(&operation))?;
        outcome(&state, &principal, &operation, &current, &replacements)
    }
    /// Read a revision or reconstruct the graph belief at an exact transaction instant.
    /// # Errors
    /// Requires read scope and current visibility of current and historical footprints.
    pub fn get(&self, auth: &Auth, request: &GetRequest) -> Result<GetResponse> {
        GardenRequest::parse(
            GardenTool::Get,
            &serde_json::to_value(request).map_err(storage_error)?,
        )?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        envelope(&state, &request.workspace_id, &request.generation)?;
        let principal = principal_read(&tx, auth, GardenAction::Read)?;
        require_allowed(&principal, &request.reference)?;
        let record = history::get(
            &tx,
            &request.reference.id,
            request.revision,
            request.known_at.as_deref(),
        )?
        .filter(|r| r.draft.kind() == request.reference.kind)
        .ok_or_else(not_found)?;
        principal
            .garden
            .require_visible(&[footprint(&record)])
            .map_err(|_| not_found())?;
        let mut seeds = record_seeds(&record);
        seeds.push(record.metadata.id.clone());
        let (current, replacements) = context_read(&tx, &principal, seeds)?;
        visible(&principal, &record, &current, &replacements)?;
        record_response(&state, &principal, record, state.sequence)
    }
    /// Test a selected assertion against a half-open valid-time interval.
    /// # Errors
    /// Uses the same authenticated snapshot and history checks as [`Self::get`].
    pub fn get_valid_at(
        &self,
        auth: &Auth,
        request: &GetRequest,
        valid_at: &str,
    ) -> Result<Option<GetResponse>> {
        let response = self.get(auth, request)?;
        if history::valid_at(&response.record, valid_at)?
            && response.record.metadata.lifecycle == Lifecycle::Active
        {
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
    /// Verify canonical history and synchronous index consistency in a read snapshot.
    /// # Errors
    /// Requires full graph read authority and reports any inconsistency.
    pub fn audit(&self, auth: &Auth) -> Result<indexes::StoreStats> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        let principal = principal_read(&tx, auth, GardenAction::Read)?;
        if !matches!(principal.garden, GardenAccess::All {}) {
            return Err(failure(
                GardenErrorCode::Forbidden,
                "Audit requires full graph visibility",
            ));
        }
        let stats = indexes::audit_read(&tx)?;
        inbox::audit_read(&tx)?;
        Ok(stats)
    }
}

fn validate_identity(workspace: &str, generation: &str) -> Result<()> {
    contracts::id(workspace)
        .and_then(|_| contracts::id(generation))
        .map_err(|_| {
            failure(
                GardenErrorCode::InvalidInput,
                "Explicit canonical workspace and generation UUIDs are required",
            )
        })
}
fn validate_state(state: &State) -> Result<()> {
    validate_identity(&state.workspace_id, &state.generation)?;
    if state.format != 1
        || state.sequence > MAX_SAFE_INTEGER
        || state.epoch == 0
        || (state.sequence == 0) != state.last_time.is_none()
    {
        return Err(failure(
            GardenErrorCode::StoreUnavailable,
            "Unsupported Garden identity or format",
        ));
    }
    if let Some(time) = &state.last_time {
        DateTime::parse_from_rfc3339(time).map_err(storage_error)?;
    }
    Ok(())
}
fn state_write(tx: &redb::WriteTransaction) -> Result<State> {
    let table = tx.open_table(META).map_err(storage_error)?;
    let state = decode(
        table
            .get("state")
            .map_err(storage_error)?
            .ok_or_else(not_found)?
            .value(),
    )?;
    validate_state(&state)?;
    Ok(state)
}
fn state_read(tx: &redb::ReadTransaction) -> Result<State> {
    let table = tx.open_table(META).map_err(storage_error)?;
    let state = decode(
        table
            .get("state")
            .map_err(storage_error)?
            .ok_or_else(not_found)?
            .value(),
    )?;
    validate_state(&state)?;
    Ok(state)
}
fn bump_epoch(tx: &redb::WriteTransaction) -> Result<()> {
    let mut state = state_write(tx)?;
    state.epoch = state
        .epoch
        .checked_add(1)
        .ok_or_else(|| storage_error("epoch exhausted"))?;
    tx.open_table(META)
        .map_err(storage_error)?
        .insert("state", encode(&state)?.as_slice())
        .map_err(storage_error)?;
    Ok(())
}
fn envelope(state: &State, workspace: &str, generation: &str) -> Result<()> {
    if state.workspace_id != workspace {
        return Err(failure(
            GardenErrorCode::WorkspaceMismatch,
            "Graph workspace mismatch",
        ));
    }
    if state.generation != generation {
        return Err(failure(
            GardenErrorCode::GenerationMismatch,
            "Graph generation mismatch",
        ));
    }
    Ok(())
}
fn check_principal(principal: Principal, auth: &Auth, action: GardenAction) -> Result<Principal> {
    let actual = contracts::hash(&auth.token);
    if !bool::from(actual.as_bytes().ct_eq(principal.token_hash.as_bytes())) {
        return Err(failure(
            GardenErrorCode::Unauthorized,
            "Invalid graph credential",
        ));
    }
    principal.access.validate().map_err(storage_error)?;
    principal.garden.validate().map_err(storage_error)?;
    if !principal.scopes.iter().any(|s| s == action.scope())
        || action == GardenAction::Write && principal.access.mode == AccessMode::Shared
    {
        return Err(failure(
            GardenErrorCode::Forbidden,
            "Current graph scope required",
        ));
    }
    Ok(principal)
}
fn principal_write(
    tx: &redb::WriteTransaction,
    auth: &Auth,
    action: GardenAction,
) -> Result<Principal> {
    let table = tx.open_table(PRINCIPALS).map_err(storage_error)?;
    let principal = table
        .get(auth.client_id.as_str())
        .map_err(storage_error)?
        .ok_or_else(|| failure(GardenErrorCode::Unauthorized, "Invalid graph credential"))?;
    check_principal(decode(principal.value())?, auth, action)
}
fn principal_read(
    tx: &redb::ReadTransaction,
    auth: &Auth,
    action: GardenAction,
) -> Result<Principal> {
    let table = tx.open_table(PRINCIPALS).map_err(storage_error)?;
    let principal = table
        .get(auth.client_id.as_str())
        .map_err(storage_error)?
        .ok_or_else(|| failure(GardenErrorCode::Unauthorized, "Invalid graph credential"))?;
    check_principal(decode(principal.value())?, auth, action)
}
type Context = (BTreeMap<String, GardenRecord>, BTreeMap<String, String>);
fn work_limit() -> GardenError {
    failure(
        GardenErrorCode::LimitExceeded,
        "Graph transaction reference/dependency work budget exceeded",
    )
}
fn record_seeds(record: &GardenRecord) -> Vec<String> {
    record
        .draft
        .references()
        .into_iter()
        .filter_map(|r| match r {
            RecordRef::Existing { id, .. } => Some(id.clone()),
            _ => None,
        })
        .collect()
}
fn request_seeds(request: &BatchRequest) -> Vec<String> {
    let mut seeds = Vec::new();
    for item in &request.items {
        if let Some(reference) = &item.id {
            seeds.push(reference.id.clone());
        }
        let mut refs = item
            .draft
            .as_ref()
            .map_or_else(Vec::new, GardenDraft::references);
        refs.extend(item.replacement.as_ref());
        for reference in refs {
            if let RecordRef::Existing { id, .. } = reference {
                seeds.push(id.clone());
            }
        }
    }
    seeds
}
fn operation_seeds(operation: &StoredOperation) -> Vec<String> {
    let mut seeds = request_seeds(&operation.request);
    for record in &operation.records {
        seeds.push(record.metadata.id.clone());
        seeds.extend(record_seeds(record));
    }
    seeds
}
fn load_context(
    seeds: Vec<String>,
    principal: &Principal,
    mut record: impl FnMut(&str) -> Result<Option<GardenRecord>>,
    mut replacement: impl FnMut(&str) -> Result<Option<String>>,
) -> Result<Context> {
    let mut pending: BTreeSet<String> = seeds.into_iter().collect();
    if pending.len() > MAX_TRANSACTION_OBJECTS {
        return Err(work_limit());
    }
    let mut visited = BTreeSet::new();
    let mut records = BTreeMap::new();
    let mut replacements = BTreeMap::new();
    while let Some(id) = pending.pop_first() {
        if !visited.insert(id.clone()) {
            continue;
        }
        if visited.len() > MAX_TRANSACTION_OBJECTS {
            return Err(work_limit());
        }
        let value = record(&id)?.ok_or_else(not_found)?;
        principal
            .garden
            .require_visible(&[footprint(&value)])
            .map_err(|_| not_found())?;
        {
            pending.extend(
                record_seeds(&value)
                    .into_iter()
                    .filter(|id| !visited.contains(id)),
            );
            records.insert(id.clone(), value);
        }
        if let Some(target) = replacement(&id)? {
            let kind = records.get(&id).ok_or_else(not_found)?.draft.kind();
            require_allowed(
                principal,
                &ExternalReference {
                    kind,
                    id: target.clone(),
                },
            )?;
            if !visited.contains(&target) {
                pending.insert(target.clone());
            }
            replacements.insert(id, target);
        }
        if pending.len() + visited.len() > MAX_TRANSACTION_OBJECTS {
            return Err(work_limit());
        }
    }
    Ok((records, replacements))
}
fn context_write(
    tx: &redb::WriteTransaction,
    principal: &Principal,
    seeds: Vec<String>,
) -> Result<Context> {
    let records = tx.open_table(CURRENT).map_err(storage_error)?;
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    load_context(
        seeds,
        principal,
        |id| {
            records
                .get(id)
                .map_err(storage_error)?
                .map(|v| decode(v.value()))
                .transpose()
        },
        |id| {
            Ok(replacements
                .get(id)
                .map_err(storage_error)?
                .map(|v| v.value().to_owned()))
        },
    )
}
fn context_read(
    tx: &redb::ReadTransaction,
    principal: &Principal,
    seeds: Vec<String>,
) -> Result<Context> {
    let records = tx.open_table(CURRENT).map_err(storage_error)?;
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    load_context(
        seeds,
        principal,
        |id| {
            records
                .get(id)
                .map_err(storage_error)?
                .map(|v| decode(v.value()))
                .transpose()
        },
        |id| {
            Ok(replacements
                .get(id)
                .map_err(storage_error)?
                .map(|v| v.value().to_owned()))
        },
    )
}
fn operation_key(generation: &str, actor: &str, operation: &str) -> String {
    format!("{generation}/{actor}/{operation}")
}
fn require_allowed(principal: &Principal, reference: &ExternalReference) -> Result<()> {
    if principal.garden.allows(reference) {
        Ok(())
    } else {
        Err(not_found())
    }
}
fn authorize_request(principal: &Principal, request: &BatchRequest) -> Result<()> {
    for item in &request.items {
        if let Some(reference) = &item.id {
            require_allowed(principal, reference)?;
        }
        let mut refs = item
            .draft
            .as_ref()
            .map_or_else(Vec::new, GardenDraft::references);
        refs.extend(item.replacement.as_ref());
        for reference in refs {
            if let RecordRef::Existing { kind, id } = reference {
                require_allowed(
                    principal,
                    &ExternalReference {
                        kind: *kind,
                        id: id.clone(),
                    },
                )?;
            }
        }
    }
    Ok(())
}
fn authorize_operation(principal: &Principal, operation: &StoredOperation) -> Result<()> {
    authorize_request(principal, &operation.request)?;
    let footprints: Vec<_> = operation.records.iter().map(footprint).collect();
    principal
        .garden
        .require_visible(&footprints)
        .map_err(|_| not_found())
}
fn epoch(state: &State, principal: &Principal) -> String {
    contracts::hash(&format!(
        "{}:{}:{:?}:{:?}",
        state.generation, state.epoch, principal.access, principal.garden
    ))
}
fn canonical_json<T: Serialize>(value: &T) -> Result<String> {
    // serde_json's default map representation sorts nested keys; structs are first converted to it.
    serde_json::to_string(&serde_json::to_value(value).map_err(storage_error)?)
        .map_err(storage_error)
}
fn footprint(record: &GardenRecord) -> GardenObjectVisibility {
    let mut endpoints = Vec::new();
    let mut evidence = Vec::new();
    for reference in record.draft.references() {
        if let RecordRef::Existing { kind, id } = reference {
            let reference = ExternalReference {
                kind: *kind,
                id: id.clone(),
            };
            if *kind == GardenKind::Evidence {
                evidence.push(reference)
            } else {
                endpoints.push(reference)
            }
        }
    }
    GardenObjectVisibility {
        object: ExternalReference {
            kind: record.draft.kind(),
            id: record.metadata.id.clone(),
        },
        endpoints,
        evidence,
    }
}
fn visible(
    principal: &Principal,
    record: &GardenRecord,
    current: &BTreeMap<String, GardenRecord>,
    replacements: &BTreeMap<String, String>,
) -> Result<()> {
    let mut objects = vec![footprint(record)];
    if let Some(now) = current.get(&record.metadata.id) {
        objects.push(footprint(now));
    } else {
        return Err(not_found());
    }
    let mut cursor = record.metadata.id.as_str();
    let mut visited = BTreeSet::new();
    while let Some(next) = replacements.get(cursor) {
        if !visited.insert(cursor) {
            return Err(storage_error("replacement cycle"));
        }
        let target = current.get(next).ok_or_else(not_found)?;
        objects.push(footprint(target));
        cursor = next;
    }
    principal
        .garden
        .require_visible(&objects)
        .map_err(|_| not_found())
}
fn visible_for_write(
    principal: &Principal,
    record: &GardenRecord,
    current: &BTreeMap<String, GardenRecord>,
    created: &BTreeSet<ExternalReference>,
) -> Result<()> {
    let footprint = footprint(record);
    for reference in std::iter::once(&footprint.object)
        .chain(&footprint.endpoints)
        .chain(&footprint.evidence)
    {
        if !principal.garden.allows(reference) && !created.contains(reference) {
            return Err(not_found());
        }
        if current
            .get(&reference.id)
            .is_none_or(|r| r.draft.kind() != reference.kind)
        {
            return Err(not_found());
        }
    }
    Ok(())
}
fn outcome(
    state: &State,
    principal: &Principal,
    operation: &StoredOperation,
    current: &BTreeMap<String, GardenRecord>,
    replacements: &BTreeMap<String, String>,
) -> Result<OutcomeResponse> {
    for record in &operation.records {
        visible(principal, record, current, replacements)?;
    }
    let mut response = OutcomeResponse {
        version: 2,
        workspace_id: state.workspace_id.clone(),
        generation: state.generation.clone(),
        receipt: operation.receipt.clone(),
        original_proposal: None,
        visible_records: operation.records.clone(),
        payload_omitted: false,
        visibility_epoch: epoch(state, principal),
    };
    let value = serde_json::to_value(&response).map_err(storage_error)?;
    if bounded_json(&value).is_err() {
        response.visible_records.clear();
        response.payload_omitted = true;
    }
    GardenResponse::parse(
        GardenTool::GetReceipt,
        &serde_json::to_value(&response).map_err(storage_error)?,
    )?;
    Ok(response)
}
fn record_response(
    state: &State,
    principal: &Principal,
    record: GardenRecord,
    watermark: u64,
) -> Result<GetResponse> {
    let response = GetResponse {
        version: 2,
        workspace_id: state.workspace_id.clone(),
        generation: state.generation.clone(),
        record,
        coverage: Coverage {
            complete: true,
            truncated: false,
            reason: None,
            canonical_watermark: watermark,
            index_watermark: watermark,
            search_mode: SearchMode::Exact,
            model: None,
            profile: None,
        },
        visibility_epoch: epoch(state, principal),
    };
    GardenResponse::parse(
        GardenTool::Get,
        &serde_json::to_value(&response).map_err(storage_error)?,
    )?;
    Ok(response)
}
fn resolve_ref(
    reference: &RecordRef,
    resolved: &BTreeMap<String, ExternalReference>,
) -> Result<ExternalReference> {
    match reference {
        RecordRef::Existing { kind, id } => Ok(ExternalReference {
            kind: *kind,
            id: id.clone(),
        }),
        RecordRef::Item { kind, item_key } => resolved
            .get(item_key)
            .filter(|r| r.kind == *kind)
            .cloned()
            .ok_or_else(|| failure(GardenErrorCode::InvalidEndpoint, "Invalid batch reference")),
    }
}
fn resolve_draft(
    draft: &mut GardenDraft,
    resolved: &BTreeMap<String, ExternalReference>,
) -> Result<()> {
    fn replace(
        reference: &mut RecordRef,
        resolved: &BTreeMap<String, ExternalReference>,
    ) -> Result<()> {
        let r = resolve_ref(reference, resolved)?;
        *reference = RecordRef::Existing {
            kind: r.kind,
            id: r.id,
        };
        Ok(())
    }
    if let GardenDraft::Assertion {
        subject,
        object,
        evidence,
        ..
    } = draft
    {
        replace(subject, resolved)?;
        if let AssertionObject::EntityRef { entity_ref } = object {
            replace(entity_ref, resolved)?;
        }
        for reference in evidence {
            replace(reference, resolved)?;
        }
    }
    Ok(())
}
fn extensions(draft: &GardenDraft) -> &Extensions {
    match draft {
        GardenDraft::Entity { extensions, .. }
        | GardenDraft::Assertion { extensions, .. }
        | GardenDraft::Evidence { extensions, .. } => extensions,
    }
}
fn merge_extensions(previous: &GardenDraft, draft: &mut GardenDraft) {
    let target = match draft {
        GardenDraft::Entity { extensions, .. }
        | GardenDraft::Assertion { extensions, .. }
        | GardenDraft::Evidence { extensions, .. } => extensions,
    };
    for (namespace, extension) in extensions(previous) {
        target
            .entry(namespace.clone())
            .or_insert_with(|| extension.clone());
    }
}
fn validate_final(
    records: &BTreeMap<String, GardenRecord>,
    replacements: &BTreeMap<String, String>,
) -> Result<()> {
    for record in records
        .values()
        .filter(|r| r.metadata.lifecycle == Lifecycle::Active)
    {
        for reference in record.draft.references() {
            let RecordRef::Existing { kind, id } = reference else {
                return Err(failure(
                    GardenErrorCode::InvalidEndpoint,
                    "Unresolved graph reference",
                ));
            };
            if records.get(id).is_none_or(|r| {
                r.draft.kind() != *kind || r.metadata.lifecycle != Lifecycle::Active
            }) {
                return Err(failure(
                    GardenErrorCode::InvalidEndpoint,
                    "Active graph reference requires a matching active endpoint",
                ));
            }
        }
    }
    for (from, to) in replacements {
        let source = records.get(from).ok_or_else(not_found)?;
        let target = records
            .get(to)
            .filter(|r| r.draft.kind() == source.draft.kind())
            .ok_or_else(|| {
                failure(
                    GardenErrorCode::InvalidEndpoint,
                    "Replacement kind or existence mismatch",
                )
            })?;
        if target.metadata.lifecycle == Lifecycle::Retracted {
            return Err(failure(
                GardenErrorCode::InvalidEndpoint,
                "Replacement cannot be retracted",
            ));
        }
        let mut visited = BTreeSet::new();
        let mut cursor = from.as_str();
        while let Some(next) = replacements.get(cursor) {
            if !visited.insert(cursor) {
                return Err(failure(
                    GardenErrorCode::InvalidEndpoint,
                    "Lifecycle replacement cycle",
                ));
            }
            cursor = next;
        }
        if records
            .get(cursor)
            .is_none_or(|r| r.metadata.lifecycle != Lifecycle::Active)
        {
            return Err(failure(
                GardenErrorCode::InvalidEndpoint,
                "Supersession must reach an active replacement",
            ));
        }
    }
    Ok(())
}

struct PreparedBatch {
    state: State,
    original: BTreeMap<String, GardenRecord>,
    replacements: BTreeMap<String, String>,
    stored: StoredOperation,
    response: OutcomeResponse,
}
fn persist_batch(tx: &redb::WriteTransaction, key: &str, prepared: &PreparedBatch) -> Result<()> {
    for record in &prepared.stored.records {
        tx.open_table(CURRENT)
            .map_err(storage_error)?
            .insert(record.metadata.id.as_str(), encode(record)?.as_slice())
            .map_err(storage_error)?;
        history::append(tx, record)?;
        indexes::update(tx, prepared.original.get(&record.metadata.id), record)?;
    }
    {
        let mut table = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
        for (from, to) in &prepared.replacements {
            table
                .insert(from.as_str(), to.as_str())
                .map_err(storage_error)?;
            tx.open_table(REPLACEMENT_INCOMING)
                .map_err(storage_error)?
                .insert((to.as_str(), from.as_str()), 1)
                .map_err(storage_error)?;
        }
    }
    tx.open_table(RECEIPTS)
        .map_err(storage_error)?
        .insert(key, encode(&prepared.stored)?.as_slice())
        .map_err(storage_error)?;
    tx.open_table(META)
        .map_err(storage_error)?
        .insert("state", encode(&prepared.state)?.as_slice())
        .map_err(storage_error)?;
    Ok(())
}

fn prepare_grant(
    client: &str,
    scopes: &[String],
    access: &ClientAccess,
    garden: &GardenAccess,
) -> Result<(Auth, Principal)> {
    if !contracts::valid_client(client)
        || scopes.is_empty()
        || scopes
            .iter()
            .any(|s| !contracts::GARDEN_SCOPES.contains(&s.as_str()))
    {
        return Err(failure(
            GardenErrorCode::InvalidInput,
            "Explicit graph scopes and a valid client are required",
        ));
    }
    access
        .validate()
        .map_err(|_| failure(GardenErrorCode::InvalidInput, "Invalid client access"))?;
    garden
        .validate()
        .map_err(|_| failure(GardenErrorCode::InvalidInput, "Invalid graph access"))?;
    if access.mode == AccessMode::Shared
        && (matches!(garden, GardenAccess::All {}) || scopes.iter().any(|s| s == "garden:write"))
    {
        return Err(failure(
            GardenErrorCode::Forbidden,
            "Shared clients cannot receive graph write or full graph visibility",
        ));
    }
    let mut random = [0u8; 32];
    rand::rng().fill_bytes(&mut random);
    let token: String = random.iter().map(|b| format!("{b:02x}")).collect();
    let principal = Principal {
        token_hash: contracts::hash(&token),
        scopes: scopes.to_vec(),
        access: access.clone(),
        garden: garden.clone(),
    };
    Ok((
        Auth {
            client_id: client.into(),
            token,
        },
        principal,
    ))
}

#[path = "backup.rs"]
pub(crate) mod backup;
