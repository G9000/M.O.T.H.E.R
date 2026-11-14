use garden_store::{Assertion, Batch, CommitStage, Entity, Mutation, RedbStore, Traversal};
use std::sync::{Arc, mpsc};
use std::time::Duration;

fn creation() -> Batch {
    Batch {
        operation_id: 1,
        entities: vec![
            Entity {
                id: 1,
                namespace: 0,
                source: 0,
            },
            Entity {
                id: 2,
                namespace: 0,
                source: 0,
            },
        ],
        assertions: vec![Mutation {
            assertion: Assertion {
                id: 1,
                subject: 1,
                predicate: 1,
                object: 2,
                valid_from: 10,
                valid_until: 20,
                source: 0,
                namespace: 0,
            },
            expected_revision: 0,
        }],
    }
}

#[test]
fn atomic_graph_batch_is_durable_and_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("synthetic.redb");
    let batch = creation();
    let receipt = {
        let store = RedbStore::open(&path).unwrap();
        let receipt = store.apply(&batch).unwrap();
        assert_eq!(store.apply(&batch).unwrap(), receipt);
        assert_eq!(store.counts().unwrap().assertions, 1);
        receipt
    };
    let store = RedbStore::open(path).unwrap();
    assert_eq!(store.apply(&batch).unwrap(), receipt);
    assert_eq!(store.counts().unwrap().entities, 2);
    assert_eq!(store.verify_indexes().unwrap().history, 1);
}

#[test]
fn conflicts_and_late_invalid_endpoints_rollback_every_staged_write() {
    let dir = tempfile::tempdir().unwrap();
    let store = RedbStore::open(dir.path().join("rollback.redb")).unwrap();
    let batch = creation();
    store.apply(&batch).unwrap();
    let before = store.verify_indexes().unwrap();
    let mut changed_replay = batch.clone();
    changed_replay.assertions[0].assertion.predicate = 9;
    assert!(store.apply(&changed_replay).is_err());
    let mut late_invalid = Batch {
        operation_id: 2,
        entities: vec![Entity {
            id: 3,
            namespace: 1,
            source: 1,
        }],
        assertions: vec![
            Mutation {
                assertion: Assertion {
                    predicate: 2,
                    ..batch.assertions[0].assertion
                },
                expected_revision: 1,
            },
            Mutation {
                assertion: Assertion {
                    id: 2,
                    object: 999,
                    ..batch.assertions[0].assertion
                },
                expected_revision: 0,
            },
        ],
    };
    assert!(store.apply(&late_invalid).is_err());
    assert_eq!(store.verify_indexes().unwrap(), before);
    assert!(store.snapshot().unwrap().entity(3).unwrap().is_none());
    assert!(store.receipt(2).unwrap().is_none());
    late_invalid.assertions.truncate(1);
    late_invalid.assertions[0].expected_revision = 0;
    assert!(store.apply(&late_invalid).is_err());
    assert_eq!(store.verify_indexes().unwrap(), before);
}

#[test]
fn temporal_correction_preserves_old_knowledge_and_replaces_all_current_indexes() {
    let dir = tempfile::tempdir().unwrap();
    let store = RedbStore::open(dir.path().join("temporal.redb")).unwrap();
    let original = creation();
    let first = store.apply(&original).unwrap();
    let held = store.snapshot().unwrap();
    let changed = Assertion {
        subject: 2,
        object: 1,
        predicate: 7,
        valid_from: 20,
        valid_until: 30,
        source: 99,
        namespace: 99,
        ..original.assertions[0].assertion
    };
    let second = store
        .apply(&Batch {
            operation_id: 2,
            entities: vec![],
            assertions: vec![Mutation {
                assertion: changed,
                expected_revision: 1,
            }],
        })
        .unwrap();
    assert_eq!(held.assertion(1).unwrap().unwrap().revision, 1);
    assert_eq!(held.counts().unwrap().sequence, first.sequence);
    let now = store.snapshot().unwrap();
    assert_eq!(
        now.assertion_at(1, 15, first.sequence)
            .unwrap()
            .unwrap()
            .assertion,
        original.assertions[0].assertion
    );
    assert!(
        now.assertion_at(1, 15, second.sequence).unwrap().is_none(),
        "validity must be checked after resolving known-time"
    );
    assert_eq!(
        now.assertion_at(1, 25, second.sequence)
            .unwrap()
            .unwrap()
            .assertion,
        changed
    );
    assert!(
        now.assertion_at(1, 30, second.sequence).unwrap().is_none(),
        "half-open valid interval"
    );
    assert!(now.outgoing(1, 100).unwrap().is_empty());
    assert_eq!(now.outgoing(2, 100).unwrap().len(), 1);
    let query = Traversal {
        start: 1,
        max_hops: 1,
        max_results: 10,
        max_work: 100,
        valid_at: 15,
        known_at: first.sequence,
        predicate: Some(1),
    };
    assert_eq!(now.neighbors(query).unwrap().assertions.len(), 1);
    assert!(
        now.neighbors(Traversal {
            known_at: second.sequence,
            ..query
        })
        .unwrap()
        .assertions
        .is_empty()
    );
    assert_eq!(
        now.neighbors(Traversal {
            start: 2,
            valid_at: 25,
            known_at: second.sequence,
            predicate: Some(7),
            ..query
        })
        .unwrap()
        .assertions
        .len(),
        1
    );
    assert_eq!(store.verify_indexes().unwrap().history, 2);
}

