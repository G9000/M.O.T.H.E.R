//! Additive derived-index upgrades and exact audits use synthetic databases only.
use mother::{
    access::ClientAccess,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, GardenErrorCode, QueryRequest},
        store::GardenStore,
    },
};
use redb::{ReadableDatabase, ReadableTable, TableDefinition};
use serde_json::json;
const W: &str = "11111111-1111-4111-8111-111111111111";
const G: &str = "22222222-2222-4222-8222-222222222222";
const META: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_metadata");
const QC: TableDefinition<(u8, &str, &str, u64), u8> =
    TableDefinition::new("garden_query_current_v1");
const QH: TableDefinition<(u8, &str, &str, u64), u8> =
    TableDefinition::new("garden_query_history_v1");
const VS: TableDefinition<((i64, u32), &str, u64), u8> =
    TableDefinition::new("garden_valid_start_v1");
const VE: TableDefinition<((i64, u32), &str, u64), u8> =
    TableDefinition::new("garden_valid_end_v1");
fn query() -> QueryRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":W,"generation":G,"requiredExtensions":{},"ids":[],"kinds":[],"entityType":"test:person","predicate":null,"property":null,"validAt":null,"knownAt":null,"text":null,"vector":null,"traversal":null,"limit":100,"cursor":null})).unwrap()
}
#[test]
fn g5_index_upgrade_backfills_history_without_changing_identity_grants_or_records() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("synthetic.redb");
    let s = GardenStore::initialize(&p, W, G).unwrap();
    let a = s
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let batch:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":W,"generation":G,"requiredExtensions":{},"operationId":uuid::Uuid::new_v4().to_string(),"items":[{"itemKey":"one","expectedRevision":0,"mutation":"create","draft":{"kind":"entity","entityType":"test:person","displayName":"Before","aliases":[],"extensions":{}}}]})).unwrap();
    let made = s.apply(&a, &batch).unwrap();
    let before = s.audit(&a).unwrap();
    drop(s);
    let canonical_state;
    {
        let db = redb::Database::open(&p).unwrap();
        let tx = db.begin_write().unwrap();
        {
            let mut meta = tx.open_table(META).unwrap();
            canonical_state = meta.get("state").unwrap().unwrap().value().to_vec();
            meta.remove("query_indexes_v1").unwrap();
        }
        tx.delete_table(QC).unwrap();
        tx.delete_table(QH).unwrap();
        tx.delete_table(VS).unwrap();
        tx.delete_table(VE).unwrap();
        tx.commit().unwrap();
    }
    let s = GardenStore::open(&p).unwrap();
    assert_eq!(s.audit(&a).unwrap(), before);
    let response = s.query(&a, &query()).unwrap();
    assert_eq!(response.generation, G);
    assert_eq!(
        response.hits[0].record.metadata.id,
        made.receipt.results[0].reference.id
    );
    drop(s);
    {
        let db = redb::Database::open(&p).unwrap();
        let tx = db.begin_read().unwrap();
        assert_eq!(
            tx.open_table(META)
                .unwrap()
                .get("state")
                .unwrap()
                .unwrap()
                .value(),
            canonical_state
        );
        assert!(
            tx.open_table(META)
                .unwrap()
                .get("query_indexes_v1")
                .unwrap()
                .is_some()
        );
    }
    let s = GardenStore::open(&p).unwrap();
    assert_eq!(s.audit(&a).unwrap(), before);
}
#[test]
fn audit_rejects_extra_historical_memberships() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("synthetic.redb");
    let s = GardenStore::initialize(&p, W, G).unwrap();
    let a = s
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    assert_eq!(s.audit(&a).unwrap().history_records, 0);
    drop(s);
    {
        let db = redb::Database::open(&p).unwrap();
        let tx = db.begin_write().unwrap();
        tx.open_table(QH)
            .unwrap()
            .insert((0, "", "33333333-3333-4333-8333-333333333333", 1), 1)
            .unwrap();
        tx.commit().unwrap();
    }
    let s = GardenStore::open(&p).unwrap();
    assert_eq!(
        s.audit(&a).unwrap_err().code,
        GardenErrorCode::StoreUnavailable
    );
}
