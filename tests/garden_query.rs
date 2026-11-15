use chrono::{DateTime, Utc};
use mother::{
    access::ClientAccess,
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::*,
        store::{Clock, GardenStore},
    },
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[test]
fn concurrent_query_admission_does_not_treat_an_earlier_clock_sample_as_rollback() {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    };
    use std::time::Duration;
    #[derive(Debug)]
    struct DelayedClock {
        armed: AtomicBool,
        sampled: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }
    impl Clock for DelayedClock {
        fn now(&self) -> DateTime<Utc> {
            if self.armed.swap(false, Ordering::SeqCst) {
                let sampled = "2026-01-01T00:00:00Z".parse().unwrap();
                self.sampled.send(()).unwrap();
                self.release
                    .lock()
                    .unwrap()
                    .recv_timeout(Duration::from_secs(5))
                    .unwrap();
                sampled
            } else {
                "2026-01-01T00:00:01Z".parse().unwrap()
            }
        }
    }
    let (sampled_tx, sampled_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let clock = Arc::new(DelayedClock {
        armed: AtomicBool::new(false),
        sampled: sampled_tx,
        release: Mutex::new(release_rx),
    });
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store = GardenStore::initialize_with_clock(
        dir.path().join("concurrent.redb"),
        &workspace,
        &generation,
        clock.clone(),
    )
    .unwrap();
    let auth = store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let store = Arc::new(store);
    let request:QueryRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"requiredExtensions":{},"ids":[],"kinds":[],"limit":1,"cursor":null})).unwrap();
    clock.armed.store(true, Ordering::SeqCst);
    let first_store = store.clone();
    let first_auth = auth.clone();
    let first_request = request.clone();
    let first = std::thread::spawn(move || first_store.query(&first_auth, &first_request));
    sampled_rx.recv_timeout(Duration::from_secs(5)).unwrap();
    let (done_tx, done_rx) = mpsc::channel();
    let second = std::thread::spawn(move || {
        let result = store.query(&auth, &request);
        let _ = done_tx.send(());
        result
    });
    // A later query can finish first only if the first sample was taken outside
    // the writer gate. With correct admission, release the sampled reader first.
    let _ = done_rx.recv_timeout(Duration::from_secs(1));
    release_tx.send(()).unwrap();
    assert!(
        first.join().unwrap().is_ok(),
        "admission order must not manufacture clock rollback"
    );
    assert!(second.join().unwrap().is_ok());
}

