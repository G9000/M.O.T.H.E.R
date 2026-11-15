//! Unit qualification of retained owner recovery against synthetic pre-existing journals.
use super::*;
use crate::access::AccessMode;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

struct Fixture {
    _root: tempfile::TempDir,
    state: MigrationStatus,
}
impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        let journal = root.path().join("journal");
        fs::create_dir(&journal).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&journal, fs::Permissions::from_mode(0o700)).unwrap();
        }
        let destination = root.path().join("native.redb");
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(&destination, &workspace, &generation).unwrap();
        store
            .v1_initialize("Synthetic cutover qualification persona")
            .unwrap();
        let fingerprint = store.migration_fingerprint().unwrap();
        drop(store);
        let state = MigrationStatus {
            version: 1,
            revision: 0,
            request: MigrationRequest {
                source: root.path().join("original.sqlite"),
                destination,
                journal,
                operation_id: Uuid::new_v4().to_string(),
            },
            phase: MigrationPhase::Activated,
            workspace_id: workspace,
            source_generation: Uuid::new_v4().to_string(),
            destination_generation: Some(generation),
            source_fingerprint: "synthetic opaque source identity".into(),
            destination_fingerprint: Some(fingerprint),
            source_archive_fingerprint: None,
            archive: root.path().join("archive.sqlite"),
            source_readonly: false,
            restrictions: vec![],
        };
        let fixture = Self { _root: root, state };
        fixture.write_state();
        publish_json(
            &sidecar(&fixture.state.request.destination),
            &Fence {
                version: 1,
                journal: fixture.state.request.journal.clone(),
                operation_id: fixture.state.request.operation_id.clone(),
                role: "destination".into(),
            },
        )
        .unwrap();
        fixture
    }
    fn write_state(&self) {
        fs::write(
            self.state.request.journal.join("000000.json"),
            serde_json::to_vec(&self.state).unwrap(),
        )
        .unwrap();
    }
    fn config(&self, name: &str) -> PathBuf {
        self.state.request.destination.with_file_name(name)
    }
    fn restriction(&mut self, scopes: &[&str], access: ClientAccess) {
        self.state.restrictions = vec![GrantRestriction {
            client_id: "synthetic-reissued".into(),
            scopes: scopes.iter().map(|scope| (*scope).into()).collect(),
            access,
            garden: Some(GardenAccess::default()),
            policy_required: true,
            garden_required: true,
            revoked: false,
        }];
        self.write_state();
    }
    fn reissue(&self, config: &Path) -> Result<MigrationStatus> {
        reissue(&self.state.request.journal, "synthetic-reissued", config)
    }
    fn rollback(&self) -> Result<MigrationStatus> {
        rollback(
            &self.state.request.journal,
            &self.state.request.operation_id,
        )
    }
}

#[test]
fn native_reissuance_preserves_exact_restrictions_and_same_output_without_read_widening() {
    for scopes in [
        vec!["read", "context:propose"],
        vec!["memory:write"],
        vec!["garden:read"],
    ] {
        let mut fixture = Fixture::new();
        let access = if scopes.contains(&"memory:write") {
            ClientAccess::default()
        } else {
            ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("synthetic-agent".into()),
                records: vec![],
            }
        };
        fixture.restriction(&scopes, access);
        let output = fixture.config("reissued.private.json");
        let first = fixture.reissue(&output).unwrap();
        let bytes = fs::read(&output).unwrap();
        assert_eq!(fixture.reissue(&output).unwrap(), first);
        assert_eq!(fs::read(&output).unwrap(), bytes);
        let config = super::super::config::load_garden_config(&output).unwrap();
        let store = GardenStore::open_active(&fixture.state.request.destination).unwrap();
        let restriction = &fixture.state.restrictions[0];
        store
            .verify_migration_grant(
                &config.auth(),
                &restriction.scopes,
                &restriction.access,
                restriction.garden.as_ref(),
            )
            .unwrap();
        if !scopes.contains(&"read") {
            assert_eq!(
                store
                    .v1_authorize(&config.auth(), "read")
                    .unwrap_err()
                    .code(),
                "FORBIDDEN"
            );
        } else {
            let context = store
                .v1_call(&config.auth(), "mother_get_context", json!({}))
                .unwrap();
            assert!(context.get("persona").is_none());
            assert_eq!(context["inventory"]["totalRecords"], 0);
        }
        drop(store);
        let other = fixture.config("different.private.json");
        assert_eq!(
            fixture.reissue(&other).unwrap_err().code(),
            "OPERATION_REUSED"
        );
        assert!(!other.exists());
        assert_eq!(fs::read(&output).unwrap(), bytes);
    }
}

#[test]
fn native_reissuance_refuses_revoked_source_and_changed_or_revoked_destination_authority() {
    let mut revoked = Fixture::new();
    revoked.restriction(&["read"], ClientAccess::default());
    revoked.state.restrictions[0].revoked = true;
    revoked.write_state();
    let output = revoked.config("revoked.private.json");
    assert_eq!(revoked.reissue(&output).unwrap_err().code(), "FORBIDDEN");
    assert!(!output.exists());

    for mutation in ["revoke", "change"] {
        let mut fixture = Fixture::new();
        fixture.restriction(&["read"], ClientAccess::default());
        let output = fixture.config("issued.private.json");
        fixture.reissue(&output).unwrap();
        let original = fs::read(&output).unwrap();
        let store = GardenStore::open_active(&fixture.state.request.destination).unwrap();
        if mutation == "revoke" {
            store.revoke("synthetic-reissued").unwrap();
        } else {
            store
                .v1_grant(
                    "synthetic-reissued",
                    &["read".into(), "memory:write".into()],
                    &ClientAccess::default(),
                    Some(&GardenAccess::default()),
                )
                .unwrap();
        }
        drop(store);
        assert_eq!(
            fixture.reissue(&output).unwrap_err().code(),
            "RECONCILIATION_REQUIRED"
        );
        assert_eq!(fs::read(&output).unwrap(), original);
    }
}

