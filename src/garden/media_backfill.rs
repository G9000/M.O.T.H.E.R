//! Owner-approved links from a v1 memory to native graph evidence.
use super::super::{RECEIPTS, StoredOperation, compat};
use super::*;
use std::{fs, path::Component};

const BINDING: &str = "mother.media:memory_binding";
const MAX_CAPTURE: u64 = 256 * 1024 * 1024;

/// A private review manifest for one file and one memory. The approval UUID binds every field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttachmentPlan {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub memory_id: String,
    pub memory_title: String,
    pub expected_revision: u64,
    pub local_path: String,
    pub source_file_id: Option<String>,
    pub sha256: String,
    pub size_bytes: u64,
    pub media_type: String,
    pub embedded_metadata: Vec<String>,
    pub allow_original_in_owner_garden: bool,
    pub operation_id: String,
}

impl AttachmentPlan {
    fn binding(&self) -> Result<Value> {
        let hash = Sha256::digest(serde_json::to_vec(self).map_err(storage_error)?);
        Ok(
            json!({"planHash":format!("{hash:x}"),"operationId":self.operation_id,
            "generation":self.generation,"memoryId":self.memory_id,"expectedRevision":self.expected_revision,"allowOriginal":self.allow_original_in_owner_garden}),
        )
    }
    fn approval_id(&self) -> Result<String> {
        let mut unsigned = self.clone();
        unsigned.operation_id.clear();
        Ok(content_uuid(
            &serde_json::to_vec(&unsigned).map_err(storage_error)?,
        ))
    }
}

fn content_uuid(content: &[u8]) -> String {
    let hash = Sha256::digest(content);
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hash[..16]);
    bytes[6] = (bytes[6] & 15) | 0x80;
    bytes[8] = (bytes[8] & 63) | 0x80;
    Uuid::from_bytes(bytes).to_string()
}

fn read_selection(root: &Path, relative: &str) -> Result<Vec<u8>> {
    let invalid_path = || invalid("Choose a regular file beneath the private import root");
    if relative.is_empty()
        || relative.len() > 4096
        || relative.contains([':', '\\', '\0'])
        || relative
            .split('/')
            .any(|p| p.is_empty() || matches!(p, "." | "..") || p.ends_with(['.', ' ']))
    {
        return Err(invalid_path());
    }
    #[cfg(windows)]
    if matches!(root.components().next(), Some(Component::Prefix(prefix))
        if !matches!(prefix.kind(), std::path::Prefix::Disk(_) | std::path::Prefix::VerbatimDisk(_)))
    {
        return Err(invalid_path());
    }
    let root = fs::canonicalize(root).map_err(|_| invalid_path())?;
    let mut path = root.clone();
    for component in Path::new(relative).components() {
        let Component::Normal(name) = component else {
            return Err(invalid_path());
        };
        path.push(name);
        let meta = fs::symlink_metadata(&path).map_err(|_| invalid_path())?;
        if meta.file_type().is_symlink() {
            return Err(invalid_path());
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            if meta.file_attributes() & 0x400 != 0 {
                return Err(invalid_path());
            }
        }
    }
    let resolved = fs::canonicalize(&path).map_err(|_| invalid_path())?;
    if !resolved.starts_with(&root) || !resolved.is_file() {
        return Err(invalid_path());
    }
    let file = File::open(resolved).map_err(|_| invalid_path())?;
    if file.metadata().map_err(|_| invalid_path())?.len() > MAX_CAPTURE {
        return Err(invalid(
            "Selected file exceeds the 256 MiB attachment limit",
        ));
    }
    let mut bytes = Vec::new();
    file.take(MAX_CAPTURE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid_path())?;
    if bytes.is_empty() || bytes.len() as u64 > MAX_CAPTURE {
        return Err(invalid("Invalid attachment length"));
    }
    Ok(bytes)
}

fn check_memory(record: &Value) -> Result<()> {
    if record["kind"] != "memory" || record["status"] != "active" {
        return Err(invalid("Choose an existing active v1 memory"));
    }
    Ok(())
}

