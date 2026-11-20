use super::*;

#[test]
fn missing_profile_policy_reports_unavailability_without_legacy_fallback() {
    let fixture = Fixture::new("Synthetic persona");
    let mut client = Client::connect(&fixture.a_config);
    let reply = client.result("mother_get_context", json!({"profile":"unconfigured"}));
    assert_eq!(reply["isError"], true);
    assert_eq!(
        reply["structuredContent"]["error"]["code"],
        "RECALL_PROFILE_UNAVAILABLE"
    );
}

#[test]
fn twelve_pins_remain_readable_over_stdio_with_complete_record_readback() {
    let fixture = Fixture::new(&"Synthetic persona. ".repeat(280));
    let mut client = Client::connect(&fixture.a_config);
    let context = client.call("mother_get_context", json!({"limit":1}));
    let workspace = &context["workspace"];
    let content = "Synthetic multilingual memory 記憶🪽\\\". "
        .repeat(100)
        .trim()
        .to_owned();
    let mut ids = Vec::new();
    for index in 0..12 {
        let receipt = client.call(
            "mother_write_record",
            json!({
                "workspaceId":workspace["id"],"generation":workspace["generation"],
                "operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,
                "record":{"kind":"memory","title":format!("Synthetic pin {index}"),
                    "content":content,"status":"active","basis":"owner_statement"}
            }),
        );
        let id = receipt["record"]["id"].as_str().unwrap().to_string();
        client.call("mother_pin_record", json!({"id":id,"pinned":true}));
        ids.push(id);
        if index == 2 || index == 11 {
            let reply = client.result("mother_get_context", json!({"limit":1}));
            assert_ne!(reply["isError"], true, "{reply}");
            assert!(serde_json::to_vec(&reply).unwrap().len() <= 48 * 1024);
            let value = &reply["structuredContent"];
            assert!(serde_json::to_vec(value).unwrap().len() <= 12 * 1024);
            assert_eq!(value["pinnedSummary"]["shown"], index + 1);
            assert_eq!(value["persona"], context["persona"]);
            assert_eq!(value["responseBudget"]["pinnedContentTruncated"], true);
            for id in &ids {
                assert!(
                    value["pinned"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|pin| pin["id"] == *id)
                );
                let record = client.call("mother_get_record", json!({"id":id}));
                assert_eq!(record["content"], content);
            }
        }
    }
}
