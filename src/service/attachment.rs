//! Bounded, credential-bound attachment staging. Only the local owner can publish a plan.
use super::{LocalClient, load_service_config};
use crate::{
    config,
    contracts::{self, Auth},
    error::{Error, Result, invalid},
    garden::{
        config::{GardenClientConfig, domain_error, load_garden_config},
        media_tools::AttachmentPlan,
        store::GardenStore,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};
use subtle::ConstantTimeEq;
use uuid::Uuid;

pub const SCOPE: &str = "media:attach";
pub const PREPARE: &str = "mother_media_attachment_prepare";
pub const UPLOAD: &str = "mother_media_attachment_upload";
pub const SUBMIT: &str = "mother_media_attachment_submit";
pub const STATUS: &str = "mother_media_attachment_status";
pub const CANCEL: &str = "mother_media_attachment_cancel";
const TOOLS: [&str; 5] = [PREPARE, UPLOAD, SUBMIT, STATUS, CANCEL];

pub const MAX_BYTES: u64 = 32 * 1024 * 1024;
pub const CHUNK: usize = 64 * 1024;
const MAX_GARDEN_BYTES: u64 = 256 * 1024 * 1024;
const MAX_ACTIVE: usize = 24;
const MAX_CLIENT: usize = 4;
const MAX_ENTRIES: usize = 4096;
const MAX_PREPARES_PER_MINUTE: usize = 8;
const BYTES_PER_SECOND: usize = 16 * 1024 * 1024;
const LIFETIME: u64 = 60 * 60;
const FILE: &str = "selected-media";
const MANIFEST: &str = "session.json";

static GATE: Mutex<Rate> = Mutex::new(Rate {
    second: 0,
    bytes: 0,
});

struct Rate {
    second: u64,
    bytes: usize,
}
impl Rate {
    fn charge(&mut self, bytes: usize) -> Result<()> {
        let second = now();
        if second != self.second {
            self.second = second;
            self.bytes = 0;
        }
        if self.bytes + bytes > BYTES_PER_SECOND {
            return Err(Error::domain(
                "RATE_LIMITED",
                "Retry the same chunk after one second.",
            ));
        }
        self.bytes += bytes;
        Ok(())
    }
}
fn lock() -> Result<MutexGuard<'static, Rate>> {
    GATE.lock().map_err(|_| {
        Error::domain(
            "STORE_UNAVAILABLE",
            "Restart the Garden owner and reconcile the operation UUID.",
        )
    })
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn denied() -> Error {
    Error::domain(
        "FORBIDDEN",
        "A current owner-issued read and media:attach grant is required.",
    )
}
fn missing() -> Error {
    Error::domain(
        "NOT_FOUND",
        "Unknown attachment operation for this credential.",
    )
}
fn conflict(message: &str) -> Error {
    Error::domain("CONFLICT", message)
}
fn source_unavailable() -> Error {
    Error::domain(
        "SOURCE_BYTES_UNAVAILABLE",
        "Open the local live Garden, choose Attachment review, and select the original file for this operation.",
    )
}

pub fn is_tool(name: &str) -> bool {
    TOOLS.contains(&name)
}
pub fn capabilities(store: &GardenStore, auth: &Auth) -> Value {
    let available =
        store.v1_authorize(auth, SCOPE).is_ok() && store.v1_authorize(auth, "read").is_ok();
    let host_file: &[&str] = if available {
        &[super::host_file::IMPORT]
    } else {
        &[]
    };
    let trust = store
        .trust_policy(&auth.client_id)
        .unwrap_or_default()
        .effective(available);
    json!({"available":available,"scope":SCOPE,"availableTools":if available {TOOLS.as_slice()} else {&[]},
        "trust":trust,
        "hostFileTools":host_file,
        "maxFileBytes":MAX_BYTES,"maxChunkBytes":CHUNK,"expiresAfterSeconds":LIFETIME,
        "requiresOriginalBytes":true,"ownerFileHandoff":true})
}

pub(super) fn root(database: &Path, workspace: &str, generation: &str) -> Result<PathBuf> {
    let identity = Sha256::digest(serde_json::to_vec(&(workspace, generation))?);
    Ok(database
        .parent()
        .ok_or_else(|| invalid("Missing private Garden directory"))?
        .join(format!("attachment-sessions-{identity:x}")))
}

pub trait AttachmentClient {
    fn request(&self, method: &str, args: &Value) -> Result<Value>;
}
impl AttachmentClient for LocalClient {
    fn request(&self, method: &str, args: &Value) -> Result<Value> {
        self.call(method, args)
    }
}
impl AttachmentClient for GardenClientConfig {
    fn request(&self, method: &str, args: &Value) -> Result<Value> {
        let store = GardenStore::open_active(&self.database_path)?;
        self.authorize(&store)?;
        let auth = self.auth();
        let identity = store.v1_identity(&auth)?;
        let root = root(
            &self.database_path,
            contracts::string(&identity, "workspaceId")?,
            contracts::string(&identity, "generation")?,
        )?;
        dispatch(&store, &auth, &root, method, args)
    }
}
pub fn client(path: &Path) -> Result<Box<dyn AttachmentClient>> {
    match config::read_json(path, 16_384)?["backend"].as_str() {
        Some("garden_service") => Ok(Box::new(LocalClient::new(load_service_config(path)?)?)),
        Some("garden_redb") => Ok(Box::new(load_garden_config(path)?)),
        _ => Err(invalid("Expected a native Garden config")),
    }
}

fn private_directory(path: &Path) -> Result<()> {
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    #[cfg(not(unix))]
    let builder = &mut builder;
    match builder.create(path) {
        // Only a directory created here is restricted; an existing one must already pass.
        Ok(()) => crate::private_paths::restrict_new_directory(path)?,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e.into()),
    }
    crate::private_paths::owner_directory(path)
}

fn private_temp(root: &Path) -> std::io::Result<tempfile::TempDir> {
    let mut builder = tempfile::Builder::new();
    builder.prefix("prepare-");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        builder.permissions(fs::Permissions::from_mode(0o700));
    }
    builder.tempdir_in(root)
}

