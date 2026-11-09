//! Optional owner-authorized connection to a loopback AnimaOS daemon.
use fs2::FileExt;
use mother::{
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
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Binding {
    workspace_id: String,
    generation: String,
    endpoint: String,
    pending: Option<String>,
}

struct Local {
    _lock: fs::File,
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
        let name = format!("animaos-{digest}");
        let lock_path = clients.join(format!("{name}.lock"));
        let lock = match create_private_file(&lock_path) {
            Ok(file) => file,
            Err(_) => fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&lock_path)
                .map_err(|_| failure("Cannot open the AnimaOS connection lock."))?,
        };
        lock.try_lock_exclusive().map_err(|_|failure("This agent is already being connected from MOTHER. Wait for that attempt to finish."))?;
        let config_path = clients.join(format!("{name}.private.json"));
        let binding_path = clients.join(format!("{name}.binding.json"));
        let mut store = Store::open(&database)
            .map_err(|_| failure("Open an initialized MOTHER workspace before connecting."))?;
        let workspace = store
            .workspace()
            .map_err(|_| failure("Cannot verify the current MOTHER workspace."))?;
        let binding = if config_path.exists() {
            let binding: Binding =
                serde_json::from_value(read_json(&binding_path, 16384).map_err(|_| {
                    failure(
                        "Existing AnimaOS grant has no valid workspace binding; it was preserved.",
                    )
                })?)
                .map_err(|_| {
                    failure("Existing AnimaOS workspace binding is invalid; it was preserved.")
                })?;
            if binding.workspace_id != workspace["id"]
                || binding.generation != workspace["generation"]
                || binding.endpoint != endpoint
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
            store.grant_to_file(&name,&["read".into(),"context:propose".into()],&config_path,&database).map_err(|_|failure("Cannot create the separate AnimaOS grant; existing clients were preserved."))?;
            Binding {
                workspace_id: workspace["id"].as_str().unwrap_or_default().into(),
                generation: workspace["generation"].as_str().unwrap_or_default().into(),
                endpoint: endpoint.into(),
                pending: None,
            }
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
        let parent = self
            .binding_path
            .parent()
            .ok_or_else(|| failure("Cannot locate the private connection state."))?;
        let mut file = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| failure("Cannot save private AnimaOS reconciliation state."))?;
        serde_json::to_writer(&mut file, &self.binding)
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
            if local.relevant(&connection) || local.matches(&connection) {
                if !local.matches(&connection) || matching.is_some() {
                    return Err(failure(
                        "Duplicate or conflicting MOTHER connections exist. Review them in AnimaOS; no entry was overwritten.",
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
        let mut connection = match matching {
            Some(connection) => connection,
            None => {
                let input = json!({"label":LABEL,"stdio":local.spec});
                self.mutate(&mut local, "create", "POST", &base, &input, agent_id)?
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
                &mut local,
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
                &mut local,
                "connect",
                "POST",
                &format!("{path}/connect"),
                &json!({"expectedVersion":connection.version}),
                agent_id,
            )?;
        }
        discovery_status(&connection)
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
