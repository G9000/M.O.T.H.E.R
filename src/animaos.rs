//! Optional owner-authorized connection to a loopback AnimaOS daemon.
use mother::{
    access::{AccessMode, ClientAccess},
    config::{create_private_file, load_config, read_json},
    error::{Error, Result},
    store::Store,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};
use zeroize::{Zeroize, Zeroizing};

const LABEL: &str = "M.O.T.H.E.R";
const MAX_BODY: u64 = 1024 * 1024;
// Includes the largest accepted 10,000-record policy and reconciliation metadata.
const MAX_BINDING_BYTES: usize = 512 * 1024;
const REQUIRED_TOOLS: [&str; 6] = [
    "mother_get_context",
    "mother_search_context",
    "mother_get_record",
    "mother_get_receipt",
    "mother_propose_memory",
    "mother_get_proposal",
];

fn failure(message: &str) -> Error {
    Error::domain("ANIMAOS_CONNECTION", message)
}
fn unknown() -> Error {
    Error::domain(
        "ANIMAOS_UNKNOWN",
        "AnimaOS outcome is uncertain. Check connection state before trying Connect again; no action was repeated.",
    )
}

/// An agent offered by the daemon's transcript-free summary endpoint.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
}

/// Credentials live only in this session and are never serialized or printed.
pub struct Session {
    endpoint: String,
    owner_token: Zeroizing<String>,
    api_key: Zeroizing<String>,
    http: ureq::Agent,
}
impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Connection {
    id: String,
    agent_id: String,
    label: String,
    transport: String,
    stdio: Option<Stdio>,
    endpoint: String,
    enabled: bool,
    version: u64,
    state: String,
    tool_access: Option<String>,
    physical_calls_active: bool,
    discovery_id: Option<String>,
    candidates: Vec<Tool>,
    #[serde(default)]
    reviewed_tools: Vec<Tool>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Stdio {
    executable: String,
    args: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tool {
    remote_name: String,
    #[serde(default)]
    descriptor_digest: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Binding {
    workspace_id: String,
    generation: String,
    endpoint: String,
    pending: Option<String>,
    #[serde(default)]
    access: Option<ClientAccess>,
    #[serde(default)]
    pending_binding: Option<WorkspaceBinding>,
    #[serde(default)]
    pending_review: Option<ReaderReview>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReaderReview {
    connection_id: String,
    expected_version: u64,
    discovery_id: String,
    descriptor_digest: String,
}
impl ReaderReview {
    fn applied(&self, connection: &Connection) -> bool {
        connection.id == self.connection_id
            && self.expected_version.checked_add(1) == Some(connection.version)
            && connection.discovery_id.as_deref() == Some(self.discovery_id.as_str())
            && has_discovery(connection)
            && reader_reviewed(connection, &self.descriptor_digest)
    }
}
impl Binding {
    fn encoded(&self) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self)
            .map_err(|_| failure("Cannot encode private AnimaOS reconciliation state."))?;
        if bytes.len() > MAX_BINDING_BYTES {
            return Err(failure(
                "AnimaOS setup state exceeds the supported 512 KiB limit.",
            ));
        }
        Ok(bytes)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WorkspaceBinding {
    connection_id: String,
    tool: String,
    mode: AccessMode,
    workspace_id: String,
    generation: String,
    subject: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceContext {
    revision: String,
    binding: Option<WorkspaceBinding>,
    connections: Vec<BindingConnection>,
}
#[derive(Deserialize)]
struct BindingConnection {
    id: String,
    state: String,
    tool: Option<String>,
}

struct Local {
    _lock: crate::lock::FileLock,
    binding_path: PathBuf,
    config_path: PathBuf,
    binding: Binding,
    spec: Stdio,
}

fn normalized(path: &Path) -> Result<PathBuf> {
    if !path.is_absolute() {
        return Err(failure("MOTHER paths must be absolute."));
    }
    fs::canonicalize(path).map_err(|_| failure("Cannot resolve the selected MOTHER path."))
}
fn text_path(path: &Path) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| failure("MOTHER path must be valid Unicode."))
}

impl Local {
    fn open(agent_id: &str, root: &Path, exe: &Path, endpoint: &str) -> Result<Self> {
        Self::open_with_access(agent_id, root, exe, endpoint, None)
    }
    fn open_with_access(
        agent_id: &str,
        root: &Path,
        exe: &Path,
        endpoint: &str,
        access: Option<&ClientAccess>,
    ) -> Result<Self> {
        if let Some(access) = access
            && (access.mode == AccessMode::Legacy || access.subject.as_deref() != Some(agent_id))
        {
            return Err(failure(
                "Choose companion or shared access for this exact agent.",
            ));
        }
        let root = normalized(root)?;
        let exe = normalized(exe)?;
        if !exe.is_file() {
            return Err(failure("The selected MOTHER executable is unavailable."));
        }
        let database = normalized(&root.join("context.sqlite"))?;
        let clients = root.join("clients");
        fs::create_dir_all(&clients)
            .map_err(|_| failure("Cannot create the private clients directory."))?;
        let clients = normalized(&clients)?;
        if clients.parent() != Some(root.as_path()) {
            return Err(failure(
                "The clients directory must belong to this MOTHER workspace.",
            ));
        }
        let digest = format!("{:x}", Sha256::digest(agent_id.as_bytes()));
        let name = match access.map(|a| a.mode) {
            Some(AccessMode::Companion) => format!("animaos-c-{digest}"),
            Some(AccessMode::Shared) => format!("animaos-s-{digest}"),
            _ => format!("animaos-{digest}"),
        };
        let lock_path = clients.join(format!("{name}.lock"));
        let lock = match create_private_file(&lock_path) {
            Ok(file) => file,
            Err(_) => fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&lock_path)
                .map_err(|_| failure("Cannot open the AnimaOS connection lock."))?,
        };
        let lock = crate::lock::FileLock::try_exclusive(lock).map_err(|_| {
            failure(
                "This agent is already being connected from MOTHER. Wait for that attempt to finish.",
            )
        })?;
        let config_path = clients.join(format!("{name}.private.json"));
        let binding_path = clients.join(format!("{name}.binding.json"));
        let mut store = Store::open(&database)
            .map_err(|_| failure("Open an initialized MOTHER workspace before connecting."))?;
        let workspace = store
            .workspace()
            .map_err(|_| failure("Cannot verify the current MOTHER workspace."))?;
        let binding = if config_path.exists() {
            let binding: Binding = serde_json::from_value(
                read_json(&binding_path, MAX_BINDING_BYTES).map_err(|_| {
                    failure(
                        "Existing AnimaOS grant has no valid workspace binding; it was preserved.",
                    )
                })?,
            )
            .map_err(|_| {
                failure("Existing AnimaOS workspace binding is invalid; it was preserved.")
            })?;
            if binding.workspace_id != workspace["id"]
                || binding.generation != workspace["generation"]
                || binding.endpoint != endpoint
                || binding.access.as_ref() != access
            {
                return Err(failure(
                    "Existing AnimaOS grant belongs to another workspace, restore generation or daemon address; it was preserved.",
                ));
            }
            binding
        } else {
            if binding_path.exists() {
                return Err(failure(
                    "AnimaOS workspace binding exists without its grant. Resolve local setup before reconnecting.",
                ));
            }
            let binding = Binding {
                workspace_id: workspace["id"].as_str().unwrap_or_default().into(),
                generation: workspace["generation"].as_str().unwrap_or_default().into(),
                endpoint: endpoint.into(),
                pending: None,
                access: access.cloned(),
                pending_binding: None,
                pending_review: None,
            };
            // Reject unpersistable setup before provisioning any credential.
            binding.encoded()?;
            let scopes = ["read".into(), "context:propose".into()];
            match access {
                Some(access) => {
                    store.grant_to_file_with_access(&name, &scopes, &config_path, &database, access)
                }
                None => store.grant_to_file(&name, &scopes, &config_path, &database),
            }
            .map_err(|_| {
                failure(
                    "Cannot create the separate AnimaOS grant; existing clients were preserved.",
                )
            })?;
            binding
        };
        let mut config = load_config(&config_path).map_err(|_| {
            failure("AnimaOS private grant configuration is invalid; it was preserved.")
        })?;
        let validation = (|| -> Result<()> {
            if config.client_id != name || normalized(&config.database_path)? != database {
                return Err(failure(
                    "AnimaOS private grant does not match this workspace and agent.",
                ));
            }
            for scope in ["read", "context:propose"] {
                store.authorize(&config.auth(),scope).map_err(|_|failure("AnimaOS grant is missing, revoked or invalid. Existing credentials were preserved."))?;
            }
            if store.access_for(&config.auth())? != access.cloned().unwrap_or_default() {
                return Err(failure(
                    "Existing grant access differs from this selection. It was preserved; review or revoke it explicitly before replacing setup.",
                ));
            }
            // Read only the selected local client's scopes; do not widen existing grants.
            let db = rusqlite::Connection::open_with_flags(
                &database,
                rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
            )
            .map_err(|_| failure("Cannot inspect AnimaOS client scopes."))?;
            let raw: String = db
                .query_row("SELECT scopes FROM clients WHERE id=?", [&name], |row| {
                    row.get(0)
                })
                .map_err(|_| failure("Cannot inspect AnimaOS client scopes."))?;
            let mut scopes: Vec<String> = serde_json::from_str(&raw)
                .map_err(|_| failure("AnimaOS client scopes are invalid."))?;
            scopes.sort();
            if scopes != ["context:propose", "read"] {
                return Err(failure(
                    "AnimaOS client must have exactly read and context:propose scopes. Existing grant was preserved.",
                ));
            }
            Ok(())
        })();
        config.token.zeroize();
        validation?;
        let local = Self {
            _lock: lock,
            binding_path,
            config_path: config_path.clone(),
            binding,
            spec: Stdio {
                executable: text_path(&exe)?,
                args: vec!["mcp".into(), "--config".into(), text_path(&config_path)?],
            },
        };
        local.save()?;
        Ok(local)
    }
    fn save(&self) -> Result<()> {
        let bytes = self.binding.encoded()?;
        let parent = self
            .binding_path
            .parent()
            .ok_or_else(|| failure("Cannot locate the private connection state."))?;
        let mut file = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| failure("Cannot save private AnimaOS reconciliation state."))?;
        file.write_all(&bytes)
            .map_err(|_| failure("Cannot save private AnimaOS reconciliation state."))?;
        file.flush()
            .and_then(|()| file.as_file().sync_all())
            .map_err(|_| failure("Cannot save private AnimaOS reconciliation state."))?;
        file.persist(&self.binding_path)
            .map_err(|_| failure("Cannot save private AnimaOS reconciliation state."))?;
        Ok(())
    }
    fn pending(&mut self, operation: Option<&str>) -> Result<()> {
        self.binding.pending = operation.map(str::to_owned);
        self.save()
    }
    fn matches(&self, connection: &Connection) -> bool {
        connection.transport == "stdio"
            && connection.endpoint.is_empty()
            && connection.stdio.as_ref() == Some(&self.spec)
    }
    fn relevant(&self, connection: &Connection) -> bool {
        connection.label == LABEL
            || connection.label.eq_ignore_ascii_case("mother")
            || connection.stdio.as_ref().is_some_and(|spec| {
                spec.executable == self.spec.executable
                    || spec
                        .args
                        .iter()
                        .any(|arg| arg == &self.config_path.to_string_lossy())
            })
    }
}

impl Session {
    /// Connect using an explicitly entered owner bearer token.
    pub fn new(endpoint: &str, key: &str) -> Result<Self> {
        Self::with_api_key(endpoint, key, "")
    }
    /// Keep optional API-gate and owner credentials separate for the session.
    pub fn with_api_key(endpoint: &str, owner_token: &str, api_key: &str) -> Result<Self> {
        let mut url = url::Url::parse(endpoint)
            .map_err(|_| failure("Enter a loopback HTTP daemon address."))?;
        let authority = endpoint
            .strip_prefix("http://")
            .and_then(|rest| rest.split(['/', '?', '#']).next())
            .ok_or_else(|| failure("Use http://127.0.0.1:8080 or http://[::1]:8080."))?;
        let valid_authority = ["127.0.0.1", "[::1]", "localhost"].iter().any(|host| {
            authority == *host
                || authority
                    .strip_prefix(host)
                    .and_then(|s| s.strip_prefix(':'))
                    .is_some_and(|port| {
                        !port.is_empty() && port.bytes().all(|b| b.is_ascii_digit())
                    })
        });
        if !valid_authority
            || url.scheme() != "http"
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
            || endpoint.chars().any(char::is_whitespace)
        {
            return Err(failure(
                "Use a plain loopback HTTP address without credentials, paths, queries or fragments.",
            ));
        }
        if url.host_str() == Some("localhost") {
            url.set_host(Some("127.0.0.1"))
                .map_err(|_| failure("Invalid loopback address."))?;
        }
        for secret in [owner_token, api_key] {
            if secret.len() > 8192 || !secret.bytes().all(|b| (0x21..=0x7e).contains(&b)) {
                return Err(failure(
                    "Entered credential contains unsupported characters or is too long.",
                ));
            }
        }
        let http = ureq::Agent::config_builder()
            .proxy(None)
            .max_redirects(0)
            .http_status_as_error(false)
            .max_idle_connections(0)
            .timeout_global(Some(Duration::from_secs(30)))
            .timeout_connect(Some(Duration::from_secs(2)))
            .max_response_header_size(16384)
            .build()
            .into();
        Ok(Self {
            endpoint: url.as_str().trim_end_matches('/').into(),
            owner_token: Zeroizing::new(owner_token.into()),
            api_key: Zeroizing::new(api_key.into()),
            http,
        })
    }
    fn request(&self, method: &str, path: &str, body: Option<&Value>) -> Result<Value> {
        let url = format!("{}{path}", self.endpoint);
        let authorization = Zeroizing::new(format!("Bearer {}", self.owner_token.as_str()));
        let response = match method {
            "GET" => self
                .http
                .get(&url)
                .header("authorization", authorization.as_str())
                .header("x-api-key", self.api_key.as_str())
                .header("connection", "close")
                .call(),
            "POST" => self
                .http
                .post(&url)
                .header("authorization", authorization.as_str())
                .header("x-api-key", self.api_key.as_str())
                .header("connection", "close")
                .header("content-type", "application/json")
                .send(body.unwrap_or(&Value::Null).to_string().as_bytes()),
            "PATCH" => self
                .http
                .patch(&url)
                .header("authorization", authorization.as_str())
                .header("x-api-key", self.api_key.as_str())
                .header("connection", "close")
                .header("content-type", "application/json")
                .send(body.unwrap_or(&Value::Null).to_string().as_bytes()),
            "PUT" => self
                .http
                .put(&url)
                .header("authorization", authorization.as_str())
                .header("x-api-key", self.api_key.as_str())
                .header("connection", "close")
                .header("content-type", "application/json")
                .send(body.unwrap_or(&Value::Null).to_string().as_bytes()),
            _ => return Err(failure("Unsupported connection operation.")),
        };
        let mut response = response.map_err(|_| unknown())?;
        match response.status().as_u16() {
            200..=299 => {}
            401 | 403 => {
                return Err(failure(
                    "AnimaOS owner authorization was denied. Enter the owner bearer token and, if configured separately, the API key.",
                ));
            }
            409 => {
                return Err(failure(
                    "AnimaOS connection changed. Refresh and inspect its current version before trying Connect again.",
                ));
            }
            300..=399 => {
                return Err(failure(
                    "AnimaOS redirected the request. Redirects are disabled; enter the direct loopback daemon address.",
                ));
            }
            400 | 404 | 405 | 413 | 429 => {
                return Err(failure(
                    "AnimaOS rejected this connection request. Check the agent, connection and daemon compatibility.",
                ));
            }
            _ => return Err(unknown()),
        }
        let text = response
            .body_mut()
            .with_config()
            .limit(MAX_BODY)
            .read_to_string()
            .map_err(|_| unknown())?;
        serde_json::from_str(&text).map_err(|_| unknown())
    }
    /// List agent names and IDs without requesting conversation transcripts.
    pub fn agents(&self) -> Result<Vec<Agent>> {
        #[derive(Deserialize)]
        struct Snapshot {
            state: Agent,
        }
        #[derive(Deserialize)]
        struct Envelope {
            agents: Vec<Snapshot>,
        }
        let envelope: Envelope =
            serde_json::from_value(self.request("GET", "/api/agents?view=summary", None)?)
                .map_err(|_| failure("AnimaOS agent summaries are unavailable or incompatible."))?;
        if envelope.agents.len() > 1024 {
            return Err(failure("AnimaOS agent list exceeds the supported limit."));
        }
        let mut agents = Vec::with_capacity(envelope.agents.len());
        for snapshot in envelope.agents {
            valid_id(&snapshot.state.id)?;
            if agents
                .iter()
                .any(|agent: &Agent| agent.id == snapshot.state.id)
            {
                return Err(failure("AnimaOS returned duplicate agent identities."));
            }
            agents.push(snapshot.state);
        }
        Ok(agents)
    }
    fn one(
        &self,
        method: &str,
        path: &str,
        body: &Value,
        agent: &str,
        local: &Local,
    ) -> Result<Connection> {
        let value = self.request(method, path, Some(body))?;
        let connection: Connection =
            serde_json::from_value(value.get("connection").cloned().ok_or_else(unknown)?)
                .map_err(|_| unknown())?;
        validate_connection(&connection, agent).map_err(|_| unknown())?;
        if !local.matches(&connection) {
            return Err(unknown());
        }
        Ok(connection)
    }
    fn mutate(
        &self,
        local: &mut Local,
        operation: &str,
        method: &str,
        path: &str,
        body: &Value,
        agent: &str,
    ) -> Result<Connection> {
        local.pending(Some(operation))?;
        match self.one(method, path, body, agent, local) {
            Ok(connection) => {
                let expected = body["expectedVersion"].as_u64();
                let consistent = match operation {
                    "create" => !connection.enabled && connection.state == "disconnected",
                    "enable" => {
                        path.rsplit('/').next() == Some(connection.id.as_str())
                            && connection.enabled
                            && expected.and_then(|value| value.checked_add(1))
                                == Some(connection.version)
                    }
                    "connect" => {
                        path.rsplit('/').nth(1) == Some(connection.id.as_str())
                            && expected == Some(connection.version)
                            && has_discovery(&connection)
                    }
                    _ => false,
                };
                if !consistent {
                    return Err(unknown());
                }
                local.pending(None)?;
                Ok(connection)
            }
            Err(error) => {
                if error.code() != "ANIMAOS_UNKNOWN" {
                    local.pending(None)?;
                }
                Err(error)
            }
        }
    }
    /// Create or reconcile the selected agent's separate, proposal-only MOTHER connection.
    pub fn connect(&self, agent_id: &str, root: &Path, exe: &Path) -> Result<String> {
        valid_id(agent_id)?;
        let mut local = Local::open(agent_id, root, exe, &self.endpoint)?;
        let connection = self.connect_local(agent_id, &mut local)?;
        discovery_status(&connection)
    }

    /// Configure an explicitly scoped connection and optional per-agent identity binding.
    /// Existing legacy grants and differing bindings are never replaced automatically.
    pub fn connect_with_access(
        &self,
        agent_id: &str,
        root: &Path,
        exe: &Path,
        access: &ClientAccess,
    ) -> Result<String> {
        valid_id(agent_id)?;
        let mut local = Local::open_with_access(agent_id, root, exe, &self.endpoint, Some(access))?;
        let path = format!("/api/agents/{}/workspace-context", segment(agent_id));
        if let Some(pending) = local.binding.pending_binding.clone() {
            let current = self.workspace_context(&path)?;
            if current.binding.as_ref() != Some(&pending) {
                return Err(unknown());
            }
            local.binding.pending_binding = None;
            local.save()?;
        }
        let mut connection = self.connect_local(agent_id, &mut local)?;
        discovery_status(&connection)?;
        if connection.tool_access.as_deref() == Some("automatic") {
            connection = self.review_reader(agent_id, &mut local, connection)?;
        }
        let current = self.workspace_context(&path)?;
        let desired = WorkspaceBinding {
            connection_id: connection.id.clone(),
            tool: "mother_get_context".into(),
            mode: access.mode,
            workspace_id: local.binding.workspace_id.clone(),
            generation: local.binding.generation.clone(),
            subject: agent_id.into(),
        };
        if current.binding.as_ref() == Some(&desired) {
            return Ok(binding_status(access.mode));
        }
        if current.binding.is_some() {
            return Err(failure(
                "This agent already has a different workspace-context binding. Disable or replace it explicitly in AnimaOS settings; the existing binding was preserved.",
            ));
        }
        if connection.state != "ready"
            || !current.connections.iter().any(|c| {
                c.id == connection.id
                    && c.state == "ready"
                    && c.tool.as_deref() == Some("mother_get_context")
            })
        {
            return Ok("MOTHER tools discovered. Complete owner tool review in AnimaOS, then return to Connect to finish the workspace binding. No identity or primary-memory binding was installed.".into());
        }
        local.binding.pending_binding = Some(desired.clone());
        local.save()?;
        let result = self.request(
            "PUT",
            &path,
            Some(&json!({"expectedRevision":current.revision,"binding":desired})),
        );
        match result {
            Ok(value) => {
                let saved: WorkspaceContext =
                    serde_json::from_value(value).map_err(|_| unknown())?;
                if saved.binding.as_ref() != Some(&desired) {
                    return Err(unknown());
                }
            }
            Err(error) if error.code() == "ANIMAOS_UNKNOWN" => {
                let current = self.workspace_context(&path)?;
                if current.binding.as_ref() != Some(&desired) {
                    return Err(unknown());
                }
            }
            Err(error) => {
                local.binding.pending_binding = None;
                local.save()?;
                return Err(error);
            }
        }
        local.binding.pending_binding = None;
        local.save()?;
        Ok(binding_status(access.mode))
    }
    fn review_reader(
        &self,
        agent: &str,
        local: &mut Local,
        connection: Connection,
    ) -> Result<Connection> {
        let candidates = connection
            .candidates
            .iter()
            .filter(|tool| tool.remote_name == "mother_get_context")
            .collect::<Vec<_>>();
        let [reader] = candidates.as_slice() else {
            return Err(failure(
                "Expected exactly one current MOTHER context reader descriptor.",
            ));
        };
        let digest = &reader.descriptor_digest;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
        {
            return Err(failure(
                "MOTHER context reader descriptor digest is unavailable or invalid; no review was sent.",
            ));
        }
        if reader_reviewed(&connection, digest) {
            return Ok(connection);
        }
        let review = ReaderReview {
            connection_id: connection.id.clone(),
            expected_version: connection.version,
            discovery_id: connection.discovery_id.clone().ok_or_else(unknown)?,
            descriptor_digest: digest.clone(),
        };
        let path = format!(
            "/api/agents/{}/mcp-connections/{}/review",
            segment(agent),
            segment(&connection.id)
        );
        local.binding.pending_review = Some(review.clone());
        local.save()?;
        let body = json!({"expectedVersion":review.expected_version,"discoveryId":review.discovery_id,"descriptorDigest":review.descriptor_digest});
        match self.one("POST", &path, &body, agent, local) {
            Ok(connection) => {
                if !review.applied(&connection) {
                    return Err(unknown());
                }
                local.binding.pending_review = None;
                local.save()?;
                Ok(connection)
            }
            Err(error) => {
                if error.code() != "ANIMAOS_UNKNOWN" {
                    local.binding.pending_review = None;
                    local.save()?;
                }
                Err(error)
            }
        }
    }
    fn workspace_context(&self, path: &str) -> Result<WorkspaceContext> {
        let context: WorkspaceContext = serde_json::from_value(self.request("GET", path, None)?)
            .map_err(|_| {
                failure("AnimaOS workspace-context settings are unavailable or incompatible.")
            })?;
        if context.revision.is_empty()
            || context.revision.len() > 256
            || context.connections.len() > 256
        {
            return Err(failure("AnimaOS workspace-context metadata is invalid."));
        }
        Ok(context)
    }
    fn connect_local(&self, agent_id: &str, local: &mut Local) -> Result<Connection> {
        let base = format!("/api/agents/{}/mcp-connections", segment(agent_id));
        let value = self.request("GET", &base, None)?;
        let connections: Vec<Connection> =
            serde_json::from_value(value.get("connections").cloned().ok_or_else(unknown)?)
                .map_err(|_| failure("AnimaOS MCP connection metadata is incompatible."))?;
        if connections.len() > 256 {
            return Err(failure(
                "AnimaOS connection list exceeds the supported limit.",
            ));
        }
        let mut matching = None;
        let mut ids = std::collections::HashSet::new();
        for connection in connections {
            validate_connection(&connection, agent_id)?;
            if !ids.insert(connection.id.clone()) {
                return Err(failure(
                    "AnimaOS returned duplicate connection identities; no server changes were made.",
                ));
            }
            if local.binding.access.is_some() && !connection.enabled && !local.matches(&connection)
            {
                continue;
            }
            if local.relevant(&connection) || local.matches(&connection) {
                if !local.matches(&connection) || matching.is_some() {
                    return Err(failure(
                        "Duplicate or conflicting MOTHER connections exist. Review and disable the old connection in AnimaOS before retrying; no entry or grant was overwritten.",
                    ));
                }
                matching = Some(connection);
            }
        }
        if let Some(operation) = local.binding.pending.as_deref() {
            let resolved = matching.as_ref().is_some_and(|connection| match operation {
                "create" => true,
                "enable" => connection.enabled,
                "connect" => has_discovery(connection),
                _ => false,
            });
            if !resolved {
                return Err(unknown());
            }
            local.pending(None)?;
        }
        if let Some(review) = local.binding.pending_review.as_ref() {
            if !matching
                .as_ref()
                .is_some_and(|connection| review.applied(connection))
            {
                return Err(unknown());
            }
            local.binding.pending_review = None;
            local.save()?;
        }
        let mut connection = match matching {
            Some(connection) => connection,
            None => {
                let input = json!({"label":LABEL,"stdio":local.spec});
                self.mutate(local, "create", "POST", &base, &input, agent_id)?
            }
        };
        let path = format!("{base}/{}", segment(&connection.id));
        if connection.physical_calls_active
            || matches!(connection.state.as_str(), "connecting" | "stopping")
        {
            return Err(failure(
                "AnimaOS reports an active connection operation. Check its state before trying Connect again.",
            ));
        }
        if !connection.enabled {
            connection = self.mutate(
                local,
                "enable",
                "PATCH",
                &path,
                &json!({"expectedVersion":connection.version,"enabled":true}),
                agent_id,
            )?;
            if !connection.enabled {
                return Err(unknown());
            }
        }
        if !has_discovery(&connection) {
            connection = self.mutate(
                local,
                "connect",
                "POST",
                &format!("{path}/connect"),
                &json!({"expectedVersion":connection.version}),
                agent_id,
            )?;
        }
        Ok(connection)
    }
}

fn reader_reviewed(connection: &Connection, digest: &str) -> bool {
    let matches =
        |tool: &&Tool| tool.remote_name == "mother_get_context" && tool.descriptor_digest == digest;
    connection.candidates.iter().filter(matches).count() == 1
        && connection.reviewed_tools.iter().filter(matches).count() == 1
}

fn binding_status(mode: AccessMode) -> String {
    match mode {
        AccessMode::Companion => "Companion configured for this agent. MOTHER supplies personality and primary durable memory. If unavailable, the same chat uses the agent's configured local personality; memory is unavailable and writes are not mirrored or replayed. Conversation host test pending.".into(),
        AccessMode::Shared => "Shared context configured for this agent's explicit record allowlist; its personality stays local. Empty allowlist shares no records. Conversation host test pending.".into(),
        AccessMode::Legacy => "Legacy context tools connected; no companion identity binding.".into(),
    }
}

fn segment(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}
fn valid_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 256
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
    {
        return Err(failure(
            "AnimaOS returned an unsupported agent or connection identity.",
        ));
    }
    Ok(())
}
fn validate_connection(connection: &Connection, agent: &str) -> Result<()> {
    valid_id(&connection.id)?;
    if connection.agent_id != agent
        || connection.version == 0
        || connection.version > 9_007_199_254_740_991
    {
        return Err(failure(
            "AnimaOS returned inconsistent connection ownership or version metadata.",
        ));
    }
    Ok(())
}
fn has_discovery(connection: &Connection) -> bool {
    connection.enabled
        && connection
            .discovery_id
            .as_ref()
            .is_some_and(|id| !id.is_empty())
        && matches!(connection.state.as_str(), "ready" | "needs_review")
}
fn discovery_status(connection: &Connection) -> Result<String> {
    if !has_discovery(connection) {
        return Err(failure(
            "AnimaOS has not completed tool discovery. Inspect its connection state; host test pending.",
        ));
    }
    let tools: Vec<&str> = connection
        .candidates
        .iter()
        .map(|tool| tool.remote_name.as_str())
        .collect();
    if !REQUIRED_TOOLS.iter().all(|name| tools.contains(name)) {
        return Err(failure(
            "MOTHER discovery is missing required context or proposal tools. Host test pending.",
        ));
    }
    if connection.tool_access.as_deref() != Some("automatic") {
        Ok("MOTHER tools discovered. Complete owner review of tool permissions in AnimaOS before use; conversation read/proposal host test pending.".into())
    } else {
        Ok("MOTHER tools ready. Conversation read/proposal host test pending; discovery did not read or save a conversation.".into())
    }
}

#[cfg(test)]
mod tests;
