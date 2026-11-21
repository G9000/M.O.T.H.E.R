//! Synthetic stdio host check of the scoped media tools; no installed harness is touched.
//! Protocol tests show what the server sends, not that a real host renders it.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::BatchRequest,
        media_tools::{AcquisitionInput, AttachRequest, AttachSource, IngestOptions, MediaOrigin},
        store::GardenStore,
    },
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

struct Client {
    child: Child,
    input: ChildStdin,
    responses: mpsc::Receiver<Value>,
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
        thread::spawn(move || {
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
            next: 0,
        };
        client.rpc(
            "initialize",
            json!({"protocolVersion":"2025-06-18","capabilities":{},
            "clientInfo":{"name":"synthetic-media","version":"1"}}),
        );
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
                .expect("bounded media MCP reply");
            if value["id"] == id {
                return value;
            }
        }
    }
    fn tool_names(&mut self) -> Vec<String> {
        discovery::collect(|params| self.rpc("tools/list", params))
            .iter()
            .map(|t| t["name"].as_str().unwrap().to_owned())
            .collect()
    }
    fn result(&mut self, name: &str, args: Value) -> Value {
        self.rpc("tools/call", json!({"name":name,"arguments":args}))["result"].clone()
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn png() -> Vec<u8> {
    let mut b = b"\x89PNG\r\n\x1a\n".to_vec();
    for (kind, data) in [
        (&b"IHDR"[..], &[0, 0, 0, 3, 0, 0, 0, 1, 8, 2, 0, 0, 0][..]),
        (b"IDAT", &[1, 2, 3, 4]),
        (b"IEND", &[]),
    ] {
        b.extend_from_slice(&(data.len() as u32).to_be_bytes());
        b.extend_from_slice(kind);
        b.extend_from_slice(data);
        b.extend_from_slice(&[0; 4]);
    }
    b
}

fn config(dir: &Path, database: &Path, auth: &Auth) -> std::path::PathBuf {
    let path = dir.join(format!("{}.private.json", auth.client_id));
    std::fs::write(
        &path,
        serde_json::to_vec(&json!({"version":2,"backend":"garden_redb",
        "databasePath":database,"clientId":auth.client_id,"token":auth.token}))
        .unwrap(),
    )
    .unwrap();
    path
}

#[test]
fn media_tools_are_advertised_by_scope_and_return_image_content() {
    let dir = tempfile::tempdir().unwrap();
    let database = dir.path().join("garden.redb");
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(&database, &ws, &generation).unwrap();
    let companion = |client: &str, scopes: &[&str]| {
        store
            .grant(
                client,
                &scopes.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
                &ClientAccess {
                    mode: AccessMode::Companion,
                    subject: Some(format!("agent:{client}")),
                    records: vec![],
                },
                &GardenAccess::All {},
            )
            .unwrap()
    };
    let writer = companion("writer", &["garden:read", "garden:write"]);
    let viewer = companion("viewer", &["garden:read", "media:meta", "media:bytes"]);
    let batch: BatchRequest = serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,
        "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
        {"itemKey":"p","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:place","displayName":"Synthetic Harbour","aliases":[],"extensions":{}}},
        {"itemKey":"m","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":"p"},"predicate":"test:seen","object":{"type":"value","value":{"type":"boolean","value":true}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[],"confidence":null,"extensions":{}}}]})).unwrap();
    let memory = store.apply(&writer, &batch).unwrap().receipt.results[1]
        .reference
        .id
        .clone();
    let image = dir.path().join("harbour.png");
    std::fs::write(&image, png()).unwrap();
    let attached = store
        .media_attach(&AttachRequest {
            source: AttachSource::File {
                path: &image,
                input: AcquisitionInput::new(MediaOrigin::Captured),
                options: IngestOptions::default(),
            },
            target: memory,
            expected_revision: 1,
            note: None,
        })
        .unwrap();
    let sha256 = attached["ingest"]["sha256"].as_str().unwrap().to_owned();
    let writer_config = config(dir.path(), &database, &writer);
    let viewer_config = config(dir.path(), &database, &viewer);
    drop(store);

    let mut plain = Client::connect(&writer_config);
    let names = plain.tool_names();
    assert!(!names.iter().any(|n| n.starts_with("mother_garden_media")));
    let capabilities =
        plain.result("mother_get_capabilities", json!({}))["structuredContent"].clone();
    assert!(
        capabilities["garden"]["definedTools"]
            .as_array()
            .unwrap()
            .contains(&json!("mother_garden_media_get"))
    );
    assert!(
        !capabilities["garden"]["availableTools"]
            .as_array()
            .unwrap()
            .contains(&json!("mother_garden_media_get"))
    );
    drop(plain);

    let mut client = Client::connect(&viewer_config);
    let names = client.tool_names();
    assert!(names.contains(&"mother_garden_media_list".to_owned()));
    assert!(names.contains(&"mother_garden_media_get".to_owned()));
    let envelope = json!({"version":2,"workspaceId":ws,"generation":generation});
    let listed = client.result("mother_garden_media_list", envelope.clone());
    assert_ne!(listed["isError"], true, "{listed}");
    assert_eq!(
        listed["structuredContent"]["assets"][0]["sha256"],
        sha256.as_str()
    );

    let mut request = envelope;
    request["sha256"] = json!(sha256);
    let got = client.result("mother_garden_media_get", request);
    assert_ne!(got["isError"], true, "{got}");
    assert_eq!(got["content"][0]["type"], "image");
    assert_eq!(got["content"][0]["mimeType"], "image/png");
    assert_eq!(got["content"][0]["data"], mother::media::base64(&png()));
    assert!(got["structuredContent"].get("data").is_none());
    assert_eq!(got["structuredContent"]["variant"], "original");
}
