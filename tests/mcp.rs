use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{Barrier, mpsc},
    thread,
    time::Duration,
};
use tempfile::TempDir;
use uuid::Uuid;

struct Client {
    child: Child,
    stdin: ChildStdin,
    responses: mpsc::Receiver<Value>,
    reader: Option<thread::JoinHandle<()>>,
    next_id: u64,
}
impl Client {
    fn connect(config: &Path) -> Self {
        Self::connect_with_model_dir(config, &config.parent().unwrap().join("missing-model"))
    }
    fn connect_with_model_dir(config: &Path, model_dir: &Path) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
            .arg("mcp")
            .arg("--config")
            .arg(config)
            .env("MOTHER_MODEL_DIR", model_dir)
            .current_dir(config.parent().unwrap())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("start native MCP server");
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
        let mut client = Self {
            child,
            stdin,
            responses,
            reader: Some(reader),
            next_id: 0,
        };
        let initialized = client.rpc(
            "initialize",
            json!({
                "protocolVersion":"2025-06-18", "capabilities":{},
                "clientInfo":{"name":"mother-synthetic-test", "version":"1.0.0"}
            }),
        );
        assert_eq!(initialized["serverInfo"]["name"], "mother-workspace");
        client.send(json!({"jsonrpc":"2.0", "method":"notifications/initialized"}));
        client
    }
    fn send(&mut self, value: Value) {
        writeln!(self.stdin, "{value}").expect("write MCP request");
        self.stdin.flush().expect("flush MCP request");
    }
    fn rpc(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = self.next_id;
        self.send(json!({"jsonrpc":"2.0", "id":id, "method":method, "params":params}));
        loop {
            let message = self
                .responses
                .recv_timeout(Duration::from_secs(15))
                .expect("MCP response within 15 seconds");
            if message["id"] == id {
                assert!(message.get("error").is_none(), "protocol error: {message}");
                return message["result"].clone();
            }
        }
    }
    fn result(&mut self, name: &str, args: Value) -> Value {
        self.rpc("tools/call", json!({"name":name, "arguments":args}))
    }
    fn call(&mut self, name: &str, args: Value) -> Value {
        let result = self.result(name, args);
        assert_ne!(result["isError"], true, "tool failed: {result}");
        let parsed: Value =
            serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(parsed, result["structuredContent"]);
        result["structuredContent"].clone()
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}
struct Fixture {
    dir: TempDir,
    workspace: Value,
    a_config: PathBuf,
    b_config: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("SOUL.md"), "Synthetic MCP persona").unwrap();
        let initialized = cli(dir.path(), &["init"]);
        for client in ["harness-a", "harness-b"] {
            cli(
                dir.path(),
                &[
                    "grant",
                    client,
                    "--scopes",
                    "read,memory:write,decision:write,task:write,checkpoint:write",
                ],
            );
        }
        Self {
            workspace: initialized["workspace"].clone(),
            a_config: dir.path().join("clients/harness-a.private.json"),
            b_config: dir.path().join("clients/harness-b.private.json"),
            dir,
        }
    }
    fn request(&self, record: Value) -> Value {
        json!({"workspaceId":self.workspace["id"], "generation":self.workspace["generation"], "operationId":Uuid::new_v4().to_string(), "expectedRevision":0, "record":record})
    }
    fn update(&self, record: Value, id: &Value, revision: u64) -> Value {
        let mut request = self.request(record);
        request["id"] = id.clone();
        request["expectedRevision"] = revision.into();
        request
    }
}
fn cli(dir: &Path, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", dir)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
fn memory(content: &str) -> Value {
    json!({"kind":"memory", "title":"Tea", "content":content, "basis":"owner_statement", "status":"active"})
}

#[test]
fn real_stdio_round_trip_handoff_correction_task_progress_restart_and_connected_revocation() {
    let f = Fixture::new();
    let mut a = Client::connect(&f.a_config);
    let mut b = Client::connect(&f.b_config);
    let catalog = a.rpc("tools/list", json!({}));
    let mut expected: Value =
        serde_json::from_str(include_str!("../contracts/mcp-tools.json")).unwrap();
    // rmcp 3.9 no longer serializes the legacy task execution metadata.
    // The tools remain synchronous and advertise no task capability.
    for tool in expected.as_array_mut().unwrap() {
        assert_eq!(tool["execution"]["taskSupport"], "forbidden");
        tool.as_object_mut().unwrap().remove("execution");
        tool["inputSchema"]["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
    }
    assert_eq!(
        catalog["tools"], expected,
        "preserve all advertised tool names, descriptions, schemas and annotations"
    );
    let first = a.call("mother_write_record", f.request(memory("Green tea")));
    let task = a.call("mother_write_record", f.request(json!({"kind":"task", "title":"Plan a weekend", "content":"Choose a destination", "status":"todo"})));
    let checkpoint = a.call("mother_checkpoint", f.request(json!({"kind":"checkpoint", "title":"Weekend handoff", "content":"Destination still open", "status":"active", "nextStep":"Choose a destination with the owner", "references":[first["record"]["id"], task["record"]["id"]]})));
    let context = b.call("mother_get_context", json!({}));
    assert_eq!(context["inventory"]["recallableRecords"], 3);
    assert_eq!(context["inventory"]["recallableByKind"]["checkpoint"], 1);
    assert_eq!(context["selection"]["matchingRecords"], 3);
    assert_eq!(context["unsettledWorkSummary"]["total"], 0);

    assert_eq!(context["workspace"]["id"], f.workspace["id"]);
    assert!(
        context["records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == checkpoint["record"]["id"])
    );
    let corrected = b.call(
        "mother_write_record",
        f.update(memory("Black tea"), &first["record"]["id"], 1),
    );
    b.call("mother_write_record", f.update(json!({"kind":"task", "title":"Plan a weekend", "content":"Destination selected", "status":"done"}), &task["record"]["id"], 1));
    assert_eq!(corrected["record"]["actor"], "harness-b");
    drop(a);
    let mut resumed = Client::connect(&f.a_config);
    assert_eq!(
        resumed.call("mother_get_record", json!({"id":first["record"]["id"]})),
        corrected["record"]
    );
    assert_eq!(
        resumed.call("mother_get_record", json!({"id":task["record"]["id"]}))["status"],
        "done"
    );
    let stale = b.result(
        "mother_write_record",
        f.update(memory("Green tea"), &first["record"]["id"], 1),
    );
    assert_eq!(stale["isError"], true);
    assert_eq!(stale["structuredContent"]["error"]["code"], "CONFLICT");
    cli(f.dir.path(), &["revoke", "harness-b"]);
    let revoked = b.result("mother_get_context", json!({}));
    assert_eq!(revoked["isError"], true);
    assert_eq!(
        revoked["structuredContent"]["error"]["code"],
        "UNAUTHORIZED"
    );
}

#[test]
fn two_processes_race_one_revision_exactly_one_commit_survives_and_both_see_it_after_restart() {
    let f = Fixture::new();
    let mut a = Client::connect(&f.a_config);
    let mut b = Client::connect(&f.b_config);
    let first = a.call("mother_write_record", f.request(memory("Green tea")));
    let one = f.update(memory("Winner one"), &first["record"]["id"], 1);
    let two = f.update(memory("Winner two"), &first["record"]["id"], 1);
    let barrier = Barrier::new(2);
    let results = thread::scope(|scope| {
        let first = scope.spawn(|| {
            barrier.wait();
            a.result("mother_write_record", one)
        });
        let second = scope.spawn(|| {
            barrier.wait();
            b.result("mother_write_record", two)
        });
        [first.join().unwrap(), second.join().unwrap()]
    });
    assert_eq!(results.iter().filter(|r| r["isError"] != true).count(), 1);
    let loser = results.iter().find(|r| r["isError"] == true).unwrap();
    assert_eq!(loser["structuredContent"]["error"]["code"], "CONFLICT");
    let winner = &results.iter().find(|r| r["isError"] != true).unwrap()["structuredContent"];
    assert_eq!(winner["record"]["revision"], 2);
    drop(a);
    drop(b);
    let mut a = Client::connect(&f.a_config);
    let mut b = Client::connect(&f.b_config);
    assert_eq!(
        a.call("mother_get_record", json!({"id":first["record"]["id"]})),
        winner["record"]
    );
    assert_eq!(
        b.call("mother_get_record", json!({"id":first["record"]["id"]})),
        winner["record"]
    );
    let owner = if winner["record"]["actor"] == "harness-a" {
        &mut a
    } else {
        &mut b
    };
    assert_eq!(
        owner.call(
            "mother_get_receipt",
            json!({"operationId":winner["operationId"]})
        ),
        *winner
    );
}

#[test]
fn duplicate_operation_race_creates_one_record_and_read_only_clients_cannot_mutate() {
    let f = Fixture::new();
    let mut a = Client::connect(&f.a_config);
    let mut twin = Client::connect(&f.a_config);
    let mutation = f.request(memory("Green tea"));
    let barrier = Barrier::new(2);
    let receipts = thread::scope(|scope| {
        let one = scope.spawn(|| {
            barrier.wait();
            a.call("mother_write_record", mutation.clone())
        });
        let two = scope.spawn(|| {
            barrier.wait();
            twin.call("mother_write_record", mutation.clone())
        });
        [one.join().unwrap(), two.join().unwrap()]
    });
    assert_eq!(receipts[0], receipts[1]);
    assert_eq!(
        a.call("mother_get_context", json!({}))["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        a.call(
            "mother_get_receipt",
            json!({"operationId":mutation["operationId"]})
        ),
        receipts[0]
    );
    cli(f.dir.path(), &["grant", "reader"]);
    let mut reader = Client::connect(&f.dir.path().join("clients/reader.private.json"));
    let denied = reader.result("mother_write_record", f.request(memory("Black tea")));
    assert_eq!(denied["isError"], true);
    assert_eq!(denied["structuredContent"]["error"]["code"], "FORBIDDEN");
    let mut proposal = f.request(memory("Black tea"));
    proposal["source"] = json!("Synthetic read-only submission");
    proposal["basis"] = json!("owner_statement");
    let denied = reader.result("mother_propose_memory", proposal);
    assert_eq!(denied["isError"], true);
    assert_eq!(denied["structuredContent"]["error"]["code"], "FORBIDDEN");
}

#[test]
fn malformed_tool_arguments_fail_without_mutating_records() {
    let f = Fixture::new();
    let mut client = Client::connect(&f.a_config);
    for (name, arguments) in [
        ("mother_search_context", json!({})),
        ("mother_search_context", json!({"query":"   "})),
        (
            "mother_search_context",
            json!({"query":"tea", "mode":"remote"}),
        ),
        ("mother_search_context", json!({"query":"tea", "mode":null})),
        ("mother_search_context", json!({"query":"tea", "limit":0})),
        ("mother_search_context", json!({"query":"tea", "limit":101})),
        (
            "mother_search_context",
            json!({"query":"tea", "kind":"unknown"}),
        ),
        ("mother_get_context", json!({"unknown":true})),
        (
            "mother_get_record",
            json!({"id":Uuid::new_v4().to_string(), "extra":true}),
        ),
        ("mother_get_receipt", json!({"operationId":"bad-id"})),
        ("mother_checkpoint", f.request(memory("Green tea"))),
    ] {
        let result = client.result(name, arguments);
        assert_eq!(result["isError"], true, "{name}: {result}");
        assert_eq!(
            result["structuredContent"]["error"]["code"], "INVALID_INPUT",
            "{name}: {result}"
        );
    }
    assert!(
        client.call("mother_get_context", json!({}))["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn search_uses_javascript_whitespace_semantics() {
    let f = Fixture::new();
    let mut client = Client::connect(&f.a_config);
    let saved = client.call("mother_write_record", f.request(memory("Green tea")));
    let bom = client.call(
        "mother_search_context",
        json!({"query":"\u{feff}Green tea\u{feff}"}),
    );
    let next_line = client.call(
        "mother_search_context",
        json!({"query":"\u{0085}Green tea\u{0085}"}),
    );
    assert_eq!(
        (
            bom["records"].as_array().unwrap().len(),
            next_line["records"].as_array().unwrap().len()
        ),
        (1, 0),
        "JavaScript trims BOM but keeps U+0085 literal"
    );
    assert_eq!(bom["records"][0]["id"], saved["record"]["id"]);
}

#[test]
fn semantic_search_reports_literal_fallback_without_a_model() {
    let f = Fixture::new();
    let mut client = Client::connect(&f.a_config);
    let record = client.call(
        "mother_write_record",
        f.request(memory("Tea with identifier ZX-731")),
    );
    let result = client.call(
        "mother_search_context",
        json!({"query":"ZX-731", "mode":"hybrid"}),
    );
    assert_eq!(result["search"]["requestedMode"], "hybrid");
    assert_eq!(result["search"]["effectiveMode"], "literal");
    assert_eq!(result["search"]["semanticUsed"], false);
    assert_eq!(result["search"]["reason"], "model_unavailable");
    assert_eq!(result["records"][0]["id"], record["record"]["id"]);
    let literal = client.call(
        "mother_search_context",
        json!({"query":"ZX-731", "mode":"literal"}),
    );
    assert_eq!(literal["search"]["effectiveMode"], "literal");
    assert_eq!(literal["records"], result["records"]);
    let invalid = client.result(
        "mother_search_context",
        json!({"query":"ZX-731", "mode":"remote"}),
    );
    assert_eq!(invalid["isError"], true);
}

#[test]
fn search_respects_kind_limits_and_connected_revocation_in_every_mode() {
    let f = Fixture::new();
    let mut client = Client::connect(&f.a_config);
    client.call("mother_write_record", f.request(memory("Tea ZX-731")));
    client.call("mother_write_record", f.request(memory("More tea ZX-731")));
    let task = client.call(
        "mother_write_record",
        f.request(json!({"kind":"task", "title":"Tea task", "content":"ZX-731", "status":"todo"})),
    );
    let result = client.call(
        "mother_search_context",
        json!({"query":"ZX-731", "kind":"task", "limit":1}),
    );
    assert_eq!(result["records"].as_array().unwrap().len(), 1);
    assert_eq!(result["records"][0]["id"], task["record"]["id"]);
    let memories = client.call(
        "mother_search_context",
        json!({"query":"ZX-731", "kind":"memory", "limit":1}),
    );
    assert_eq!(memories["records"].as_array().unwrap().len(), 1);
    assert_eq!(memories["omitted"], 1);
    cli(f.dir.path(), &["revoke", "harness-a"]);
    for mode in ["hybrid", "semantic", "literal"] {
        let result = client.result(
            "mother_search_context",
            json!({"query":"ZX-731", "mode":mode}),
        );
        assert_eq!(result["isError"], true, "{mode}: {result}");
        assert_eq!(
            result["structuredContent"]["error"]["code"], "UNAUTHORIZED",
            "{mode}: {result}"
        );
    }
}

#[test]
fn invalid_model_directory_does_not_block_mcp_reads_writes_or_search() {
    let f = Fixture::new();
    let relative_model = Path::new("relative-model");
    let mut client = Client::connect_with_model_dir(&f.a_config, relative_model);
    let initial = client.call("mother_get_context", json!({}));
    assert_eq!(initial["workspace"]["id"], f.workspace["id"]);
    let saved = client.call("mother_write_record", f.request(memory("Tea ZX-731")));
    let read = client.call("mother_get_record", json!({"id":saved["record"]["id"]}));
    assert_eq!(read, saved["record"]);
    for mode in ["literal", "hybrid", "semantic"] {
        let found = client.call(
            "mother_search_context",
            json!({"query":"ZX-731", "mode":mode}),
        );
        assert_eq!(found["records"][0]["id"], saved["record"]["id"]);
        assert_eq!(found["search"]["requestedMode"], mode);
        assert_eq!(found["search"]["effectiveMode"], "literal");
        assert_eq!(found["search"]["semanticUsed"], false);
        if mode != "literal" {
            assert_eq!(found["search"]["reason"], "model_unavailable");
        }
    }
    assert!(!f.a_config.parent().unwrap().join(relative_model).exists());
}

#[test]
#[ignore = "requires MOTHER_TEST_MODEL_DIR; exercises actual learned embeddings over stdio in release mode"]
fn real_model_semantic_mcp_search_survives_process_restart() {
    let model = PathBuf::from(
        std::env::var_os("MOTHER_TEST_MODEL_DIR").expect("set the verified model directory"),
    );
    let f = Fixture::new();
    let mut c = Client::connect_with_model_dir(&f.a_config, &model);
    let wanted = c.call(
        "mother_write_record",
        f.request(memory(
            "I avoid meat and fish and choose plant based meals.",
        )),
    );
    c.call(
        "mother_write_record",
        f.request(memory("The garden needs fresh soil and regular watering.")),
    );
    let result = c.call(
        "mother_search_context",
        json!({"query":"What can I cook for a vegetarian dinner?"}),
    );
    assert_eq!(result["search"]["semanticUsed"], true);
    assert_eq!(result["records"][0]["id"], wanted["record"]["id"]);
    drop(c);
    let mut c = Client::connect_with_model_dir(&f.a_config, &model);
    let result = c.call(
        "mother_search_context",
        json!({"query":"What can I cook for a vegetarian dinner?"}),
    );
    assert_eq!(result["search"]["semanticUsed"], true);
    assert_eq!(result["search"]["newlyIndexedRecords"], 0);
    assert_eq!(result["records"][0]["id"], wanted["record"]["id"]);
}

#[test]
fn connected_harnesses_observe_owner_soul_updates_without_restarting() {
    let f = Fixture::new();
    let mut a = Client::connect(&f.a_config);
    let mut b = Client::connect(&f.b_config);
    let before = a.call("mother_get_context", json!({}));
    let old_hash = before["persona"]["hash"].as_str().unwrap();
    let file = f.dir.path().join("edited-soul.md");
    let content = "Synthetic revised companion: prefer short, practical answers.";
    std::fs::write(&file, content).unwrap();
    let updated = cli(
        f.dir.path(),
        &[
            "soul",
            "update",
            "--file",
            file.to_str().unwrap(),
            "--expected-hash",
            old_hash,
        ],
    );
    for client in [&mut a, &mut b] {
        let context = client.call("mother_get_context", json!({}));
        assert_eq!(context["persona"]["content"], content);
        assert_eq!(context["persona"]["hash"], updated["hash"]);
        assert_eq!(context["workspace"], before["workspace"]);
        let search = client.call(
            "mother_search_context",
            json!({"query":"companion","mode":"literal"}),
        );
        assert_eq!(search["persona"], context["persona"]);
        assert_eq!(
            client.rpc("tools/list", json!({}))["tools"]
                .as_array()
                .unwrap()
                .len(),
            10
        );
    }
    let saved = a.call(
        "mother_write_record",
        f.request(memory("Unchanged memory capability")),
    );
    assert_eq!(
        b.call("mother_get_record", json!({"id":saved["record"]["id"]})),
        saved["record"]
    );
    drop(a);
    let mut resumed = Client::connect(&f.a_config);
    assert_eq!(
        resumed.call("mother_get_context", json!({}))["persona"]["hash"],
        updated["hash"]
    );
}
#[test]
fn reviewed_capture_crosses_two_real_harness_processes_only_after_owner_save() {
    use mother::store::Store;
    let f = Fixture::new();
    let mut chatgpt = Client::connect(&f.a_config);
    let mut hermes = Client::connect(&f.b_config);
    let mut owner = Store::open(&f.dir.path().join("context.sqlite")).unwrap();
    let mut q = f.request(json!({"kind":"decision","title":"Synthetic decision capture","content":"Choose tea","status":"confirmed"}));
    q["source"] = json!("Synthetic owner instruction in ChatGPT");
    q["basis"] = json!("owner_statement");
    let submitted = chatgpt.call("mother_propose_memory", q.clone());
    let p = &submitted["proposal"];
    assert!(
        hermes.call("mother_get_context", json!({}))["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        hermes.call(
            "mother_search_context",
            json!({"query":"tea","mode":"literal"})
        )["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        hermes.call("mother_get_proposal", json!({"id":p["id"]}))["record"],
        p["record"]
    );
    let save = json!({"workspaceId":f.workspace["id"],"generation":f.workspace["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":p["id"],"expectedRevision":1,"action":"save"});
    let saved = owner.review_proposal(save).unwrap();
    let record = &saved["receipt"]["record"];
    assert_eq!(
        hermes.call("mother_get_record", json!({"id":record["id"]})),
        *record
    );
    let mut correction = q;
    correction["id"] = record["id"].clone();
    correction["expectedRevision"] = json!(1);
    correction["operationId"] = json!(Uuid::new_v4().to_string());
    correction["record"]["content"] = json!("Choose black tea");
    let next = hermes.call("mother_propose_memory", correction);
    let reviewed=owner.review_proposal(json!({"workspaceId":f.workspace["id"],"generation":f.workspace["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":next["proposal"]["id"],"expectedRevision":1,"action":"save"})).unwrap();
    assert_eq!(reviewed["receipt"]["record"]["revision"], 2);
    assert_eq!(
        chatgpt.call("mother_get_record", json!({"id":record["id"]})),
        reviewed["receipt"]["record"]
    );
    assert_eq!(
        chatgpt.call(
            "mother_get_receipt",
            json!({"operationId":submitted["operationId"]})
        ),
        submitted
    );
}

#[test]
fn advertised_schema_dialect_matches_current_mcp_clients() {
    let fixture = Fixture::new();
    let mut client = Client::connect(&fixture.a_config);
    let catalog = client.rpc("tools/list", json!({}));
    for tool in catalog["tools"].as_array().unwrap() {
        assert_eq!(
            tool["inputSchema"]["$schema"], "https://json-schema.org/draft/2020-12/schema",
            "{} advertises a rejected legacy dialect",
            tool["name"]
        );
    }
}
#[test]
fn scoped_clients_over_stdio_keep_persona_and_private_records_separate() {
    let f = Fixture::new();
    let mut owner = Client::connect(&f.a_config);
    let shown = owner.call(
        "mother_write_record",
        f.request(memory("Shared synthetic tea")),
    );
    let hidden = owner.call(
        "mother_write_record",
        f.request(memory("Private synthetic tea")),
    );
    cli(
        f.dir.path(),
        &[
            "grant",
            "companion",
            "--access",
            "companion",
            "--subject",
            "agent-a",
        ],
    );
    cli(
        f.dir.path(),
        &[
            "grant",
            "shared",
            "--access",
            "shared",
            "--subject",
            "agent-b",
            "--records",
            shown["record"]["id"].as_str().unwrap(),
            "--scopes",
            "read,context:propose",
        ],
    );
    let mut companion = Client::connect(&f.dir.path().join("clients/companion.private.json"));
    let mut shared = Client::connect(&f.dir.path().join("clients/shared.private.json"));
    let c = companion.call("mother_get_context", json!({}));
    assert_eq!(c["inventory"]["totalRecords"], 2);
    assert!(c.get("persona").is_some());
    assert_eq!(c["access"]["subject"], "agent-a");
    let c = shared.call("mother_get_context", json!({}));
    assert_eq!(c["inventory"]["totalRecords"], 1);
    assert!(c.get("persona").is_none());
    assert_eq!(c["access"]["subject"], "agent-b");
    assert_eq!(c["selection"]["scope"], "granted_records");
    assert!(
        !c.to_string()
            .contains(hidden["record"]["id"].as_str().unwrap())
    );
    assert_eq!(
        shared.result("mother_get_record", json!({"id":hidden["record"]["id"]}))["isError"],
        true
    );
    assert_eq!(
        shared.result("mother_get_context", json!({"subject":"agent-a"}))["isError"],
        true
    );
    for mode in ["literal", "hybrid", "semantic"] {
        let v = shared.call("mother_search_context", json!({"query":"tea","mode":mode}));
        assert_eq!(v["search"]["eligibleRecords"], 1);
        assert!(v.get("persona").is_none());
        assert!(
            !v.to_string()
                .contains(hidden["record"]["id"].as_str().unwrap())
        );
    }
    assert_eq!(
        shared.result("mother_write_record", f.request(memory("Forbidden write")))["isError"],
        true
    );
    cli(f.dir.path(), &["revoke", "shared"]);
    assert_eq!(
        shared.result("mother_get_context", json!({}))["isError"],
        true
    );
    assert_eq!(
        companion.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        2
    );
}
#[test]
fn version_two_mcp_rejects_missing_policy_at_startup_and_after_connection() {
    let f = Fixture::new();
    cli(
        f.dir.path(),
        &[
            "grant",
            "scoped-v2",
            "--access",
            "shared",
            "--subject",
            "agent-b",
        ],
    );
    let config = f.dir.path().join("clients/scoped-v2.private.json");
    let raw: Value = serde_json::from_slice(&std::fs::read(&config).unwrap()).unwrap();
    assert_eq!(raw["version"], 2);
    let mut c = Client::connect(&config);
    assert_eq!(
        c.call("mother_get_context", json!({}))["access"]["mode"],
        "shared"
    );
    let db = rusqlite::Connection::open(raw["databasePath"].as_str().unwrap()).unwrap();
    db.execute("DELETE FROM client_access WHERE client_id='scoped-v2'", [])
        .unwrap();
    let response = c.result("mother_get_context", json!({}));
    assert_eq!(response["isError"], true);
    assert_eq!(response["structuredContent"]["error"]["code"], "FORBIDDEN");
    assert!(!response.to_string().contains("Synthetic MCP persona"));
    let output = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["mcp", "--config", config.to_str().unwrap()])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("FORBIDDEN"));
}
#[test]
fn a_second_harness_reads_what_a_record_said_before_it_was_corrected() {
    let f = Fixture::new();
    let mut a = Client::connect(&f.a_config);
    let mut b = Client::connect(&f.b_config);
    let first = a.call("mother_write_record", f.request(memory("Green tea")))["record"].clone();
    b.call(
        "mother_write_record",
        f.update(memory("Oolong"), &first["id"], 1),
    );
    let history = a.call("mother_get_record_history", json!({"id":first["id"]}));
    assert_eq!(history["currentRevision"], 2);
    assert_eq!(history["revisions"][0]["actor"], "harness-b");
    assert_eq!(history["revisions"][0]["changed"], json!(["content"]));
    assert_eq!(history["revisions"][1]["excerpt"], "Green tea");
    let old = a.call(
        "mother_get_record_history",
        json!({"id":first["id"],"revision":1}),
    );
    assert_eq!(old["record"], first);
    assert_eq!(old["current"], false);
    let denied = a.result(
        "mother_get_record_history",
        json!({"id":first["id"],"limit":"all"}),
    );
    assert_eq!(denied["isError"], true);
    assert_eq!(
        denied["structuredContent"]["error"]["code"],
        "INVALID_INPUT"
    );
}
#[test]
fn a_memory_pinned_in_one_harness_is_in_every_harness_first_read() {
    let f = Fixture::new();
    let mut a = Client::connect(&f.a_config);
    let mut b = Client::connect(&f.b_config);
    let name = a.call(
        "mother_write_record",
        f.request(json!({"kind":"memory","title":"Preferred name","content":"Call me Leo","basis":"owner_statement","status":"active"})),
    )["record"]
        .clone();
    for n in 0..25 {
        a.call(
            "mother_write_record",
            f.request(memory(&format!("Note {n}"))),
        );
    }
    let pinned = a.call("mother_pin_record", json!({"id":name["id"],"pinned":true}));
    assert_eq!(pinned["pinned"], true);
    let context = b.call("mother_get_context", json!({"limit":3}));
    assert_eq!(context["pinned"][0]["content"], "Call me Leo");
    assert_eq!(context["pinned"][0]["pinnedBy"], "harness-a");
    let bad = b.result("mother_pin_record", json!({"id":name["id"]}));
    assert_eq!(bad["isError"], true);
    assert_eq!(bad["structuredContent"]["error"]["code"], "INVALID_INPUT");
}