#[test]
fn unchanged_native_destination_rolls_back_opaque_source_and_appends_idempotent_journal() {
    let mut fixture = Fixture::new();
    let original = b"Synthetic opaque source bytes; no database engine is used";
    fs::write(&fixture.state.archive, original).unwrap();
    fixture.state.source_archive_fingerprint = Some(format!("{:x}", Sha256::digest(original)));
    fixture.write_state();
    publish_json(
        &sidecar(&fixture.state.request.source),
        &Fence {
            version: 1,
            journal: fixture.state.request.journal.clone(),
            operation_id: fixture.state.request.operation_id.clone(),
            role: "source".into(),
        },
    )
    .unwrap();
    let original_transition = fs::read(fixture.state.request.journal.join("000000.json")).unwrap();
    let rolled_back = fixture.rollback().unwrap();
    assert_eq!(rolled_back.phase, MigrationPhase::RolledBack);
    assert_eq!(rolled_back.revision, 2);
    assert_eq!(fs::read(&fixture.state.request.source).unwrap(), original);
    assert!(!fixture.state.archive.exists());
    assert!(!sidecar(&fixture.state.request.source).exists());
    assert_eq!(
        fs::read(fixture.state.request.journal.join("000000.json")).unwrap(),
        original_transition
    );
    assert_eq!(fixture.rollback().unwrap(), rolled_back);
    assert_eq!(
        check_destination(&fixture.state.request.destination)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
}

#[test]
fn native_rollback_refuses_canonical_writes_and_uncertain_missing_destination() {
    for change in ["canonical_write", "missing"] {
        let fixture = Fixture::new();
        let original = b"Synthetic source must remain untouched";
        fs::write(&fixture.state.request.source, original).unwrap();
        if change == "missing" {
            fs::rename(
                &fixture.state.request.destination,
                fixture.config("preserved.redb"),
            )
            .unwrap();
        } else {
            let store = GardenStore::open_active(&fixture.state.request.destination).unwrap();
            let auth = store
                .v1_grant(
                    "writer",
                    &["memory:write".into()],
                    &ClientAccess::default(),
                    None,
                )
                .unwrap();
            let request: Value = json!({"workspaceId":fixture.state.workspace_id,"generation":fixture.state.destination_generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Acknowledged write","content":"Must survive rollback refusal","status":"active","basis":"owner_statement"}});
            store
                .v1_call(&auth, "mother_write_record", request)
                .unwrap();
        }
        let before = fs::read(fixture.state.request.journal.join("000000.json")).unwrap();
        assert_eq!(
            fixture.rollback().unwrap_err().code(),
            "RECONCILIATION_REQUIRED"
        );
        assert_eq!(fs::read(&fixture.state.request.source).unwrap(), original);
        assert_eq!(
            fs::read(fixture.state.request.journal.join("000000.json")).unwrap(),
            before
        );
        assert!(!fixture.state.request.journal.join("000001.json").exists());
    }
}

#[test]
fn prior_native_rollback_never_clears_a_newer_operations_source_fence() {
    let fixture = Fixture::new();
    let original = b"Synthetic restored source retained across later cutover";
    fs::write(&fixture.state.request.source, original).unwrap();
    let rolled_back = fixture.rollback().unwrap();
    let marker = sidecar(&fixture.state.request.source);
    publish_json(
        &marker,
        &Fence {
            version: 1,
            journal: fixture
                .state
                .request
                .journal
                .with_file_name("newer-journal"),
            operation_id: Uuid::new_v4().to_string(),
            role: "source".into(),
        },
    )
    .unwrap();
    let newer_fence = fs::read(&marker).unwrap();
    assert_eq!(fixture.rollback().unwrap_err().code(), "MIGRATION_FENCED");
    assert_eq!(fs::read(&marker).unwrap(), newer_fence);
    assert_eq!(fs::read(&fixture.state.request.source).unwrap(), original);
    assert_eq!(status(&fixture.state.request.journal).unwrap(), rolled_back);
    assert_eq!(
        check_source(&fixture.state.request.source)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
}

#[test]
fn native_reissuance_rejects_invalid_or_nonprivate_output_before_publishing_intent() {
    let mut fixture = Fixture::new();
    fixture.restriction(&["read"], ClientAccess::default());
    let intent = fixture.state.request.journal.join(format!(
        "reissue-{}.json",
        crate::contracts::hash("synthetic-reissued")
    ));
    let blocker = fixture.config("blocked-parent");
    let original = b"Synthetic existing parent file must remain unchanged";
    fs::write(&blocker, original).unwrap();
    let output = blocker.join("credential.private.json");
    assert_eq!(
        fixture.reissue(&output).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(fs::read(&blocker).unwrap(), original);
    assert!(!output.exists());
    assert!(!intent.exists());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let public = fixture.config("public-parent");
        fs::create_dir(&public).unwrap();
        fs::set_permissions(&public, fs::Permissions::from_mode(0o777)).unwrap();
        let output = public.join("credential.private.json");
        assert_eq!(
            fixture.reissue(&output).unwrap_err().code(),
            "MIGRATION_FENCED"
        );
        assert!(!output.exists());
        assert!(!intent.exists());
        fs::set_permissions(&public, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let store = GardenStore::open_active(&fixture.state.request.destination).unwrap();
    assert!(!store.migration_grant_exists("synthetic-reissued").unwrap());
}