#[test]
fn traversal_independently_bounds_hops_results_and_examined_history() {
    let dir = tempfile::tempdir().unwrap();
    let store = RedbStore::open(dir.path().join("bounded.redb")).unwrap();
    let mut batch = creation();
    batch.entities.push(Entity {
        id: 3,
        namespace: 0,
        source: 0,
    });
    batch.assertions.extend([
        Mutation {
            assertion: Assertion {
                id: 2,
                subject: 1,
                object: 3,
                ..batch.assertions[0].assertion
            },
            expected_revision: 0,
        },
        Mutation {
            assertion: Assertion {
                id: 3,
                subject: 2,
                object: 3,
                ..batch.assertions[0].assertion
            },
            expected_revision: 0,
        },
    ]);
    store.apply(&batch).unwrap();
    let view = store.snapshot().unwrap();
    let query = Traversal {
        start: 1,
        max_hops: 1,
        max_results: 10,
        max_work: 100,
        valid_at: 15,
        known_at: u64::MAX,
        predicate: None,
    };
    assert_eq!(view.neighbors(query).unwrap().assertions.len(), 2);
    assert_eq!(
        view.neighbors(Traversal {
            max_hops: 2,
            ..query
        })
        .unwrap()
        .assertions
        .len(),
        3
    );
    let limited = view
        .neighbors(Traversal {
            max_results: 1,
            ..query
        })
        .unwrap();
    assert_eq!(limited.assertions.len(), 1);
    assert!(limited.truncated);
    let limited = view
        .neighbors(Traversal {
            max_work: 1,
            ..query
        })
        .unwrap();
    assert_eq!(limited.work, 1);
    assert!(limited.truncated);
    assert!(
        view.neighbors(Traversal {
            max_hops: 17,
            ..query
        })
        .is_err()
    );
    assert_eq!(
        view.neighbors(Traversal {
            known_at: 0,
            ..query
        })
        .unwrap()
        .assertions
        .len(),
        0
    );
}

#[test]
fn held_snapshot_and_second_writer_are_serialized_without_dirty_reads() {
    let dir = tempfile::tempdir().unwrap();
    let store = Arc::new(RedbStore::open(dir.path().join("writers.redb")).unwrap());
    store.apply(&creation()).unwrap();
    let held = store.snapshot().unwrap();
    let (staged_tx, staged_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let writer_one = {
        let store = Arc::clone(&store);
        std::thread::spawn(move || {
            store
                .apply_with_hook(
                    &Batch {
                        operation_id: 2,
                        entities: vec![Entity {
                            id: 3,
                            namespace: 0,
                            source: 0,
                        }],
                        assertions: vec![],
                    },
                    |stage| {
                        if stage == CommitStage::Staged {
                            staged_tx.send(()).unwrap();
                            release_rx.recv().unwrap();
                        }
                    },
                )
                .unwrap()
        })
    };
    staged_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    assert!(store.snapshot().unwrap().entity(3).unwrap().is_none());
    let (started_tx, started_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    let writer_two = {
        let store = Arc::clone(&store);
        std::thread::spawn(move || {
            let receipt = store
                .apply_with_hook(
                    &Batch {
                        operation_id: 3,
                        entities: vec![Entity {
                            id: 4,
                            namespace: 0,
                            source: 0,
                        }],
                        assertions: vec![],
                    },
                    |stage| {
                        if stage == CommitStage::BeforeWrite {
                            started_tx.send(()).unwrap();
                        }
                    },
                )
                .unwrap();
            done_tx.send(receipt).unwrap();
        })
    };
    started_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    assert!(
        done_rx.recv_timeout(Duration::from_millis(100)).is_err(),
        "writer two must wait while first holds the transaction"
    );
    release_tx.send(()).unwrap();
    let first = writer_one.join().unwrap();
    let second = done_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    writer_two.join().unwrap();
    assert_eq!(second.sequence, first.sequence + 1);
    assert!(held.entity(3).unwrap().is_none());
    assert!(held.entity(4).unwrap().is_none());
    assert_eq!(store.verify_indexes().unwrap().entities, 4);
}