/// Only direct owner operations may publish a reader binding. A client-supplied extension
/// or a reviewed generic graph proposal alone confers no such authority.
pub(super) fn memory_links(
    tx: &redb::ReadTransaction,
    evidence_ids: &[String],
) -> Result<Vec<Value>> {
    let current = tx.open_table(CURRENT).map_err(storage_error)?;
    let receipts = tx.open_table(RECEIPTS).map_err(storage_error)?;
    let mut links = Vec::new();
    for id in evidence_ids {
        let Some(row) = current.get(id.as_str()).map_err(storage_error)? else {
            continue;
        };
        let evidence: GardenRecord = decode(row.value())?;
        let GardenDraft::Evidence { extensions, .. } = &evidence.draft else {
            continue;
        };
        let Some(ext) = extensions.get(BINDING) else {
            continue;
        };
        if evidence.metadata.actor != inbox::OWNER
            || evidence.metadata.revision != 1
            || evidence.metadata.lifecycle != Lifecycle::Active
        {
            continue;
        }
        let b = &ext.payload;
        let (Some(op), Some(generation), Some(memory)) = (
            b["operationId"].as_str(),
            b["generation"].as_str(),
            b["memoryId"].as_str(),
        ) else {
            continue;
        };
        let key = operation_key(generation, inbox::OWNER, op);
        let Some(row) = receipts.get(key.as_str()).map_err(storage_error)? else {
            continue;
        };
        let stored: StoredOperation = decode(row.value())?;
        if stored.receipt.proposal_id.is_some()
            || !stored
                .receipt
                .results
                .iter()
                .any(|item| item.item_key == "media" && item.reference.id == *id)
            || !stored.records.contains(&evidence)
        {
            continue;
        }
        let Some(binding) = stored
            .receipt
            .results
            .iter()
            .find(|item| item.item_key == "binding")
        else {
            continue;
        };
        let Some(row) = current
            .get(binding.reference.id.as_str())
            .map_err(storage_error)?
        else {
            continue;
        };
        let assertion: GardenRecord = decode(row.value())?;
        if assertion.metadata.lifecycle != Lifecycle::Active
            || assertion.metadata.revision != binding.revision
        {
            continue;
        }
        let Some(row) = tx
            .open_table(compat::RECORDS)
            .map_err(storage_error)?
            .get(memory)
            .map_err(storage_error)?
        else {
            continue;
        };
        let record: Value = decode(row.value())?;
        if record["kind"] != "memory"
            || record["status"] != "active"
            || record["revision"].as_u64() < b["expectedRevision"].as_u64()
        {
            continue;
        }
        links.push(
            json!({"memoryId":memory,"evidenceId":id,"assertionId":binding.reference.id,
            "operationId":op,"approvedAt":stored.receipt.committed_at,
            "allowOriginal":b["allowOriginal"] == true,"acquisitionId":extensions.get(USE_EXTENSION).map(|e| &e.payload["acquisitionId"])}),
        );
    }
    Ok(links)
}

impl GardenStore {
    /// Preview file identity and the selected memory without a write transaction.
    /// # Errors
    /// Rejects missing, inactive or unsupported selections.
    pub fn plan_attachment(
        &self,
        root: &Path,
        file: &str,
        memory: &str,
        source: Option<String>,
        allow_original_in_owner_garden: bool,
    ) -> Result<AttachmentPlan> {
        if let Some(source) = &source {
            bounded(source, 512, "Invalid source file ID")?;
        }
        let bytes = read_selection(root, file)?;
        let found = crate::media::inspect(&bytes).map_err(invalid)?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let table = tx.open_table(compat::RECORDS).map_err(storage_error)?;
        let row = table
            .get(memory)
            .map_err(storage_error)?
            .ok_or_else(|| invalid("Unknown v1 memory"))?;
        let record: Value = decode(row.value())?;
        check_memory(&record)?;
        let mut plan = AttachmentPlan {
            version: 1,
            workspace_id: state.workspace_id,
            generation: state.generation,
            memory_id: memory.into(),
            memory_title: record["title"].as_str().unwrap_or_default().into(),
            expected_revision: record["revision"]
                .as_u64()
                .ok_or_else(|| invalid("Invalid memory revision"))?,
            local_path: file.into(),
            source_file_id: source,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size_bytes: bytes.len() as u64,
            media_type: found.media_type.into(),
            embedded_metadata: found.embedded.into_iter().map(str::to_owned).collect(),
            allow_original_in_owner_garden,
            operation_id: String::new(),
        };
        plan.operation_id = plan.approval_id()?;
        Ok(plan)
    }

