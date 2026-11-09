use mother::{contracts::Auth, store::Store};
use serde_json::{Value, json};
use tempfile::TempDir;
use uuid::Uuid;
fn fixture() -> (TempDir, Store, Auth) {
    let dir = tempfile::tempdir().unwrap();
    let mut s = Store::initialize(&dir.path().join("context.sqlite"), "Synthetic persona").unwrap();
    let g = s
        .grant("chatgpt", &["read".into(), "context:propose".into()])
        .unwrap();
    let a = Auth {
        client_id: "chatgpt".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    (dir, s, a)
}
fn draft() -> Value {
    json!({"kind":"memory","title":"Tea","content":"Green tea","status":"active","basis":"owner_statement"})
}
fn proposal(s: &Store) -> Value {
    let w = s.workspace().unwrap();
    json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":draft(),"source":"Owner explicitly asked in synthetic chat","basis":"owner_statement"})
}
fn review(s: &Store, p: &Value, action: &str) -> Value {
    let w = s.workspace().unwrap();
    json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":p["id"],"expectedRevision":p["revision"],"action":action})
}
fn code<T: std::fmt::Debug>(r: mother::error::Result<T>, c: &str) {
    assert_eq!(r.unwrap_err().code(), c);
}
#[test]
fn proposals_are_separate_durable_attributed_and_owner_save_is_atomic_idempotent() {
    let (dir, mut s, a) = fixture();
    let q = proposal(&s);
    let submitted = s.propose(&a, q.clone()).unwrap();
    let p = &submitted["proposal"];
    assert!(
        s.context(&a, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(p["actor"], "chatgpt");
    assert_eq!(p["status"], "pending");
    let mut save = review(&s, p, "save");
    save["record"] = draft();
    save["record"]["content"] = json!("Black tea");
    let result = s.review_proposal(save.clone()).unwrap();
    assert_eq!(result["receipt"]["record"]["actor"], "@owner-review");
    assert_eq!(result["receipt"]["record"]["content"], "Black tea");
    assert_eq!(
        s.inbox_get(p["id"].as_str().unwrap()).unwrap()["record"]["content"],
        "Green tea"
    );
    drop(s);
    let mut s = Store::open(&dir.path().join("context.sqlite")).unwrap();
    assert_eq!(s.review_proposal(save.clone()).unwrap(), result);
    assert_eq!(s.propose(&a, q.clone()).unwrap(), submitted);
    assert_eq!(
        s.receipt(&a, q["operationId"].as_str().unwrap()).unwrap(),
        submitted
    );
    save["record"]["content"] = json!("changed retry");
    code(s.review_proposal(save), "OPERATION_REUSED");
    assert!(
        s.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn dedup_compares_all_normalized_fields_and_preserves_each_submission_receipt() {
    let (_, mut s, a) = fixture();
    let q = proposal(&s);
    let one = s.propose(&a, q.clone()).unwrap();
    let mut two = q.clone();
    two["operationId"] = json!(Uuid::new_v4().to_string());
    let r = s.propose(&a, two.clone()).unwrap();
    assert_eq!(one["proposal"]["id"], r["proposal"]["id"]);
    assert_ne!(one["operationId"], r["operationId"]);
    two["operationId"] = json!(Uuid::new_v4().to_string());
    two["source"] = json!("Other source");
    assert_ne!(
        s.propose(&a, two).unwrap()["proposal"]["id"],
        one["proposal"]["id"]
    );
    s.review_proposal(review(&s, &one["proposal"], "save"))
        .unwrap();
    let mut q = q;
    q["operationId"] = json!(Uuid::new_v4().to_string());
    let duplicate = s.propose(&a, q).unwrap();
    assert!(
        s.inbox_get(duplicate["proposal"]["id"].as_str().unwrap())
            .unwrap()["duplicateRecordId"]
            .is_string()
    );
    code(
        s.review_proposal(review(&s, &duplicate["proposal"], "save")),
        "DUPLICATE_RECORD",
    );
}
#[test]
fn read_only_revoked_and_changed_scopes_cannot_approve_but_dismiss_is_available() {
    let (dir, mut s, a) = fixture();
    let g = s.grant("reader", &["read".into()]).unwrap();
    let reader = Auth {
        client_id: "reader".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    code(s.propose(&reader, proposal(&s)), "FORBIDDEN");
    let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    s.revoke(&a.client_id).unwrap();
    code(s.review_proposal(review(&s, &p, "save")), "UNAUTHORIZED");
    assert_eq!(
        s.review_proposal(review(&s, &p, "dismiss")).unwrap()["status"],
        "dismissed"
    );
    let g = s
        .grant("hermes", &["read".into(), "memory:write".into()])
        .unwrap();
    let b = Auth {
        client_id: "hermes".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let p = s.propose(&b, proposal(&s)).unwrap()["proposal"].clone();
    rusqlite::Connection::open(dir.path().join("context.sqlite"))
        .unwrap()
        .execute(
            "UPDATE clients SET scopes='[\"read\"]' WHERE id='hermes'",
            [],
        )
        .unwrap();
    code(s.review_proposal(review(&s, &p, "save")), "FORBIDDEN");
}
#[test]
fn correction_snapshot_conflicts_and_cross_family_operations_cannot_be_reused() {
    let (_, mut s, a) = fixture();
    let g = s
        .grant("writer", &["read".into(), "memory:write".into()])
        .unwrap();
    let b = Auth {
        client_id: "writer".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let q = proposal(&s);
    let mut direct = q.clone();
    direct.as_object_mut().unwrap().remove("source");
    direct.as_object_mut().unwrap().remove("basis");
    let first = s.write(&b, direct.clone()).unwrap();
    code(s.propose(&b, q), "OPERATION_REUSED");
    let mut q = proposal(&s);
    q["id"] = first["record"]["id"].clone();
    q["expectedRevision"] = json!(1);
    q["record"]["content"] = json!("Correction");
    let p = s.propose(&a, q.clone()).unwrap()["proposal"].clone();
    assert_eq!(p["before"], first["record"]);
    direct["operationId"] = json!(Uuid::new_v4().to_string());
    direct["id"] = first["record"]["id"].clone();
    direct["expectedRevision"] = json!(1);
    s.write(&b, direct).unwrap();
    assert_eq!(
        s.inbox_get(p["id"].as_str().unwrap()).unwrap()["conflict"],
        true
    );
    code(s.review_proposal(review(&s, &p, "save")), "CONFLICT");
    let q = proposal(&s);
    s.propose(&b, q.clone()).unwrap();
    let mut direct = q;
    direct.as_object_mut().unwrap().remove("source");
    direct.as_object_mut().unwrap().remove("basis");
    code(s.write(&b, direct), "OPERATION_REUSED");
}
#[test]
fn restore_preserves_inbox_but_old_generation_cannot_save_and_malformed_is_precreation() {
    let (dir, mut s, a) = fixture();
    let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    let bundle = s.export().unwrap();
    assert!(!bundle.to_string().contains(&a.token));
    assert_eq!(bundle["inbox"]["proposals"].as_array().unwrap().len(), 1);
    let mut restored = Store::restore(&dir.path().join("restored.sqlite"), bundle.clone()).unwrap();
    assert_eq!(
        restored.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    code(
        restored.review_proposal(review(&restored, &p, "save")),
        "STALE_GENERATION",
    );
    assert_eq!(
        restored
            .review_proposal(review(&restored, &p, "dismiss"))
            .unwrap()["status"],
        "dismissed"
    );
    restored.export().unwrap();
    for field in ["id", "revision", "actor", "generation", "requestHash"] {
        let mut bad = bundle.clone();
        if field == "requestHash" {
            bad["inbox"]["operations"][0][field] = json!("bad");
        } else {
            bad["inbox"]["proposals"][0][field] = json!("bad");
        }
        let path = dir.path().join(format!("invalid-{field}.sqlite"));
        assert!(Store::restore(&path, bad).is_err());
        assert!(!path.exists());
    }
}
#[test]
fn invalid_attribution_references_kinds_and_spoofing_never_commit() {
    let (_, mut s, a) = fixture();
    for (field, value) in [
        ("source", json!("  ")),
        ("basis", json!("fact")),
        ("actor", json!("owner")),
    ] {
        let mut q = proposal(&s);
        q[field] = value;
        code(s.propose(&a, q), "INVALID_INPUT");
    }
    let mut q = proposal(&s);
    q["record"]["basis"] = json!("model_inference");
    code(s.propose(&a, q), "INVALID_INPUT");
    let mut q = proposal(&s);
    q["record"]["references"] = json!([Uuid::new_v4().to_string()]);
    code(s.propose(&a, q), "REFERENCE_NOT_FOUND");
    assert!(
        s.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn inbox_reads_are_lazy_and_listing_is_bounded_without_losing_pending_items() {
    let (dir, mut s, a) = fixture();
    s.inbox_list().unwrap();
    assert!(s.inbox_get(&Uuid::new_v4().to_string()).is_err());
    let conn = rusqlite::Connection::open(dir.path().join("context.sqlite")).unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name LIKE 'inbox_%'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
    assert!(s.export().unwrap().get("inbox").is_none());
    let w = s.workspace().unwrap();
    for i in 0..102 {
        let mut q = proposal(&s);
        q["record"]["title"] = json!(format!("Proposal {i}"));
        s.propose(&a, q).unwrap();
    }
    let list = s.inbox_list().unwrap();
    assert_eq!(list["proposals"].as_array().unwrap().len(), 100);
    assert_eq!(list["omitted"], 2);
    assert_eq!(s.workspace().unwrap(), w);
    let version: u32 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(version, 1);
}
#[test]
fn concurrent_approval_commits_once_and_keeps_the_losing_proposal_pending() {
    let (dir, mut s, a) = fixture();
    let one = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    let mut q = proposal(&s);
    q["source"] = json!("Another attributed submission");
    let two = s.propose(&a, q).unwrap()["proposal"].clone();
    let requests = [review(&s, &one, "save"), review(&s, &two, "save")];
    let path = dir.path().join("context.sqlite");
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let joins = requests
            .into_iter()
            .map(|q| {
                let barrier = &barrier;
                let path = &path;
                scope.spawn(move || {
                    let mut s = Store::open(path).unwrap();
                    barrier.wait();
                    s.review_proposal(q)
                })
            })
            .collect::<Vec<_>>();
        joins
            .into_iter()
            .map(|j| j.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .find_map(|r| r.as_ref().err())
            .unwrap()
            .code(),
        "DUPLICATE_RECORD"
    );
    assert_eq!(
        s.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        s.context(&a, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    s.export().unwrap();
}
#[test]
fn edited_invalid_references_roll_back_and_source_changes_are_auditable() {
    let (_, mut s, a) = fixture();
    let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    let mut save = review(&s, &p, "save");
    save["record"] = draft();
    save["record"]["references"] = json!([Uuid::new_v4().to_string()]);
    code(s.review_proposal(save.clone()), "REFERENCE_NOT_FOUND");
    assert_eq!(
        s.inbox_get(p["id"].as_str().unwrap()).unwrap()["revision"],
        1
    );
    assert!(
        s.context(&a, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    save["record"]["references"] = json!([]);
    save["record"]["basis"] = json!("source");
    save["record"]["sources"] =
        json!([{"label":"Synthetic evidence","uri":"https://example.com/tea"}]);
    let saved = s.review_proposal(save).unwrap();
    assert_eq!(saved["receipt"]["record"]["basis"], "source");
    let p = s.inbox_get(p["id"].as_str().unwrap()).unwrap();
    assert_eq!(p["basis"], "owner_statement");
    assert_eq!(p["record"]["basis"], "owner_statement");
    s.export().unwrap();
}
#[test]
fn malformed_saved_audit_and_missing_extension_references_fail_before_restore_creation() {
    let (dir, mut s, a) = fixture();
    let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    s.review_proposal(review(&s, &p, "save")).unwrap();
    let b = s.export().unwrap();
    let mut cases = Vec::new();
    let mut v = b.clone();
    v["inbox"]["proposals"][0]["receipt"]["status"] = json!("dismissed");
    cases.push(v);
    let mut v = b.clone();
    v["inbox"]["proposals"][0]["record"]["references"] = json!([Uuid::new_v4().to_string()]);
    cases.push(v);
    let mut v = b.clone();
    v["inbox"]["operations"][0]["request"]["source"] = json!("Forged source");
    cases.push(v);
    let mut v = b.clone();
    v["inbox"]["operations"] = json!([]);
    cases.push(v);
    for (i, v) in cases.into_iter().enumerate() {
        let path = dir.path().join(format!("bad-audit-{i}.sqlite"));
        assert!(Store::restore(&path, v).is_err());
        assert!(!path.exists());
    }
    let mut restored = Store::restore(&dir.path().join("saved.sqlite"), b).unwrap();
    restored.export().unwrap();
}
#[test]
fn approval_hash_tampering_and_forged_submission_flags_are_rejected() {
    let (dir, mut s, a) = fixture();
    let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    s.review_proposal(review(&s, &p, "save")).unwrap();
    let bundle = s.export().unwrap();
    let mut changed = bundle.clone();
    changed["operations"][0]["requestHash"] = json!("a".repeat(64));
    let path = dir.path().join("forged-write-hash.sqlite");
    assert!(Store::restore(&path, changed).is_err());
    assert!(!path.exists());
    let mut changed = bundle;
    let submission = changed["inbox"]["operations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|o| o["family"] == "propose")
        .unwrap();
    submission["receipt"]["proposal"]["conflict"] = json!(true);
    let path = dir.path().join("forged-flag.sqlite");
    assert!(Store::restore(&path, changed).is_err());
    assert!(!path.exists());
}
#[test]
fn capacity_failure_rolls_back_submission_and_approval_without_partial_receipts() {
    let (dir, mut s, a) = fixture();
    let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    let conn = rusqlite::Connection::open(dir.path().join("context.sqlite")).unwrap();
    conn.execute(
        "UPDATE metadata SET value=? WHERE key='usage'",
        [json!({"records":0,"operations":100000,"payloadBytes":0}).to_string()],
    )
    .unwrap();
    let mut q = proposal(&s);
    q["source"] = json!("Distinct source");
    code(s.propose(&a, q), "RESOURCE_LIMIT");
    code(s.review_proposal(review(&s, &p, "save")), "RESOURCE_LIMIT");
    assert_eq!(
        s.inbox_get(p["id"].as_str().unwrap()).unwrap()["status"],
        "pending"
    );
    assert_eq!(
        s.inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(
        s.context(&a, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM operations", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}
#[test]
fn clock_rollback_review_timestamps_stay_exportable_for_save_and_dismiss() {
    for action in ["save", "dismiss"] {
        let (dir, mut s, a) = fixture();
        let p = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
        let conn = rusqlite::Connection::open(dir.path().join("context.sqlite")).unwrap();
        let raw: String = conn
            .query_row("SELECT payload FROM inbox_proposals", [], |r| r.get(0))
            .unwrap();
        let mut historical: Value = serde_json::from_str(&raw).unwrap();
        historical["createdAt"] = json!("2099-01-01T00:00:00.000Z");
        historical["updatedAt"] = historical["createdAt"].clone();
        conn.execute(
            "UPDATE inbox_proposals SET payload=?",
            [historical.to_string()],
        )
        .unwrap();
        let raw: String = conn
            .query_row("SELECT receipt FROM inbox_operations", [], |r| r.get(0))
            .unwrap();
        let mut submission: Value = serde_json::from_str(&raw).unwrap();
        submission["proposal"]["createdAt"] = historical["createdAt"].clone();
        submission["proposal"]["updatedAt"] = historical["updatedAt"].clone();
        conn.execute(
            "UPDATE inbox_operations SET receipt=?",
            [submission.to_string()],
        )
        .unwrap();
        s.review_proposal(review(&s, &p, action)).unwrap();
        let resolved = s.inbox_get(p["id"].as_str().unwrap()).unwrap();
        assert_eq!(resolved["updatedAt"], historical["updatedAt"]);
        let bundle = s.export().unwrap();
        let mut restored = Store::restore(&dir.path().join("restored.sqlite"), bundle).unwrap();
        restored.export().unwrap();
    }
}
#[test]
fn similar_title_or_content_warns_without_merging_distinct_drafts() {
    let (_, mut s, a) = fixture();
    let first = s.propose(&a, proposal(&s)).unwrap()["proposal"].clone();
    let canonical = s.review_proposal(review(&s, &first, "save")).unwrap();
    let mut q = proposal(&s);
    q["record"]["content"] = json!("A different preference");
    let p = s.propose(&a, q).unwrap()["proposal"].clone();
    assert!(p["duplicateRecordId"].is_null());
    assert_eq!(
        p["similarRecordIds"],
        json!([canonical["receipt"]["record"]["id"]])
    );
    let saved = s.review_proposal(review(&s, &p, "save")).unwrap();
    assert_ne!(
        saved["receipt"]["record"]["id"],
        canonical["receipt"]["record"]["id"]
    );
    s.export().unwrap();
}
