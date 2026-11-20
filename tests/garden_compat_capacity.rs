//! Synthetic native capacity acceptance beyond the unchanged small v1 export limits.
use mother::{access::ClientAccess, garden::store::GardenStore};
use serde_json::{Value, json};
use uuid::Uuid;

fn capacity_bundle() -> Value {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/v1.json")).unwrap();
    let mut bundle = fixture["bundle"].clone();
    let mut base = bundle["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|record| record["kind"] == "memory")
        .unwrap()
        .clone();
    base["references"] = json!([]);
    base["sources"] = json!([]);
    base["revision"] = json!(1);
    base["actor"] = json!("synthetic-capacity");
    base["status"] = json!("active");
    base["createdAt"] = json!("2026-11-08T20:50:17.682Z");
    base["updatedAt"] = base["createdAt"].clone();
    let records:Vec<Value>=(0..10_000).map(|index|{
        let mut record=base.clone();
        record["id"]=json!(format!("10000000-0000-4000-8000-{index:012x}"));
        record["title"]=json!(format!("Synthetic current memory {index}"));
        record["content"]=json!("Synthetic capacity fixture, with intentionally absent historical receipts permitted by v1 portability.");
        record
    }).collect();
    bundle["records"] = json!(records);
    bundle["operations"] = json!([]);
    bundle["omittedExternalSources"] = json!([]);
    let fields = bundle.as_object_mut().unwrap();
    for optional in ["inbox", "pins", "assignments"] {
        fields.remove(optional);
    }
    bundle
}

#[test]
fn native_accepts_ten_thousand_and_first_record_and_full_backup_retains_them_all() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let bundle = capacity_bundle();
    let workspace = bundle["workspace"]["id"].as_str().unwrap();
    let generation = bundle["workspace"]["generation"].as_str().unwrap();
    let store =
        GardenStore::initialize(dir.path().join("source.redb"), workspace, generation).unwrap();
    // Import goes through the ordinary small v1 validator before entering native storage.
    store.import_v1_bundle(bundle.clone()).unwrap();
    let auth = store
        .v1_grant(
            "synthetic-capacity",
            &["read".into(), "memory:write".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    let before = store
        .v1_call(&auth, "mother_get_context", json!({}))
        .unwrap();
    assert_eq!(before["inventory"]["totalRecords"], 10_000);
    assert_eq!(before["records"].as_array().unwrap().len(), 20);
    let request = json!({"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"First record beyond portable limit","content":"Native capacity is independent of the small export contract.","status":"active","basis":"owner_statement"}});
    let receipt = store
        .v1_call(&auth, "mother_write_record", request.clone())
        .unwrap();
    assert_eq!(receipt["record"]["revision"], 1);
    assert_eq!(
        store
            .v1_call(&auth, "mother_write_record", request)
            .unwrap(),
        receipt
    );
    assert_eq!(
        store.v1_export().unwrap_err().code(),
        "FULL_BACKUP_REQUIRED"
    );
    let backup = dir.path().join("full-backup");
    store.backup_to(&backup).unwrap();
    GardenStore::verify_backup(&backup).unwrap();
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(backup.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["sections"]["garden_v1_records"], 10_001);
    let restored = GardenStore::restore_backup(&backup, &dir.path().join("restored.redb")).unwrap();
    assert_eq!(
        restored.v1_authorize(&auth, "read").unwrap_err().code(),
        "UNAUTHORIZED"
    );
    let read = restored
        .v1_grant(
            "synthetic-reader",
            &["read".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    let context = restored
        .v1_call(&read, "mother_get_context", json!({}))
        .unwrap();
    assert_eq!(context["inventory"]["totalRecords"], 10_001);
    assert_eq!(context["records"].as_array().unwrap().len(), 20);
    assert_eq!(context["selection"]["omittedRecords"], 9_981);
    assert_eq!(
        restored
            .v1_call(
                &read,
                "mother_get_record",
                json!({"id":receipt["record"]["id"]})
            )
            .unwrap(),
        receipt["record"]
    );
    assert_eq!(
        restored.v1_export().unwrap_err().code(),
        "FULL_BACKUP_REQUIRED"
    );
}
