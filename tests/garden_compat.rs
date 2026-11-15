use mother::{
    access::ClientAccess,
    contracts::Auth,
    garden::{access::GardenAccess, store::GardenStore},
};
use serde_json::{Value, json};
use uuid::Uuid;

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/v1.json")).unwrap()
}
fn grant(s: &GardenStore) -> Auth {
    s.v1_grant(
        "fixture-client",
        &[
            "read".into(),
            "memory:write".into(),
            "task:write".into(),
            "checkpoint:write".into(),
            "decision:write".into(),
            "artifact:write".into(),
            "action:write".into(),
            "context:propose".into(),
        ],
        &ClientAccess::default(),
        None,
    )
    .unwrap()
}

#[test]
fn native_import_replays_typescript_receipts_and_preserves_portable_state() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let f = fixture();
    let b = &f["bundle"];
    let s = GardenStore::initialize(
        dir.path().join("native.redb"),
        b["workspace"]["id"].as_str().unwrap(),
        b["workspace"]["generation"].as_str().unwrap(),
    )
    .unwrap();
    s.import_v1_bundle(b.clone()).unwrap();
    let auth = grant(&s);
    for (request, receipt) in f["requests"]
        .as_array()
        .unwrap()
        .iter()
        .zip(f["receipts"].as_array().unwrap())
    {
        assert_eq!(
            s.v1_call(&auth, "mother_write_record", request.clone())
                .unwrap(),
            *receipt
        );
    }
    let mut exported = s.v1_export().unwrap();
    exported["exportedAt"] = b["exportedAt"].clone();
    assert_eq!(exported, *b);
    for r in b["records"].as_array().unwrap() {
        assert_eq!(
            s.v1_call(&auth, "mother_get_record", json!({"id":r["id"]}))
                .unwrap(),
            *r
        );
    }
    drop(s);
    let s = GardenStore::open(dir.path().join("native.redb")).unwrap();
    assert!(s.v1_initialized().unwrap());
    assert!(!s.v1_export().unwrap().to_string().contains(&auth.token));
}

