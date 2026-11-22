//! The read-only memory garden explorer: owner snapshot and loopback serving, on synthetic data.
use mother::{
    access::ClientAccess,
    contracts::Auth,
    garden::store::GardenStore,
    service::{LocalClient, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::Path,
    process::{Child, Command, Stdio},
};
use uuid::Uuid;

struct Garden {
    store: GardenStore,
    auth: Auth,
    workspace: String,
    generation: String,
}
impl Garden {
    fn new(path: &Path) -> Self {
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(path, &workspace, &generation).unwrap();
        store.v1_initialize("Synthetic owner persona.").unwrap();
        let scopes = [
            "read",
            "memory:write",
            "decision:write",
            "task:write",
            "checkpoint:write",
            "artifact:write",
            "action:write",
        ]
        .map(String::from);
        let auth = store
            .v1_grant("synthetic-harness", &scopes, &ClientAccess::default(), None)
            .unwrap();
        Self {
            store,
            auth,
            workspace,
            generation,
        }
    }
    fn write(&self, id: Option<(&str, u64)>, record: Value) -> Value {
        let mut request = json!({"workspaceId":self.workspace,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record});
        if let Some((id, revision)) = id {
            request["id"] = json!(id);
            request["expectedRevision"] = json!(revision);
        }
        self.store
            .v1_call(&self.auth, "mother_write_record", request)
            .unwrap()["record"]
            .clone()
    }
    fn call(&self, tool: &str, args: Value) {
        self.store.v1_call(&self.auth, tool, args).unwrap();
    }
}
fn memory(title: &str, content: &str) -> Value {
    json!({"kind":"memory","title":title,"content":content,"status":"active","basis":"owner_statement"})
}
fn id(record: &Value) -> &str {
    record["id"].as_str().unwrap()
}
fn find<'a>(view: &'a Value, id: &str) -> &'a Value {
    view["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == id)
        .unwrap()
}

#[test]
fn snapshot_carries_pins_handoffs_links_and_history_without_secrets() {
    let dir = tempfile::tempdir().unwrap();
    let garden = Garden::new(&dir.path().join("garden.redb"));
    let tea = garden.write(None, memory("Synthetic tea", "Green tea."));
    let tea = garden.write(Some((id(&tea), 1)), memory("Synthetic tea", "Oolong tea."));
    let task = garden.write(
        None,
        json!({"kind":"task","title":"Synthetic errand","content":"Water the test bed.","status":"doing","references":[id(&tea)]}),
    );
    let note = garden.write(
        None,
        json!({"kind":"checkpoint","title":"Synthetic handoff","content":"Bed watered.","status":"active","nextStep":"Check tomorrow."}),
    );
    garden.call("mother_pin_record", json!({"id":id(&tea),"pinned":true}));
    garden.call(
        "mother_assign_task",
        json!({"id":id(&task),"assignee":"synthetic-harness"}),
    );

    let view = garden.store.v1_garden_view(500).unwrap();
    assert_eq!(
        (view["total"].clone(), view["shown"].clone()),
        (json!(3), json!(3))
    );
    assert_eq!(view["workspaceId"], json!(garden.workspace));
    assert_eq!(view["records"][0]["id"], tea["id"], "pinned records lead");

    let shown = find(&view, id(&tea));
    assert_eq!(shown["content"], "Oolong tea.");
    assert_eq!(shown["pinned"]["actor"], "synthetic-harness");
    let history = shown["history"].as_array().unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["changed"], Value::Null);
    assert_eq!(history[0]["excerpt"], "Green tea.");
    assert_eq!(history[1]["changed"], json!(["content"]));
    assert_eq!(history[1]["actor"], "synthetic-harness");

    let shown = find(&view, id(&task));
    assert_eq!(shown["references"], json!([id(&tea)]));
    assert_eq!(shown["assignment"]["assignee"], "synthetic-harness");
    assert_eq!(find(&view, id(&note))["nextStep"], "Check tomorrow.");

    let text = view.to_string();
    assert!(!text.contains(&garden.auth.token));
    assert!(!text.contains("Synthetic owner persona"));
}

#[test]
fn snapshot_is_bounded_and_requires_the_enabled_view() {
    let dir = tempfile::tempdir().unwrap();
    let garden = Garden::new(&dir.path().join("garden.redb"));
    let first = garden.write(None, memory("Synthetic first", "Oldest."));
    garden.call("mother_pin_record", json!({"id":id(&first),"pinned":true}));
    for n in 0..4 {
        garden.write(
            None,
            memory(&format!("Synthetic {n}"), &"long ".repeat(1_000)),
        );
    }
    let view = garden.store.v1_garden_view(2).unwrap();
    assert_eq!(
        (view["total"].clone(), view["shown"].clone()),
        (json!(5), json!(2))
    );
    assert_eq!(view["records"][0]["id"], first["id"]);
    let long = &view["records"][1];
    assert_eq!(long["truncated"], true);
    assert_eq!(long["content"].as_str().unwrap().len(), 4_000);
    for limit in [0, 2_001] {
        assert_eq!(
            garden.store.v1_garden_view(limit).unwrap_err().code(),
            "INVALID_INPUT"
        );
    }

    let bare = GardenStore::initialize(
        dir.path().join("bare.redb"),
        &Uuid::new_v4().to_string(),
        &Uuid::new_v4().to_string(),
    )
    .unwrap();
    assert_eq!(
        bare.v1_garden_view(10).unwrap_err().code(),
        "COMPAT_NOT_INITIALIZED"
    );
}

