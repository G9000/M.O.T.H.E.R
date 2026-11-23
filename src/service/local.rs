//! Private local endpoint ownership and transport-independent native tool dispatch.
use super::protocol::{self, Connection, Hello, OwnerProof, Request, Response};
use crate::{
    contracts::{Auth, hex_digest, valid_client},
    error::{Error, Result},
    garden::{
        config::domain_error,
        contracts::{GardenErrorCode, GardenRequest, GardenTool},
        store::GardenStore,
        v1_search::NativeSearch,
    },
};
use fs2::FileExt;
use interprocess::local_socket::{
    Listener, ListenerNonblockingMode, ListenerOptions,
    traits::{Listener as _, Stream as _},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};
use subtle::ConstantTimeEq;
use uuid::Uuid;

const GRAPH_TOOLS: [&str; 7] = [
    "mother_garden_get",
    "mother_garden_query",
    "mother_garden_apply",
    "mother_garden_propose",
    "mother_garden_get_proposal",
    "mother_garden_get_receipt",
    "mother_garden_changes",
];

/// An explicit credential bound to a workspace generation and local service descriptor.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceClientConfig {
    pub version: u32,
    pub backend: String,
    pub endpoint_path: PathBuf,
    pub workspace_id: String,
    pub generation: String,
    pub client_id: String,
    pub token: String,
}
impl std::fmt::Debug for ServiceClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceClientConfig")
            .field("endpoint_path", &self.endpoint_path)
            .field("workspace_id", &self.workspace_id)
            .field("generation", &self.generation)
            .field("client_id", &self.client_id)
            .finish_non_exhaustive()
    }
}
impl ServiceClientConfig {
    /// Reject implicit backends, relative endpoints and malformed owner-issued credentials.
    /// # Errors
    /// Returns invalid input without opening a database or connecting to an endpoint.
    pub fn validate(&self) -> Result<()> {
        if self.version != 3
            || self.backend != "garden_service"
            || !self.endpoint_path.is_absolute()
            || !valid_client(&self.client_id)
            || !hex_digest(&self.token)
            || Uuid::parse_str(&self.workspace_id).is_err()
            || Uuid::parse_str(&self.generation).is_err()
        {
            return Err(invalid());
        }
        Ok(())
    }
    fn auth(&self) -> Auth {
        Auth {
            client_id: self.client_id.clone(),
            token: self.token.clone(),
        }
    }
}

/// Read strict bounded service credentials without touching database state.
/// # Errors
/// Rejects unsupported versions, unknown fields and malformed explicit credentials.
pub fn load_service_config(path: &Path) -> Result<ServiceClientConfig> {
    let value = crate::config::read_json(path, 16_384)?;
    let config: ServiceClientConfig = serde_json::from_value(value)?;
    config.validate()?;
    Ok(config)
}

/// Public endpoint identity; this descriptor never contains tokens or database paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceInfo {
    pub version: u32,
    pub service_id: String,
    pub address: String,
    pub workspace_id: String,
    pub generation: String,
    pub v1_available: bool,
}

fn invalid() -> Error {
    Error::domain("INVALID_INPUT", "Invalid local service input.")
}
fn unavailable() -> Error {
    Error::domain(
        "STORE_UNAVAILABLE",
        "Local service is unavailable; reconcile an uncertain operation using its original UUID.",
    )
}

fn private_parent(path: &Path) -> Result<&Path> {
    if !path.is_absolute() {
        return Err(invalid());
    }
    let parent = path.parent().ok_or_else(invalid)?;
    crate::private_paths::owner_directory(parent)?;
    let metadata = fs::symlink_metadata(parent)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(invalid());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(Error::domain(
                "FORBIDDEN",
                "Local service directory must be owner-private.",
            ));
        }
    }
    Ok(parent)
}

