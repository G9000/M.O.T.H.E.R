use mother::{contracts::Auth, error::Result, store::Store};
use serde_json::{Value, json};
use uuid::Uuid;
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/v1.json")).unwrap()
}
fn grant(s: &mut Store) -> Auth {
    let g = s
        .grant("reader", &["read".into(), "memory:write".into()])
        .unwrap();
    Auth {
        client_id: "reader".into(),
        token: g["token"].as_str().unwrap().into(),
    }
}
fn check_err<T: std::fmt::Debug>(r: Result<T>, code: &str) {
    assert_eq!(r.unwrap_err().code(), code)
}
#[test]
fn export_restores_persona_identity_records_unknown_outcomes_without_credentials() {
    let dir = tempfile::tempdir().unwrap();
    let f = fixture();
    let bundle = f["bundle"].clone();
    let mut s = Store::restore(&dir.path().join("restored.sqlite"), bundle.clone()).unwrap();
    let old: Auth = serde_json::from_value(
        json!({"clientId":f["auth"]["clientId"],"token":f["auth"]["token"]}),
    )
    .unwrap();
    check_err(s.context(&old, json!({})), "UNAUTHORIZED");
    assert_eq!(s.workspace().unwrap()["id"], bundle["workspace"]["id"]);
    assert_ne!(
        s.workspace().unwrap()["generation"],
        bundle["workspace"]["generation"]
    );
    assert_eq!(s.persona().unwrap(), bundle["persona"]);
    let auth = grant(&mut s);
    for r in bundle["records"].as_array().unwrap() {
        assert_eq!(s.get(&auth, r["id"].as_str().unwrap()).unwrap(), *r);
    }
    let exported = s.export().unwrap();
    assert_eq!(exported["records"], bundle["records"]);
    assert_eq!(exported["operations"], bundle["operations"]);
    assert_eq!(
        exported["omittedExternalSources"],
        json!(["https://example.com/evidence"])
    );
    assert!(!exported.to_string().contains(&auth.token));
    check_err(s.write(&auth, f["requests"][0].clone()), "STALE_GENERATION");
}
#[test]
fn malformed_bundles_are_rejected_before_destination_creation() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("invalid.sqlite");
    let b = fixture()["bundle"].clone();
    let mut cases = Vec::new();
    let mut x = b.clone();
    x["schemaVersion"] = json!(99);
    cases.push(x);
    let mut x = b.clone();
    x["credentials"] = json!("secret");
    cases.push(x);
    let mut x = b.clone();
    x["persona"]["content"] = json!("changed");
    cases.push(x);
    let mut x = b.clone();
    let r = x["records"][0].clone();
    x["records"].as_array_mut().unwrap().push(r);
    cases.push(x);
    let mut x = b.clone();
    x["records"][0]["references"] = json!([Uuid::new_v4().to_string()]);
    cases.push(x);
    let mut x = b.clone();
    let op = x["operations"][0].clone();
    x["operations"].as_array_mut().unwrap().push(op);
    cases.push(x);
    let mut x = b.clone();
    x["omittedExternalSources"] = json!([]);
    cases.push(x);
    for bad in cases {
        assert!(Store::restore(&path, bad).is_err());
        assert!(!path.exists());
    }
}
#[test]
fn restore_is_fresh_only_and_retraction_survives() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("source.sqlite");
    let mut s = Store::initialize(&path, "persona").unwrap();
    let a = grant(&mut s);
    let w = s.workspace().unwrap();
    let r=s.write(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Old preference","content":"No longer current","status":"retracted","basis":"owner_statement"}})).unwrap();
    let bundle = s.export().unwrap();
    assert!(Store::restore(&path, bundle.clone()).is_err());
    drop(s);
    std::fs::remove_file(path).unwrap();
    let mut dest = Store::restore(&dir.path().join("fresh.sqlite"), bundle).unwrap();
    let reader = grant(&mut dest);
    assert_eq!(
        dest.get(&reader, r["record"]["id"].as_str().unwrap())
            .unwrap()["status"],
        "retracted"
    );
    assert_eq!(
        dest.context(&reader, json!({})).unwrap()["records"],
        json!([])
    );
}
#[test]
fn clock_rollback_never_makes_a_correction_unreadable() {
    let dir = tempfile::tempdir().unwrap();
    let mut b = fixture()["bundle"].clone();
    let future = "2099-01-01T00:00:00.000Z";
    for r in b["records"].as_array_mut().unwrap() {
        r["createdAt"] = json!(future);
        r["updatedAt"] = json!(future);
    }
    for op in b["operations"].as_array_mut().unwrap() {
        op["receipt"]["committedAt"] = json!(future);
        op["receipt"]["record"]["createdAt"] = json!(future);
        op["receipt"]["record"]["updatedAt"] = json!(future);
    }
    let mut s = Store::restore(&dir.path().join("future.sqlite"), b.clone()).unwrap();
    let a = grant(&mut s);
    let w = s.workspace().unwrap();
    let old = b["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "memory")
        .unwrap();
    let r=s.write(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"id":old["id"],"expectedRevision":old["revision"],"record":{"kind":"memory","title":"Tea","content":"Corrected","status":"active","basis":"owner_statement"}})).unwrap();
    assert_eq!(r["record"]["updatedAt"], future);
    assert_eq!(
        s.get(&a, r["record"]["id"].as_str().unwrap()).unwrap(),
        r["record"]
    );
    s.export().unwrap();
}
#[test]
fn admission_bound_preserves_exportability() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.sqlite");
    let mut s = Store::initialize(&path, "persona").unwrap();
    let a = grant(&mut s);
    let w = s.workspace().unwrap();
    let request = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Tea","content":"Green","status":"active","basis":"owner_statement"}});
    let first = s.write(&a, request.clone()).unwrap();
    let mut db = rusqlite::Connection::open(&path).unwrap();
    let tx = db.transaction().unwrap();
    for _ in 1..10000 {
        let mut r = first["record"].clone();
        r["id"] = json!(Uuid::new_v4().to_string());
        tx.execute(
            "INSERT INTO records VALUES (?1,'memory','active',1,?2,'Tea','Green',?3)",
            rusqlite::params![
                r["id"].as_str().unwrap(),
                r["updatedAt"].as_str().unwrap(),
                r.to_string()
            ],
        )
        .unwrap();
    }
    let bytes:i64=tx.query_row("SELECT (SELECT SUM(length(CAST(payload AS BLOB))) FROM records)+(SELECT SUM(length(CAST(receipt AS BLOB))) FROM operations)",[],|r|r.get(0)).unwrap();
    tx.execute(
        "UPDATE metadata SET value=? WHERE key='usage'",
        [json!({"records":10000,"operations":1,"payloadBytes":bytes}).to_string()],
    )
    .unwrap();
    tx.commit().unwrap();
    drop(db);
    let mut q = request.clone();
    q["operationId"] = json!(Uuid::new_v4().to_string());
    check_err(s.write(&a, q.clone()), "RESOURCE_LIMIT");
    check_err(
        s.receipt(&a, q["operationId"].as_str().unwrap()),
        "NOT_FOUND",
    );
    assert_eq!(
        s.export().unwrap()["records"].as_array().unwrap().len(),
        10000
    );
    assert_eq!(s.context(&a, json!({})).unwrap()["omitted"], 9980);
    q["id"] = first["record"]["id"].clone();
    q["expectedRevision"] = json!(1);
    assert_eq!(s.write(&a, q).unwrap()["record"]["revision"], 2);
}
