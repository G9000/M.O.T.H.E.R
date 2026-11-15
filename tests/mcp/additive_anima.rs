//! Opt-in acceptance across two real executables, with disposable owner data only.
//! Owner review uses the canonical backend boundary; no visual acceptance is implied.
use super::{Client, cli};
use mother::garden::store::GardenStore;
use serde_json::{Value, json};
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;

const POLL_LIMIT: Duration = Duration::from_secs(50);

struct OwnerProcess(Child);
impl Drop for OwnerProcess {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

struct NativeOwner {
    root: PathBuf,
    database: PathBuf,
    endpoint: PathBuf,
    process: Option<OwnerProcess>,
}
impl NativeOwner {
    fn start(&mut self) {
        assert!(self.process.is_none());
        let previous = std::fs::read(&self.endpoint).ok();
        let mut command = Command::new(env!("CARGO_BIN_EXE_mother"));
        command
            .args(["garden", "service", "--db"])
            .arg(&self.database)
            .arg("--endpoint")
            .arg(&self.endpoint)
            .env("MOTHER_HOME", &self.root)
            .env("MOTHER_MODEL_DIR", self.root.join("absent-model"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }
        let mut process = OwnerProcess(command.spawn().unwrap());
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            assert!(
                process.0.try_wait().unwrap().is_none(),
                "native owner exited before readiness"
            );
            if let Ok(bytes) = std::fs::read(&self.endpoint)
                && Some(&bytes) != previous.as_ref()
                && serde_json::from_slice::<Value>(&bytes).is_ok()
            {
                self.process = Some(process);
                return;
            }
            assert!(Instant::now() < deadline, "native owner readiness deadline");
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn stop(&mut self) {
        drop(self.process.take());
    }

    fn inspect(
        &mut self,
        read: impl FnOnce(&GardenStore) -> mother::error::Result<Value>,
    ) -> mother::error::Result<Value> {
        self.stop();
        let store = GardenStore::open_active(&self.database).unwrap();
        let result = read(&store);
        drop(store);
        self.start();
        result
    }

    fn bridge_config(&self, direct: &Path, out: &Path) {
        cli(
            &self.root,
            &[
                "garden",
                "service-config",
                "--config",
                direct.to_str().unwrap(),
                "--endpoint",
                self.endpoint.to_str().unwrap(),
                "--out",
                out.to_str().unwrap(),
            ],
        );
    }
}

enum AcceptanceOwner {
    Native(NativeOwner),
}
impl AcceptanceOwner {
    fn inbox_list(&mut self) -> mother::error::Result<Value> {
        match self {
            Self::Native(owner) => owner.inspect(GardenStore::v1_inbox_list),
        }
    }
    fn inbox_get(&mut self, id: &str) -> mother::error::Result<Value> {
        match self {
            Self::Native(owner) => owner.inspect(|store| store.v1_inbox_get(id)),
        }
    }
    fn review_proposal(&mut self, request: Value) -> mother::error::Result<Value> {
        match self {
            Self::Native(owner) => {
                owner.stop();
                let path = owner.root.join(format!("review-{}.json", Uuid::new_v4()));
                std::fs::write(&path, serde_json::to_vec(&request).unwrap()).unwrap();
                let reviewed = cli(
                    &owner.root,
                    &[
                        "garden",
                        "review",
                        "--db",
                        owner.database.to_str().unwrap(),
                        "--request",
                        path.to_str().unwrap(),
                    ],
                );
                owner.start();
                Ok(reviewed)
            }
        }
    }
}

struct AcceptanceFixture {
    native: Option<NativeOwner>,
    dir: tempfile::TempDir,
    workspace: Value,
    b_config: PathBuf,
}
impl AcceptanceFixture {
    fn new() -> Self {
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let database = dir.path().join("garden.redb");
        let soul = dir.path().join("SOUL.md");
        std::fs::write(&soul, "Synthetic native-service acceptance persona").unwrap();
        let initialized = cli(
            dir.path(),
            &["garden", "init", "--db", database.to_str().unwrap()],
        );
        cli(
            dir.path(),
            &[
                "garden",
                "compat-init",
                "--db",
                database.to_str().unwrap(),
                "--soul",
                soul.to_str().unwrap(),
            ],
        );
        let direct = dir.path().join("harness-b.direct.private.json");
        cli(
            dir.path(),
            &[
                "garden",
                "grant-v1",
                "harness-b",
                "--db",
                database.to_str().unwrap(),
                "--out",
                direct.to_str().unwrap(),
                "--scopes",
                "read,memory:write",
            ],
        );
        let mut owner = NativeOwner {
            root: dir.path().to_path_buf(),
            database,
            endpoint: dir.path().join("service.json"),
            process: None,
        };
        owner.start();
        let b_config = dir.path().join("harness-b.service.private.json");
        owner.bridge_config(&direct, &b_config);
        assert!(!dir.path().join("context.sqlite").exists());
        Self {
            dir,
            workspace: initialized["workspace"].clone(),
            b_config,
            native: Some(owner),
        }
    }

    fn grant_anima(&mut self, agent: &str) -> PathBuf {
        let owner = self.native.as_mut().expect("native synthetic owner");
        owner.stop();
        let direct = self.dir.path().join("anima.direct.private.json");
        cli(
            self.dir.path(),
            &[
                "garden",
                "grant-v1",
                "anima-acceptance",
                "--db",
                owner.database.to_str().unwrap(),
                "--out",
                direct.to_str().unwrap(),
                "--access",
                "companion",
                "--subject",
                agent,
                "--scopes",
                "read,context:propose",
            ],
        );
        owner.start();
        let config = self.dir.path().join("anima.service.private.json");
        owner.bridge_config(&direct, &config);
        config
    }
    fn owner(&mut self) -> AcceptanceOwner {
        AcceptanceOwner::Native(self.native.take().expect("native synthetic owner"))
    }
    fn update(&self, record: Value, id: &Value, revision: u64) -> Value {
        json!({"workspaceId":self.workspace["id"],"generation":self.workspace["generation"],
            "operationId":Uuid::new_v4().to_string(),"expectedRevision":revision,"id":id,"record":record})
    }
}

fn harness(config: &Path) -> Client {
    use std::io::{BufRead, BufReader};
    use std::sync::mpsc;
    let mut command = Command::new(env!("CARGO_BIN_EXE_mother"));
    command
        .args(["mcp", "--config"])
        .arg(config)
        .env("MOTHER_HOME", config.parent().unwrap())
        .env(
            "MOTHER_MODEL_DIR",
            config.parent().unwrap().join("absent-model"),
        )
        .current_dir(config.parent().unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command.spawn().unwrap();
    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let (tx, responses) = mpsc::channel();
    let reader = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            let Ok(value) = serde_json::from_str(&line) else {
                break;
            };
            if tx.send(value).is_err() {
                break;
            }
        }
    });
    let mut client = Client {
        child,
        stdin,
        responses,
        reader: Some(reader),
        next_id: 0,
    };
    let initialized = client.rpc(
        "initialize",
        json!({
            "protocolVersion":"2025-06-18","capabilities":{},
            "clientInfo":{"name":"synthetic-native-anima-acceptance","version":"1"}
        }),
    );
    assert_eq!(initialized["serverInfo"]["name"], "mother-garden");
    client.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
    client
}

struct Daemon {
    child: Child,
    http: ureq::Agent,
    base: String,
    owner_token: String,
    stderr_path: PathBuf,
}

impl Daemon {
    fn start(executable: &Path, root: &Path, namespace: &str, owner_token: &str) -> Self {
        assert!(executable.is_absolute() && executable.is_file());
        let workspace = root.parent().unwrap().join("anima-workspace");
        std::fs::create_dir_all(&workspace).unwrap();
        let socket = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = socket.local_addr().unwrap().port();
        drop(socket);
        let mut command = Command::new(executable);
        let stderr_path = root.join(format!("synthetic-daemon-{}.stderr.log", Uuid::new_v4()));
        let stderr = std::fs::File::create(&stderr_path).unwrap();
        command.env_clear();
        for key in ["SystemRoot", "WINDIR", "PATH", "TEMP", "TMP"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        for key in [
            "HOME",
            "USERPROFILE",
            "APPDATA",
            "LOCALAPPDATA",
            "ANIMA_HOME",
        ] {
            command.env(key, root);
        }
        command
            .env("ANIMA_LOCAL_ADMIN_TOKEN", owner_token)
            .env("ANIMAOS_RS_HOST", "127.0.0.1")
            .env("ANIMAOS_RS_PORT", port.to_string())
            .env("ANIMAOS_RS_CONTROL_PLANE_FILE", root.join("control.json"))
            .env("ANIMAOS_RS_MEMORY_FILE", root.join("native-memory.json"))
            .env("ANIMAOS_RS_MEMORY_EMBEDDINGS", "disabled")
            .env("ANIMAOS_RS_KEYCHAIN_NAMESPACE", namespace)
            .env("RUST_LOG", "anima_daemon::mcp=debug")
            .env("ANIMAOS_WORKSPACE_ROOT", &workspace)
            .env("MOTHER_HOME", root.join("unused-mother-home"))
            .env("MOTHER_MODEL_DIR", root.join("absent-model"))
            .current_dir(root)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stderr.try_clone().unwrap()))
            .stderr(Stdio::from(stderr));
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }
        let child = command
            .spawn()
            .expect("start selected synthetic Anima daemon");
        let mut daemon = Self {
            child,
            http: ureq::Agent::config_builder()
                .proxy(None)
                .max_redirects(0)
                .http_status_as_error(false)
                .timeout_global(Some(Duration::from_secs(10)))
                .timeout_connect(Some(Duration::from_secs(1)))
                .build()
                .into(),
            base: format!("http://127.0.0.1:{port}"),
            owner_token: owner_token.to_owned(),
            stderr_path,
        };
        let deadline = Instant::now() + POLL_LIMIT;
        loop {
            assert!(
                daemon.child.try_wait().unwrap().is_none(),
                "synthetic daemon exited during startup: {}",
                daemon.diagnostics()
            );
            if daemon
                .http
                .get(format!("{}/health", daemon.base))
                .call()
                .is_ok_and(|r| r.status().is_success())
            {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "synthetic daemon startup deadline: {}",
                daemon.diagnostics()
            );
            thread::sleep(Duration::from_millis(100));
        }
        daemon
    }

    fn api(&self, method: &str, path: &str, body: Value) -> Value {
        let url = format!("{}{path}", self.base);
        let bearer = format!("Bearer {}", self.owner_token);
        let bytes = body.to_string();
        let response = match method {
            "GET" => self.http.get(&url).header("authorization", &bearer).call(),
            "POST" => self
                .http
                .post(&url)
                .header("authorization", &bearer)
                .header("content-type", "application/json")
                .send(bytes.as_bytes()),
            "PUT" => self
                .http
                .put(&url)
                .header("authorization", &bearer)
                .header("content-type", "application/json")
                .send(bytes.as_bytes()),
            "PATCH" => self
                .http
                .patch(&url)
                .header("authorization", &bearer)
                .header("content-type", "application/json")
                .send(bytes.as_bytes()),
            _ => panic!("unsupported fixture HTTP method"),
        };
        let mut response = response.unwrap_or_else(|_| {
            panic!(
                "synthetic {method} {path} transport failed: {}",
                self.diagnostics()
            )
        });
        let status = response.status();
        let text = response
            .body_mut()
            .with_config()
            .limit(256 * 1024)
            .read_to_string()
            .unwrap();
        assert!(
            status.is_success(),
            "synthetic {method} {path}: {status}; body={}; daemon output={}",
            text.replace(&self.owner_token, "[synthetic owner token]"),
            self.diagnostics()
        );
        if text.is_empty() {
            Value::Null
        } else {
            serde_json::from_str(&text).unwrap()
        }
    }

    fn diagnostics(&self) -> String {
        let bytes = std::fs::read(&self.stderr_path).unwrap_or_default();
        String::from_utf8_lossy(&bytes[bytes.len().saturating_sub(16 * 1024)..])
            .replace(&self.owner_token, "[synthetic owner token]")
    }

    fn wait_state(&self, path: &str, expected: &str) -> Value {
        let deadline = Instant::now() + POLL_LIMIT;
        loop {
            let value = self.api("GET", path, Value::Null);
            if value["state"] == expected {
                return value;
            }
            assert!(
                Instant::now() < deadline,
                "publication failed to reach {expected}; state={}, reason={}",
                value["state"],
                value["reason"]
            );
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn stop(&mut self) {
        self.api("POST", "/api/system/shutdown", Value::Null);
        let deadline = Instant::now() + POLL_LIMIT;
        while self.child.try_wait().unwrap().is_none() {
            assert!(
                Instant::now() < deadline,
                "synthetic daemon shutdown deadline"
            );
            thread::sleep(Duration::from_millis(100));
        }
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn review_connection(daemon: &Daemon, agent: &str, config: &Path) -> String {
    let base = format!("/api/agents/{agent}/mcp-connections");
    let created = daemon.api(
        "POST",
        &base,
        json!({
            "label":"Synthetic garden acceptance",
            "stdio":{"executable":env!("CARGO_BIN_EXE_mother"),"args":["mcp","--config",config]}
        }),
    );
    let id = created["connection"]["id"].as_str().unwrap().to_owned();
    let path = format!("{base}/{id}");
    let enabled = daemon.api(
        "PATCH",
        &path,
        json!({"expectedVersion":created["connection"]["version"],"enabled":true}),
    );
    let mut connected = daemon.api(
        "POST",
        &format!("{path}/connect"),
        json!({"expectedVersion":enabled["connection"]["version"]}),
    );
    for name in [
        "mother_get_context",
        "mother_propose_memory",
        "mother_get_receipt",
        "mother_get_proposal",
    ] {
        let connection = &connected["connection"];
        let candidate = connection["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["remoteName"] == name)
            .unwrap();
        connected = daemon.api("POST", &format!("{path}/review"), json!({
            "expectedVersion":connection["version"],"discoveryId":connection["discoveryId"],"descriptorDigest":candidate["descriptorDigest"]
        }));
    }
    assert_eq!(connected["connection"]["state"], "ready");
    id
}

fn native_fact(daemon: &Daemon, agent: &str, content: &str) -> Value {
    daemon.api("POST", "/api/memories", json!({"agentId":agent,"agentName":"Synthetic additive owner","type":"fact","content":content,"importance":0.5,"scope":"shared"}))
}

fn assert_local_fact(daemon: &Daemon, record: &Value) {
    let current = daemon.api(
        "GET",
        &format!(
            "/api/memories/recent?agentId={}",
            record["agentId"].as_str().unwrap()
        ),
        Value::Null,
    );
    let found = current["memories"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == record["id"])
        .expect("native fact retained across garden changes and daemon restart");
    assert_eq!(
        found["content"], record["content"],
        "garden edits must not overwrite native memory"
    );
}

#[test]
#[ignore = "Requires ANIMA_MOTHER_TEST_DAEMON pointing to the explicitly built additive Anima daemon"]
fn additive_anima_native_service_selected_sharing_and_reconciliation() {
    additive_anima_acceptance();
}

fn additive_anima_acceptance() {
    let executable = PathBuf::from(
        std::env::var_os("ANIMA_MOTHER_TEST_DAEMON")
            .expect("select the additive Anima daemon executable explicitly"),
    );
    assert!(executable.is_absolute() && executable.is_file());
    let mut fixture = AcceptanceFixture::new();
    let profile = fixture.dir.path().join("anima-profile");
    std::fs::create_dir(&profile).unwrap();
    let namespace = Uuid::new_v4().simple().to_string();
    let owner_token = Uuid::new_v4().to_string();
    let mut daemon = Daemon::start(&executable, &profile, &namespace, &owner_token);
    let created = daemon.api(
        "POST",
        "/api/agents",
        json!({"name":"Synthetic additive owner","model":"synthetic-no-model-calls"}),
    );
    let agent = created["agent"]["state"]["id"].as_str().unwrap().to_owned();
    daemon.api(
        "PUT",
        &format!("/api/agents/{agent}/approval-policy"),
        json!({"write":"allow","exec":"allow","network":"deny","delegate":"deny"}),
    );
    let config = fixture.grant_anima(&agent);
    let connection = review_connection(&daemon, &agent, &config);
    let binding_path = format!("/api/agents/{agent}/workspace-context");
    let current = daemon.api("GET", &binding_path, Value::Null);
    let configured = daemon.api("PUT", &binding_path, json!({
        "expectedRevision":current["revision"],"binding":null,
        "providerBinding":{"version":1,"provider":"mother","connectionId":connection,
            "workspaceId":fixture.workspace["id"],"generation":fixture.workspace["generation"],
            "subject":agent,"publication":"selected","personaSource":"local","accessMode":"companion"}
    }));
    assert_eq!(configured["binding"], Value::Null);
    assert_eq!(configured["providerBinding"]["personaSource"], "local");
    let local_only = native_fact(
        &daemon,
        &agent,
        "Synthetic unselected fact stays solely in Anima",
    );
    let selected = native_fact(&daemon, &agent, "Synthetic selected preference: green tea");
    let publication_id = Uuid::new_v4().to_string();
    let publication_base = format!("/api/agents/{agent}/memory-publications");
    let publication_path = format!("{publication_base}/{publication_id}");
    let request = json!({"publicationId":publication_id,"localId":selected["id"],"expectedContent":selected["content"],"title":"Synthetic tea preference","source":"Explicit synthetic owner selection","basis":"owner_statement"});
    let admitted = daemon.api("POST", &publication_base, request.clone());
    assert_eq!(admitted["state"], "queued");
    assert_eq!(admitted["canonical"], Value::Null);
    let repeated = daemon.api("POST", &publication_base, request);
    assert_eq!(repeated["id"], admitted["id"]);
    assert_eq!(repeated["operationId"], admitted["operationId"]);
    let pending = daemon.wait_state(&publication_path, "submitted_for_review");
    assert_eq!(pending["canonical"], Value::Null);
    let mut other = harness(&fixture.b_config);
    assert_eq!(
        other.call("mother_get_context", json!({}))["inventory"]["recallableRecords"],
        0
    );
    assert_eq!(
        other.call(
            "mother_search_context",
            json!({"query":"Synthetic","mode":"literal"})
        )["records"],
        json!([])
    );
    let mut owner = fixture.owner();
    let proposal_id = pending["proposalId"].as_str().unwrap();
    assert_eq!(
        owner.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let original_proposal = owner.inbox_get(proposal_id).unwrap();
    assert_eq!(original_proposal["record"]["content"], selected["content"]);

    // Restart while still pending. Only fixture HTTP shutdown is used; no live service is touched.
    daemon.stop();
    daemon = Daemon::start(&executable, &profile, &namespace, &owner_token);
    let restored = daemon.wait_state(&publication_path, "submitted_for_review");
    assert_eq!(restored["proposalId"], pending["proposalId"]);
    assert_eq!(restored["operationId"], pending["operationId"]);
    assert_eq!(restored["canonical"], Value::Null);
    assert_eq!(
        owner.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1,
        "restart must not submit another proposal"
    );
    assert_local_fact(&daemon, &local_only);
    assert_local_fact(&daemon, &selected);
    daemon.stop();

    // Keep Anima offline until canonical revision 2 exists. Reconciliation must use
    // the immutable revision-1 approval receipt, not the proposal's currentRecord.
    let mut edited = original_proposal["record"].clone();
    edited["content"] = json!("Synthetic owner-edited preference: black tea");
    let reviewed = owner.review_proposal(json!({"workspaceId":fixture.workspace["id"],"generation":fixture.workspace["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":proposal_id,"expectedRevision":original_proposal["revision"],"action":"save","record":edited})).unwrap();
    let approved = &reviewed["receipt"]["record"];
    assert_eq!(approved["revision"], 1);
    assert_eq!(approved["content"], edited["content"]);
    assert_eq!(
        other.call("mother_get_record", json!({"id":approved["id"]})),
        *approved
    );
    let mut corrected = edited;
    corrected["content"] = json!("Synthetic correction by the second harness: oolong tea");
    let correction = other.call(
        "mother_write_record",
        fixture.update(corrected, &approved["id"], 1),
    );
    assert_eq!(correction["record"]["revision"], 2);
    assert_eq!(correction["record"]["actor"], "harness-b");
    assert_eq!(
        owner.inbox_get(proposal_id).unwrap()["currentRecord"],
        Value::Null,
        "a creation proposal has no target record"
    );

    daemon = Daemon::start(&executable, &profile, &namespace, &owner_token);
    // The publication worker owns reconnection after restart. A test-issued
    // connect can race it and would hide whether unattended recovery works.
    let saved = daemon.wait_state(&publication_path, "saved");
    assert_eq!(saved["canonical"]["id"], approved["id"]);
    assert_eq!(
        saved["canonical"]["revision"], 1,
        "mapping identifies the approved revision, not a later harness correction"
    );
    assert_eq!(saved["canonical"]["ownerEdited"], true);
    assert_eq!(saved["operationId"], admitted["operationId"]);
    let all = daemon.api("GET", &publication_base, Value::Null);
    assert_eq!(
        all["items"].as_array().unwrap().len(),
        1,
        "ordinary native facts never enter the selected queue"
    );
    assert_local_fact(&daemon, &local_only);
    assert_local_fact(&daemon, &selected);
    assert_eq!(
        other.call("mother_get_record", json!({"id":approved["id"]})),
        correction["record"]
    );
    assert_eq!(
        other.call("mother_get_context", json!({}))["inventory"]["recallableRecords"],
        1,
        "unselected local content never reaches the garden"
    );
    assert_eq!(owner.inbox_list().unwrap()["proposals"], json!([]));
    let foreign_receipt = other.result(
        "mother_get_receipt",
        json!({"operationId":admitted["operationId"]}),
    );
    assert_eq!(foreign_receipt["isError"], true);
    assert_eq!(
        foreign_receipt["structuredContent"]["error"]["code"],
        "NOT_FOUND"
    );
    let mut original_client = harness(&config);
    assert_eq!(
        original_client.call(
            "mother_get_receipt",
            json!({"operationId":admitted["operationId"]})
        )["proposal"]["status"],
        "pending",
        "original submission receipt remains immutable"
    );

    // A targeted correction has a non-null currentRecord. Correct it again while
    // Anima is offline to exercise that field diverging from the approval receipt.
    let correction_source = native_fact(&daemon, &agent, "Synthetic Anima correction: jasmine tea");
    let correction_id = Uuid::new_v4().to_string();
    let correction_path = format!("{publication_base}/{correction_id}");
    daemon.api("POST", &publication_base, json!({"publicationId":correction_id,"localId":correction_source["id"],"expectedContent":correction_source["content"],"title":"Synthetic tea preference","source":"Explicit synthetic owner correction","basis":"owner_statement","target":{"id":approved["id"],"expectedRevision":2}}));
    let correction_pending = daemon.wait_state(&correction_path, "submitted_for_review");
    assert_eq!(correction_pending["canonical"], Value::Null);
    daemon.stop();
    let correction_proposal_id = correction_pending["proposalId"].as_str().unwrap();
    let correction_proposal = owner.inbox_get(correction_proposal_id).unwrap();
    let correction_reviewed = owner.review_proposal(json!({"workspaceId":fixture.workspace["id"],"generation":fixture.workspace["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":correction_proposal_id,"expectedRevision":correction_proposal["revision"],"action":"save"})).unwrap();
    assert_eq!(correction_reviewed["receipt"]["record"]["revision"], 3);
    let mut last_draft = correction_proposal["record"].clone();
    last_draft["content"] = json!("Synthetic later cross-harness correction: white tea");
    let latest = other.call(
        "mother_write_record",
        fixture.update(last_draft, &approved["id"], 3),
    );
    assert_eq!(latest["record"]["revision"], 4);
    assert_eq!(
        owner.inbox_get(correction_proposal_id).unwrap()["currentRecord"],
        latest["record"]
    );
    daemon = Daemon::start(&executable, &profile, &namespace, &owner_token);
    let correction_saved = daemon.wait_state(&correction_path, "saved");
    assert_eq!(correction_saved["canonical"]["id"], approved["id"]);
    assert_eq!(
        correction_saved["canonical"]["revision"], 3,
        "targeted correction must map its immutable approval, not currentRecord revision 4"
    );
    assert_eq!(
        daemon.api("GET", &publication_path, Value::Null)["canonical"]["revision"],
        1
    );
    assert_local_fact(&daemon, &local_only);
    assert_local_fact(&daemon, &selected);
    assert_local_fact(&daemon, &correction_source);
    assert_eq!(
        other.call("mother_get_record", json!({"id":approved["id"]})),
        latest["record"]
    );
    assert_eq!(
        other.call("mother_get_context", json!({}))["inventory"]["recallableRecords"],
        1
    );
    assert_eq!(
        daemon.api("GET", &publication_base, Value::Null)["items"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(owner.inbox_list().unwrap()["proposals"], json!([]));
    daemon.stop();
    {
        let context = other.call("mother_get_context", json!({}));
        assert_eq!(context["workspace"]["id"], fixture.workspace["id"]);
        assert_eq!(
            context["workspace"]["generation"],
            fixture.workspace["generation"]
        );
        assert!(fixture.dir.path().join("garden.redb").is_file());
        assert!(!fixture.dir.path().join("context.sqlite").exists());
        assert!(!profile.join("unused-mother-home/context.sqlite").exists());
    }
}

const GRAPH_TOOLS: [&str; 7] = [
    "mother_garden_get",
    "mother_garden_query",
    "mother_garden_apply",
    "mother_garden_propose",
    "mother_garden_get_proposal",
    "mother_garden_get_receipt",
    "mother_garden_changes",
];

fn full_graph_grant(fixture: &mut AcceptanceFixture, agent: &str) -> PathBuf {
    let owner = fixture.native.as_mut().expect("native synthetic owner");
    owner.stop();
    let direct = fixture.dir.path().join("graph-anima.direct.private.json");
    cli(
        fixture.dir.path(),
        &[
            "garden",
            "grant-v1",
            "graph-anima-acceptance",
            "--db",
            owner.database.to_str().unwrap(),
            "--out",
            direct.to_str().unwrap(),
            "--access",
            "companion",
            "--subject",
            agent,
            "--all",
            "--scopes",
            "read,context:propose,garden:read,garden:write,garden:propose",
        ],
    );
    owner.start();
    let config = fixture.dir.path().join("graph-anima.service.private.json");
    owner.bridge_config(&direct, &config);
    config
}

fn full_catalog(config: &Path) -> Vec<Value> {
    let mut bridge = harness(config);
    let mut tools = Vec::new();
    let mut request = json!({});
    let mut pages = 0;
    loop {
        let page = bridge.rpc("tools/list", request);
        assert!(serde_json::to_vec(&page).unwrap().len() <= 48 * 1024);
        tools.extend(page["tools"].as_array().unwrap().iter().cloned());
        pages += 1;
        let Some(cursor) = page["nextCursor"].as_str() else {
            break;
        };
        assert!(
            pages < 8,
            "native discovery must finish within host page budget"
        );
        request = json!({"cursor":cursor});
    }
    assert!(pages >= 2, "full graph catalog exercises paged discovery");
    assert_eq!(tools.len(), 19, "all v1, capability and graph descriptors");
    tools
}

fn assert_complete_candidates(connection: &Value, expected: &[Value]) {
    let candidates = connection["candidates"].as_array().unwrap();
    assert_eq!(candidates.len(), expected.len());
    for tool in expected {
        let matches: Vec<_> = candidates
            .iter()
            .filter(|candidate| candidate["remoteName"] == tool["name"])
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "one unchanged candidate for {}",
            tool["name"]
        );
        let candidate = matches[0];
        assert_eq!(candidate["inputSchema"], tool["inputSchema"]);
        assert_eq!(candidate["outputSchema"], tool["outputSchema"]);
        assert_eq!(candidate["description"], tool["description"]);
        assert!(
            candidate["descriptorDigest"]
                .as_str()
                .is_some_and(|s| !s.is_empty())
        );
    }
}

#[test]
#[ignore = "Requires ANIMA_MOTHER_TEST_DAEMON pointing to the explicitly built full graph Anima daemon"]
fn additive_anima_native_full_graph_discovery_review_grants_and_restart() {
    let executable = PathBuf::from(
        std::env::var_os("ANIMA_MOTHER_TEST_DAEMON")
            .expect("select the full graph Anima daemon executable explicitly"),
    );
    let mut fixture = AcceptanceFixture::new();
    let profile = fixture.dir.path().join("graph-anima-profile");
    std::fs::create_dir(&profile).unwrap();
    let namespace = Uuid::new_v4().simple().to_string();
    let owner_token = Uuid::new_v4().to_string();
    let mut daemon = Daemon::start(&executable, &profile, &namespace, &owner_token);
    let created = daemon.api(
        "POST",
        "/api/agents",
        json!({
            "name":"Synthetic graph admission owner","model":"synthetic-no-model-calls"
        }),
    );
    let agent = created["agent"]["state"]["id"].as_str().unwrap();
    let config = full_graph_grant(&mut fixture, agent);
    let catalog = full_catalog(&config);
    let local = native_fact(
        &daemon,
        agent,
        "Synthetic native fact remains outside graph",
    );
    let base = format!("/api/agents/{agent}/mcp-connections");
    let saved = daemon.api(
        "POST",
        &base,
        json!({
            "label":"Synthetic complete native graph",
            "stdio":{"executable":env!("CARGO_BIN_EXE_mother"),"args":["mcp","--config",config]}
        }),
    );
    let path = format!("{base}/{}", saved["connection"]["id"].as_str().unwrap());
    let enabled = daemon.api(
        "PATCH",
        &path,
        json!({
            "expectedVersion":saved["connection"]["version"],"enabled":true
        }),
    );
    let mut connected = daemon.api(
        "POST",
        &format!("{path}/connect"),
        json!({
            "expectedVersion":enabled["connection"]["version"]
        }),
    );
    assert_complete_candidates(&connected["connection"], &catalog);
    assert_eq!(connected["connection"]["reviewedTools"], json!([]));
    let original_candidates = connected["connection"]["candidates"].clone();
    for name in GRAPH_TOOLS {
        let row = &connected["connection"];
        let candidate = row["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|candidate| candidate["remoteName"] == name)
            .unwrap();
        connected = daemon.api(
            "POST",
            &format!("{path}/review"),
            json!({
                "expectedVersion":row["version"],"discoveryId":row["discoveryId"],
                "descriptorDigest":candidate["descriptorDigest"]
            }),
        );
    }
    assert_eq!(
        connected["connection"]["reviewedTools"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    for tool in connected["connection"]["reviewedTools"]
        .as_array()
        .unwrap()
        .clone()
    {
        let response = daemon.api(
            "PATCH",
            &format!("{path}/grants/{}", tool["alias"].as_str().unwrap()),
            json!({
                "expectedVersion":connected["connection"]["version"],
                "descriptorDigest":tool["descriptorDigest"],"granted":true
            }),
        );
        assert_eq!(response["granted"], true);
        assert!(
            response["toolNames"]
                .as_array()
                .unwrap()
                .contains(&tool["alias"])
        );
        connected = response;
    }
    let reviewed = connected["connection"]["reviewedTools"].clone();
    daemon.stop();
    daemon = Daemon::start(&executable, &profile, &namespace, &owner_token);
    let restored = daemon.api("GET", &path, Value::Null);
    assert_eq!(restored["connection"]["reviewedTools"], reviewed);
    connected = daemon.api(
        "POST",
        &format!("{path}/connect"),
        json!({
            "expectedVersion":restored["connection"]["version"]
        }),
    );
    assert_complete_candidates(&connected["connection"], &catalog);
    assert_eq!(connected["connection"]["candidates"], original_candidates);
    assert_eq!(connected["connection"]["reviewedTools"], reviewed);
    for tool in reviewed.as_array().unwrap() {
        let response = daemon.api(
            "PATCH",
            &format!("{path}/grants/{}", tool["alias"].as_str().unwrap()),
            json!({
                "expectedVersion":connected["connection"]["version"],
                "descriptorDigest":tool["descriptorDigest"],"granted":false
            }),
        );
        assert_eq!(response["granted"], false);
        assert!(
            !response["toolNames"]
                .as_array()
                .unwrap()
                .contains(&tool["alias"])
        );
        connected = response;
    }
    assert_local_fact(&daemon, &local);
    assert!(!fixture.dir.path().join("context.sqlite").exists());
    assert!(!profile.join("unused-mother-home/context.sqlite").exists());
    daemon.stop();
}