    /// Retract the reviewed binding, preserving its evidence, original bytes and immutable receipt.
    /// # Errors
    /// Refuses other workspaces, stale generations, revised assertions and unrelated approvals.
    pub fn revoke_attachment(&self, plan: &AttachmentPlan, approval: &str) -> Result<Value> {
        if plan.version != 1 || approval != plan.operation_id || approval != plan.approval_id()? {
            return Err(invalid(
                "The approval must name this exact reviewed attachment",
            ));
        }
        let tx = self.begin()?;
        let state = state_write(&tx)?;
        envelope(&state, &plan.workspace_id, &plan.generation)?;
        let original_key = operation_key(&plan.generation, inbox::OWNER, approval);
        let fetch = |key: &str| -> Result<Option<StoredOperation>> {
            tx.open_table(RECEIPTS)
                .map_err(storage_error)?
                .get(key)
                .map_err(storage_error)?
                .map(|row| decode(row.value()))
                .transpose()
        };
        let original = fetch(&original_key)?
            .ok_or_else(|| invalid("No owner approval for this attachment"))?;
        let binding = original
            .receipt
            .results
            .iter()
            .find(|i| i.item_key == "binding")
            .ok_or_else(|| invalid("No attachment in this receipt"))?;
        let expected = plan.binding()?;
        if !original.records.iter().any(|record| matches!(&record.draft, GardenDraft::Evidence {extensions,..} if extensions.get(BINDING).is_some_and(|e| e.payload == expected))) {
            return Err(invalid("Receipt belongs to another operation"));
        }
        let op = content_uuid(format!("revoke-memory-attachment/{approval}").as_bytes());
        let key = operation_key(&plan.generation, inbox::OWNER, &op);
        if let Some(prior) = fetch(&key)? {
            if prior.request.items.len() != 1
                || prior.request.items[0].mutation != Mutation::Retract
                || prior.request.items[0].id.as_ref() != Some(&binding.reference)
            {
                return Err(failure(
                    GardenErrorCode::OperationReused,
                    "Revocation UUID already exists",
                ));
            }
            return Ok(json!({"status":"already_revoked","receipt":prior.receipt}));
        }
        let item = BatchItem {
            item_key: "binding".into(),
            mutation: Mutation::Retract,
            expected_revision: binding.revision,
            id: Some(binding.reference.clone()),
            draft: None,
            replacement: None,
        };
        let result = self.owner_apply_with_id(&tx, vec![item], op)?;
        tx.commit().map_err(storage_error)?;
        Ok(json!({"status":"revoked","receipt":result.receipt}))
    }

    /// Check the source file, stored bytes, authoritative receipt and active link.
    /// # Errors
    /// Reports stale generation, substituted files, missing assets or missing approval.
    pub fn verify_attachment(&self, root: &Path, plan: &AttachmentPlan) -> Result<Value> {
        if plan.version != 1 || plan.approval_id()? != plan.operation_id {
            return Err(invalid("Invalid review manifest"));
        }
        let bytes = read_selection(root, &plan.local_path)?;
        if format!("{:x}", Sha256::digest(&bytes)) != plan.sha256 {
            return Err(invalid("Selected bytes changed after review"));
        }
        drop(bytes);
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        envelope(&state, &plan.workspace_id, &plan.generation)?;
        let table = tx.open_table(ASSETS).map_err(storage_error)?;
        let asset = asset_read(&table, &plan.sha256)?
            .ok_or_else(|| invalid("Media has not been imported"))?;
        let links = memory_links(&tx, &uses(&tx, &asset)?)?;
        let link = links
            .into_iter()
            .find(|l| l["operationId"] == plan.operation_id && l["memoryId"] == plan.memory_id)
            .ok_or_else(|| invalid("No active owner-approved attachment for this manifest"))?;
        read_bytes(&tx, &plan.sha256)?;
        Ok(json!({"verified":true,"sha256":plan.sha256,"link":link}))
    }

