use mother::{
    access::ClientAccess,
    garden::{
        access::GardenAccess,
        config::{GardenClientConfig, load_garden_config},
        store::GardenStore,
    },
};
use serde_json::json;

#[test]
fn graph_config_requires_explicit_backend_and_redacts_credentials() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("garden.redb");
    let store = GardenStore::initialize(
        &path,
        &uuid::Uuid::new_v4().to_string(),
        &uuid::Uuid::new_v4().to_string(),
    )
    .unwrap();
    let auth = store
        .grant(
            "synthetic",
            &["garden:propose".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let value = json!({"version":2,"backend":"garden_redb","databasePath":path,"clientId":auth.client_id,"token":auth.token});
    let config_path = dir.path().join("graph.private.json");
    std::fs::write(&config_path, serde_json::to_vec(&value).unwrap()).unwrap();
    let config = load_garden_config(&config_path).unwrap();
    assert!(!format!("{config:?}").contains(&auth.token));
    config.authorize(&store).unwrap();
    store.revoke("synthetic").unwrap();
    assert_eq!(config.authorize(&store).unwrap_err().code(), "UNAUTHORIZED");
    for invalid in [
        json!({"version":2,"databasePath":path,"clientId":"synthetic","token":"a".repeat(64)}),
        json!({"version":1,"backend":"garden_redb","databasePath":path,"clientId":"synthetic","token":"a".repeat(64)}),
        json!({"version":2,"backend":"sqlite","databasePath":path,"clientId":"synthetic","token":"a".repeat(64)}),
        json!({"version":2,"backend":"garden_redb","databasePath":"relative.redb","clientId":"synthetic","token":"a".repeat(64)}),
        json!({"version":2,"backend":"garden_redb","databasePath":path,"clientId":"synthetic","token":"a".repeat(64),"subject":"spoof"}),
    ] {
        std::fs::write(&config_path, serde_json::to_vec(&invalid).unwrap()).unwrap();
        assert!(load_garden_config(&config_path).is_err());
    }
    let _: GardenClientConfig = config;
}