#[derive(Debug)]
struct TestClock(Mutex<DateTime<Utc>>);
impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}
struct Fixture {
    dir: tempfile::TempDir,
    store: GardenStore,
    auth: Auth,
    workspace: String,
    generation: String,
    clock: Arc<TestClock>,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let clock = Arc::new(TestClock(Mutex::new(
            "2026-01-01T00:00:00Z".parse().unwrap(),
        )));
        let store = GardenStore::initialize_with_clock(
            dir.path().join("garden.redb"),
            &workspace,
            &generation,
            clock.clone(),
        )
        .unwrap();
        let auth = store
            .grant(
                "writer",
                &["garden:read".into(), "garden:write".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        Self {
            dir,
            store,
            auth,
            workspace,
            generation,
            clock,
        }
    }
    fn apply(&self, items: Value) -> OutcomeResponse {
        self.store.apply(&self.auth,&serde_json::from_value(json!({"version":2,"workspaceId":self.workspace,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items})).unwrap()).unwrap()
    }
    fn query(&self) -> QueryRequest {
        serde_json::from_value(json!({"version":2,"workspaceId":self.workspace,"generation":self.generation,"requiredExtensions":{},"ids":[],"kinds":[],"entityType":null,"predicate":null,"property":null,"validAt":null,"knownAt":null,"text":null,"vector":null,"traversal":null,"limit":100,"cursor":null})).unwrap()
    }
}
fn entity(key: &str, name: &str) -> Value {
    json!({"itemKey":key,"mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}})
}
fn edge(key: &str, from: &str, to: &str) -> Value {
    json!({"itemKey":key,"mutation":"create","expectedRevision":0,"draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":from},"predicate":"test:knows","object":{"type":"entityRef","entityRef":{"refType":"item","kind":"entity","itemKey":to}},"validTime":{"validFrom":"2020-01-01T00:00:00Z","validUntil":"2030-01-01T00:00:00Z","sourceTimezone":null,"precision":"instant"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[],"confidence":null,"extensions":{}}})
}

#[test]
fn exact_filters_literal_and_active_only() {
    let f = Fixture::new();
    let made = f.apply(json!([
        entity("a", "Alpha"),
        entity("b", "Beta"),
        edge("ab", "a", "b")
    ]));
    let mut q = f.query();
    q.ids = vec![made.receipt.results[0].reference.clone()];
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap().hits[0]
            .record
            .metadata
            .id,
        q.ids[0].id
    );
    q.ids.clear();
    q.entity_type = Some("test:person".into());
    q.text = Some("alpha".into());
    let hits = f.store.query(&f.auth, &q).unwrap().hits;
    assert_eq!(hits.len(), 1);
    assert!(hits[0].scores.lexical.is_some());
    assert_eq!(hits[0].scores.vector, None);
    q.text = None;
    q.entity_type = None;
    q.predicate = Some("test:knows".into());
    q.valid_at = Some("2025-01-01T00:00:00Z".into());
    assert_eq!(f.store.query(&f.auth, &q).unwrap().hits.len(), 1);
    q.valid_at = Some("2030-01-01T00:00:00Z".into());
    assert!(f.store.query(&f.auth, &q).unwrap().hits.is_empty());
    f.apply(json!([{"itemKey":"ab","mutation":"retract","id":made.receipt.results[2].reference,"expectedRevision":1}]));
    q.valid_at = None;
    assert!(f.store.query(&f.auth, &q).unwrap().hits.is_empty());
}

#[test]
fn snapshot_survives_reopen_and_updates_but_suppresses_new_retractions() {
    let mut f = Fixture::new();
    f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        entity("c", "C")
    ]));
    let all = f.store.query(&f.auth, &f.query()).unwrap();
    let mut q = f.query();
    q.limit = 1;
    let first = f.store.query(&f.auth, &q).unwrap();
    q.cursor = first.page.next_cursor.clone();
    assert!(q.cursor.is_some());
    let second = &all.hits[1].record;
    let third = &all.hits[2].record;
    let mut draft = second.draft.clone();
    if let GardenDraft::Entity { display_name, .. } = &mut draft {
        *display_name = "new value".into();
    }
    f.apply(json!([{"itemKey":"b","mutation":"update","id":{"kind":"entity","id":second.metadata.id},"expectedRevision":1,"draft":draft},{"itemKey":"c","mutation":"retract","id":{"kind":"entity","id":third.metadata.id},"expectedRevision":1}]));
    let path = f.dir.path().join("garden.redb");
    drop(f.store);
    f.store = GardenStore::open_with_clock(path, f.clock.clone()).unwrap();
    let page = f.store.query(&f.auth, &q).unwrap();
    assert_eq!(page.hits[0].record, *second);
    q.cursor = page.page.next_cursor;
    if q.cursor.is_some() {
        assert!(f.store.query(&f.auth, &q).unwrap().hits.is_empty());
    }
}

