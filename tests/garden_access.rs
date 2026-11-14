//! Synthetic checks for future graph authority helpers, not graph handlers.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::{Auth, SCOPES},
    garden::{
        access::{GardenAccess, GardenAction, GardenObjectVisibility, GardenOperationIdentity},
        contracts::{ExternalReference, GardenKind},
    },
    store::Store,
};
use serde_json::json;

fn reference(kind: GardenKind) -> ExternalReference {
    ExternalReference {
        kind,
        id: uuid::Uuid::new_v4().to_string(),
    }
}
fn fixture() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store =
        Store::initialize(&dir.path().join("synthetic.sqlite"), "Synthetic persona").unwrap();
    (dir, store)
}
fn auth(grant: serde_json::Value) -> Auth {
    Auth {
        client_id: grant["clientId"].as_str().unwrap().into(),
        token: grant["token"].as_str().unwrap().into(),
    }
}
fn shared() -> ClientAccess {
    ClientAccess {
        mode: AccessMode::Shared,
        subject: Some("synthetic-agent".into()),
        records: vec![],
    }
}
fn grant(store: &mut Store, name: &str, scopes: &[&str], objects: Vec<ExternalReference>) -> Auth {
    auth(
        store
            .grant_with_garden_access(
                name,
                &scopes.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
                &shared(),
                &GardenAccess::Allowlist { objects },
            )
            .unwrap(),
    )
}
fn check(
    store: &mut Store,
    auth: &Auth,
    action: GardenAction,
    objects: &[GardenObjectVisibility],
) -> mother::error::Result<mother::garden::access::GardenAuthority> {
    let workspace = store.workspace().unwrap();
    store.authorize_garden(
        auth,
        action,
        workspace["id"].as_str().unwrap(),
        workspace["generation"].as_str().unwrap(),
        objects,
    )
}

