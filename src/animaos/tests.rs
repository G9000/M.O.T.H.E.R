use super::*;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

#[derive(Default)]
struct Remote {
    entries: Vec<Value>,
    requests: Vec<(String, String, Value)>,
    headers: Vec<String>,
    drop_after_create: bool,
    deny: bool,
    stale: bool,
    redirect: bool,
    huge: bool,
    review: bool,
    drop_after_enable: bool,
    drop_after_connect: bool,
    wrong_create_owner: bool,
    wrong_update_id: bool,
    unchanged_update_version: bool,
    excessive_agents: bool,
    binding: Value,
    binding_ready: bool,
    drop_after_binding: bool,
    drop_without_binding: bool,
    drop_after_review: bool,
    drop_without_review: bool,
}

struct Server {
    endpoint: String,
    state: Arc<Mutex<Remote>>,
    stop: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
}
impl Server {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let state = Arc::new(Mutex::new(Remote::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let (remote, done) = (state.clone(), stop.clone());
        let worker = thread::spawn(move || {
            while !done.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut socket, _)) => {
                        socket.set_nonblocking(false).unwrap();
                        respond(&mut socket, &remote);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(3))
                    }
                    Err(e) => panic!("synthetic server: {e}"),
                }
            }
        });
        Self {
            endpoint,
            state,
            stop,
            worker: Some(worker),
        }
    }
    fn session(&self) -> Session {
        Session::with_api_key(&self.endpoint, "owner-secret", "api-secret").unwrap()
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let result = self.worker.take().unwrap().join();
        if !thread::panicking() {
            assert!(result.is_ok(), "synthetic HTTP server failed");
        }
    }
}
fn respond(socket: &mut TcpStream, state: &Mutex<Remote>) {
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut bytes = Vec::new();
    let mut buffer = [0; 4096];
    let (header_end, length) = loop {
        let count = socket.read(&mut buffer).unwrap();
        if count == 0 {
            return;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            let text = String::from_utf8_lossy(&bytes[..end]);
            let len = text
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .and_then(|n| n.parse::<usize>().ok())
                })
                .unwrap_or(0);
            break (end + 4, len);
        }
        assert!(bytes.len() < 32 * 1024);
    };
    while bytes.len() < header_end + length {
        let n = socket.read(&mut buffer).unwrap();
        assert_ne!(n, 0);
        bytes.extend_from_slice(&buffer[..n]);
    }
    let header = String::from_utf8_lossy(&bytes[..header_end]).to_string();
    let mut first = header.lines().next().unwrap().split_whitespace();
    let method = first.next().unwrap().to_string();
    let path = first.next().unwrap().to_string();
    let body: Value = if length == 0 {
        Value::Null
    } else {
        serde_json::from_slice(&bytes[header_end..header_end + length]).unwrap()
    };
    let mut state = state.lock().unwrap();
    state
        .requests
        .push((method.clone(), path.clone(), body.clone()));
    state.headers.push(header);
    let (status, reply) = if state.deny {
        (403, json!({"error":"owner-secret api-secret do not echo"}))
    } else if state.huge {
        (200, json!({"padding":"x".repeat(1024*1024+1)}))
    } else if state.redirect {
        let _ = write!(
            socket,
            "HTTP/1.1 302 Found\r\nLocation: http://192.0.2.1/secret\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        );
        return;
    } else if path == "/api/agents?view=summary" {
        let agents = if state.excessive_agents {
            (0..1025)
                .map(|i| json!({"state":{"id":format!("agent-{i}"),"name":"Synthetic"}}))
                .collect::<Vec<_>>()
        } else {
            vec![json!({"state":{"id":"test-agent","name":"Synthetic agent"}})]
        };
        (200, json!({"agents":agents}))
    } else if path.ends_with("/workspace-context") {
        if method == "PUT" {
            assert_eq!(body["expectedRevision"], "synthetic-revision");
            if state.drop_without_binding {
                return;
            }
            state.binding = body["binding"].clone();
            if state.drop_after_binding {
                state.drop_after_binding = false;
                return;
            }
        }
        (
            200,
            json!({"revision":"synthetic-revision","binding":state.binding,"status":if state.binding.is_null(){"independent"}else{"configured"},"connections":state.entries.iter().filter(|e| e["reviewedTools"].as_array().is_some_and(|tools| tools.iter().any(|tool|tool["remoteName"]=="mother_get_context"))).map(|e| json!({"id":e["id"],"label":"MOTHER","state":e["state"],"tool":"mother_get_context"})).collect::<Vec<_>>()}),
        )
    } else if method == "GET" && path.ends_with("mcp-connections") {
        (200, json!({"connections":state.entries}))
    } else if method == "POST" && path.ends_with("mcp-connections") {
        let id = format!("conn-{}", state.entries.len() + 1);
        let entry = json!({"id":id,"agentId":"test-agent","label":body["label"],"transport":"stdio","stdio":body["stdio"],"endpoint":"","enabled":false,"version":1,"toolAccess":"automatic","state":"disconnected","physicalCallsActive":false,"discoveryId":null,"candidates":[],"reviewedTools":[]});
        state.entries.push(entry.clone());
        if state.drop_after_create {
            state.drop_after_create = false;
            return;
        }
        let mut reply = entry;
        if state.wrong_create_owner {
            reply["agentId"] = json!("another-agent");
        }
        (201, json!({"connection":reply}))
    } else if method == "POST" && path.ends_with("/review") {
        let index = state
            .entries
            .iter()
            .position(|entry| {
                path.split('/')
                    .any(|part| Some(part) == entry["id"].as_str())
            })
            .unwrap();
        assert_eq!(body["expectedVersion"], state.entries[index]["version"]);
        assert_eq!(body["discoveryId"], state.entries[index]["discoveryId"]);
        let reader = state.entries[index]["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["remoteName"] == "mother_get_context")
            .unwrap()
            .clone();
        assert_eq!(body["descriptorDigest"], reader["descriptorDigest"]);
        if state.drop_without_review {
            return;
        }
        let entry = &mut state.entries[index];
        entry["reviewedTools"] = json!([reader]);
        entry["version"] = json!(entry["version"].as_u64().unwrap() + 1);
        entry["state"] = json!("ready");
        let reply = entry.clone();
        if state.drop_after_review {
            state.drop_after_review = false;
            return;
        }
        (200, json!({"connection":reply}))
    } else if method == "PATCH" || method == "POST" {
        let index = state
            .entries
            .iter()
            .position(|e| path.split('/').any(|s| Some(s) == e["id"].as_str()))
            .unwrap();
        if state.stale || body["expectedVersion"] != state.entries[index]["version"] {
            (409, json!({"error":"stale"}))
        } else {
            let review = state.review;
            let ready = state.binding_ready;
            let entry = &mut state.entries[index];
            if review {
                entry["toolAccess"] = json!("review");
            }
            if method == "PATCH" {
                entry["enabled"] = json!(true);
                entry["version"] = json!(2);
            } else {
                assert!(path.ends_with("/connect"));
                // A fresh discovery is unreviewed even when the agent's automatic policy permits use.
                entry["state"] = json!(if ready { "ready" } else { "needs_review" });
                entry["discoveryId"] = json!("discovery-1");
                entry["candidates"] = json!([{"remoteName":"mother_get_context","descriptorDigest":"a".repeat(64)},{"remoteName":"mother_search_context"},{"remoteName":"mother_get_record"},{"remoteName":"mother_get_receipt"},{"remoteName":"mother_propose_memory"},{"remoteName":"mother_get_proposal"}]);
            }
            let mut reply = entry.clone();
            if method == "PATCH" {
                if state.drop_after_enable {
                    state.drop_after_enable = false;
                    return;
                }
                if state.wrong_update_id {
                    reply["id"] = json!("another-connection");
                }
                if state.unchanged_update_version {
                    reply["version"] = body["expectedVersion"].clone();
                }
            } else if state.drop_after_connect {
                state.drop_after_connect = false;
                return;
            }
            (200, json!({"connection":reply}))
        }
    } else {
        (404, json!({}))
    };
    let reply = reply.to_string();
    let _ = write!(
        socket,
        "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{reply}",
        reply.len()
    );
}
fn fixture() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    mother::store::Store::initialize(
        &dir.path().join("context.sqlite"),
        "Synthetic owner persona.",
    )
    .unwrap();
    let exe = dir.path().join("mother.exe");
    std::fs::write(&exe, "synthetic executable path only; never launched").unwrap();
    (dir, exe)
}