    /// Commit a reviewed selection and retain the exact owner receipt for replay.
    /// # Errors
    /// Refuses changed bytes, manifest, workspace, generation, or memory revision.
    pub fn apply_attachment(
        &self,
        root: &Path,
        plan: &AttachmentPlan,
        approval: &str,
    ) -> Result<Value> {
        if plan.version != 1 || approval != plan.operation_id || approval != plan.approval_id()? {
            return Err(invalid(
                "The approval must name this exact reviewed manifest",
            ));
        }
        let tx = self.begin()?;
        let state = state_write(&tx)?;
        envelope(&state, &plan.workspace_id, &plan.generation)?;
        let key = operation_key(&state.generation, inbox::OWNER, approval);
        if let Some(stored) = tx
            .open_table(RECEIPTS)
            .map_err(storage_error)?
            .get(key.as_str())
            .map_err(storage_error)?
        {
            let original: StoredOperation = decode(stored.value())?;
            let expected = plan.binding()?;
            if !original.records.iter().any(|r| matches!(&r.draft, GardenDraft::Evidence {extensions,..} if extensions.get(BINDING).is_some_and(|e| e.payload == expected))) {
                return Err(failure(GardenErrorCode::OperationReused,"Operation UUID already belongs to another request"));
            }
            return Ok(json!({"status":"already_applied","receipt":original.receipt}));
        }
        let record: Value = {
            let table = tx.open_table(compat::RECORDS).map_err(storage_error)?;
            let row = table
                .get(plan.memory_id.as_str())
                .map_err(storage_error)?
                .ok_or_else(|| invalid("Unknown v1 memory"))?;
            decode(row.value())?
        };
        check_memory(&record)?;
        if record["revision"] != plan.expected_revision || record["title"] != plan.memory_title {
            return Err(failure(
                GardenErrorCode::Conflict,
                "Memory changed after review",
            ));
        }
        let bytes = read_selection(root, &plan.local_path)?;
        if plan.sha256 != format!("{:x}", Sha256::digest(&bytes))
            || plan.size_bytes != bytes.len() as u64
        {
            return Err(invalid("Selected bytes changed after review"));
        }
        let mut input = AcquisitionInput::new(MediaOrigin::Imported);
        input.original_filename = plan.local_path.rsplit('/').next().map(str::to_owned);
        input.source.clone_from(&plan.source_file_id);
        input.declared_media_type = Some(plan.media_type.clone());
        let report = ingest_tx(
            &tx,
            &bytes,
            &input,
            IngestOptions {
                max_bytes: MAX_CAPTURE,
                strip_metadata: true,
                ..Default::default()
            },
            &self.now(),
        )?;
        let mut evidence = evidence_draft(
            &tx,
            &report.sha256,
            Some(&report.acquisition_id),
            None,
            None,
        )?;
        if let GardenDraft::Evidence { extensions, .. } = &mut evidence {
            extensions.insert(
                BINDING.into(),
                Extension {
                    schema_version: 1,
                    payload: plan.binding()?,
                },
            );
        }
        let create = |key: &str, draft| BatchItem {
            item_key: key.into(),
            expected_revision: 0,
            mutation: Mutation::Create,
            id: None,
            draft: Some(draft),
            replacement: None,
        };
        let representation = GardenDraft::Entity {
            entity_type: "mother.memory:record".into(),
            display_name: "Owner-approved memory attachment".into(),
            aliases: vec![],
            extensions: BTreeMap::new(),
        };
        let assertion = GardenDraft::Assertion {
            subject: RecordRef::Item {
                kind: GardenKind::Entity,
                item_key: "memory".into(),
            },
            predicate: "mother.memory:has_attachment".into(),
            object: AssertionObject::Value {
                value: TypedValue::String {
                    value: plan.memory_id.clone(),
                },
            },
            valid_time: ValidTime {
                valid_from: None,
                valid_until: None,
                source_timezone: None,
                precision: TimePrecision::Unknown,
            },
            observed_time: ObservedTime {
                at: None,
                source_timezone: None,
                precision: TimePrecision::Unknown,
            },
            evidence: vec![RecordRef::Item {
                kind: GardenKind::Evidence,
                item_key: "media".into(),
            }],
            confidence: None,
            extensions: BTreeMap::new(),
        };
        let result = self.owner_apply_with_id(
            &tx,
            vec![
                create("memory", representation),
                create("media", evidence),
                create("binding", assertion),
            ],
            approval.into(),
        )?;
        tx.commit().map_err(storage_error)?;
        Ok(json!({"status":"applied","receipt":result.receipt}))
    }
}
