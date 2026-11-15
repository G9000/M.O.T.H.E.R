//! Exact audit regressions use only synthetic, owner-private databases.
use mother::{
    access::ClientAccess,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, GardenErrorCode},
        store::GardenStore,
    },
};
use redb::{ReadableTable, TableDefinition};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const W: &str = "11111111-1111-4111-8111-111111111111";
const G: &str = "22222222-2222-4222-8222-222222222222";
const QH: TableDefinition<(u8, &str, &str, u64), u8> =
    TableDefinition::new("garden_query_history_v1");
const HISTORY: TableDefinition<(&str, u64), &[u8]> = TableDefinition::new("garden_history");
const RECEIPTS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_receipts");
const REVISIONS: TableDefinition<(&str, u64), u64> = TableDefinition::new("garden_revisions");
const SEQUENCES: TableDefinition<(u64, &str), u64> =
    TableDefinition::new("garden_transaction_sequence");

#[derive(Debug, Clone, Copy)]
enum Damage {
    ExtraPosting,
    MissingPosting,
    WrongPostingValue,
    MissingHistory,
    ExtraRevision,
    MissingSequence,
    MissingReceipt,
    DuplicateReceiptSequence,
    MissingReceiptResult,
}

#[test]
fn audit_checks_late_rows_and_exact_bijections() {
    for damage in [
        Damage::ExtraPosting,
        Damage::MissingPosting,
        Damage::WrongPostingValue,
        Damage::MissingHistory,
        Damage::ExtraRevision,
        Damage::MissingSequence,
        Damage::MissingReceipt,
        Damage::DuplicateReceiptSequence,
        Damage::MissingReceiptResult,
    ] {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("synthetic.redb");
        let store = GardenStore::initialize(&path, W, G).unwrap();
        let auth = store
            .grant(
                "writer",
                &["garden:read".into(), "garden:write".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        let mut last_id = String::new();
        for batch in 0..4 {
            let request: BatchRequest = serde_json::from_value(json!({
                "version":2,"workspaceId":W,"generation":G,"requiredExtensions":{},
                "operationId":uuid::Uuid::new_v4().to_string(),
                "items":(0..8).map(|item| json!({
                    "itemKey":format!("item-{item}"),"expectedRevision":0,"mutation":"create",
                    "draft":{"kind":"entity","entityType":"test:person",
                    "displayName":format!("Synthetic {batch} {item}"),"aliases":[],"extensions":{}}
                })).collect::<Vec<_>>()
            }))
            .unwrap();
            let applied = store.apply(&auth, &request).unwrap();
            last_id = applied.receipt.results.last().unwrap().reference.id.clone();
        }
        let stats = store.audit(&auth).unwrap();
        assert_eq!(
            (
                stats.current_records,
                stats.history_records,
                stats.receipts,
                stats.sequence
            ),
            (32, 32, 4, 4)
        );
        drop(store);
        {
            let db = redb::Database::open(&path).unwrap();
            let tx = db.begin_write().unwrap();
            match damage {
                Damage::ExtraPosting => {
                    tx.open_table(QH)
                        .unwrap()
                        .insert((255, "late-extra", last_id.as_str(), 4), 1)
                        .unwrap();
                }
                Damage::MissingPosting => {
                    tx.open_table(QH)
                        .unwrap()
                        .remove((0, "", last_id.as_str(), 4))
                        .unwrap();
                }
                Damage::WrongPostingValue => {
                    tx.open_table(QH)
                        .unwrap()
                        .insert((0, "", last_id.as_str(), 4), 2)
                        .unwrap();
                }
                Damage::MissingHistory => {
                    tx.open_table(HISTORY)
                        .unwrap()
                        .remove((last_id.as_str(), 4))
                        .unwrap();
                }
                Damage::ExtraRevision => {
                    tx.open_table(REVISIONS)
                        .unwrap()
                        .insert((last_id.as_str(), 2), 4)
                        .unwrap();
                }
                Damage::MissingSequence => {
                    tx.open_table(SEQUENCES)
                        .unwrap()
                        .remove((4, last_id.as_str()))
                        .unwrap();
                }
                Damage::MissingReceipt
                | Damage::DuplicateReceiptSequence
                | Damage::MissingReceiptResult => {
                    let mut table = tx.open_table(RECEIPTS).unwrap();
                    let (key, mut operation) = table
                        .iter()
                        .unwrap()
                        .map(|row| {
                            let (key, bytes) = row.unwrap();
                            (
                                key.value().to_owned(),
                                serde_json::from_slice::<Value>(bytes.value()).unwrap(),
                            )
                        })
                        .find(|(_, operation)| operation["receipt"]["commitSequence"] == 4)
                        .unwrap();
                    match damage {
                        Damage::MissingReceipt => {
                            table.remove(key.as_str()).unwrap();
                        }
                        Damage::DuplicateReceiptSequence => {
                            let removed_key = table
                                .iter()
                                .unwrap()
                                .map(|row| {
                                    let (key, bytes) = row.unwrap();
                                    (
                                        key.value().to_owned(),
                                        serde_json::from_slice::<Value>(bytes.value()).unwrap(),
                                    )
                                })
                                .find(|(_, operation)| operation["receipt"]["commitSequence"] == 3)
                                .unwrap()
                                .0;
                            table.remove(removed_key.as_str()).unwrap();
                            let id = uuid::Uuid::new_v4().to_string();
                            operation["request"]["operationId"] = json!(id);
                            operation["receipt"]["operationId"] = json!(id);
                            operation["payload_hash"] = json!(format!(
                                "{:x}",
                                Sha256::digest(
                                    serde_json::to_string(&operation["request"]).unwrap()
                                )
                            ));
                            let extra_key = format!("{G}/writer/{id}");
                            let bytes = serde_json::to_vec(&operation).unwrap();
                            table.insert(extra_key.as_str(), bytes.as_slice()).unwrap();
                        }
                        Damage::MissingReceiptResult => {
                            operation["receipt"]["results"]
                                .as_array_mut()
                                .unwrap()
                                .pop();
                            operation["records"].as_array_mut().unwrap().pop();
                            let bytes = serde_json::to_vec(&operation).unwrap();
                            table.insert(key.as_str(), bytes.as_slice()).unwrap();
                        }
                        _ => unreachable!("receipt damage only"),
                    }
                }
            }
            tx.commit().unwrap();
        }
        let store = GardenStore::open(&path).unwrap();
        assert_eq!(
            store.audit(&auth).unwrap_err().code,
            GardenErrorCode::StoreUnavailable,
            "{damage:?}"
        );
    }
}