#[test]
fn cursor_rejects_actor_request_epoch_tamper_expiry_and_clock_rollback() {
    let f = Fixture::new();
    f.apply(json!([entity("a", "A"), entity("b", "B")]));
    let other = f
        .store
        .grant(
            "other",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let mut q = f.query();
    q.limit = 1;
    let page = f.store.query(&f.auth, &q).unwrap();
    q.cursor = page.page.next_cursor;
    let token = q.cursor.clone().unwrap();
    assert_eq!(
        f.store.query(&other, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    q.entity_type = Some("test:person".into());
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    q.entity_type = None;
    q.cursor = Some(format!("{token}x"));
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    q.cursor = Some(token);
    *f.clock.0.lock().unwrap() = "2025-12-31T23:59:59Z".parse().unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    *f.clock.0.lock().unwrap() = "2026-01-01T00:06:00Z".parse().unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    *f.clock.0.lock().unwrap() = "2026-01-01T00:00:00Z".parse().unwrap();
    f.store.revoke("other").unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    f.store.revoke("writer").unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::Unauthorized
    );
}

#[test]
fn cyclic_traversal_obeys_direction_and_budget() {
    let f = Fixture::new();
    let made = f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        entity("c", "C"),
        edge("ab", "a", "b"),
        edge("bc", "b", "c"),
        edge("ca", "c", "a")
    ]));
    let mut q = f.query();
    q.kinds = vec![GardenKind::Assertion];
    q.traversal = Some(Traversal {
        start: vec![made.receipt.results[0].reference.clone()],
        direction: Direction::Outgoing,
        max_hops: 1,
        max_edges: 20,
        work_budget: 100,
    });
    let out = f.store.query(&f.auth, &q).unwrap();
    assert_eq!(out.hits.len(), 1);
    assert_eq!(
        out.hits[0].record.metadata.id,
        made.receipt.results[3].reference.id
    );
    assert!(out.coverage.truncated);
    q.traversal.as_mut().unwrap().direction = Direction::Incoming;
    let incoming = f.store.query(&f.auth, &q).unwrap();
    assert_eq!(
        incoming.hits[0].record.metadata.id,
        made.receipt.results[5].reference.id
    );
    q.traversal.as_mut().unwrap().max_hops = 8;
    q.traversal.as_mut().unwrap().direction = Direction::Both;
    assert_eq!(f.store.query(&f.auth, &q).unwrap().hits.len(), 3);
    q.traversal.as_mut().unwrap().max_edges = 1;
    let limited = f.store.query(&f.auth, &q).unwrap();
    assert!(limited.coverage.truncated);
    assert!(limited.hits.len() <= 1);
}

#[test]
fn hidden_neighbors_cannot_change_visible_hits_or_coverage() {
    let f = Fixture::new();
    let made = f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        edge("ab", "a", "b")
    ]));
    let reader = f
        .store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::Allowlist {
                objects: vec![
                    made.receipt.results[0].reference.clone(),
                    made.receipt.results[2].reference.clone(),
                ],
            },
        )
        .unwrap();
    let mut q = f.query();
    q.traversal = Some(Traversal {
        start: vec![made.receipt.results[0].reference.clone()],
        direction: Direction::Outgoing,
        max_hops: 8,
        max_edges: 1,
        work_budget: 1,
    });
    q.kinds = vec![GardenKind::Assertion];
    let hidden = f.store.query(&reader, &q).unwrap();
    assert!(hidden.hits.is_empty());
    assert!(hidden.coverage.complete);
    assert!(!hidden.coverage.truncated);
}

#[test]
fn explicit_known_time_preserves_old_active_belief_and_property_filters() {
    let f = Fixture::new();
    let mut a = entity("a", "before");
    a["draft"]["extensions"] =
        json!({"test:color":{"schemaVersion":1,"payload":{"type":"string","value":"blue"}}});
    let made = f.apply(json!([a]));
    let mut q = f.query();
    q.property = Some(PropertyFilter {
        name: "test:color".into(),
        value: TypedValue::String {
            value: "blue".into(),
        },
    });
    assert_eq!(f.store.query(&f.auth, &q).unwrap().hits.len(), 1);
    q.known_at = made.receipt.committed_at.clone();
    f.apply(json!([{"itemKey":"a","mutation":"retract","id":made.receipt.results[0].reference,"expectedRevision":1}]));
    assert_eq!(f.store.query(&f.auth, &q).unwrap().hits.len(), 1);
    q.known_at = None;
    assert!(f.store.query(&f.auth, &q).unwrap().hits.is_empty());
}

