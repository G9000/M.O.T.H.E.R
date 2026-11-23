//! Authenticated, credential-specific transfer inbox. The owner service serializes all
//! proposal operations. Only the owner CLI writes a decision into its private directory.
use super::{LocalClient, ServiceClientConfig};
use crate::{
    config,
    contracts::Auth,
    error::{Error, Result},
    garden::{config::domain_error, media_tools::AttachmentPlan, store::GardenStore},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub const PROPOSE: &str = "mother_propose_media_attachment";
pub const STATUS: &str = "mother_get_media_attachment";
pub const MAX_BYTES: usize = 2 * 1024 * 1024;
const FILE: &str = "selected-media";
const MAX_PENDING: usize = 24;
const MAX_ENTRIES: usize = 4096;

fn invalid() -> Error {
    Error::domain("INVALID_INPUT", "Invalid media attachment request.")
}
fn denied() -> Error {
    Error::domain(
        "FORBIDDEN",
        "The owner has not enabled this media bridge or approved this decision.",
    )
}
fn conflict() -> Error {
    Error::domain("CONFLICT", "The review or proposal has changed.")
}

/// A rotated credential or restored Garden has its own disabled inbox.
pub(super) fn root(
    endpoint: &Path,
    workspace: &str,
    generation: &str,
    client: &str,
    token: &str,
) -> Result<PathBuf> {
    let key = serde_json::to_vec(&(workspace, generation, client, token))?;
    Ok(endpoint
        .parent()
        .ok_or_else(invalid)?
        .join(format!("media-bridge-{:x}", Sha256::digest(key))))
}
fn client_root(config: &ServiceClientConfig) -> Result<PathBuf> {
    root(
        &config.endpoint_path,
        &config.workspace_id,
        &config.generation,
        &config.client_id,
        &config.token,
    )
}
pub(super) fn enabled(root: &Path) -> bool {
    root.join("enabled").is_file() && config::check_private_output(&root.join("enabled")).is_ok()
}
pub fn set_enabled(client: &LocalClient, enable: bool) -> Result<Value> {
    let root = client_root(client.config())?;
    if enable {
        crate::private_paths::owner_directory(root.parent().ok_or_else(invalid)?)?;
        let builder = fs::DirBuilder::new();
        #[cfg(unix)]
        let builder = {
            use std::os::unix::fs::DirBuilderExt;
            let mut private = builder;
            private.mode(0o700);
            private
        };
        match builder.create(&root) {
            Ok(()) => (),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(e.into()),
        }
        let marker = root.join("enabled");
        config::check_private_output(&marker)?;
        if !marker.exists() {
            publish(&marker, b"1")?;
        }
    } else if enabled(&root) {
        fs::remove_file(root.join("enabled"))?;
    }
    let caps = client.call("mother_get_capabilities", &json!({}))?;
    if caps["mediaAttachmentBridge"]["available"].as_bool() != Some(enable) {
        return Err(Error::domain(
            "RESTART_REQUIRED",
            "Install the new owner service and restart it to load the media bridge.",
        ));
    }
    Ok(json!({"enabled":enable,"clientId":client.config().client_id}))
}
fn publish(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = tempfile::NamedTempFile::new_in(path.parent().ok_or_else(invalid)?)?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    file.persist_noclobber(path).map_err(|e| e.error)?;
    Ok(())
}
fn proposal_dir(root: &Path, id: &str) -> Result<PathBuf> {
    let uuid = Uuid::parse_str(id).map_err(|_| invalid())?;
    if uuid.to_string() != id {
        return Err(invalid());
    }
    Ok(root.join(id))
}
fn field<'a>(args: &'a Value, key: &str) -> Result<&'a str> {
    args.get(key).and_then(Value::as_str).ok_or_else(invalid)
}
fn target(store: &GardenStore, auth: &Auth, id: &str) -> Result<Value> {
    let record = store.v1_call(auth, "mother_get_record", json!({"id":id}))?;
    if record["kind"] != "memory" || record["status"] != "active" {
        return Err(invalid());
    }
    Ok(record)
}
#[derive(Serialize, Deserialize)]
struct Entry {
    plan: AttachmentPlan,
}
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Decision {
    operation_id: String,
    approve: bool,
}
fn entry(dir: &Path) -> Result<Entry> {
    if !dir.is_dir() {
        return Err(Error::domain("NOT_FOUND", "Unknown attachment proposal."));
    }
    Ok(serde_json::from_value(config::read_json(
        &dir.join("entry.json"),
        32_768,
    )?)?)
}
fn read_decision(dir: &Path) -> Result<Decision> {
    Ok(serde_json::from_value(config::read_json(
        &dir.join("decision.json"),
        512,
    )?)?)
}