fn descriptor(path: &Path) -> Result<ServiceInfo> {
    let parent = private_parent(path)?;
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(invalid());
    }
    let info: ServiceInfo = serde_json::from_value(crate::config::read_json(path, 16_384)?)?;
    if info.version != 1
        || Uuid::parse_str(&info.service_id).is_err()
        || Uuid::parse_str(&info.workspace_id).is_err()
        || Uuid::parse_str(&info.generation).is_err()
    {
        return Err(invalid());
    }
    let valid_address = info.address == address(parent, &info.service_id)?;
    #[cfg(unix)]
    // Version-1 upgrades may retain a descriptor using the former exact derived pathname.
    let valid_address = valid_address
        || unix_address(parent, &format!(".mother-{}.sock", info.service_id))
            .is_ok_and(|legacy| info.address == legacy);
    if !valid_address {
        return Err(invalid());
    }
    Ok(info)
}

fn address(parent: &Path, service_id: &str) -> Result<String> {
    #[cfg(windows)]
    {
        let _ = parent;
        Ok(format!("mother-garden-{service_id}"))
    }
    #[cfg(unix)]
    {
        use sha2::{Digest, Sha256};
        // Keep 96 digest bits in the pathname; the full UUID remains in the owner proof.
        let digest = format!("{:x}", Sha256::digest(service_id.as_bytes()));
        unix_address(parent, &format!(".m-{}", &digest[..24]))
    }
}

#[cfg(unix)]
fn unix_address(parent: &Path, filename: &str) -> Result<String> {
    let path = parent.join(filename);
    let address = path.to_str().ok_or_else(invalid)?;
    // Validate the target's sun_path capacity without binding or creating owner state.
    std::os::unix::net::SocketAddr::from_pathname(address).map_err(|_| {
        Error::domain(
            "INVALID_INPUT",
            "Local service socket path exceeds platform capacity; choose a shorter owner-private directory.",
        )
    })?;
    Ok(address.to_owned())
}

fn socket_name(address: &str) -> io::Result<interprocess::local_socket::Name<'_>> {
    #[cfg(windows)]
    {
        use interprocess::local_socket::{GenericNamespaced, ToNsName};
        address.to_ns_name::<GenericNamespaced>()
    }
    #[cfg(unix)]
    {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        address.to_fs_name::<GenericFilePath>()
    }
}

fn connect(info: &ServiceInfo) -> Result<Connection> {
    #[cfg(windows)]
    let stream = {
        use interprocess::{
            ConnectWaitMode,
            local_socket::Stream,
            os::windows::named_pipe::{DuplexPipeStream, pipe_mode::Bytes},
        };
        // The generic Windows constructor in 2.4.8 ignores ConnectOptions::wait_mode.
        let pipe = DuplexPipeStream::<Bytes>::connect_by_path_with_wait_mode(
            format!(r"\\.\pipe\{}", info.address),
            ConnectWaitMode::Timeout(protocol::IO_TIMEOUT),
        )
        .map_err(|error| {
            Error::domain(
                "STORE_UNAVAILABLE",
                format!("Local pipe connect failed: {error}"),
            )
        })?;
        Stream::NamedPipe(pipe.into())
    };
    #[cfg(unix)]
    let stream = {
        use interprocess::{ConnectWaitMode, local_socket::ConnectOptions};
        ConnectOptions::new()
            .name(socket_name(&info.address)?)
            .wait_mode(ConnectWaitMode::Timeout(protocol::IO_TIMEOUT))
            .nonblocking_stream(true)
            .connect_sync()
            .map_err(|_| unavailable())?
    };
    stream.set_nonblocking(true).map_err(|error| {
        Error::domain(
            "STORE_UNAVAILABLE",
            format!("Local nonblocking setup failed: {error}"),
        )
    })?;
    Ok(Connection(stream))
}

fn bind_identity(info: &ServiceInfo, config: &ServiceClientConfig) -> Result<()> {
    if info.workspace_id != config.workspace_id {
        return Err(Error::domain(
            "WORKSPACE_MISMATCH",
            "Configured workspace does not match the service.",
        ));
    }
    if info.generation != config.generation {
        return Err(Error::domain(
            "GENERATION_MISMATCH",
            "Configured generation does not match the service.",
        ));
    }
    Ok(())
}

