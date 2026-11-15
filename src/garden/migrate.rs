//! Existing owner cutover journals, fencing, recovery and explicit grant reissuance.
#[cfg(test)]
#[path = "../../tests/support/native_cutover.rs"]
mod tests;
use crate::{
    access::ClientAccess,
    error::{Error, Result},
    garden::{access::GardenAccess, config::domain_error, store::GardenStore},
};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// Immutable identity and paths for one owner-requested cutover.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MigrationRequest {
    pub source: PathBuf,
    pub destination: PathBuf,
    /// A new owner-private directory containing append-only transition files.
    pub journal: PathBuf,
    pub operation_id: String,
}
/// Durable authority barriers. Only `Activated` permits destination serving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationPhase {
    Preparing,
    Prepared,
    Fenced,
    Activated,
    RollingBack,
    RolledBack,
    NeedsReconciliation,
}
/// Exact restrictions for explicit owner reissuance; credentials are never captured.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrantRestriction {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub access: ClientAccess,
    pub garden: Option<GardenAccess>,
    pub policy_required: bool,
    pub garden_required: bool,
    pub revoked: bool,
}
/// Private validation manifest and latest durable journal state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MigrationStatus {
    pub version: u32,
    pub revision: u64,
    pub request: MigrationRequest,
    pub phase: MigrationPhase,
    pub workspace_id: String,
    pub source_generation: String,
    pub destination_generation: Option<String>,
    pub source_fingerprint: String,
    pub destination_fingerprint: Option<String>,
    pub source_archive_fingerprint: Option<String>,
    pub archive: PathBuf,
    pub source_readonly: bool,
    pub restrictions: Vec<GrantRestriction>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fence {
    version: u32,
    journal: PathBuf,
    operation_id: String,
    role: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReissueIntent {
    operation_id: String,
    client_id: String,
    config: PathBuf,
}

fn problem(code: &'static str, message: &str) -> Error {
    Error::domain(code, message)
}
fn fenced() -> Error {
    problem(
        "MIGRATION_FENCED",
        "This endpoint is fenced by an owner cutover; inspect the cutover journal before retrying.",
    )
}
fn sidecar(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(".mother-cutover");
    PathBuf::from(name)
}
fn file_fingerprint(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut file = File::open(path)?;
    let mut bytes = [0; 64 * 1024];
    let mut hash = Sha256::new();
    loop {
        let count = file.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        hash.update(&bytes[..count]);
    }
    Ok(format!("{:x}", hash.finalize()))
}
fn archive_sidecars_clear(path: &Path) -> Result<()> {
    for suffix in ["-wal", "-journal"] {
        let mut name = path.as_os_str().to_os_string();
        name.push(suffix);
        match fs::metadata(PathBuf::from(name)) {
            Ok(meta) if meta.len() != 0 => {
                return Err(problem(
                    "RECONCILIATION_REQUIRED",
                    "Archived source has a pending SQLite journal; reconcile it before rollback.",
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
fn safe_path(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        return Err(problem(
            "INVALID_INPUT",
            "Migration paths must be absolute.",
        ));
    }
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    return Err(problem(
                        "INVALID_INPUT",
                        "Migration paths cannot contain links.",
                    ));
                }
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    if meta.file_attributes() & 0x400 != 0 {
                        return Err(problem(
                            "INVALID_INPUT",
                            "Migration paths cannot contain reparse points.",
                        ));
                    }
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
fn normalized(path: &Path) -> Result<PathBuf> {
    safe_path(path)?;
    let parent = path
        .parent()
        .ok_or_else(|| problem("INVALID_INPUT", "Expected a file path."))?;
    let name = path
        .file_name()
        .ok_or_else(|| problem("INVALID_INPUT", "Expected a named path."))?;
    Ok(fs::canonicalize(parent)?.join(name))
}
#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}
#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<()> {
    Ok(())
}
fn publish_json(path: &Path, value: &impl Serialize) -> Result<()> {
    safe_path(path)?;
    if path.exists() {
        return Err(problem(
            "CONFLICT",
            "Migration publication destination already exists.",
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| problem("INVALID_INPUT", "Invalid publication path."))?;
    let temporary = parent.join(format!(".migration-{}.pending", Uuid::new_v4()));
    let mut file = crate::config::create_private_file(&temporary)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    drop(file);
    fs::rename(&temporary, path)?;
    sync_directory(parent)
}
struct JournalLock(File);
impl Drop for JournalLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}
fn lock(journal: &Path) -> Result<JournalLock> {
    safe_path(journal)?;
    crate::private_paths::owner_directory(journal)?;
    let path = journal.join(".lock");
    let file = match crate::config::create_private_file(&path) {
        Ok(file) => file,
        Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            safe_path(&path)?;
            fs::OpenOptions::new().read(true).write(true).open(&path)?
        }
        Err(error) => return Err(error),
    };
    file.try_lock_exclusive()
        .map_err(|_| problem("CONFLICT", "Another owner migration command is running."))?;
    Ok(JournalLock(file))
}
fn append(state: &mut MigrationStatus, phase: MigrationPhase) -> Result<()> {
    let mut next = state.clone();
    next.revision += 1;
    next.phase = phase;
    if next.revision > 999_999 {
        return Err(problem(
            "LIMIT_EXCEEDED",
            "Migration journal transition bound exceeded.",
        ));
    }
    publish_json(
        &next
            .request
            .journal
            .join(format!("{:06}.json", next.revision)),
        &next,
    )?;
    *state = next;
    Ok(())
}
/// Read a bounded journal without opening either database or starting a service.
/// # Errors
/// Rejects gaps, invalid identity, links and malformed transition files.
pub fn status(journal: &Path) -> Result<MigrationStatus> {
    safe_path(journal)?;
    crate::private_paths::owner_directory(journal)?;
    let mut revisions = Vec::new();
    for entry in fs::read_dir(journal)? {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if name.len() == 11
            && name.ends_with(".json")
            && name[..6].bytes().all(|b| b.is_ascii_digit())
        {
            revisions.push(name[..6].parse::<u64>().map_err(|_| fenced())?);
        }
    }
    revisions.sort_unstable();
    let mut latest: Option<MigrationStatus> = None;
    for (index, revision) in revisions.into_iter().enumerate() {
        if revision != index as u64 {
            return Err(fenced());
        }
        let path = journal.join(format!("{revision:06}.json"));
        safe_path(&path)?;
        let next: MigrationStatus =
            serde_json::from_value(crate::config::read_json(&path, 128 * 1024 * 1024)?)?;
        if next.version != 1
            || next.revision != revision
            || Uuid::parse_str(&next.request.operation_id).is_err()
            || normalized(&next.request.journal)? != normalized(journal)?
            || latest.as_ref().is_some_and(|previous| {
                previous.request != next.request
                    || previous.source_fingerprint != next.source_fingerprint
                    || previous.restrictions != next.restrictions
            })
        {
            return Err(fenced());
        }
        latest = Some(next);
    }
    latest.ok_or_else(|| {
        problem(
            "NOT_FOUND",
            "Migration journal has no committed transitions.",
        )
    })
}
fn check_operation(state: &MigrationStatus, operation: &str) -> Result<()> {
    if state.request.operation_id != operation {
        return Err(problem(
            "OPERATION_REUSED",
            "The journal belongs to a different operation UUID.",
        ));
    }
    Ok(())
}
fn clear_source_fence(state: &MigrationStatus) -> Result<()> {
    let marker = sidecar(&state.request.source);
    match fs::symlink_metadata(&marker) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
        Ok(_) => {}
    }
    safe_path(&marker)?;
    let existing: Fence = serde_json::from_value(crate::config::read_json(&marker, 16_384)?)?;
    if existing.version != 1
        || existing.role != "source"
        || existing.operation_id != state.request.operation_id
        || existing.journal != state.request.journal
    {
        return Err(fenced());
    }
    fs::remove_file(marker)?;
    sync_directory(state.request.source.parent().ok_or_else(fenced)?)
}
/// Reject a former SQLite endpoint or its archive before opening or mutating it.
/// # Errors
/// Any cutover marker fails closed, including unreadable or incomplete markers.
pub fn check_source(path: &Path) -> Result<()> {
    crate::private_paths::single_link(path)?;
    match fs::symlink_metadata(sidecar(path)) {
        Ok(_) => Err(fenced()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match fs::canonicalize(path) {
                Ok(canonical) => {
                    crate::private_paths::single_link(&canonical)?;
                    match fs::symlink_metadata(sidecar(&canonical)) {
                        Ok(_) => return Err(fenced()),
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                        Err(_) => return Err(fenced()),
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => return Err(fenced()),
            }
            Ok(())
        }
        Err(_) => Err(fenced()),
    }
}
/// Require explicit activation for a prepared destination at each serving admission.
/// # Errors
/// Missing or malformed journals, rollback and preparation all fail closed.
pub fn check_destination(path: &Path) -> Result<()> {
    crate::private_paths::single_link(path)?;
    let canonical = match fs::canonicalize(path) {
        Ok(canonical) => canonical,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => path.to_path_buf(),
        Err(_) => return Err(fenced()),
    };
    crate::private_paths::single_link(&canonical)?;
    let marker = sidecar(&canonical);
    match fs::symlink_metadata(&marker) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(fenced()),
        Ok(_) => {}
    }
    safe_path(&marker).map_err(|_| fenced())?;
    let fence: Fence =
        serde_json::from_value(crate::config::read_json(&marker, 16_384).map_err(|_| fenced())?)
            .map_err(|_| fenced())?;
    let state = status(&fence.journal).map_err(|_| fenced())?;
    if fence.version != 1
        || fence.role != "destination"
        || fence.operation_id != state.request.operation_id
        || state.phase != MigrationPhase::Activated
        || normalized(&canonical)? != state.request.destination
    {
        return Err(fenced());
    }
    Ok(())
}

/// Restore the original endpoint only if no destination canonical state would be lost.
/// # Errors
/// A live destination owner, changed canonical state or uncertain files blocks rollback.
pub fn rollback(journal: &Path, operation: &str) -> Result<MigrationStatus> {
    let _lock = lock(journal)?;
    let mut state = status(journal)?;
    check_operation(&state, operation)?;
    if state.phase == MigrationPhase::RolledBack {
        clear_source_fence(&state)?;
        return Ok(state);
    }
    let destination = if state.request.destination.exists() {
        crate::private_paths::single_link(&state.request.destination)?;
        let store = GardenStore::open(&state.request.destination).map_err(domain_error)?;
        if let Some(expected) = &state.destination_fingerprint {
            if &store.migration_fingerprint().map_err(domain_error)? != expected {
                return Err(problem(
                    "RECONCILIATION_REQUIRED",
                    "Destination canonical state changed; verified reverse export or reconciliation is required.",
                ));
            }
        } else {
            return Err(problem(
                "RECONCILIATION_REQUIRED",
                "Complete preparation validation before rolling back an existing destination.",
            ));
        }
        Some(store)
    } else {
        if state.destination_fingerprint.is_some() {
            return Err(problem(
                "RECONCILIATION_REQUIRED",
                "Verified destination is missing; acknowledged state cannot be ruled out.",
            ));
        }
        None
    };
    if state.archive.exists() {
        crate::private_paths::single_link(&state.archive)?;
        archive_sidecars_clear(&state.archive)?;
        if state.request.source.exists() {
            return Err(problem(
                "RECONCILIATION_REQUIRED",
                "Both source and archive exist; owner reconciliation is required.",
            ));
        }
        if Some(file_fingerprint(&state.archive)?) != state.source_archive_fingerprint {
            return Err(problem(
                "RECONCILIATION_REQUIRED",
                "Archived source bytes changed; owner reconciliation is required.",
            ));
        }
    } else if !state.request.source.exists() {
        return Err(problem(
            "RECONCILIATION_REQUIRED",
            "Neither the original source nor its archive is available.",
        ));
    } else if state.phase == MigrationPhase::RollingBack
        && let Some(expected) = &state.source_archive_fingerprint
        && &file_fingerprint(&state.request.source)? != expected
    {
        return Err(problem(
            "RECONCILIATION_REQUIRED",
            "Restored source changed during an interrupted rollback.",
        ));
    }
    append(&mut state, MigrationPhase::RollingBack)?;
    if state.archive.exists() {
        let mut permissions = fs::metadata(&state.archive)?.permissions();
        // set_readonly(false) on Unix would widen permissions; restore only the owner write bit.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = permissions.mode();
            permissions.set_mode(if state.source_readonly {
                mode & !0o200
            } else {
                mode | 0o200
            });
        }
        #[cfg(not(unix))]
        permissions.set_readonly(state.source_readonly);
        fs::set_permissions(&state.archive, permissions)?;
        fs::rename(&state.archive, &state.request.source)?;
        sync_directory(state.request.source.parent().ok_or_else(fenced)?)?;
    }
    append(&mut state, MigrationPhase::RolledBack)?;
    clear_source_fence(&state)?;
    drop(destination);
    Ok(state)
}

/// Reissue one client's exact restrictions to a new private native config.
/// # Errors
/// Requires activated state, a non-revoked source grant and exclusive new config path.
/// An uncertain previous reissuance is reconciled against that same file, never replayed.
pub fn reissue(journal: &Path, client: &str, config: &Path) -> Result<MigrationStatus> {
    let _lock = lock(journal)?;
    let state = status(journal)?;
    if state.phase != MigrationPhase::Activated {
        return Err(fenced());
    }
    let restriction = state
        .restrictions
        .iter()
        .find(|grant| grant.client_id == client && !grant.revoked)
        .ok_or_else(|| {
            problem(
                "FORBIDDEN",
                "No active source restriction exists for this client.",
            )
        })?;
    let config = normalized(config)?;
    crate::private_paths::owner_directory(config.parent().ok_or_else(fenced)?)?;
    let intent_path = journal.join(format!("reissue-{}.json", crate::contracts::hash(client)));
    if intent_path.exists() {
        safe_path(&intent_path)?;
        let intent: ReissueIntent =
            serde_json::from_value(crate::config::read_json(&intent_path, 16_384)?)?;
        if intent.operation_id != state.request.operation_id
            || intent.client_id != client
            || intent.config != config
        {
            return Err(problem(
                "OPERATION_REUSED",
                "This client's reissuance is bound to a different private output path.",
            ));
        }
    } else {
        if config.exists() {
            return Err(problem(
                "CONFLICT",
                "Reissuance requires a new private output file.",
            ));
        }
        publish_json(
            &intent_path,
            &ReissueIntent {
                operation_id: state.request.operation_id.clone(),
                client_id: client.into(),
                config: config.clone(),
            },
        )?;
    }
    let store = GardenStore::open(&state.request.destination).map_err(domain_error)?;
    if config.exists() {
        let saved = super::config::load_garden_config(&config)?;
        if saved.client_id != client || saved.database_path != state.request.destination {
            return Err(problem(
                "OPERATION_REUSED",
                "Reissuance output belongs to another identity or destination.",
            ));
        }
        // A credential with changed restrictions is never accepted as an idempotent retry.
        store.verify_migration_grant(&saved.auth(), &restriction.scopes, &restriction.access, restriction.garden.as_ref()).map_err(|_| problem("RECONCILIATION_REQUIRED", "Previous reissuance outcome or restrictions differ; reconcile this exact private credential instead of issuing another."))?;
        return Ok(state);
    }
    if store.migration_grant_exists(client).map_err(domain_error)? {
        return Err(problem(
            "RECONCILIATION_REQUIRED",
            "A destination principal already exists but its intended output is unavailable; reconcile instead of rotating its key.",
        ));
    }
    if restriction
        .scopes
        .iter()
        .any(|scope| crate::contracts::SCOPES.contains(&scope.as_str()))
    {
        store.v1_grant_to_file_with_access(
            client,
            &restriction.scopes,
            &config,
            &state.request.destination,
            &restriction.access,
            restriction.garden.as_ref(),
        )?;
    } else {
        let garden = restriction.garden.as_ref().ok_or_else(|| {
            problem(
                "FORBIDDEN",
                "Graph-only source grants require an explicit graph policy.",
            )
        })?;
        store
            .grant_to_file_with_access(
                client,
                &restriction.scopes,
                &config,
                &state.request.destination,
                &restriction.access,
                garden,
            )
            .map_err(domain_error)?;
    }
    Ok(state)
}
