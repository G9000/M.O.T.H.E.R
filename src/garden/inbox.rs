//! Separate reviewed-capture tables and the trusted owner approval transaction.
use super::*;
use redb::ReadableTableMetadata;
use serde_json::{Value, json};

pub const OWNER_ACTOR: &str = "@owner-review";
pub(super) const OWNER: &str = OWNER_ACTOR;
const PROPOSALS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_inbox");
const PROPOSAL_OPERATIONS: TableDefinition<&str, &str> =
    TableDefinition::new("garden_inbox_operations");
const REVIEW_OPERATIONS: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_review_operations");
const OPERATION_FAMILIES: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_operation_families");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Proposal {
    id: String,
    revision: u64,
    proposer: String,
    principal: Option<Principal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    subject: Option<String>,
    original: BatchRequest,
    admission: OperationReceipt,
    outcome: OperationReceipt,
    reviewed_items: Option<Vec<BatchItem>>,
    records: Vec<GardenRecord>,
    blocked_error: Option<GardenError>,
}
impl Proposal {
    fn subject(&self) -> Option<&str> {
        self.subject.as_deref().or_else(|| {
            self.principal
                .as_ref()
                .and_then(|p| p.access.subject.as_deref())
        })
    }
}

/// Portable history retains attribution without carrying a credential or grant snapshot.
pub(super) fn portable_proposal(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut proposal: Proposal = decode(bytes)?;
    proposal.subject = proposal.subject().map(str::to_owned);
    proposal.principal = None;
    encode(&proposal)
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReviewRequest {
    version: u32,
    workspace_id: String,
    generation: String,
    operation_id: String,
    proposal_id: String,
    expected_revision: u64,
    action: ReviewAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reviewed_items: Option<Vec<BatchItem>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReviewAction {
    Approve,
    Dismiss,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewOperation {
    request: ReviewRequest,
    receipt: OperationReceipt,
    records: Vec<GardenRecord>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OperationFamily {
    actor: String,
    family: String,
}

pub(super) struct ReviewAttribution {
    pub original: BatchRequest,
    pub proposer: String,
    pub proposal_id: String,
}

pub(super) fn initialize(tx: &redb::WriteTransaction) -> Result<()> {
    tx.open_table(PROPOSALS).map_err(storage_error)?;
    tx.open_table(PROPOSAL_OPERATIONS).map_err(storage_error)?;
    tx.open_table(REVIEW_OPERATIONS).map_err(storage_error)?;
    let mut families = tx.open_table(OPERATION_FAMILIES).map_err(storage_error)?;
    let mut meta = tx.open_table(META).map_err(storage_error)?;
    if meta
        .get("inbox_registry_initialized")
        .map_err(storage_error)?
        .is_some()
    {
        return Ok(());
    }
    // Backfill the additive UUID registry once for pre-inbox canonical receipts.
    for entry in tx
        .open_table(RECEIPTS)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (_, bytes) = entry.map_err(storage_error)?;
        let operation: StoredOperation = decode(bytes.value())?;
        let key = family_key(
            &operation.receipt.operation_generation,
            &operation.receipt.actor,
            &operation.receipt.operation_id,
        );
        if families.get(key.as_str()).map_err(storage_error)?.is_none() {
            let family = if operation.receipt.proposal_id.is_some() {
                "review"
            } else {
                "apply"
            };
            families
                .insert(
                    key.as_str(),
                    encode(&OperationFamily {
                        actor: operation.receipt.actor,
                        family: family.into(),
                    })?
                    .as_slice(),
                )
                .map_err(storage_error)?;
        }
    }
    meta.insert("inbox_registry_initialized", &[1u8][..])
        .map_err(storage_error)?;
    Ok(())
}
fn family_key(generation: &str, actor: &str, operation: &str) -> String {
    operation_key(generation, actor, operation)
}
fn reused() -> GardenError {
    failure(
        GardenErrorCode::OperationReused,
        "Operation UUID was used by a different operation family, actor or payload",
    )
}
pub(super) fn check_operation(
    tx: &redb::WriteTransaction,
    generation: &str,
    operation: &str,
    actor: &str,
    family: &str,
) -> Result<()> {
    let key = family_key(generation, actor, operation);
    if let Some(bytes) = tx
        .open_table(OPERATION_FAMILIES)
        .map_err(storage_error)?
        .get(key.as_str())
        .map_err(storage_error)?
    {
        let stored: OperationFamily = decode(bytes.value())?;
        if stored.actor != actor || stored.family != family {
            return Err(reused());
        }
    }
    Ok(())
}
pub(super) fn claim_operation(
    tx: &redb::WriteTransaction,
    generation: &str,
    operation: &str,
    actor: &str,
    family: &str,
) -> Result<()> {
    check_operation(tx, generation, operation, actor, family)?;
    tx.open_table(OPERATION_FAMILIES)
        .map_err(storage_error)?
        .insert(
            family_key(generation, actor, operation).as_str(),
            encode(&OperationFamily {
                actor: actor.into(),
                family: family.into(),
            })?
            .as_slice(),
        )
        .map_err(storage_error)?;
    Ok(())
}
pub(super) fn is_proposal_operation(
    tx: &redb::ReadTransaction,
    generation: &str,
    operation: &str,
    actor: &str,
) -> Result<bool> {
    Ok(tx
        .open_table(PROPOSAL_OPERATIONS)
        .map_err(storage_error)?
        .get(operation_key(generation, actor, operation).as_str())
        .map_err(storage_error)?
        .is_some())
}
fn proposal_by_operation(
    tx: &redb::ReadTransaction,
    generation: &str,
    operation: &str,
    actor: &str,
) -> Result<Proposal> {
    let index = tx.open_table(PROPOSAL_OPERATIONS).map_err(storage_error)?;
    let id = index
        .get(operation_key(generation, actor, operation).as_str())
        .map_err(storage_error)?
        .ok_or_else(not_found)?;
    let proposals = tx.open_table(PROPOSALS).map_err(storage_error)?;
    decode(
        proposals
            .get(id.value())
            .map_err(storage_error)?
            .ok_or_else(not_found)?
            .value(),
    )
}
pub(super) fn owner_principal() -> Principal {
    Principal {
        token_hash: String::new(),
        scopes: vec!["garden:read".into(), "garden:write".into()],
        access: ClientAccess::default(),
        garden: GardenAccess::All {},
    }
}
fn validate_original_current(
    tx: &redb::WriteTransaction,
    principal: &Principal,
    request: &BatchRequest,
) -> Result<()> {
    authorize_request(principal, request)?;
    let (current, replacements) = context_write(tx, principal, request_seeds(request))?;
    for item in &request.items {
        if let Some(reference) = &item.id {
            let target = current
                .get(&reference.id)
                .filter(|r| r.draft.kind() == reference.kind)
                .ok_or_else(not_found)?;
            visible(principal, target, &current, &replacements)?;
        }
        let mut refs = item
            .draft
            .as_ref()
            .map_or_else(Vec::new, GardenDraft::references);
        refs.extend(item.replacement.as_ref());
        for reference in refs {
            if let RecordRef::Existing { kind, id } = reference {
                let target = current
                    .get(id)
                    .filter(|r| r.draft.kind() == *kind)
                    .ok_or_else(not_found)?;
                visible(principal, target, &current, &replacements)?;
            }
        }
    }
    Ok(())
}
/// Validate the owner's reviewed batch without changing original keys, semantic
/// kinds, target IDs, mutations or expected revisions. Edits never rebase a batch.
/// # Errors
/// Rejects malformed, oversized or lineage-changing reviewed items.
pub fn validate_reviewed_items(original: &BatchRequest, reviewed: &[BatchItem]) -> Result<()> {
    let mut batch = original.clone();
    batch.items = reviewed.to_vec();
    GardenRequest::parse(
        GardenTool::Propose,
        &serde_json::to_value(&batch).map_err(storage_error)?,
    )?;
    for old in &original.items {
        let item = reviewed
            .iter()
            .find(|item| item.item_key == old.item_key)
            .ok_or_else(|| {
                failure(
                    GardenErrorCode::InvalidInput,
                    "Every original itemKey must remain in the reviewed batch",
                )
            })?;
        if item.expected_revision != old.expected_revision
            || item.id != old.id
            || item.mutation != old.mutation
            || item.draft.as_ref().map(GardenDraft::kind)
                != old.draft.as_ref().map(GardenDraft::kind)
        {
            return Err(failure(
                GardenErrorCode::InvalidInput,
                "Review cannot change original kind, target, mutation or expectedRevision",
            ));
        }
    }
    Ok(())
}
fn no_commit_receipt(
    operation: &str,
    generation: &str,
    actor: &str,
    id: &str,
    status: OutcomeStatus,
) -> OperationReceipt {
    OperationReceipt {
        operation_id: operation.into(),
        operation_generation: generation.into(),
        actor: actor.into(),
        status,
        proposal_id: Some(id.into()),
        committed_at: None,
        commit_sequence: None,
        results: vec![],
    }
}
fn bounded_outcome(mut response: OutcomeResponse) -> Result<OutcomeResponse> {
    if bounded_json(&serde_json::to_value(&response).map_err(storage_error)?).is_err() {
        response.original_proposal = None;
        response.visible_records.clear();
        response.payload_omitted = true;
    }
    GardenResponse::parse(
        GardenTool::GetReceipt,
        &serde_json::to_value(&response).map_err(storage_error)?,
    )?;
    Ok(response)
}
pub(super) fn proposal_outcome_read(
    tx: &redb::ReadTransaction,
    state: &State,
    auth: &Auth,
    request: &OutcomeRequest,
    current_outcome: bool,
) -> Result<OutcomeResponse> {
    let principal = principal_read(tx, auth, GardenAction::Propose)?;
    let proposal = proposal_by_operation(
        tx,
        &request.operation_generation,
        &request.operation_id,
        &auth.client_id,
    )?;
    if proposal.proposer != auth.client_id
        || proposal.subject() != principal.access.subject.as_deref()
    {
        return Err(not_found());
    }
    if proposal.original.generation != state.generation {
        return Err(failure(
            GardenErrorCode::GenerationMismatch,
            "Proposal belongs to another generation",
        ));
    }
    let mut original_visible = authorize_request(&principal, &proposal.original).is_ok();
    if original_visible {
        original_visible = context_read(tx, &principal, request_seeds(&proposal.original)).is_ok();
    }
    let mut visible_records = vec![];
    if current_outcome {
        for record in &proposal.records {
            let mut seeds = record_seeds(record);
            seeds.push(record.metadata.id.clone());
            match context_read(tx, &principal, seeds).and_then(|(current, replacements)| {
                visible(&principal, record, &current, &replacements)
            }) {
                Ok(()) => visible_records.push(record.clone()),
                Err(error) if error.code == GardenErrorCode::NotFound => {}
                Err(error) => return Err(error),
            }
        }
    }
    let mut receipt = if current_outcome {
        proposal.outcome.clone()
    } else {
        proposal.admission.clone()
    };
    // The submitter reconciles its own admission UUID. The owner operation has a
    // separate immutable UUID and is available only through trusted owner APIs.
    receipt.operation_id = proposal.original.operation_id.clone();
    let payload_omitted =
        !original_visible || current_outcome && visible_records.len() != proposal.records.len();
    bounded_outcome(OutcomeResponse {
        version: 2,
        workspace_id: state.workspace_id.clone(),
        generation: state.generation.clone(),
        receipt,
        original_proposal: original_visible.then_some(proposal.original),
        visible_records,
        payload_omitted,
        visibility_epoch: epoch(state, &principal),
    })
}
impl GardenStore {
    /// Check a configured client against any current explicit graph scope.
    /// # Errors
    /// Rejects missing, revoked, invalid or scope-less graph credentials.
    pub fn authorize(&self, auth: &Auth) -> Result<()> {
        self.identity(auth).map(|_| ())
    }
    /// Read authenticated graph identity for trusted host startup.
    /// # Errors
    /// Requires at least one current graph scope and a valid credential.
    pub fn identity(&self, auth: &Auth) -> Result<Value> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let mut denied = None;
        for action in [
            GardenAction::Read,
            GardenAction::Propose,
            GardenAction::Write,
        ] {
            match principal_read(&tx, auth, action) {
                Ok(_) => {
                    return Ok(
                        json!({"workspaceId":state.workspace_id,"generation":state.generation}),
                    );
                }
                Err(error) if error.code == GardenErrorCode::Forbidden => denied = Some(error),
                Err(error) => return Err(error),
            }
        }
        Err(denied
            .unwrap_or_else(|| failure(GardenErrorCode::Forbidden, "Current graph scope required")))
    }
    /// Admit a bounded proposal into isolated inbox tables without canonical writes.
    /// # Errors
    /// Requires propose scope, current visible references and an unused operation UUID.
    pub fn propose(&self, auth: &Auth, request: &BatchRequest) -> Result<ProposalResponse> {
        GardenRequest::parse(
            GardenTool::Propose,
            &serde_json::to_value(request).map_err(storage_error)?,
        )?;
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let state = state_write(&tx)?;
        envelope(&state, &request.workspace_id, &request.generation)?;
        let principal = principal_write(&tx, auth, GardenAction::Propose)?;
        check_operation(
            &tx,
            &request.generation,
            &request.operation_id,
            &auth.client_id,
            "propose",
        )?;
        let key = operation_key(&request.generation, &auth.client_id, &request.operation_id);
        let existing = tx
            .open_table(PROPOSAL_OPERATIONS)
            .map_err(storage_error)?
            .get(key.as_str())
            .map_err(storage_error)?
            .map(|v| v.value().to_owned());
        let proposal = if let Some(id) = existing {
            let proposals = tx.open_table(PROPOSALS).map_err(storage_error)?;
            let bytes = proposals
                .get(id.as_str())
                .map_err(storage_error)?
                .ok_or_else(not_found)?;
            let proposal: Proposal = decode(bytes.value())?;
            if proposal.subject() != principal.access.subject.as_deref() {
                return Err(not_found());
            }
            if proposal.principal.is_none() {
                return Err(failure(
                    GardenErrorCode::Forbidden,
                    "Restored proposals require a fresh submission",
                ));
            }
            if proposal.original != *request {
                return Err(reused());
            }
            validate_original_current(&tx, &principal, request)?;
            proposal
        } else {
            validate_original_current(&tx, &principal, request)?;
            super::media::check_proposal(&tx, &principal, request)?;
            let id = Uuid::new_v4().to_string();
            let admission = no_commit_receipt(
                &request.operation_id,
                &request.generation,
                &auth.client_id,
                &id,
                OutcomeStatus::Pending,
            );
            let proposal = Proposal {
                id: id.clone(),
                revision: 1,
                proposer: auth.client_id.clone(),
                subject: principal.access.subject.clone(),
                principal: Some(principal.clone()),
                original: request.clone(),
                outcome: admission.clone(),
                admission,
                reviewed_items: None,
                records: vec![],
                blocked_error: None,
            };
            claim_operation(
                &tx,
                &request.generation,
                &request.operation_id,
                &auth.client_id,
                "propose",
            )?;
            tx.open_table(PROPOSALS)
                .map_err(storage_error)?
                .insert(id.as_str(), encode(&proposal)?.as_slice())
                .map_err(storage_error)?;
            tx.open_table(PROPOSAL_OPERATIONS)
                .map_err(storage_error)?
                .insert(key.as_str(), id.as_str())
                .map_err(storage_error)?;
            proposal
        };
        let response = ProposalResponse {
            version: 2,
            workspace_id: state.workspace_id.clone(),
            generation: state.generation.clone(),
            proposal_id: proposal.id,
            operation_id: request.operation_id.clone(),
            status: OutcomeStatus::Pending,
            visibility_epoch: epoch(&state, &principal),
        };
        GardenResponse::parse(
            GardenTool::Propose,
            &serde_json::to_value(&response).map_err(storage_error)?,
        )?;
        tx.commit().map_err(storage_error)?;
        Ok(response)
    }
    /// Read the caller's proposal current outcome using its admission operation UUID.
    /// # Errors
    /// Requires original subject, current propose scope and current generation.
    pub fn get_proposal(&self, auth: &Auth, request: &OutcomeRequest) -> Result<OutcomeResponse> {
        GardenRequest::parse(
            GardenTool::GetProposal,
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
        proposal_outcome_read(&tx, &state, auth, request, true)
    }
    /// List bounded proposal summaries for a trusted local owner surface.
    /// # Errors
    /// Returns storage errors; this API must never be exposed as an MCP tool.
    pub fn owner_inbox_list(&self) -> Result<Value> {
        self.owner_inbox_page(None)
    }
    /// Read the next bounded owner inbox page after an exclusive proposal UUID.
    /// # Errors
    /// Rejects malformed cursor UUIDs and unavailable private storage.
    pub fn owner_inbox_page(&self, cursor: Option<&str>) -> Result<Value> {
        if let Some(cursor) = cursor {
            contracts::id(cursor).map_err(|_| {
                failure(GardenErrorCode::InvalidInput, "Invalid owner inbox cursor")
            })?;
        }
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let table = tx.open_table(PROPOSALS).map_err(storage_error)?;
        let range = if let Some(cursor) = cursor {
            table
                .range::<&str>((
                    std::ops::Bound::Excluded(cursor),
                    std::ops::Bound::Unbounded,
                ))
                .map_err(storage_error)?
        } else {
            table.iter().map_err(storage_error)?
        };
        let mut proposals = vec![];
        let mut next_cursor = None;
        let total = table.len().map_err(storage_error)?;
        let mut has_more = false;
        for entry in range {
            let (_, bytes) = entry.map_err(storage_error)?;
            if proposals.len() == 100 {
                has_more = true;
                break;
            }
            let p: Proposal = decode(bytes.value())?;
            next_cursor = Some(p.id.clone());
            proposals.push(json!({"id":p.id,"revision":p.revision,"status":p.outcome.status,"generation":p.original.generation,"proposer":p.proposer,"itemCount":p.original.items.len(),"blockedError":p.blocked_error}));
        }
        if !has_more {
            next_cursor = None;
        }
        let omitted = total.saturating_sub(proposals.len() as u64);
        Ok(
            json!({"workspace":{"id":state.workspace_id,"generation":state.generation},"omitted":omitted,"nextCursor":next_cursor,"proposals":proposals}),
        )
    }
    /// Show original attribution, reviewed items and current target revisions to the owner.
    /// # Errors
    /// Rejects missing proposal IDs and storage failure; never expose this API to MCP.
    pub fn owner_inbox_get(&self, id: &str) -> Result<Value> {
        contracts::id(id)
            .map_err(|_| failure(GardenErrorCode::InvalidInput, "Invalid proposal UUID"))?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let table = tx.open_table(PROPOSALS).map_err(storage_error)?;
        let p: Proposal = decode(
            table
                .get(id)
                .map_err(storage_error)?
                .ok_or_else(not_found)?
                .value(),
        )?;
        let current = tx.open_table(CURRENT).map_err(storage_error)?;
        let mut targets = vec![];
        for item in &p.original.items {
            if let Some(reference) = &item.id {
                let record = current
                    .get(reference.id.as_str())
                    .map_err(storage_error)?
                    .map(|v| decode::<GardenRecord>(v.value()))
                    .transpose()?;
                targets.push(json!({"itemKey":item.item_key,"revision":record.as_ref().map(|r|r.metadata.revision),"lifecycle":record.as_ref().map(|r|r.metadata.lifecycle)}));
            }
        }
        let subject = p.subject().map(str::to_owned);
        Ok(
            json!({"id":p.id,"revision":p.revision,"status":p.outcome.status,"workspaceId":p.original.workspace_id,"generation":p.original.generation,"proposer":p.proposer,"subject":subject,"request":p.original,"original":p.original,"receipt":p.outcome,"admissionReceipt":p.admission,"reviewedItems":p.reviewed_items,"blockedError":p.blocked_error,"currentTargets":targets}),
        )
    }
    /// Atomically review a proposal as the trusted local owner.
    /// # Errors
    /// Rejects invalid review lineage, reused operations and stale proposal revisions.
    /// Current authorization/revision failures close the proposal as explicitly blocked.
    pub fn review_proposal(&self, value: Value) -> Result<OutcomeResponse> {
        self.review_with_hook(value, |_| {})
    }
    /// Instrument actual approval transaction barriers without exposing hooks over MCP.
    #[doc(hidden)]
    pub fn review_with_hook(
        &self,
        value: Value,
        hook: impl FnMut(CommitStage),
    ) -> Result<OutcomeResponse> {
        self.review_inner(value, hook, None)
            .map(|(outcome, _)| outcome)
    }
    /// Owner-only: approve a pending attachment proposal with the original file. Ingest,
    /// evidence, the assertion link and the proposal's closing receipt commit together.
    /// # Errors
    /// Any failure commits nothing and leaves the proposal pending.
    pub fn review_attachment(
        &self,
        value: Value,
        attachment: &super::media::AttachmentReview<'_>,
    ) -> Result<(OutcomeResponse, super::media::IngestReport)> {
        let (outcome, report) = self.review_inner(value, |_| {}, Some(attachment))?;
        Ok((
            outcome,
            report.ok_or_else(|| storage_error("attachment ingest missing"))?,
        ))
    }
    fn review_inner(
        &self,
        value: Value,
        mut hook: impl FnMut(CommitStage),
        attachment: Option<&super::media::AttachmentReview<'_>>,
    ) -> Result<(OutcomeResponse, Option<super::media::IngestReport>)> {
        bounded_json(&value)?;
        let request: ReviewRequest = serde_json::from_value(value).map_err(|_| {
            failure(
                GardenErrorCode::InvalidInput,
                "Invalid strict owner review request",
            )
        })?;
        for id in [
            &request.workspace_id,
            &request.generation,
            &request.operation_id,
            &request.proposal_id,
        ] {
            contracts::id(id)
                .map_err(|_| failure(GardenErrorCode::InvalidInput, "Invalid review UUID"))?;
        }
        if request.version != 2
            || request.expected_revision == 0
            || request.expected_revision > MAX_SAFE_INTEGER
            || request.action == ReviewAction::Dismiss && request.reviewed_items.is_some()
        {
            return Err(failure(
                GardenErrorCode::InvalidInput,
                "Invalid review version, revision or dismissal items",
            ));
        }
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let state = state_write(&tx)?;
        envelope(&state, &request.workspace_id, &request.generation)?;
        check_operation(
            &tx,
            &request.generation,
            &request.operation_id,
            OWNER,
            "review",
        )?;
        let key = operation_key(&request.generation, OWNER, &request.operation_id);
        let prior = tx
            .open_table(REVIEW_OPERATIONS)
            .map_err(storage_error)?
            .get(key.as_str())
            .map_err(storage_error)?
            .map(|v| decode::<ReviewOperation>(v.value()))
            .transpose()?;
        let owner = owner_principal();
        if let Some(prior) = prior {
            let mut comparable = prior.request.clone();
            if attachment.is_some() {
                comparable
                    .reviewed_items
                    .clone_from(&request.reviewed_items);
            }
            if comparable != request {
                return Err(reused());
            }
            return bounded_outcome(OutcomeResponse {
                version: 2,
                workspace_id: state.workspace_id.clone(),
                generation: state.generation.clone(),
                receipt: prior.receipt,
                original_proposal: None,
                visible_records: prior.records,
                payload_omitted: false,
                visibility_epoch: epoch(&state, &owner),
            })
            .map(|outcome| (outcome, None));
        }
        let mut proposal: Proposal = decode(
            tx.open_table(PROPOSALS)
                .map_err(storage_error)?
                .get(request.proposal_id.as_str())
                .map_err(storage_error)?
                .ok_or_else(not_found)?
                .value(),
        )?;
        if proposal.revision != request.expected_revision
            || proposal.outcome.status != OutcomeStatus::Pending
        {
            return Err(failure(
                GardenErrorCode::Conflict,
                "Proposal revision or pending status changed",
            ));
        }
        let mut owned = request
            .reviewed_items
            .clone()
            .unwrap_or_else(|| proposal.original.items.clone());
        validate_reviewed_items(&proposal.original, &owned)?;
        let mut pending = Vec::new();
        for (index, item) in owned.iter().enumerate() {
            if let Some(found) = super::media::pending_item(item)? {
                pending.push((index, found));
            }
        }
        let mut request = request;
        let mut ingest = None;
        if let Some(attachment) = attachment {
            let [(index, found)] = pending.as_slice() else {
                return Err(failure(
                    GardenErrorCode::InvalidInput,
                    "Attachment review needs exactly one pending attachment",
                ));
            };
            if request.action != ReviewAction::Approve {
                return Err(failure(
                    GardenErrorCode::InvalidInput,
                    "Attachment review approves; dismiss through the ordinary review",
                ));
            }
            for key in ["mother.media.link", "mother.media.description"] {
                if owned.iter().any(|item| item.item_key == key) {
                    return Err(failure(
                        GardenErrorCode::InvalidInput,
                        "Reserved media itemKey already used",
                    ));
                }
            }
            let now = self
                .clock
                .now()
                .to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
            let mut input = attachment.input.clone();
            if input.event_time.is_none() {
                input.event_time.clone_from(&found.event_time);
            }
            let report = super::media::ingest_file_tx(
                &tx,
                attachment.path,
                &input,
                attachment.options,
                &now,
            )?;
            let key = owned[*index].item_key.clone();
            owned[*index].draft = Some(super::media::evidence_draft(
                &tx,
                &report.sha256,
                Some(&report.acquisition_id),
                None,
                input.event_time.as_ref(),
            )?);
            owned.push(super::media::link_item(
                &tx,
                "mother.media.link",
                &found.target.id,
                found.expected_revision,
                &key,
            )?);
            if attachment.accept_description
                && let Some(description) = &found.description
            {
                owned.push(super::media::description_item(
                    &tx,
                    "mother.media.description",
                    &found.target.id,
                    &key,
                    description,
                    &proposal.proposer,
                )?);
            }
            validate_reviewed_items(&proposal.original, &owned)?;
            request.reviewed_items = Some(owned.clone());
            ingest = Some(report);
        } else if request.action == ReviewAction::Approve && !pending.is_empty() {
            return Err(failure(
                GardenErrorCode::InvalidInput,
                "This proposal awaits the owner's original image; complete it as an attachment review",
            ));
        }
        let reviewed = owned.as_slice();
        hook(CommitStage::BeforeWrite);
        let mut receipt = no_commit_receipt(
            &request.operation_id,
            &request.generation,
            OWNER,
            &proposal.id,
            OutcomeStatus::Dismissed,
        );
        let mut records = vec![];
        let mut result_state = state.clone();
        if request.action == ReviewAction::Approve {
            let validate = (|| -> Result<()> {
                envelope(
                    &state,
                    &proposal.original.workspace_id,
                    &proposal.original.generation,
                )?;
                let table = tx.open_table(PRINCIPALS).map_err(storage_error)?;
                let bytes = table
                    .get(proposal.proposer.as_str())
                    .map_err(storage_error)?
                    .ok_or_else(|| {
                        failure(
                            GardenErrorCode::Unauthorized,
                            "Original proposer grant was revoked",
                        )
                    })?;
                let current: Principal = decode(bytes.value())?;
                // A new token or changed policy is a new grant, never an implicit
                // renewal of the old proposal's authority.
                let original_principal = proposal.principal.as_ref().ok_or_else(|| {
                    failure(
                        GardenErrorCode::Forbidden,
                        "Restored proposals require a fresh submission",
                    )
                })?;
                if canonical_json(&current)? != canonical_json(original_principal)? {
                    return Err(failure(
                        GardenErrorCode::Forbidden,
                        "Original proposer grant or subject changed",
                    ));
                }
                if !current.scopes.iter().any(|s| s == "garden:propose") {
                    return Err(failure(
                        GardenErrorCode::Forbidden,
                        "Current proposer scope required",
                    ));
                }
                validate_original_current(&tx, &current, &proposal.original)
            })();
            let preparation = validate.and_then(|()| {
                let mut batch = proposal.original.clone();
                batch.operation_id = request.operation_id.clone();
                batch.items = reviewed.to_vec();
                let attribution = ReviewAttribution {
                    original: proposal.original.clone(),
                    proposer: proposal.proposer.clone(),
                    proposal_id: proposal.id.clone(),
                };
                self.prepare_batch(
                    &tx,
                    &batch,
                    &owner,
                    state.clone(),
                    Some(&attribution),
                    OWNER,
                )
            });
            match preparation {
                Ok(prepared) => {
                    result_state = prepared.state.clone();
                    receipt = prepared.stored.receipt.clone();
                    records = prepared.stored.records.clone();
                    persist_batch(&tx, &key, &prepared)?;
                }
                Err(error)
                    if attachment.is_none()
                        && error.code != GardenErrorCode::StoreUnavailable
                        && error.code != GardenErrorCode::RestartRequired =>
                {
                    receipt.status = OutcomeStatus::Blocked;
                    proposal.blocked_error = Some(error);
                }
                Err(error) => return Err(error),
            }
        }
        claim_operation_review(&tx, &request.generation, &request.operation_id)?;
        proposal.revision += 1;
        proposal.outcome = receipt.clone();
        proposal.records = records.clone();
        proposal.reviewed_items =
            (request.action == ReviewAction::Approve).then(|| reviewed.to_vec());
        receipt.validate()?;
        let operation = ReviewOperation {
            request: request.clone(),
            receipt: receipt.clone(),
            records: records.clone(),
        };
        let response = bounded_outcome(OutcomeResponse {
            version: 2,
            workspace_id: result_state.workspace_id.clone(),
            generation: result_state.generation.clone(),
            receipt,
            original_proposal: None,
            visible_records: records,
            payload_omitted: false,
            visibility_epoch: epoch(&result_state, &owner),
        })?;
        tx.open_table(PROPOSALS)
            .map_err(storage_error)?
            .insert(proposal.id.as_str(), encode(&proposal)?.as_slice())
            .map_err(storage_error)?;
        tx.open_table(REVIEW_OPERATIONS)
            .map_err(storage_error)?
            .insert(key.as_str(), encode(&operation)?.as_slice())
            .map_err(storage_error)?;
        hook(CommitStage::Staged);
        hook(CommitStage::BeforeCommit);
        tx.commit().map_err(storage_error)?;
        hook(CommitStage::Committed);
        Ok((response, ingest))
    }
}
fn claim_operation_review(
    tx: &redb::WriteTransaction,
    generation: &str,
    operation: &str,
) -> Result<()> {
    // Canonical approval has already staged its owner receipt; its identity is
    // deliberately the same review UUID, not a direct apply collision.
    tx.open_table(OPERATION_FAMILIES)
        .map_err(storage_error)?
        .insert(
            family_key(generation, OWNER, operation).as_str(),
            encode(&OperationFamily {
                actor: OWNER.into(),
                family: "review".into(),
            })?
            .as_slice(),
        )
        .map_err(storage_error)?;
    Ok(())
}
impl GardenStore {
    /// Provision an explicit private graph credential without replacing its output file.
    /// # Errors
    /// Ordinary staging failures roll back the grant and remove only the new file.
    /// A commit error retains the file for reconciliation of the uncertain grant.
    pub fn grant_to_file_with_access(
        &self,
        client: &str,
        scopes: &[String],
        path: &Path,
        database_path: &Path,
        access: &ClientAccess,
        garden: &GardenAccess,
    ) -> Result<()> {
        use std::io::Write;
        if !database_path.is_absolute() {
            return Err(failure(
                GardenErrorCode::InvalidInput,
                "Graph database path must be absolute",
            ));
        }
        let (auth, principal) = prepare_grant(client, scopes, access, garden)?;
        let config = super::super::config::GardenClientConfig {
            version: 2,
            backend: "garden_redb".into(),
            database_path: database_path.to_path_buf(),
            client_id: auth.client_id,
            token: auth.token,
        };
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let mut file = crate::config::create_private_file(path).map_err(storage_error)?;
        let staged = (|| -> Result<()> {
            tx.open_table(PRINCIPALS)
                .map_err(storage_error)?
                .insert(client, encode(&principal)?.as_slice())
                .map_err(storage_error)?;
            bump_epoch(&tx)?;
            serde_json::to_writer_pretty(&mut file, &config).map_err(storage_error)?;
            file.write_all(b"\n").map_err(storage_error)?;
            file.sync_all().map_err(storage_error)?;
            Ok(())
        })();
        drop(file);
        if let Err(error) = staged {
            drop(tx);
            let _ = std::fs::remove_file(path);
            return Err(error);
        }
        tx.commit().map_err(storage_error)
    }
}

/// Audit immutable admission, review and canonical mappings in the same snapshot
/// as the canonical/history/index audit. This never repairs stored provenance.
pub(super) fn audit_read(tx: &redb::ReadTransaction) -> Result<()> {
    let admitted_generations = indexes::admitted_generations(tx)?;
    let current_generation = state_read(tx)?.generation;
    let corrupt = || {
        failure(
            GardenErrorCode::StoreUnavailable,
            "Garden inbox consistency check failed",
        )
    };
    let families = tx.open_table(OPERATION_FAMILIES).map_err(storage_error)?;
    let canonical_table = tx.open_table(RECEIPTS).map_err(storage_error)?;
    let reviews_table = tx.open_table(REVIEW_OPERATIONS).map_err(storage_error)?;
    let admission_table = tx.open_table(PROPOSAL_OPERATIONS).map_err(storage_error)?;
    let require_family = |key: &str, actor: &str, family: &str| -> Result<()> {
        let stored: OperationFamily = decode(
            families
                .get(key)
                .map_err(storage_error)?
                .ok_or_else(corrupt)?
                .value(),
        )?;
        if stored.actor != actor || stored.family != family {
            return Err(corrupt());
        }
        Ok(())
    };
    let mut family_count = 0u64;
    super::compat::registry_visit(tx, |key, actor, family| {
        require_family(&key, &actor, &family)?;
        if canonical_table
            .get(key.as_str())
            .map_err(storage_error)?
            .is_some()
            || reviews_table
                .get(key.as_str())
                .map_err(storage_error)?
                .is_some()
            || admission_table
                .get(key.as_str())
                .map_err(storage_error)?
                .is_some()
        {
            return Err(corrupt());
        }
        family_count += 1;
        Ok(())
    })?;
    for entry in canonical_table.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let operation: StoredOperation = decode(bytes.value())?;
        let family = if operation.receipt.proposal_id.is_some() {
            "review"
        } else {
            "apply"
        };
        require_family(key.value(), &operation.receipt.actor, family)?;
        if family == "apply" {
            family_count += 1;
        }
    }
    let mut proposal_count = 0u64;
    let mut review_count = 0u64;
    for entry in tx
        .open_table(PROPOSALS)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (key, bytes) = entry.map_err(storage_error)?;
        let proposal: Proposal = decode(bytes.value())?;
        proposal.original.validate()?;
        proposal.admission.validate()?;
        proposal.outcome.validate()?;
        if !admitted_generations.contains(&proposal.original.generation) {
            return Err(corrupt());
        }
        if let Some(principal) = &proposal.principal {
            principal.access.validate().map_err(storage_error)?;
            principal.garden.validate().map_err(storage_error)?;
            if !contracts::hex_digest(&principal.token_hash)
                || !principal
                    .scopes
                    .iter()
                    .any(|scope| scope == "garden:propose")
                || proposal.subject() != principal.access.subject.as_deref()
            {
                return Err(corrupt());
            }
        } else if proposal.original.generation == current_generation {
            return Err(corrupt());
        }
        if key.value() != proposal.id
            || contracts::id(&proposal.id).is_err()
            || !contracts::valid_client(&proposal.proposer)
            || proposal.admission
                != no_commit_receipt(
                    &proposal.original.operation_id,
                    &proposal.original.generation,
                    &proposal.proposer,
                    &proposal.id,
                    OutcomeStatus::Pending,
                )
            || proposal.outcome.proposal_id.as_deref() != Some(proposal.id.as_str())
        {
            return Err(corrupt());
        }
        let admission_key = operation_key(
            &proposal.original.generation,
            &proposal.proposer,
            &proposal.original.operation_id,
        );
        if admission_table
            .get(admission_key.as_str())
            .map_err(storage_error)?
            .is_none_or(|id| id.value() != proposal.id)
        {
            return Err(corrupt());
        }
        require_family(&admission_key, &proposal.proposer, "propose")?;
        proposal_count += 1;
        family_count += 1;
        if proposal.outcome.status == OutcomeStatus::Pending {
            if proposal.revision != 1
                || proposal.outcome != proposal.admission
                || proposal.reviewed_items.is_some()
                || !proposal.records.is_empty()
                || proposal.blocked_error.is_some()
            {
                return Err(corrupt());
            }
            continue;
        }
        if proposal.revision != 2 || proposal.outcome.actor != OWNER {
            return Err(corrupt());
        }
        let review_key = operation_key(
            &proposal.outcome.operation_generation,
            OWNER,
            &proposal.outcome.operation_id,
        );
        let review: ReviewOperation = decode(
            reviews_table
                .get(review_key.as_str())
                .map_err(storage_error)?
                .ok_or_else(corrupt)?
                .value(),
        )?;
        if review.request.proposal_id != proposal.id
            || review.request.expected_revision != 1
            || review.request.operation_id != proposal.outcome.operation_id
            || review.request.generation != proposal.outcome.operation_generation
            || review.request.workspace_id != proposal.original.workspace_id
            || review.request.version != 2
            || review.receipt != proposal.outcome
            || review.records != proposal.records
        {
            return Err(corrupt());
        }
        require_family(&review_key, OWNER, "review")?;
        review_count += 1;
        family_count += 1;
        match proposal.outcome.status {
            OutcomeStatus::Approved => {
                let reviewed = proposal.reviewed_items.as_ref().ok_or_else(corrupt)?;
                validate_reviewed_items(&proposal.original, reviewed)?;
                if review.request.action != ReviewAction::Approve
                    || proposal.blocked_error.is_some()
                    || review
                        .request
                        .reviewed_items
                        .as_ref()
                        .unwrap_or(&proposal.original.items)
                        != reviewed
                {
                    return Err(corrupt());
                }
                let operation: StoredOperation = decode(
                    canonical_table
                        .get(review_key.as_str())
                        .map_err(storage_error)?
                        .ok_or_else(corrupt)?
                        .value(),
                )?;
                let mut request = proposal.original.clone();
                request.operation_id = review.request.operation_id.clone();
                request.items = reviewed.clone();
                if operation.request != request
                    || operation.receipt != proposal.outcome
                    || operation.records != proposal.records
                    || operation.subject.is_some()
                {
                    return Err(corrupt());
                }
                for item in reviewed {
                    let mapping = proposal
                        .outcome
                        .results
                        .iter()
                        .find(|mapping| mapping.item_key == item.item_key)
                        .ok_or_else(corrupt)?;
                    let original = proposal
                        .original
                        .items
                        .iter()
                        .find(|original| original.item_key == item.item_key);
                    let record = proposal
                        .records
                        .iter()
                        .find(|record| record.metadata.id == mapping.reference.id)
                        .ok_or_else(corrupt)?;
                    if mapping.owner_added != original.is_none()
                        || mapping.owner_edited != original.is_some_and(|original| original != item)
                        || record.metadata.actor != OWNER
                        || record.metadata.approver.as_deref() != Some(OWNER)
                        || item.mutation == Mutation::Create
                            && record.metadata.contributor
                                != if original.is_some() {
                                    proposal.proposer.as_str()
                                } else {
                                    OWNER
                                }
                    {
                        return Err(corrupt());
                    }
                }
            }
            OutcomeStatus::Dismissed => {
                if review.request.action != ReviewAction::Dismiss
                    || review.request.reviewed_items.is_some()
                    || proposal.reviewed_items.is_some()
                    || proposal.blocked_error.is_some()
                    || !proposal.records.is_empty()
                    || canonical_table
                        .get(review_key.as_str())
                        .map_err(storage_error)?
                        .is_some()
                {
                    return Err(corrupt());
                }
            }
            OutcomeStatus::Blocked => {
                let reviewed = proposal.reviewed_items.as_ref().ok_or_else(corrupt)?;
                validate_reviewed_items(&proposal.original, reviewed)?;
                if review.request.action != ReviewAction::Approve
                    || proposal.blocked_error.is_none()
                    || !proposal.records.is_empty()
                    || canonical_table
                        .get(review_key.as_str())
                        .map_err(storage_error)?
                        .is_some()
                    || review
                        .request
                        .reviewed_items
                        .as_ref()
                        .unwrap_or(&proposal.original.items)
                        != reviewed
                {
                    return Err(corrupt());
                }
            }
            _ => return Err(corrupt()),
        }
    }
    if admission_table.len().map_err(storage_error)? != proposal_count
        || families.len().map_err(storage_error)? != family_count
        || reviews_table.len().map_err(storage_error)? != review_count
    {
        return Err(corrupt());
    }
    Ok(())
}
