//! Binary-only ChatGPT connection adapter.
use mother::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub version: u32,
    pub tunnel_id: String,
    pub organization_id: Option<String>,
    pub client_config: PathBuf,
    pub tunnel_client: PathBuf,
    pub mother_exe: PathBuf,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    Unconfigured,
    MissingDependency,
    MissingCredential,
    MissingGrant,
    Stopped,
    Unhealthy,
    Ready,
    Unknown,
}
impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub state: ConnectionState,
    pub detail: String,
    pub process_running: bool,
    pub healthy: bool,
    pub ready: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Startup {
    Enabled,
    Disabled,
}

use mother::config::{data_directory, load_config, read_json};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
use zeroize::Zeroizing;
mod alias;
mod process;
mod startup;

impl Status {
    fn new(state: ConnectionState, detail: &str) -> Self {
        Self {
            state,
            detail: detail.into(),
            process_running: false,
            healthy: false,
            ready: false,
        }
    }
}
fn failure(message: &str) -> Error {
    Error::domain("CONNECTION", message)
}
fn root() -> Result<PathBuf> {
    Ok(data_directory()?.join("connections"))
}
fn settings_path() -> Result<PathBuf> {
    Ok(root()?.join("chatgpt.json"))
}
fn safe_text(value: &str) -> bool {
    !value
        .chars()
        .any(|c| c.is_control() || "\"'$`%;&|<>".contains(c))
}
fn quoted_path(path: &Path) -> Result<String> {
    let text = path
        .to_str()
        .ok_or_else(|| failure("Paths must be valid Unicode."))?;
    if !safe_text(text) {
        return Err(failure(
            "Paths contain unsupported command-parser characters.",
        ));
    }
    Ok(format!("\"{}\"", text.replace('\\', "/")))
}
fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 200
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
}
fn validate(settings: &Settings) -> Result<()> {
    if settings.version != 1
        || !settings.tunnel_id.starts_with("tunnel_")
        || !valid_id(&settings.tunnel_id)
        || settings
            .organization_id
            .as_ref()
            .is_some_and(|o| !valid_id(o))
    {
        return Err(failure(
            "Provide a supported settings version, existing tunnel ID and valid organization ID.",
        ));
    }
    for path in [
        &settings.client_config,
        &settings.tunnel_client,
        &settings.mother_exe,
    ] {
        if !path.is_absolute() {
            return Err(failure("Connection paths must be absolute."));
        }
        quoted_path(path)?;
    }
    Ok(())
}
/// Suggest paths without reading credentials or changing files.
pub fn suggested_settings() -> Result<Settings> {
    let home = data_directory()?;
    let exe =
        std::env::current_exe().map_err(|_| failure("Cannot locate M.O.T.H.E.R executable."))?;
    let tunnel = home
        .join("tools/tunnel-client/0.0.20")
        .join(if cfg!(windows) {
            "tunnel-client.exe"
        } else {
            "tunnel-client"
        });
    Ok(Settings {
        version: 1,
        tunnel_id: String::new(),
        organization_id: None,
        client_config: home.join("clients/chatgpt.private.json"),
        tunnel_client: tunnel,
        mother_exe: exe,
    })
}
fn load_at(path: &Path) -> Result<Option<Settings>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(m) if !m.is_file() || m.file_type().is_symlink() => {
            return Err(failure(
                "Connection settings must be an ordinary owned file.",
            ));
        }
        Err(_) => return Err(failure("Cannot inspect connection settings.")),
        _ => {}
    }
    let s: Settings =
        serde_json::from_value(read_json(path, 16384).map_err(|_| {
            failure("Connection settings are invalid; existing file was preserved.")
        })?)
        .map_err(|_| failure("Connection settings are invalid; existing file was preserved."))?;
    validate(&s)?;
    Ok(Some(s))
}
/// Read non-secret connection settings.
pub fn load_settings() -> Result<Option<Settings>> {
    load_at(&settings_path()?)
}
fn private_directory(path: &Path) -> Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(path)
        .map_err(|_| failure("Cannot create private connection directory."))?;
    Ok(())
}
fn save_at(path: &Path, settings: &Settings) -> Result<()> {
    validate(settings)?;
    let existing = load_at(path)?;
    let parent = path
        .parent()
        .ok_or_else(|| failure("Settings require a parent directory."))?;
    private_directory(parent)?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| failure("Cannot prepare settings file."))?;
    serde_json::to_writer_pretty(&mut temp, settings)?;
    temp.write_all(b"\n")?;
    temp.as_file().sync_all()?;
    if existing.is_some() {
        temp.persist(path)
            .map_err(|_| failure("Cannot atomically save connection settings."))?;
    } else {
        temp.persist_noclobber(path).map_err(|_| {
            failure("Connection settings appeared concurrently; reload before saving.")
        })?;
    }
    Ok(())
}
/// Atomically save validated non-secret settings, preserving unknown files.
pub fn save_settings(settings: &Settings) -> Result<()> {
    validate(settings)?;
    if !settings.mother_exe.is_file() || !settings.tunnel_client.is_file() {
        return Err(failure(
            "Select existing M.O.T.H.E.R and official tunnel-client executables before saving setup.",
        ));
    }
    verify_grant(settings)?;
    let _lock = OperationLock::acquire()?;
    if let Some(previous) = load_settings()? {
        if &previous != settings && startup::enabled()? {
            return Err(failure(
                "Disable sign-in startup before changing saved connection settings.",
            ));
        }
        let current = inspect(&previous)?;
        if current.state != ConnectionState::Stopped || pending_path(&previous)?.exists() {
            return Err(failure(
                "Disconnect and verify the current runtime before changing settings.",
            ));
        }
    }
    save_at(&settings_path()?, settings)
}
fn credential_identity(settings: &Settings) -> Result<String> {
    let path = fs::canonicalize(&settings.client_config)
        .map_err(|_| failure("The separate ChatGPT client config must exist."))?;
    let encoded = serde_json::to_vec(&(path, &settings.organization_id, &settings.tunnel_id))?;
    Ok(format!("{:x}", Sha256::digest(encoded)))
}
fn alias(settings: &Settings) -> Result<String> {
    Ok(format!(
        "mother-chatgpt-{}",
        &credential_identity(settings)?[..24]
    ))
}
fn vault_entry(settings: &Settings) -> Result<keyring::Entry> {
    if !cfg!(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "ios"
    )) {
        return Err(failure(
            "Native credential storage is unavailable on this platform.",
        ));
    }
    keyring::Entry::new("MOTHER.ChatGPT.runtime.v1", &credential_identity(settings)?)
        .map_err(|_| failure("Native credential store is unavailable; no fallback is used."))
}
fn secret(settings: &Settings) -> Result<Option<Zeroizing<String>>> {
    decode_vault_result(vault_entry(settings)?.get_password())
}
fn decode_vault_result(value: keyring::Result<String>) -> Result<Option<Zeroizing<String>>> {
    match value {
        Ok(s) if !s.trim().is_empty() => Ok(Some(Zeroizing::new(s))),
        Ok(_) | Err(keyring::Error::NoEntry) => Ok(None),
        Err(_) => Err(failure("Native credential store is locked or unavailable.")),
    }
}
/// Save a runtime key in the OS credential store, bound to this connection identity.
pub fn store_key(settings: &Settings, key: &str) -> Result<()> {
    validate(settings)?;
    if key.trim().is_empty() || key.len() > 8192 || key.chars().any(char::is_control) {
        return Err(failure(
            "Enter a nonempty runtime key without control characters.",
        ));
    }
    vault_entry(settings)?
        .set_password(key)
        .map_err(|_| failure("Could not save the runtime key in the native credential store."))
}
fn command(settings: &Settings) -> Result<String> {
    Ok(format!(
        "{} mcp --config {}",
        quoted_path(&settings.mother_exe)?,
        quoted_path(&settings.client_config)?
    ))
}
fn profiles(settings: &Settings) -> Result<PathBuf> {
    Ok(root()?.join(alias(settings)?).join("profiles"))
}
fn parse_status(
    value: &serde_json::Value,
    alias: &str,
    tunnel: &str,
    profile_dir: &str,
    command: &str,
) -> Status {
    let unknown = || {
        Status::new(
            ConnectionState::Unknown,
            "Runtime inspection was incomplete or did not match this connection; no action was repeated.",
        )
    };
    if value["alias"].as_str() != Some(alias)
        || value["tunnel_id"].as_str() != Some(tunnel)
        || value["profile_name"].as_str() != Some(alias)
        || value["profile_dir"].as_str() != Some(profile_dir)
    {
        return unknown();
    }
    let (Some(running), Some(healthy), Some(ready)) = (
        value["process_running"].as_bool(),
        value["healthy"].as_bool(),
        value["ready"].as_bool(),
    ) else {
        return unknown();
    };
    if !running {
        return if healthy || ready {
            unknown()
        } else {
            Status::new(ConnectionState::Stopped, "The managed runtime is stopped.")
        };
    }
    if running
        && (value["process"]["target_value"].as_str() != Some(command)
            || value["process"]["tunnel_id"].as_str() != Some(tunnel)
            || value["process"]["alias"].as_str() != Some(alias))
    {
        return unknown();
    }
    let (Some(own_healthy), Some(own_ready)) = (
        value["local"]["health"]["healthz"]["ok"].as_bool(),
        value["local"]["health"]["readyz"]["ok"].as_bool(),
    ) else {
        return unknown();
    };
    if running && (own_healthy || own_ready) && !own_health_origin(value, alias) {
        return unknown();
    }
    let healthy = healthy && own_healthy;
    let ready = ready && own_ready;
    let state = if running && healthy && ready {
        ConnectionState::Ready
    } else if running {
        ConnectionState::Unhealthy
    } else if healthy || ready {
        return unknown();
    } else {
        ConnectionState::Stopped
    };
    let detail = match state {
        ConnectionState::Ready => {
            "Local runtime is running, healthy and ready. ChatGPT read/save acceptance remains separate."
        }
        ConnectionState::Unhealthy => {
            "The existing runtime is running but not ready. Inspect it before disconnecting or reconnecting."
        }
        _ => "The managed runtime is stopped.",
    };
    Status {
        state,
        detail: detail.into(),
        process_running: running,
        healthy,
        ready,
    }
}
fn own_health_origin(value: &serde_json::Value, alias: &str) -> bool {
    let Some(path) = value["health_url_file"].as_str() else {
        return false;
    };
    if value["process"]["health_url_file"].as_str() != Some(path)
        || value["local"]["health"]["path"].as_str() != Some(path)
        || Path::new(path).file_name().and_then(|s| s.to_str()) != Some(&format!("{alias}.url"))
    {
        return false;
    }
    let health = &value["local"]["health"];
    let Some(base) = health["base_url"].as_str() else {
        return false;
    };
    let Ok(url) = url::Url::parse(base) else {
        return false;
    };
    if url.scheme() != "http"
        || url.host_str() != Some("127.0.0.1")
        || url.port().is_none_or(|p| p == 0)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return false;
    }
    ["healthz", "readyz"].iter().all(|endpoint| {
        health[*endpoint]["url"].as_str()
            == Some(&format!("{}/{endpoint}", base.trim_end_matches('/')))
    })
}
fn inspect(settings: &Settings) -> Result<Status> {
    validate(settings)?;
    if !settings.tunnel_client.is_file() || !settings.mother_exe.is_file() {
        return Ok(Status::new(
            ConnectionState::MissingDependency,
            "Install the official tunnel-client and a stable M.O.T.H.E.R executable, then update setup paths.",
        ));
    }
    let alias = alias(settings)?;
    let args = vec![
        "runtimes".into(),
        "status".into(),
        alias.clone(),
        "--json".into(),
        "--control-plane-base-url".into(),
        "https://api.openai.com".into(),
        "--control-plane-url-path".into(),
        "/".into(),
    ];
    let output = process::run(settings, &args, None)?;
    if output.uncertain {
        return Ok(Status::new(
            ConnectionState::Unknown,
            "Runtime inspection timed out or exceeded its output limit; no action was repeated.",
        ));
    }
    if alias::is_absent(output.success, &output.stdout, &output.stderr, &alias) {
        return Ok(Status::new(
            ConnectionState::Stopped,
            "This connection has no managed runtime yet.",
        ));
    }
    if !output.success {
        return Ok(Status::new(
            ConnectionState::Unknown,
            diagnosis(&output.stderr),
        ));
    }
    let value = match serde_json::from_slice(&output.stdout) {
        Ok(v) => v,
        Err(_) => {
            return Ok(Status::new(
                ConnectionState::Unknown,
                "Runtime status was unavailable or invalid; no action was repeated.",
            ));
        }
    };
    Ok(parse_status(
        &value,
        &alias,
        &settings.tunnel_id,
        &profiles(settings)?.to_string_lossy(),
        &command(settings)?,
    ))
}
fn verify_grant(settings: &Settings) -> Result<()> {
    let config=load_config(&settings.client_config).map_err(|_|failure("Create or select the separate ChatGPT client config using mother grant; existing files were preserved."))?;
    mother::store::Store::open(&config.database_path).and_then(|store|store.authorize(&config.auth(),"read")).map_err(|_|failure("ChatGPT client read grant is missing, revoked or invalid. Use owner grant setup and select its private config."))
}
/// Inspect local readiness without changing settings or grants.
pub fn status() -> Result<Status> {
    let Some(settings) = load_settings()? else {
        return Ok(Status::new(
            ConnectionState::Unconfigured,
            "Set up an existing ChatGPT tunnel and separate client config.",
        ));
    };
    if verify_grant(&settings).is_err() {
        return Ok(Status::new(
            ConnectionState::MissingGrant,
            "Select a valid ChatGPT client config with an existing read grant.",
        ));
    }
    let current = inspect(&settings)?;
    if current.state == ConnectionState::Stopped && pending_path(&settings)?.exists() {
        return Ok(Status::new(
            ConnectionState::Unknown,
            "An earlier connect attempt is unresolved. Explicitly disconnect after inspection before reconnecting.",
        ));
    }
    if current.state == ConnectionState::Stopped && secret(&settings)?.is_none() {
        return Ok(Status::new(
            ConnectionState::MissingCredential,
            "Enter the runtime key in setup to save it in the OS credential store.",
        ));
    }
    Ok(current)
}
fn may_start(status: &Status, pending: bool) -> bool {
    status.state == ConnectionState::Stopped && !pending
}
fn pending_path(settings: &Settings) -> Result<PathBuf> {
    Ok(root()?.join(format!("{}.pending", alias(settings)?)))
}
fn clear_pending(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(failure("Cannot clear connection reconciliation marker.")),
    }
}
/// Connect once after reconciling existing state; uncertain effects require inspection.
pub fn connect() -> Result<Status> {
    let _lock = OperationLock::acquire()?;
    let Some(settings) = load_settings()? else {
        return Ok(Status::new(
            ConnectionState::Unconfigured,
            "Run interactive connection setup first.",
        ));
    };
    verify_grant(&settings)?;
    let existing = inspect(&settings)?;
    let pending = pending_path(&settings)?;
    if existing.state == ConnectionState::Ready {
        clear_pending(&pending)?;
        return Ok(existing);
    }
    if !may_start(&existing, pending.exists()) {
        if pending.exists() {
            return Ok(Status::new(
                ConnectionState::Unknown,
                "An earlier connection effect is unresolved. Refresh, then explicitly disconnect before trying again.",
            ));
        }
        return Ok(existing);
    }
    verify_no_other_runtime(&settings)?;
    let Some(key) = secret(&settings)? else {
        return Ok(Status::new(
            ConnectionState::MissingCredential,
            "Save a runtime key through interactive setup first.",
        ));
    };
    let alias = alias(&settings)?;
    let profiles = profiles(&settings)?;
    private_directory(&profiles)?;
    // This adapter owns this generated profile name only. Unknown files are never replaced.
    let profile = profiles.join(format!("{alias}.yaml"));
    if profile.exists() {
        let v = read_json(&profile, 65536)
            .map_err(|_| failure("Existing generated profile is unknown; it was preserved."))?;
        if v["control_plane"]["tunnel_id"].as_str() != Some(&settings.tunnel_id)
            || v["control_plane"]["api_key"].as_str() != Some("env:MOTHER_TUNNEL_RUNTIME_KEY")
        {
            return Err(failure(
                "Existing profile does not match this connection; it was preserved.",
            ));
        }
    }
    let mut args = vec![
        "runtimes".into(),
        "connect".into(),
        "--alias".into(),
        alias.clone(),
        "--tunnel-id".into(),
        settings.tunnel_id.clone(),
        "--profile".into(),
        alias,
        "--profile-dir".into(),
        profiles.to_string_lossy().into_owned(),
        "--mcp-command".into(),
        command(&settings)?,
        "--runtime-api-key".into(),
        "env:MOTHER_TUNNEL_RUNTIME_KEY".into(),
        "--control-plane-base-url".into(),
        "https://api.openai.com".into(),
        "--control-plane-url-path".into(),
        "/".into(),
        "--json".into(),
    ];
    if let Some(org) = &settings.organization_id {
        args.extend(["--organization-id".into(), org.clone()]);
    }
    perform_once(
        &pending,
        || process::run(&settings, &args, Some(&key)),
        || inspect(&settings),
    )
}
fn perform_once(
    pending: &Path,
    effect: impl FnOnce() -> Result<process::Output>,
    inspect: impl FnOnce() -> Result<Status>,
) -> Result<Status> {
    let mut marker = mother::config::create_private_file(pending)
        .map_err(|_| failure("An unresolved connection attempt already exists."))?;
    marker.write_all(b"connection attempt requires reconciliation\n")?;
    marker.sync_all()?;
    let result = effect();
    match result {
        Err(_) => {
            return Ok(Status::new(
                ConnectionState::Unknown,
                "Connect outcome is uncertain. Refresh status; do not replay automatically.",
            ));
        }
        Ok(output) if output.uncertain => {
            return Ok(Status::new(
                ConnectionState::Unknown,
                "Connect timed out or exceeded its output bound. Refresh status; do not replay automatically.",
            ));
        }
        Ok(output) if !output.success => {
            return Ok(Status::new(
                ConnectionState::Unknown,
                diagnosis(&output.stderr),
            ));
        }
        Ok(_) => {}
    }
    let after = inspect()?;
    if after.state == ConnectionState::Ready {
        clear_pending(pending)?;
    }
    Ok(after)
}
fn diagnosis(stderr: &str) -> &'static str {
    let lower = stderr.to_ascii_lowercase();
    if lower.contains("unauthorized")
        || lower.contains("forbidden")
        || lower.contains("status 401")
        || lower.contains("status 403")
    {
        "Runtime authorization failed. Check the saved runtime key and organization/tunnel permissions; no automatic retry was attempted."
    } else if lower.contains("429") || lower.contains("rate limit") {
        "The tunnel service rate-limited this request. Inspect status before a later explicit retry."
    } else if lower.contains("mcp")
        && (lower.contains("launch") || lower.contains("spawn") || lower.contains("initialize"))
    {
        "The local MCP process did not initialize. Verify the stable executable and separate client grant; no automatic retry was attempted."
    } else {
        "The runtime command failed. Inspect the installed tunnel-client and saved setup; external diagnostics were withheld to protect credentials."
    }
}
/// Stop only this identity's verified managed runtime, then verify stopped state.
pub fn disconnect() -> Result<Status> {
    let _lock = OperationLock::acquire()?;
    let Some(settings) = load_settings()? else {
        return Ok(Status::new(
            ConnectionState::Unconfigured,
            "No connection is configured.",
        ));
    };
    let current = inspect(&settings)?;
    if current.state == ConnectionState::Stopped {
        clear_pending(&pending_path(&settings)?)?;
        return Ok(current);
    }
    if !matches!(
        current.state,
        ConnectionState::Ready | ConnectionState::Unhealthy
    ) {
        return Ok(current);
    }
    let args = vec![
        "runtimes".into(),
        "stop".into(),
        alias(&settings)?,
        "--json".into(),
        "--control-plane-base-url".into(),
        "https://api.openai.com".into(),
        "--control-plane-url-path".into(),
        "/".into(),
    ];
    let output = process::run(&settings, &args, None)?;
    if !output.success && !output.uncertain {
        return Ok(Status::new(
            ConnectionState::Unknown,
            diagnosis(&output.stderr),
        ));
    }
    if output.uncertain {
        return Ok(Status::new(
            ConnectionState::Unknown,
            "Stop outcome is uncertain. Refresh status before any further action.",
        ));
    }
    let after = inspect(&settings)?;
    if after.state == ConnectionState::Stopped {
        clear_pending(&pending_path(&settings)?)?;
    }
    Ok(after)
}
struct OperationLock {
    _file: fs::File,
}
impl OperationLock {
    fn acquire() -> Result<Self> {
        let dir = root()?;
        private_directory(&dir)?;
        Self::acquire_at(&dir.join("operation.lock"))
    }
    fn acquire_at(path: &Path) -> Result<Self> {
        use std::io::Read;
        let mut file = match mother::config::create_private_file(path) {
            Ok(file) => {
                drop(file);
                fs::OpenOptions::new().read(true).write(true).open(path)
            }
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let meta = fs::symlink_metadata(path)
                    .map_err(|_| failure("Cannot inspect connection lock."))?;
                if !meta.is_file() || meta.file_type().is_symlink() {
                    return Err(failure("Connection lock is not an ordinary file."));
                }
                fs::OpenOptions::new().read(true).write(true).open(path)
            }
            Err(_) => return Err(failure("Cannot create connection lock.")),
        }
        .map_err(|_| failure("Cannot open connection lock."))?;
        fs2::FileExt::try_lock_exclusive(&file)
            .map_err(|_| failure("Another connection operation is active."))?;
        let mut content = String::new();
        std::io::Read::by_ref(&mut file)
            .take(128)
            .read_to_string(&mut content)
            .map_err(|_| failure("Cannot inspect connection lock."))?;
        if !content.is_empty() && content != "MOTHER connection operation lock v1\n" {
            return Err(failure(
                "Connection lock is not recognized; it was preserved.",
            ));
        }
        if content.is_empty() {
            file.write_all(b"MOTHER connection operation lock v1\n")?;
            file.sync_all()?;
        }
        Ok(Self { _file: file })
    }
}
/// Enable or disable explicit Windows sign-in startup without storing keys.
pub fn set_startup(value: Startup) -> Result<()> {
    startup::set(value)
}
/// Inspect whether the owned Windows startup launcher is installed.
pub fn startup_enabled() -> Result<bool> {
    startup::enabled()
}

