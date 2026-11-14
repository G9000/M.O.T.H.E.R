//! Synthetic indexed change feed acceptance: no live store or host configuration.
use chrono::{DateTime, Utc};
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, ChangesRequest, ExternalReference, GardenErrorCode, Lifecycle},
        store::{Clock, GardenStore},
    },
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
const W: &str = "11111111-1111-4111-8111-111111111111";
const G: &str = "22222222-2222-4222-8222-222222222222";
#[derive(Debug)]
struct TestClock(Mutex<DateTime<Utc>>);
impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}
fn grant(s: &GardenStore, name: &str, objects: GardenAccess) -> Auth {
    s.grant(
        name,
        &["garden:read".into(), "garden:write".into()],
        &ClientAccess {
            mode: AccessMode::Companion,
            subject: Some(name.into()),
            records: vec![],
        },
        &objects,
    )
    .unwrap()
}
fn batch(items: Value) -> BatchRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":W,"generation":G,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items})).unwrap()
}
fn create(name: &str) -> Value {
    json!({"itemKey":name,"expectedRevision":0,"mutation":"create","draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}})
}
fn request(cursor: Option<String>, limit: u32) -> ChangesRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":W,"generation":G,"requiredExtensions":{},"limit":limit,"cursor":cursor})).unwrap()
}
#[test]
fn immutable_pages_survive_reopen_and_checkpoint_polls_future_changes() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("synthetic.redb");
    let s = GardenStore::initialize(&path, W, G).unwrap();
    let a = grant(&s, "writer", GardenAccess::All {});
    s.apply(
        &a,
        &batch(json!([create("one"), create("two"), create("three")])),
    )
    .unwrap();
    let first = s.changes(&a, &request(None, 1)).unwrap();
    assert_eq!(first.changes.len(), 1);
    let cursor = first.page.next_cursor.clone().unwrap();
    assert_eq!(
        s.changes(&a, &request(Some(cursor.clone()), 1)).unwrap(),
        s.changes(&a, &request(Some(cursor.clone()), 1)).unwrap()
    );
    drop(s);
    let s = GardenStore::open(&path).unwrap();
    let second = s.changes(&a, &request(Some(cursor), 1)).unwrap();
    assert_eq!(second.changes.len(), 1);
    assert!(first.changes[0].reference.id < second.changes[0].reference.id);
    s.apply(&a, &batch(json!([create("future")]))).unwrap();
    let third = s
        .changes(&a, &request(second.page.next_cursor.clone(), 1))
        .unwrap();
    assert_eq!(third.changes.len(), 1);
    assert_eq!(third.changes[0].sequence, 1);
    let checkpoint = s.changes(&a, &request(third.page.next_cursor, 1)).unwrap();
    assert_eq!(checkpoint.changes.len(), 1);
    assert_eq!(checkpoint.changes[0].sequence, 2);
    assert!(checkpoint.coverage.complete);
    assert!(checkpoint.page.next_cursor.is_some());
    let empty = s
        .changes(&a, &request(checkpoint.page.next_cursor.clone(), 1))
        .unwrap();
    assert!(empty.changes.is_empty());
    assert!(empty.coverage.complete);
}
#[test]
fn retraction_is_a_tombstone_and_old_active_events_cannot_resurrect() {
    let d = tempfile::tempdir().unwrap();
    let s = GardenStore::initialize(d.path().join("synthetic.redb"), W, G).unwrap();
    let a = grant(&s, "writer", GardenAccess::All {});
    let made = s.apply(&a, &batch(json!([create("one")]))).unwrap();
    let id = &made.receipt.results[0].reference;
    let initial = s.changes(&a, &request(None, 10)).unwrap();
    let checkpoint = initial.page.next_cursor;
    s.apply(
        &a,
        &batch(json!([{"itemKey":"one","expectedRevision":1,"mutation":"retract","id":id}])),
    )
    .unwrap();
    let tombstone = s.changes(&a, &request(checkpoint, 10)).unwrap();
    assert_eq!(tombstone.changes.len(), 1);
    assert_eq!(tombstone.changes[0].lifecycle, Lifecycle::Retracted);
    assert!(tombstone.changes[0].record.is_none());
    let bootstrap = s.changes(&a, &request(None, 10)).unwrap();
    assert!(bootstrap.changes.iter().all(|c| c.record.is_none()));
}
#[test]
fn revoked_reissued_narrowed_expired_forged_cursors_fail_closed() {
    let d = tempfile::tempdir().unwrap();
    let clock = Arc::new(TestClock(Mutex::new(
        "2026-11-15T00:00:00Z".parse().unwrap(),
    )));
    let s =
        GardenStore::initialize_with_clock(d.path().join("synthetic.redb"), W, G, clock.clone())
            .unwrap();
    let a = grant(&s, "writer", GardenAccess::All {});
    let made = s
        .apply(&a, &batch(json!([create("one"), create("two")])))
        .unwrap();
    let cursor = s.changes(&a, &request(None, 1)).unwrap().page.next_cursor;
    assert_eq!(
        s.changes(&a, &request(Some("forged".into()), 1))
            .unwrap_err()
            .code,
        GardenErrorCode::RestartRequired
    );
    *clock.0.lock().unwrap() = "2027-11-15T00:00:00Z".parse().unwrap();
    assert_eq!(
        s.changes(&a, &request(cursor.clone(), 1)).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    *clock.0.lock().unwrap() = "2026-11-15T00:00:00Z".parse().unwrap();
    s.revoke("writer").unwrap();
    assert_eq!(
        s.changes(&a, &request(cursor.clone(), 1)).unwrap_err().code,
        GardenErrorCode::Unauthorized
    );
    let narrow = grant(
        &s,
        "writer",
        GardenAccess::Allowlist {
            objects: vec![made.receipt.results[0].reference.clone()],
        },
    );
    assert_eq!(
        s.changes(&narrow, &request(cursor, 1)).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
}
#[test]
fn historical_hidden_endpoint_and_evidence_never_enter_feed() {
    let d = tempfile::tempdir().unwrap();
    let s = GardenStore::initialize(d.path().join("synthetic.redb"), W, G).unwrap();
    let a = grant(&s, "owner", GardenAccess::All {});
    let made = s
        .apply(&a, &batch(json!([create("visible"), create("hidden")])))
        .unwrap();
    let visible = &made.receipt.results[0].reference;
    let hidden = &made.receipt.results[1].reference;
    let assertion = |subject: &ExternalReference| json!({"kind":"assertion","subject":{"refType":"existing","kind":"entity","id":subject.id},"predicate":"test:property","object":{"type":"value","value":{"type":"string","value":"secret"}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[],"confidence":null,"extensions":{}});
    let claim=s.apply(&a,&batch(json!([{"itemKey":"claim","expectedRevision":0,"mutation":"create","draft":assertion(hidden)}]))).unwrap();
    let claim = &claim.receipt.results[0].reference;
    s.apply(&a,&batch(json!([{"itemKey":"claim","expectedRevision":1,"mutation":"update","id":claim,"draft":assertion(visible)}]))).unwrap();
    let reader = grant(
        &s,
        "reader",
        GardenAccess::Allowlist {
            objects: vec![visible.clone(), claim.clone()],
        },
    );
    let response = s.changes(&reader, &request(None, 100)).unwrap();
    assert!(
        !serde_json::to_string(&response)
            .unwrap()
            .contains(&hidden.id)
    );
    assert!(
        response
            .changes
            .iter()
            .all(|c| c.reference != *claim || c.revision == 2)
    );
    assert!(response.coverage.complete);
}
#[test]
fn duplicate_allowlist_entries_do_not_duplicate_events() {
    let d = tempfile::tempdir().unwrap();
    let s = GardenStore::initialize(d.path().join("synthetic.redb"), W, G).unwrap();
    let a = grant(&s, "owner", GardenAccess::All {});
    let made = s.apply(&a, &batch(json!([create("one")]))).unwrap();
    let reference = made.receipt.results[0].reference.clone();
    let reader = grant(
        &s,
        "reader",
        GardenAccess::Allowlist {
            objects: vec![reference.clone(), reference],
        },
    );
    let response = s.changes(&reader, &request(None, 10)).unwrap();
    assert_eq!(response.changes.len(), 1);
    assert!(response.coverage.complete);
}
#[test]
fn hidden_evidence_and_source_payload_are_absent_from_changes() {
    let d = tempfile::tempdir().unwrap();
    let s = GardenStore::initialize(d.path().join("synthetic.redb"), W, G).unwrap();
    let a = grant(&s, "owner", GardenAccess::All {});
    let made=s.apply(&a,&batch(json!([create("person"),{"itemKey":"source","expectedRevision":0,"mutation":"create","draft":{"kind":"evidence","source":{"type":"uri","uri":"urn:test:hidden-source"},"excerpt":"private source payload","observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}},{"itemKey":"claim","expectedRevision":0,"mutation":"create","draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":"person"},"predicate":"test:property","object":{"type":"value","value":{"type":"string","value":"claim"}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[{"refType":"item","kind":"evidence","itemKey":"source"}],"confidence":null,"extensions":{}}}]))).unwrap();
    let reader = grant(
        &s,
        "reader",
        GardenAccess::Allowlist {
            objects: vec![
                made.receipt.results[0].reference.clone(),
                made.receipt.results[2].reference.clone(),
            ],
        },
    );
    let result = s.changes(&reader, &request(None, 100)).unwrap();
    assert_eq!(result.changes.len(), 1);
    assert_eq!(
        result.changes[0].reference,
        made.receipt.results[0].reference
    );
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(!serialized.contains("private source payload"));
    assert!(!serialized.contains(&made.receipt.results[1].reference.id));
    assert!(result.coverage.complete);
}

#[test]
fn large_events_are_paged_within_response_byte_budget() {
    let d = tempfile::tempdir().unwrap();
    let s = GardenStore::initialize(d.path().join("synthetic.redb"), W, G).unwrap();
    let a = grant(&s, "owner", GardenAccess::All {});
    for name in ["one", "two"] {
        let mut item = create(name);
        item["draft"]["extensions"] =
            json!({"test:large":{"schemaVersion":1,"payload":{"text":"x".repeat(150_000)}}});
        s.apply(&a, &batch(json!([item]))).unwrap();
    }
    let first = s.changes(&a, &request(None, 100)).unwrap();
    assert_eq!(first.changes.len(), 1);
    assert!(first.coverage.truncated);
    let second = s
        .changes(&a, &request(first.page.next_cursor, 100))
        .unwrap();
    assert_eq!(second.changes.len(), 1);
    assert!(second.coverage.complete);
}
#[test]
fn unexpired_cursors_reject_changed_grants_and_same_policy_reissue() {
    let dir = tempfile::tempdir().unwrap();
    let clock = Arc::new(TestClock(Mutex::new(
        "2026-11-15T00:00:00Z".parse().unwrap(),
    )));
    let store =
        GardenStore::initialize_with_clock(dir.path().join("synthetic.redb"), W, G, clock).unwrap();
    let owner = grant(&store, "owner", GardenAccess::All {});
    let made = store
        .apply(&owner, &batch(json!([create("one"), create("two")])))
        .unwrap();
    let reader = grant(&store, "reader", GardenAccess::All {});
    let cursor = store
        .changes(&reader, &request(None, 1))
        .unwrap()
        .page
        .next_cursor;
    let reissued = grant(&store, "reader", GardenAccess::All {});
    assert_eq!(
        store
            .changes(&reader, &request(cursor.clone(), 1))
            .unwrap_err()
            .code,
        GardenErrorCode::Unauthorized
    );
    assert_eq!(
        store
            .changes(&reissued, &request(cursor, 1))
            .unwrap_err()
            .code,
        GardenErrorCode::RestartRequired
    );
    let cursor = store
        .changes(&reissued, &request(None, 1))
        .unwrap()
        .page
        .next_cursor;
    let narrowed = grant(
        &store,
        "reader",
        GardenAccess::Allowlist {
            objects: vec![made.receipt.results[0].reference.clone()],
        },
    );
    assert_eq!(
        store
            .changes(&narrowed, &request(cursor, 1))
            .unwrap_err()
            .code,
        GardenErrorCode::RestartRequired
    );
}
#[test]
fn oversized_accepted_record_emits_metadata_and_feed_continues() {
    use mother::garden::contracts::{GetRequest, MAX_JSON_BYTES};
    let dir = tempfile::tempdir().unwrap();
    let store = GardenStore::initialize(dir.path().join("synthetic.redb"), W, G).unwrap();
    let owner = grant(&store, "owner", GardenAccess::All {});
    let mut large = create("large");
    large["draft"]["extensions"] = json!({"test:large":{"schemaVersion":1,"payload":{"text":"x".repeat(MAX_JSON_BYTES-1400)}}});
    let large = store
        .apply(&owner, &batch(json!([large])))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let normal = store
        .apply(&owner, &batch(json!([create("normal")])))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let get:GetRequest=serde_json::from_value(json!({"version":2,"workspaceId":W,"generation":G,"requiredExtensions":{},"reference":large,"revision":null,"knownAt":null})).unwrap();
    assert_eq!(
        store.get(&owner, &get).unwrap().record.metadata.id,
        large.id
    );
    let first = store.changes(&owner, &request(None, 1)).unwrap();
    assert_eq!(first.changes.len(), 1);
    assert_eq!(first.changes[0].reference, large);
    assert_eq!(first.changes[0].lifecycle, Lifecycle::Active);
    assert!(first.changes[0].record.is_none());
    assert!(!first.coverage.complete);
    assert!(first.coverage.truncated);
    assert_eq!(
        first.coverage.reason.as_deref(),
        Some("event_payload_omitted")
    );
    let second = store
        .changes(&owner, &request(first.page.next_cursor, 1))
        .unwrap();
    assert_eq!(second.changes.len(), 1);
    assert_eq!(second.changes[0].reference, normal);
    assert!(second.changes[0].record.is_some());
    assert!(second.coverage.complete);
    let empty = store
        .changes(&owner, &request(second.page.next_cursor, 1))
        .unwrap();
    assert!(empty.changes.is_empty());
}
#[test]
fn accepted_deep_record_emits_metadata_without_stalling_checkpoint() {
    use mother::garden::contracts::GetRequest;
    let dir = tempfile::tempdir().unwrap();
    let store = GardenStore::initialize(dir.path().join("synthetic.redb"), W, G).unwrap();
    let owner = grant(&store, "owner", GardenAccess::All {});
    let mut nested = Value::Null;
    for _ in 0..18 {
        nested = json!([nested]);
    }
    let mut item = create("deep");
    item["draft"]["extensions"] = json!({"test:deep":{"schemaVersion":1,"payload":nested}});
    let reference = store
        .apply(&owner, &batch(json!([item])))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let get:GetRequest=serde_json::from_value(json!({"version":2,"workspaceId":W,"generation":G,"requiredExtensions":{},"reference":reference,"revision":null,"knownAt":null})).unwrap();
    assert_eq!(
        store.get(&owner, &get).unwrap().record.metadata.id,
        reference.id
    );
    let page = store.changes(&owner, &request(None, 100)).unwrap();
    assert_eq!(page.changes.len(), 1);
    assert_eq!(page.changes[0].reference, reference);
    assert!(page.changes[0].record.is_none());
    assert_eq!(
        page.coverage.reason.as_deref(),
        Some("event_payload_omitted")
    );
    assert!(!page.coverage.complete);
    let poll = store
        .changes(&owner, &request(page.page.next_cursor, 100))
        .unwrap();
    assert!(poll.changes.is_empty());
    assert!(poll.coverage.complete);
}
