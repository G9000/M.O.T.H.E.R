//! Synthetic cross-harness qualification through real CLI, owner IPC and MCP processes.
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

fn owner_cli(home: &Path, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", home)
        .env("MOTHER_MODEL_DIR", home.join("missing-model"))
        .output()
        .expect("run synthetic owner CLI");
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("owner CLI JSON result")
}

fn start_owner(home: &Path, db: &Path, endpoint: &Path) -> Process {
    let previous = std::fs::read(endpoint).ok();
    let mut owner = Process(
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(["garden", "service", "--db"])
            .arg(db)
            .arg("--endpoint")
            .arg(endpoint)
            .env("MOTHER_HOME", home)
            .env("MOTHER_MODEL_DIR", home.join("missing-model"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("start synthetic native owner"),
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        assert!(
            owner.0.try_wait().unwrap().is_none(),
            "owner exited before publishing a fresh endpoint"
        );
        if let Ok(bytes) = std::fs::read(endpoint)
            && Some(&bytes) != previous.as_ref()
            && serde_json::from_slice::<Value>(&bytes).is_ok()
        {
            return owner;
        }
        assert!(Instant::now() < deadline, "owner startup timed out");
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn selected_capture_crosses_harnesses_after_local_review_and_survives_restart_and_revocation() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let db = dir.path().join("garden.redb");
    let workspace = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let direct_first = dir.path().join("first.private.json");
    let direct_second = dir.path().join("second.private.json");
    let store = GardenStore::initialize(&db, &workspace, &generation).unwrap();
    store
        .v1_initialize("Synthetic qualification persona")
        .unwrap();
    for (client, config) in [
        ("synthetic-collector", &direct_first),
        ("synthetic-reader", &direct_second),
    ] {
        store
            .v1_grant_to_file_with_access(
                client,
                &["read".into(), "context:propose".into()],
                config,
                &db,
                &ClientAccess::default(),
                Some(&GardenAccess::default()),
            )
            .unwrap();
    }
    drop(store);
    let endpoint = dir.path().join("service.json");
    let owner = start_owner(dir.path(), &db, &endpoint);
    let first_config = dir.path().join("first-service.private.json");
    let second_config = dir.path().join("second-service.private.json");
    for (direct, config) in [
        (&direct_first, &first_config),
        (&direct_second, &second_config),
    ] {
        owner_cli(
            dir.path(),
            &[
                "garden",
                "service-config",
                "--config",
                direct.to_str().unwrap(),
                "--endpoint",
                endpoint.to_str().unwrap(),
                "--out",
                config.to_str().unwrap(),
            ],
        );
    }
    let mut first = Harness::new(&first_config);
    let mut second = Harness::new(&second_config);
    let operation = uuid::Uuid::new_v4().to_string();
    // The collector selects one attributed fact from this local synthetic conversation.
    let conversation = [
        "Qualification tea preference: choose black tea.",
        "Unselected synthetic noise: violet umbrellas 3921.",
    ];
    let request = json!({
        "workspaceId":workspace,"generation":generation,"operationId":operation,
        "expectedRevision":0,"source":"Synthetic owner statement in collector harness",
        "basis":"owner_statement","record":{"kind":"memory","title":"Qualification tea preference",
        "content":conversation[0],"basis":"owner_statement","status":"active",
        "sources":[{"label":"Synthetic selected conversation","uri":"https://example.invalid/synthetic/selected"}]}
    });
    let admitted = first.call("mother_propose_memory", request.clone());
    // Discarding an acknowledgement is reconciled with the same immutable request UUID.
    assert_eq!(
        first.call("mother_propose_memory", request.clone()),
        admitted
    );
    assert_eq!(admitted["operationId"], operation);
    let proposal_id = admitted["proposal"]["id"].clone();
    let pending = first.call("mother_get_proposal", json!({"id":proposal_id}));
    assert_eq!(pending["actor"], "synthetic-collector");
    assert_eq!(pending["source"], request["source"]);
    assert_eq!(pending["status"], "pending");
    assert_eq!(
        second.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        0
    );
    assert_eq!(
        second.call(
            "mother_search_context",
            json!({"query":"Qualification tea preference","mode":"literal"})
        )["records"],
        json!([])
    );
    // MCP has no owner approval capability, even for a client allowed to propose.
    let tools = discovery::collect(|params| first.rpc("tools/list", params));
    assert!(tools.iter().all(|tool| {
        !tool["name"].as_str().unwrap().contains("review")
            && !tool["name"].as_str().unwrap().contains("approve")
    }));
    drop(owner);
    let review = json!({"workspaceId":workspace,"generation":generation,
        "operationId":uuid::Uuid::new_v4().to_string(),"proposalId":proposal_id,
        "expectedRevision":1,"action":"save"});
    let review_path = dir.path().join("owner-reviewed.json");
    std::fs::write(&review_path, serde_json::to_vec(&review).unwrap()).unwrap();
    let review_args = [
        "garden",
        "review",
        "--db",
        db.to_str().unwrap(),
        "--request",
        review_path.to_str().unwrap(),
    ];
    let saved = owner_cli(dir.path(), &review_args);
    assert_eq!(saved["status"], "saved");
    assert_eq!(saved["operationId"], review["operationId"]);
    assert_eq!(owner_cli(dir.path(), &review_args), saved);
    let canonical = saved["receipt"]["record"].clone();
    assert_eq!(canonical["actor"], "@owner-review");
    assert_eq!(canonical["sources"], request["record"]["sources"]);
    let owner = start_owner(dir.path(), &db, &endpoint);
    // Both already-running bridges rediscover the restarted owner from its descriptor.
    assert_eq!(first.call("mother_propose_memory", request), admitted);
    assert_eq!(
        first.call("mother_get_receipt", json!({"operationId":operation})),
        admitted
    );
    let resolved = first.call("mother_get_proposal", json!({"id":proposal_id}));
    assert_eq!(resolved["actor"], "synthetic-collector");
    assert_eq!(resolved["source"], pending["source"]);
    assert_eq!(resolved["receipt"], saved);
    assert_eq!(
        second.call("mother_get_record", json!({"id":canonical["id"]})),
        canonical
    );
    let recalled = second.call(
        "mother_search_context",
        json!({"query":"Qualification tea preference","mode":"literal"}),
    );
    assert_eq!(recalled["records"].as_array().unwrap().len(), 1);
    assert_eq!(recalled["records"][0]["id"], canonical["id"]);
    assert_eq!(
        second.call(
            "mother_search_context",
            json!({"query":conversation[1],"mode":"literal"})
        )["records"],
        json!([])
    );
    assert_eq!(
        second.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        1
    );
    drop(owner);
    assert_eq!(
        owner_cli(
            dir.path(),
            &[
                "garden",
                "review",
                "--db",
                db.to_str().unwrap(),
                "--request",
                review_path.to_str().unwrap()
            ]
        ),
        saved
    );
    owner_cli(
        dir.path(),
        &[
            "garden",
            "revoke",
            "synthetic-collector",
            "--db",
            db.to_str().unwrap(),
        ],
    );
    let owner = start_owner(dir.path(), &db, &endpoint);
    // A bridge that authenticated before revocation must not retain that authority.
    let denied = first.rpc(
        "tools/call",
        json!({"name":"mother_get_context","arguments":{}}),
    );
    assert_eq!(denied["result"]["isError"], true, "{denied}");
    assert_eq!(
        denied["result"]["structuredContent"]["error"]["code"],
        "UNAUTHORIZED"
    );
    assert_eq!(
        second.call("mother_get_record", json!({"id":canonical["id"]})),
        canonical
    );
    assert_eq!(
        second.call("mother_get_context", json!({}))["workspace"]["generation"],
        generation
    );
    drop(first);
    drop(second);
    drop(owner);
    assert!(!dir.path().join("context.sqlite").exists());
    assert!(!dir.path().join("missing-model").exists());
}
