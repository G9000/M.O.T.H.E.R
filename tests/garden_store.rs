use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, contracts::*, store::GardenStore},
};
use serde_json::json;
use uuid::Uuid;

fn batch(workspace: &str, generation: &str, items: serde_json::Value) -> BatchRequest {
    serde_json::from_value(
        json!({"version":2,"workspaceId":workspace,"generation":generation,
        "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items}),
    )
    .unwrap()
}
fn create(name: &str) -> serde_json::Value {
    json!({"itemKey":"entity","mutation":"create","expectedRevision":0,
        "draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}})
}
fn scopes() -> Vec<String> {
    vec!["garden:read".into(), "garden:write".into()]
}

#[test]
fn durable_exact_retry_and_changed_payload_reuse() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("garden.redb");
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store = GardenStore::initialize(&path, &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let request = batch(&workspace, &generation, json!([create("before")]));
    let original = store.apply(&auth, &request).unwrap();
    let id = original.receipt.results[0].reference.clone();
    let update = batch(
        &workspace,
        &generation,
        json!([{"itemKey":"updated","mutation":"update","id":id,"expectedRevision":1,
        "draft":{"kind":"entity","entityType":"test:person","displayName":"after","aliases":[],"extensions":{}}}]),
    );
    store.apply(&auth, &update).unwrap();
    drop(store);
    let store = GardenStore::open(&path).unwrap();
    let retry = store.apply(&auth, &request).unwrap();
    assert_eq!(retry.receipt, original.receipt);
    assert_eq!(retry.visible_records, original.visible_records);
    let mut changed = request.clone();
    changed.items[0].draft =
        Some(serde_json::from_value(create("different")["draft"].clone()).unwrap());
    assert_eq!(
        store.apply(&auth, &changed).unwrap_err().code,
        GardenErrorCode::OperationReused
    );
    assert_eq!(
        store.apply(&auth, &update).unwrap().receipt.results[0].revision,
        2
    );
}

#[test]
fn stale_batch_has_no_partial_create_or_receipt_and_revocation_is_current() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let made = store
        .apply(
            &auth,
            &batch(&workspace, &generation, json!([create("first")])),
        )
        .unwrap();
    let id = made.receipt.results[0].reference.clone();
    let request = batch(
        &workspace,
        &generation,
        json!([create("rolled back"),{"itemKey":"stale","mutation":"retract","id":id,"expectedRevision":7}]),
    );
    assert_eq!(
        store.apply(&auth, &request).unwrap_err().code,
        GardenErrorCode::Conflict
    );
    let lookup: OutcomeRequest = serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,
        "operationId":request.operation_id,"operationGeneration":generation,"requiredExtensions":{}})).unwrap();
    assert_eq!(
        store.receipt(&auth, &lookup).unwrap_err().code,
        GardenErrorCode::NotFound
    );
    store.revoke("writer").unwrap();
    assert_eq!(
        store.apply(&auth, &request).unwrap_err().code,
        GardenErrorCode::Unauthorized
    );
}

#[test]
fn initialize_never_replaces_and_open_never_creates() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("garden.redb");
    assert!(GardenStore::open(&path).is_err());
    assert!(!path.exists());
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    drop(GardenStore::initialize(&path, &workspace, &generation).unwrap());
    assert!(GardenStore::initialize(&path, &workspace, &generation).is_err());
    assert!(GardenStore::open(&path).is_ok());
}

fn graph_items() -> serde_json::Value {
    json!([
        create("subject"),
        {"itemKey":"evidence","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence","source":{"type":"uri","uri":"https://example.test/source"},"excerpt":"synthetic evidence","observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}},
        {"itemKey":"assertion","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":"entity"},"predicate":"test:knows","object":{"type":"entityRef","entityRef":{"refType":"item","kind":"entity","itemKey":"entity"}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[{"refType":"item","kind":"evidence","itemKey":"evidence"}],"confidence":null,"extensions":{}}}
    ])
}

