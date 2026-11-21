use super::*;

#[test]
fn capability_discovery_is_persona_independent_and_does_not_enable_graph_tools() {
    let first = Fixture::new("Synthetic first persona");
    let second = Fixture::new("A different synthetic owner persona");
    let mut client = Client::connect(&first.a_config);
    let before = client.call("mother_get_capabilities", json!({}));
    let mut another = Client::connect(&second.a_config);
    let after = another.call("mother_get_capabilities", json!({}));
    assert_eq!(before["garden"], after["garden"]);
    assert_eq!(before["v1"], after["v1"]);
    assert_eq!(before["garden"]["contractVersion"], 2);
    assert_eq!(before["garden"]["available"], false);
    assert_eq!(before["garden"]["capabilities"], json!([]));
    assert_eq!(before["v1"]["available"], true);
    let future: Value =
        serde_json::from_str(include_str!("../../contracts/garden-tools-v2.json")).unwrap();
    let media: Value =
        serde_json::from_str(include_str!("../../contracts/media-tools.json")).unwrap();
    assert_eq!(
        before["garden"]["definedTools"],
        Value::Array(
            future["tools"]
                .as_array()
                .unwrap()
                .iter()
                .chain(media["tools"].as_array().unwrap())
                .map(|tool| tool["name"].clone())
                .collect()
        )
    );
    assert_eq!(before["garden"]["availableTools"], json!([]));
    assert!(serde_json::to_vec(&before).unwrap().len() < 4096);
    assert!(before.get("persona").is_none());
    assert_ne!(
        client.call("mother_get_context", json!({}))["persona"]["content"],
        another.call("mother_get_context", json!({}))["persona"]["content"]
    );
    let catalogue = client.rpc("tools/list", json!({}));
    assert!(
        !catalogue["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"].as_str().unwrap().starts_with("mother_garden_"))
    );
    let invalid = client.result(
        "mother_get_capabilities",
        json!({"personaSource":"workspace"}),
    );
    assert_eq!(invalid["isError"], true);
    assert_eq!(
        invalid["structuredContent"]["error"]["code"],
        "INVALID_INPUT"
    );
}

#[test]
fn shared_empty_v1_allowlist_can_discover_contracts_without_graph_authority() {
    use mother::{
        access::{AccessMode, ClientAccess},
        garden::store::GardenStore,
    };
    let fixture = Fixture::new("Synthetic owner persona");
    let store = GardenStore::open_active(&fixture.database).unwrap();
    let auth = store
        .v1_grant(
            "shared-contract-reader",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("synthetic-graph-agent".into()),
                records: vec![],
            },
            None,
        )
        .unwrap();
    let config = fixture.dir.path().join("shared.private.json");
    std::fs::write(&config,serde_json::to_vec(&json!({"version":2,"backend":"garden_redb","databasePath":fixture.database,"clientId":auth.client_id,"token":auth.token})).unwrap()).unwrap();
    drop(store);
    let mut client = Client::connect(&config);
    let capabilities = client.call("mother_get_capabilities", json!({}));
    assert_eq!(capabilities["garden"]["available"], false);
    assert!(capabilities.get("grants").is_none());
    assert_eq!(
        client.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        0
    );
    drop(client);
    let store = GardenStore::open_active(&fixture.database).unwrap();
    store.revoke("shared-contract-reader").unwrap();
    drop(store);
    let rejected = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["mcp", "--config"])
        .arg(config)
        .env("MOTHER_HOME", fixture.dir.path())
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!rejected.status.success());
}
