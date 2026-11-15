#[cfg(feature = "legacy-sqlite")]
use mother::{access::AccessMode, store::Store};
use mother::{access::ClientAccess, contracts::Auth, garden::store::GardenStore};
use serde_json::{Value, json};
#[cfg(feature = "legacy-sqlite")]
use std::path::Path;
use uuid::Uuid;

enum Backend {
    #[cfg(feature = "legacy-sqlite")]
    Sqlite(Store),
    Native(GardenStore),
}
impl Backend {
    fn grant(&mut self, name: &str, scopes: &[&str], access: ClientAccess) -> Auth {
        let scopes = scopes.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        match self {
            Self::Native(s) => s.v1_grant(name, &scopes, &access, None).unwrap(),
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(s) => {
                let grant = s.grant_with_access(name, &scopes, &access).unwrap();
                Auth {
                    client_id: name.into(),
                    token: grant["token"].as_str().unwrap().into(),
                }
            }
        }
    }
    fn writer(&mut self) -> Auth {
        self.grant(
            "writer",
            &[
                "read",
                "memory:write",
                "decision:write",
                "task:write",
                "checkpoint:write",
                "context:propose",
            ],
            ClientAccess::default(),
        )
    }
    fn call(&mut self, auth: &Auth, tool: &str, args: Value) -> mother::error::Result<Value> {
        match self {
            Self::Native(s) => s.v1_call(auth, tool, args),
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(s) => match tool {
                "mother_get_context" => s.context(auth, args),
                "mother_write_record" | "mother_checkpoint" => s.write(auth, args),
                "mother_get_record" => s.get(auth, args["id"].as_str().unwrap()),
                "mother_get_receipt" => s.receipt(auth, args["operationId"].as_str().unwrap()),
                "mother_get_record_history" => s.history(auth, args),
                "mother_propose_memory" => s.propose(auth, args),
                "mother_get_proposal" => s.get_proposal(auth, args["id"].as_str().unwrap()),
                "mother_pin_record" => s.pin(auth, args),
                "mother_assign_task" => s.assign(auth, args),
                _ => panic!("Unsupported boundary tool {tool}"),
            },
        }
    }
    fn review(&mut self, args: Value) -> mother::error::Result<Value> {
        match self {
            Self::Native(s) => s.v1_review_proposal(args),
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(s) => s.review_proposal(args),
        }
    }
    fn inbox_get(&self, id: &str) -> Value {
        match self {
            Self::Native(s) => s.v1_inbox_get(id).unwrap(),
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(s) => s.inbox_get(id).unwrap(),
        }
    }
    #[cfg(feature = "legacy-sqlite")]
    fn revoke(&mut self, id: &str) {
        match self {
            Self::Native(s) => s.revoke(id).unwrap(),
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(s) => s.revoke(id).unwrap(),
        }
    }
    #[cfg(feature = "legacy-sqlite")]
    fn set_scopes(&mut self, owned_root: &Path, id: &str, scopes: &[&str]) {
        let scopes = scopes.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        match self {
            Self::Native(s) => {
                s.v1_grant(id, &scopes, &ClientAccess::default(), None)
                    .unwrap();
            }
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(_) => {
                // The legacy owner API provisions new identities. This fixture
                // withdraws scopes only in its own synthetic temporary store.
                let db = rusqlite::Connection::open(owned_root.join("legacy.sqlite")).unwrap();
                assert_eq!(
                    db.execute(
                        "UPDATE clients SET scopes=?1 WHERE id=?2",
                        rusqlite::params![serde_json::to_string(&scopes).unwrap(), id]
                    )
                    .unwrap(),
                    1
                );
            }
        }
    }
    fn export(&mut self) -> Value {
        match self {
            Self::Native(s) => s.v1_export().unwrap(),
            #[cfg(feature = "legacy-sqlite")]
            Self::Sqlite(s) => s.export().unwrap(),
        }
    }
    fn request(&mut self, auth: &Auth, record: Value) -> Value {
        let w = self.call(auth, "mother_get_context", json!({})).unwrap()["workspace"].clone();
        json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})
    }
    fn proposal(&mut self, auth: &Auth, record: Value) -> Value {
        let mut request = self.request(auth, record);
        request["source"] = json!("Synthetic explicit owner statement");
        request["basis"] = json!("owner_statement");
        request
    }
}
#[cfg(feature = "legacy-sqlite")]
fn backends(dir: &Path) -> [Backend; 2] {
    let sqlite =
        Store::initialize(&dir.join("legacy.sqlite"), "Synthetic explicit persona").unwrap();
    let native = GardenStore::initialize(
        dir.join("native.redb"),
        &Uuid::new_v4().to_string(),
        &Uuid::new_v4().to_string(),
    )
    .unwrap();
    native.v1_initialize("Synthetic explicit persona").unwrap();
    [Backend::Sqlite(sqlite), Backend::Native(native)]
}
fn memory(text: &str) -> Value {
    json!({"kind":"memory","title":"Synthetic preference","content":text,"status":"active","basis":"owner_statement"})
}
fn task() -> Value {
    json!({"kind":"task","title":"Synthetic task","content":"Do explicit work","status":"todo"})
}
#[cfg(feature = "legacy-sqlite")]
fn checkpoint(title: &str) -> Value {
    json!({"kind":"checkpoint","title":title,"content":"Synthetic handoff","status":"active","nextStep":"Inspect evidence"})
}
fn review(proposal: &Value, action: &str) -> Value {
    json!({"workspaceId":proposal["workspaceId"],"generation":proposal["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":proposal["id"],"expectedRevision":1,"action":action})
}

#[test]
#[cfg(feature = "legacy-sqlite")]
fn both_backends_recheck_current_proposer_grant_and_target_revision_before_approval() {
    let dir = tempfile::tempdir().unwrap();
    for mut backend in backends(dir.path()) {
        let writer = backend.writer();
        let proposer = backend.grant(
            "proposer",
            &["read", "context:propose"],
            ClientAccess::default(),
        );
        let request = backend.request(&writer, memory("Original"));
        let original = backend
            .call(&writer, "mother_write_record", request)
            .unwrap();
        let mut correction = backend.proposal(&proposer, memory("Proposed correction"));
        correction["id"] = original["record"]["id"].clone();
        correction["expectedRevision"] = json!(1);
        let proposed = backend
            .call(&proposer, "mother_propose_memory", correction)
            .unwrap()["proposal"]
            .clone();
        let mut intervening = backend.request(&writer, memory("New canonical fact"));
        intervening["id"] = original["record"]["id"].clone();
        intervening["expectedRevision"] = json!(1);
        let current = backend
            .call(&writer, "mother_write_record", intervening)
            .unwrap();
        assert_eq!(
            backend
                .review(review(&proposed, "save"))
                .unwrap_err()
                .code(),
            "CONFLICT"
        );
        assert_eq!(
            backend.inbox_get(proposed["id"].as_str().unwrap())["status"],
            "pending"
        );
        assert_eq!(
            backend
                .call(
                    &writer,
                    "mother_get_record",
                    json!({"id":original["record"]["id"]})
                )
                .unwrap(),
            current["record"]
        );
        backend.review(review(&proposed, "dismiss")).unwrap();
        let request = backend.proposal(&proposer, memory("Revoked proposal"));
        let revoked = backend
            .call(&proposer, "mother_propose_memory", request)
            .unwrap()["proposal"]
            .clone();
        backend.revoke("proposer");
        assert_eq!(
            backend.review(review(&revoked, "save")).unwrap_err().code(),
            "UNAUTHORIZED"
        );
        assert_eq!(
            backend.review(review(&revoked, "dismiss")).unwrap()["status"],
            "dismissed"
        );
        let renewed = backend.grant(
            "scope-proposer",
            &["read", "context:propose"],
            ClientAccess::default(),
        );
        let request = backend.proposal(&renewed, memory("Downgraded proposal"));
        let downgraded = backend
            .call(&renewed, "mother_propose_memory", request)
            .unwrap()["proposal"]
            .clone();
        backend.set_scopes(dir.path(), "scope-proposer", &["read"]);
        assert_eq!(
            backend
                .review(review(&downgraded, "save"))
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
        let dismiss = review(&downgraded, "dismiss");
        let receipt = backend.review(dismiss.clone()).unwrap();
        assert_eq!(backend.review(dismiss.clone()).unwrap(), receipt);
        let mut reused = dismiss;
        reused["action"] = json!("save");
        assert_eq!(
            backend.review(reused).unwrap_err().code(),
            "OPERATION_REUSED"
        );
    }
}

#[test]
#[cfg(feature = "legacy-sqlite")]
fn both_backends_owner_edits_validate_references_and_keep_failed_approvals_atomic() {
    let dir = tempfile::tempdir().unwrap();
    for mut backend in backends(dir.path()) {
        let writer = backend.writer();
        let request = backend.proposal(&writer, memory("Submitted text"));
        let proposal = backend
            .call(&writer, "mother_propose_memory", request.clone())
            .unwrap()["proposal"]
            .clone();
        let mut invalid = review(&proposal, "save");
        let mut edited = memory("Owner edited text");
        edited["references"] = json!([Uuid::new_v4().to_string()]);
        invalid["record"] = edited;
        let before = backend.export();
        assert_eq!(
            backend.review(invalid).unwrap_err().code(),
            "REFERENCE_NOT_FOUND"
        );
        let after = backend.export();
        assert_eq!(before["records"], after["records"]);
        assert_eq!(before["operations"], after["operations"]);
        assert_eq!(before["inbox"], after["inbox"]);
        let mut accepted = review(&proposal, "save");
        accepted["record"] = memory("Owner edited text");
        let receipt = backend.review(accepted.clone()).unwrap();
        assert_eq!(receipt["receipt"]["record"]["content"], "Owner edited text");
        assert_eq!(receipt["receipt"]["record"]["actor"], "@owner-review");
        assert_eq!(backend.review(accepted).unwrap(), receipt);
        let full = backend.inbox_get(proposal["id"].as_str().unwrap());
        assert_eq!(full["record"]["content"], "Submitted text");
        assert_eq!(full["source"], "Synthetic explicit owner statement");
        assert_eq!(
            backend
                .call(
                    &writer,
                    "mother_get_receipt",
                    json!({"operationId":request["operationId"]})
                )
                .unwrap()["proposal"]["record"]["content"],
            "Submitted text"
        );
    }
}

#[test]
#[cfg(feature = "legacy-sqlite")]
fn both_backends_checkpoint_supersession_retires_once_and_conflicts_roll_back_every_receipt() {
    let dir = tempfile::tempdir().unwrap();
    for mut backend in backends(dir.path()) {
        let auth = backend.writer();
        let first_request = backend.request(&auth, checkpoint("First"));
        let first = backend
            .call(&auth, "mother_checkpoint", first_request)
            .unwrap();
        let mut next = backend.request(&auth, checkpoint("Next"));
        next["supersedes"] = json!({"id":first["record"]["id"],"expectedRevision":1});
        let receipt = backend
            .call(&auth, "mother_checkpoint", next.clone())
            .unwrap();
        assert_eq!(
            backend.call(&auth, "mother_checkpoint", next).unwrap(),
            receipt
        );
        let old = backend
            .call(
                &auth,
                "mother_get_record",
                json!({"id":first["record"]["id"]}),
            )
            .unwrap();
        assert_eq!(old["status"], "superseded");
        assert_eq!(old["revision"], 2);
        let history = backend
            .call(&auth, "mother_get_record_history", json!({"id":old["id"]}))
            .unwrap();
        assert_eq!(history["revisions"][0]["changed"], json!(["status"]));
        let before = backend.export();
        let mut failed = backend.request(&auth, checkpoint("Must roll back"));
        failed["supersedes"] = json!({"id":old["id"],"expectedRevision":1});
        assert_eq!(
            backend
                .call(&auth, "mother_checkpoint", failed.clone())
                .unwrap_err()
                .code(),
            "CONFLICT"
        );
        assert_eq!(
            backend
                .call(
                    &auth,
                    "mother_get_receipt",
                    json!({"operationId":failed["operationId"]})
                )
                .unwrap_err()
                .code(),
            "NOT_FOUND"
        );
        let after = backend.export();
        assert_eq!(before["records"], after["records"]);
        assert_eq!(before["operations"], after["operations"]);
    }
}

#[test]
#[cfg(feature = "legacy-sqlite")]
fn both_backends_scrub_shared_references_and_never_implicitly_share_saved_new_proposals() {
    let dir = tempfile::tempdir().unwrap();
    for mut backend in backends(dir.path()) {
        let writer = backend.writer();
        let request = backend.request(&writer, memory("Hidden fact"));
        let hidden = backend
            .call(&writer, "mother_write_record", request)
            .unwrap();
        let mut visible = task();
        visible["references"] = json!([hidden["record"]["id"]]);
        let request = backend.request(&writer, visible);
        let visible = backend
            .call(&writer, "mother_write_record", request)
            .unwrap();
        let shared = backend.grant(
            "shared",
            &["read", "context:propose"],
            ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("synthetic-shared".into()),
                records: vec![visible["record"]["id"].as_str().unwrap().into()],
            },
        );
        assert_eq!(
            backend
                .call(
                    &shared,
                    "mother_get_record",
                    json!({"id":visible["record"]["id"]})
                )
                .unwrap()["references"],
            json!([])
        );
        let mut correction = backend.proposal(&shared, task());
        correction["record"]["status"] = json!("done");
        correction["id"] = visible["record"]["id"].clone();
        correction["expectedRevision"] = json!(1);
        let proposed = backend
            .call(&shared, "mother_propose_memory", correction.clone())
            .unwrap();
        assert_eq!(proposed["proposal"]["before"]["references"], json!([]));
        assert_eq!(
            backend
                .call(
                    &shared,
                    "mother_get_receipt",
                    json!({"operationId":correction["operationId"]})
                )
                .unwrap()["proposal"]["before"]["references"],
            json!([])
        );
        assert_eq!(
            backend.export()["inbox"]["proposals"][0]["before"]["references"],
            json!([hidden["record"]["id"]])
        );
        backend
            .review(review(
                &backend.inbox_get(proposed["proposal"]["id"].as_str().unwrap()),
                "dismiss",
            ))
            .unwrap();
        let request = backend.proposal(&shared, memory("New unshared fact"));
        let proposal = backend
            .call(&shared, "mother_propose_memory", request.clone())
            .unwrap()["proposal"]
            .clone();
        let accepted = backend.review(review(&proposal, "save")).unwrap();
        let canonical = accepted["receipt"]["record"]["id"].clone();
        assert_eq!(
            backend
                .call(&shared, "mother_get_record", json!({"id":canonical}))
                .unwrap_err()
                .code(),
            "NOT_FOUND"
        );
        assert_eq!(
            backend
                .call(&shared, "mother_get_proposal", json!({"id":proposal["id"]}))
                .unwrap_err()
                .code(),
            "NOT_FOUND"
        );
        assert_eq!(
            backend
                .call(&shared, "mother_get_context", json!({}))
                .unwrap()["inventory"]["totalRecords"],
            1
        );
        // Admission is immutable; its historic payload grants no new canonical read.
        let admission = backend
            .call(
                &shared,
                "mother_get_receipt",
                json!({"operationId":request["operationId"]}),
            )
            .unwrap();
        assert_eq!(admission["proposal"]["status"], "pending");
    }
}

#[test]
fn native_full_backup_preserves_all_v1_auxiliary_state_and_blocks_old_generation_replays() {
    let dir = tempfile::tempdir().unwrap();
    let native = GardenStore::initialize(
        dir.path().join("source.redb"),
        &Uuid::new_v4().to_string(),
        &Uuid::new_v4().to_string(),
    )
    .unwrap();
    native.v1_initialize("Synthetic explicit persona").unwrap();
    let mut backend = Backend::Native(native);
    let auth = backend.writer();
    let mut original_draft = memory("Original retained source");
    original_draft["sources"] =
        json!([{"label":"Historical synthetic source","uri":"https://example.invalid/historical"}]);
    let request = backend.request(&auth, original_draft);
    let original = backend
        .call(&auth, "mother_write_record", request.clone())
        .unwrap();
    let mut correction = backend.request(&auth, memory("Corrected retained source"));
    correction["id"] = original["record"]["id"].clone();
    correction["expectedRevision"] = json!(1);
    let current = backend
        .call(&auth, "mother_write_record", correction)
        .unwrap();
    backend
        .call(
            &auth,
            "mother_pin_record",
            json!({"id":current["record"]["id"],"pinned":true}),
        )
        .unwrap();
    let task_request = backend.request(&auth, task());
    let task = backend
        .call(&auth, "mother_write_record", task_request)
        .unwrap();
    backend
        .call(
            &auth,
            "mother_assign_task",
            json!({"id":task["record"]["id"],"assignee":"writer"}),
        )
        .unwrap();
    let submitted = backend.proposal(&auth, memory("Saved original proposal"));
    let saved = backend
        .call(&auth, "mother_propose_memory", submitted.clone())
        .unwrap()["proposal"]
        .clone();
    let saved_review = review(&saved, "save");
    backend.review(saved_review).unwrap();
    let mut pending_draft = memory("Pending original proposal");
    pending_draft["sources"] =
        json!([{"label":"Pending synthetic source","uri":"https://example.invalid/pending"}]);
    let pending_request = backend.proposal(&auth, pending_draft);
    let pending = backend
        .call(&auth, "mother_propose_memory", pending_request)
        .unwrap()["proposal"]
        .clone();
    let before = backend.export();
    assert_eq!(
        before["omittedExternalSources"],
        json!([
            "https://example.invalid/historical",
            "https://example.invalid/pending"
        ])
    );
    let bundle = dir.path().join("backup");
    #[cfg(not(feature = "legacy-sqlite"))]
    let Backend::Native(source) = backend;
    #[cfg(feature = "legacy-sqlite")]
    let source = match backend {
        Backend::Native(store) => store,
        #[cfg(feature = "legacy-sqlite")]
        Backend::Sqlite(_) => unreachable!("This fixture explicitly creates a native backend"),
    };
    source.backup_to(&bundle).unwrap();
    let restored = GardenStore::restore_backup(&bundle, &dir.path().join("restored.redb")).unwrap();
    assert_eq!(
        restored.v1_authorize(&auth, "read").unwrap_err().code(),
        "UNAUTHORIZED"
    );
    let mut restored = Backend::Native(restored);
    let fresh = restored.writer();
    let after = restored.export();
    for field in [
        "persona",
        "records",
        "operations",
        "inbox",
        "pins",
        "assignments",
        "omittedExternalSources",
    ] {
        assert_eq!(before[field], after[field], "native restore lost {field}");
    }
    assert_eq!(before["workspace"]["id"], after["workspace"]["id"]);
    assert_ne!(
        before["workspace"]["generation"],
        after["workspace"]["generation"]
    );
    let context = restored
        .call(&fresh, "mother_get_context", json!({}))
        .unwrap();
    assert_eq!(context["pinned"][0]["content"], "Corrected retained source");
    assert_eq!(context["assignedToYou"][0]["id"], task["record"]["id"]);
    let history = restored
        .call(
            &fresh,
            "mother_get_record_history",
            json!({"id":original["record"]["id"]}),
        )
        .unwrap();
    assert_eq!(history["returnedRevisions"], 2);
    assert!(
        history["revisions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["currentGeneration"] == false)
    );
    assert_eq!(
        restored
            .call(
                &fresh,
                "mother_get_record_history",
                json!({"id":original["record"]["id"],"revision":1})
            )
            .unwrap()["record"],
        original["record"]
    );
    assert_eq!(
        restored
            .call(
                &fresh,
                "mother_get_receipt",
                json!({"operationId":request["operationId"]})
            )
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    assert_eq!(
        restored
            .call(&fresh, "mother_write_record", request)
            .unwrap_err()
            .code(),
        "STALE_GENERATION"
    );
    let mut stale_review = review(&pending, "save");
    stale_review["generation"] = after["workspace"]["generation"].clone();
    assert_eq!(
        restored.review(stale_review).unwrap_err().code(),
        "STALE_GENERATION"
    );
    assert_eq!(
        restored.inbox_get(pending["id"].as_str().unwrap())["status"],
        "pending"
    );
    // A new canonical write and another backup preserve the complete old audit trail.
    let request = restored.request(&fresh, memory("Post restore fact"));
    restored
        .call(&fresh, "mother_write_record", request)
        .unwrap();
    #[cfg(not(feature = "legacy-sqlite"))]
    let Backend::Native(restored) = restored;
    #[cfg(feature = "legacy-sqlite")]
    let restored = match restored {
        Backend::Native(store) => store,
        #[cfg(feature = "legacy-sqlite")]
        Backend::Sqlite(_) => unreachable!("This fixture explicitly creates a native backend"),
    };
    restored
        .backup_to(&dir.path().join("second-backup"))
        .unwrap();
}
