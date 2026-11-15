//! Native v1 replay preserves the TypeScript fixture's numeric normalization and hashes.
use mother::{access::ClientAccess, garden::store::GardenStore};
use serde_json::{Value, json};

#[test]
fn integral_json_numbers_replay_typescript_golden_hashes_and_receipts_on_native_storage() {
    let dir = tempfile::tempdir().unwrap();
    let fixture: Value = serde_json::from_str(include_str!("fixtures/v1.json")).unwrap();
    let bundle = &fixture["bundle"];
    let store = GardenStore::initialize(
        dir.path().join("native.redb"),
        bundle["workspace"]["id"].as_str().unwrap(),
        bundle["workspace"]["generation"].as_str().unwrap(),
    )
    .unwrap();
    store.import_v1_bundle(bundle.clone()).unwrap();
    let auth = store
        .v1_grant(
            "fixture-client",
            &["read".into(), "memory:write".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();

    // These requests must recompute the original TypeScript operation hashes:
    // numerically integral floats and JavaScript trimming are equivalent inputs.
    for index in [0, 1] {
        let mut request = fixture["requests"][index].clone();
        request["expectedRevision"] = json!(index as f64);
        if index == 0 {
            request["record"]["title"] = json!("\u{feff} Tea 🫖 \u{00a0}");
            request["record"]["content"] = json!("\u{feff} Prefers green tea — café \u{00a0}");
        }
        assert_eq!(
            store
                .v1_call(&auth, "mother_write_record", request)
                .unwrap(),
            fixture["receipts"][index],
            "native replay must match the TypeScript golden receipt and request hash"
        );
    }
    assert_eq!(
        store
            .v1_call(&auth, "mother_get_context", json!({"limit":1.0}))
            .unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let mut changed = fixture["requests"][1].clone();
    changed["record"]["content"] = json!("A different synthetic payload");
    assert_eq!(
        store
            .v1_call(&auth, "mother_write_record", changed)
            .unwrap_err()
            .code(),
        "OPERATION_REUSED"
    );
    assert_eq!(
        store.v1_export().unwrap()["operations"],
        bundle["operations"]
    );
}
