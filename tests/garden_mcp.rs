//! Synthetic graph-only stdio host acceptance; no installed harness is touched.
use mother::{
    access::{AccessMode, ClientAccess},
    garden::{access::GardenAccess, contracts::BatchRequest, store::GardenStore},
};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

#[path = "support/mcp_discovery.rs"]
mod discovery;
#[path = "mcp/garden_queries.rs"]
mod queries;

struct Client {
    child: Child,
    input: ChildStdin,
    responses: mpsc::Receiver<Value>,
    reader: Option<thread::JoinHandle<()>>,
    next: u64,
}
impl Client {
    fn connect(path: &Path) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(["garden", "mcp", "--config"])
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
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
            input,
            responses,
            reader: Some(reader),
            next: 0,
        };
        let result=client.rpc("initialize",json!({"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"synthetic-garden","version":"1"}}));
        assert_eq!(result["result"]["serverInfo"]["name"], "mother-garden");
        client.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
        client
    }
    fn send(&mut self, value: Value) {
        writeln!(self.input, "{value}").unwrap();
        self.input.flush().unwrap();
    }
    fn rpc(&mut self, method: &str, params: Value) -> Value {
        self.next += 1;
        let id = self.next;
        self.send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}));
        loop {
            let value = self
                .responses
                .recv_timeout(Duration::from_secs(15))
                .expect("bounded graph MCP reply");
            if value["id"] == id {
                return value;
            }
        }
    }
    fn result(&mut self, name: &str, args: Value) -> Value {
        self.rpc("tools/call", json!({"name":name,"arguments":args}))["result"].clone()
    }
    fn tools(&mut self) -> Vec<Value> {
        discovery::collect(|params| self.rpc("tools/list", params))
    }
    fn call(&mut self, name: &str, args: Value) -> Value {
        let result = self.result(name, args);
        assert_ne!(result["isError"], true, "{result}");
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

#[test]
fn graph_stdio_proposals_owner_approval_receipts_and_live_revocation() {
    let dir = tempfile::tempdir().unwrap();
    let database = dir.path().join("garden.redb");
    let batch: BatchRequest =
        serde_json::from_str(include_str!("fixtures/garden-v2.json")).unwrap();
    let store = GardenStore::initialize(&database, &batch.workspace_id, &batch.generation).unwrap();
    let auth = store
        .grant(
            "synthetic-proposer",
            &["garden:read".into(), "garden:propose".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("synthetic-agent".into()),
                records: vec![],
            },
            &GardenAccess::default(),
        )
        .unwrap();
    let config = dir.path().join("graph.private.json");
    std::fs::write(&config,serde_json::to_vec(&json!({"version":2,"backend":"garden_redb","databasePath":database,"clientId":auth.client_id,"token":auth.token})).unwrap()).unwrap();
    drop(store);
    let mut client = Client::connect(&config);
    let tools = client.tools();
    let names: Vec<_> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names.len(), 8);
    assert!(names.contains(&"mother_garden_propose"));
    assert!(names.contains(&"mother_garden_query"));
    assert!(names.contains(&"mother_garden_changes"));
    assert!(!names.contains(&"mother_garden_approve"));
    let capability = client.call("mother_get_capabilities", json!({}));
    assert_eq!(capability["garden"]["available"], true);
    assert_eq!(capability["v1"]["available"], false);
    let proposed = client.call(
        "mother_garden_propose",
        serde_json::to_value(&batch).unwrap(),
    );
    assert_eq!(proposed["status"], "pending");
    let lookup = json!({"version":2,"workspaceId":batch.workspace_id,"generation":batch.generation,"requiredExtensions":{},"operationId":batch.operation_id,"operationGeneration":batch.generation});
    assert_eq!(
        client.call("mother_garden_get_proposal", lookup.clone())["receipt"]["status"],
        "pending"
    );
    let store = GardenStore::open(&database).unwrap();
    store.review_proposal(json!({"version":2,"workspaceId":batch.workspace_id,"generation":batch.generation,
        "operationId":uuid::Uuid::new_v4().to_string(),"proposalId":proposed["proposalId"],"expectedRevision":1,"action":"approve"})).unwrap();
    drop(store);
    let approved = client.call("mother_garden_get_proposal", lookup.clone());
    assert_eq!(approved["receipt"]["status"], "approved");
    assert_eq!(approved["receipt"]["results"].as_array().unwrap().len(), 3);
    assert_eq!(approved["visibleRecords"], json!([]));
    assert_eq!(approved["payloadOmitted"], true);
    assert_eq!(
        client.call("mother_garden_get_receipt", lookup)["receipt"]["status"],
        "pending"
    );
    let get = json!({"version":2,"workspaceId":batch.workspace_id,"generation":batch.generation,"requiredExtensions":{},"reference":approved["receipt"]["results"][0]["reference"],"revision":null,"knownAt":null});
    assert_eq!(
        client.result("mother_garden_get", get)["structuredContent"]["error"]["code"],
        "NOT_FOUND"
    );
    assert_eq!(
        client.result("mother_garden_apply", serde_json::to_value(&batch).unwrap())["structuredContent"]
            ["error"]["code"],
        "FORBIDDEN"
    );
    assert!(
        client
            .rpc(
                "tools/call",
                json!({"name":"mother_garden_approve","arguments":{}})
            )
            .get("error")
            .is_some()
    );
    let store = GardenStore::open(&database).unwrap();
    store.revoke("synthetic-proposer").unwrap();
    drop(store);
    assert_eq!(
        client.result("mother_get_capabilities", json!({}))["structuredContent"]["error"]["code"],
        "UNAUTHORIZED"
    );
}
