use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::store::GardenStore,
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use uuid::Uuid;

struct Fixture {
    root: tempfile::TempDir,
    db: PathBuf,
    state: PathBuf,
    config: PathBuf,
    workspace: String,
    generation: String,
    owner: Auth,
    witness: Auth,
}

fn private(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    #[cfg(windows)]
    assert!(path.is_dir());
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        private(root.path());
        let state = root.path().join("state");
        fs::create_dir(&state).unwrap();
        private(&state);
        let db = root.path().join("fixture.redb");
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(&db, &workspace, &generation).unwrap();
        store.v1_initialize("Synthetic gardening owner").unwrap();
        let grant = |id: &str| {
            store
                .v1_grant(
                    id,
                    &["read".into(), "memory:write".into(), "task:write".into()],
                    &ClientAccess::default(),
                    None,
                )
                .unwrap()
        };
        let owner = grant("fictional-owner");
        let witness = grant("fictional-witness");
        let config = root.path().join("direct.private.json");
        Self {
            root,
            db,
            state,
            config,
            workspace,
            generation,
            owner,
            witness,
        }
    }

    fn store(&self) -> GardenStore {
        GardenStore::open_active(&self.db).unwrap()
    }

    fn reviewer(&self, records: &[String]) -> Auth {
        let auth = self
            .store()
            .v1_grant(
                "fictional-reviewer",
                &["read".into(), "context:propose".into()],
                &ClientAccess {
                    mode: AccessMode::Shared,
                    subject: Some("test-agent".into()),
                    records: records.to_vec(),
                },
                None,
            )
            .unwrap();
        fs::write(
            &self.config,
            json!({"version":2,"backend":"garden_redb","databasePath":self.db,
            "clientId":auth.client_id,"token":auth.token})
            .to_string(),
        )
        .unwrap();
        auth
    }

    fn record(&self, actor: &Auth, draft: Value) -> String {
        self.store().v1_call(actor,"mother_write_record",json!({
            "workspaceId":self.workspace,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),
            "expectedRevision":0,"record":draft
        })).unwrap()["record"]["id"].as_str().unwrap().to_owned()
    }

    fn submit(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(["garden", "tend", "--config"])
            .arg(&self.config)
            .arg("--state")
            .arg(&self.state)
            .args(["--as-of", "2090-01-01"])
            .args(args)
            .output()
            .unwrap()
    }

    fn result(&self, args: &[&str]) -> Value {
        let output = self.submit(args);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        serde_json::from_slice(&output.stdout).unwrap()
    }

    fn review(&self, proposal: &Value, action: &str) -> mother::error::Result<Value> {
        self.store().v1_review_proposal(json!({"workspaceId":self.workspace,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),
            "proposalId":proposal["proposalId"],"expectedRevision":1,"action":action}))
    }

    fn get(&self, auth: &Auth, id: &str) -> Value {
        self.store()
            .v1_call(auth, "mother_get_record", json!({"id":id}))
            .unwrap()
    }
}

fn memory(title: &str, content: &str) -> Value {
    json!({"kind":"memory","title":title,"content":content,"status":"active","basis":"owner_statement"})
}

fn task(title: &str) -> Value {
    json!({"kind":"task","title":title,"content":"A fictional open follow-up.","status":"doing"})
}

fn finding<'a>(report: &'a Value, kind: &str) -> &'a Value {
    report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["kind"] == kind)
        .unwrap()
}