#[test]
fn rejects_remote_ambiguous_and_credential_urls() {
    for endpoint in [
        "https://example.org",
        "http://127.0.0.2:8080",
        "http://127.1:8080",
        "http://user:pass@127.0.0.1:8080",
        "http://127.0.0.1:8080/api",
        "http://127.0.0.1:8080/?token=secret",
        "http://[::ffff:127.0.0.1]:8080",
    ] {
        assert!(
            Session::new(endpoint, "secret").is_err(),
            "accepted {endpoint}"
        );
    }
}

#[test]
fn uncertain_enable_and_connect_reconcile_without_repeating_effects() {
    for phase in ["enable", "connect"] {
        let server = Server::new();
        {
            let mut remote = server.state.lock().unwrap();
            if phase == "enable" {
                remote.drop_after_enable = true;
            } else {
                remote.drop_after_connect = true;
            }
        }
        let (dir, exe) = fixture();
        let session = server.session();
        assert!(
            session.connect("test-agent", dir.path(), &exe).is_err(),
            "{phase}"
        );
        session.connect("test-agent", dir.path(), &exe).unwrap();
        let remote = server.state.lock().unwrap();
        assert_eq!(
            remote
                .requests
                .iter()
                .filter(|(method, _, _)| method == "PATCH")
                .count(),
            1,
            "{phase}"
        );
        assert_eq!(
            remote
                .requests
                .iter()
                .filter(|(method, path, _)| method == "POST" && path.ends_with("/connect"))
                .count(),
            1,
            "{phase}"
        );
    }
}

