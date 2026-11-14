use super::*;

#[test]
fn capability_discovery_is_persona_independent_and_does_not_enable_graph_tools() {
    let f = Fixture::new();
    let mut client = Client::connect(&f.a_config);
    let before = client.call("mother_get_capabilities", json!({}));
    assert_eq!(before["garden"]["contractVersion"], 2);
    assert_eq!(before["garden"]["status"], "contracts_only");
    assert_eq!(before["garden"]["available"], false);
    assert_eq!(before["garden"]["capabilities"], json!([]));
    assert_eq!(
        before["garden"]["definedTools"].as_array().unwrap().len(),
        7
    );
    let future: Value =
        serde_json::from_str(include_str!("../../contracts/garden-tools-v2.json")).unwrap();
    assert_eq!(
        before["garden"]["definedTools"],
        Value::Array(
            future["tools"]
                .as_array()
                .unwrap()
                .iter()
                .map(|tool| tool["name"].clone())
                .collect()
        )
    );
    assert!(
        serde_json::to_vec(&before).unwrap().len() < 4096,
        "bootstrap discovery must remain bounded"
    );
    assert!(before.get("persona").is_none());
    assert!(before.get("workspace").is_none());
    let context = client.call("mother_get_context", json!({}));
    let soul = f.dir.path().join("updated-synthetic-soul.md");
    std::fs::write(
        &soul,
        "A different synthetic persona for capability isolation",
    )
    .unwrap();
    cli(
        f.dir.path(),
        &[
            "soul",
            "update",
            "--file",
            soul.to_str().unwrap(),
            "--expected-hash",
            context["persona"]["hash"].as_str().unwrap(),
        ],
    );
    assert_eq!(client.call("mother_get_capabilities", json!({})), before);
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
    let f = Fixture::new();
    cli(
        f.dir.path(),
        &[
            "grant",
            "shared-contract-reader",
            "--access",
            "shared",
            "--subject",
            "synthetic-graph-agent",
            "--scopes",
            "read",
        ],
    );
    let config = f
        .dir
        .path()
        .join("clients/shared-contract-reader.private.json");
    let mut client = Client::connect(&config);
    let capabilities = client.call("mother_get_capabilities", json!({}));
    assert_eq!(capabilities["garden"]["available"], false);
    assert!(capabilities.get("grants").is_none());
    assert_eq!(
        client.call("mother_get_context", json!({}))["inventory"]["totalRecords"],
        0
    );
    cli(f.dir.path(), &["revoke", "shared-contract-reader"]);
    assert_eq!(
        client.result("mother_get_capabilities", json!({}))["isError"],
        true
    );
}