#[test]
fn native_writes_conflict_retry_shared_visibility_and_proposal_review() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let s =
        GardenStore::initialize(dir.path().join("native.redb"), &workspace, &generation).unwrap();
    s.v1_initialize("Synthetic explicit soul").unwrap();
    let auth = grant(&s);
    let request = json!({"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Tea","content":"Green tea","status":"active","basis":"owner_statement"}});
    let receipt = s
        .v1_call(&auth, "mother_write_record", request.clone())
        .unwrap();
    assert_eq!(
        s.v1_call(&auth, "mother_write_record", request.clone())
            .unwrap(),
        receipt
    );
    let mut changed = request.clone();
    changed["record"]["content"] = json!("different");
    assert_eq!(
        s.v1_call(&auth, "mother_write_record", changed)
            .unwrap_err()
            .code(),
        "OPERATION_REUSED"
    );
    let mut stale = request.clone();
    stale["operationId"] = json!(Uuid::new_v4().to_string());
    stale["id"] = receipt["record"]["id"].clone();
    assert_eq!(
        s.v1_call(&auth, "mother_write_record", stale)
            .unwrap_err()
            .code(),
        "CONFLICT"
    );
    let shared = s
        .v1_grant(
            "shared",
            &["read".into(), "context:propose".into()],
            &mother::access::ClientAccess {
                mode: mother::access::AccessMode::Shared,
                subject: Some("synthetic".into()),
                records: vec![],
            },
            Some(&GardenAccess::default()),
        )
        .unwrap();
    let c = s.v1_call(&shared, "mother_get_context", json!({})).unwrap();
    assert!(c.get("persona").is_none());
    assert_eq!(c["inventory"]["totalRecords"], 0);
    assert_eq!(
        s.v1_call(
            &shared,
            "mother_get_record",
            json!({"id":receipt["record"]["id"]})
        )
        .unwrap_err()
        .code(),
        "NOT_FOUND"
    );
    let mut proposal = request;
    proposal["operationId"] = json!(Uuid::new_v4().to_string());
    proposal["record"]["content"] = json!("Another preference");
    proposal["source"] = json!("Synthetic user statement");
    proposal["basis"] = json!("owner_statement");
    let p = s.v1_call(&auth, "mother_propose_memory", proposal).unwrap();
    let before = s.v1_call(&auth, "mother_get_context", json!({})).unwrap();
    assert_eq!(before["inventory"]["totalRecords"], 1);
    let review = json!({"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"proposalId":p["proposal"]["id"],"expectedRevision":1,"action":"save"});
    let saved = s.v1_review_proposal(review.clone()).unwrap();
    assert_eq!(saved["status"], "saved");
    assert_eq!(s.v1_review_proposal(review).unwrap(), saved);
}

struct Backend(GardenStore);
impl Backend {
    fn call(&mut self, auth: &Auth, tool: &str, args: Value) -> mother::error::Result<Value> {
        if tool == "mother_search_context" {
            mother::garden::v1_search::NativeSearch::new(std::path::PathBuf::from(
                "synthetic-missing-model",
            ))
            .search(&self.0, auth, args)
        } else {
            self.0.v1_call(auth, tool, args)
        }
    }
    fn grant(&mut self, name: &str, scopes: &[String], access: &ClientAccess) -> Auth {
        self.0.v1_grant(name, scopes, access, None).unwrap()
    }
    fn review(&mut self, args: Value) -> mother::error::Result<Value> {
        self.0.v1_review_proposal(args)
    }
    fn inbox(&self) -> Value {
        self.0.v1_inbox_list().unwrap()
    }
    fn export(&mut self) -> Value {
        self.0.v1_export().unwrap()
    }
    fn revoke(&mut self, name: &str) {
        self.0.revoke(name).unwrap();
    }
}
fn backends(dir: &std::path::Path) -> [Backend; 1] {
    let f = fixture();
    let b = &f["bundle"];
    let native = GardenStore::initialize(
        dir.join("compat.redb"),
        b["workspace"]["id"].as_str().unwrap(),
        b["workspace"]["generation"].as_str().unwrap(),
    )
    .unwrap();
    native.import_v1_bundle(b.clone()).unwrap();
    [Backend(native)]
}
fn request(record: Value) -> Value {
    let b = fixture()["bundle"].clone();
    json!({"workspaceId":b["workspace"]["id"],"generation":b["workspace"]["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})
}
fn memory(content: &str) -> Value {
    json!({"kind":"memory","title":"Synthetic preference","content":content,"status":"active","basis":"owner_statement"})
}

#[test]
fn common_semantic_runner_covers_all_eleven_tools_and_current_grants() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    for mut backend in backends(dir.path()) {
        let auth = backend.grant(
            "parity-writer",
            &[
                "read".into(),
                "memory:write".into(),
                "decision:write".into(),
                "task:write".into(),
                "checkpoint:write".into(),
                "context:propose".into(),
            ],
            &ClientAccess::default(),
        );
        let initial = backend
            .call(&auth, "mother_get_context", json!({}))
            .unwrap();
        assert_eq!(initial["inventory"]["totalRecords"], 6);
        assert!(initial["persona"]["content"].is_string());
        assert!(
            initial["unsettledWork"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["kind"] == "action" && r["status"] == "unknown")
        );
        let write = request(memory("\u{feff} Green tea \u{00a0}"));
        let saved = backend
            .call(&auth, "mother_write_record", write.clone())
            .unwrap();
        let id = saved["record"]["id"].clone();
        assert_eq!(saved["record"]["content"], "Green tea");
        assert_eq!(saved["record"]["revision"], 1);
        assert_eq!(saved["record"]["actor"], "parity-writer");
        assert_eq!(
            backend
                .call(&auth, "mother_write_record", write.clone())
                .unwrap(),
            saved
        );
        assert_eq!(
            backend
                .call(
                    &auth,
                    "mother_get_receipt",
                    json!({"operationId":write["operationId"]})
                )
                .unwrap(),
            saved
        );
        assert_eq!(
            backend
                .call(&auth, "mother_get_record", json!({"id":id}))
                .unwrap(),
            saved["record"]
        );
        let mut reused = write.clone();
        reused["record"]["content"] = json!("Changed payload");
        assert_eq!(
            backend
                .call(&auth, "mother_write_record", reused)
                .unwrap_err()
                .code(),
            "OPERATION_REUSED"
        );
        let mut edit = request(memory("Black tea"));
        edit["id"] = id.clone();
        edit["expectedRevision"] = json!(1);
        let revised = backend
            .call(&auth, "mother_write_record", edit.clone())
            .unwrap();
        let mut stale = edit.clone();
        stale["operationId"] = json!(Uuid::new_v4().to_string());
        assert_eq!(
            backend
                .call(&auth, "mother_write_record", stale)
                .unwrap_err()
                .code(),
            "CONFLICT"
        );
        let history = backend
            .call(&auth, "mother_get_record_history", json!({"id":id}))
            .unwrap();
        assert_eq!(history["returnedRevisions"], 2);
        assert_eq!(history["revisions"][0]["changed"], json!(["content"]));
        assert_eq!(
            backend
                .call(
                    &auth,
                    "mother_get_record_history",
                    json!({"id":id,"revision":1})
                )
                .unwrap()["record"],
            saved["record"]
        );
        let pin = backend
            .call(&auth, "mother_pin_record", json!({"id":id,"pinned":true}))
            .unwrap();
        assert_eq!(pin["changed"], true);
        assert_eq!(
            backend
                .call(&auth, "mother_pin_record", json!({"id":id,"pinned":true}))
                .unwrap()["changed"],
            false
        );
        let pinned = backend
            .call(
                &auth,
                "mother_get_context",
                json!({"query":"unlikely no matches"}),
            )
            .unwrap();
        assert_eq!(pinned["pinned"][0]["content"], "Black tea");
        let hits = backend
            .call(
                &auth,
                "mother_search_context",
                json!({"query":" Black tea ","mode":"literal"}),
            )
            .unwrap();
        assert_eq!(hits["search"]["effectiveMode"], "literal");
        assert_eq!(hits["search"]["reason"], Value::Null);
        assert!(
            hits["records"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["id"] == id)
        );
        let fallback = backend
            .call(
                &auth,
                "mother_search_context",
                json!({"query":"Black tea","mode":"semantic"}),
            )
            .unwrap();
        assert_eq!(fallback["search"]["semanticUsed"], false);
        assert_eq!(fallback["search"]["reason"], "model_unavailable");
        let task_id = fixture()["bundle"]["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["kind"] == "task")
            .unwrap()["id"]
            .clone();
        let assigned = backend
            .call(
                &auth,
                "mother_assign_task",
                json!({"id":task_id,"assignee":"parity-writer"}),
            )
            .unwrap();
        assert_eq!(assigned["changed"], true);
        assert_eq!(
            backend
                .call(
                    &auth,
                    "mother_assign_task",
                    json!({"id":task_id,"assignee":"parity-writer"})
                )
                .unwrap()["changed"],
            false
        );
        assert_eq!(
            backend
                .call(&auth, "mother_get_context", json!({}))
                .unwrap()["assignedToYou"][0]["id"],
            task_id
        );
        assert_eq!(
            backend
                .call(
                    &auth,
                    "mother_assign_task",
                    json!({"id":task_id,"assignee":null})
                )
                .unwrap()["changed"],
            true
        );
        let checkpoint = request(
            json!({"kind":"checkpoint","title":"Synthetic handoff","content":"Completed parity checks","status":"active","completed":["Tested"],"openQuestions":[],"nextStep":"Review"}),
        );
        assert_eq!(
            backend
                .call(&auth, "mother_checkpoint", checkpoint)
                .unwrap()["record"]["kind"],
            "checkpoint"
        );
        let mut proposal = request(memory("Another explicit preference"));
        proposal["source"] = json!("Synthetic owner statement");
        proposal["basis"] = json!("owner_statement");
        let p = backend
            .call(&auth, "mother_propose_memory", proposal.clone())
            .unwrap();
        assert_eq!(
            backend
                .call(&auth, "mother_propose_memory", proposal.clone())
                .unwrap(),
            p
        );
        assert_eq!(backend.inbox()["proposals"].as_array().unwrap().len(), 1);
        let p_id = p["proposal"]["id"].clone();
        let read = backend
            .call(&auth, "mother_get_proposal", json!({"id":p_id}))
            .unwrap();
        assert_eq!(read["source"], "Synthetic owner statement");
        assert_eq!(read["status"], "pending");
        let mut family_collision = proposal.clone();
        family_collision.as_object_mut().unwrap().remove("source");
        family_collision.as_object_mut().unwrap().remove("basis");
        assert_eq!(
            backend
                .call(&auth, "mother_write_record", family_collision)
                .unwrap_err()
                .code(),
            "OPERATION_REUSED"
        );
        let review = json!({"workspaceId":proposal["workspaceId"],"generation":proposal["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":p_id,"expectedRevision":1,"action":"save"});
        let accepted = backend.review(review.clone()).unwrap();
        assert_eq!(backend.review(review).unwrap(), accepted);
        assert_eq!(accepted["receipt"]["record"]["actor"], "@owner-review");
        assert_eq!(backend.inbox()["proposals"], json!([]));
        let exported = backend.export();
        assert!(exported.get("inbox").is_some());
        assert_eq!(exported["pins"][0]["recordId"], id);
        assert!(exported.get("assignments").is_some());
        assert!(!exported.to_string().contains(&auth.token));
        let companion = backend.grant(
            "companion",
            &["read".into()],
            &ClientAccess {
                mode: mother::access::AccessMode::Companion,
                subject: Some("synthetic-companion".into()),
                records: vec![],
            },
        );
        assert!(
            backend
                .call(&companion, "mother_get_context", json!({}))
                .unwrap()
                .get("persona")
                .is_some()
        );
        let shared = backend.grant(
            "reader-shared",
            &["read".into(), "context:propose".into()],
            &ClientAccess {
                mode: mother::access::AccessMode::Shared,
                subject: Some("synthetic-shared".into()),
                records: vec![id.as_str().unwrap().into()],
            },
        );
        let visible = backend
            .call(&shared, "mother_get_context", json!({}))
            .unwrap();
        assert!(visible.get("persona").is_none());
        assert!(visible.get("harnesses").is_none());
        assert_eq!(visible["inventory"]["totalRecords"], 1);
        assert_eq!(visible["pinnedSummary"]["total"], 1);
        assert_eq!(
            backend
                .call(&shared, "mother_get_record", json!({"id":task_id}))
                .unwrap_err()
                .code(),
            "NOT_FOUND"
        );
        assert_eq!(
            backend
                .call(&shared, "mother_get_record_history", json!({"id":id}))
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
        assert_eq!(
            backend
                .call(&shared, "mother_get_record", json!({"id":id}))
                .unwrap(),
            revised["record"]
        );
        let mut forbidden_ref = request(memory("Hidden reference"));
        forbidden_ref["record"]["references"] = json!([task_id]);
        forbidden_ref["source"] = json!("Synthetic shared statement");
        forbidden_ref["basis"] = json!("owner_statement");
        assert_eq!(
            backend
                .call(&shared, "mother_propose_memory", forbidden_ref)
                .unwrap_err()
                .code(),
            "NOT_FOUND"
        );
        backend.revoke("parity-writer");
        assert_eq!(
            backend
                .call(&auth, "mother_get_context", json!({}))
                .unwrap_err()
                .code(),
            "UNAUTHORIZED"
        );
    }
}

#[test]
fn graph_proposal_dismissal_or_blob_requires_full_backup_even_without_graph_records() {
    use mother::garden::contracts::BatchRequest;
    for state in ["pending", "dismissed", "blob"] {
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let s = GardenStore::initialize(dir.path().join("guard.redb"), &workspace, &generation)
            .unwrap();
        s.v1_initialize("Synthetic explicit persona").unwrap();
        assert!(s.v1_export().is_ok());
        if state == "blob" {
            s.put_blob(&b"synthetic referenced bytes"[..]).unwrap();
        } else {
            let auth = s
                .grant(
                    "graph-proposer",
                    &["garden:read".into(), "garden:propose".into()],
                    &ClientAccess::default(),
                    &GardenAccess::All {},
                )
                .unwrap();
            let batch:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"entity","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":"Synthetic proposal","aliases":[],"extensions":{}}}]})).unwrap();
            let proposal = s.propose(&auth, &batch).unwrap();
            if state == "dismissed" {
                s.review_proposal(json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"proposalId":proposal.proposal_id,"expectedRevision":1,"action":"dismiss"})).unwrap();
            }
        }
        assert_eq!(
            s.v1_export().unwrap_err().code(),
            "FULL_BACKUP_REQUIRED",
            "state {state}"
        );
    }
}