#[test]
fn unknown_create_with_no_matching_remote_entry_is_not_replayed() {
    let server = Server::new();
    server.state.lock().unwrap().drop_after_create = true;
    let (dir, exe) = fixture();
    let session = server.session();
    assert!(session.connect("test-agent", dir.path(), &exe).is_err());
    {
        let mut remote = server.state.lock().unwrap();
        remote.entries.clear();
        remote.requests.clear();
    }
    assert!(session.connect("test-agent", dir.path(), &exe).is_err());
    let remote = server.state.lock().unwrap();
    assert_eq!(remote.requests.len(), 1);
    assert_eq!(remote.requests[0].0, "GET");
}

#[test]
fn inconsistent_mutation_responses_remain_unknown_until_reconciled() {
    for phase in ["owner", "id", "version"] {
        let server = Server::new();
        {
            let mut remote = server.state.lock().unwrap();
            match phase {
                "owner" => remote.wrong_create_owner = true,
                "id" => remote.wrong_update_id = true,
                _ => remote.unchanged_update_version = true,
            };
        }
        let (dir, exe) = fixture();
        let error = server
            .session()
            .connect("test-agent", dir.path(), &exe)
            .unwrap_err();
        assert_eq!(error.code(), "ANIMAOS_UNKNOWN", "{phase}: {error}");
        let binding = std::fs::read_dir(dir.path().join("clients"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| path.to_string_lossy().ends_with(".binding.json"))
            .unwrap();
        assert!(
            mother::config::read_json(&binding, 16384).unwrap()["pending"].is_string(),
            "{phase}"
        );
    }
}

#[test]
fn rejects_excessive_agent_lists() {
    let server = Server::new();
    server.state.lock().unwrap().excessive_agents = true;
    assert!(server.session().agents().is_err());
}

#[test]
fn stale_reviewed_tools_cannot_substitute_missing_current_descriptors() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let session = server.session();
    session.connect("test-agent", dir.path(), &exe).unwrap();
    {
        let mut remote = server.state.lock().unwrap();
        let entry = &mut remote.entries[0];
        entry["reviewedTools"] = entry["candidates"].clone();
        entry["candidates"]
            .as_array_mut()
            .unwrap()
            .retain(|tool| tool["remoteName"] != "mother_propose_memory");
    }
    assert!(session.connect("test-agent", dir.path(), &exe).is_err());
}

