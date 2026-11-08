use mother::{contracts::Auth, store::Store};
use serde_json::{Value, json};
use tempfile::TempDir;
use uuid::Uuid;

fn setup() -> (TempDir, Store, Auth) {
    let dir = tempfile::tempdir().unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic persona").unwrap();
    let grant = store
        .grant(
            "client-a",
            &[
                "read",
                "memory:write",
                "task:write",
                "checkpoint:write",
                "action:write",
            ]
            .map(str::to_owned),
        )
        .unwrap();
    let auth = Auth {
        client_id: "client-a".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    (dir, store, auth)
}
fn memory() -> Value {
    json!({"kind":"memory","title":"Tea","content":"Prefers green tea","status":"active","basis":"owner_statement"})
}
fn request(store: &Store) -> Value {
    let w = store.workspace().unwrap();
    json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":memory()})
}
fn code<T: std::fmt::Debug>(result: mother::error::Result<T>, expected: &str) {
    assert_eq!(result.unwrap_err().code(), expected);
}

#[test]
fn two_connections_preserve_identity_and_attribution() {
    let (dir, mut s, a) = setup();
    let r = s.write(&a, request(&s)).unwrap();
    let mut other = Store::open(&dir.path().join("context.sqlite")).unwrap();
    assert_eq!(other.workspace().unwrap(), s.workspace().unwrap());
    assert_eq!(
        other.get(&a, r["record"]["id"].as_str().unwrap()).unwrap(),
        r["record"]
    );
    assert_eq!(r["record"]["actor"], "client-a");
}
#[test]
fn correction_rejects_stale_write() {
    let (_, mut s, a) = setup();
    let r = s.write(&a, request(&s)).unwrap();
    let mut q = request(&s);
    q["id"] = r["record"]["id"].clone();
    q["expectedRevision"] = json!(1);
    q["record"]["content"] = json!("Black tea");
    let updated = s.write(&a, q.clone()).unwrap();
    assert_eq!(updated["record"]["revision"], 2);
    q["operationId"] = json!(Uuid::new_v4().to_string());
    code(s.write(&a, q), "CONFLICT");
}
#[test]
fn retries_survive_restart_and_payload_changes_fail() {
    let (dir, mut s, a) = setup();
    let q = request(&s);
    let r = s.write(&a, q.clone()).unwrap();
    drop(s);
    let mut s = Store::open(&dir.path().join("context.sqlite")).unwrap();
    assert_eq!(s.write(&a, q.clone()).unwrap(), r);
    let mut changed = q;
    changed["record"]["content"] = json!("different");
    code(s.write(&a, changed), "OPERATION_REUSED");
    assert_eq!(
        s.context(&a, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}
#[test]
fn read_only_and_revoked_grants_fail() {
    let (_, mut s, a) = setup();
    let r = s.grant("reader", &["read".into()]).unwrap();
    let readonly = Auth {
        client_id: "reader".into(),
        token: r["token"].as_str().unwrap().into(),
    };
    code(s.write(&readonly, request(&s)), "FORBIDDEN");
    s.revoke(&a.client_id).unwrap();
    code(s.context(&a, json!({})), "UNAUTHORIZED");
    code(s.write(&a, request(&s)), "UNAUTHORIZED");
}
#[test]
fn token_workspace_generation_and_actor_cannot_be_spoofed() {
    let (_, mut s, a) = setup();
    let bad = Auth {
        client_id: a.client_id.clone(),
        token: "wrong".into(),
    };
    code(s.context(&bad, json!({})), "UNAUTHORIZED");
    let mut q = request(&s);
    q["workspaceId"] = json!(Uuid::new_v4().to_string());
    code(s.write(&a, q), "WORKSPACE_MISMATCH");
    let mut q = request(&s);
    q["generation"] = json!(Uuid::new_v4().to_string());
    code(s.write(&a, q), "STALE_GENERATION");
    let mut q = request(&s);
    q["actor"] = json!("owner");
    code(s.write(&a, q), "INVALID_INPUT");
}
#[test]
fn retractions_are_not_active_context() {
    let (_, mut s, a) = setup();
    let r = s.write(&a, request(&s)).unwrap();
    let mut q = request(&s);
    q["id"] = r["record"]["id"].clone();
    q["expectedRevision"] = json!(1);
    q["record"]["status"] = json!("retracted");
    s.write(&a, q).unwrap();
    assert_eq!(s.context(&a, json!({})).unwrap()["records"], json!([]));
    assert_eq!(
        s.get(&a, r["record"]["id"].as_str().unwrap()).unwrap()["status"],
        "retracted"
    );
}
#[test]
fn missing_reference_rolls_back_operation() {
    let (_, mut s, a) = setup();
    let q = request(&s);
    let mut bad = q.clone();
    bad["record"]["references"] = json!([Uuid::new_v4().to_string()]);
    code(s.write(&a, bad), "REFERENCE_NOT_FOUND");
    assert_eq!(s.write(&a, q).unwrap()["record"]["revision"], 1);
}
#[test]
fn search_is_literal_and_omissions_are_explicit() {
    let (_, mut s, a) = setup();
    s.write(&a, request(&s)).unwrap();
    s.write(&a, request(&s)).unwrap();
    assert_eq!(s.context(&a, json!({"limit":1})).unwrap()["omitted"], 1);
    assert_eq!(
        s.context(&a, json!({"query":"%"})).unwrap()["records"],
        json!([])
    );
    assert_eq!(
        s.context(&a, json!({"query":"green tea"})).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}
#[test]
fn record_kind_and_revision_contract_is_preserved() {
    let (_, mut s, a) = setup();
    let r = s.write(&a, request(&s)).unwrap();
    let mut q = request(&s);
    q["expectedRevision"] = json!(1);
    code(s.write(&a, q), "INVALID_INPUT");
    let mut q = request(&s);
    q["id"] = r["record"]["id"].clone();
    code(s.write(&a, q.clone()), "CONFLICT");
    q["expectedRevision"] = json!(1);
    q["record"] = json!({"kind":"task","title":"Tea","content":"Buy tea","status":"todo"});
    code(s.write(&a, q), "KIND_MISMATCH");
}
#[test]
fn initialization_never_overwrites() {
    let (dir, s, _) = setup();
    let id = s.workspace().unwrap();
    assert!(Store::initialize(&dir.path().join("context.sqlite"), "other").is_err());
    assert_eq!(s.workspace().unwrap(), id);
}

#[test]
fn opens_typescript_database_and_replays_original_operations() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("legacy.sqlite");
    let conn = rusqlite::Connection::open(&path).unwrap();
    conn.execute_batch(include_str!("fixtures/v1.sql")).unwrap();
    drop(conn);
    let f: Value = serde_json::from_str(include_str!("fixtures/v1.json")).unwrap();
    let auth: Auth = serde_json::from_value(
        f["auth"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, _)| *k != "scopes")
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<serde_json::Map<_, _>>()
            .into(),
    )
    .unwrap();
    let mut s = Store::open(&path).unwrap();
    assert_eq!(s.workspace().unwrap(), f["bundle"]["workspace"]);
    for (q, r) in f["requests"]
        .as_array()
        .unwrap()
        .iter()
        .zip(f["receipts"].as_array().unwrap())
    {
        assert_eq!(s.write(&auth, q.clone()).unwrap(), *r);
    }
    let mut exported = s.export().unwrap();
    exported["exportedAt"] = f["bundle"]["exportedAt"].clone();
    assert_eq!(exported, f["bundle"]);
    let q = request(&s);
    assert_eq!(s.write(&auth, q).unwrap()["record"]["revision"], 1);
}

#[test]
fn persona_updates_preserve_identity_records_receipts_grants_and_portability() {
    let (dir, mut s, auth) = setup();
    let request = request(&s);
    let receipt = s.write(&auth, request.clone()).unwrap();
    let before = s.export().unwrap();
    let original = s.persona().unwrap();
    let old_hash = original["hash"].as_str().unwrap();
    for text in ["", " \n\t", "\u{feff}", &"x".repeat(32_001)] {
        code(s.update_persona(text, old_hash), "INVALID_INPUT");
        assert_eq!(s.persona().unwrap(), original);
    }
    code(s.update_persona("New persona", "bad hash"), "INVALID_INPUT");
    code(s.update_persona("New persona", &"0".repeat(64)), "CONFLICT");
    let result = s.update_persona("New persona", old_hash).unwrap();
    assert_eq!(result["changed"], true);
    let after = s.export().unwrap();
    let mut comparable = after.clone();
    comparable["persona"] = before["persona"].clone();
    comparable["exportedAt"] = before["exportedAt"].clone();
    assert_eq!(
        comparable, before,
        "only persona may change in the portable state"
    );
    s.authorize(&auth, "memory:write").unwrap();
    assert_eq!(s.write(&auth, request).unwrap(), receipt);
    let restored = Store::restore(&dir.path().join("restored.sqlite"), after).unwrap();
    assert_eq!(restored.persona().unwrap(), s.persona().unwrap());
    let reopened = Store::open(&dir.path().join("context.sqlite")).unwrap();
    assert_eq!(reopened.persona().unwrap(), s.persona().unwrap());
}

#[test]
fn concurrent_owner_persona_updates_allow_exactly_one_current_hash() {
    use std::sync::{Arc, Barrier};
    let (dir, s, _) = setup();
    let path = dir.path().join("context.sqlite");
    let hash = s.persona().unwrap()["hash"].as_str().unwrap().to_owned();
    let barrier = Arc::new(Barrier::new(2));
    let jobs: Vec<_> = ["First owner edit", "Second owner edit"]
        .into_iter()
        .map(|text| {
            let path = path.clone();
            let hash = hash.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut store = Store::open(&path).unwrap();
                barrier.wait();
                store.update_persona(text, &hash)
            })
        })
        .collect();
    let results: Vec<_> = jobs.into_iter().map(|job| job.join().unwrap()).collect();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| r.as_ref().is_err_and(|e| e.code() == "CONFLICT"))
            .count(),
        1
    );
    assert_ne!(s.persona().unwrap()["hash"], hash);
}