fn uuid(id: &str) -> Result<&str> {
    if Uuid::parse_str(id).is_ok_and(|parsed| parsed.to_string() == id) {
        Ok(id)
    } else {
        Err(invalid("Expected a canonical UUID"))
    }
}
fn bounded(text: &str, max: usize) -> bool {
    !text.is_empty() && text.len() <= max && !text.chars().any(char::is_control)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FileSpec {
    file_name: String,
    size_bytes: u64,
    sha256: String,
    media_type: Option<String>,
}
impl FileSpec {
    fn validate(&self) -> Result<()> {
        if !bounded(&self.file_name, 255)
            || self.file_name.contains(['/', '\\', ':'])
            || !contracts::hex_digest(&self.sha256)
            || !(1..=MAX_BYTES).contains(&self.size_bytes)
            || self.media_type.as_deref().is_some_and(|mime| {
                !bounded(mime, 80)
                    || !(mime == "application/octet-stream"
                        || ["image/", "audio/", "video/"]
                            .iter()
                            .any(|prefix| mime.starts_with(prefix)))
            })
        {
            return Err(invalid("Invalid file name, size, SHA-256 or media type"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Association {
    Confirmed,
    Uncertain,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Initiation {
    OwnerRequest,
    Autonomous,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Prepare {
    operation_id: String,
    memory_id: String,
    expected_revision: u64,
    /// The agent's own claim about the memory match. Only "confirmed" can auto-commit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    association: Option<Association>,
    /// Whether the owner asked for this attachment or the agent started it itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    initiation: Option<Initiation>,
    source_locator: Option<String>,
    file: Option<FileSpec>,
}
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Uploading,
    PendingOwnerReview,
    OwnerDecisionRecorded,
    Published,
    Rejected,
    Cancelled,
    Expired,
    Stale,
    GrantRevoked,
}
impl Phase {
    fn pending(self) -> bool {
        matches!(
            self,
            Self::Uploading | Self::PendingOwnerReview | Self::OwnerDecisionRecorded
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Session {
    request: Prepare,
    client: String,
    credential_hash: String,
    transfer_token: String,
    title: String,
    created: u64,
    phase: Phase,
    owner_file: Option<FileSpec>,
    plan: Option<AttachmentPlan>,
    original_plan: Option<AttachmentPlan>,
    display: Option<String>,
    /// Why an otherwise complete submission still waits for the owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    review_reason: Option<String>,
}
impl Session {
    fn spec(&self) -> Option<&FileSpec> {
        self.request.file.as_ref().or(self.owner_file.as_ref())
    }
    fn expires(&self) -> u64 {
        self.created + LIFETIME
    }
    fn view(&self, dir: &Path) -> Result<Value> {
        let mut item = json!({"operationId":self.request.operation_id,"memoryId":self.request.memory_id,
            "memoryTitle":self.title,"expectedRevision":self.request.expected_revision,"proposedBy":self.client,
            "sourceLocator":self.request.source_locator,"sourceOrigin":if self.request.file.is_some() {"host_selected"} else {"owner_selected"},
            "file":self.spec(),"createdAt":self.created,"expiresAt":self.expires(),"status":self.phase,
            "receivedBytes":if self.phase.pending() {length(dir)?} else {0},
            "reviewId":self.plan.as_ref().map(|p| &p.operation_id),
            "originalReviewId":self.original_plan.as_ref().map(|p| &p.operation_id),
            "inspectedMediaType":self.plan.as_ref().map(|p| &p.media_type),
            "embeddedMetadata":self.plan.as_ref().map(|p| &p.embedded_metadata),"display":self.display});
        if self.request.file.is_none() && self.phase == Phase::Uploading {
            item["status"] = json!("source_bytes_unavailable");
            item["code"] = json!("SOURCE_BYTES_UNAVAILABLE");
            item["handoff"] = json!(
                "Open Attachment review in the local live Garden and select the original file for this operation."
            );
        }
        if let Some(reason) = &self.review_reason {
            item["reviewReason"] = json!(reason);
        }
        if let Some(plan) = &self.plan
            && matches!(self.phase, Phase::OwnerDecisionRecorded | Phase::Published)
        {
            item["authorization"] =
                json!({"mechanism": plan.mechanism(), "policy": plan.authorization});
        }
        if self.request.association.is_some() || self.request.initiation.is_some() {
            item["declared"] = json!({"association": self.request.association, "initiation": self.request.initiation});
        }
        Ok(item)
    }
}

fn save(dir: &Path, session: &Session) -> Result<()> {
    let path = dir.join(MANIFEST);
    crate::private_paths::single_link(&path)?;
    let mut file = tempfile::NamedTempFile::new_in(dir)?;
    serde_json::to_writer(file.as_file_mut(), session)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|e| e.error)?;
    Ok(())
}
fn load(dir: &Path) -> Result<Session> {
    if !dir.try_exists()? {
        return Err(missing());
    }
    crate::private_paths::owner_directory(dir)?;
    let manifest = dir.join(MANIFEST);
    crate::private_paths::single_link(&manifest)?;
    Ok(serde_json::from_value(config::read_json(
        &manifest,
        128 * 1024,
    )?)?)
}
fn length(dir: &Path) -> Result<u64> {
    let file = dir.join(FILE);
    crate::private_paths::single_link(&file)?;
    Ok(fs::metadata(file).map(|m| m.len()).or_else(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            Ok(0)
        } else {
            Err(e)
        }
    })?)
}
fn remove_bytes(dir: &Path) -> Result<()> {
    let file = dir.join(FILE);
    crate::private_paths::single_link(&file)?;
    match fs::remove_file(file) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}
fn close(dir: &Path, session: &mut Session, phase: Phase) -> Result<()> {
    session.phase = phase;
    save(dir, session)?;
    remove_bytes(dir)
}
fn outcome(store: &GardenStore, dir: &Path, session: &mut Session) -> Result<Option<Value>> {
    if matches!(
        session.phase,
        Phase::OwnerDecisionRecorded | Phase::Published
    ) {
        let plan = session
            .plan
            .as_ref()
            .ok_or_else(|| invalid("Missing review plan"))?;
        if let Some(receipt) = store.attachment_outcome(plan).map_err(domain_error)? {
            if session.phase == Phase::OwnerDecisionRecorded {
                session.phase = Phase::Published;
                save(dir, session)?;
            }
            remove_bytes(dir)?;
            let mut view = session.view(dir)?;
            view["status"] = json!(if receipt["verified"] == true {
                "approved"
            } else {
                "retracted"
            });
            view["verification"] = receipt;
            return Ok(Some(view));
        }
        if session.phase == Phase::Published {
            return Err(Error::domain(
                "STORE_UNAVAILABLE",
                "The published attachment receipt could not be verified.",
            ));
        }
    }
    if session.phase.pending() && now() >= session.expires() {
        close(dir, session, Phase::Expired)?;
    } else if !session.phase.pending() {
        remove_bytes(dir)?;
    }
    Ok(None)
}

fn sessions(root: &Path) -> Result<Vec<(PathBuf, Session)>> {
    if !root.try_exists()? {
        return Ok(Vec::new());
    }
    crate::private_paths::owner_directory(root)?;
    let mut result = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry
            .file_name()
            .to_str()
            .is_some_and(|id| uuid(id).is_ok())
        {
            let dir = entry.path();
            result.push((dir.clone(), load(&dir)?));
        }
    }
    Ok(result)
}

fn quotas(
    store: &GardenStore,
    root: &Path,
    sender: &str,
    additional_bytes: u64,
    preparing: bool,
) -> Result<()> {
    let mut reserved = additional_bytes;
    let mut pending = 0;
    let mut client_pending = 0;
    let mut recent = 0;
    let sessions = sessions(root)?;
    if preparing && sessions.len() >= MAX_ENTRIES {
        return Err(Error::domain(
            "RESOURCE_LIMIT",
            "Attachment history is full. Archive completed sessions locally.",
        ));
    }
    for (dir, mut session) in sessions {
        if session.client == sender && now().saturating_sub(session.created) < 60 {
            recent += 1;
        }
        if !session.phase.pending() {
            continue;
        }
        outcome(store, &dir, &mut session)?;
        if !session.phase.pending() {
            continue;
        }
        reserved += session.spec().map_or(0, |spec| spec.size_bytes);
        pending += 1;
        client_pending += usize::from(session.client == sender);
    }
    if preparing && recent >= MAX_PREPARES_PER_MINUTE {
        return Err(Error::domain(
            "RATE_LIMITED",
            "Retry preparation after one minute.",
        ));
    }
    if reserved > MAX_GARDEN_BYTES
        || preparing && (pending >= MAX_ACTIVE || client_pending >= MAX_CLIENT)
    {
        return Err(Error::domain(
            "RESOURCE_LIMIT",
            "The Garden's pending attachment quota is full.",
        ));
    }
    Ok(())
}

fn record(store: &GardenStore, session: &Session) -> Result<Value> {
    store.v1_attachment_record(
        &session.client,
        &session.credential_hash,
        &session.request.memory_id,
    )
}
fn check_target(value: &Value, revision: u64) -> Result<()> {
    if value["kind"] != "memory" || value["status"] != "active" || value["revision"] != revision {
        return Err(conflict(
            "The active target memory or its revision changed. Prepare a new operation.",
        ));
    }
    Ok(())
}

fn prepare(store: &GardenStore, auth: &Auth, root: &Path, args: &Value) -> Result<Value> {
    let request: Prepare = serde_json::from_value(args.clone())?;
    let id = uuid(&request.operation_id)?;
    uuid(&request.memory_id)?;
    if request.expected_revision == 0
        || request.source_locator.as_deref().is_some_and(|text| {
            !bounded(text, 480) || text.contains("://") || text.contains(['?', '#'])
        })
    {
        return Err(invalid("Invalid expected revision or source locator"));
    }
    if let Some(spec) = &request.file {
        spec.validate()?;
    }
    let dir = root.join(id);
    let credential_hash = contracts::hash(&auth.token);
    let mut session = if dir.try_exists()? {
        let prior = load(&dir)?;
        if prior.client != auth.client_id || prior.credential_hash != credential_hash {
            return Err(missing());
        }
        if prior.request != request {
            return Err(Error::domain(
                "OPERATION_REUSED",
                "This operation UUID already binds different inputs.",
            ));
        }
        record(store, &prior)?;
        prior
    } else {
        let target =
            store.v1_attachment_record(&auth.client_id, &credential_hash, &request.memory_id)?;
        check_target(&target, request.expected_revision)?;
        private_directory(root)?;
        quotas(
            store,
            root,
            &auth.client_id,
            request.file.as_ref().map_or(0, |spec| spec.size_bytes),
            true,
        )?;
        let session = Session {
            request,
            client: auth.client_id.clone(),
            credential_hash,
            transfer_token: Uuid::new_v4().to_string(),
            title: contracts::string(&target, "title")?.into(),
            created: now(),
            phase: Phase::Uploading,
            owner_file: None,
            plan: None,
            original_plan: None,
            display: None,
            review_reason: None,
        };
        let staging = private_temp(root)?;
        save(staging.path(), &session)?;
        fs::rename(staging.path(), &dir)?;
        session
    };
    if let Some(verified) = outcome(store, &dir, &mut session)? {
        return Ok(verified);
    }
    let mut view = session.view(&dir)?;
    if session.phase == Phase::Uploading && session.request.file.is_some() {
        view["transferToken"] = json!(session.transfer_token);
    }
    Ok(view)
}

fn append(rate: &mut Rate, dir: &Path, spec: &FileSpec, offset: u64, encoded: &str) -> Result<()> {
    if encoded.len() > CHUNK.div_ceil(3) * 4 {
        return Err(invalid("Chunk exceeds the 64 KiB bound"));
    }
    let bytes = base64::decode(encoded).map_err(|_| invalid("Expected a base64 chunk"))?;
    if bytes.is_empty()
        || bytes.len() > CHUNK
        || offset.saturating_add(bytes.len() as u64) > spec.size_bytes
    {
        return Err(invalid("Chunk exceeds the declared file length"));
    }
    rate.charge(bytes.len())?;
    let current = length(dir)?;
    if offset > current {
        return Err(conflict(
            "Upload offset is ahead of staged bytes. Read status and resume.",
        ));
    }
    let mut opts = OpenOptions::new();
    opts.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut file = opts.open(dir.join(FILE))?;
    file.seek(SeekFrom::Start(offset))?;
    let overlap = (current - offset).min(bytes.len() as u64) as usize;
    let mut previous = vec![0; overlap];
    file.read_exact(&mut previous)?;
    if previous != bytes[..overlap] {
        return Err(conflict("Retried bytes differ from staged bytes"));
    }
    file.write_all(&bytes[overlap..])?;
    file.sync_all()?;
    Ok(())
}

fn submit(store: &GardenStore, dir: &Path, session: &mut Session) -> Result<()> {
    if session.phase == Phase::PendingOwnerReview {
        return Ok(());
    }
    if session.phase != Phase::Uploading {
        return Err(conflict("The operation cannot accept a submission"));
    }
    let target = record(store, session)?;
    if let Err(error) = check_target(&target, session.request.expected_revision) {
        close(dir, session, Phase::Stale)?;
        return Err(error);
    }
    let spec = session.spec().ok_or_else(source_unavailable)?;
    if length(dir)? != spec.size_bytes {
        return Err(conflict(
            "The transfer is incomplete. Resume from the reported offset.",
        ));
    }
    let origin = if session.request.file.is_some() {
        "host-selected"
    } else {
        "owner-selected"
    };
    let source = session.request.source_locator.as_deref().map_or_else(
        || format!("{origin}:sha256:{}", spec.sha256),
        |locator| format!("{origin}:{locator}"),
    );
    let plan = store
        .plan_attachment(dir, FILE, &session.request.memory_id, Some(source), false)
        .and_then(|plan| {
            plan.bind_sender(
                &session.client,
                &spec.file_name,
                &session.request.operation_id,
            )
        })
        .map_err(domain_error)?;
    if plan.expected_revision != session.request.expected_revision {
        close(dir, session, Phase::Stale)?;
        return Err(conflict("Memory changed during submission"));
    }
    if plan.sha256 != spec.sha256
        || spec
            .media_type
            .as_deref()
            .is_some_and(|mime| mime != "application/octet-stream" && mime != plan.media_type)
    {
        return Err(invalid(
            "SHA-256 or inspected media type differs from the declared file. Cancel and prepare a new operation.",
        ));
    }
    session.display = Some(
        if plan.embedded_metadata.is_empty() {
            "original"
        } else if crate::media::inspect(&fs::read(dir.join(FILE))?)
            .map_err(invalid)?
            .strippable
        {
            "sanitized_derivative"
        } else {
            "withheld_embedded_metadata"
        }
        .into(),
    );
    if session.display.as_deref() == Some("withheld_embedded_metadata") {
        session.original_plan = Some(plan.clone().with_original_display().map_err(domain_error)?);
    }
    session.plan = Some(plan);
    session.phase = Phase::PendingOwnerReview;
    save(dir, session)
}

pub(super) fn dispatch(
    store: &GardenStore,
    auth: &Auth,
    root: &Path,
    name: &str,
    args: &Value,
) -> Result<Value> {
    if !is_tool(name) {
        return Err(invalid("Unknown attachment tool"));
    }
    store.v1_authorize(auth, SCOPE)?;
    store.v1_authorize(auth, "read")?;
    let mut rate = lock()?;
    if name == PREPARE {
        return prepare(store, auth, root, args);
    }
    contracts::strict(
        args,
        match name {
            UPLOAD => &["operationId", "transferToken", "offset", "contentBase64"],
            SUBMIT => &["operationId", "transferToken"],
            _ => &["operationId"],
        },
    )?;
    let dir = root.join(uuid(contracts::string(args, "operationId")?)?);
    let mut session = load(&dir)?;
    if session.client != auth.client_id || session.credential_hash != contracts::hash(&auth.token) {
        return Err(missing());
    }
    record(store, &session)?;
    if name == UPLOAD || name == SUBMIT {
        if session.request.file.is_none() {
            return Err(source_unavailable());
        }
        if !bool::from(
            session
                .transfer_token
                .as_bytes()
                .ct_eq(contracts::string(args, "transferToken")?.as_bytes()),
        ) {
            return Err(denied());
        }
    }
    if let Some(verified) = outcome(store, &dir, &mut session)? {
        return Ok(verified);
    }
    match name {
        UPLOAD if session.phase == Phase::Uploading => append(
            &mut rate,
            &dir,
            session.spec().ok_or_else(source_unavailable)?,
            contracts::integer(args, "offset", 0, MAX_BYTES)?,
            contracts::string(args, "contentBase64")?,
        )?,
        UPLOAD => return Err(conflict("This operation no longer accepts chunks")),
        SUBMIT => {
            let fresh = session.phase == Phase::Uploading;
            submit(store, &dir, &mut session)?;
            if fresh && session.phase == Phase::PendingOwnerReview {
                match trust_review_reason(store, auth, root, &session)? {
                    None => return commit_by_policy(store, auth, &dir, &mut session),
                    Some(reason) => {
                        session.review_reason = Some(reason.into());
                        save(&dir, &session)?;
                    }
                }
            }
        }
        CANCEL if session.phase.pending() => close(&dir, &mut session, Phase::Cancelled)?,
        _ => (),
    }
    session.view(&dir)
}

/// Same-origin, owner-session-only operations. These are deliberately absent from IPC and MCP.
pub fn owner_request(
    store: &GardenStore,
    root: &Path,
    action: &str,
    args: &Value,
) -> Result<Value> {
    let mut rate = lock()?;
    if action == "list" {
        let mut items = Vec::new();
        for (dir, mut session) in sessions(root)? {
            if session.phase.pending()
                && outcome(store, &dir, &mut session)?.is_none()
                && session.phase.pending()
            {
                items.push(session.view(&dir)?);
            }
        }
        return Ok(json!({"items":items,"maxFileBytes":MAX_BYTES,"maxChunkBytes":CHUNK}));
    }
    if action == "activity" {
        // Every recent operation, automatic or manual, with its recorded authorization.
        let mut all = sessions(root)?;
        all.sort_by_key(|(_, entry)| std::cmp::Reverse(entry.created));
        let mut items = Vec::new();
        for (dir, mut session) in all.into_iter().take(200) {
            items.push(match outcome(store, &dir, &mut session)? {
                Some(verified) => verified,
                None => session.view(&dir)?,
            });
        }
        return Ok(json!({"items":items}));
    }
    let dir = root.join(uuid(contracts::string(args, "operationId")?)?);
    let mut session = load(&dir)?;
    if action == "retract" {
        // Removes the active link only; the original asset, evidence and receipt remain.
        let plan = match (&session.phase, &session.plan) {
            (Phase::Published, Some(plan)) => plan.clone(),
            _ => return Err(conflict("Only a published attachment can be retracted")),
        };
        if args["reviewId"].as_str() != Some(plan.operation_id.as_str()) {
            return Err(conflict(
                "The displayed attachment changed. Refresh the activity.",
            ));
        }
        store
            .revoke_attachment(&plan, &plan.operation_id)
            .map_err(domain_error)?;
        return outcome(store, &dir, &mut session)?
            .ok_or_else(|| conflict("The retraction receipt is unavailable"));
    }
    if let Some(verified) = outcome(store, &dir, &mut session)? {
        return Ok(verified);
    }
    if !session.phase.pending() {
        return session.view(&dir);
    }
    if action == "reject" {
        if session.plan.as_ref().map(|plan| plan.operation_id.as_str()) != args["reviewId"].as_str()
        {
            return Err(conflict(
                "The displayed review changed. Refresh the proposal.",
            ));
        }
        close(&dir, &mut session, Phase::Rejected)?;
        return session.view(&dir);
    }
    let target = match record(store, &session) {
        Ok(target) => target,
        Err(error) if matches!(error.code(), "UNAUTHORIZED" | "FORBIDDEN" | "NOT_FOUND") => {
            close(&dir, &mut session, Phase::GrantRevoked)?;
            return session.view(&dir);
        }
        Err(error) => return Err(error),
    };
    if check_target(&target, session.request.expected_revision).is_err() {
        close(&dir, &mut session, Phase::Stale)?;
        return session.view(&dir);
    }
    match action {
        "choose" | "upload" | "submit"
            if session.request.file.is_none() && session.phase == Phase::Uploading =>
        {
            match action {
                "choose" => {
                    let spec: FileSpec = serde_json::from_value(args["file"].clone())?;
                    spec.validate()?;
                    if let Some(prior) = &session.owner_file {
                        if prior != &spec {
                            return Err(conflict(
                                "Choose the same original file or reject this operation",
                            ));
                        }
                    } else {
                        quotas(store, root, &session.client, spec.size_bytes, false)?;
                        session.owner_file = Some(spec);
                        save(&dir, &session)?;
                    }
                }
                "upload" => append(
                    &mut rate,
                    &dir,
                    session.spec().ok_or_else(source_unavailable)?,
                    contracts::integer(args, "offset", 0, MAX_BYTES)?,
                    contracts::string(args, "contentBase64")?,
                )?,
                _ => submit(store, &dir, &mut session)?,
            }
        }
        "approve"
            if matches!(
                session.phase,
                Phase::PendingOwnerReview | Phase::OwnerDecisionRecorded
            ) =>
        {
            let review = contracts::string(args, "reviewId")?;
            if session.phase == Phase::PendingOwnerReview
                && session
                    .original_plan
                    .as_ref()
                    .is_some_and(|plan| plan.operation_id == review)
            {
                session.plan = session.original_plan.take();
                session.display = Some("owner_approved_original".into());
            }
            let plan = session
                .plan
                .as_ref()
                .ok_or_else(|| invalid("Missing review plan"))?;
            if review != plan.operation_id {
                return Err(conflict(
                    "The displayed review changed. Refresh the proposal.",
                ));
            }
            let review = review.to_owned();
            return commit(store, &dir, &mut session, &review);
        }
        _ => {
            return Err(conflict(
                "This action is unavailable in the current attachment state",
            ));
        }
    }
    session.view(&dir)
}

/// The single canonical commit used by owner approval and by owner-granted trust.
fn commit(store: &GardenStore, dir: &Path, session: &mut Session, review: &str) -> Result<Value> {
    let plan = session
        .plan
        .clone()
        .ok_or_else(|| invalid("Missing review plan"))?;
    session.original_plan = None;
    session.phase = Phase::OwnerDecisionRecorded;
    save(dir, session)?;
    match store
        .apply_attachment(dir, &plan, review)
        .map_err(domain_error)
    {
        Ok(_) => {}
        Err(error) if error.code() == "CONFLICT" => {
            close(dir, session, Phase::Stale)?;
            return session.view(dir);
        }
        Err(error) => return Err(error),
    }
    outcome(store, dir, session)?.ok_or_else(|| conflict("A verified receipt is unavailable"))
}

const DAY: u64 = 24 * 60 * 60;

/// None when the owner's current trust policy authorizes this exact submission;
/// otherwise the reason it stays in owner review. Everything is re-read now, so a
/// policy or grant revoked a moment ago already applies.
fn trust_review_reason(
    store: &GardenStore,
    auth: &Auth,
    root: &Path,
    session: &Session,
) -> Result<Option<&'static str>> {
    let policy = store.trust_policy(&auth.client_id)?;
    if !policy.automatic() {
        return Ok(Some("trust_policy_manual_review"));
    }
    if store.v1_authorize(auth, SCOPE).is_err()
        || store.v1_authorize(auth, "read").is_err()
        || !store.trust_eligible(&auth.client_id)?
    {
        return Ok(Some("credential_not_eligible"));
    }
    if session.display.as_deref() == Some("withheld_embedded_metadata")
        || session.original_plan.is_some()
    {
        return Ok(Some("embedded_metadata_display"));
    }
    if session.request.association != Some(Association::Confirmed) {
        return Ok(Some("association_review_required"));
    }
    if session.request.initiation == Some(Initiation::Autonomous)
        && policy.mode != crate::garden::store::trust::TrustMode::AutonomousGardening
    {
        return Ok(Some("autonomous_work_not_permitted"));
    }
    let size = session.spec().map_or(0, |spec| spec.size_bytes);
    let since = now().saturating_sub(DAY);
    let (mut count, mut bytes) = (0u32, 0u64);
    for (_, other) in sessions(root)? {
        if other.client == auth.client_id
            && other.created >= since
            && matches!(other.phase, Phase::OwnerDecisionRecorded | Phase::Published)
            && other
                .plan
                .as_ref()
                .is_some_and(|plan| plan.authorization.is_some())
        {
            count += 1;
            bytes += other.spec().map_or(0, |spec| spec.size_bytes);
        }
    }
    if count >= policy.max_auto_commits_per_day {
        return Ok(Some("daily_commit_limit"));
    }
    if bytes + size > policy.max_auto_bytes_per_day {
        return Ok(Some("daily_byte_limit"));
    }
    Ok(None)
}

fn commit_by_policy(
    store: &GardenStore,
    auth: &Auth,
    dir: &Path,
    session: &mut Session,
) -> Result<Value> {
    let policy = store.trust_policy(&auth.client_id)?;
    let plan = session
        .plan
        .clone()
        .ok_or_else(|| invalid("Missing review plan"))?
        .with_authorization(crate::garden::media_tools::AttachmentAuthorization {
            mechanism: "owner_granted_trust_policy".into(),
            client_id: auth.client_id.clone(),
            trust_mode: serde_json::to_value(policy.mode)?
                .as_str()
                .unwrap_or_default()
                .into(),
            policy_revision: policy.revision,
            decided_at: chrono::Utc::now().to_rfc3339(),
        })
        .map_err(domain_error)?;
    let review = plan.operation_id.clone();
    session.plan = Some(plan);
    commit(store, dir, session, &review)
}

/// Reconcile interrupted publications and remove staged bytes after expiry.
pub(super) fn sweep(store: &GardenStore, root: &Path) -> Result<()> {
    let _guard = lock()?;
    for (dir, mut session) in sessions(root)? {
        if !session.phase.pending() {
            remove_bytes(&dir)?;
        } else if now() >= session.expires() {
            outcome(store, &dir, &mut session)?;
        }
    }
    Ok(())
}

/// Stream an explicitly selected local original without sending its bytes through model context.
pub fn send_file(client: &dyn AttachmentClient, path: &Path, mut args: Value) -> Result<Value> {
    if !fs::metadata(path)?.is_file() {
        return Err(invalid("Select a regular local file"));
    }
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| invalid("Expected a file name"))?
        .to_owned();
    args["file"] = json!({"fileName":name});
    send_original(client, &mut File::open(path)?, args)
}

/// Hash, declare and stream original bytes through the ordinary chunked protocol.
///
/// `args["file"]` supplies the name and optional media type; size and digest
/// always come from the bytes themselves. An exact retry resumes at the
/// owner's `receivedBytes`, and a non-uploading state is returned unchanged.
pub fn send_original<R: Read + Seek>(
    client: &dyn AttachmentClient,
    file: &mut R,
    mut args: Value,
) -> Result<Value> {
    let mut digest = Sha256::new();
    file.seek(SeekFrom::Start(0))?;
    let size = std::io::copy(&mut file.by_ref().take(MAX_BYTES + 1), &mut digest)?;
    if size == 0 || size > MAX_BYTES {
        return Err(Error::domain(
            "LIMIT_EXCEEDED",
            "The original must be between one byte and the attachment size limit.",
        ));
    }
    args["file"]["sizeBytes"] = json!(size);
    args["file"]["sha256"] = json!(format!("{:x}", digest.finalize()));
    let state = client.request(PREPARE, &args)?;
    let Some(token) = state["transferToken"].as_str() else {
        return Ok(state);
    };
    let mut offset = contracts::integer(&state, "receivedBytes", 0, size)?;
    file.seek(SeekFrom::Start(offset))?;
    let id = &args["operationId"];
    let mut chunk = vec![0; CHUNK];
    while offset < size {
        let count = (size - offset).min(CHUNK as u64) as usize;
        file.read_exact(&mut chunk[..count])?;
        let payload = json!({"operationId":id,"transferToken":token,"offset":offset,"contentBase64":base64::encode(&chunk[..count])});
        let result = match client.request(UPLOAD, &payload) {
            Err(error) if error.code() == "RATE_LIMITED" => {
                std::thread::sleep(std::time::Duration::from_secs(1));
                client.request(UPLOAD, &payload)?
            }
            other => other?,
        };
        offset = contracts::integer(&result, "receivedBytes", offset + 1, size)?;
    }
    client.request(SUBMIT, &json!({"operationId":id,"transferToken":token}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::ClientAccess;

    struct Fixture {
        _directory: tempfile::TempDir,
        root: PathBuf,
        store: GardenStore,
        writer: Auth,
        transfer: Auth,
        reader: Auth,
        other: Auth,
        ws: String,
        generation: String,
    }
    impl Fixture {
        fn new() -> Self {
            let directory = private_temp(&fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
            let ws = Uuid::new_v4().to_string();
            let generation = Uuid::new_v4().to_string();
            let path = directory.path().join("garden.redb");
            let store = GardenStore::initialize(&path, &ws, &generation).unwrap();
            store.v1_initialize("Synthetic attachment owner").unwrap();
            let grant = |id, scopes: &[&str]| {
                store
                    .v1_grant(
                        id,
                        &scopes
                            .iter()
                            .map(|scope| scope.to_string())
                            .collect::<Vec<_>>(),
                        &ClientAccess::default(),
                        None,
                    )
                    .unwrap()
            };
            let writer = grant("writer", &["read", "memory:write"]);
            let transfer = grant("codex-test", &["read", SCOPE]);
            let reader = grant("reader", &["read"]);
            let other = grant("claude-test", &["read", SCOPE]);
            Self {
                root: root(&path, &ws, &generation).unwrap(),
                _directory: directory,
                store,
                writer,
                transfer,
                reader,
                other,
                ws,
                generation,
            }
        }
        fn write(&self, prior: Option<&Value>) -> Value {
            let item = json!({"kind":"memory","title":"Synthetic memory","content":"Synthetic content",
                "status":"active","basis":"source"});
            let mut args = json!({
                "workspaceId":self.ws,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),
                "expectedRevision":prior.map_or(json!(0), |p| p["revision"].clone()),"record":item
            });
            if let Some(prior) = prior {
                args["id"] = prior["id"].clone();
                args["record"]["content"] = json!("Updated synthetic content");
            }
            self.store
                .v1_call(&self.writer, "mother_write_record", args)
                .unwrap()["record"]
                .clone()
        }
        fn start(&self, target: &Value, bytes: &[u8]) -> Value {
            json!({"operationId":Uuid::new_v4().to_string(),"memoryId":target["id"],"expectedRevision":target["revision"],
                "file":{"fileName":"selected.mp4","sizeBytes":bytes.len(),"sha256":format!("{:x}",Sha256::digest(bytes)),"mediaType":"video/mp4"}})
        }
        fn call(&self, name: &str, args: Value) -> Result<Value> {
            dispatch(&self.store, &self.transfer, &self.root, name, &args)
        }
        fn owner(&self, name: &str, args: &Value) -> Result<Value> {
            owner_request(&self.store, &self.root, name, args)
        }
        fn send(&self, id: &Value, token: &Value, bytes: &[u8]) -> Result<Value> {
            for (index, chunk) in bytes.chunks(CHUNK).enumerate() {
                self.call(UPLOAD,json!({"operationId":id,"transferToken":token,"offset":index*CHUNK,"contentBase64":base64::encode(chunk)}))?;
            }
            self.call(SUBMIT, json!({"operationId":id,"transferToken":token}))
        }
    }

    fn video() -> Vec<u8> {
        let mut clip = include_bytes!("../../tests/fixtures/attachment-sample.mp4").to_vec();
        let total = 2 * 1024 * 1024 + 719;
        clip.extend_from_slice(&((total - clip.len()) as u32).to_be_bytes());
        clip.extend_from_slice(b"free");
        clip.resize(total, 0);
        clip
    }

    #[test]
    fn chunks_and_owner_approval_publish_an_active_verified_receipt() {
        let f = Fixture::new();
        let target = f.write(None);
        let bytes = video();
        let args = f.start(&target, &bytes);
        let id = &args["operationId"];
        let query = json!({"operationId":id});
        assert_eq!(
            dispatch(&f.store, &f.reader, &f.root, PREPARE, &args)
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
        let prepared = f.call(PREPARE, args.clone()).unwrap();
        let token = &prepared["transferToken"];
        assert_eq!(f.call(PREPARE, args.clone()).unwrap(), prepared);
        assert_eq!(
            dispatch(&f.store, &f.other, &f.root, STATUS, &query)
                .unwrap_err()
                .code(),
            "NOT_FOUND"
        );
        let first = json!({"operationId":id,"transferToken":token,"offset":0,"contentBase64":base64::encode(&bytes[..CHUNK])});
        let mut wrong = first.clone();
        wrong["transferToken"] = json!(Uuid::new_v4().to_string());
        assert_eq!(f.call(UPLOAD, wrong).unwrap_err().code(), "FORBIDDEN");
        assert_eq!(
            f.call(UPLOAD, first.clone()).unwrap()["receivedBytes"],
            CHUNK
        );
        assert_eq!(f.call(UPLOAD, first).unwrap()["receivedBytes"], CHUNK);
        let changed =
            json!({"operationId":id,"transferToken":token,"offset":0,"contentBase64":"AA=="});
        // The first byte of an MP4 box length is zero, so use a different byte.
        let mut changed = changed;
        changed["contentBase64"] = json!("/w==");
        assert_eq!(f.call(UPLOAD, changed).unwrap_err().code(), "CONFLICT");
        let staged = f.send(id, token, &bytes).unwrap();
        assert_eq!(staged["status"], "pending_owner_review");
        assert_eq!(f.call(STATUS, query.clone()).unwrap(), staged);
        assert_eq!(
            f.store
                .v1_call(&f.writer, "mother_get_record", json!({"id":target["id"]}))
                .unwrap(),
            target
        );
        let wrong_review = json!({"operationId":id,"reviewId":Uuid::new_v4().to_string()});
        assert_eq!(
            f.owner("approve", &wrong_review).unwrap_err().code(),
            "CONFLICT"
        );
        assert_eq!(staged["display"], "withheld_embedded_metadata");
        let review = json!({"operationId":id,"reviewId":staged["originalReviewId"]});
        let approved = f.owner("approve", &review).unwrap();
        assert_eq!(approved["verification"]["verified"], true);
        assert_eq!(approved["verification"]["sha256"], args["file"]["sha256"]);
        assert_eq!(approved["verification"]["link"]["memoryId"], target["id"]);
        assert_eq!(
            approved["verification"]["receipt"]["operationId"],
            staged["originalReviewId"]
        );
        assert_eq!(f.call(CANCEL, query.clone()).unwrap(), approved);
        assert_eq!(f.owner("approve", &review).unwrap(), approved);
        assert_eq!(f.call(STATUS, query.clone()).unwrap(), approved);
        let dir = f.root.join(id.as_str().unwrap());
        assert!(!dir.join(FILE).exists());
        let plan = load(&dir).unwrap().plan.unwrap();
        let second_target = f.write(None);
        let second = f.start(&second_target, &bytes);
        let second_id = &second["operationId"];
        let next = f.call(PREPARE, second.clone()).unwrap();
        let staged = f.send(second_id, &next["transferToken"], &bytes).unwrap();
        let second_approved = f
            .owner(
                "approve",
                &json!({"operationId":second_id,"reviewId":staged["reviewId"]}),
            )
            .unwrap();
        assert_eq!(
            approved["verification"]["sha256"],
            second_approved["verification"]["sha256"]
        );
        assert_eq!(
            second_approved["verification"]["link"]["memoryId"],
            second_target["id"]
        );
        assert_eq!(f.store.media_verify().unwrap()["assetsChecked"], 1);
        f.store
            .revoke_attachment(&plan, &plan.operation_id)
            .unwrap();
        let revoked = f.call(STATUS, query).unwrap();
        assert_eq!(revoked["status"], "retracted");
        assert_eq!(revoked["verification"]["verified"], false);
        assert_eq!(
            revoked["verification"]["receipt"],
            approved["verification"]["receipt"]
        );
    }

    #[test]
    fn handoff_requires_selection_and_review_and_terminal_operations_publish_nothing() {
        let f = Fixture::new();
        let target = f.write(None);
        let bytes = include_bytes!("../../tests/fixtures/attachment-sample.mp4");
        let args = f.start(&target, bytes);
        let id = &args["operationId"];
        let operation = json!({"operationId":id});
        let mut locator = args.clone();
        locator.as_object_mut().unwrap().remove("file");
        locator["sourceLocator"] = json!("file_synthetic");
        assert_eq!(
            f.call(PREPARE, locator).unwrap()["code"],
            "SOURCE_BYTES_UNAVAILABLE"
        );
        assert_eq!(
            f.call(SUBMIT, operation.clone()).unwrap_err().code(),
            "SOURCE_BYTES_UNAVAILABLE"
        );
        f.owner("choose", &json!({"operationId":id,"file":args["file"]}))
            .unwrap();
        f.owner(
            "upload",
            &json!({"operationId":id,"offset":0,"contentBase64":base64::encode(bytes)}),
        )
        .unwrap();
        let staged = f.owner("submit", &operation).unwrap();
        assert_eq!(staged["status"], "pending_owner_review");
        assert_eq!(
            f.call(CANCEL, operation.clone()).unwrap()["status"],
            "cancelled"
        );
        assert_eq!(
            f.owner(
                "approve",
                &json!({"operationId":id,"reviewId":staged["reviewId"]})
            )
            .unwrap()["status"],
            "cancelled"
        );

        let args = f.start(&target, bytes);
        let id = &args["operationId"];
        let prepared = f.call(PREPARE, args.clone()).unwrap();
        let staged = f.send(id, &prepared["transferToken"], bytes).unwrap();
        let dir = f.root.join(id.as_str().unwrap());
        let mut session = load(&dir).unwrap();
        session.created = now() - LIFETIME;
        save(&dir, &session).unwrap();
        let expired = f
            .owner(
                "approve",
                &json!({"operationId":id,"reviewId":staged["reviewId"]}),
            )
            .unwrap();
        assert_eq!(expired["status"], "expired");
        assert!(!dir.join(FILE).exists());

        let args = f.start(&target, bytes);
        let id = &args["operationId"];
        let prepared = f.call(PREPARE, args.clone()).unwrap();
        let staged = f.send(id, &prepared["transferToken"], bytes).unwrap();
        assert_eq!(f.write(Some(&target))["revision"], 2);
        let stale = f
            .owner(
                "approve",
                &json!({"operationId":id,"reviewId":staged["reviewId"]}),
            )
            .unwrap();
        assert_eq!(stale["status"], "stale");
        assert_eq!(f.store.media_verify().unwrap()["assetsChecked"], 0);
    }

    /// Reproduces the live Windows failure: a staging root created under a parent
    /// whose inheritable entries include a foreign read grant (mask 0x001200a9).
    #[cfg(windows)]
    #[test]
    fn staging_root_is_owner_only_under_a_parent_with_an_inherited_read_grant() {
        use windows_permissions::{
            LocalBox, SecurityDescriptor,
            constants::{SeObjectType, SecurityInformation},
        };
        let parent =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let current = windows_permissions::utilities::current_process_sid().unwrap();
        let sd: LocalBox<SecurityDescriptor> = format!(
            "D:P(A;OICI;FA;;;{current})(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)(A;OICI;0x1200a9;;;BU)"
        )
        .parse()
        .unwrap();
        windows_permissions::wrappers::SetNamedSecurityInfo(
            parent.path().as_os_str(),
            SeObjectType::SE_FILE_OBJECT,
            SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
            None,
            None,
            sd.dacl(),
            None,
        )
        .unwrap();
        // The pre-fix creation path inherits the read grant and is fenced.
        let inherited = parent.path().join("inherited");
        fs::create_dir(&inherited).unwrap();
        let error = crate::private_paths::owner_directory(&inherited).unwrap_err();
        assert_eq!(error.code(), "MIGRATION_FENCED");
        assert!(error.to_string().contains("0x001200a9"), "{error}");
        // An existing insecure directory is never repaired, so it still fails closed.
        assert_eq!(
            private_directory(&inherited).unwrap_err().code(),
            "MIGRATION_FENCED"
        );
        // A newly created staging root is owner-only and its sessions inherit that.
        let staging = parent.path().join("attachment-sessions-synthetic");
        private_directory(&staging).unwrap();
        crate::private_paths::owner_directory(&staging).unwrap();
        private_directory(&staging).unwrap();
        let session = private_temp(&staging).unwrap();
        crate::private_paths::owner_directory(session.path()).unwrap();
    }

    struct Direct<'a>(&'a Fixture, &'a Auth);
    impl AttachmentClient for Direct<'_> {
        fn request(&self, method: &str, args: &Value) -> Result<Value> {
            dispatch(&self.0.store, self.1, &self.0.root, method, args)
        }
    }

    /// A host file parameter reaches owner review from real bytes, survives an
    /// exact retry, needs owner approval and reuses stored bytes for a second memory.
    #[test]
    fn host_file_parameter_reaches_owner_review_and_reuses_the_stored_original() {
        use super::super::host_file::import_with;
        let f = Fixture::new();
        let target = f.write(None);
        let bytes = video();
        let link = "https://files.oaiusercontent.com/file-synthetic?sig=redacted";
        let host_file = |id: &str| {
            json!({"operationId":id,"memoryId":target["id"],"expectedRevision":target["revision"],
                "file":{"download_url":link,"file_id":"file_00000000synthetic","mime_type":"video/mp4",
                    "file_name":"/mnt/data/synthetic-clip.mp4"}})
        };
        let id = Uuid::new_v4().to_string();
        let fetched = std::cell::Cell::new(0);
        let fetch = |url: &str| {
            assert_eq!(url, link);
            fetched.set(fetched.get() + 1);
            Ok(bytes.clone())
        };
        let client = Direct(&f, &f.transfer);
        // A credential without media:attach is refused before anything is staged.
        assert_eq!(
            import_with(&Direct(&f, &f.reader), &host_file(&id), |_| Ok(
                bytes.clone()
            ))
            .unwrap_err()
            .code(),
            "FORBIDDEN"
        );
        let staged = import_with(&client, &host_file(&id), fetch).unwrap();
        assert_eq!(staged["status"], "pending_owner_review");
        assert_eq!(staged["file"]["fileName"], "synthetic-clip.mp4");
        assert_eq!(staged["file"]["sizeBytes"], bytes.len());
        assert_eq!(
            staged["file"]["sha256"],
            format!("{:x}", Sha256::digest(&bytes))
        );
        assert_eq!(staged["sourceLocator"], "file_00000000synthetic");
        // Unapproved media is not canonical: the narrative record is unchanged.
        assert_eq!(
            f.store
                .v1_call(&f.writer, "mother_get_record", json!({"id":target["id"]}))
                .unwrap(),
            target
        );
        assert_eq!(f.store.media_verify().unwrap()["assetsChecked"], 0);
        // A retry of the same operation reconciles without downloading again.
        let retry = import_with(&client, &host_file(&id), |_| -> Result<Vec<u8>> {
            panic!("an existing operation must not download again")
        })
        .unwrap();
        assert_eq!(retry, staged);
        assert_eq!(fetched.get(), 1);
        let approved = f
            .owner(
                "approve",
                &json!({"operationId":id,"reviewId":staged["reviewId"]}),
            )
            .unwrap();
        assert_eq!(approved["verification"]["verified"], true);
        assert_eq!(approved["verification"]["link"]["memoryId"], target["id"]);
        // The same original for another memory reuses the stored bytes.
        let second_target = f.write(None);
        let second_id = Uuid::new_v4().to_string();
        let mut second = host_file(&second_id);
        second["memoryId"] = second_target["id"].clone();
        second["expectedRevision"] = second_target["revision"].clone();
        let staged = import_with(&client, &second, |_| Ok(bytes.clone())).unwrap();
        let approved_again = f
            .owner(
                "approve",
                &json!({"operationId":second_id,"reviewId":staged["reviewId"]}),
            )
            .unwrap();
        assert_eq!(
            approved_again["verification"]["sha256"],
            approved["verification"]["sha256"]
        );
        assert_eq!(f.store.media_verify().unwrap()["assetsChecked"], 1);
        // A cancelled import publishes nothing.
        let third_target = f.write(None);
        let third_id = Uuid::new_v4().to_string();
        let mut third = host_file(&third_id);
        third["memoryId"] = third_target["id"].clone();
        third["expectedRevision"] = third_target["revision"].clone();
        import_with(&client, &third, |_| Ok(bytes.clone())).unwrap();
        let cancelled = f.call(CANCEL, json!({"operationId":third_id})).unwrap();
        assert_eq!(cancelled["status"], "cancelled");
        assert!(cancelled["verification"].is_null());
    }

    /// Live regression: ChatGPT sent a complete file object whose file_id was not
    /// in the `file_...` form. It must reach owner review from the real bytes.
    #[test]
    fn host_file_objects_with_opaque_ids_reach_owner_review() {
        use super::super::host_file::import_with;
        let f = Fixture::new();
        let target = f.write(None);
        let bytes = video();
        let client = Direct(&f, &f.transfer);
        for (file_id, expected) in [
            (
                "sediment://file_00000000synthetic",
                "sediment:file_00000000synthetic",
            ),
            ("att_9f8e7d6c5b4a", "att_9f8e7d6c5b4a"),
        ] {
            let id = Uuid::new_v4().to_string();
            let args = json!({"operationId":id,"memoryId":target["id"],"expectedRevision":target["revision"],
                "file":{"download_url":"https://files.oaiusercontent.com/x?sig=redacted","file_id":file_id,
                    "mime_type":"video/mp4","file_name":"IMG_synthetic.mp4"}});
            let staged = import_with(&client, &args, |_| Ok(bytes.clone())).unwrap();
            assert_eq!(staged["status"], "pending_owner_review");
            assert_eq!(staged["sourceLocator"], expected);
            assert_eq!(
                staged["file"]["sha256"],
                format!("{:x}", Sha256::digest(&bytes))
            );
            let cancelled = f.call(CANCEL, json!({"operationId":id})).unwrap();
            assert_eq!(cancelled["status"], "cancelled");
        }
        // Bounds still apply before any download.
        let id = Uuid::new_v4().to_string();
        let args = json!({"operationId":id,"memoryId":target["id"],"expectedRevision":target["revision"],
            "file":{"download_url":"https://files.oaiusercontent.com/x","file_id":"x".repeat(481)}});
        let error = import_with(&client, &args, |_| -> Result<Vec<u8>> {
            panic!("an oversized identifier must not download")
        })
        .unwrap_err();
        assert_eq!(error.code(), "SOURCE_BYTES_UNAVAILABLE");
        assert!(error.to_string().contains("over 480 bytes"));
        assert_eq!(f.store.media_verify().unwrap()["assetsChecked"], 0);
    }

    /// Trust policies relax review only for authorized, confirmed, in-limit work,
    /// commit through the owner's canonical writer, and stay owner-controlled.
    #[test]
    fn owner_granted_trust_commits_routine_attachments_and_escalates_the_rest() {
        use super::super::host_file::import_with;
        use crate::garden::store::trust::TrustMode;
        let f = Fixture::new();
        // A synthetic PNG without embedded metadata, so it can be shown as the original.
        let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
        for (kind, data) in [
            (&b"IHDR"[..], &[0u8, 0, 0, 5, 0, 0, 0, 7, 8, 2, 0, 0, 0][..]),
            (b"IDAT", &[1, 2, 3]),
            (b"IEND", &[]),
        ] {
            bytes.extend_from_slice(&(data.len() as u32).to_be_bytes());
            bytes.extend_from_slice(kind);
            bytes.extend_from_slice(data);
            bytes.extend_from_slice(&[0; 4]);
        }
        let send_as = |auth: &Auth,
                       association: &str,
                       initiation: Option<&str>,
                       payload: &[u8],
                       mime: &str| {
            let target = f.write(None);
            let mut args = json!({"operationId":Uuid::new_v4().to_string(),"memoryId":target["id"],
                "expectedRevision":target["revision"],"association":association,
                "file":{"download_url":"https://files.oaiusercontent.com/x","file_id":"file_synthetic",
                    "mime_type":mime,"file_name":"synthetic-original"}});
            if let Some(initiation) = initiation {
                args["initiation"] = json!(initiation);
            }
            let payload = payload.to_vec();
            (
                target,
                import_with(&Direct(&f, auth), &args, move |_| Ok(payload)).unwrap(),
            )
        };
        let next = |association: &str, initiation: Option<&str>| {
            send_as(&f.transfer, association, initiation, &bytes, "image/png")
        };
        let other = |association: &str, initiation: Option<&str>| {
            send_as(&f.other, association, initiation, &bytes, "image/png")
        };
        // Escalated operations are cancelled once checked, keeping within pending quotas.
        let cancel = |auth: &Auth, staged: &Value| {
            let cancelled = dispatch(
                &f.store,
                auth,
                &f.root,
                CANCEL,
                &json!({"operationId":staged["operationId"]}),
            )
            .unwrap();
            assert_eq!(cancelled["status"], "cancelled");
        };
        // Default: manual review, even for a confirmed match.
        let (_, staged) = next("confirmed", None);
        assert_eq!(staged["status"], "pending_owner_review");
        assert_eq!(staged["reviewReason"], "trust_policy_manual_review");
        cancel(&f.transfer, &staged);
        let caps = capabilities(&f.store, &f.transfer);
        assert_eq!(caps["trust"]["routineAttachments"], "owner_review");
        // Trust never substitutes for authorization, and revisions guard every change.
        assert_eq!(
            f.store
                .set_trust_policy("reader", TrustMode::TrustedCompanion, 5, 1 << 20, 0)
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
        assert_eq!(
            f.store
                .set_trust_policy("codex-test", TrustMode::TrustedCompanion, 2, 64 << 20, 7)
                .unwrap_err()
                .code(),
            "CONFLICT"
        );
        let policy = f
            .store
            .set_trust_policy("codex-test", TrustMode::TrustedCompanion, 2, 64 << 20, 0)
            .unwrap();
        assert_eq!(policy.revision, 1);
        let caps = capabilities(&f.store, &f.transfer);
        assert_eq!(caps["trust"]["routineAttachments"], "automatic");
        assert_eq!(caps["trust"]["autonomousBackfill"], false);
        // Another credential's policy is untouched.
        assert_eq!(
            capabilities(&f.store, &f.other)["trust"]["routineAttachments"],
            "owner_review"
        );
        // A confirmed, owner-requested attachment commits with a verified receipt.
        let (target, committed) = next("confirmed", Some("owner_request"));
        assert_eq!(committed["status"], "approved");
        assert_eq!(committed["verification"]["verified"], true);
        assert_eq!(committed["verification"]["link"]["memoryId"], target["id"]);
        assert_eq!(
            committed["authorization"]["mechanism"],
            "owner_granted_trust_policy"
        );
        assert_eq!(committed["authorization"]["policy"]["policyRevision"], 1);
        assert_eq!(
            committed["authorization"]["policy"]["clientId"],
            "codex-test"
        );
        // The narrative memory is not revised by attaching evidence.
        assert_eq!(
            f.store
                .v1_call(&f.writer, "mother_get_record", json!({"id":target["id"]}))
                .unwrap(),
            target
        );
        // Media with unstrippable embedded metadata still needs the owner's display decision.
        let (_, tagged) = send_as(&f.transfer, "confirmed", None, &video(), "video/mp4");
        assert_eq!(tagged["reviewReason"], "embedded_metadata_display");
        cancel(&f.transfer, &tagged);
        // Escalations: uncertain match, self-started work under companion trust.
        let (_, uncertain) = next("uncertain", None);
        assert_eq!(uncertain["reviewReason"], "association_review_required");
        let (_, autonomous) = next("confirmed", Some("autonomous"));
        assert_eq!(autonomous["reviewReason"], "autonomous_work_not_permitted");
        cancel(&f.transfer, &autonomous);
        // The daily limit (2) counts only automatic commits.
        let (_, second) = next("confirmed", None);
        assert_eq!(second["status"], "approved");
        let (_, limited) = next("confirmed", None);
        assert_eq!(limited["reviewReason"], "daily_commit_limit");
        cancel(&f.transfer, &limited);
        // Autonomous gardening (second credential) allows self-started work;
        // returning to manual review applies to the very next submission.
        f.store
            .set_trust_policy(
                "claude-test",
                TrustMode::AutonomousGardening,
                10,
                64 << 20,
                0,
            )
            .unwrap();
        assert_eq!(
            capabilities(&f.store, &f.other)["trust"]["autonomousBackfill"],
            true
        );
        let (_, gardened) = other("confirmed", Some("autonomous"));
        assert_eq!(gardened["status"], "approved");
        f.store
            .set_trust_policy("claude-test", TrustMode::ManualReview, 10, 64 << 20, 1)
            .unwrap();
        let (_, revoked) = other("confirmed", None);
        assert_eq!(revoked["reviewReason"], "trust_policy_manual_review");
        cancel(&f.other, &revoked);
        // Agents have no tool that changes trust or approves.
        for name in ["mother_trust_set", "approve"] {
            assert_eq!(f.call(name, json!({})).unwrap_err().code(), "INVALID_INPUT");
        }
        // The owner can still approve an escalated one manually, recorded as such.
        let manual = f
            .owner(
                "approve",
                &json!({"operationId":uncertain["operationId"],"reviewId":uncertain["reviewId"]}),
            )
            .unwrap();
        assert_eq!(
            manual["authorization"]["mechanism"],
            "owner_manual_approval"
        );
        assert!(manual["authorization"]["policy"].is_null());
        // Owner activity shows every operation with its mechanism.
        let activity = f.owner("activity", &json!({})).unwrap();
        let mechanisms: Vec<_> = activity["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item["authorization"]["mechanism"].as_str())
            .collect();
        assert_eq!(
            mechanisms
                .iter()
                .filter(|m| **m == "owner_granted_trust_policy")
                .count(),
            3
        );
        assert_eq!(
            mechanisms
                .iter()
                .filter(|m| **m == "owner_manual_approval")
                .count(),
            1
        );
        // Retracting one automatic link keeps the shared original for the others.
        let retracted = f
            .owner(
                "retract",
                &json!({"operationId":committed["operationId"],"reviewId":committed["reviewId"]}),
            )
            .unwrap();
        assert_eq!(retracted["status"], "retracted");
        assert_eq!(f.store.media_verify().unwrap()["assetsChecked"], 1);
        assert_eq!(
            f.call(STATUS, json!({"operationId":second["operationId"]}))
                .unwrap()["verification"]["verified"],
            true
        );
        // Backups accept the trust table and never export it.
        let out = f._directory.path().join("backup");
        f.store.backup_to(&out).unwrap();
        let listing = fs::read_dir(&out)
            .unwrap()
            .map(|entry| fs::read(entry.unwrap().path()).unwrap_or_default())
            .collect::<Vec<_>>()
            .concat();
        assert!(!String::from_utf8_lossy(&listing).contains("garden_trust_policies"));
        assert!(!String::from_utf8_lossy(&listing).contains("autonomous_gardening"));
    }
}
