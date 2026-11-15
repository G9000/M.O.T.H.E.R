use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, contracts::*, store::GardenStore},
};
use serde_json::json;
use std::io::Cursor;
use uuid::Uuid;

fn setup(path: &std::path::Path) -> (GardenStore, mother::contracts::Auth, String, String) {
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store = GardenStore::initialize(path, &workspace, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &[
                "garden:read".into(),
                "garden:write".into(),
                "garden:propose".into(),
            ],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    (store, auth, workspace, generation)
}
fn request(workspace: &str, generation: &str, name: &str) -> BatchRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"entity","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{"opaque:example":{"schemaVersion":1,"payload":{"unfamiliar":[1,{"preserve":"exact"}]}}}}}]})).unwrap()
}
#[test]
fn full_backup_restores_history_receipts_and_pending_proposals_without_credentials() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let (store, auth, workspace, generation) = setup(&dir.path().join("source.redb"));
    let original = store
        .apply(&auth, &request(&workspace, &generation, "original"))
        .unwrap();
    store
        .propose(&auth, &request(&workspace, &generation, "pending"))
        .unwrap();
    let bundle = dir.path().join("backup");
    store.backup_to(&bundle).unwrap();
    GardenStore::verify_backup(&bundle).unwrap();
    let restored = GardenStore::restore_backup(&bundle, &dir.path().join("restored.redb")).unwrap();
    assert!(restored.authorize(&auth).is_err());
    let new_auth = restored
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let identity = restored.identity(&new_auth).unwrap();
    assert_eq!(identity["workspaceId"], workspace);
    assert_ne!(identity["generation"], generation);
    let stats = restored.audit(&new_auth).unwrap();
    assert_eq!(stats.current_records, 1);
    assert_eq!(stats.history_records, 1);
    assert_eq!(stats.receipts, 1);
    assert_eq!(
        restored.owner_inbox_list().unwrap()["proposals"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(
        restored
            .apply(&new_auth, &request(&workspace, &generation, "stale"))
            .is_err()
    );
    let read:GetRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":identity["generation"],"requiredExtensions":{},"reference":original.receipt.results[0].reference,"revision":1,"knownAt":null})).unwrap();
    let record = restored.get(&new_auth, &read).unwrap();
    assert_eq!(record.record, original.visible_records[0]);
    let exported = std::fs::read_dir(&bundle)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|e| e == "jsonl"))
        .map(|entry| std::fs::read_to_string(entry.path()).unwrap())
        .flat_map(|text| {
            text.lines()
                .map(|line| {
                    let row: serde_json::Value = serde_json::from_str(line).unwrap();
                    let encoded = row["value"].as_str().unwrap();
                    let bytes: Vec<u8> = encoded
                        .as_bytes()
                        .as_chunks::<2>()
                        .0
                        .iter()
                        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
                        .collect();
                    String::from_utf8_lossy(&bytes).into_owned()
                })
                .collect::<Vec<_>>()
        })
        .collect::<String>();
    assert!(!exported.contains(&auth.token));
    assert!(!exported.contains(&{
        use sha2::{Digest, Sha256};
        format!("{:x}", Sha256::digest(auth.token.as_bytes()))
    }));
}
#[test]
fn corrupt_partial_unknown_and_existing_destinations_are_rejected_without_publish() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let (store, _, _, _) = setup(&dir.path().join("source.redb"));
    let bundle = dir.path().join("backup");
    store.backup_to(&bundle).unwrap();
    assert!(store.backup_to(&bundle).is_err());
    let destination = dir.path().join("target.redb");
    std::fs::write(bundle.join("unlisted.bin"), b"unknown").unwrap();
    assert!(GardenStore::restore_backup(&bundle, &destination).is_err());
    assert!(!destination.exists());
    std::fs::remove_file(bundle.join("unlisted.bin")).unwrap();
    let manifest = bundle.join("manifest.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&manifest).unwrap()).unwrap();
    value["requiredExtensions"] = json!({"unknown:required":1});
    std::fs::write(&manifest, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(GardenStore::restore_backup(&bundle, &destination).is_err());
    assert!(!destination.exists());
}
#[test]
fn stored_blob_bytes_are_verified_and_restored_atomically() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let (store, _, _, _) = setup(&dir.path().join("source.redb"));
    let bytes = vec![23; 2_300_000];
    let blob = store.put_blob(Cursor::new(&bytes)).unwrap();
    assert_eq!(blob.size_bytes, bytes.len() as u64);
    let bundle = dir.path().join("backup");
    store.backup_to(&bundle).unwrap();
    let restored = GardenStore::restore_backup(&bundle, &dir.path().join("restored.redb")).unwrap();
    let again = restored.put_blob(Cursor::new(&bytes)).unwrap();
    assert_eq!(blob, again);
    let value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(bundle.join("manifest.json")).unwrap()).unwrap();
    let file = value["chunks"][0]["file"].as_str().unwrap();
    std::fs::write(bundle.join(file), b"corrupt\n").unwrap();
    let invalid = dir.path().join("invalid.redb");
    assert!(GardenStore::restore_backup(&bundle, &invalid).is_err());
    assert!(!invalid.exists());
}