#[test]
fn atomic_graph_and_final_state_retirement_and_contributor() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let first = store
        .grant(
            "first",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let second = store
        .grant(
            "second",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let outcome = store
        .apply(&first, &batch(&workspace, &generation, graph_items()))
        .unwrap();
    assert_eq!(outcome.visible_records.len(), 3);
    assert!(
        outcome
            .visible_records
            .iter()
            .all(|r| r.metadata.commit_sequence == 1)
    );
    let entity = outcome.receipt.results[0].reference.clone();
    let evidence = outcome.receipt.results[1].reference.clone();
    let assertion = outcome.receipt.results[2].reference.clone();
    let retire_evidence = batch(
        &workspace,
        &generation,
        json!([{"itemKey":"evidence","mutation":"retract","expectedRevision":1,"id":evidence}]),
    );
    assert_eq!(
        store.apply(&second, &retire_evidence).unwrap_err().code,
        GardenErrorCode::InvalidEndpoint
    );
    let retired=store.apply(&second,&batch(&workspace,&generation,json!([
        {"itemKey":"evidence","mutation":"retract","expectedRevision":1,"id":evidence},
        {"itemKey":"assertion","mutation":"retract","expectedRevision":1,"id":assertion},
        {"itemKey":"entity","mutation":"retract","expectedRevision":1,"id":entity}
    ]))).unwrap();
    assert!(
        retired
            .visible_records
            .iter()
            .all(|r| r.metadata.commit_sequence == 2
                && r.metadata.contributor == "first"
                && r.metadata.actor == "second")
    );
    assert_eq!(store.audit(&first).unwrap().history_records, 6);
}

#[test]
fn extension_namespaces_merge_and_lifecycle_cycles_roll_back() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let mut item = create("one");
    item["draft"]["extensions"] =
        json!({"other:owned":{"schemaVersion":4,"payload":{"keep":true}}});
    let mut other = create("two");
    other["itemKey"] = json!("other");
    let original = store
        .apply(&auth, &batch(&workspace, &generation, json!([item, other])))
        .unwrap();
    let one = original.receipt.results[0].reference.clone();
    let two = original.receipt.results[1].reference.clone();
    let mut update = create("updated");
    update["mutation"] = json!("update");
    update["expectedRevision"] = json!(1);
    update["id"] = json!(one);
    update["draft"]["extensions"] = json!({"new:owned":{"schemaVersion":1,"payload":false}});
    let updated = store
        .apply(&auth, &batch(&workspace, &generation, json!([update])))
        .unwrap();
    let GardenDraft::Entity { extensions, .. } = &updated.visible_records[0].draft else {
        panic!("expected entity")
    };
    assert_eq!(extensions.len(), 2);
    assert_eq!(extensions["other:owned"].payload, json!({"keep":true}));
    let cycle = batch(
        &workspace,
        &generation,
        json!([
            {"itemKey":"one","mutation":"supersede","expectedRevision":2,"id":one,"replacement":{"refType":"existing","kind":"entity","id":two.id}},
            {"itemKey":"two","mutation":"supersede","expectedRevision":1,"id":two,"replacement":{"refType":"existing","kind":"entity","id":one.id}}
        ]),
    );
    assert_eq!(
        store.apply(&auth, &cycle).unwrap_err().code,
        GardenErrorCode::InvalidEndpoint
    );
    assert_eq!(store.audit(&auth).unwrap().sequence, 2);
}

#[test]
fn grants_are_graph_native_and_current_footprints_bound_history() {
    use mother::access::AccessMode;
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let owner = store
        .grant(
            "owner",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let made = store
        .apply(&owner, &batch(&workspace, &generation, graph_items()))
        .unwrap();
    let objects: Vec<_> = made
        .receipt
        .results
        .iter()
        .map(|r| r.reference.clone())
        .collect();
    let access = ClientAccess {
        mode: AccessMode::Shared,
        subject: Some("synthetic-subject".into()),
        records: vec![],
    };
    assert_eq!(
        store
            .grant(
                "shared",
                &scopes(),
                &access,
                &GardenAccess::Allowlist {
                    objects: objects.clone()
                }
            )
            .unwrap_err()
            .code,
        GardenErrorCode::Forbidden
    );
    let shared = store
        .grant(
            "shared",
            &["garden:read".into()],
            &access,
            &GardenAccess::Allowlist {
                objects: objects.clone(),
            },
        )
        .unwrap();
    let request:GetRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"requiredExtensions":{},"reference":objects[2],"revision":1,"knownAt":null})).unwrap();
    assert!(store.get(&shared, &request).is_ok());
    // Hiding evidence also hides the assertion, including an older revision.
    let reduced = store
        .grant(
            "shared",
            &["garden:read".into()],
            &access,
            &GardenAccess::Allowlist {
                objects: vec![objects[0].clone(), objects[2].clone()],
            },
        )
        .unwrap();
    assert_eq!(
        store.get(&reduced, &request).unwrap_err().code,
        GardenErrorCode::NotFound
    );
    assert_eq!(
        store.get(&shared, &request).unwrap_err().code,
        GardenErrorCode::Unauthorized
    );
}