fn other_alias_stopped(value: &serde_json::Value, alias: &str, tunnel: &str) -> bool {
    value["alias"].as_str() == Some(alias)
        && value["tunnel_id"].as_str() == Some(tunnel)
        && ["process_running", "healthy", "ready"]
            .iter()
            .all(|key| value[*key].as_bool() == Some(false))
}
fn verify_no_other_runtime(settings: &Settings) -> Result<()> {
    let started = std::time::Instant::now();
    let budget = std::time::Duration::from_secs(24);
    let own = alias(settings)?;
    let common = [
        "--json",
        "--control-plane-base-url",
        "https://api.openai.com",
        "--control-plane-url-path",
        "/",
    ];
    let mut args = vec!["runtimes".into(), "list".into()];
    args.extend(common.map(String::from));
    let output = process::run_with_timeout(
        settings,
        &args,
        None,
        budget.saturating_sub(started.elapsed()),
    )?;
    if output.uncertain || !output.success {
        return Err(failure(
            "Could not reconcile existing runtime aliases; connect was not attempted.",
        ));
    }
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| failure("Runtime inventory was invalid; connect was not attempted."))?;
    let aliases = value["aliases"]
        .as_array()
        .ok_or_else(|| failure("Runtime inventory was incomplete; connect was not attempted."))?;
    for record in aliases {
        let name = record["alias"]
            .as_str()
            .ok_or_else(|| failure("Runtime inventory identity was incomplete."))?;
        let tunnel = record["tunnel_id"]
            .as_str()
            .ok_or_else(|| failure("Runtime inventory tunnel identity was incomplete."))?;
        if name == own || tunnel != settings.tunnel_id {
            continue;
        }
        if !valid_id(name) {
            return Err(failure("Runtime inventory alias was invalid."));
        }
        let mut args = vec!["runtimes".into(), "status".into(), name.into()];
        args.extend(common.map(String::from));
        let output = process::run_with_timeout(
            settings,
            &args,
            None,
            budget.saturating_sub(started.elapsed()),
        )?;
        let value = serde_json::from_slice(&output.stdout).unwrap_or(serde_json::Value::Null);
        if output.uncertain
            || !output.success
            || !other_alias_stopped(&value, name, &settings.tunnel_id)
        {
            return Err(failure(
                "Another runtime alias uses this tunnel or could not be verified stopped. Inspect and explicitly stop it before connecting here; it was not changed.",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn readiness_requires_all_three_signals_and_identity() {
        let value = serde_json::json!({"alias":"owned", "tunnel_id":"tunnel_test", "profile_name":"owned", "profile_dir":"/profiles", "process_running":true, "healthy":true, "ready":true,"health_url_file":"/state/health/owned.url", "local":{"health":{"path":"/state/health/owned.url","base_url":"http://127.0.0.1:49200","healthz":{"ok":true,"url":"http://127.0.0.1:49200/healthz"},"readyz":{"ok":true,"url":"http://127.0.0.1:49200/readyz"}}}, "process":{"target_value":"mother mcp", "tunnel_id":"tunnel_test", "alias":"owned", "health_url_file":"/state/health/owned.url"}});
        assert_eq!(
            parse_status(&value, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Ready
        );
        let mut fallback = value.clone();
        fallback["local"]["health"]["healthz"]["ok"] = false.into();
        fallback["local"]["health"]["readyz"]["ok"] = false.into();
        assert_eq!(
            parse_status(&fallback, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Unhealthy
        );
        let mut wrong_origin = value.clone();
        wrong_origin["local"]["health"]["readyz"]["url"] = "http://127.0.0.1:49999/readyz".into();
        assert_eq!(
            parse_status(
                &wrong_origin,
                "owned",
                "tunnel_test",
                "/profiles",
                "mother mcp"
            )
            .state,
            ConnectionState::Unknown
        );
        let mut mismatch = value.clone();
        mismatch["tunnel_id"] = "different".into();
        assert_eq!(
            parse_status(&mismatch, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Unknown
        );
        let mut missing = value.clone();
        missing.as_object_mut().unwrap().remove("healthy");
        assert_eq!(
            parse_status(&missing, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Unknown
        );
        let mut unhealthy = value;
        unhealthy["ready"] = false.into();
        assert_eq!(
            parse_status(
                &unhealthy,
                "owned",
                "tunnel_test",
                "/profiles",
                "mother mcp"
            )
            .state,
            ConnectionState::Unhealthy
        );
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn command_quotes_spaces_and_rejects_parser_controls() {
        assert_eq!(
            quoted_path(std::path::Path::new("C:/Program Files/MOTHER/mother.exe")).unwrap(),
            "\"C:/Program Files/MOTHER/mother.exe\""
        );
        for value in ["C:/bad\"x", "C:/bad\nx", "C:/bad$x", "C:/bad`x", "C:/bad%x"] {
            assert!(quoted_path(std::path::Path::new(value)).is_err());
        }
    }
    #[test]
    fn missing_identity_never_means_ready() {
        assert_eq!(
            parse_status(
                &json!({"process_running":true,"healthy":true,"ready":true}),
                "a",
                "t",
                "p",
                "m"
            )
            .state,
            ConnectionState::Unknown
        );
    }
    #[test]
    fn credentials_are_bound_to_config_organization_and_tunnel() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("client.json");
        std::fs::write(&p, "{}").unwrap();
        let mut s = Settings {
            version: 1,
            tunnel_id: "tunnel_one".into(),
            organization_id: None,
            client_config: p,
            tunnel_client: dir.path().join("tunnel.exe"),
            mother_exe: dir.path().join("mother.exe"),
        };
        let first = credential_identity(&s).unwrap();
        s.tunnel_id = "tunnel_two".into();
        assert_ne!(first, credential_identity(&s).unwrap());
        s.tunnel_id = "tunnel_one".into();
        s.organization_id = Some("org-two".into());
        assert_ne!(first, credential_identity(&s).unwrap());
    }
    #[test]
    fn settings_refuse_unknown_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, "unowned").unwrap();
        let s = Settings {
            version: 1,
            tunnel_id: "tunnel_test".into(),
            organization_id: None,
            client_config: dir.path().join("client.json"),
            tunnel_client: dir.path().join("tunnel.exe"),
            mother_exe: dir.path().join("mother.exe"),
        };
        assert!(save_at(&path, &s).is_err());
        assert_eq!(std::fs::read_to_string(path).unwrap(), "unowned");
    }
    #[test]
    fn uncertain_and_unhealthy_connections_are_not_replayed() {
        assert!(!may_start(
            &Status::new(ConnectionState::Unknown, "unknown"),
            false
        ));
        assert!(!may_start(
            &Status::new(ConnectionState::Unhealthy, "unhealthy"),
            false
        ));
        assert!(!may_start(
            &Status::new(ConnectionState::Stopped, "stopped"),
            true
        ));
        assert!(may_start(
            &Status::new(ConnectionState::Stopped, "stopped"),
            false
        ));
    }
}

#[cfg(test)]
mod conflicts_tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn other_alias_status_must_prove_stopped_before_connecting() {
        assert!(!other_alias_stopped(
            &json!({"alias":"legacy","tunnel_id":"tunnel_test","process_running":true,"healthy":true,"ready":true}),
            "legacy",
            "tunnel_test"
        ));
        assert!(!other_alias_stopped(
            &json!({"alias":"legacy","process_running":false,"healthy":false,"ready":false}),
            "legacy",
            "tunnel_test"
        ));
        assert!(other_alias_stopped(
            &json!({"alias":"legacy","tunnel_id":"tunnel_test","process_running":false,"healthy":false,"ready":false}),
            "legacy",
            "tunnel_test"
        ));
    }
}

#[cfg(test)]
mod lock_tests {
    use super::*;
    #[test]
    fn operation_lock_excludes_concurrent_work_and_releases_on_drop() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("operation.lock");
        let first = OperationLock::acquire_at(&path).unwrap();
        assert!(OperationLock::acquire_at(&path).is_err());
        drop(first);
        assert!(OperationLock::acquire_at(&path).is_ok());
    }
}

#[cfg(test)]
mod execution_tests {
    use super::*;
    #[test]
    fn timeout_after_effect_keeps_marker_and_blocks_replay() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join("pending");
        let calls = std::cell::Cell::new(0);
        let result = perform_once(
            &marker,
            || {
                calls.set(calls.get() + 1);
                Ok(process::Output {
                    stdout: vec![],
                    stderr: String::new(),
                    uncertain: true,
                    success: false,
                })
            },
            || panic!("timeout must not claim inspection"),
        )
        .unwrap();
        assert_eq!(result.state, ConnectionState::Unknown);
        assert!(marker.is_file());
        assert!(
            perform_once(
                &marker,
                || {
                    calls.set(calls.get() + 1);
                    unreachable!()
                },
                || unreachable!()
            )
            .is_err()
        );
        assert_eq!(calls.get(), 1);
    }
    #[test]
    fn failed_command_never_accepts_ready_output_or_exposes_secret() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join("pending");
        let result = perform_once(
            &marker,
            || {
                Ok(process::Output {
                    stdout: br#"{"ready":true}"#.to_vec(),
                    stderr: "unauthorized synthetic-secret".into(),
                    uncertain: false,
                    success: false,
                })
            },
            || panic!("failed command must not claim ready"),
        )
        .unwrap();
        assert_eq!(result.state, ConnectionState::Unknown);
        assert!(result.detail.contains("authorization"));
        assert!(!result.detail.contains("synthetic-secret"));
    }
}

#[cfg(test)]
mod vault_tests {
    use super::*;
    #[test]
    fn vault_absence_and_failure_are_distinct_and_never_expose_details() {
        assert!(
            decode_vault_result(Err(keyring::Error::NoEntry))
                .unwrap()
                .is_none()
        );
        let error = decode_vault_result(Err(keyring::Error::NoStorageAccess(Box::new(
            std::io::Error::other("synthetic-secret"),
        ))))
        .unwrap_err();
        assert!(!error.to_string().contains("synthetic-secret"));
        assert!(error.to_string().contains("unavailable"));
        assert_eq!(
            decode_vault_result(Ok("synthetic-key".into()))
                .unwrap()
                .unwrap()
                .as_str(),
            "synthetic-key"
        );
    }
}

#[cfg(test)]
mod stopped_tests {
    use super::*;
    #[test]
    fn matching_stopped_alias_does_not_require_local_health_probes() {
        let mut value = serde_json::json!({"alias":"owned","tunnel_id":"tunnel_test","profile_name":"owned","profile_dir":"/profiles","process_running":false,"healthy":false,"ready":false,"process":null,"local":{"health":{"exists":false}}});
        assert_eq!(
            parse_status(&value, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Stopped
        );
        value["healthy"] = true.into();
        assert_eq!(
            parse_status(&value, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Unknown
        );
        value["healthy"] = false.into();
        value["ready"] = true.into();
        assert_eq!(
            parse_status(&value, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Unknown
        );
        value["ready"] = false.into();
        value["tunnel_id"] = "other".into();
        assert_eq!(
            parse_status(&value, "owned", "tunnel_test", "/profiles", "mother mcp").state,
            ConnectionState::Unknown
        );
    }
}