/// A bounded client that never opens the database or starts another owner process.
#[derive(Debug, Clone)]
pub struct LocalClient {
    config: ServiceClientConfig,
    info: ServiceInfo,
}
impl LocalClient {
    /// Read the private descriptor and authenticate against current service authority.
    /// # Errors
    /// Rejects a stale workspace generation, invalid credential or unavailable owner.
    pub fn new(config: ServiceClientConfig) -> Result<Self> {
        config.validate()?;
        let info = descriptor(&config.endpoint_path)?;
        bind_identity(&info, &config)?;
        let client = Self { config, info };
        client.call("mother_get_capabilities", &json!({}))?;
        Ok(client)
    }
    /// Return the validated initial public endpoint identity.
    pub fn info(&self) -> &ServiceInfo {
        &self.info
    }
    /// Return the configured identity with redacted debug formatting.
    pub fn config(&self) -> &ServiceClientConfig {
        &self.config
    }
    /// Make exactly one domain request without retrying an uncertain mutation.
    /// # Errors
    /// Transport loss leaves writes unconfirmed; preserve the original operation UUID.
    pub fn call(&self, name: &str, arguments: &Value) -> Result<Value> {
        let info = descriptor(&self.config.endpoint_path)?;
        bind_identity(&info, &self.config)?;
        let request = Request {
            version: 1,
            service_id: info.service_id.clone(),
            workspace_id: self.config.workspace_id.clone(),
            generation: self.config.generation.clone(),
            auth: self.config.auth(),
            method: name.to_owned(),
            arguments: arguments.clone(),
        };
        let mut stream = connect(&info)?;
        let hello = Hello {
            version: 1,
            client_id: self.config.client_id.clone(),
            challenge: Uuid::new_v4().to_string(),
            service_id: info.service_id.clone(),
            workspace_id: info.workspace_id.clone(),
            generation: info.generation.clone(),
            address: info.address.clone(),
        };
        protocol::write_frame(&mut stream.0, &hello)?;
        let owner_proof: OwnerProof =
            protocol::read_frame_with_timeout(&mut stream.0, std::time::Duration::from_secs(2))
                .map_err(|error| {
                    Error::domain(
                        "UNAUTHORIZED",
                        format!("Local service owner proof read failed: {error}"),
                    )
                })?;
        let expected = crate::garden::store::service_auth::proof_mac(
            &crate::contracts::hash(&self.config.token),
            &hello.context()?,
        )
        .map_err(domain_error)?;
        if !matches!(owner_proof, OwnerProof::Proof { version: 1, mac } if bool::from(expected.ct_eq(&mac)))
        {
            return Err(Error::domain(
                "UNAUTHORIZED",
                "Local service owner proof failed.",
            ));
        }
        // The credential is transferred only after this fresh challenge authenticates the owner.
        protocol::write_frame(&mut stream.0, &request)?;
        let response: Response =
            protocol::read_frame_with_timeout(&mut stream.0, std::time::Duration::from_secs(30))?;
        // This acknowledgement lets Windows discard the pipe without a lingering flush worker.
        let _ = protocol::acknowledge(&mut stream.0);
        response.into_result()
    }
}

/// One local process retaining the native database and an exclusive owner lease.
#[derive(Debug)]
pub struct LocalService {
    store: Arc<GardenStore>,
    listener: Listener,
    info: ServiceInfo,
    endpoint_path: PathBuf,
    database_path: PathBuf,
    _owner_lease: File,
    search: NativeSearch,
    recall_profiles: crate::recall_profiles::RecallProfiles,
}
impl LocalService {
    /// Exclusively bind an existing native workspace to an owner-private descriptor.
    /// # Errors
    /// Rejects another database owner, unsafe paths and unrelated existing descriptors.
    pub fn bind(database_path: &Path, endpoint_path: &Path) -> Result<Self> {
        Self::bind_with_recall_profiles(
            database_path,
            endpoint_path,
            crate::recall_profiles::RecallProfiles::default(),
        )
    }