#[test]
fn oversized_optional_response_payload_is_omitted_before_atomic_commit() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let items: Vec<_> = (0..64).map(|n| json!({"itemKey":format!("e{n}"),"mutation":"create","expectedRevision":0,
        "draft":{"kind":"evidence","source":{"type":"uri","uri":"https://example.test/source"},"excerpt":"x".repeat(3650),
        "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}})).collect();
    let request = batch(&workspace, &generation, json!(items));
    let response = store.apply(&auth, &request).unwrap();
    assert!(response.payload_omitted);
    assert!(response.visible_records.is_empty());
    assert_eq!(response.receipt.results.len(), 64);
    assert_eq!(store.apply(&auth, &request).unwrap(), response);
    assert_eq!(store.audit(&auth).unwrap().history_records, 64);
}

#[test]
fn concurrent_expected_revision_writers_cannot_both_commit() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let made = store
        .apply(
            &auth,
            &batch(&workspace, &generation, json!([create("one")])),
        )
        .unwrap();
    let reference = &made.receipt.results[0].reference;
    let first = batch(
        &workspace,
        &generation,
        json!([{"itemKey":"retire","mutation":"retract","id":reference,"expectedRevision":1}]),
    );
    let second = batch(
        &workspace,
        &generation,
        json!([{"itemKey":"retire","mutation":"retract","id":reference,"expectedRevision":1}]),
    );
    let results = std::thread::scope(|scope| {
        let first = scope.spawn(|| store.apply(&auth, &first));
        let second = scope.spawn(|| store.apply(&auth, &second));
        [first.join().unwrap(), second.join().unwrap()]
    });
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| r
                .as_ref()
                .is_err_and(|e| e.code == GardenErrorCode::Conflict))
            .count(),
        1
    );
    assert_eq!(store.audit(&auth).unwrap().sequence, 2);
}

#[test]
fn guessed_missing_and_hidden_endpoints_have_the_same_failure() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let owner = store
        .grant(
            "owner",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let made = store
        .apply(&owner, &batch(&workspace, &generation, graph_items()))
        .unwrap();
    let hidden = store
        .apply(
            &owner,
            &batch(&workspace, &generation, json!([create("hidden")])),
        )
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let objects = made
        .receipt
        .results
        .iter()
        .map(|r| r.reference.clone())
        .collect();
    let writer = store
        .grant(
            "limited",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::Allowlist { objects },
        )
        .unwrap();
    let target = &made.receipt.results[2].reference;
    let mut errors = Vec::new();
    for endpoint in [hidden.id, Uuid::new_v4().to_string()] {
        let mut draft = serde_json::to_value(&made.visible_records[2].draft).unwrap();
        draft["object"] = json!({"type":"entityRef","entityRef":{"refType":"existing","kind":"entity","id":endpoint}});
        let request = batch(
            &workspace,
            &generation,
            json!([{"itemKey":"update","mutation":"update","expectedRevision":1,"id":target,"draft":draft}]),
        );
        errors.push(store.apply(&writer, &request).unwrap_err().code);
    }
    assert_eq!(
        errors,
        vec![GardenErrorCode::NotFound, GardenErrorCode::NotFound]
    );
    assert_eq!(store.audit(&owner).unwrap().sequence, 2);
}

#[test]
fn hidden_retirement_dependents_are_not_disclosed_by_validation_errors() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let owner = store
        .grant(
            "owner",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let made = store
        .apply(&owner, &batch(&workspace, &generation, graph_items()))
        .unwrap();
    let entity = made.receipt.results[0].reference.clone();
    let writer = store
        .grant(
            "limited",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::Allowlist {
                objects: vec![entity.clone()],
            },
        )
        .unwrap();
    let request = batch(
        &workspace,
        &generation,
        json!([{"itemKey":"retire","mutation":"retract","expectedRevision":1,"id":entity}]),
    );
    assert_eq!(
        store.apply(&writer, &request).unwrap_err().code,
        GardenErrorCode::NotFound
    );
    let lookup:OutcomeRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"requiredExtensions":{},"operationId":request.operation_id,"operationGeneration":generation})).unwrap();
    assert_eq!(
        store.receipt(&writer, &lookup).unwrap_err().code,
        GardenErrorCode::NotFound
    );
    assert_eq!(store.audit(&owner).unwrap().sequence, 1);
}

#[test]
fn reissued_client_subject_cannot_probe_prior_operation_payload() {
    use mother::access::AccessMode;
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("garden.redb"), &workspace, &generation).unwrap();
    let policy = |subject: &str| ClientAccess {
        mode: AccessMode::Companion,
        subject: Some(subject.into()),
        records: vec![],
    };
    let first = store
        .grant("client", &scopes(), &policy("first"), &GardenAccess::All {})
        .unwrap();
    let mut request = batch(
        &workspace,
        &generation,
        json!([create("private first subject")]),
    );
    store.apply(&first, &request).unwrap();
    let second = store
        .grant(
            "client",
            &scopes(),
            &policy("second"),
            &GardenAccess::All {},
        )
        .unwrap();
    request.items[0].draft =
        Some(serde_json::from_value(create("changed payload")["draft"].clone()).unwrap());
    assert_eq!(
        store.apply(&second, &request).unwrap_err().code,
        GardenErrorCode::NotFound
    );
    let lookup:OutcomeRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"requiredExtensions":{},"operationId":request.operation_id,"operationGeneration":generation})).unwrap();
    assert_eq!(
        store.receipt(&second, &lookup).unwrap_err().code,
        GardenErrorCode::NotFound
    );
    assert_eq!(store.audit(&second).unwrap().sequence, 1);
}
