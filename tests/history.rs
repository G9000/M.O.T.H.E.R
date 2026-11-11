//! Earlier revisions of a record can be read back from its retained receipts.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
use std::{path::PathBuf, thread::sleep, time::Duration};
use uuid::Uuid;
struct Fixture {
    dir: tempfile::TempDir,
    store: Store,
    auth: Auth,
}
fn client(store: &mut Store, name: &str) -> Auth {
    let grant = store.grant(name, &SCOPES.map(str::to_owned)).unwrap();
    Auth {
        client_id: name.into(),
        token: grant["token"].as_str().unwrap().into(),
    }
}
fn fixture() -> Fixture {
    let dir = tempfile::Builder::new()
        .prefix("mother-history-")
        .tempdir()
        .unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic history").unwrap();
    let auth = client(&mut store, "synthetic");
    Fixture { dir, store, auth }
}
impl Fixture {
    fn database(&self) -> PathBuf {
        self.dir.path().join("context.sqlite")
    }
}
fn memory(content: &str, status: &str) -> Value {
    json!({"kind":"memory","status":status,"title":"Tea","content":content,"basis":"owner_statement"})
}
/// Create or revise a record and return its new state.
fn save(s: &mut Store, a: &Auth, record: Value, previous: Option<&Value>) -> Value {
    let w = s.workspace().unwrap();
    let mut q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record});
    if let Some(p) = previous {
        q["id"] = p["id"].clone();
        q["expectedRevision"] = p["revision"].clone();
    }
    let saved = s.write(a, q).unwrap()["record"].clone();
    sleep(Duration::from_millis(2));
    saved
}
fn revisions(history: &Value) -> Vec<u64> {
    history["revisions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["revision"].as_u64().unwrap())
        .collect()
}
#[test]
fn history_lists_every_revision_newest_first_with_who_changed_what() {
    let mut f = fixture();
    let other = client(&mut f.store, "other");
    let first = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    let second = save(
        &mut f.store,
        &other,
        memory("Oolong", "active"),
        Some(&first),
    );
    let third = save(
        &mut f.store,
        &f.auth,
        memory("Oolong", "retracted"),
        Some(&second),
    );
    let h = f.store.history(&f.auth, json!({"id":first["id"]})).unwrap();
    assert_eq!(h["id"], first["id"]);
    assert_eq!(h["kind"], "memory");
    assert_eq!(h["currentRevision"], 3);
    assert_eq!(h["currentStatus"], "retracted");
    assert_eq!(revisions(&h), [3, 2, 1]);
    assert_eq!(h["returnedRevisions"], 3);
    assert_eq!(h["omittedRevisions"], 0);
    assert_eq!(h["missingRevisions"], json!([]));
    let entries = h["revisions"].as_array().unwrap();
    assert_eq!(entries[0]["actor"], "synthetic");
    assert_eq!(entries[0]["status"], "retracted");
    assert_eq!(entries[0]["changed"], json!(["status"]));
    assert_eq!(entries[0]["committedAt"], third["updatedAt"]);
    assert_eq!(entries[1]["actor"], "other");
    assert_eq!(entries[1]["excerpt"], "Oolong");
    assert_eq!(entries[1]["changed"], json!(["content"]));
    assert_eq!(entries[2]["excerpt"], "Green tea");
    assert_eq!(entries[2]["truncated"], false);
    assert_eq!(
        entries[2]["changed"],
        Value::Null,
        "the first revision has nothing earlier to compare"
    );
    assert!(entries.iter().all(|e| e["currentGeneration"] == true));
}
#[test]
fn one_earlier_revision_can_be_read_in_full() {
    let mut f = fixture();
    let first = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    let second = save(
        &mut f.store,
        &f.auth,
        memory("Oolong", "active"),
        Some(&first),
    );
    let old = f
        .store
        .history(&f.auth, json!({"id":first["id"],"revision":1}))
        .unwrap();
    assert_eq!(old["record"], first);
    assert_eq!(old["current"], false);
    assert_eq!(old["currentRevision"], 2);
    assert_eq!(old["entry"]["revision"], 1);
    let latest = f
        .store
        .history(&f.auth, json!({"id":first["id"],"revision":2}))
        .unwrap();
    assert_eq!(latest["record"], second);
    assert_eq!(latest["current"], true);
    assert_eq!(
        f.store
            .history(&f.auth, json!({"id":first["id"],"revision":3}))
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
}
#[test]
fn the_list_is_bounded_and_still_compares_against_omitted_revisions() {
    let mut f = fixture();
    let mut record = save(&mut f.store, &f.auth, memory("Version 1", "active"), None);
    for n in 2..=5 {
        let next = memory(&format!("Version {n}"), "active");
        record = save(&mut f.store, &f.auth, next, Some(&record));
    }
    let h = f
        .store
        .history(&f.auth, json!({"id":record["id"],"limit":2}))
        .unwrap();
    assert_eq!(revisions(&h), [5, 4]);
    assert_eq!(h["returnedRevisions"], 2);
    assert_eq!(h["omittedRevisions"], 3);
    assert_eq!(h["revisions"][1]["changed"], json!(["content"]));
}
#[test]
fn records_that_only_reference_this_one_are_not_its_revisions() {
    let mut f = fixture();
    let note = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    let checkpoint = json!({"kind":"checkpoint","status":"active","title":"Handoff","content":"Mentions the tea memory","nextStep":"none","references":[note["id"]]});
    save(&mut f.store, &f.auth, checkpoint, None);
    let h = f.store.history(&f.auth, json!({"id":note["id"]})).unwrap();
    assert_eq!(revisions(&h), [1]);
}
#[test]
fn a_superseded_checkpoint_shows_its_retirement() {
    let mut f = fixture();
    let checkpoint = |title: &str| json!({"kind":"checkpoint","status":"active","title":title,"content":"Synthetic","nextStep":"none"});
    let old = save(&mut f.store, &f.auth, checkpoint("Monday"), None);
    let w = f.store.workspace().unwrap();
    f.store.write(&f.auth, json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":checkpoint("Tuesday"),"supersedes":{"id":old["id"],"expectedRevision":1}})).unwrap();
    let h = f.store.history(&f.auth, json!({"id":old["id"]})).unwrap();
    assert_eq!(revisions(&h), [2, 1]);
    assert_eq!(h["revisions"][0]["status"], "superseded");
    assert_eq!(h["revisions"][0]["changed"], json!(["status"]));
}
#[test]
fn a_revision_without_a_retained_receipt_is_reported_not_invented() {
    let mut f = fixture();
    let first = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    let second = save(
        &mut f.store,
        &f.auth,
        memory("Oolong", "active"),
        Some(&first),
    );
    save(
        &mut f.store,
        &f.auth,
        memory("Oolong", "retracted"),
        Some(&second),
    );
    let conn = rusqlite::Connection::open(f.database()).unwrap();
    let removed = conn
        .execute(
            "DELETE FROM operations WHERE receipt LIKE '%\"revision\":2%'",
            [],
        )
        .unwrap();
    assert_eq!(removed, 1);
    drop(conn);
    let h = f.store.history(&f.auth, json!({"id":first["id"]})).unwrap();
    assert_eq!(revisions(&h), [3, 1]);
    assert_eq!(h["missingRevisions"], json!([2]));
    assert_eq!(
        h["revisions"][0]["changed"],
        Value::Null,
        "no snapshot of revision 2 exists to compare against"
    );
    assert_eq!(
        f.store
            .history(&f.auth, json!({"id":first["id"],"revision":2}))
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
}
#[test]
fn history_survives_restore_and_marks_revisions_from_an_earlier_generation() {
    let mut f = fixture();
    let first = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    save(
        &mut f.store,
        &f.auth,
        memory("Oolong", "active"),
        Some(&first),
    );
    let bundle = f.store.export().unwrap();
    let mut restored = Store::restore(&f.dir.path().join("restored.sqlite"), bundle).unwrap();
    let auth = client(&mut restored, "restored");
    let current = restored.get(&auth, first["id"].as_str().unwrap()).unwrap();
    save(
        &mut restored,
        &auth,
        memory("Jasmine", "active"),
        Some(&current),
    );
    let h = restored.history(&auth, json!({"id":first["id"]})).unwrap();
    assert_eq!(revisions(&h), [3, 2, 1]);
    let entries = h["revisions"].as_array().unwrap();
    assert_eq!(entries[0]["currentGeneration"], true);
    assert_eq!(entries[1]["currentGeneration"], false);
    assert_eq!(entries[2]["currentGeneration"], false);
    assert_eq!(entries[0]["actor"], "restored");
    assert_eq!(entries[1]["actor"], "synthetic");
}
#[test]
fn history_requires_a_current_full_context_read_grant() {
    let mut f = fixture();
    let note = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    let id = note["id"].as_str().unwrap().to_owned();
    let grant = f
        .store
        .grant_with_access(
            "shared",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: vec![id.clone()],
            },
        )
        .unwrap();
    let shared = Auth {
        client_id: "shared".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    assert_eq!(
        f.store
            .history(&shared, json!({"id":id}))
            .unwrap_err()
            .code(),
        "FORBIDDEN",
        "a shared client sees the granted record as it is now, not what it used to say"
    );
    assert_eq!(
        f.store
            .history(&f.auth, json!({"id":Uuid::new_v4().to_string()}))
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    f.store.revoke("synthetic").unwrap();
    assert_eq!(
        f.store
            .history(&f.auth, json!({"id":id}))
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
}
#[test]
fn malformed_history_requests_are_rejected() {
    let mut f = fixture();
    let note = save(&mut f.store, &f.auth, memory("Green tea", "active"), None);
    for bad in [
        json!({}),
        json!({"id":"not-a-uuid"}),
        json!({"id":note["id"],"revision":0}),
        json!({"id":note["id"],"limit":0}),
        json!({"id":note["id"],"limit":101}),
        json!({"id":note["id"],"revision":1,"limit":5}),
        json!({"id":note["id"],"actor":"other"}),
    ] {
        assert_eq!(
            f.store.history(&f.auth, bad.clone()).unwrap_err().code(),
            "INVALID_INPUT",
            "{bad}"
        );
    }
}
