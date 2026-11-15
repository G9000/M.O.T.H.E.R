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
    let config = dir.path().join("service.private.json");
    let configured = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", "service-config", "--config"])
        .arg(&direct)
        .arg("--endpoint")
        .arg(&endpoint)
        .arg("--out")
        .arg(&config)
        .env("MOTHER_HOME", dir.path())
        .output()
        .unwrap();
    assert!(
        configured.status.success(),
        "{}",
        String::from_utf8_lossy(&configured.stderr)
    );
    let secret: Value = serde_json::from_slice(&std::fs::read(&config).unwrap()).unwrap();
    assert_eq!(secret["backend"], "garden_service");
    assert!(
        !String::from_utf8_lossy(&configured.stdout).contains(secret["token"].as_str().unwrap())
    );
    let mut first = Harness::new(&config);
    let mut second = Harness::new(&config);
    let tools = first.rpc("tools/list", json!({}));
    assert_eq!(tools["result"]["tools"].as_array().unwrap().len(), 19);
    let legacy: Value = serde_json::from_str(include_str!("../contracts/mcp-tools.json")).unwrap();
    for expected in legacy.as_array().unwrap() {
        let actual = tools["result"]["tools"]
            .as_array()
            .unwrap()
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
    drop(first);
    drop(second);
    drop(owner);
    let reopened = GardenStore::open(&db).unwrap();
    assert!(reopened.v1_initialized().unwrap());
    assert!(!dir.path().join("context.sqlite").exists());
}
