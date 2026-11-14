//! Synthetic temporal behavior through fresh, currently authorized reads.
use chrono::{DateTime, Utc};
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, ExternalReference, GardenErrorCode, GardenKind, GetRequest},
        store::{Clock, GardenStore},
    },
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

const WORKSPACE: &str = "11111111-1111-4111-8111-111111111111";
const GENERATION: &str = "22222222-2222-4222-8222-222222222222";

#[derive(Debug)]
struct TestClock(Mutex<DateTime<Utc>>);
impl TestClock {
    fn set(&self, time: &str) {
        *self.0.lock().unwrap() = time.parse().unwrap();
    }
}
impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}
fn clock() -> Arc<TestClock> {
    Arc::new(TestClock(Mutex::new(
        "2026-11-15T12:00:00.1Z".parse().unwrap(),
    )))
}
fn grant(store: &GardenStore, client: &str, objects: GardenAccess) -> Auth {
    store
        .grant(
            client,
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess {
                mode: AccessMode::Companion,
                subject: Some(format!("synthetic-{client}")),
                records: vec![],
            },
            &objects,
        )
        .unwrap()
}
fn batch(items: Value) -> BatchRequest {
    serde_json::from_value(json!({
        "version":2,"workspaceId":WORKSPACE,"generation":GENERATION,
        "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items
    }))
    .unwrap()
}
fn entity(name: &str) -> Value {
    json!({"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}})
}
fn create(key: &str, draft: Value) -> Value {
    json!({"itemKey":key,"expectedRevision":0,"mutation":"create","draft":draft})
}
fn request(
    reference: &ExternalReference,
    revision: Option<u64>,
    known_at: Option<&str>,
) -> GetRequest {
    serde_json::from_value(json!({
        "version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"requiredExtensions":{},
        "reference":reference,"revision":revision,"knownAt":known_at
    }))
    .unwrap()
}
fn existing(reference: &ExternalReference) -> Value {
    json!({"refType":"existing","kind":reference.kind,"id":reference.id})
}
fn assertion(subject: &ExternalReference, text: &str) -> Value {
    json!({"kind":"assertion","subject":existing(subject),"predicate":"test:lives_in",
        "object":{"type":"value","value":{"type":"string","value":text}},
        "validTime":{"validFrom":"2020-01-01T00:00:00Z","validUntil":"2021-01-01T00:00:00Z","sourceTimezone":null,"precision":"instant"},
        "observedTime":{"at":"2020-06-01T00:00:00Z","sourceTimezone":null,"precision":"instant"},
        "evidence":[],"confidence":null,"extensions":{}})
}

#[test]
fn exact_known_at_preserves_old_belief_after_backdated_correction_and_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("synthetic.redb");
    let time = clock();
    let store =
        GardenStore::initialize_with_clock(&path, WORKSPACE, GENERATION, time.clone()).unwrap();
    let auth = grant(&store, "writer", GardenAccess::All {});
    let created = store
        .apply(&auth, &batch(json!([create("person", entity("Before"))])))
        .unwrap();
    let id = &created.receipt.results[0].reference;
    let original_time = created.receipt.committed_at.clone().unwrap();
    // A backdated host clock must not backdate the store-owned transaction time.
    time.set("2020-01-01T00:00:00Z");
    let corrected = store.apply(&auth, &batch(json!([{
        "itemKey":"person","expectedRevision":1,"mutation":"update","id":id,"draft":entity("After")
    }]))).unwrap();
    let correction_time = corrected.receipt.committed_at.as_deref().unwrap();
    assert!(
        correction_time.parse::<DateTime<Utc>>().unwrap()
            > original_time.parse::<DateTime<Utc>>().unwrap()
    );
    assert_eq!(
        store
            .get(&auth, &request(id, None, Some(&original_time)))
            .unwrap()
            .record
            .metadata
            .revision,
        1
    );
    assert_eq!(
        store
            .get(
                &auth,
                &request(id, None, Some("2026-11-15T12:00:00.100000000Z"))
            )
            .unwrap()
            .record
            .metadata
            .revision,
        1
    );
    assert_eq!(
        store
            .get(&auth, &request(id, Some(2), Some(&original_time)))
            .unwrap_err()
            .code,
        GardenErrorCode::NotFound
    );
    let expected = store
        .get(&auth, &request(id, None, Some(correction_time)))
        .unwrap()
        .record;
    drop(store);
    let reopened = GardenStore::open_with_clock(&path, time).unwrap();
    assert_eq!(
        reopened
            .get(&auth, &request(id, None, Some(correction_time)))
            .unwrap()
            .record,
        expected
    );
    assert_eq!(
        reopened
            .get(
                &auth,
                &request(id, None, Some("2026-11-15T11:59:59.999999999Z"))
            )
            .unwrap_err()
            .code,
        GardenErrorCode::NotFound
    );
}

#[test]
fn conflicting_assertions_and_late_evidence_coexist_with_half_open_validity() {
    let dir = tempfile::tempdir().unwrap();
    let time = clock();
    let store = GardenStore::initialize_with_clock(
        dir.path().join("synthetic.redb"),
        WORKSPACE,
        GENERATION,
        time.clone(),
    )
    .unwrap();
    let auth = grant(&store, "writer", GardenAccess::All {});
    let created = store
        .apply(&auth, &batch(json!([create("person", entity("Person"))])))
        .unwrap();
    let person = &created.receipt.results[0].reference;
    time.set("2026-11-15T12:01:00Z");
    let evidence = json!({"kind":"evidence","source":{"type":"uri","uri":"urn:test:late-evidence"},"excerpt":"Observed years before ingestion",
        "observedTime":{"at":"2020-06-01T00:00:00Z","sourceTimezone":null,"precision":"instant"},"extensions":{}});
    let mut first = assertion(person, "First city");
    first["evidence"] = json!([{"refType":"item","kind":"evidence","itemKey":"source"}]);
    let result = store
        .apply(
            &auth,
            &batch(json!([
                create("first", first),
                create("second", assertion(person, "Second city")),
                create("source", evidence)
            ])),
        )
        .unwrap();
    for item in result
        .receipt
        .results
        .iter()
        .filter(|item| item.reference.kind == GardenKind::Assertion)
    {
        let get = request(&item.reference, None, None);
        assert!(
            store
                .get_valid_at(&auth, &get, "2020-01-01T00:00:00Z")
                .unwrap()
                .is_some()
        );
        assert!(
            store
                .get_valid_at(&auth, &get, "2021-01-01T00:00:00Z")
                .unwrap()
                .is_none()
        );
        assert_eq!(
            store
                .get(
                    &auth,
                    &request(&item.reference, None, Some("2020-06-01T00:00:00Z"))
                )
                .unwrap_err()
                .code,
            GardenErrorCode::NotFound
        );
    }
    let late = result
        .visible_records
        .iter()
        .find(|r| r.draft.kind() == GardenKind::Evidence)
        .unwrap();
    assert!(late.metadata.transaction_time.starts_with("2026-11-15"));
    let claim = &result.receipt.results[0].reference;
    let old_time = result.receipt.committed_at.as_deref().unwrap();
    let mut corrected = assertion(person, "Corrected city");
    corrected["validTime"]["validFrom"] = json!("2019-01-01T00:00:00Z");
    time.set("2026-11-15T12:02:00Z");
    store.apply(&auth, &batch(json!([{
        "itemKey":"correction","expectedRevision":1,"mutation":"update","id":claim,"draft":corrected
    }]))).unwrap();
    assert!(
        store
            .get_valid_at(&auth, &request(claim, None, None), "2019-06-01T00:00:00Z")
            .unwrap()
            .is_some()
    );
    assert!(
        store
            .get_valid_at(
                &auth,
                &request(claim, None, Some(old_time)),
                "2019-06-01T00:00:00Z"
            )
            .unwrap()
            .is_none()
    );
    let old = store
        .get(&auth, &request(claim, None, Some(old_time)))
        .unwrap();
    assert_eq!(
        serde_json::to_value(old.record.draft).unwrap()["evidence"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let stats = store.audit(&auth).unwrap();
    assert_eq!(
        (
            stats.current_records,
            stats.history_records,
            stats.receipts,
            stats.sequence
        ),
        (4, 5, 3, 3)
    );
}

#[test]
fn updates_preserve_opaque_extensions_and_original_contributor() {
    let dir = tempfile::tempdir().unwrap();
    let store =
        GardenStore::initialize(dir.path().join("synthetic.redb"), WORKSPACE, GENERATION).unwrap();
    let first_auth = grant(&store, "first", GardenAccess::All {});
    let second_auth = grant(&store, "second", GardenAccess::All {});
    let mut original = entity("Original");
    original["extensions"] =
        json!({"test:future":{"schemaVersion":7,"payload":{"opaque":[1,{"x":true}]}}});
    let created = store
        .apply(&first_auth, &batch(json!([create("person", original)])))
        .unwrap();
    let id = &created.receipt.results[0].reference;
    store.apply(&second_auth, &batch(json!([{
        "itemKey":"person","expectedRevision":1,"mutation":"update","id":id,"draft":entity("Updated")
    }]))).unwrap();
    let first = store
        .get(&second_auth, &request(id, Some(1), None))
        .unwrap()
        .record;
    let current = store
        .get(&second_auth, &request(id, None, None))
        .unwrap()
        .record;
    assert_eq!(current.metadata.contributor, first.metadata.contributor);
    assert_ne!(current.metadata.actor, first.metadata.actor);
    assert_eq!(
        serde_json::to_value(current.draft).unwrap()["extensions"],
        serde_json::to_value(first.draft).unwrap()["extensions"]
    );
}

#[test]
fn historical_hidden_endpoint_uses_current_policy_and_revocation() {
    let dir = tempfile::tempdir().unwrap();
    let store =
        GardenStore::initialize(dir.path().join("synthetic.redb"), WORKSPACE, GENERATION).unwrap();
    let owner = grant(&store, "owner", GardenAccess::All {});
    let people = store
        .apply(
            &owner,
            &batch(json!([
                create("visible", entity("Visible")),
                create("hidden", entity("Hidden"))
            ])),
        )
        .unwrap();
    let visible = &people.receipt.results[0].reference;
    let hidden = &people.receipt.results[1].reference;
    let created = store
        .apply(
            &owner,
            &batch(json!([create("claim", assertion(hidden, "Old"))])),
        )
        .unwrap();
    let claim = &created.receipt.results[0].reference;
    store.apply(&owner, &batch(json!([{
        "itemKey":"claim","expectedRevision":1,"mutation":"update","id":claim,"draft":assertion(visible,"New")
    }]))).unwrap();
    let reader = grant(
        &store,
        "reader",
        GardenAccess::Allowlist {
            objects: vec![visible.clone(), claim.clone()],
        },
    );
    assert!(store.get(&reader, &request(claim, None, None)).is_ok());
    assert_eq!(
        store
            .get(&reader, &request(claim, Some(1), None))
            .unwrap_err()
            .code,
        GardenErrorCode::NotFound
    );
    store.revoke("reader").unwrap();
    assert_eq!(
        store
            .get(&reader, &request(claim, None, None))
            .unwrap_err()
            .code,
        GardenErrorCode::Unauthorized
    );
}

#[test]
fn valid_time_lookup_omits_retired_current_assertions_but_preserves_old_belief() {
    let dir = tempfile::tempdir().unwrap();
    let store =
        GardenStore::initialize(dir.path().join("synthetic.redb"), WORKSPACE, GENERATION).unwrap();
    let auth = grant(&store, "writer", GardenAccess::All {});
    let created = store
        .apply(&auth, &batch(json!([create("person", entity("Person"))])))
        .unwrap();
    let person = &created.receipt.results[0].reference;
    let asserted = store
        .apply(
            &auth,
            &batch(json!([create("claim", assertion(person, "City"))])),
        )
        .unwrap();
    let claim = &asserted.receipt.results[0].reference;
    let before = asserted.receipt.committed_at.as_deref().unwrap();
    assert!(
        store
            .get_valid_at(&auth, &request(claim, None, None), "2020-06-01T00:00:00Z")
            .unwrap()
            .is_some()
    );
    store
        .apply(
            &auth,
            &batch(json!([{
                "itemKey":"claim","expectedRevision":1,"mutation":"retract","id":claim
            }])),
        )
        .unwrap();
    assert!(
        store
            .get_valid_at(&auth, &request(claim, None, None), "2020-06-01T00:00:00Z")
            .unwrap()
            .is_none()
    );
    assert!(
        store
            .get_valid_at(
                &auth,
                &request(claim, None, Some(before)),
                "2020-06-01T00:00:00Z"
            )
            .unwrap()
            .is_some()
    );
}

#[test]
fn supersession_indices_and_immutable_history_survive_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("synthetic.redb");
    let store = GardenStore::initialize(&path, WORKSPACE, GENERATION).unwrap();
    let auth = grant(&store, "writer", GardenAccess::All {});
    let created = store
        .apply(&auth, &batch(json!([create("person", entity("Person"))])))
        .unwrap();
    let person = &created.receipt.results[0].reference;
    let asserted = store
        .apply(
            &auth,
            &batch(json!([create("old", assertion(person, "Old city"))])),
        )
        .unwrap();
    let old = &asserted.receipt.results[0].reference;
    let operation = batch(json!([
        {"itemKey":"old","expectedRevision":1,"mutation":"supersede","id":old,
            "replacement":{"refType":"item","kind":"assertion","itemKey":"new"}},
        create("new",assertion(person,"New city"))
    ]));
    store.apply(&auth, &operation).unwrap();
    let expected = store.audit(&auth).unwrap();
    assert_eq!(
        (
            expected.current_records,
            expected.history_records,
            expected.receipts,
            expected.sequence
        ),
        (3, 4, 3, 3)
    );
    drop(store);
    let reopened = GardenStore::open(&path).unwrap();
    assert_eq!(reopened.audit(&auth).unwrap(), expected);
    assert_eq!(
        reopened
            .get(&auth, &request(old, Some(1), None))
            .unwrap()
            .record
            .metadata
            .lifecycle,
        mother::garden::contracts::Lifecycle::Active
    );
    assert_eq!(
        reopened
            .get(&auth, &request(old, None, None))
            .unwrap()
            .record
            .metadata
            .lifecycle,
        mother::garden::contracts::Lifecycle::Superseded
    );
}
