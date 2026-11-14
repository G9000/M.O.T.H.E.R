//! A successful write must fit the mandatory single-record response envelope.
use mother::{
    access::ClientAccess,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, GardenErrorCode, GetRequest, MAX_JSON_BYTES},
        store::GardenStore,
    },
};
use serde_json::json;

#[test]
fn every_accepted_near_limit_record_can_be_read() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let mut committed = 0;
    for reserve in [1400, 1000, 800, 700, 600, 500, 400] {
        let request: BatchRequest = serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,
            "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
            {"itemKey":"entity","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person",
                "displayName":"Synthetic bounds","aliases":[],"extensions":{"test:blob":{"schemaVersion":1,"payload":"x".repeat(MAX_JSON_BYTES-reserve)}}}}
        ]})).unwrap();
        match store.apply(&auth, &request) {
            Ok(response) => {
                committed += 1;
                let get: GetRequest = serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,
                    "requiredExtensions":{},"reference":response.receipt.results[0].reference,"revision":null,"knownAt":null})).unwrap();
                assert!(
                    store.get(&auth, &get).is_ok(),
                    "accepted record with reserve {reserve} cannot be read"
                );
            }
            Err(error) => assert_eq!(error.code, GardenErrorCode::LimitExceeded),
        }
        assert_eq!(store.audit(&auth).unwrap().sequence, committed);
    }
    assert!(committed > 0);
}
