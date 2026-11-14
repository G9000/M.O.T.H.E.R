use mother::garden::contracts::{GardenErrorCode, GardenRequest, GardenTool};
use serde_json::{Value, json};
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/garden-v2.json")).unwrap()
}
fn parse(value: &Value) -> Result<GardenRequest, mother::garden::contracts::GardenError> {
    GardenRequest::parse(GardenTool::Apply, value)
}
#[test]
fn fixture_and_unknown_extensions_round_trip() {
    let value = fixture();
    let typed = parse(&value).unwrap();
    assert_eq!(serde_json::to_value(typed).unwrap(), value);
}
#[test]
fn versions_and_required_extensions_fail_explicitly() {
    let mut value = fixture();
    value["version"] = json!(3);
    assert_eq!(
        parse(&value).unwrap_err().code,
        GardenErrorCode::UnsupportedVersion
    );
    let mut value = fixture();
    value["requiredExtensions"] = json!({"example:future":7});
    assert_eq!(
        parse(&value).unwrap_err().code,
        GardenErrorCode::UnsupportedExtension
    );
}
#[test]
fn protected_fields_never_enter_drafts() {
    for field in [
        "id",
        "revision",
        "actor",
        "contributor",
        "approver",
        "transactionTime",
        "commitSequence",
        "status",
        "visibility",
    ] {
        let mut value = fixture();
        value["items"][0]["draft"][field] = json!("forged");
        assert!(parse(&value).is_err(), "accepted {field}");
    }
}
#[test]
fn validates_half_open_utc_intervals_and_finite_confidence() {
    for interval in [
        json!({"validFrom":"2026-11-15T00:00:00Z","validUntil":"2026-11-15T00:00:00Z","sourceTimezone":null,"precision":"instant"}),
        json!({"validFrom":"2026-11-16T00:00:00Z","validUntil":"2026-11-15T00:00:00Z","sourceTimezone":null,"precision":"instant"}),
        json!({"validFrom":"2026-11-15T08:00:00+08:00","validUntil":null,"sourceTimezone":"Asia/Kuala_Lumpur","precision":"instant"}),
    ] {
        let mut value = fixture();
        value["items"][2]["draft"]["validTime"] = interval;
        assert!(parse(&value).is_err());
    }
    let mut value = fixture();
    value["items"][2]["draft"]["confidence"]["score"] = json!(1.01);
    assert!(parse(&value).is_err());
    let raw = fixture().to_string().replace("0.8", "1e9999");
    assert!(serde_json::from_str::<Value>(&raw).is_err());
}
#[test]
fn rejects_contradictory_objects_and_bad_batch_endpoints() {
    let mut value = fixture();
    value["items"][2]["draft"]["object"]["entityRef"] =
        json!({"refType":"item","kind":"entity","itemKey":"person"});
    assert!(parse(&value).is_err());
    for key in ["missing", "source"] {
        let mut value = fixture();
        value["items"][2]["draft"]["subject"]["itemKey"] = json!(key);
        assert!(parse(&value).is_err(), "accepted {key}");
    }
    let mut value = fixture();
    value["items"][1]["itemKey"] = json!("person");
    assert!(parse(&value).is_err());
    let mut value = fixture();
    value["items"][0]["expectedRevision"] = json!(1);
    assert!(parse(&value).is_err());
}
#[test]
fn bounds_names_aliases_batch_json_and_extension_depth() {
    let mut value = fixture();
    value["items"][0]["draft"]["displayName"] = json!("a".repeat(513));
    assert!(parse(&value).is_err());
    let mut value = fixture();
    value["items"][0]["draft"]["aliases"] = json!(vec!["x"; 33]);
    assert!(parse(&value).is_err());
    let mut value = fixture();
    value["items"] = json!(vec![value["items"][0].clone(); 65]);
    assert!(parse(&value).is_err());
    let mut deep = json!(null);
    for _ in 0..40 {
        deep = json!({"nested":deep});
    }
    let mut value = fixture();
    value["items"][0]["draft"]["extensions"]["example:future"]["payload"] = deep;
    assert!(parse(&value).is_err());
    let mut value = fixture();
    value["items"][0]["draft"]["extensions"]["example:future"]["payload"] =
        json!("x".repeat(270_000));
    assert!(parse(&value).is_err());
}
#[test]
fn v1_write_rejects_v2_fields() {
    use mother::{contracts::Auth, store::Store};
    let dir = tempfile::tempdir().unwrap();
    let mut s =
        Store::initialize(&dir.path().join("synthetic.sqlite"), "Synthetic persona").unwrap();
    let g = s
        .grant("synthetic-client", &["memory:write".into()])
        .unwrap();
    let a = Auth {
        client_id: "synthetic-client".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let w = s.workspace().unwrap();
    let value = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Synthetic","content":"Example","status":"active","basis":"owner_statement","extensions":{}}});
    assert_eq!(s.write(&a, value).unwrap_err().code(), "INVALID_INPUT");
}
#[test]
fn every_future_tool_has_resolvable_standalone_schemas() {
    let catalog: Value =
        serde_json::from_str(include_str!("../contracts/garden-tools-v2.json")).unwrap();
    assert_eq!(catalog["contractVersion"], 2);
    assert_eq!(catalog["status"], "contractOnly");
    let tools = catalog["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 7);
    fn check_refs(node: &Value, root: &Value) {
        match node {
            Value::Object(map) => {
                for (key, value) in map {
                    if key == "$ref" {
                        let reference = value.as_str().unwrap();
                        assert!(reference.starts_with("#/"));
                        assert!(
                            root.pointer(&reference[1..]).is_some(),
                            "unresolved {reference}"
                        );
                    } else {
                        check_refs(value, root);
                    }
                }
            }
            Value::Array(list) => {
                for value in list {
                    check_refs(value, root)
                }
            }
            _ => {}
        }
    }
    for tool in tools {
        assert!(GardenTool::parse(tool["name"].as_str().unwrap()).is_ok());
        for field in ["inputSchema", "outputSchema"] {
            check_refs(&tool[field], &tool[field]);
        }
    }
}
fn schemas() -> Value {
    serde_json::from_str(include_str!("../contracts/garden-tools-v2.json")).unwrap()
}
fn validate_schema(tool: GardenTool, field: &str, value: &Value) {
    let catalog = schemas();
    let descriptor = catalog["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == tool.name())
        .unwrap();
    let validator = jsonschema::options()
        .offline()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .build(&descriptor[field])
        .unwrap();
    if let Err(error) = validator.validate(value) {
        panic!("{} {field}: {error}", tool.name());
    }
}
fn record() -> Value {
    json!({"metadata":{"id":"44444444-4444-4444-8444-444444444444","revision":1,"previousRevision":null,"commitSequence":1,"transactionTime":"2026-11-15T00:00:00Z","actor":"synthetic-client","contributor":"synthetic-client","approver":null,"lifecycle":"active"},"draft":fixture()["items"][0]["draft"]})
}
fn coverage() -> Value {
    json!({"complete":true,"truncated":false,"reason":null,"canonicalWatermark":1,"indexWatermark":1,"searchMode":"exact","model":null,"profile":null})
}
fn page_state() -> Value {
    json!({"nextCursor":null,"visibilityEpoch":"synthetic-epoch","snapshotId":"synthetic-snapshot","snapshotExpiresAt":"2026-11-15T01:00:00Z"})
}
fn samples() -> Vec<(GardenTool, Value, Value)> {
    let batch = fixture();
    let base = json!({"version":2,"workspaceId":batch["workspaceId"],"generation":batch["generation"],"requiredExtensions":{}});
    let rb =
        json!({"version":2,"workspaceId":batch["workspaceId"],"generation":batch["generation"]});
    let reference = json!({"kind":"entity","id":record()["metadata"]["id"]});
    fn with(mut base: Value, values: Value) -> Value {
        base.as_object_mut()
            .unwrap()
            .extend(values.as_object().unwrap().clone());
        base
    }
    let outcome = with(
        rb.clone(),
        json!({"receipt":{"operationId":batch["operationId"],"operationGeneration":batch["generation"],"actor":"synthetic-client","status":"approved","proposalId":"55555555-5555-4555-8555-555555555555","committedAt":"2026-11-15T00:00:00Z","commitSequence":1,"results":[{"itemKey":"person","reference":reference,"revision":1,"ownerEdited":false,"ownerAdded":false},{"itemKey":"source","reference":{"kind":"evidence","id":"66666666-6666-4666-8666-666666666666"},"revision":1,"ownerEdited":false,"ownerAdded":false},{"itemKey":"claim","reference":{"kind":"assertion","id":"77777777-7777-4777-8777-777777777777"},"revision":1,"ownerEdited":false,"ownerAdded":false}]},"originalProposal":batch,"visibleRecords":[record()],"payloadOmitted":true,"visibilityEpoch":"synthetic-epoch"}),
    );
    let mut applied = outcome.clone();
    applied["receipt"]["status"] = json!("applied");
    applied["receipt"]["proposalId"] = Value::Null;
    applied["originalProposal"] = Value::Null;
    let outcome_request = with(
        base.clone(),
        json!({"operationId":batch["operationId"],"operationGeneration":batch["generation"]}),
    );
    vec![
        (
            GardenTool::Get,
            with(
                base.clone(),
                json!({"reference":reference,"revision":null,"knownAt":null}),
            ),
            with(
                rb.clone(),
                json!({"record":record(),"coverage":coverage(),"visibilityEpoch":"synthetic-epoch"}),
            ),
        ),
        (
            GardenTool::Query,
            with(
                base.clone(),
                json!({"ids":[],"kinds":[],"limit":10,"cursor":null}),
            ),
            with(
                rb.clone(),
                json!({"hits":[{"record":record(),"sources":[],"scores":{"lexical":null,"vector":null}}],"coverage":coverage(),"page":page_state()}),
            ),
        ),
        (GardenTool::Apply, batch.clone(), applied),
        (
            GardenTool::Propose,
            batch.clone(),
            with(
                rb.clone(),
                json!({"proposalId":"55555555-5555-4555-8555-555555555555","operationId":batch["operationId"],"status":"pending","visibilityEpoch":"synthetic-epoch"}),
            ),
        ),
        (
            GardenTool::GetProposal,
            outcome_request.clone(),
            outcome.clone(),
        ),
        (GardenTool::GetReceipt, outcome_request, outcome),
        (
            GardenTool::Changes,
            with(base, json!({"limit":10,"cursor":null})),
            with(
                rb,
                json!({"changes":[{"reference":reference,"revision":1,"sequence":1,"lifecycle":"active","record":record()}],"coverage":coverage(),"page":page_state()}),
            ),
        ),
    ]
}
#[test]
fn all_seven_request_response_contracts_validate_against_standalone_json_schemas() {
    use mother::garden::contracts::GardenResponse;
    for (tool, request, response) in samples() {
        GardenRequest::parse(tool, &request).unwrap();
        validate_schema(tool, "inputSchema", &request);
        let typed = GardenResponse::parse(tool, &response).unwrap();
        validate_schema(tool, "outputSchema", &response);
        assert_eq!(
            serde_json::to_value(typed).unwrap(),
            response,
            "response roundtrip for {}",
            tool.name()
        );
    }
}
#[test]
fn response_validation_rejects_forged_metadata_and_inconsistent_receipts() {
    use mother::garden::contracts::GardenResponse;
    let data = samples();
    let original = &data[2].2;
    for changes in [
        json!({"committedAt":null}),
        json!({"commitSequence":null}),
        json!({"results":[]}),
    ] {
        let mut value = original.clone();
        value["receipt"]
            .as_object_mut()
            .unwrap()
            .extend(changes.as_object().unwrap().clone());
        assert!(GardenResponse::parse(GardenTool::Apply, &value).is_err());
    }
    for field in ["committedAt", "commitSequence"] {
        let mut value = original.clone();
        value["receipt"]["status"] = json!("pending");
        value["receipt"]["results"] = json!([]);
        value["receipt"][field] = Value::Null;
        assert!(
            GardenResponse::parse(GardenTool::GetProposal, &value).is_err(),
            "accepted partial commit {field}"
        );
    }
    let mut get = data[0].2.clone();
    get["record"]["metadata"]["commitSequence"] = json!(2);
    assert!(GardenResponse::parse(GardenTool::Get, &get).is_err());
    let mut get = data[0].2.clone();
    get["record"]["metadata"]["previousRevision"] = json!(1);
    assert!(GardenResponse::parse(GardenTool::Get, &get).is_err());
    let mut query = data[1].2.clone();
    query["hits"][0]["record"]["metadata"]["commitSequence"] = json!(2);
    assert!(GardenResponse::parse(GardenTool::Query, &query).is_err());
    let mut get = data[0].2.clone();
    get["coverage"]["indexWatermark"] = json!(2);
    assert!(GardenResponse::parse(GardenTool::Get, &get).is_err());
    let mut get = data[0].2.clone();
    get["record"]["draft"] = fixture()["items"][2]["draft"].clone();
    assert!(GardenResponse::parse(GardenTool::Get, &get).is_err());
}
#[test]
fn structured_restart_errors_are_validated_without_claiming_cursor_runtime() {
    use mother::garden::contracts::GardenResponse;
    for (tool, _, _) in samples() {
        let value = json!({"code":"RESTART_REQUIRED","message":"Snapshot expired; restart the query","restartRequired":true});
        let typed = GardenResponse::parse(tool, &value).unwrap();
        assert_eq!(serde_json::to_value(typed).unwrap(), value);
        validate_schema(tool, "outputSchema", &value);
    }
    for value in [
        json!({"code":"RESTART_REQUIRED","message":"Snapshot expired","restartRequired":false}),
        json!({"code":"RESTART_REQUIRED","message":"","restartRequired":true}),
    ] {
        assert!(GardenResponse::parse(GardenTool::Query, &value).is_err());
    }
}
#[test]
fn public_drafts_reject_nonfinite_typed_numbers_before_serialization() {
    use mother::garden::contracts::TypedValue;
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(TypedValue::Number { value, unit: None }.validate().is_err());
    }
}
#[test]
fn externally_referenced_endpoints_are_returned_for_separate_authority_checks() {
    let mut value = fixture();
    value["items"][2]["draft"]["subject"] =
        json!({"refType":"existing","kind":"entity","id":"44444444-4444-4444-8444-444444444444"});
    let GardenRequest::Batch(batch) = parse(&value).unwrap() else {
        panic!("batch expected")
    };
    let references = batch.external_references();
    assert_eq!(references.len(), 1);
    assert_eq!(references[0].id, "44444444-4444-4444-8444-444444444444");
}
#[test]
fn omitted_only_batch_fields_reject_explicit_nulls_and_long_instants() {
    for field in ["id", "replacement"] {
        let mut value = fixture();
        value["items"][0][field] = Value::Null;
        assert!(parse(&value).is_err(), "accepted create {field}:null");
    }
    let mut value = fixture();
    value["items"][2]["draft"]["observedTime"] = json!({"at":"2026-11-15T00:00:00.123456789012345678901234567890Z","sourceTimezone":null,"precision":"instant"});
    assert!(parse(&value).is_err());
}
#[test]
fn approved_receipt_maps_all_original_item_keys_and_marks_owner_additions() {
    use mother::garden::contracts::GardenResponse;
    let mut response = samples()[4].2.clone();
    // The full visible proposal makes complete immutable item-key coverage checkable.
    response["receipt"]["results"].as_array_mut().unwrap().pop();
    assert!(GardenResponse::parse(GardenTool::GetProposal, &response).is_err());
    response["receipt"]["results"] = json!([
    {"itemKey":"person","reference":{"kind":"entity","id":"44444444-4444-4444-8444-444444444444"},"revision":1,"ownerEdited":false,"ownerAdded":false},
    {"itemKey":"source","reference":{"kind":"evidence","id":"66666666-6666-4666-8666-666666666666"},"revision":1,"ownerEdited":false,"ownerAdded":false},
    {"itemKey":"claim","reference":{"kind":"assertion","id":"77777777-7777-4777-8777-777777777777"},"revision":1,"ownerEdited":false,"ownerAdded":false}
    ]);
    assert!(GardenResponse::parse(GardenTool::GetProposal, &response).is_ok());
    response["receipt"]["results"][0]["ownerAdded"] = json!(true);
    assert!(GardenResponse::parse(GardenTool::GetProposal, &response).is_err());
}
#[test]
fn change_payload_commit_sequence_matches_change_sequence() {
    use mother::garden::contracts::GardenResponse;
    let mut response = samples()[6].2.clone();
    response["changes"][0]["record"]["metadata"]["commitSequence"] = json!(2);
    assert!(GardenResponse::parse(GardenTool::Changes, &response).is_err());
}
#[test]
fn success_outcome_statuses_match_the_invoked_tool() {
    use mother::garden::contracts::GardenResponse;
    let approved = samples()[4].2.clone();
    assert!(GardenResponse::parse(GardenTool::Apply, &approved).is_err());
    let mut applied = approved;
    applied["receipt"]["status"] = json!("applied");
    applied["receipt"]["proposalId"] = Value::Null;
    applied["originalProposal"] = Value::Null;
    assert!(GardenResponse::parse(GardenTool::GetProposal, &applied).is_err());
    assert!(GardenResponse::parse(GardenTool::GetReceipt, &applied).is_ok());
}
#[test]
fn outcome_payloads_match_the_immutable_receipt_mapping() {
    use mother::garden::contracts::GardenResponse;
    let original = samples()[4].2.clone();
    let mut response = original.clone();
    response["visibleRecords"][0]["metadata"]["revision"] = json!(2);
    response["visibleRecords"][0]["metadata"]["previousRevision"] = json!(1);
    assert!(GardenResponse::parse(GardenTool::GetProposal, &response).is_err());
    let mut response = original.clone();
    response["visibleRecords"][0]["metadata"]["commitSequence"] = json!(2);
    assert!(GardenResponse::parse(GardenTool::GetProposal, &response).is_err());
    let mut response = original;
    let duplicate = response["visibleRecords"][0].clone();
    response["visibleRecords"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert!(GardenResponse::parse(GardenTool::GetProposal, &response).is_err());
}
