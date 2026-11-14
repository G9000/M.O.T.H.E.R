//! Synthetic graph-only stdio host acceptance; no installed harness is touched.
use mother::{
    access::ClientAccess,
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
            .args(["mcp", "--config"])
            .arg(path)
            .env("MOTHER_HOME", path.parent().unwrap())
            .env(
                "MOTHER_MODEL_DIR",
                path.parent().unwrap().join("missing-model"),
            )
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

fn configured(
    dir: &Path,
    initialize_v1: bool,
    graph: bool,
) -> (std::path::PathBuf, String, String) {
    let workspace = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let database = dir.join("native.redb");
    let store = GardenStore::initialize(&database, &workspace, &generation).unwrap();
    let auth = if initialize_v1 {
        store.v1_initialize("Synthetic owner persona").unwrap();
        let mut scopes = vec!["read".into(), "memory:write".into()];
        if graph {
            scopes.extend(["garden:read".into(), "garden:write".into()]);
        }
        store
            .v1_grant(
                "native-client",
                &scopes,
                &ClientAccess::default(),
                Some(&GardenAccess::All {}),
            )
            .unwrap()
    } else {
        store
            .grant(
                "native-client",
                &["garden:read".into(), "garden:write".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap()
    };
    let config = dir.join("native.private.json");
    std::fs::write(&config,serde_json::to_vec(&json!({"version":2,"backend":"garden_redb","databasePath":database,"clientId":auth.client_id,"token":auth.token})).unwrap()).unwrap();
    (config, workspace, generation)
}
#[test]
fn ordinary_mcp_selects_native_v1_and_preserves_legacy_schemas() {
    let dir = tempfile::tempdir().unwrap();
    let (config, workspace, generation) = configured(dir.path(), true, false);
    let mut client = Client::connect(&config);
    let tools = client.rpc("tools/list", json!({}))["result"]["tools"]
        .as_array()
        .unwrap()
        .clone();
    assert_eq!(tools.len(), 19);
    let legacy: Vec<Value> =
        serde_json::from_str(include_str!("../contracts/mcp-tools.json")).unwrap();
    for mut expected in legacy {
        expected["inputSchema"]["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
        let actual = tools
            .iter()
            .find(|tool| tool["name"] == expected["name"])
            .unwrap();
        assert_eq!(actual["inputSchema"], expected["inputSchema"]);
    }
    let cap = client.call("mother_get_capabilities", json!({}));
    assert_eq!(cap["v1"]["available"], true);
    assert_eq!(cap["workspace"]["workspaceId"], workspace);
    let write = json!({"workspaceId":workspace,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Synthetic v1","content":"Native legacy content","status":"active","basis":"owner_statement"}});
    let receipt = client.call("mother_write_record", write);
    assert_eq!(
        client.call("mother_get_record", json!({"id":receipt["record"]["id"]}))["title"],
        "Synthetic v1"
    );
    let graph:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"entity","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:item","displayName":"private graph","aliases":[],"extensions":{}}}]})).unwrap();
    assert_eq!(
        client.result("mother_garden_apply", serde_json::to_value(graph).unwrap())["structuredContent"]
            ["error"]["code"],
        "FORBIDDEN"
    );
}
#[test]
fn native_v1_and_graph_authority_remain_separate_views() {
    let dir = tempfile::tempdir().unwrap();
    let (config, workspace, generation) = configured(dir.path(), true, true);
    let mut client = Client::connect(&config);
    let batch = json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"entity","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:item","displayName":"graph only","aliases":[],"extensions":{}}}]});
    let applied = client.call("mother_garden_apply", batch);
    let id = applied["receipt"]["results"][0]["reference"]["id"].clone();
    assert_eq!(
        client.result("mother_get_record", json!({"id":id}))["structuredContent"]["error"]["code"],
        "NOT_FOUND"
    );
    assert_eq!(
        client.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        0
    );
}
#[test]
fn explicit_graph_store_without_v1_retains_eight_tools_on_ordinary_entrypoint() {
    let dir = tempfile::tempdir().unwrap();
    let (config, _, _) = configured(dir.path(), false, true);
    let mut client = Client::connect(&config);
    assert_eq!(
        client.rpc("tools/list", json!({}))["result"]["tools"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    assert_eq!(
        client.call("mother_get_capabilities", json!({}))["v1"]["available"],
        false
    );
}
#[test]
fn unknown_backend_is_rejected_without_creating_a_database() {
    let dir = tempfile::tempdir().unwrap();
    let database = dir.path().join("absent.redb");
    let config = dir.path().join("unknown.private.json");
    std::fs::write(&config,serde_json::to_vec(&json!({"version":2,"backend":"unknown_engine","databasePath":database,"clientId":"synthetic","token":"ab".repeat(32)})).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["mcp", "--config"])
        .arg(config)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!database.exists());
}

#[test]
fn hosted_native_search_has_truthful_local_model_fallback() {
    let dir = tempfile::tempdir().unwrap();
    let (config, workspace, generation) = configured(dir.path(), true, false);
    let mut client = Client::connect(&config);
    client.call("mother_write_record",json!({"workspaceId":workspace,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Synthetic tea","content":"Green tea","status":"active","basis":"owner_statement"}}));
    let result = client.call(
        "mother_search_context",
        json!({"query":"tea","mode":"semantic"}),
    );
    assert_eq!(result["search"]["requestedMode"], "semantic");
    assert_eq!(result["search"]["effectiveMode"], "literal");
    assert_eq!(result["search"]["semanticUsed"], false);
    assert_eq!(result["records"].as_array().unwrap().len(), 1);
    assert!(!dir.path().join("semantic.sqlite").exists());
}