    /// Bind the local owner using an explicitly loaded recall profile policy.
    pub fn bind_with_recall_profiles(
        database_path: &Path,
        endpoint_path: &Path,
        recall_profiles: crate::recall_profiles::RecallProfiles,
    ) -> Result<Self> {
        crate::garden::migrate::check_destination(database_path)?;
        let parent = private_parent(endpoint_path)?;
        let service_id = Uuid::new_v4().to_string();
        let address = address(parent, &service_id)?;
        let canonical_db = fs::canonicalize(database_path)?;
        crate::private_paths::single_link(&canonical_db)?;
        let lease_path = canonical_db.with_added_extension("service-owner.lock");
        if let Ok(metadata) = fs::symlink_metadata(&lease_path)
            && (!metadata.is_file() || metadata.file_type().is_symlink())
        {
            return Err(invalid());
        }
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let owner_lease = options.open(lease_path)?;
        owner_lease
            .try_lock_exclusive()
            .map_err(|_| unavailable())?;
        let store = GardenStore::open(&canonical_db).map_err(domain_error)?;
        crate::garden::migrate::check_destination(&canonical_db)?;
        let workspace = store.owner_inbox_list().map_err(domain_error)?;
        let workspace = &workspace["workspace"];
        let info = ServiceInfo {
            version: 1,
            address,
            service_id,
            workspace_id: workspace["id"].as_str().ok_or_else(unavailable)?.to_owned(),
            generation: workspace["generation"]
                .as_str()
                .ok_or_else(unavailable)?
                .to_owned(),
            v1_available: store.v1_initialized()?,
        };
        if endpoint_path.try_exists()? {
            let previous = descriptor(endpoint_path)?;
            if previous.workspace_id != info.workspace_id || previous.generation != info.generation
            {
                return Err(Error::domain(
                    "CONFLICT",
                    "Existing endpoint belongs to a different workspace generation.",
                ));
            }
            if connect(&previous).is_ok() {
                return Err(Error::domain(
                    "CONFLICT",
                    "Existing endpoint still has a local owner.",
                ));
            }
            // The database-specific owner lease has been acquired before replacing this descriptor.
            fs::remove_file(endpoint_path)?;
        }
        let listener = ListenerOptions::new()
            .name(socket_name(&info.address)?)
            .nonblocking(ListenerNonblockingMode::Both)
            .create_sync()?;
        let mut file = tempfile::Builder::new()
            .prefix(".mother-endpoint-")
            .tempfile_in(parent)?;
        file.write_all(&serde_json::to_vec(&info)?)?;
        file.as_file().sync_all()?;
        file.persist_noclobber(endpoint_path)
            .map_err(|_| unavailable())?;
        Ok(Self {
            store: Arc::new(store),
            listener,
            info,
            endpoint_path: endpoint_path.to_owned(),
            database_path: canonical_db,
            _owner_lease: owner_lease,
            search: NativeSearch::from_environment(),
            recall_profiles,
        })
    }
    /// Return the public descriptor, including native v1 availability.
    pub fn info(&self) -> &ServiceInfo {
        &self.info
    }
    /// Access trusted owner operations inside this process; never expose this through IPC.
    pub fn store(&self) -> &GardenStore {
        &self.store
    }
    /// Lend the browser explorer a read source within this owner process.
    pub fn explorer_store(&self) -> std::sync::Weak<GardenStore> {
        Arc::downgrade(&self.store)
    }
    /// Serve until this process is stopped by its owner.
    /// # Errors
    /// Reports listener failure; malformed and disconnected peers are isolated per request.
    pub fn run(self) -> Result<()> {
        self.serve_until(Arc::new(AtomicBool::new(false)))
    }
    /// Serve synthetic or embedded owners until their local stop flag becomes true.
    /// # Errors
    /// Returns listener failures; a slow peer delays the next accept by a fixed I/O deadline.
    pub fn serve_until(self, stopped: Arc<AtomicBool>) -> Result<()> {
        self.serve(&stopped)
    }
    /// Serve while retaining trusted owner access through this same database handle.
    /// # Errors
    /// Returns listener failures; the owner must keep this service alive until serving ends.
    pub fn serve(&self, stopped: &AtomicBool) -> Result<()> {
        while !stopped.load(Ordering::Acquire) {
            match self.listener.accept() {
                Ok(stream) => self.handle(Connection(stream)),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(protocol::POLL_INTERVAL)
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(_) => return Err(unavailable()),
            }
        }
        Ok(())
    }
    fn handle(&self, mut stream: Connection) {
        let hello = match protocol::read_frame::<Hello>(&mut stream.0) {
            Ok(hello) => hello,
            Err(_) => return,
        };
        let proof = match self.owner_proof(&hello) {
            Ok(mac) => OwnerProof::Proof { version: 1, mac },
            Err(_) => OwnerProof::Unauthorized,
        };
        let authenticated_owner = matches!(proof, OwnerProof::Proof { .. });
        if protocol::write_frame(&mut stream.0, &proof).is_err() {
            return;
        }
        if !authenticated_owner {
            // Keep an explicit denial available long enough for the client to consume it.
            let _ = protocol::read_acknowledgement(&mut stream.0);
            return;
        }
        let request = match protocol::read_frame::<Request>(&mut stream.0) {
            Ok(request) => request,
            Err(_) => return,
        };
        if request.auth.client_id != hello.client_id
            || request.service_id != hello.service_id
            || request.workspace_id != hello.workspace_id
            || request.generation != hello.generation
        {
            return;
        }
        let response = Response::from_result(self.dispatch(request));
        if let Err(error) = protocol::write_frame(&mut stream.0, &response) {
            if error.code() == "LIMIT_EXCEEDED"
                && protocol::write_frame(&mut stream.0, &Response::from_result(Err(error))).is_ok()
            {
                let _ = protocol::read_acknowledgement(&mut stream.0);
            }
            return;
        }
        let _ = protocol::read_acknowledgement(&mut stream.0);
    }
    fn owner_proof(&self, hello: &Hello) -> Result<[u8; 32]> {
        crate::garden::migrate::check_destination(&self.database_path)?;
        if hello.version != 1
            || !valid_client(&hello.client_id)
            || Uuid::parse_str(&hello.challenge).is_err()
            || hello.service_id != self.info.service_id
            || hello.workspace_id != self.info.workspace_id
            || hello.generation != self.info.generation
            || hello.address != self.info.address
        {
            return Err(invalid());
        }
        self.store
            .local_service_proof(&hello.client_id, &hello.context()?)
            .map_err(domain_error)
    }
    fn dispatch(&self, request: Request) -> Result<Value> {
        crate::garden::migrate::check_destination(&self.database_path)?;
        if request.version != 1 {
            return Err(invalid());
        }
        if request.service_id != self.info.service_id {
            return Err(Error::domain(
                "RESTART_REQUIRED",
                "Local service owner changed.",
            ));
        }
        if request.workspace_id != self.info.workspace_id {
            return Err(Error::domain(
                "WORKSPACE_MISMATCH",
                "Configured workspace does not match the service.",
            ));
        }
        if request.generation != self.info.generation {
            return Err(Error::domain(
                "GENERATION_MISMATCH",
                "Configured generation does not match the service.",
            ));
        }
        let auth = &request.auth;
        let (identity, graph_available) = match self.store.identity(auth) {
            Ok(identity) => (identity, true),
            Err(error) if error.code == GardenErrorCode::Forbidden && self.info.v1_available => {
                (self.store.v1_identity(auth)?, false)
            }
            Err(error) => return Err(domain_error(error)),
        };
        let name = request.method.as_str();
        let args = request.arguments;
        let bridge_root = super::attachment::root(
            &self.endpoint_path,
            &self.info.workspace_id,
            &self.info.generation,
            &auth.client_id,
            &auth.token,
        )?;
        if matches!(
            name,
            "local_media_target"
                | "local_media_stage"
                | "local_media_finish_review"
                | super::attachment::STATUS
        ) {
            return super::attachment::dispatch(&self.store, auth, &bridge_root, name, &args);
        }
        if name == "mother_get_capabilities" {
            if args.as_object().is_none_or(|object| !object.is_empty()) {
                return Err(invalid());
            }
            return Ok(json!({"version":1,"workspace":identity,
                "v1":{"available":self.info.v1_available,"contractVersion":1,"gardeningEvidenceReview":1},
                "garden":{"contractVersion":2,"available":graph_available,"status":"prototype",
                    "optionalVectorProfile":crate::search::embedding_profile(),
                    "vectorPreparation":"explicit_local_sdk_or_owner_cli",
                    "availableTools":if graph_available {GRAPH_TOOLS.as_slice()} else {&[]},
                    "capabilities":if graph_available {vec!["garden.entities.v2","garden.assertions.v2","garden.proposals.v2","garden.query.v2","garden.changes.v2"]} else {vec![]},
                    "definedCapabilities":["garden.entities.v2","garden.assertions.v2","garden.query.v2","garden.changes.v2","garden.proposals.v2"],
                    "definedTools":GRAPH_TOOLS,
                    "reason":if !graph_available {"native_context_only_current_grant"} else if self.info.v1_available {"native_v1_compatibility_with_independent_graph_authority"} else {"graph_only_prototype_without_v1_compatibility_or_migration"}}}))
            .map(|mut value| {
                let enabled = self.info.v1_available && super::attachment::enabled(&bridge_root);
                value["mediaAttachmentBridge"] = json!({"available":enabled, "maxFileBytes":super::attachment::MAX_BYTES,
                    "availableTools":if enabled {vec![super::attachment::PROPOSE, super::attachment::STATUS]} else {vec![]}});
                crate::garden::media_tools::add_capabilities(
                    &mut value,
                    &self.store.media_tools(auth).unwrap_or_default(),
                );
                value
            });
        }
        if name == crate::garden::media_tools::LIST_TOOL {
            return self.store.media_list(auth, &args).map_err(domain_error);
        }
        if name == crate::garden::media_tools::GET_TOOL {
            return self.store.media_get(auth, &args).map_err(domain_error);
        }
        if !GRAPH_TOOLS.contains(&name) {
            if !self.info.v1_available {
                return Err(invalid());
            }
            let catalog: Value =
                serde_json::from_str(include_str!("../../contracts/mcp-tools.json"))?;
            if !catalog
                .as_array()
                .is_some_and(|tools| tools.iter().any(|tool| tool["name"] == name))
            {
                return Err(invalid());
            }
            if name == "mother_search_context" {
                return self.search.search(&self.store, auth, args);
            }
            if name == "mother_get_context" {
                return self.store.v1_context(auth, args, &self.recall_profiles);
            }
            return self.store.v1_call(auth, name, args);
        }
        let tool = GardenTool::parse(name).map_err(domain_error)?;
        let parsed = GardenRequest::parse(tool, &args).map_err(domain_error)?;
        match (tool, parsed) {
            (GardenTool::Get, GardenRequest::Get(request)) => {
                serde_json::to_value(self.store.get(auth, &request).map_err(domain_error)?)
            }
            (GardenTool::Query, GardenRequest::Query(request)) => {
                serde_json::to_value(self.store.query(auth, &request).map_err(domain_error)?)
            }
            (GardenTool::Changes, GardenRequest::Changes(request)) => {
                serde_json::to_value(self.store.changes(auth, &request).map_err(domain_error)?)
            }
            (GardenTool::Apply, GardenRequest::Batch(request)) => {
                serde_json::to_value(self.store.apply(auth, &request).map_err(domain_error)?)
            }
            (GardenTool::Propose, GardenRequest::Batch(request)) => {
                serde_json::to_value(self.store.propose(auth, &request).map_err(domain_error)?)
            }
            (GardenTool::GetProposal, GardenRequest::Outcome(request)) => serde_json::to_value(
                self.store
                    .get_proposal(auth, &request)
                    .map_err(domain_error)?,
            ),
            (GardenTool::GetReceipt, GardenRequest::Outcome(request)) => {
                serde_json::to_value(self.store.receipt(auth, &request).map_err(domain_error)?)
            }
            _ => return Err(invalid()),
        }
        .map_err(Error::from)
    }
}
impl Drop for LocalService {
    fn drop(&mut self) {
        // A caller may replace the descriptor; only remove the exact identity we created.
        if descriptor(&self.endpoint_path).is_ok_and(|info| info == self.info) {
            let _ = fs::remove_file(&self.endpoint_path);
        }
    }
}