#[test]
fn graph_scopes_require_explicit_policy_and_never_follow_v1_scopes() {
    let (_dir, mut store) = fixture();
    let old = auth(store.grant("legacy", &SCOPES.map(str::to_owned)).unwrap());
    assert_eq!(
        check(&mut store, &old, GardenAction::Read, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    assert_eq!(
        store
            .grant("implicit", &["read".into(), "garden:read".into()])
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
    let graph = grant(&mut store, "graph", &["read", "garden:read"], vec![]);
    assert_eq!(
        check(&mut store, &graph, GardenAction::Read, &[])
            .unwrap()
            .subject(),
        Some("synthetic-agent")
    );
    assert_eq!(
        check(&mut store, &graph, GardenAction::Propose, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    assert_eq!(
        store
            .grant_with_garden_access(
                "writer",
                &["read".into(), "garden:write".into()],
                &shared(),
                &GardenAccess::default()
            )
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
}

#[test]
fn every_typed_endpoint_evidence_and_object_needs_separate_graph_visibility() {
    let (_dir, mut store) = fixture();
    let assertion = reference(GardenKind::Assertion);
    let entity = reference(GardenKind::Entity);
    let hidden = reference(GardenKind::Entity);
    let evidence = reference(GardenKind::Evidence);
    let client = grant(
        &mut store,
        "graph",
        &["read", "garden:read"],
        vec![assertion.clone(), entity.clone(), evidence.clone()],
    );
    let visible = GardenObjectVisibility {
        object: assertion.clone(),
        endpoints: vec![entity],
        evidence: vec![evidence],
    };
    check(
        &mut store,
        &client,
        GardenAction::Read,
        std::slice::from_ref(&visible),
    )
    .unwrap();
    let mut hidden_endpoint = visible.clone();
    hidden_endpoint.endpoints.push(hidden);
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[hidden_endpoint])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    let mut hidden_evidence = visible.clone();
    hidden_evidence
        .evidence
        .push(reference(GardenKind::Evidence));
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[hidden_evidence])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    let wrong_kind = GardenObjectVisibility {
        object: ExternalReference {
            kind: GardenKind::Entity,
            id: assertion.id,
        },
        endpoints: vec![],
        evidence: vec![],
    };
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[wrong_kind])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
}

#[test]
fn graph_allowlist_never_inherits_canonical_records_or_claims_object_existence() {
    let (_dir, mut store) = fixture();
    let owner = auth(store.grant("owner", &SCOPES.map(str::to_owned)).unwrap());
    let w = store.workspace().unwrap();
    let record = store.write(&owner, json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","status":"active","title":"Synthetic","content":"Synthetic","basis":"owner_statement"}})).unwrap();
    let canonical_id = record["record"]["id"].as_str().unwrap().to_owned();
    let access = ClientAccess {
        records: vec![canonical_id.clone()],
        ..shared()
    };
    let graph = auth(
        store
            .grant_with_garden_access(
                "graph",
                &["read".into(), "garden:read".into()],
                &access,
                &GardenAccess::default(),
            )
            .unwrap(),
    );
    let visible = GardenObjectVisibility {
        object: ExternalReference {
            kind: GardenKind::Entity,
            id: canonical_id,
        },
        endpoints: vec![],
        evidence: vec![],
    };
    assert_eq!(
        check(&mut store, &graph, GardenAction::Read, &[visible])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    // Provisioning a valid UUID is an allowlist declaration, not an existence check.
    grant(
        &mut store,
        "future",
        &["read", "garden:read"],
        vec![reference(GardenKind::Entity)],
    );
}

#[test]
fn full_visibility_is_explicit_companion_or_legacy_only() {
    let (dir, mut store) = fixture();
    let full = GardenAccess::All {};
    assert_eq!(
        store
            .grant_with_garden_access("shared-full", &["garden:read".into()], &shared(), &full)
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
    let companion = ClientAccess {
        mode: AccessMode::Companion,
        subject: Some("synthetic-companion".into()),
        records: vec![],
    };
    let client = auth(
        store
            .grant_with_garden_access("companion", &["garden:read".into()], &companion, &full)
            .unwrap(),
    );
    let visible = GardenObjectVisibility {
        object: reference(GardenKind::Assertion),
        endpoints: vec![reference(GardenKind::Entity)],
        evidence: vec![reference(GardenKind::Evidence)],
    };
    check(
        &mut store,
        &client,
        GardenAction::Read,
        std::slice::from_ref(&visible),
    )
    .unwrap();
    assert_eq!(
        check(&mut store, &client, GardenAction::Write, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    let legacy = auth(
        store
            .grant_with_garden_access(
                "legacy-full",
                &["garden:read".into()],
                &ClientAccess::default(),
                &full,
            )
            .unwrap(),
    );
    check(&mut store, &legacy, GardenAction::Read, &[visible]).unwrap();
    let shared_client = grant(&mut store, "shared-policy", &["garden:read"], vec![]);
    let conn = rusqlite::Connection::open(dir.path().join("synthetic.sqlite")).unwrap();
    conn.execute(
        "UPDATE client_garden_access SET policy=? WHERE client_id='shared-policy'",
        [serde_json::to_string(&full).unwrap()],
    )
    .unwrap();
    assert_eq!(
        check(&mut store, &shared_client, GardenAction::Read, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
}

#[test]
fn graph_grant_anchors_and_shared_write_denial_survive_policy_tampering() {
    let (dir, mut store) = fixture();
    let client = grant(&mut store, "graph", &["garden:read"], vec![]);
    let conn = rusqlite::Connection::open(dir.path().join("synthetic.sqlite")).unwrap();
    conn.execute(
        "UPDATE clients SET scopes=? WHERE id='graph'",
        [json!(["garden:read", "garden:write"]).to_string()],
    )
    .unwrap();
    assert_eq!(
        check(&mut store, &client, GardenAction::Write, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    conn.execute("DELETE FROM client_garden_policy_requirements", [])
        .unwrap();
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    let malformed = GardenAccess::Allowlist {
        objects: vec![ExternalReference {
            kind: GardenKind::Entity,
            id: "not-a-uuid".into(),
        }],
    };
    assert_eq!(
        store
            .grant_with_garden_access("malformed", &["garden:read".into()], &shared(), &malformed,)
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clients WHERE id='malformed'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn garden_policy_wire_shape_is_separate_and_strict() {
    assert_eq!(
        serde_json::to_value(GardenAccess::All {}).unwrap(),
        json!({"mode":"all"})
    );
    assert_eq!(
        serde_json::to_value(GardenAccess::default()).unwrap(),
        json!({"mode":"allowlist","objects":[]})
    );
    for policy in [
        json!({"mode":"all","objects":[]}),
        json!({"mode":"allowlist","records":[]}),
        json!({"mode":"allowlist","objects":[],"subject":"spoofed"}),
    ] {
        assert!(
            serde_json::from_value::<GardenAccess>(policy.clone()).is_err(),
            "policy must be rejected: {policy}"
        );
    }
    assert_eq!(
        serde_json::to_value(shared()).unwrap(),
        json!({"mode":"shared","subject":"synthetic-agent","records":[]})
    );
}

#[test]
fn missing_graph_policy_revocation_and_generation_changes_fail_closed() {
    let (dir, mut store) = fixture();
    let client = grant(&mut store, "graph", &["read", "garden:read"], vec![]);
    let w = store.workspace().unwrap();
    assert_eq!(
        store
            .authorize_garden(
                &client,
                GardenAction::Read,
                w["id"].as_str().unwrap(),
                &uuid::Uuid::new_v4().to_string(),
                &[]
            )
            .unwrap_err()
            .code(),
        "GENERATION_MISMATCH"
    );
    let conn = rusqlite::Connection::open(dir.path().join("synthetic.sqlite")).unwrap();
    conn.execute(
        "DELETE FROM client_garden_access WHERE client_id='graph'",
        [],
    )
    .unwrap();
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
    store.revoke("graph").unwrap();
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[])
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(version, 1);
}

#[test]
fn own_approved_proposal_metadata_does_not_grant_new_object_visibility() {
    let (dir, mut store) = fixture();
    let client = grant(
        &mut store,
        "graph",
        &["read", "garden:read", "garden:propose"],
        vec![],
    );
    let w = store.workspace().unwrap();
    let identity = GardenOperationIdentity {
        workspace_id: w["id"].as_str().unwrap().into(),
        generation: w["generation"].as_str().unwrap().into(),
        actor: "graph".into(),
        subject: Some("synthetic-agent".into()),
        action: GardenAction::Propose,
    };
    let approved = GardenObjectVisibility {
        object: reference(GardenKind::Entity),
        endpoints: vec![],
        evidence: vec![],
    };
    let visibility = store
        .garden_receipt_visibility(&client, &identity, std::slice::from_ref(&approved))
        .unwrap();
    assert!(!visibility.payload_visible);
    assert_eq!(
        check(&mut store, &client, GardenAction::Read, &[approved])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    let mut another = identity.clone();
    another.actor = "someone-else".into();
    assert_eq!(
        store
            .garden_receipt_visibility(&client, &another, &[])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    another = identity.clone();
    another.subject = Some("wrong-subject".into());
    assert_eq!(
        store
            .garden_receipt_visibility(&client, &another, &[])
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    let conn = rusqlite::Connection::open(dir.path().join("synthetic.sqlite")).unwrap();
    conn.execute(
        "UPDATE clients SET scopes=? WHERE id='graph'",
        [json!(["read", "garden:read"]).to_string()],
    )
    .unwrap();
    assert_eq!(
        store
            .garden_receipt_visibility(&client, &identity, &[])
            .unwrap_err()
            .code(),
        "FORBIDDEN"
    );
}