pub fn review(
    client: &LocalClient,
    proposal: &str,
    operation: &str,
    approve: bool,
) -> Result<Value> {
    let root = client_root(client.config())?;
    if !enabled(&root) {
        return Err(denied());
    }
    let args = json!({"proposalId":proposal});
    let current = client.call(STATUS, &args)?;
    if current["reviewId"] != operation {
        return Err(conflict());
    }
    let dir = proposal_dir(&root, proposal)?;
    let decision = Decision {
        operation_id: operation.to_owned(),
        approve,
    };
    let file = dir.join("decision.json");
    // An exclusive first decision survives lost acknowledgements and exact retries.
    if let Err(err) = publish(&file, &serde_json::to_vec(&decision)?) {
        if !file.is_file() {
            return Err(err);
        }
        if read_decision(&dir)? != decision {
            return Err(conflict());
        }
    }
    client.call("local_media_finish_review", &args)
}

fn enforce_quota(root: &Path) -> Result<()> {
    let mut pending = 0;
    let mut total = 0;
    for item in fs::read_dir(root)? {
        let item = item?;
        if item.file_type()?.is_dir() {
            total += 1;
            if item.path().join(FILE).exists() {
                pending += 1;
            }
        }
    }
    if pending >= MAX_PENDING || total >= MAX_ENTRIES {
        return Err(Error::domain(
            "LIMIT_EXCEEDED",
            "Owner media inbox is full.",
        ));
    }
    Ok(())
}

fn stage(store: &GardenStore, auth: &Auth, root: &Path, dir: &Path, args: &Value) -> Result<()> {
    let memory = field(args, "memoryId")?;
    let record = target(store, auth, memory)?;
    let source = field(args, "fileId")?;
    let name = args
        .get("fileName")
        .filter(|v| !v.is_null())
        .map(|v| v.as_str().ok_or_else(invalid))
        .transpose()?;
    if source.is_empty()
        || source.len() > 512
        || source.chars().any(char::is_control)
        || name.is_some_and(|n| n.len() > 255 || n.chars().any(char::is_control))
    {
        return Err(invalid());
    }
    let encoded = field(args, "content")?;
    if encoded.len() > MAX_BYTES.div_ceil(3) * 4 {
        return Err(invalid());
    }
    let bytes = base64::decode(encoded).map_err(|_| invalid())?;
    if bytes.is_empty() || bytes.len() > MAX_BYTES {
        return Err(invalid());
    }
    let media = crate::media::inspect(&bytes).map_err(|_| invalid())?;
    if let Some(mime) = args["mimeType"].as_str()
        && mime != "application/octet-stream"
        && mime != media.media_type
    {
        return Err(invalid());
    }
    if dir.is_dir() {
        let existing = entry(dir)?;
        if existing.plan.memory_id != memory
            || existing.plan.source_file_id.as_deref() != Some(source)
            || existing.plan.original_filename.as_deref() != name
            || existing.plan.sha256 != format!("{:x}", Sha256::digest(&bytes))
        {
            return Err(Error::domain(
                "OPERATION_REUSED",
                "Proposal UUID already belongs to another attachment.",
            ));
        }
        return Ok(());
    }
    if record["revision"] != args["expectedRevision"] {
        return Err(conflict());
    }
    enforce_quota(root)?;
    let temp = tempfile::Builder::new()
        .prefix("incoming-")
        .tempdir_in(root)?;
    publish(&temp.path().join(FILE), &bytes)?;
    let plan = store
        .plan_attachment(temp.path(), FILE, memory, Some(source.into()), false)
        .and_then(|plan| plan.bind_sender(&auth.client_id, name))
        .map_err(domain_error)?;
    if json!(plan.expected_revision) != args["expectedRevision"] {
        return Err(conflict());
    }
    publish(
        &temp.path().join("entry.json"),
        &serde_json::to_vec(&Entry { plan })?,
    )?;
    fs::rename(temp.path(), dir)?;
    Ok(())
}

