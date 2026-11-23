use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, store::GardenStore},
    service::{LocalClient, LocalService, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;

struct Owner(Child);
impl Drop for Owner {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

struct Fixture {
    _directory: tempfile::TempDir,
    db: PathBuf,
    endpoint: PathBuf,
    config: ServiceClientConfig,
}
impl Fixture {
    fn new() -> Self {
        let directory =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let parent = directory.path().to_owned();
        Self::in_directory(directory, &parent)
    }
    fn in_directory(directory: tempfile::TempDir, parent: &Path) -> Self {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let db = parent.join("garden.redb");
        let endpoint = parent.join("service.json");
        let workspace_id = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(&db, &workspace_id, &generation).unwrap();
        let auth = store
            .grant(
                "service-test",
                &["garden:read".into(), "garden:write".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        drop(store);
        let config = ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: endpoint.clone(),
            workspace_id,
            generation,
            client_id: auth.client_id,
            token: auth.token,
        };
        Self {
            _directory: directory,
            db,
            endpoint,
            config,
        }
    }
    fn start(&self) -> Owner {
        let child = Command::new(std::env::current_exe().unwrap())
            .args(["--ignored", "--exact", "service_child", "--nocapture"])
            .env("MOTHER_TEST_SERVICE_DB", &self.db)
            .env("MOTHER_TEST_SERVICE_ENDPOINT", &self.endpoint)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let mut owner = Owner(child);
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut last_error = String::new();
        while Instant::now() < deadline {
            match LocalClient::new(self.config.clone()) {
                Ok(_) => return owner,
                Err(error) => last_error = error.to_string(),
            }
            assert!(
                owner.0.try_wait().unwrap().is_none(),
                "synthetic owner exited before readiness"
            );
            thread::sleep(Duration::from_millis(20));
        }
        panic!("synthetic service owner did not become ready: {last_error}");
    }
    fn batch(&self) -> Value {
        json!({"version":2,"workspaceId":self.config.workspace_id,"generation":self.config.generation,
            "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{
                "itemKey":"entity","mutation":"create","expectedRevision":0,
                "draft":{"kind":"entity","entityType":"test:person","displayName":"synthetic",
                    "aliases":[],"extensions":{}}}]})
    }
}

#[test]
fn scoped_attachment_waits_for_owner_and_verifies_the_live_link() {
    use mother::service::attachment::{self, STATUS};
    let mut fixture = Fixture::new();
    let store = GardenStore::open(&fixture.db).unwrap();
    store.v1_initialize("Synthetic transfer owner").unwrap();
    let grant = |id, scopes: &[String]| {
        store
            .v1_grant(id, scopes, &ClientAccess::default(), None)
            .unwrap()
    };
    let writer = grant("fixture-writer", &["read".into(), "memory:write".into()]);
    let reader = grant("file-reader", &["read".into()]);
    let other = grant("other-reader", &["read".into()]);
    let memory = store.v1_call(&writer, "mother_write_record", json!({
        "workspaceId":fixture.config.workspace_id,"generation":fixture.config.generation,
        "operationId":Uuid::new_v4().to_string(),"expectedRevision":0,
        "record":{"kind":"memory","title":"Synthetic attachment target","content":"Synthetic bytes.","status":"active","basis":"source"}
    })).unwrap()["record"]["id"].clone();
    drop(store);
    fixture.config.client_id = reader.client_id;
    fixture.config.token = reader.token;
    let _owner = fixture.start();
    let client = LocalClient::new(fixture.config.clone()).unwrap();
    let id = Uuid::new_v4().to_string();
    let query = json!({"proposalId":id});
    let mut clip = include_bytes!("fixtures/attachment-sample.mp4").to_vec();
    // A valid synthetic MP4 with an ISO BMFF free-space box exercises a realistic phone upload.
    let total: usize = 726_107;
    clip.extend_from_slice(&((total - clip.len()) as u32).to_be_bytes());
    clip.extend_from_slice(b"free");
    clip.resize(total, 0);
    let submit = |metadata: &Value| attachment::submit(&client, metadata.clone(), &clip);
    let mut upload = json!({"proposalId":id,"memoryId":memory,"expectedRevision":1,"fileId":"file-synthetic","fileName":"clip.mp4",
        "mimeType":"video/mp4"});
    assert_eq!(submit(&upload).unwrap_err().code(), "FORBIDDEN");
    attachment::set_enabled(&client, true).unwrap();
    let item = submit(&upload).unwrap();
    assert_eq!(item["status"], "pending_owner_review");
    assert!(item["verification"].is_null());
    assert_eq!(item["sizeBytes"], total);
    assert_eq!(item["fileName"], "clip.mp4");
    assert_eq!(item["proposedBy"], "file-reader");
    assert_eq!(submit(&upload).unwrap(), item);
    assert_eq!(
        client
            .call("local_media_finish_review", &query)
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    let mut other_config = fixture.config.clone();
    other_config.client_id = other.client_id;
    other_config.token = other.token;
    let other_client = LocalClient::new(other_config).unwrap();
    assert_eq!(
        other_client
            .call("mother_get_capabilities", &json!({}))
            .unwrap()["mediaAttachmentBridge"]["available"],
        true
    );
    assert_eq!(
        other_client.call(STATUS, &query).unwrap_err().code(),
        "NOT_FOUND"
    );
    let independent = attachment::submit(&other_client, upload.clone(), &clip).unwrap();
    assert_eq!(independent["proposedBy"], "other-reader");
    assert_eq!(independent["status"], "pending_owner_review");
    assert_ne!(independent["reviewId"], item["reviewId"]);
    upload["fileId"] = json!("a-different-file");
    assert_eq!(submit(&upload).unwrap_err().code(), "OPERATION_REUSED");
    let review = item["reviewId"].as_str().unwrap();
    assert!(attachment::review(&client, &id, "wrong", true).is_err());
    let approved = attachment::review(&client, &id, review, true).unwrap();
    assert_eq!(approved["verification"]["verified"], true);
    assert_eq!(approved["verification"]["link"]["memoryId"], memory);
    assert_eq!(client.call(STATUS, &query).unwrap(), approved);
    assert_eq!(
        attachment::review(&client, &id, review, true).unwrap(),
        approved
    );
    let other_id = Uuid::new_v4().to_string();
    upload["proposalId"] = json!(other_id);
    let pending = submit(&upload).unwrap();
    assert_eq!(
        attachment::review(
            &client,
            &other_id,
            pending["reviewId"].as_str().unwrap(),
            false
        )
        .unwrap()["status"],
        "rejected"
    );
    attachment::set_enabled(&client, false).unwrap();
    assert_eq!(client.call(STATUS, &query).unwrap_err().code(), "FORBIDDEN");
}

#[cfg(unix)]
fn fixture_with_private_parent_length(length: usize) -> Fixture {
    use std::os::unix::ffi::OsStrExt;
    let directory =
        tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let padding = length
        .checked_sub(directory.path().as_os_str().as_bytes().len() + 1)
        .filter(|padding| *padding > 0)
        .expect("synthetic temporary root must leave room for the test parent");
    let parent = directory.path().join("p".repeat(padding));
    std::fs::create_dir(&parent).unwrap();
    Fixture::in_directory(directory, &parent)
}

#[cfg(unix)]
#[test]
fn unix_service_authenticates_under_a_long_private_parent() {
    let fixture = fixture_with_private_parent_length(72);
    let _owner = fixture.start();
    let client = LocalClient::new(fixture.config.clone()).unwrap();
    let info = client.info();
    assert_eq!(
        Uuid::parse_str(&info.service_id).unwrap().to_string(),
        info.service_id
    );
    assert_eq!(Path::new(&info.address).parent(), fixture.endpoint.parent());
    assert!(
        info.address.len() <= 103,
        "address must also fit macOS sun_path"
    );
    assert_eq!(
        client.call("mother_get_capabilities", &json!({})).unwrap()["workspace"]["workspaceId"],
        fixture.config.workspace_id
    );

    let mut redirected = info.clone();
    redirected.address = fixture
        .endpoint
        .with_file_name("unrelated.sock")
        .to_str()
        .unwrap()
        .into();
    std::fs::write(&fixture.endpoint, serde_json::to_vec(&redirected).unwrap()).unwrap();
    assert_eq!(
        LocalClient::new(fixture.config.clone()).unwrap_err().code(),
        "INVALID_INPUT"
    );
}

#[cfg(unix)]
#[test]
fn unix_service_replaces_only_an_exact_legacy_descriptor() {
    let directory = tempfile::tempdir_in(std::fs::canonicalize("/tmp").unwrap()).unwrap();
    let parent = directory.path().to_owned();
    let fixture = Fixture::in_directory(directory, &parent);
    let service_id = Uuid::new_v4().to_string();
    let legacy_address = parent.join(format!(".mother-{service_id}.sock"));
    std::os::unix::net::SocketAddr::from_pathname(&legacy_address).unwrap();
    let mut descriptor = json!({
        "version":1, "serviceId":service_id, "address":legacy_address,
        "workspaceId":fixture.config.workspace_id, "generation":fixture.config.generation,
        "v1Available":false
    });
    descriptor["address"] = json!(parent.join("unrelated.sock"));
    let unrelated = serde_json::to_vec(&descriptor).unwrap();
    std::fs::write(&fixture.endpoint, &unrelated).unwrap();
    assert_eq!(
        LocalService::bind(&fixture.db, &fixture.endpoint)
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
    assert_eq!(std::fs::read(&fixture.endpoint).unwrap(), unrelated);

    descriptor["address"] = json!(legacy_address);
    std::fs::write(&fixture.endpoint, serde_json::to_vec(&descriptor).unwrap()).unwrap();
    let service = LocalService::bind(&fixture.db, &fixture.endpoint).unwrap();
    assert_ne!(service.info().service_id, service_id);
    assert_ne!(service.info().address, legacy_address.to_str().unwrap());
    assert!(service.info().address.len() < legacy_address.as_os_str().len());
}

#[cfg(unix)]
#[test]
fn unix_service_rejects_an_overlong_address_before_creating_owner_state() {
    let fixture = fixture_with_private_parent_length(160);
    let before = std::fs::read_dir(fixture.endpoint.parent().unwrap())
        .unwrap()
        .count();
    let error = LocalService::bind(&fixture.db, &fixture.endpoint).unwrap_err();
    assert_eq!(error.code(), "INVALID_INPUT");
    assert!(
        error
            .to_string()
            .contains("shorter owner-private directory")
    );
    assert!(!fixture.endpoint.exists());
    assert!(
        !fixture
            .db
            .with_added_extension("service-owner.lock")
            .exists()
    );
    assert_eq!(
        std::fs::read_dir(fixture.endpoint.parent().unwrap())
            .unwrap()
            .count(),
        before
    );
    let store = GardenStore::open(&fixture.db).expect("rejected bind must release the database");
    drop(store);
}

#[test]
#[ignore = "synthetic child process entry point"]
fn service_child() {
    let db = std::env::var_os("MOTHER_TEST_SERVICE_DB").expect("synthetic child database");
    let endpoint =
        std::env::var_os("MOTHER_TEST_SERVICE_ENDPOINT").expect("synthetic child endpoint");
    LocalService::bind(Path::new(&db), Path::new(&endpoint))
        .unwrap()
        .run()
        .unwrap();
}

#[test]
fn two_real_clients_share_one_writer_and_durable_retry_survives_owner_restart() {
    let fixture = Fixture::new();
    let owner = fixture.start();
    assert!(
        LocalService::bind(&fixture.db, &fixture.endpoint).is_err(),
        "second owner must fail"
    );
    assert!(
        GardenStore::open(&fixture.db).is_err(),
        "service must retain its database handle"
    );
    let first = LocalClient::new(fixture.config.clone()).unwrap();
    let second = LocalClient::new(fixture.config.clone()).unwrap();
    let request = fixture.batch();
    let original = first.call("mother_garden_apply", &request).unwrap();
    assert_eq!(
        second.call("mother_garden_apply", &request).unwrap(),
        original
    );
    assert!(
        !std::fs::read_to_string(&fixture.endpoint)
            .unwrap()
            .contains(&fixture.config.token)
    );
    drop(owner);
    let _owner = fixture.start();
    assert_eq!(
        first.call("mother_garden_apply", &request).unwrap(),
        original
    );
    let mut changed = request;
    changed["items"][0]["draft"]["displayName"] = json!("changed");
    assert_eq!(
        second
            .call("mother_garden_apply", &changed)
            .unwrap_err()
            .code(),
        "OPERATION_REUSED"
    );
}

#[test]
fn service_checks_current_credential_workspace_generation_and_revoke_after_restart() {
    let fixture = Fixture::new();
    let owner = fixture.start();
    for (field, expected) in [
        ("token", "UNAUTHORIZED"),
        ("workspace", "WORKSPACE_MISMATCH"),
        ("generation", "GENERATION_MISMATCH"),
    ] {
        let mut config = fixture.config.clone();
        match field {
            "token" => config.token = "0".repeat(64),
            "workspace" => config.workspace_id = Uuid::new_v4().to_string(),
            _ => config.generation = Uuid::new_v4().to_string(),
        }
        assert_eq!(
            LocalClient::new(config).unwrap_err().code(),
            expected,
            "{field}"
        );
    }
    let client = LocalClient::new(fixture.config.clone()).unwrap();
    for method in ["approve", "grant", "import", "restore", "shutdown"] {
        assert_eq!(
            client.call(method, &json!({})).unwrap_err().code(),
            "INVALID_INPUT"
        );
    }
    drop(owner);
    let store = GardenStore::open(&fixture.db).unwrap();
    store.revoke(&fixture.config.client_id).unwrap();
    drop(store);
    // Start without waiting for credential readiness: the revoked client must stay revoked.
    let service = LocalService::bind(&fixture.db, &fixture.endpoint).unwrap();
    let stopped = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = stopped.clone();
    let worker = thread::spawn(move || service.serve_until(control));
    assert_eq!(
        client
            .call("mother_get_capabilities", &json!({}))
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
    stopped.store(true, std::sync::atomic::Ordering::Release);
    worker.join().unwrap().unwrap();
}

fn raw_connect(endpoint: &Path) -> interprocess::local_socket::Stream {
    use interprocess::local_socket::ConnectOptions;
    let info: Value = serde_json::from_slice(&std::fs::read(endpoint).unwrap()).unwrap();
    let address = info["address"].as_str().unwrap();
    #[cfg(windows)]
    let name = {
        use interprocess::local_socket::{GenericNamespaced, ToNsName};
        address.to_ns_name::<GenericNamespaced>().unwrap()
    };
    #[cfg(unix)]
    let name = {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        address.to_fs_name::<GenericFilePath>().unwrap()
    };
    ConnectOptions::new().name(name).connect_sync().unwrap()
}

#[test]
fn incomplete_and_oversized_unauthenticated_peers_have_fixed_deadlines() {
    let fixture = Fixture::new();
    let _owner = fixture.start();
    let client = LocalClient::new(fixture.config.clone()).unwrap();
    let mut peer = raw_connect(&fixture.endpoint);
    peer.write_all(&[0, 0]).unwrap();
    let started = Instant::now();
    client.call("mother_get_capabilities", &json!({})).unwrap();
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "partial unauthenticated peer blocked service"
    );
    drop(peer);
    let mut peer = raw_connect(&fixture.endpoint);
    peer.write_all(&(4_194_305_u32).to_be_bytes()).unwrap();
    let started = Instant::now();
    client.call("mother_get_capabilities", &json!({})).unwrap();
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "oversized unauthenticated peer blocked service"
    );
}

#[test]
fn v1_transport_reuses_native_contracts_and_preserves_independent_graph_authority() {
    let mut fixture = Fixture::new();
    let store = GardenStore::open(&fixture.db).unwrap();
    store
        .v1_initialize("Synthetic local service persona")
        .unwrap();
    let auth = store
        .v1_grant(
            "v1-client",
            &["read".into(), "memory:write".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    fixture.config.client_id = auth.client_id.clone();
    fixture.config.token = auth.token.clone();
    let write = json!({"workspaceId":fixture.config.workspace_id,"generation":fixture.config.generation,
        "operationId":Uuid::new_v4().to_string(),"expectedRevision":0,
        "record":{"kind":"memory","title":"Synthetic IPC","content":"Local IPC carries synthetic context.",
            "status":"active","basis":"source"}});
    let expected = store
        .v1_call(&auth, "mother_write_record", write.clone())
        .unwrap();
    drop(store);
    let _owner = fixture.start();
    let client = LocalClient::new(fixture.config.clone()).unwrap();
    let capabilities = client.call("mother_get_capabilities", &json!({})).unwrap();
    assert_eq!(capabilities["v1"]["available"], true);
    assert_eq!(capabilities["garden"]["available"], false);
    assert_eq!(capabilities["garden"]["availableTools"], json!([]));
    assert_eq!(
        client.call("mother_write_record", &write).unwrap(),
        expected
    );
    assert_eq!(
        client
            .call("mother_get_record", &json!({"id":expected["record"]["id"]}))
            .unwrap(),
        expected["record"]
    );
    let search = client
        .call(
            "mother_search_context",
            &json!({"query":"synthetic","mode":"literal"}),
        )
        .unwrap();
    assert_eq!(search["search"]["effectiveMode"], "literal");
    assert_eq!(search["records"][0]["id"], expected["record"]["id"]);
    assert_eq!(
        client
            .call("mother_garden_apply", &fixture.batch())
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    let mut wrong_generation = write;
    wrong_generation["generation"] = json!(Uuid::new_v4().to_string());
    wrong_generation["operationId"] = json!(Uuid::new_v4().to_string());
    assert_eq!(
        client
            .call("mother_write_record", &wrong_generation)
            .unwrap_err()
            .code(),
        "STALE_GENERATION"
    );
}

#[test]
fn strict_service_config_redacts_secrets_and_never_follows_unrelated_descriptors() {
    let fixture = Fixture::new();
    assert!(!format!("{:?}", fixture.config).contains(&fixture.config.token));
    let mut encoded = serde_json::to_value(&fixture.config).unwrap();
    encoded["databasePath"] = json!(fixture.db);
    assert!(serde_json::from_value::<ServiceClientConfig>(encoded).is_err());
    let mut encoded = serde_json::to_value(&fixture.config).unwrap();
    encoded["backend"] = json!("garden_redb");
    let config: ServiceClientConfig = serde_json::from_value(encoded).unwrap();
    assert!(config.validate().is_err());
    std::fs::write(&fixture.endpoint, b"unrelated owner text").unwrap();
    assert!(LocalService::bind(&fixture.db, &fixture.endpoint).is_err());
    assert_eq!(
        std::fs::read(&fixture.endpoint).unwrap(),
        b"unrelated owner text"
    );
}

#[test]
fn a_hard_link_cannot_create_an_unfenced_database_endpoint() {
    let fixture = Fixture::new();
    let alias = fixture.db.with_file_name("alias.redb");
    std::fs::hard_link(&fixture.db, &alias).unwrap();
    assert_eq!(
        LocalService::bind(&alias, &fixture.endpoint)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert!(!fixture.endpoint.exists());
}

#[test]
fn running_owner_revocation_is_checked_on_the_next_request() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let fixture = Fixture::new();
    let service = LocalService::bind(&fixture.db, &fixture.endpoint).unwrap();
    let stopped = AtomicBool::new(false);
    struct StopOnDrop<'a>(&'a AtomicBool);
    impl Drop for StopOnDrop<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }
    thread::scope(|scope| {
        let worker = scope.spawn(|| service.serve(&stopped));
        let guard = StopOnDrop(&stopped);
        let client = LocalClient::new(fixture.config.clone()).unwrap();
        client.call("mother_get_capabilities", &json!({})).unwrap();
        service.store().revoke(&fixture.config.client_id).unwrap();
        assert_eq!(
            client
                .call("mother_get_capabilities", &json!({}))
                .unwrap_err()
                .code(),
            "UNAUTHORIZED"
        );
        drop(guard);
        worker.join().unwrap().unwrap();
    });
}

#[test]
fn redirected_fake_owner_never_receives_the_bearer_token() {
    fake_owner_attempt(BadProof::Invalid);
}

#[test]
fn replayed_owner_proof_cannot_authorize_a_fresh_client_challenge() {
    fake_owner_attempt(BadProof::PreviousChallenge);
}

#[test]
fn owner_proof_cannot_be_replayed_for_another_service_identity() {
    fake_owner_attempt(BadProof::OtherService);
}

enum BadProof {
    Invalid,
    PreviousChallenge,
    OtherService,
}

fn fake_owner_attempt(bad_proof: BadProof) {
    use interprocess::local_socket::{
        ListenerNonblockingMode, ListenerOptions, traits::Listener as _,
    };
    use std::io::Read;
    let fixture = Fixture::new();
    let service = LocalService::bind(&fixture.db, &fixture.endpoint).unwrap();
    let info = service.info().clone();
    drop(service);
    #[cfg(windows)]
    let name = {
        use interprocess::local_socket::{GenericNamespaced, ToNsName};
        info.address
            .as_str()
            .to_ns_name::<GenericNamespaced>()
            .unwrap()
    };
    #[cfg(unix)]
    let name = {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        info.address
            .as_str()
            .to_fs_name::<GenericFilePath>()
            .unwrap()
    };
    let listener = ListenerOptions::new()
        .name(name)
        .nonblocking(ListenerNonblockingMode::Both)
        .create_sync()
        .unwrap();
    std::fs::write(&fixture.endpoint, serde_json::to_vec(&info).unwrap()).unwrap();
    let database_path = fixture.db.clone();
    let client_id = fixture.config.client_id.clone();
    let fake = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut stream = loop {
            assert!(
                Instant::now() < deadline,
                "fake owner did not receive a connection"
            );
            match listener.accept() {
                Ok(stream) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(2))
                }
                Err(error) => panic!("fake owner accept: {error}"),
            }
        };
        let mut captured = Vec::new();
        let mut buffer = [0; 8192];
        while Instant::now() < deadline {
            match stream.read(&mut buffer) {
                Ok(count) if count > 0 => {
                    captured.extend_from_slice(&buffer[..count]);
                    if captured.len() >= 4 {
                        let length = u32::from_be_bytes(captured[..4].try_into().unwrap()) as usize;
                        if captured.len() >= length + 4 {
                            break;
                        }
                    }
                }
                Ok(_) => thread::sleep(Duration::from_millis(2)),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(2))
                }
                Err(_) => break,
            }
        }
        let invalid_mac = if !matches!(bad_proof, BadProof::Invalid) {
            let context = String::from_utf8(captured[4..].to_vec()).unwrap();
            let hello: Value = serde_json::from_str(&context).unwrap();
            let fresh = match bad_proof {
                BadProof::PreviousChallenge => hello["challenge"].as_str().unwrap(),
                _ => hello["serviceId"].as_str().unwrap(),
            };
            let previous_context = context.replace(fresh, &Uuid::new_v4().to_string());
            let store = GardenStore::open(&database_path).unwrap();
            store
                .local_service_proof(&client_id, previous_context.as_bytes())
                .unwrap()
        } else {
            [0_u8; 32]
        };
        let invalid =
            serde_json::to_vec(&json!({"status":"proof","version":1,"mac":invalid_mac})).unwrap();
        let _ = stream.write_all(&(invalid.len() as u32).to_be_bytes());
        let _ = stream.write_all(&invalid);
        let deadline = Instant::now() + Duration::from_millis(100);
        while Instant::now() < deadline {
            match stream.read(&mut buffer) {
                Ok(count) if count > 0 => captured.extend_from_slice(&buffer[..count]),
                _ => thread::sleep(Duration::from_millis(2)),
            }
        }
        #[cfg(windows)]
        {
            let interprocess::local_socket::Stream::NamedPipe(pipe) = &stream;
            pipe.inner().assume_flushed();
        }
        captured
    });
    let result = LocalClient::new(fixture.config.clone());
    let captured = fake.join().unwrap();
    assert!(
        !String::from_utf8_lossy(&captured).contains(&fixture.config.token),
        "fake owner received plaintext credential"
    );
    assert_eq!(result.unwrap_err().code(), "UNAUTHORIZED");
}