#[test]
fn validates_reused_scope_and_generation_without_rotating_grants() {
    for mismatch in ["scope", "generation", "database"] {
        let server = Server::new();
        let (dir, exe) = fixture();
        let session = server.session();
        session.connect("test-agent", dir.path(), &exe).unwrap();
        let config_path = {
            let remote = server.state.lock().unwrap();
            PathBuf::from(remote.entries[0]["stdio"]["args"][2].as_str().unwrap())
        };
        let original = std::fs::read(&config_path).unwrap();
        let config = mother::config::load_config(&config_path).unwrap();
        let db = rusqlite::Connection::open(dir.path().join("context.sqlite")).unwrap();
        match mismatch {
            "scope" => {
                db.execute(
                    "UPDATE clients SET scopes=? WHERE id=?",
                    rusqlite::params![
                        "[\"read\",\"context:propose\",\"memory:write\"]",
                        config.client_id
                    ],
                )
                .unwrap();
            }
            "generation" => {
                db.execute("UPDATE metadata SET value=json_set(value,'$.generation',?) WHERE key='workspace'",[uuid::Uuid::new_v4().to_string()]).unwrap();
            }
            _ => {
                let mut replacement = serde_json::to_value(config).unwrap();
                let other = dir.path().join("other.sqlite");
                mother::store::Store::initialize(&other, "Other synthetic persona.").unwrap();
                replacement["databasePath"] = json!(other);
                std::fs::write(&config_path, replacement.to_string()).unwrap();
            }
        }
        drop(db);
        server.state.lock().unwrap().requests.clear();
        assert!(
            session.connect("test-agent", dir.path(), &exe).is_err(),
            "{mismatch}"
        );
        assert_eq!(server.state.lock().unwrap().requests.len(), 0, "{mismatch}");
        if mismatch != "database" {
            assert_eq!(std::fs::read(&config_path).unwrap(), original, "{mismatch}");
        }
    }
}

#[test]
fn per_agent_lock_prevents_concurrent_local_connects() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let _local = Local::open("test-agent", dir.path(), &exe, &server.endpoint).unwrap();
    assert!(
        server
            .session()
            .connect("test-agent", dir.path(), &exe)
            .is_err()
    );
    assert!(server.state.lock().unwrap().requests.is_empty());
}

#[test]
fn lists_summary_agents_with_separate_owner_and_api_auth() {
    let server = Server::new();
    assert_eq!(
        server.session().agents().unwrap(),
        vec![Agent {
            id: "test-agent".into(),
            name: "Synthetic agent".into()
        }]
    );
    let state = server.state.lock().unwrap();
    let header = state.headers[0].to_ascii_lowercase();
    assert!(header.contains("authorization: bearer owner-secret"));
    assert!(header.contains("x-api-key: api-secret"));
    assert!(!format!("{:?}", server.session()).contains("secret"));
}