#[test]
fn typed_numeric_property_signed_zero_matches_for_all_and_allowlist() {
    let f = Fixture::new();
    let mut assertion = edge("value", "a", "a");
    assertion["draft"]["object"] =
        json!({"type":"value","value":{"type":"number","value":-0.0,"unit":null}});
    let made = f.apply(json!([entity("a", "A"), assertion]));
    let reader = f
        .store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::Allowlist {
                objects: made
                    .receipt
                    .results
                    .iter()
                    .map(|result| result.reference.clone())
                    .collect(),
            },
        )
        .unwrap();
    let mut q = f.query();
    q.property = Some(PropertyFilter {
        name: "test:knows".into(),
        value: TypedValue::Number {
            value: 0.0,
            unit: None,
        },
    });
    assert_eq!(f.store.query(&f.auth, &q).unwrap().hits.len(), 1);
    assert_eq!(f.store.query(&reader, &q).unwrap().hits.len(), 1);
}

#[test]
fn cursor_retry_preserves_successor_and_original_deadline() {
    let f = Fixture::new();
    f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        entity("c", "C")
    ]));
    let mut q = f.query();
    q.limit = 1;
    let first = f.store.query(&f.auth, &q).unwrap();
    q.cursor = first.page.next_cursor;
    *f.clock.0.lock().unwrap() = "2026-01-01T00:04:00Z".parse().unwrap();
    let second = f.store.query(&f.auth, &q).unwrap();
    let retry = f.store.query(&f.auth, &q).unwrap();
    assert_eq!(second, retry);
    assert_eq!(
        second.page.snapshot_expires_at,
        first.page.snapshot_expires_at
    );
    *f.clock.0.lock().unwrap() = "2026-01-01T00:05:00Z".parse().unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
}

#[test]
fn traversal_returns_directed_neighbor_entities_with_predicate_filter() {
    let f = Fixture::new();
    let mut other = edge("ac", "a", "c");
    other["draft"]["predicate"] = json!("test:other");
    let made = f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        entity("c", "C"),
        edge("ab", "a", "b"),
        other
    ]));
    let mut q = f.query();
    q.kinds = vec![GardenKind::Entity];
    q.predicate = Some("test:knows".into());
    q.traversal = Some(Traversal {
        start: vec![made.receipt.results[0].reference.clone()],
        direction: Direction::Outgoing,
        max_hops: 8,
        max_edges: 100,
        work_budget: 100,
    });
    let result = f.store.query(&f.auth, &q).unwrap();
    assert_eq!(result.hits.len(), 1);
    assert_eq!(
        result.hits[0].record.metadata.id,
        made.receipt.results[1].reference.id
    );
    assert!(result.coverage.complete);
}

#[test]
fn historical_results_recheck_current_evidence_footprint() {
    let f = Fixture::new();
    let evidence = |key: &str| json!({"itemKey":key,"mutation":"create","expectedRevision":0,"draft":{"kind":"evidence","source":{"type":"uri","uri":format!("https://example.test/{key}")},"excerpt":key,"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}});
    let mut assertion = edge("assertion", "a", "b");
    assertion["draft"]["evidence"] =
        json!([{"refType":"item","kind":"evidence","itemKey":"visible"}]);
    let made = f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        evidence("visible"),
        evidence("hidden"),
        assertion
    ]));
    let reader = f
        .store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::Allowlist {
                objects: made
                    .receipt
                    .results
                    .iter()
                    .filter(|result| result.item_key != "hidden")
                    .map(|result| result.reference.clone())
                    .collect(),
            },
        )
        .unwrap();
    let mut q = f.query();
    q.kinds = vec![GardenKind::Assertion];
    q.known_at = made.receipt.committed_at;
    assert_eq!(
        f.store.query(&reader, &q).unwrap().hits[0].sources,
        vec![made.receipt.results[2].reference.clone()]
    );
    let mut draft = made
        .visible_records
        .iter()
        .find(|record| record.draft.kind() == GardenKind::Assertion)
        .unwrap()
        .draft
        .clone();
    if let GardenDraft::Assertion { evidence, .. } = &mut draft {
        *evidence = vec![RecordRef::Existing {
            kind: GardenKind::Evidence,
            id: made.receipt.results[3].reference.id.clone(),
        }];
    }
    f.apply(json!([{"itemKey":"changed","mutation":"update","id":made.receipt.results[4].reference,"expectedRevision":1,"draft":draft}]));
    let hidden = f.store.query(&reader, &q).unwrap();
    assert!(hidden.hits.is_empty());
    assert!(hidden.coverage.complete);
}