#[test]
fn reviews_four_finding_types_without_exposing_hidden_sources_and_preserves_originals() {
    let f = Fixture::new();
    let overlap_a = f.record(
        &f.owner,
        memory(
            "River footbridge",
            "The blue lantern shone above the timber river footbridge.",
        ),
    );
    let overlap_b = f.record(
        &f.witness,
        memory(
            "River footbridge",
            "The small blue lantern shone above the timber river footbridge.",
        ),
    );
    let opposite_a = f.record(&f.owner, memory("West Hall", "The west hall was open."));
    let opposite_b = f.record(
        &f.witness,
        memory("West Hall", "The west hall was not open."),
    );
    let hidden = f.record(
        &f.owner,
        memory(
            "SECRET TEST TARGET",
            "Unseen proof text. HIDDEN SYNTHETIC PAYLOAD.",
        ),
    );
    let mut milestone = memory("Workshop recap", "Milestone: SECRET TEST TARGET");
    milestone["references"] = json!([hidden]);
    let gap = f.record(&f.owner, milestone);
    let support = f.record(
        &f.owner,
        memory("Published drawings", "The fictional drawings were filed."),
    );
    let mut linked = memory("Drawing recap", "Milestone: Published drawings");
    linked["references"] = json!([support]);
    let supported = f.record(&f.owner, linked);
    let stale = f.record(&f.owner, task("Ask about the lantern loan"));
    let sources = [
        overlap_a.clone(),
        overlap_b.clone(),
        opposite_a.clone(),
        opposite_b.clone(),
        gap.clone(),
        stale.clone(),
        support,
        supported,
    ];
    let reviewer = f.reviewer(&sources);
    let original = f.get(&f.owner, &opposite_a);
    let snapshots: Vec<_> = sources.iter().map(|id| f.get(&f.owner, id)).collect();
    f.store()
        .v1_call(
            &f.owner,
            "mother_pin_record",
            json!({"id":opposite_a,"pinned":true}),
        )
        .unwrap();
    let before = f.store().v1_export().unwrap();
    let report = f.result(&[]);
    assert_eq!(report["reviewingAgent"], "fictional-reviewer");
    assert_eq!(report["findings"].as_array().unwrap().len(), 4);
    assert_eq!(report["hasMoreFindings"], false);
    let visible = report.to_string();
    assert!(!visible.contains(&hidden));
    for text in [
        "SECRET TEST TARGET",
        "HIDDEN SYNTHETIC PAYLOAD",
        "Unseen proof text",
    ] {
        assert!(!visible.contains(text));
    }
    let overlap = finding(&report, "overlapping_accounts");
    let contradiction = finding(&report, "possible_contradiction");
    let milestone = finding(&report, "possible_milestone_gap");
    let follow_up = finding(&report, "stale_follow_up");
    assert!(
        overlap["evidence"]
            .to_string()
            .contains("fictional-witness")
    );
    assert!(overlap["evidence"].to_string().contains("fictional-owner"));
    assert!(overlap["evidence"][0]["recordedAt"].is_string());
    assert_eq!(milestone["evidence"][0]["id"], gap);
    assert_eq!(follow_up["evidence"][0]["id"], stale);
    assert!(
        milestone["reason"]
            .as_str()
            .unwrap()
            .contains("reviewed slice")
    );
    assert_eq!(contradiction["basis"], "suggested_interpretation");
    assert_eq!(contradiction["evidence"].as_array().unwrap().len(), 2);
    let after = f.store().v1_export().unwrap();
    for field in ["workspace", "records", "pins", "operations"] {
        assert_eq!(after[field], before[field]);
    }

    let local_id = milestone["id"].as_str().unwrap();
    assert_eq!(f.result(&["--dismiss", local_id])["status"], "dismissed");
    assert_eq!(f.result(&["--dismiss", local_id])["status"], "dismissed");
    let overlap_id = overlap["id"].as_str().unwrap();
    let pending = f.result(&["--propose", overlap_id]);
    assert_eq!(pending["status"], "admission_recorded");
    fs::remove_file(f.state.join(format!("admitted-{overlap_id}.json"))).unwrap();
    assert_eq!(
        f.result(&["--propose", overlap_id])["proposalId"],
        pending["proposalId"]
    );
    assert_eq!(
        f.store().v1_inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        f.review(&pending, "dismiss").unwrap()["status"],
        "dismissed"
    );
    let next = f.result(&[]);
    assert!(!next.to_string().contains(overlap_id));
    assert!(!next.to_string().contains(local_id));

    let contradiction_id = contradiction["id"].as_str().unwrap();
    let pending = f.result(&["--propose", contradiction_id]);
    let detail = f
        .store()
        .v1_inbox_get(pending["proposalId"].as_str().unwrap())
        .unwrap();
    assert!(
        detail["source"]
            .as_str()
            .unwrap()
            .contains(&format!("{opposite_a}@1"))
    );
    assert_eq!(detail["actor"], "fictional-reviewer");
    let approved = f.review(&pending, "save").unwrap();
    let annotation = &approved["receipt"]["record"];
    assert_eq!(annotation["basis"], "model_inference");
    assert!(annotation["references"].to_string().contains(&opposite_b));
    assert!(
        annotation["content"]
            .as_str()
            .unwrap()
            .contains("Suggested interpretation")
    );
    assert_eq!(f.get(&f.owner, &opposite_a), original);

    let stale_id = follow_up["id"].as_str().unwrap();
    let pending = f.result(&["--propose", stale_id]);
    f.store().v1_call(&f.owner,"mother_write_record",json!({"workspaceId":f.workspace,"generation":f.generation,
        "operationId":Uuid::new_v4().to_string(),"id":stale,"expectedRevision":1,
        "record":{"kind":"task","title":"Ask about the lantern loan","content":"Updated fictional wording.","status":"doing"}
    })).unwrap();
    let detail = f
        .store()
        .v1_inbox_get(pending["proposalId"].as_str().unwrap())
        .unwrap();
    assert_eq!(detail["conflict"], true);
    assert!(!detail.to_string().contains(&stale));
    assert_eq!(f.review(&pending, "save").unwrap_err().code(), "CONFLICT");
    assert_eq!(
        f.review(&pending, "dismiss").unwrap()["status"],
        "dismissed"
    );
    let historical_receipt = f
        .store()
        .v1_call(
            &reviewer,
            "mother_get_receipt",
            json!({"operationId":stale_id}),
        )
        .unwrap();
    assert!(!historical_receipt.to_string().contains(&stale));

    for snapshot in snapshots {
        let id = snapshot["id"].as_str().unwrap();
        if id != stale {
            assert_eq!(f.get(&f.owner, id), snapshot);
        }
    }
}