#[test]
fn creates_separate_review_only_grant_and_reconciles_repeated_connect() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let session = server.session();
    let status = session.connect("test-agent", dir.path(), &exe).unwrap();
    assert!(status.contains("tools ready"), "{status}");
    assert!(status.contains("pending"), "{status}");
    session.connect("test-agent", dir.path(), &exe).unwrap();
    let state = server.state.lock().unwrap();
    assert_eq!(state.entries.len(), 1);
    assert_eq!(
        state
            .requests
            .iter()
            .filter(|(method, _, _)| method == "POST")
            .count(),
        2
    );
    let args = state.entries[0]["stdio"]["args"].as_array().unwrap();
    assert_eq!(args[0], "mcp");
    assert_eq!(args[1], "--config");
    let config = mother::config::load_config(Path::new(args[2].as_str().unwrap())).unwrap();
    let store = mother::store::Store::open(&dir.path().join("context.sqlite")).unwrap();
    store.authorize(&config.auth(), "read").unwrap();
    store.authorize(&config.auth(), "context:propose").unwrap();
    assert!(store.authorize(&config.auth(), "memory:write").is_err());
}

#[test]
fn dropped_create_is_reconciled_before_any_new_mutation() {
    let server = Server::new();
    server.state.lock().unwrap().drop_after_create = true;
    let (dir, exe) = fixture();
    let session = server.session();
    assert!(session.connect("test-agent", dir.path(), &exe).is_err());
    session.connect("test-agent", dir.path(), &exe).unwrap();
    let state = server.state.lock().unwrap();
    assert_eq!(state.entries.len(), 1);
    assert_eq!(
        state
            .requests
            .iter()
            .filter(|(method, path, _)| method == "POST" && path.ends_with("mcp-connections"))
            .count(),
        1
    );
    assert_eq!(state.requests[2].0, "GET");
}

#[test]
fn preserves_tool_review_policy() {
    let server = Server::new();
    server.state.lock().unwrap().review = true;
    let (dir, exe) = fixture();
    let status = server
        .session()
        .connect("test-agent", dir.path(), &exe)
        .unwrap();
    assert!(status.contains("review"), "{status}");
    assert!(!status.contains("tools ready"));
    assert!(
        !server
            .state
            .lock()
            .unwrap()
            .requests
            .iter()
            .any(|(_, path, _)| path.ends_with("/review") || path.contains("/grants/"))
    );
}

#[test]
fn refuses_duplicates_conflicts_and_stale_versions_without_overwrite() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let session = server.session();
    session.connect("test-agent", dir.path(), &exe).unwrap();
    {
        let mut state = server.state.lock().unwrap();
        let duplicate = state.entries[0].clone();
        state.entries.push(duplicate);
        state.requests.clear();
    }
    assert!(session.connect("test-agent", dir.path(), &exe).is_err());
    assert_eq!(server.state.lock().unwrap().requests.len(), 1);
    {
        let mut state = server.state.lock().unwrap();
        state.entries.pop();
        state.entries[0]["stdio"]["args"] = json!(["mcp", "--config", "other.private.json"]);
        state.requests.clear();
    }
    assert!(session.connect("test-agent", dir.path(), &exe).is_err());
    assert_eq!(server.state.lock().unwrap().requests.len(), 1);
    let server = Server::new();
    server.state.lock().unwrap().stale = true;
    let (dir, exe) = fixture();
    assert!(
        server
            .session()
            .connect("test-agent", dir.path(), &exe)
            .is_err()
    );
    assert!(
        !server
            .state
            .lock()
            .unwrap()
            .requests
            .iter()
            .any(|(_, path, _)| path.ends_with("/connect"))
    );
}

#[test]
fn rejects_remote_body_auth_errors_and_redirects_without_echoing_secrets() {
    for mode in ["deny", "redirect", "huge"] {
        let server = Server::new();
        {
            let mut state = server.state.lock().unwrap();
            match mode {
                "deny" => state.deny = true,
                "redirect" => state.redirect = true,
                _ => state.huge = true,
            }
        }
        let error = server.session().agents().unwrap_err();
        let text = format!("{error:?} {error}");
        assert!(!text.contains("owner-secret") && !text.contains("api-secret"));
        assert_eq!(server.state.lock().unwrap().requests.len(), 1);
    }
}

