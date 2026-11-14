//! Competing writers and owner revocation share the canonical transaction boundary.
use mother::{
    access::ClientAccess,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, GardenErrorCode},
        store::{CommitStage, GardenStore},
    },
};
use serde_json::json;
use std::sync::{Arc, Barrier, mpsc};
use std::time::Duration;

fn batch(items: serde_json::Value) -> BatchRequest {
    serde_json::from_value(json!({"version":2,
        "workspaceId":"11111111-1111-4111-8111-111111111111",
        "generation":"22222222-2222-4222-8222-222222222222",
        "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items}))
    .unwrap()
}
fn draft(name: &str) -> serde_json::Value {
    json!({"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}})
}
fn create() -> BatchRequest {
    batch(
        json!([{"itemKey":"person","mutation":"create","expectedRevision":0,"draft":draft("before")}]),
    )
}
fn store(path: &std::path::Path) -> GardenStore {
    GardenStore::initialize(
        path,
        "11111111-1111-4111-8111-111111111111",
        "22222222-2222-4222-8222-222222222222",
    )
    .unwrap()
}

#[test]
fn simultaneous_stale_writers_commit_exactly_one_revision() {
    let dir = tempfile::tempdir().unwrap();
    let store = Arc::new(store(&dir.path().join("garden.redb")));
    let auth = store
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let original = store.apply(&auth, &create()).unwrap();
    let reference = &original.receipt.results[0].reference;
    let requests: Vec<_> = ["first","second"].into_iter().map(|name| batch(json!([
        {"itemKey":"person","mutation":"update","expectedRevision":1,"id":reference,"draft":draft(name)}
    ]))).collect();
    let barrier = Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let handles: Vec<_> = requests
            .iter()
            .map(|request| {
                scope.spawn(|| {
                    barrier.wait();
                    store.apply(&auth, request)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let failed = results
        .iter()
        .find_map(|result| result.as_ref().err())
        .unwrap();
    assert_eq!(failed.code, GardenErrorCode::Conflict);
    let stats = store.audit(&auth).unwrap();
    assert_eq!(
        (
            stats.sequence,
            stats.current_records,
            stats.history_records,
            stats.receipts
        ),
        (2, 1, 2, 2)
    );
}

#[test]
fn revocation_serializes_with_in_flight_writer() {
    let dir = tempfile::tempdir().unwrap();
    let store = store(&dir.path().join("garden.redb"));
    let auth = store
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (revoking_tx, revoking_rx) = mpsc::channel();
    let (revoked_tx, revoked_rx) = mpsc::channel();
    let request = create();
    std::thread::scope(|scope| {
        let writer_store = &store;
        let writer_auth = &auth;
        let writer_request = &request;
        let writer = scope.spawn(move || {
            writer_store.apply_with_hook(writer_auth, writer_request, |stage| {
                if stage == CommitStage::BeforeWrite {
                    entered_tx.send(()).unwrap();
                    release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                }
            })
        });
        entered_rx.recv_timeout(Duration::from_secs(10)).unwrap();
        let revoker = scope.spawn(|| {
            revoking_tx.send(()).unwrap();
            let result = store.revoke("writer");
            revoked_tx.send(()).unwrap();
            result
        });
        revoking_rx.recv_timeout(Duration::from_secs(10)).unwrap();
        assert!(matches!(
            revoked_rx.recv_timeout(Duration::from_millis(50)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
        release_tx.send(()).unwrap();
        assert!(writer.join().unwrap().is_ok());
        revoker.join().unwrap().unwrap();
    });
    assert_eq!(
        store.apply(&auth, &request).unwrap_err().code,
        GardenErrorCode::Unauthorized
    );
    let inspector = store
        .grant(
            "inspector",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    assert_eq!(store.audit(&inspector).unwrap().sequence, 1);
}