#[test]
fn accessed_sources_are_rechecked_after_a_grant_change() {
    let f = Fixture::new();
    let a = f.record(&f.owner, memory("Lantern plans", "The lantern was ready."));
    let b = f.record(
        &f.witness,
        memory("Lantern plans", "The lantern was not ready."),
    );
    f.reviewer(&[a.clone(), b.clone()]);
    let report = f.result(&[]);
    let candidate = finding(&report, "possible_contradiction")["id"]
        .as_str()
        .unwrap();
    let pending = f.result(&["--propose", candidate]);
    let restricted = f.reviewer(&[a]);
    let result = f.result(&[]);
    assert_eq!(result["findings"], json!([]));
    assert!(!result.to_string().contains(&b));
    let details = f
        .store()
        .v1_inbox_get(pending["proposalId"].as_str().unwrap())
        .unwrap();
    assert_eq!(details["conflict"], true);
    assert!(!details.to_string().contains(&b));
    assert!(f.review(&pending, "save").is_err());
    let receipt = f
        .store()
        .v1_call(
            &restricted,
            "mother_get_receipt",
            json!({"operationId":candidate}),
        )
        .unwrap();
    assert!(!receipt.to_string().contains(&b));
    assert_eq!(
        f.review(&pending, "dismiss").unwrap()["status"],
        "dismissed"
    );
}

#[test]
fn dispositions_reveal_the_next_candidates_and_reject_wrong_state() {
    let f = Fixture::new();
    let sources = (0..14)
        .map(|i| f.record(&f.owner, task(&format!("Synthetic follow-up {i}"))))
        .collect::<Vec<_>>();
    f.reviewer(&sources);
    let first = f.result(&[]);
    assert_eq!(first["findings"].as_array().unwrap().len(), 12);
    assert_eq!(first["hasMoreFindings"], true);
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(f.state.join("gardening.lock"))
        .unwrap();
    fs2::FileExt::lock_exclusive(&lock).unwrap();
    let busy = f.submit(&[]);
    assert!(!busy.status.success());
    assert!(String::from_utf8_lossy(&busy.stderr).contains("GARDENING_BUSY"));
    drop(lock);
    for item in first["findings"].as_array().unwrap() {
        f.result(&["--dismiss", item["id"].as_str().unwrap()]);
    }
    let next = f.result(&[]);
    assert_eq!(next["findings"].as_array().unwrap().len(), 2);
    assert_eq!(next["hasMoreFindings"], false);
    fs::write(
        f.state.join("identity.json"),
        r#"{"version":1,"workspaceId":"different"}"#,
    )
    .unwrap();
    let blocked = f.submit(&[]);
    assert!(!blocked.status.success());
    assert!(blocked.stdout.is_empty());
    assert!(f.root.path().is_dir());
}

#[test]
fn owner_service_requires_and_applies_revision_bound_review() {
    use mother::service::{LocalService, ServiceClientConfig};
    use std::sync::atomic::{AtomicBool, Ordering};

    let f = Fixture::new();
    let source = f.record(&f.owner, task("Check the paper lantern colours"));
    let reviewer = f.reviewer(&[source]);
    let endpoint = f.root.path().join("owner.json");
    let service = LocalService::bind(&f.db, &endpoint).unwrap();
    fs::write(
        &f.config,
        serde_json::to_vec(&ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: endpoint,
            workspace_id: f.workspace.clone(),
            generation: f.generation.clone(),
            client_id: reviewer.client_id,
            token: reviewer.token,
        })
        .unwrap(),
    )
    .unwrap();
    let stopped = AtomicBool::new(false);
    let pending = std::thread::scope(|scope| {
        struct Stop<'a>(&'a AtomicBool);
        impl Drop for Stop<'_> {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Release);
            }
        }
        scope.spawn(|| service.serve(&stopped).unwrap());
        let _stop = Stop(&stopped);
        let report = f.result(&[]);
        let id = finding(&report, "stale_follow_up")["id"].as_str().unwrap();
        let pending = f.result(&["--propose", id]);
        assert_eq!(f.result(&[])["findings"], json!([]));
        pending
    });
    drop(service);
    assert_eq!(f.review(&pending, "save").unwrap()["status"], "saved");
}