fn finish(store: &GardenStore, dir: &Path, plan: &AttachmentPlan) -> Result<()> {
    if !dir.join("decision.json").is_file() {
        return Err(denied());
    }
    let decision = read_decision(dir)?;
    if decision.operation_id != plan.operation_id {
        return Err(conflict());
    }
    let outcome = dir.join("outcome.json");
    if !outcome.exists() {
        if decision.approve {
            if let Err(error) = store.apply_attachment(dir, plan, &decision.operation_id) {
                let error = domain_error(error);
                // A definitive stale revision permits the owner to discard this proposal.
                // Retain the decision after an uncertain write for exact reconciliation.
                if error.code() == "CONFLICT" {
                    fs::remove_file(dir.join("decision.json"))?;
                }
                return Err(error);
            }
            store.verify_stored_attachment(plan).map_err(domain_error)?;
        }
        publish(&outcome, &serde_json::to_vec(&decision)?)?;
    }
    match fs::remove_file(dir.join(FILE)) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub(super) fn dispatch(
    store: &GardenStore,
    auth: &Auth,
    root: &Path,
    method: &str,
    args: &Value,
) -> Result<Value> {
    if !enabled(root) {
        return Err(denied());
    }
    if method == "local_media_target" {
        let record = target(store, auth, field(args, "memoryId")?)?;
        return Ok(json!({"revision":record["revision"]}));
    }
    let id = field(args, "proposalId")?;
    let dir = proposal_dir(root, id)?;
    if method == "local_media_stage" {
        stage(store, auth, root, &dir, args)?;
    }
    let Entry { plan } = entry(&dir)?;
    // Fresh visibility checks also protect terminal outcomes after scoped access changes.
    store.v1_call(auth, "mother_get_record", json!({"id":plan.memory_id}))?;
    if method == "local_media_finish_review" {
        finish(store, &dir, &plan)?;
    }
    let outcome = dir.join("outcome.json");
    let (status, verification) = if outcome.exists() {
        let decision: Decision = serde_json::from_value(config::read_json(&outcome, 512)?)?;
        if decision.approve {
            (
                "approved",
                Some(
                    store
                        .verify_stored_attachment(&plan)
                        .map_err(domain_error)?,
                ),
            )
        } else {
            ("rejected", None)
        }
    } else if dir.join("decision.json").is_file() {
        ("owner_decision_recorded", None)
    } else {
        ("pending_owner_review", None)
    };
    Ok(
        json!({"proposalId":id,"reviewId":plan.operation_id,"status":status,"verification":verification,
        "memoryId":plan.memory_id,"memoryTitle":plan.memory_title,"expectedRevision":plan.expected_revision,
        "sourceFileId":plan.source_file_id,"fileName":plan.original_filename,"proposedBy":plan.proposed_by,"mediaType":plan.media_type,"sizeBytes":plan.size_bytes,
        "sha256":plan.sha256,"embeddedMetadata":plan.embedded_metadata,"allowOriginalInOwnerGarden":plan.allow_original_in_owner_garden}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::ClientAccess;

    #[test]
    fn staged_bytes_need_an_exact_local_decision_and_survive_receipt_replay() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("transfers");
        fs::create_dir(&root).unwrap();
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store =
            GardenStore::initialize(temp.path().join("synthetic.redb"), &workspace, &generation)
                .unwrap();
        store.v1_initialize("Synthetic owner.").unwrap();
        let writer = store
            .v1_grant(
                "synthetic",
                &["read".into(), "memory:write".into()],
                &ClientAccess::default(),
                None,
            )
            .unwrap();
        let write = json!({"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),
            "expectedRevision":0,"record":{"kind":"memory","title":"Synthetic clip","content":"Fixture.","status":"active","basis":"source"}});
        let memory = store
            .v1_call(&writer, "mother_write_record", write.clone())
            .unwrap()["record"]["id"]
            .clone();
        let bytes = include_bytes!("../../tests/fixtures/attachment-sample.mp4");
        let mut args = json!({"memoryId":memory,"expectedRevision":1,"fileId":"file-synthetic","fileName":"clip.mp4",
            "mimeType":"video/mp4","content":base64::encode(bytes)});
        let dir = root.join(Uuid::new_v4().to_string());
        stage(&store, &writer, &root, &dir, &args).unwrap();
        let plan = entry(&dir).unwrap().plan;
        assert!(store.verify_stored_attachment(&plan).is_err());
        assert_eq!(finish(&store, &dir, &plan).unwrap_err().code(), "FORBIDDEN");
        stage(&store, &writer, &root, &dir, &args).unwrap();
        args["fileId"] = json!("other-synthetic-file");
        assert_eq!(
            stage(&store, &writer, &root, &dir, &args)
                .unwrap_err()
                .code(),
            "OPERATION_REUSED"
        );
        let decision = Decision {
            operation_id: plan.operation_id.clone(),
            approve: true,
        };
        publish(
            &dir.join("decision.json"),
            &serde_json::to_vec(&decision).unwrap(),
        )
        .unwrap();
        finish(&store, &dir, &plan).unwrap();
        assert!(!dir.join(FILE).exists());
        assert_eq!(
            store.verify_stored_attachment(&plan).unwrap()["link"]["memoryId"],
            memory
        );
        finish(&store, &dir, &plan).unwrap();

        let stale = root.join(Uuid::new_v4().to_string());
        stage(&store, &writer, &root, &stale, &args).unwrap();
        let stale_plan = entry(&stale).unwrap().plan;
        let mut update = write;
        update["id"] = memory;
        update["operationId"] = json!(Uuid::new_v4().to_string());
        update["expectedRevision"] = json!(1);
        update["record"]["title"] = json!("Revised target");
        store
            .v1_call(&writer, "mother_write_record", update)
            .unwrap();
        let mut decision = Decision {
            operation_id: stale_plan.operation_id.clone(),
            approve: true,
        };
        publish(
            &stale.join("decision.json"),
            &serde_json::to_vec(&decision).unwrap(),
        )
        .unwrap();
        assert_eq!(
            finish(&store, &stale, &stale_plan).unwrap_err().code(),
            "CONFLICT"
        );
        decision.approve = false;
        publish(
            &stale.join("decision.json"),
            &serde_json::to_vec(&decision).unwrap(),
        )
        .unwrap();
        finish(&store, &stale, &stale_plan).unwrap();
        assert!(!stale.join(FILE).exists());
        assert!(store.verify_stored_attachment(&stale_plan).is_err());
    }
}