#[test]
fn enabling_native_view_does_not_upgrade_graph_only_credentials() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let s = GardenStore::initialize(
        dir.path().join("authority.redb"),
        &Uuid::new_v4().to_string(),
        &Uuid::new_v4().to_string(),
    )
    .unwrap();
    let graph = s
        .grant(
            "graph-only",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    assert!(!s.v1_initialized().unwrap());
    s.v1_initialize("Synthetic explicit owner persona").unwrap();
    assert_eq!(
        s.v1_call(&graph, "mother_get_context", json!({}))
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    assert!(
        s.v1_grant(
            "implicit-graph",
            &["read".into(), "garden:read".into()],
            &ClientAccess::default(),
            None
        )
        .is_err()
    );
}

#[test]
fn full_native_backup_keeps_pins_assignments_persona_and_blocks_old_pending_approval() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let f = fixture();
    let b = &f["bundle"];
    let s = GardenStore::initialize(
        dir.path().join("source.redb"),
        b["workspace"]["id"].as_str().unwrap(),
        b["workspace"]["generation"].as_str().unwrap(),
    )
    .unwrap();
    s.import_v1_bundle(b.clone()).unwrap();
    let auth = grant(&s);
    let memory = b["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "memory")
        .unwrap();
    let task = b["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "task")
        .unwrap();
    s.v1_call(
        &auth,
        "mother_pin_record",
        json!({"id":memory["id"],"pinned":true}),
    )
    .unwrap();
    s.v1_call(
        &auth,
        "mother_assign_task",
        json!({"id":task["id"],"assignee":"fixture-client"}),
    )
    .unwrap();
    let mut proposal = request(memory_draft_for_backup());
    proposal["source"] = json!("Synthetic owner statement");
    proposal["basis"] = json!("owner_statement");
    let p = s.v1_call(&auth, "mother_propose_memory", proposal).unwrap();
    let out = dir.path().join("backup");
    s.backup_to(&out).unwrap();
    let restored = GardenStore::restore_backup(&out, &dir.path().join("restored.redb")).unwrap();
    assert_eq!(
        restored
            .v1_call(&auth, "mother_get_context", json!({}))
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
    let new_auth = grant(&restored);
    let context = restored
        .v1_call(&new_auth, "mother_get_context", json!({}))
        .unwrap();
    assert_ne!(
        context["workspace"]["generation"],
        b["workspace"]["generation"]
    );
    assert_eq!(context["persona"], b["persona"]);
    assert_eq!(context["pinned"][0]["id"], memory["id"]);
    assert_eq!(context["assignedToYou"][0]["id"], task["id"]);
    let current = restored
        .v1_inbox_get(p["proposal"]["id"].as_str().unwrap())
        .unwrap();
    assert_eq!(current["conflict"], true);
    let review = json!({"workspaceId":context["workspace"]["id"],"generation":context["workspace"]["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":p["proposal"]["id"],"expectedRevision":1,"action":"save"});
    assert_eq!(
        restored.v1_review_proposal(review).unwrap_err().code(),
        "STALE_GENERATION"
    );
    let exported = restored.v1_export().unwrap();
    assert_eq!(exported["operations"], b["operations"]);
    assert_eq!(exported["pins"][0]["recordId"], memory["id"]);
    assert!(!exported.to_string().contains(&auth.token));
    assert!(!exported.to_string().contains(&new_auth.token));
}
fn memory_draft_for_backup() -> Value {
    memory("Pending proposal survives native full backup")
}

#[test]
fn native_runtime_exceeds_legacy_record_capacity_and_requires_full_backup() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let f = fixture();
    let b = &f["bundle"];
    let path = dir.path().join("larger.redb");
    let s = GardenStore::initialize(
        &path,
        b["workspace"]["id"].as_str().unwrap(),
        b["workspace"]["generation"].as_str().unwrap(),
    )
    .unwrap();
    s.import_v1_bundle(b.clone()).unwrap();
    let auth = grant(&s);
    drop(s);
    // Synthetic canonical seed avoids ten thousand individual fsyncs; admission is tested at the boundary.
    let db = redb::Database::open(&path).unwrap();
    let tx = db.begin_write().unwrap();
    {
        let mut table = tx
            .open_table(redb::TableDefinition::<&str, &[u8]>::new(
                "garden_v1_records",
            ))
            .unwrap();
        let template = b["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["kind"] == "memory")
            .unwrap();
        for _ in 6..10_000 {
            let mut r = template.clone();
            let id = Uuid::new_v4().to_string();
            r["id"] = json!(id);
            table
                .insert(id.as_str(), serde_json::to_vec(&r).unwrap().as_slice())
                .unwrap();
        }
    }
    tx.commit().unwrap();
    drop(db);
    let s = GardenStore::open(&path).unwrap();
    let receipt = s
        .v1_call(
            &auth,
            "mother_write_record",
            request(memory("Native record number ten thousand and one")),
        )
        .unwrap();
    assert_eq!(receipt["record"]["revision"], 1);
    assert_eq!(s.v1_export().unwrap_err().code(), "FULL_BACKUP_REQUIRED");
    let out = dir.path().join("large-backup");
    s.backup_to(&out).unwrap();
    let restored = GardenStore::restore_backup(&out, &dir.path().join("restored.redb")).unwrap();
    let auth = grant(&restored);
    let context = restored
        .v1_call(&auth, "mother_get_context", json!({"limit":1}))
        .unwrap();
    assert_eq!(context["inventory"]["totalRecords"], 10_001);
}

#[test]
fn sparse_native_history_rejects_unbounded_missing_revision_list() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let mut bundle = fixture()["bundle"].clone();
    let mut r = bundle["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "memory")
        .unwrap()
        .clone();
    r["revision"] = json!(9_007_199_254_740_991u64);
    let id = r["id"].clone();
    bundle["records"] = json!([r]);
    bundle["operations"] = json!([]);
    bundle["omittedExternalSources"] = json!([]);
    let store = GardenStore::initialize(
        dir.path().join("sparse.redb"),
        bundle["workspace"]["id"].as_str().unwrap(),
        bundle["workspace"]["generation"].as_str().unwrap(),
    )
    .unwrap();
    store.import_v1_bundle(bundle).unwrap();
    let auth = grant(&store);
    assert_eq!(
        store
            .v1_call(&auth, "mother_get_record_history", json!({"id":id}))
            .unwrap_err()
            .code(),
        "RESOURCE_LIMIT"
    );
    assert_eq!(
        store
            .v1_call(
                &auth,
                "mother_get_record_history",
                json!({"id":id,"revision":1})
            )
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
}
