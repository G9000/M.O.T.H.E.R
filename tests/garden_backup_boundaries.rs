use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, contracts::*, store::GardenStore},
};
use serde_json::json;

fn batch(workspace: &str, generation: &str, name: &str) -> BatchRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,
        "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
        {"itemKey":"person","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}}]})).unwrap()
}
fn scopes() -> Vec<String> {
    vec![
        "garden:read".into(),
        "garden:write".into(),
        "garden:propose".into(),
    ]
}

#[test]
fn repeated_restore_retains_historical_generations_but_never_revives_old_proposals() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let workspace = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let source =
        GardenStore::initialize(dir.path().join("source.redb"), &workspace, &generation).unwrap();
    let auth = source
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let original_request = batch(&workspace, &generation, "original");
    let original = source.apply(&auth, &original_request).unwrap();
    let pending = source
        .propose(&auth, &batch(&workspace, &generation, "pending"))
        .unwrap();
    let first_bundle = dir.path().join("first-backup");
    source.backup_to(&first_bundle).unwrap();
    let first = GardenStore::restore_backup(&first_bundle, &dir.path().join("first.redb")).unwrap();
    assert!(first.authorize(&auth).is_err());
    let fresh = first
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let identity = first.identity(&fresh).unwrap();
    let first_generation = identity["generation"].as_str().unwrap();
    assert_ne!(first_generation, generation);
    let blocked=first.review_proposal(json!({"version":2,"workspaceId":workspace,"generation":first_generation,
        "operationId":uuid::Uuid::new_v4().to_string(),"proposalId":pending.proposal_id,"expectedRevision":1,"action":"approve"})).unwrap();
    assert_eq!(blocked.receipt.status, OutcomeStatus::Blocked);
    assert_eq!(first.audit(&fresh).unwrap().sequence, 1);
    first
        .apply(
            &fresh,
            &batch(&workspace, first_generation, "after restore"),
        )
        .unwrap();
    let second_bundle = dir.path().join("second-backup");
    first.backup_to(&second_bundle).unwrap();
    let second =
        GardenStore::restore_backup(&second_bundle, &dir.path().join("second.redb")).unwrap();
    assert!(second.authorize(&fresh).is_err());
    let final_auth = second
        .grant(
            "writer",
            &scopes(),
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let final_identity = second.identity(&final_auth).unwrap();
    let final_generation = final_identity["generation"].as_str().unwrap();
    assert_ne!(final_generation, first_generation);
    assert_ne!(final_generation, generation);
    let stats = second.audit(&final_auth).unwrap();
    assert_eq!(
        (
            stats.current_records,
            stats.history_records,
            stats.receipts,
            stats.sequence
        ),
        (2, 2, 2, 2)
    );
    let request:GetRequest=serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":final_generation,
        "requiredExtensions":{},"reference":original.receipt.results[0].reference,"revision":1,"knownAt":null})).unwrap();
    assert_eq!(
        second.get(&final_auth, &request).unwrap().record,
        original.visible_records[0]
    );
    assert_eq!(
        second
            .apply(&final_auth, &original_request)
            .unwrap_err()
            .code,
        GardenErrorCode::GenerationMismatch
    );
}