#[test]
fn observed_clock_high_water_survives_reopen_and_generation_cursor_mismatch_restarts() {
    let mut f = Fixture::new();
    f.apply(json!([
        entity("a", "A"),
        entity("b", "B"),
        entity("c", "C")
    ]));
    let mut q = f.query();
    q.limit = 1;
    q.cursor = f.store.query(&f.auth, &q).unwrap().page.next_cursor;
    let mut wrong = q.clone();
    wrong.generation = Uuid::new_v4().to_string();
    assert_eq!(
        f.store.query(&f.auth, &wrong).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    *f.clock.0.lock().unwrap() = "2026-01-01T00:04:00Z".parse().unwrap();
    f.store.query(&f.auth, &q).unwrap();
    let path = f.dir.path().join("garden.redb");
    drop(f.store);
    *f.clock.0.lock().unwrap() = "2026-01-01T00:03:00Z".parse().unwrap();
    f.store = GardenStore::open_with_clock(path, f.clock.clone()).unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
}

#[test]
fn current_epoch_and_narrowed_grants_invalidate_unexpired_cursors() {
    let f = Fixture::new();
    let made = f.apply(json!([entity("a", "A"), entity("b", "B")]));
    let reader = f
        .store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let mut q = f.query();
    q.limit = 1;
    q.cursor = f.store.query(&f.auth, &q).unwrap().page.next_cursor;
    f.store.revoke("reader").unwrap();
    assert_eq!(
        f.store.query(&f.auth, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    q.cursor = None;
    q.cursor = f.store.query(&f.auth, &q).unwrap().page.next_cursor;
    let narrowed = f
        .store
        .grant(
            "writer",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::Allowlist {
                objects: vec![made.receipt.results[0].reference.clone()],
            },
        )
        .unwrap();
    assert_eq!(
        f.store.query(&narrowed, &q).unwrap_err().code,
        GardenErrorCode::RestartRequired
    );
    assert_eq!(
        f.store.query(&reader, &q).unwrap_err().code,
        GardenErrorCode::Unauthorized
    );
}

#[test]
fn tiny_pages_complete_a_snapshot_larger_than_512_positions() {
    let f = Fixture::new();
    for first in (0..514).step_by(64) {
        let items = (first..(first + 64).min(514))
            .map(|index| entity(&format!("entity{index}"), &format!("Entity {index}")))
            .collect::<Vec<_>>();
        f.apply(json!(items));
    }
    let mut q = f.query();
    q.limit = 1;
    let mut ids = std::collections::BTreeSet::new();
    let mut expiry = None;
    loop {
        let page = f.store.query(&f.auth, &q).unwrap();
        if let Some(expected) = &expiry {
            assert_eq!(&page.page.snapshot_expires_at, expected)
        } else {
            expiry = Some(page.page.snapshot_expires_at.clone())
        }
        assert_eq!(page.hits.len(), 1);
        assert!(ids.insert(page.hits[0].record.metadata.id.clone()));
        if ids.len() == 256 {
            assert_eq!(page, f.store.query(&f.auth, &q).unwrap());
        }
        match page.page.next_cursor {
            Some(cursor) => q.cursor = Some(cursor),
            None => break,
        }
    }
    assert_eq!(ids.len(), 514);
}
