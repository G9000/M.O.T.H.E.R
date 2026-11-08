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
        let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
            .arg("mcp")
            .arg("--config")
            .arg(config)
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
        let initialized = cli(dir.path(), &["init"]);
        for client in ["harness-a", "harness-b"] {
            cli(
                dir.path(),
                &[
                    "grant",
                    client,
                    "--scopes",
                    "read,memory:write,task:write,checkpoint:write",
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
    // The six tools remain synchronous and advertise no task capability.
    for tool in expected.as_array_mut().unwrap() {
        assert_eq!(tool["execution"]["taskSupport"], "forbidden");
        tool.as_object_mut().unwrap().remove("execution");
    }
    assert_eq!(
        catalog["tools"], expected,
        "preserve all six tool names, descriptions, schemas and annotations"
    );
    let first = a.call("mother_write_record", f.request(memory("Green tea")));
    let task = a.call("mother_write_record", f.request(json!({"kind":"task", "title":"Plan a weekend", "content":"Choose a destination", "status":"todo"})));
    let checkpoint = a.call("mother_checkpoint", f.request(json!({"kind":"checkpoint", "title":"Weekend handoff", "content":"Destination still open", "status":"active", "nextStep":"Choose a destination with the owner", "references":[first["record"]["id"], task["record"]["id"]]})));
    let context = b.call("mother_get_context", json!({}));
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
}

#[test]
fn malformed_tool_arguments_fail_without_mutating_records() {
    let f = Fixture::new();
    let mut client = Client::connect(&f.a_config);
    for (name, arguments) in [
        ("mother_search_context", json!({})),
        ("mother_search_context", json!({"query":"   "})),
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
