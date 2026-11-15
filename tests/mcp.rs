use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};
use tempfile::TempDir;

#[path = "mcp/capabilities.rs"]
mod capabilities;

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
        assert_eq!(initialized["serverInfo"]["name"], "mother-garden");
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
    database: PathBuf,
    a_config: PathBuf,
}
impl Fixture {
    fn new(soul: &str) -> Self {
        use mother::{access::ClientAccess, garden::store::GardenStore};
        let dir = TempDir::new().unwrap();
        let database = dir.path().join("garden.redb");
        let store = GardenStore::initialize(
            &database,
            &uuid::Uuid::new_v4().to_string(),
            &uuid::Uuid::new_v4().to_string(),
        )
        .unwrap();
        store.v1_initialize(soul).unwrap();
        let auth = store
            .v1_grant(
                "harness-a",
                &["read".into(), "memory:write".into()],
                &ClientAccess::default(),
                None,
            )
            .unwrap();
        let a_config = dir.path().join("a.private.json");
        std::fs::write(&a_config,serde_json::to_vec(&json!({"version":2,"backend":"garden_redb","databasePath":database,"clientId":auth.client_id,"token":auth.token})).unwrap()).unwrap();
        Self {
            dir,
            database,
            a_config,
        }
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
#[path = "mcp/additive_anima.rs"]
mod additive_anima;