fn scoped(mode: AccessMode) -> ClientAccess {
    ClientAccess {
        mode,
        subject: Some("test-agent".into()),
        records: vec![],
    }
}

#[test]
fn large_shared_allowlist_roundtrips_and_reconciles_unknown_binding_without_replay() {
    let server = Server::new();
    {
        let mut remote = server.state.lock().unwrap();
        remote.binding_ready = true;
        remote.drop_without_binding = true;
    }
    let (dir, exe) = fixture();
    let mut store = Store::open(&dir.path().join("context.sqlite")).unwrap();
    let owner = store
        .grant("synthetic-owner", &["memory:write".into()])
        .unwrap();
    let auth = mother::contracts::Auth {
        client_id: "synthetic-owner".into(),
        token: owner["token"].as_str().unwrap().into(),
    };
    let workspace = store.workspace().unwrap();
    let mut access = scoped(AccessMode::Shared);
    for index in 0..500 {
        let receipt = store.write(&auth, json!({"workspaceId":workspace["id"],"generation":workspace["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","status":"active","basis":"owner_statement","title":format!("Synthetic record {index}"),"content":"Synthetic boundary fixture"}})).unwrap();
        access
            .records
            .push(receipt["record"]["id"].as_str().unwrap().into());
    }
    let session = server.session();
    assert!(
        session
            .connect_with_access("test-agent", dir.path(), &exe, &access)
            .is_err()
    );
    let binding_path = fs::read_dir(dir.path().join("clients"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.to_string_lossy().ends_with(".binding.json"))
        .unwrap();
    assert!(fs::metadata(&binding_path).unwrap().len() > 16384);
    assert!(
        session
            .connect_with_access("test-agent", dir.path(), &exe, &access)
            .is_err()
    );
    {
        let mut remote = server.state.lock().unwrap();
        remote.binding = remote
            .requests
            .iter()
            .find(|(method, _, _)| method == "PUT")
            .unwrap()
            .2["binding"]
            .clone();
    }
    session
        .connect_with_access("test-agent", dir.path(), &exe, &access)
        .unwrap();
    assert_eq!(
        server
            .state
            .lock()
            .unwrap()
            .requests
            .iter()
            .filter(|(method, _, _)| method == "PUT")
            .count(),
        1
    );
    let saved: Binding =
        serde_json::from_value(read_json(&binding_path, MAX_BINDING_BYTES).unwrap()).unwrap();
    assert_eq!(saved.access.as_ref(), Some(&access));
    assert!(saved.pending_binding.is_none());
    let maximum = Binding {
        access: Some(ClientAccess {
            records: vec![access.records[0].clone(); 10000],
            ..access
        }),
        ..saved
    };
    assert!(maximum.encoded().unwrap().len() <= MAX_BINDING_BYTES);
}

#[test]
fn oversized_binding_is_rejected_before_provisioning_a_grant() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let access = ClientAccess {
        records: vec!["00000000-0000-4000-8000-000000000001".into(); 14000],
        ..scoped(AccessMode::Shared)
    };
    let error = server
        .session()
        .connect_with_access("test-agent", dir.path(), &exe, &access)
        .unwrap_err();
    assert!(error.to_string().contains("512 KiB"));
    let database = rusqlite::Connection::open(dir.path().join("context.sqlite")).unwrap();
    assert_eq!(
        database
            .query_row("SELECT count(*) FROM clients", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        0
    );
    assert!(server.state.lock().unwrap().requests.is_empty());
    assert!(
        !fs::read_dir(dir.path().join("clients"))
            .unwrap()
            .any(|entry| entry
                .unwrap()
                .path()
                .to_string_lossy()
                .ends_with(".private.json"))
    );
}

#[test]
fn scoped_binding_requires_current_ready_owner_review() {
    let server = Server::new();
    server.state.lock().unwrap().review = true;
    let (dir, exe) = fixture();
    let status = server
        .session()
        .connect_with_access(
            "test-agent",
            dir.path(),
            &exe,
            &scoped(AccessMode::Companion),
        )
        .unwrap();
    assert!(status.contains("No identity"));
    assert!(server.state.lock().unwrap().binding.is_null());
    assert!(
        !server
            .state
            .lock()
            .unwrap()
            .requests
            .iter()
            .any(|(m, _, _)| m == "PUT")
    );
    {
        let mut remote = server.state.lock().unwrap();
        remote.entries[0]["state"] = json!("ready");
        remote.entries[0]["reviewedTools"] = json!([remote.entries[0]["candidates"][0].clone()]);
    }
    let status = server
        .session()
        .connect_with_access(
            "test-agent",
            dir.path(),
            &exe,
            &scoped(AccessMode::Companion),
        )
        .unwrap();
    assert!(status.contains("same chat"));
    let remote = server.state.lock().unwrap();
    assert_eq!(remote.binding["mode"], "companion");
    assert_eq!(remote.binding["subject"], "test-agent");
    let store = Store::open(&dir.path().join("context.sqlite")).unwrap();
    assert_eq!(
        remote.binding["workspaceId"],
        store.workspace().unwrap()["id"]
    );
}

#[test]
fn automatic_setup_pins_only_the_context_reader_once_without_changing_grants() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let session = server.session();
    for _ in 0..2 {
        let status = session
            .connect_with_access(
                "test-agent",
                dir.path(),
                &exe,
                &scoped(AccessMode::Companion),
            )
            .unwrap();
        assert!(status.contains("Companion configured"));
    }
    let remote = server.state.lock().unwrap();
    let reviews = remote
        .requests
        .iter()
        .filter(|(_, path, _)| path.ends_with("/review"))
        .collect::<Vec<_>>();
    assert_eq!(reviews.len(), 1);
    assert_eq!(
        reviews[0].2,
        json!({"expectedVersion":2,"discoveryId":"discovery-1","descriptorDigest":"a".repeat(64)})
    );
    assert_eq!(
        remote.entries[0]["reviewedTools"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        remote.entries[0]["reviewedTools"][0]["remoteName"],
        "mother_get_context"
    );
    assert!(
        !remote
            .requests
            .iter()
            .any(|(_, path, _)| path.contains("/grants/"))
    );
}

#[test]
fn uncertain_reader_review_reconciles_only_the_exact_descriptor_discovery_and_version() {
    for outcome in ["applied", "absent", "version", "digest", "discovery"] {
        let server = Server::new();
        {
            let mut remote = server.state.lock().unwrap();
            remote.drop_after_review = outcome != "absent";
            remote.drop_without_review = outcome == "absent";
        }
        let (dir, exe) = fixture();
        let session = server.session();
        assert!(
            session
                .connect_with_access("test-agent", dir.path(), &exe, &scoped(AccessMode::Shared))
                .is_err()
        );
        {
            let mut remote = server.state.lock().unwrap();
            match outcome {
                "version" => remote.entries[0]["version"] = json!(4),
                "digest" => {
                    remote.entries[0]["candidates"][0]["descriptorDigest"] = json!("b".repeat(64))
                }
                "discovery" => remote.entries[0]["discoveryId"] = json!("different-discovery"),
                _ => {}
            }
        }
        let result = session.connect_with_access(
            "test-agent",
            dir.path(),
            &exe,
            &scoped(AccessMode::Shared),
        );
        assert_eq!(result.is_ok(), outcome == "applied", "{outcome}");
        let remote = server.state.lock().unwrap();
        assert_eq!(
            remote
                .requests
                .iter()
                .filter(|(_, path, _)| path.ends_with("/review"))
                .count(),
            1,
            "{outcome}"
        );
        assert!(
            !remote
                .requests
                .iter()
                .any(|(_, path, _)| path.contains("/grants/"))
        );
        assert_eq!(remote.binding.is_null(), outcome != "applied");
    }
}

#[test]
fn dropped_binding_save_is_read_and_never_blindly_repeated() {
    for applied in [true, false] {
        let server = Server::new();
        {
            let mut remote = server.state.lock().unwrap();
            remote.binding_ready = true;
            remote.drop_after_binding = applied;
            remote.drop_without_binding = !applied;
        }
        let (dir, exe) = fixture();
        let session = server.session();
        let result = session.connect_with_access(
            "test-agent",
            dir.path(),
            &exe,
            &scoped(AccessMode::Shared),
        );
        assert_eq!(result.is_ok(), applied);
        let repeated = session.connect_with_access(
            "test-agent",
            dir.path(),
            &exe,
            &scoped(AccessMode::Shared),
        );
        assert_eq!(repeated.is_ok(), applied);
        assert_eq!(
            server
                .state
                .lock()
                .unwrap()
                .requests
                .iter()
                .filter(|(m, _, _)| m == "PUT")
                .count(),
            1
        );
    }
}

#[test]
fn local_grants_are_separate_per_agent_and_role_and_never_widened() {
    let server = Server::new();
    let (dir, exe) = fixture();
    let legacy = Local::open("test-agent", dir.path(), &exe, &server.endpoint).unwrap();
    let original = fs::read(&legacy.config_path).unwrap();
    let companion = Local::open_with_access(
        "test-agent",
        dir.path(),
        &exe,
        &server.endpoint,
        Some(&scoped(AccessMode::Companion)),
    )
    .unwrap();
    let shared = Local::open_with_access(
        "test-agent",
        dir.path(),
        &exe,
        &server.endpoint,
        Some(&scoped(AccessMode::Shared)),
    )
    .unwrap();
    let other = ClientAccess {
        subject: Some("another-agent".into()),
        ..scoped(AccessMode::Companion)
    };
    let another = Local::open_with_access(
        "another-agent",
        dir.path(),
        &exe,
        &server.endpoint,
        Some(&other),
    )
    .unwrap();
    let paths = [
        &legacy.config_path,
        &companion.config_path,
        &shared.config_path,
        &another.config_path,
    ];
    assert_eq!(
        paths
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .len(),
        4
    );
    assert_eq!(fs::read(&legacy.config_path).unwrap(), original);
    let shared_path = shared.config_path.clone();
    drop(shared);
    let mut changed = scoped(AccessMode::Shared);
    changed
        .records
        .push("00000000-0000-4000-8000-000000000001".into());
    let original = fs::read(&shared_path).unwrap();
    assert!(
        Local::open_with_access(
            "test-agent",
            dir.path(),
            &exe,
            &server.endpoint,
            Some(&changed)
        )
        .is_err()
    );
    assert_eq!(fs::read(&shared_path).unwrap(), original);
}

#[test]
fn differing_binding_and_active_old_connection_are_preserved() {
    let server = Server::new();
    server.state.lock().unwrap().binding_ready = true;
    let (dir, exe) = fixture();
    let session = server.session();
    session.connect("test-agent", dir.path(), &exe).unwrap();
    let original = server.state.lock().unwrap().entries[0].clone();
    assert!(
        session
            .connect_with_access(
                "test-agent",
                dir.path(),
                &exe,
                &scoped(AccessMode::Companion)
            )
            .is_err()
    );
    assert_eq!(server.state.lock().unwrap().entries[0], original);
    {
        let mut remote = server.state.lock().unwrap();
        remote.entries[0]["enabled"] = json!(false);
        remote.binding = json!({"connectionId":"different","tool":"mother_get_context","mode":"companion","workspaceId":"different","generation":"different","subject":"test-agent"});
    }
    assert!(
        session
            .connect_with_access(
                "test-agent",
                dir.path(),
                &exe,
                &scoped(AccessMode::Companion)
            )
            .is_err()
    );
    assert_eq!(
        server.state.lock().unwrap().binding["connectionId"],
        "different"
    );
    assert!(
        !server
            .state
            .lock()
            .unwrap()
            .requests
            .iter()
            .any(|(m, _, _)| m == "PUT")
    );
}
