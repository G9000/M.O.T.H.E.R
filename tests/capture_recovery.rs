use mother::{
    access::{AccessMode, ClientAccess},
    config::create_private_file,
    garden::store::GardenStore,
    service::{LocalClient, LocalService, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

fn capture(policy: &Path, args: &[&str], input: Option<&Value>) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["capture", "--policy"])
        .arg(policy)
        .args(args)
        .env("MOTHER_HOME", policy.parent().unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(input) = input {
        serde_json::to_writer(child.stdin.take().unwrap(), input).unwrap();
    }
    child.wait_with_output().unwrap()
}

fn status(policy: &Path, action: &str, id: &str) -> Value {
    let output = capture(policy, &[action, "--operation", id], None);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(response["operations"][0]["nextAction"].is_string());
    response["operations"][0].clone()
}

struct StopOnDrop<'a>(&'a AtomicBool);
impl Drop for StopOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

#[test]
fn named_capture_client_recovers_precommit_and_postcommit_interruptions_after_restart() {
    let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let state = root.path().join("state");
    fs::create_dir(&state).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for path in [root.path(), state.as_path()] {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
        }
    }
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let db = root.path().join("garden.redb");
    let endpoint = root.path().join("service.json");
    let store = GardenStore::initialize(&db, &workspace, &generation).unwrap();
    store.v1_initialize("Synthetic capture owner").unwrap();
    let auth = store
        .v1_grant(
            "codex-synthetic",
            &["read".into(), "context:propose".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    let config = ServiceClientConfig {
        version: 3,
        backend: "garden_service".into(),
        endpoint_path: endpoint.clone(),
        workspace_id: workspace.clone(),
        generation: generation.clone(),
        client_id: auth.client_id,
        token: auth.token,
    };
    let config_path = root.path().join("config.json");
    serde_json::to_writer(create_private_file(&config_path).unwrap(), &config).unwrap();
    let policy = root.path().join("policy.json");
    serde_json::to_writer(
        create_private_file(&policy).unwrap(),
        &json!({"enabled":true,"state_dir":state,"service_config":config_path}),
    )
    .unwrap();
    drop(store);

    let cases = [
        "Before transmission",
        "Lost commit response",
        "Dismissed selection",
        "Reused identity",
        "Rejected target",
    ];
    let ids = cases.map(|title| {
        let id = Uuid::new_v4().to_string();
        let selection = json!({
            "operationId": id, "title": title,
            "originals": [{"speaker":"user","text":"A synthetic selection."}]
        });
        let output = capture(&policy, &["select"], Some(&selection));
        assert!(!output.status.success());
        assert!(state.join(format!("{id}.json")).is_file());
        assert_eq!(status(&policy, "status", &id)["status"], "unconfirmed");
        id
    });
    let service = LocalService::bind(&db, &endpoint).unwrap();
    let stop = AtomicBool::new(false);
    std::thread::scope(|scope| {
        scope.spawn(|| service.serve(&stop).unwrap());
        let _stop = StopOnDrop(&stop);
        let client = LocalClient::new(config.clone()).unwrap();
        for (index, id) in ids.iter().enumerate() {
            let journal = state.join(format!("{id}.json"));
            let mut pending: Value = serde_json::from_slice(&fs::read(&journal).unwrap()).unwrap();
            match index {
                1 => {
                    // Original transmitter committed but its result never reached the capture client.
                    client
                        .call("mother_propose_memory", &pending["request"])
                        .unwrap();
                }
                3 => {
                    pending["request"]["record"]["title"] = json!("Different original submission");
                    client
                        .call("mother_propose_memory", &pending["request"])
                        .unwrap();
                    let failed = status(&policy, "retry", id);
                    assert_eq!(failed["status"], "failed");
                    assert_eq!(failed["errorCode"], "OPERATION_REUSED");
                    continue;
                }
                4 => {
                    // A journaled correction whose target no longer exists.
                    pending["request"]["id"] = json!(Uuid::new_v4().to_string());
                    pending["request"]["expectedRevision"] = json!(1);
                    fs::write(&journal, serde_json::to_vec(&pending).unwrap()).unwrap();
                    let failed = status(&policy, "retry", id);
                    assert_eq!(failed["status"], "failed");
                    assert_eq!(failed["errorCode"], "NOT_FOUND");
                    assert_eq!(status(&policy, "status", id), failed);
                    continue;
                }
                _ => assert_eq!(status(&policy, "status", id)["status"], "queued"),
            }
            let review = status(&policy, "retry", id);
            assert_eq!(review["status"], "awaiting_review");
            assert_eq!(status(&policy, "retry", id), review);
            let review_id = Uuid::new_v4().to_string();
            let dismissed = index == 2;
            let receipt = service
                .store()
                .v1_review_proposal(json!({
                    "workspaceId": workspace, "generation": generation,
                    "operationId": review_id, "proposalId": review["proposalId"],
                    "expectedRevision": 1, "action": if dismissed { "dismiss" } else { "save" }
                }))
                .unwrap();
            let closed = status(&policy, "retry", id);
            assert_eq!(closed["reviewOperationId"], review_id);
            assert_eq!(
                closed["status"],
                if dismissed { "dismissed" } else { "saved" }
            );
            assert_eq!(status(&policy, "retry", id), closed);
            let found = client
                .call(
                    "mother_search_context",
                    &json!({"mode":"literal","query":cases[index]}),
                )
                .unwrap();
            if dismissed {
                assert_eq!(found["records"], json!([]));
            } else {
                assert_eq!(closed["recordId"], receipt["receipt"]["record"]["id"]);
                assert_eq!(closed["recordRevision"], 1);
                assert_eq!(found["records"].as_array().unwrap().len(), 1);
                assert_eq!(found["records"][0]["id"], closed["recordId"]);
            }
        }
        let restricted = service
            .store()
            .v1_grant(
                "codex-synthetic",
                &["read".into(), "context:propose".into()],
                &ClientAccess {
                    mode: AccessMode::Shared,
                    subject: Some("synthetic-host".into()),
                    records: vec![],
                },
                None,
            )
            .unwrap();
        fs::write(
            &config_path,
            serde_json::to_vec(&ServiceClientConfig {
                token: restricted.token,
                ..config
            })
            .unwrap(),
        )
        .unwrap();
        let hidden = status(&policy, "retry", &ids[0]);
        assert_eq!(hidden["status"], "unconfirmed");
        assert!(hidden.get("recordId").is_none());
    });
}
