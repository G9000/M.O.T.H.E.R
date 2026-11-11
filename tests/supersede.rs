//! A new checkpoint can retire the one it replaces in the same transaction.
use mother::{
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
use uuid::Uuid;
fn fixture() -> (tempfile::TempDir, Store, Auth) {
    let dir = tempfile::Builder::new()
        .prefix("mother-supersede-")
        .tempdir()
        .unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic supersession").unwrap();
    let grant = store
        .grant("synthetic", &SCOPES.map(str::to_owned))
        .unwrap();
    let auth = Auth {
        client_id: "synthetic".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    (dir, store, auth)
}
fn checkpoint(title: &str) -> Value {
    json!({"kind":"checkpoint","status":"active","title":title,"content":"Synthetic context only","nextStep":"Synthetic next step"})
}
fn memory(title: &str) -> Value {
    json!({"kind":"memory","status":"active","title":title,"content":"Synthetic context only","basis":"owner_statement"})
}
fn request(s: &Store, record: Value) -> Value {
    let w = s.workspace().unwrap();
    json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})
}
fn create(s: &mut Store, a: &Auth, record: Value) -> Value {
    let q = request(s, record);
    s.write(a, q).unwrap()["record"].clone()
}
fn superseding(s: &Store, title: &str, target: &Value) -> Value {
    let mut q = request(s, checkpoint(title));
    q["supersedes"] = json!({"id":target["id"],"expectedRevision":target["revision"]});
    q
}
fn total(s: &mut Store, a: &Auth) -> Value {
    s.context(a, json!({})).unwrap()["inventory"]["totalRecords"].clone()
}
#[test]
fn a_new_checkpoint_retires_the_one_it_names() {
    let (_dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    let q = superseding(&s, "Tuesday handoff", &old);
    let new = s.write(&a, q).unwrap()["record"].clone();
    assert_eq!(new["status"], "active");
    assert_eq!(new["revision"], 1);
    let retired = s.get(&a, old["id"].as_str().unwrap()).unwrap();
    assert_eq!(retired["status"], "superseded");
    assert_eq!(retired["revision"], 2);
    assert_eq!(retired["actor"], "synthetic");
    assert_eq!(retired["title"], "Monday handoff");
    assert_eq!(retired["createdAt"], old["createdAt"]);
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(c["inventory"]["recallableByKind"]["checkpoint"], 1);
    assert_eq!(c["inventory"]["excludedRecords"], 1);
    assert_eq!(c["records"][0]["title"], "Tuesday handoff");
    assert_eq!(c["records"].as_array().unwrap().len(), 1);
}
#[test]
fn a_stale_target_revision_rejects_the_whole_write() {
    let (_dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    let mut q = superseding(&s, "Tuesday handoff", &old);
    q["supersedes"]["expectedRevision"] = json!(7);
    assert_eq!(s.write(&a, q).unwrap_err().code(), "CONFLICT");
    assert_eq!(total(&mut s, &a), 1);
    let unchanged = s.get(&a, old["id"].as_str().unwrap()).unwrap();
    assert_eq!(unchanged, old);
}
#[test]
fn an_already_retired_or_missing_target_rejects_the_whole_write() {
    let (_dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    let q = superseding(&s, "Tuesday handoff", &old);
    s.write(&a, q).unwrap();
    let retired = s.get(&a, old["id"].as_str().unwrap()).unwrap();
    let q = superseding(&s, "Wednesday handoff", &retired);
    assert_eq!(s.write(&a, q).unwrap_err().code(), "CONFLICT");
    let mut q = superseding(&s, "Wednesday handoff", &retired);
    q["supersedes"]["id"] = json!(Uuid::new_v4().to_string());
    assert_eq!(s.write(&a, q).unwrap_err().code(), "NOT_FOUND");
    assert_eq!(total(&mut s, &a), 2);
}
#[test]
fn only_checkpoints_supersede_and_only_checkpoints_are_superseded() {
    let (_dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    let note = create(&mut s, &a, memory("Tea preference"));
    let mut q = request(&s, memory("Another memory"));
    q["supersedes"] = json!({"id":old["id"],"expectedRevision":1});
    assert_eq!(s.write(&a, q).unwrap_err().code(), "INVALID_INPUT");
    let q = superseding(&s, "Tuesday handoff", &note);
    assert_eq!(s.write(&a, q).unwrap_err().code(), "KIND_MISMATCH");
    assert_eq!(total(&mut s, &a), 2);
    assert_eq!(
        s.get(&a, note["id"].as_str().unwrap()).unwrap()["status"],
        "active"
    );
}
#[test]
fn malformed_supersedes_input_is_rejected() {
    let (_dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    for bad in [
        json!("not-an-object"),
        json!({"id":old["id"]}),
        json!({"id":"not-a-uuid","expectedRevision":1}),
        json!({"id":old["id"],"expectedRevision":0}),
        json!({"id":old["id"],"expectedRevision":1,"status":"retracted"}),
    ] {
        let mut q = request(&s, checkpoint("Tuesday handoff"));
        q["supersedes"] = bad.clone();
        assert_eq!(s.write(&a, q).unwrap_err().code(), "INVALID_INPUT", "{bad}");
    }
    // A checkpoint cannot retire itself while being revised.
    let mut q = request(&s, checkpoint("Monday handoff, revised"));
    q["id"] = old["id"].clone();
    q["expectedRevision"] = json!(1);
    q["supersedes"] = json!({"id":old["id"],"expectedRevision":1});
    assert_eq!(s.write(&a, q).unwrap_err().code(), "INVALID_INPUT");
    assert_eq!(total(&mut s, &a), 1);
}
#[test]
fn a_retry_returns_the_same_receipt_without_retiring_twice() {
    let (_dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    let q = superseding(&s, "Tuesday handoff", &old);
    let first = s.write(&a, q.clone()).unwrap();
    assert_eq!(s.write(&a, q.clone()).unwrap(), first);
    assert_eq!(
        s.get(&a, old["id"].as_str().unwrap()).unwrap()["revision"],
        2
    );
    assert_eq!(total(&mut s, &a), 2);
    // The same operation ID cannot be reused to retire a different checkpoint.
    let other = create(&mut s, &a, checkpoint("Unrelated handoff"));
    let mut changed = q;
    changed["supersedes"] = json!({"id":other["id"],"expectedRevision":1});
    assert_eq!(s.write(&a, changed).unwrap_err().code(), "OPERATION_REUSED");
    assert_eq!(
        s.get(&a, other["id"].as_str().unwrap()).unwrap()["status"],
        "active"
    );
}
#[test]
fn retirement_has_its_own_receipt_and_survives_export_and_restore() {
    let (dir, mut s, a) = fixture();
    let old = create(&mut s, &a, checkpoint("Monday handoff"));
    let q = superseding(&s, "Tuesday handoff", &old);
    s.write(&a, q).unwrap();
    let bundle = s.export().unwrap();
    let operations = bundle["operations"].as_array().unwrap();
    assert_eq!(operations.len(), 3);
    let retirement = operations
        .iter()
        .find(|op| op["receipt"]["record"]["status"] == "superseded")
        .expect("the retirement is an attributed, receipted mutation");
    assert_eq!(retirement["actor"], "synthetic");
    assert_eq!(retirement["receipt"]["record"]["id"], old["id"]);
    assert_eq!(retirement["receipt"]["record"]["revision"], 2);
    let restored = Store::restore(&dir.path().join("restored.sqlite"), bundle).unwrap();
    let again = restored.grant_free_inventory();
    assert_eq!(again, (2, 1));
}
trait Inventory {
    fn grant_free_inventory(self) -> (i64, i64);
}
impl Inventory for Store {
    /// Total and recallable record counts in a restored store, read through a fresh grant.
    fn grant_free_inventory(mut self) -> (i64, i64) {
        let grant = self.grant("restored", &["read".to_owned()]).unwrap();
        let auth = Auth {
            client_id: "restored".into(),
            token: grant["token"].as_str().unwrap().into(),
        };
        let c = self.context(&auth, json!({})).unwrap();
        (
            c["inventory"]["totalRecords"].as_i64().unwrap(),
            c["inventory"]["recallableRecords"].as_i64().unwrap(),
        )
    }
}