#[test]
fn missing_artifact_in_pending_proposal_prevents_backup_until_owner_ingests_bytes() {
    use sha2::{Digest, Sha256};
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let (store, auth, workspace, generation) = setup(&dir.path().join("source.redb"));
    let bytes = b"synthetic artifact source";
    let sha = format!("{:x}", Sha256::digest(bytes));
    let evidence:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"evidence","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence","source":{"type":"artifact","sha256":sha,"sizeBytes":bytes.len(),"mediaType":"text/plain"},"excerpt":null,"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}}]})).unwrap();
    store.propose(&auth, &evidence).unwrap();
    let bundle = dir.path().join("backup");
    assert!(
        store
            .backup_to(&bundle)
            .unwrap_err()
            .message
            .contains("artifact bytes")
    );
    assert!(!bundle.exists());
    store.put_blob(Cursor::new(bytes)).unwrap();
    store.backup_to(&bundle).unwrap();
    GardenStore::verify_backup(&bundle).unwrap();
}
#[test]
fn multiple_restored_generations_retain_original_receipts_and_block_old_proposals() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let (store, auth, workspace, generation) = setup(&dir.path().join("source.redb"));
    store
        .apply(&auth, &request(&workspace, &generation, "first"))
        .unwrap();
    let proposal = store
        .propose(&auth, &request(&workspace, &generation, "pending"))
        .unwrap();
    let first = dir.path().join("first");
    store.backup_to(&first).unwrap();
    let restored = GardenStore::restore_backup(&first, &dir.path().join("second.redb")).unwrap();
    let new_auth = restored
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let identity = restored.identity(&new_auth).unwrap();
    restored
        .apply(
            &new_auth,
            &request(
                &workspace,
                identity["generation"].as_str().unwrap(),
                "second",
            ),
        )
        .unwrap();
    let second = dir.path().join("second");
    restored.backup_to(&second).unwrap();
    let twice = GardenStore::restore_backup(&second, &dir.path().join("third.redb")).unwrap();
    let final_auth = twice
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    assert_eq!(twice.audit(&final_auth).unwrap().receipts, 2);
    let identity = twice.identity(&final_auth).unwrap();
    let reviewed=twice.review_proposal(json!({"version":2,"workspaceId":workspace,"generation":identity["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":proposal.proposal_id,"expectedRevision":1,"action":"approve"})).unwrap();
    assert_eq!(reviewed.receipt.status, OutcomeStatus::Blocked);
    assert_eq!(twice.audit(&final_auth).unwrap().current_records, 2);
}
#[test]
fn streaming_backup_exceeds_legacy_lifetime_limit_and_preserves_chunk_bounds() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let (store, _, _, _) = setup(&dir.path().join("source.redb"));
    store
        .put_blob(std::io::Read::take(std::io::repeat(37), 65 * 1024 * 1024))
        .unwrap();
    let bundle = dir.path().join("large");
    let report = store
        .backup_with_chunk_size(&bundle, 3 * 1024 * 1024)
        .unwrap();
    assert!(report.bytes > 64 * 1024 * 1024);
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(bundle.join("manifest.json")).unwrap()).unwrap();
    for chunk in manifest["chunks"].as_array().unwrap() {
        assert!(chunk["sizeBytes"].as_u64().unwrap() <= 3 * 1024 * 1024);
    }
    GardenStore::verify_backup(&bundle).unwrap();
}