struct Running(Child);
impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn http(port: u16, request: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    let mut reply = String::new();
    stream.read_to_string(&mut reply).unwrap();
    let status = reply[9..12].parse().unwrap();
    (status, reply)
}

#[test]
fn command_serves_one_snapshot_on_loopback_and_releases_the_database() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("garden.redb");
    let garden = Garden::new(&db);
    let tea = garden.write(None, memory("Synthetic tea", "Green tea."));
    drop(garden);

    let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", "explore", "--db"])
        .arg(&db)
        .env("MOTHER_HOME", dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let _running = Running(child);
    let started: Value = serde_json::from_str(&line).unwrap();
    assert_eq!(
        (started["shown"].clone(), started["readOnly"].clone()),
        (json!(1), json!(true))
    );
    let url = started["url"].as_str().unwrap();
    let (address, launch) = url.split_once("/#").unwrap();
    let authority = address.strip_prefix("http://").unwrap();
    assert!(authority.starts_with("127.0.0.1:"));
    let port: u16 = authority.rsplit(':').next().unwrap().parse().unwrap();
    let get = |path: &str, extra: &str| {
        http(
            port,
            &format!("GET {path} HTTP/1.1\r\nHost: {authority}\r\n{extra}\r\n"),
        )
    };

    // The snapshot was taken at start: the database is free again while the garden is open.
    GardenStore::open_active(&db).unwrap();

    let (status, page) = get("/", "");
    assert_eq!(status, 200);
    assert!(page.contains("Content-Security-Policy: default-src 'none'"));
    assert!(page.contains("Cache-Control: no-store"));
    assert!(
        !page.contains("https://"),
        "the page must not load anything from outside"
    );
    assert_eq!(get("/assets/index.js", "").0, 200);
    assert_eq!(get("/api/garden", "").0, 401);
    assert_eq!(
        get(
            "/api/garden",
            &format!("Authorization: Bearer {launch}\r\n")
        )
        .0,
        401
    );

    let post = |origin: &str| {
        http(
            port,
            &format!(
                "POST /api/session HTTP/1.1\r\nHost: {authority}\r\nOrigin: {origin}\r\nAuthorization: Bearer {launch}\r\nContent-Length: 0\r\n\r\n"
            ),
        )
    };
    assert_eq!(post("http://example.test").0, 403);
    let (status, reply) = post(address);
    assert_eq!(status, 200);
    let body: Value = serde_json::from_str(reply.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    let session = body["session"].as_str().unwrap();
    assert_eq!(post(address).0, 401, "the link works once");

    let (status, reply) = get(
        "/api/garden",
        &format!("Authorization: Bearer {session}\r\n"),
    );
    assert_eq!(status, 200);
    let view: Value = serde_json::from_str(reply.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(view["records"][0]["id"], tea["id"]);
    assert_eq!(
        http(
            port,
            &format!("GET /api/garden HTTP/1.1\r\nHost: other.example.test\r\nAuthorization: Bearer {session}\r\n\r\n")
        )
        .0,
        403
    );
}

#[test]
fn live_explorer_observes_writes_through_the_same_running_owner() {
    let parent = std::fs::canonicalize(std::env::temp_dir()).unwrap();
    let dir = tempfile::tempdir_in(parent).unwrap();
    // The owner service writes its endpoint only into an owner-only directory.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let db = dir.path().join("garden.redb");
    let endpoint = dir.path().join("service.json");
    let garden = Garden::new(&db);
    let config = ServiceClientConfig {
        version: 3,
        backend: "garden_service".into(),
        endpoint_path: endpoint.clone(),
        workspace_id: garden.workspace,
        generation: garden.generation,
        client_id: garden.auth.client_id,
        token: garden.auth.token,
    };
    drop(garden.store);

    let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", "service", "--db"])
        .arg(&db)
        .arg("--endpoint")
        .arg(&endpoint)
        .arg("--explore")
        .env("MOTHER_HOME", dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let _running = Running(child);
    let mut lines = (&mut stdout).lines();
    let owner: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
    let launch: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
    assert_eq!(launch["live"], true);
    let reopen = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", "explore", "--endpoint"])
        .arg(&endpoint)
        .output()
        .unwrap();
    assert!(
        reopen.status.success(),
        "{}",
        String::from_utf8_lossy(&reopen.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&reopen.stdout).unwrap(),
        launch
    );
    assert!(GardenStore::open_active(&db).is_err());
    let (address, token) = launch["url"].as_str().unwrap().split_once("/#").unwrap();
    let authority = address.strip_prefix("http://").unwrap();
    let port = authority.rsplit(':').next().unwrap().parse().unwrap();

    let get = |path: &str, token: &str| {
        http(
            port,
            &format!(
                "GET {path} HTTP/1.1\r\nHost: {authority}\r\nAuthorization: Bearer {token}\r\n\r\n"
            ),
        )
    };
    let parse = |(status, response): (u16, String)| -> Value {
        assert_eq!(status, 200);
        serde_json::from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap()
    };
    assert_eq!(get("/api/garden", token).0, 401);
    let exchange = parse(http(
        port,
        &format!(
            "POST /api/session HTTP/1.1\r\nHost: {authority}\r\nOrigin: {address}\r\nAuthorization: Bearer {token}\r\nContent-Length: 0\r\n\r\n"
        ),
    ));
    let session = exchange["session"].as_str().unwrap();
    let initial = parse(get("/api/garden", session));
    let revision = initial["liveRevision"].as_str().unwrap();
    let poll = format!("/api/garden?since={revision}");
    assert_eq!(initial["shown"], 0);
    assert_eq!(parse(get(&poll, session)), json!({"unchanged":true}));

    let client = LocalClient::new(config).unwrap();
    let written = client
        .call(
            "mother_write_record",
            &json!({
                "workspaceId":owner["workspaceId"],"generation":owner["generation"],
                "operationId":Uuid::new_v4().to_string(),"expectedRevision":0,
                "record":memory("Live synthetic memory", "Written while the view is open.")
            }),
        )
        .unwrap();
    let after = parse(get(&poll, session));
    assert_eq!(after["records"][0]["id"], written["record"]["id"]);
    assert_ne!(after["liveRevision"], initial["liveRevision"]);
    let written_id = written["record"]["id"].as_str().unwrap();
    let reading = parse(get(&format!("/api/garden/records/{written_id}"), session));
    assert_eq!(reading["content"], "Written while the view is open.");
    assert_eq!(reading["account"]["form"], "captured_paraphrase");
    assert_eq!(
        get(&format!("/api/garden/records/{}", Uuid::new_v4()), session).0,
        404
    );
    assert_eq!(
        get(&format!("/api/garden/records/{written_id}"), token).0,
        401
    );
    assert_eq!(after["liveMediaRevision"], initial["liveMediaRevision"]);
    assert_eq!(parse(get("/api/media", session))["images"], json!([]));
    assert_eq!(
        client.info().service_id,
        owner["serviceId"].as_str().unwrap()
    );
}

/// Grow a synthetic garden for looking at the explorer by hand:
/// `MOTHER_EXPLORER_DEMO=<new file> cargo test --locked --test garden_explorer demo -- --ignored`
#[test]
#[ignore = "writes a synthetic demonstration garden to the path in MOTHER_EXPLORER_DEMO"]
fn demo_garden() {
    let path = std::env::var_os("MOTHER_EXPLORER_DEMO").expect("set MOTHER_EXPLORER_DEMO");
    let garden = Garden::new(Path::new(&path));
    let count: usize = std::env::var("MOTHER_EXPLORER_DEMO_RECORDS")
        .ok()
        .and_then(|n| n.parse().ok())
        .unwrap_or(60);
    let mut ids: Vec<String> = Vec::new();
    for n in 0..count {
        let link = if n % 3 == 0 && !ids.is_empty() {
            json!([ids[(n * 7) % ids.len()]])
        } else {
            json!([])
        };
        let title = format!("Synthetic record {n}");
        let content = format!(
            "Invented example text number {n}, written only to grow a demonstration garden."
        );
        let record = match n % 9 {
            0..=2 => {
                json!({"kind":"memory","title":title,"content":content,"status":if n % 27 == 2 {"retracted"} else {"active"},"basis":"owner_statement","references":link})
            }
            3 => {
                json!({"kind":"decision","title":title,"content":content,"status":(["proposed","confirmed","superseded"][n / 9 % 3]),"references":link})
            }
            4 | 5 => {
                json!({"kind":"task","title":title,"content":content,"status":(["todo","doing","blocked","done","cancelled"][n / 9 % 5]),"references":link})
            }
            6 => {
                json!({"kind":"checkpoint","title":title,"content":content,"status":"active","nextStep":"Invented next step.","references":link})
            }
            7 => {
                json!({"kind":"artifact","title":title,"content":content,"status":"active","mediaType":"text/plain","references":link})
            }
            _ => {
                json!({"kind":"action","title":title,"content":content,"status":(["pending","succeeded","failed","unknown"][n / 9 % 4]),"destination":"synthetic destination","references":link})
            }
        };
        let mut saved = garden.write(None, record.clone());
        for revision in 1..=(n % 4) as u64 {
            let mut next = record.clone();
            next["content"] = json!(format!("{content} Revised {revision}."));
            saved = garden.write(Some((id(&saved), revision)), next);
        }
        if n % 9 == 0 && n < 45 {
            garden.call("mother_pin_record", json!({"id":id(&saved),"pinned":true}));
        }
        if n % 9 == 4 && matches!(saved["status"].as_str(), Some("todo" | "doing" | "blocked")) {
            garden.call(
                "mother_assign_task",
                json!({"id":id(&saved),"assignee":"synthetic-harness"}),
            );
        }
        ids.push(id(&saved).to_owned());
    }
}
