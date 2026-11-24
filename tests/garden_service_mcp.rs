//! Real owner process and two stdio harness bridges; every store and key is synthetic.
use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, store::GardenStore},
};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
#[path = "support/mcp_discovery.rs"]
mod discovery;

struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
struct Harness {
    _process: Process,
    input: ChildStdin,
    responses: mpsc::Receiver<Value>,
    next: u64,
}
impl Harness {
    fn new(config: &Path) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(["mcp", "--config"])
            .arg(config)
            .env("MOTHER_HOME", config.parent().unwrap())
            .env(
                "MOTHER_MODEL_DIR",
                config.parent().unwrap().join("missing-model"),
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let (tx, responses) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                let Ok(line) = line else { break };
                let Ok(value) = serde_json::from_str(&line) else {
                    break;
                };
                if tx.send(value).is_err() {
                    break;
                }
            }
        });
        let mut harness = Self {
            _process: Process(child),
            input,
            responses,
            next: 0,
        };
        let initialized=harness.rpc("initialize",json!({"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"synthetic-service-harness","version":"1"}}));
        assert!(initialized["result"]["serverInfo"]["name"].is_string());
        writeln!(
            harness.input,
            "{}",
            json!({"jsonrpc":"2.0","method":"notifications/initialized"})
        )
        .unwrap();
        harness.input.flush().unwrap();
        harness
    }
    fn rpc(&mut self, method: &str, params: Value) -> Value {
        self.next += 1;
        let id = self.next;
        writeln!(
            self.input,
            "{}",
            json!({"jsonrpc":"2.0","id":id,"method":method,"params":params})
        )
        .unwrap();
        self.input.flush().unwrap();
        loop {
            let value = self
                .responses
                .recv_timeout(Duration::from_secs(10))
                .expect("bounded service MCP response");
            if value["id"] == id {
                return value;
            }
        }
    }
    fn call(&mut self, name: &str, args: Value) -> Value {
        let reply = self.rpc("tools/call", json!({"name":name,"arguments":args}));
        assert_ne!(reply["result"]["isError"], true, "{reply}");
        reply["result"]["structuredContent"].clone()
    }
}

#[test]
fn two_stdio_harnesses_share_one_local_owner_and_reconcile_the_same_write() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let db = dir.path().join("garden.redb");
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(&db, &ws, &generation).unwrap();
    store.v1_initialize("Synthetic owner persona").unwrap();
    let direct = dir.path().join("direct.private.json");
    let other_direct = dir.path().join("attachment.private.json");
    store
        .v1_grant_to_file_with_access(
            "synthetic-client",
            &["read".into(), "memory:write".into(), "garden:read".into()],
            &direct,
            &db,
            &ClientAccess::default(),
            Some(&GardenAccess::All {}),
        )
        .unwrap();
    store
        .v1_grant_to_file_with_access(
            "attachment-client",
            &["read".into(), "media:attach".into()],
            &other_direct,
            &db,
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    drop(store);
    let endpoint = dir.path().join("service.json");
    let mut owner = Process(
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(["garden", "service", "--db"])
            .arg(&db)
            .arg("--endpoint")
            .arg(&endpoint)
            .env("MOTHER_HOME", dir.path())
            .env("MOTHER_MODEL_DIR", dir.path().join("missing-model"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    while !endpoint.exists() {
        assert!(
            owner.0.try_wait().unwrap().is_none(),
            "owner exited before publishing local endpoint"
        );
        assert!(Instant::now() < deadline, "owner startup timed out");
        thread::sleep(Duration::from_millis(10));
    }
    let configure = |direct: &Path, config: &Path| {
        let configured = Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(["garden", "service-config", "--config"])
            .arg(direct)
            .arg("--endpoint")
            .arg(&endpoint)
            .arg("--out")
            .arg(config)
            .env("MOTHER_HOME", dir.path())
            .output()
            .unwrap();
        assert!(
            configured.status.success(),
            "{}",
            String::from_utf8_lossy(&configured.stderr)
        );
        let secret: Value = serde_json::from_slice(&std::fs::read(config).unwrap()).unwrap();
        assert_eq!(secret["backend"], "garden_service");
        assert!(
            !String::from_utf8_lossy(&configured.stdout)
                .contains(secret["token"].as_str().unwrap())
        );
    };
    let config = dir.path().join("service.private.json");
    let other_config = dir.path().join("attachment.service.json");
    configure(&direct, &config);
    configure(&other_direct, &other_config);
    let mut first = Harness::new(&config);
    let mut second = Harness::new(&config);
    let tools = discovery::collect(|params| first.rpc("tools/list", params));
    assert_eq!(tools.len(), 19);
    let legacy: Value = serde_json::from_str(include_str!("../contracts/mcp-tools.json")).unwrap();
    for expected in legacy.as_array().unwrap() {
        let actual = tools
            .iter()
            .find(|tool| tool["name"] == expected["name"])
            .unwrap();
        let mut schema = actual["inputSchema"].clone();
        schema.as_object_mut().unwrap().remove("$schema");
        let mut expected_schema = expected["inputSchema"].clone();
        expected_schema.as_object_mut().unwrap().remove("$schema");
        assert_eq!(schema, expected_schema);
    }
    let request = json!({"workspaceId":ws,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"IPC continuity","content":"Synthetic selected memory","status":"active","basis":"owner_statement"}});
    let receipt = first.call("mother_write_record", request.clone());
    assert_eq!(second.call("mother_write_record", request), receipt);
    assert_eq!(
        second.call("mother_get_record", json!({"id":receipt["record"]["id"]})),
        receipt["record"]
    );
    assert_eq!(
        second.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        1
    );
    use mother::service::attachment::{PREPARE, STATUS, SUBMIT, UPLOAD};
    use sha2::{Digest, Sha256};
    assert!(tools.iter().all(|tool| tool["name"] != PREPARE));
    let mut transfer = Harness::new(&other_config);
    let catalog = discovery::collect(|params| transfer.rpc("tools/list", params));
    let expected: Value =
        serde_json::from_str(include_str!("../contracts/media-attachment-tools.json")).unwrap();
    for entry in expected.as_array().unwrap() {
        let actual = catalog
            .iter()
            .find(|tool| tool["name"] == entry["name"])
            .unwrap();
        assert_eq!(actual["inputSchema"], entry["inputSchema"]);
    }
    assert_eq!(catalog.len(), 17);
    let bytes = include_bytes!("fixtures/attachment-sample.mp4");
    let operation = uuid::Uuid::new_v4().to_string();
    let target = transfer.call("mother_get_record", json!({"id":receipt["record"]["id"]}));
    let input = json!({"operationId":operation,"memoryId":target["id"],"expectedRevision":target["revision"],
        "file":{"fileName":"fixture.mp4","sizeBytes":bytes.len(),"sha256":format!("{:x}",Sha256::digest(bytes))}});
    let denied = second.rpc("tools/call", json!({"name":PREPARE,"arguments":input}));
    assert_eq!(denied["result"]["isError"], true);
    let prepared = transfer.call(PREPARE, input.clone());
    let token = prepared["transferToken"].clone();
    let first_chunk = json!({"operationId":operation,"transferToken":token,"offset":0,"contentBase64":base64::encode(&bytes[..512])});
    assert_eq!(transfer.call(UPLOAD, first_chunk)["receivedBytes"], 512);
    drop(transfer);
    let mut transfer = Harness::new(&other_config);
    let query = json!({"operationId":operation});
    assert_eq!(transfer.call(STATUS, query.clone())["receivedBytes"], 512);
    assert_eq!(transfer.call(PREPARE, input)["transferToken"], token);
    transfer.call(UPLOAD,json!({"operationId":operation,"transferToken":token,"offset":512,"contentBase64":base64::encode(&bytes[512..])}));
    let staged = transfer.call(
        SUBMIT,
        json!({"operationId":operation,"transferToken":token}),
    );
    assert_eq!(staged["status"], "pending_owner_review");
    let denied = transfer.rpc(
        "tools/call",
        json!({"name":"local_media_finish_review","arguments":query}),
    );
    assert!(denied["error"].is_object());
    drop(transfer);
    drop(first);
    drop(second);
    drop(owner);
    let mut direct_host = Harness::new(&other_direct);
    let listed = discovery::collect(|params| direct_host.rpc("tools/list", params));
    assert!(listed.iter().any(|tool| tool["name"] == PREPARE));
    assert_eq!(direct_host.call(STATUS, query), staged);
    drop(direct_host);
    let reopened = GardenStore::open(&db).unwrap();
    assert!(reopened.v1_initialized().unwrap());
    assert!(!dir.path().join("context.sqlite").exists());
}
