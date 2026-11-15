#![cfg(feature = "legacy-sqlite")]
use mother::{
    garden::{
        migrate::{self, MigrationPhase, MigrationRequest},
        store::GardenStore,
    },
    store::Store,
};
use serde_json::{Value, json};
use uuid::Uuid;

fn setup() -> (tempfile::TempDir, MigrationRequest, Value) {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let source = root.join("source.sqlite");
    let mut store = Store::initialize(&source, "Synthetic migration persona").unwrap();
    store
        .grant("synthetic-client", &["read".into(), "memory:write".into()])
        .unwrap();
    let bundle = store.export().unwrap();
    drop(store);
    let request = MigrationRequest {
        source,
        destination: root.join("destination.redb"),
        journal: root.join("cutover"),
        operation_id: Uuid::new_v4().to_string(),
    };
    (dir, request, bundle)
}

#[test]
fn dry_run_is_read_only_and_validates_fresh_generation_without_credentials() {
    let (_dir, request, mut expected) = setup();
    let before = std::fs::read(&request.source).unwrap();
    let prepared = migrate::prepare(request.clone()).unwrap();
    assert_eq!(prepared.phase, MigrationPhase::Prepared);
    assert_eq!(std::fs::read(&request.source).unwrap(), before);
    assert!(migrate::check_source(&request.source).is_ok());
    assert_eq!(
        migrate::check_destination(&request.destination)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    let destination = GardenStore::open(&request.destination).unwrap();
    let mut actual = destination.v1_export().unwrap();
    assert_ne!(
        actual["workspace"]["generation"],
        expected["workspace"]["generation"]
    );
    expected["workspace"]["generation"] = actual["workspace"]["generation"].clone();
    actual["exportedAt"] = expected["exportedAt"].clone();
    assert_eq!(actual, expected);
    assert_eq!(migrate::prepare(request.clone()).unwrap(), prepared);
    let text = std::fs::read_to_string(request.journal.join("000001.json")).unwrap();
    assert!(!text.contains("token"));
}

#[test]
fn final_source_changes_fail_without_fencing_or_losing_the_source() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let mut source = Store::open(&request.source).unwrap();
    let hash = source.persona().unwrap()["hash"]
        .as_str()
        .unwrap()
        .to_owned();
    source
        .update_persona("Changed synthetic persona", &hash)
        .unwrap();
    drop(source);
    assert_eq!(
        migrate::activate(&request.journal, &request.operation_id)
            .unwrap_err()
            .code(),
        "SOURCE_CHANGED"
    );
    assert!(migrate::check_source(&request.source).is_ok());
}

#[test]
fn activation_fences_source_and_rollback_restores_only_unchanged_canonical_state() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let active = migrate::activate(&request.journal, &request.operation_id).unwrap();
    assert_eq!(active.phase, MigrationPhase::Activated);
    assert_eq!(
        migrate::activate(&request.journal, &request.operation_id).unwrap(),
        active
    );
    assert_eq!(
        migrate::check_source(&request.source).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert!(migrate::check_destination(&request.destination).is_ok());
    let config = request.journal.join("new-client.json");
    migrate::reissue(&request.journal, "synthetic-client", &config).unwrap();
    migrate::reissue(&request.journal, "synthetic-client", &config).unwrap();
    assert_eq!(
        migrate::rollback(&request.journal, &request.operation_id)
            .unwrap()
            .phase,
        MigrationPhase::RolledBack
    );
    assert!(Store::open(&request.source).is_ok());
    assert!(migrate::check_destination(&request.destination).is_err());
}

#[test]
fn rollback_rejects_new_destination_canonical_state() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    let store = GardenStore::open(&request.destination).unwrap();
    store.put_blob(&b"new acknowledged bytes"[..]).unwrap();
    drop(store);
    assert_eq!(
        migrate::rollback(&request.journal, &request.operation_id)
            .unwrap_err()
            .code(),
        "RECONCILIATION_REQUIRED"
    );
    assert!(migrate::check_source(&request.source).is_err());
}

#[test]
fn existing_source_connection_cannot_mutate_after_fence() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let mut existing = Store::open(&request.source).unwrap();
    let hash = existing.persona().unwrap()["hash"]
        .as_str()
        .unwrap()
        .to_owned();
    // Activation can require connection shutdown on platforms which cannot archive
    // an open SQLite file; a persisted source fence must already reject the write.
    let _ = migrate::activate(&request.journal, &request.operation_id);
    assert_eq!(
        existing
            .update_persona("must not be acknowledged", &hash)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    drop(existing);
    migrate::activate(&request.journal, &request.operation_id).unwrap();
}

#[test]
fn an_operation_uuid_cannot_be_rebound_to_changed_paths() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let mut other = request;
    other.destination = other.destination.with_file_name("other.redb");
    assert_eq!(
        migrate::prepare(other).unwrap_err().code(),
        "OPERATION_REUSED"
    );
}

#[test]
fn interrupted_preparation_and_each_authority_barrier_resume_with_same_uuid() {
    use migrate::MigrationStage;
    for stage in [
        MigrationStage::Preparing,
        MigrationStage::DestinationValidated,
        MigrationStage::SourceFenced,
        MigrationStage::SourceArchived,
        MigrationStage::Activated,
    ] {
        let (_dir, request, _) = setup();
        let inject = |at| {
            if at == stage {
                Err(mother::error::Error::domain(
                    "SYNTHETIC_STOP",
                    "Synthetic interrupted owner command",
                ))
            } else {
                Ok(())
            }
        };
        if matches!(
            stage,
            MigrationStage::Preparing | MigrationStage::DestinationValidated
        ) {
            assert_eq!(
                migrate::prepare_with_hook(request.clone(), inject)
                    .unwrap_err()
                    .code(),
                "SYNTHETIC_STOP"
            );
            assert_eq!(
                migrate::prepare(request.clone()).unwrap().phase,
                MigrationPhase::Prepared
            );
        } else {
            migrate::prepare(request.clone()).unwrap();
            assert_eq!(
                migrate::activate_with_hook(&request.journal, &request.operation_id, inject)
                    .unwrap_err()
                    .code(),
                "SYNTHETIC_STOP"
            );
            assert!(migrate::check_source(&request.source).is_err());
            assert_eq!(
                migrate::activate(&request.journal, &request.operation_id)
                    .unwrap()
                    .phase,
                MigrationPhase::Activated
            );
        }
    }
}

#[test]
fn full_v1_history_sources_proposals_pins_and_assignments_survive_without_old_tokens() {
    use mother::contracts::Auth;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let fixture: Value = serde_json::from_str(include_str!("fixtures/v1.json")).unwrap();
    let source = root.join("full.sqlite");
    let mut store = Store::restore(&source, fixture["bundle"].clone()).unwrap();
    let scopes = ["read", "memory:write", "task:write", "context:propose"].map(str::to_owned);
    let key = store.grant("full-client", &scopes).unwrap();
    let auth = Auth {
        client_id: "full-client".into(),
        token: key["token"].as_str().unwrap().into(),
    };
    let w = store.workspace().unwrap();
    let records = fixture["bundle"]["records"].as_array().unwrap();
    let memory = records
        .iter()
        .find(|record| record["kind"] == "memory")
        .unwrap();
    let task = records
        .iter()
        .find(|record| record["kind"] == "task" && record["status"] == "doing")
        .unwrap();
    store
        .owner_pin(memory["id"].as_str().unwrap(), true)
        .unwrap();
    store
        .assign(&auth, json!({"id":task["id"],"assignee":"full-client"}))
        .unwrap();
    let proposal = store.propose(&auth, json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Pending synthetic memory","content":"Synthetic migration test","status":"active","basis":"owner_statement"},"source":"Synthetic owner test","basis":"owner_statement"})).unwrap();
    let mut expected = store.export().unwrap();
    drop(store);
    let request = MigrationRequest {
        source,
        destination: root.join("full.redb"),
        journal: root.join("journal"),
        operation_id: Uuid::new_v4().to_string(),
    };
    migrate::prepare(request.clone()).unwrap();
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    let native = GardenStore::open(&request.destination).unwrap();
    assert_eq!(
        native.v1_authorize(&auth, "read").unwrap_err().code(),
        "UNAUTHORIZED"
    );
    let mut actual = native.v1_export().unwrap();
    expected["workspace"]["generation"] = actual["workspace"]["generation"].clone();
    actual["exportedAt"] = expected["exportedAt"].clone();
    assert_eq!(actual, expected);
    let current = native
        .v1_inbox_get(proposal["proposal"]["id"].as_str().unwrap())
        .unwrap();
    assert_eq!(current["generation"], w["generation"]);
    drop(native);
    let output = request.journal.join("full-client.json");
    migrate::reissue(&request.journal, "full-client", &output).unwrap();
    let config = mother::garden::config::load_garden_config(&output).unwrap();
    assert_ne!(config.token, auth.token);
    let native = GardenStore::open(&request.destination).unwrap();
    let review = json!({"workspaceId":w["id"],"generation":expected["workspace"]["generation"],"operationId":Uuid::new_v4().to_string(),"proposalId":proposal["proposal"]["id"],"expectedRevision":1,"action":"save"});
    assert_eq!(
        native.v1_review_proposal(review).unwrap_err().code(),
        "STALE_GENERATION"
    );
    native
        .v1_call(
            &config.auth(),
            "mother_pin_record",
            json!({"id":memory["id"],"pinned":false}),
        )
        .unwrap();
    drop(native);
    assert_eq!(
        migrate::rollback(&request.journal, &request.operation_id)
            .unwrap_err()
            .code(),
        "RECONCILIATION_REQUIRED"
    );
}

#[test]
fn already_open_client_and_owner_mutations_are_rejected_inside_admission_after_fence() {
    use mother::contracts::Auth;
    let (_dir, request, _) = setup();
    let mut source = Store::open(&request.source).unwrap();
    let key = source
        .grant(
            "open-client",
            &[
                "read".into(),
                "memory:write".into(),
                "task:write".into(),
                "context:propose".into(),
            ],
        )
        .unwrap();
    let auth = Auth {
        client_id: "open-client".into(),
        token: key["token"].as_str().unwrap().into(),
    };
    let workspace = source.workspace().unwrap();
    drop(source);
    migrate::prepare(request.clone()).unwrap();
    let mut source = Store::open(&request.source).unwrap();
    let _ = migrate::activate_with_hook(&request.journal, &request.operation_id, |stage| {
        if stage == migrate::MigrationStage::SourceFenced {
            Err(mother::error::Error::domain(
                "SYNTHETIC_STOP",
                "Stopped with source fence",
            ))
        } else {
            Ok(())
        }
    });
    let id = Uuid::new_v4().to_string();
    assert_eq!(
        source.context(&auth, json!({})).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        source
            .grant("new-fenced", &["read".into()])
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        source.revoke("open-client").unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        source.owner_pin(&id, true).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        source
            .pin(&auth, json!({"id":id,"pinned":true}))
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        source
            .assign(&auth, json!({"id":id,"assignee":null}))
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    let draft = json!({"kind":"memory","title":"Fenced","content":"Never acknowledge","status":"active","basis":"owner_statement"});
    let mut write = json!({"workspaceId":workspace["id"],"generation":workspace["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":draft});
    assert_eq!(
        source.write(&auth, write.clone()).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    write["source"] = json!("Synthetic fence test");
    write["basis"] = json!("owner_statement");
    assert_eq!(
        source.propose(&auth, write).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
}

#[test]
#[ignore = "synthetic subprocess helper, invoked only by the recovery qualification"]
fn migration_process_cutover_worker() {
    let request: MigrationRequest =
        serde_json::from_str(&std::env::var("MOTHER_MIGRATION_SYNTHETIC_REQUEST").unwrap())
            .unwrap();
    let selected = std::env::var("MOTHER_MIGRATION_SYNTHETIC_STAGE").unwrap();
    let hook = |stage| {
        if format!("{stage:?}") == selected {
            std::process::exit(71);
        }
        Ok(())
    };
    if selected == "DestinationValidated" {
        migrate::prepare_with_hook(request, hook).unwrap();
    } else {
        migrate::activate_with_hook(&request.journal, &request.operation_id, hook).unwrap();
    }
    panic!("Synthetic subprocess did not reach its selected barrier");
}

#[test]
fn process_exit_at_durable_barriers_recovers_without_normal_cleanup() {
    for stage in [
        "DestinationValidated",
        "SourceFenced",
        "SourceArchived",
        "Activated",
    ] {
        let (_dir, request, _) = setup();
        if stage != "DestinationValidated" {
            migrate::prepare(request.clone()).unwrap();
        }
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "migration_process_cutover_worker",
                "--ignored",
                "--nocapture",
            ])
            .env(
                "MOTHER_MIGRATION_SYNTHETIC_REQUEST",
                serde_json::to_string(&request).unwrap(),
            )
            .env("MOTHER_MIGRATION_SYNTHETIC_STAGE", stage)
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(71), "{stage}");
        if stage == "DestinationValidated" {
            assert!(migrate::check_source(&request.source).is_ok());
            assert!(migrate::check_destination(&request.destination).is_err());
            migrate::prepare(request.clone()).unwrap();
        } else {
            assert!(migrate::check_source(&request.source).is_err());
        }
        assert_eq!(
            migrate::activate(&request.journal, &request.operation_id)
                .unwrap()
                .phase,
            MigrationPhase::Activated
        );
        migrate::rollback(&request.journal, &request.operation_id).unwrap();
        assert!(Store::open(&request.source).is_ok());
    }
}

#[test]
fn graph_only_and_write_only_reissuance_preserves_scopes_and_retries_without_read_widening() {
    use mother::{access::ClientAccess, garden::access::GardenAccess};
    let (_dir, request, _) = setup();
    let mut source = Store::open(&request.source).unwrap();
    let empty = GardenAccess::Allowlist { objects: vec![] };
    source
        .grant_with_garden_access(
            "graph-only",
            &["garden:read".into()],
            &ClientAccess::default(),
            &empty,
        )
        .unwrap();
    source
        .grant("write-only", &["memory:write".into()])
        .unwrap();
    drop(source);
    let prepared = migrate::prepare(request.clone()).unwrap();
    assert_eq!(
        prepared
            .restrictions
            .iter()
            .find(|grant| grant.client_id == "graph-only")
            .unwrap()
            .garden,
        Some(empty)
    );
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    for client in ["graph-only", "write-only"] {
        let config_path = request.journal.join(format!("{client}.json"));
        migrate::reissue(&request.journal, client, &config_path).unwrap();
        migrate::reissue(&request.journal, client, &config_path).unwrap();
        assert_eq!(
            migrate::reissue(
                &request.journal,
                client,
                &request.journal.join(format!("other-{client}.json"))
            )
            .unwrap_err()
            .code(),
            "OPERATION_REUSED"
        );
        let config = mother::garden::config::load_garden_config(&config_path).unwrap();
        let native = GardenStore::open(&request.destination).unwrap();
        assert_eq!(
            native
                .v1_authorize(&config.auth(), "read")
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
        if client == "write-only" {
            native.v1_authorize(&config.auth(), "memory:write").unwrap();
        } else {
            config.authorize(&native).unwrap();
        }
    }
    migrate::rollback(&request.journal, &request.operation_id).unwrap();
}

#[test]
fn lexical_source_alias_cannot_reopen_or_initialize_the_archived_endpoint() {
    let (_dir, request, _) = setup();
    let alias = request
        .source
        .parent()
        .unwrap()
        .join(".")
        .join(request.source.file_name().unwrap());
    migrate::prepare(request.clone()).unwrap();
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    assert_eq!(
        migrate::check_source(&alias).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(Store::open(&alias).unwrap_err().code(), "MIGRATION_FENCED");
    assert_eq!(
        Store::initialize(&alias, "Must not create another authority")
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    migrate::rollback(&request.journal, &request.operation_id).unwrap();
}

#[test]
fn unknown_source_extension_is_rejected_before_destination_publication() {
    let (_dir, request, _) = setup();
    let conn = rusqlite::Connection::open(&request.source).unwrap();
    conn.execute_batch("CREATE TABLE synthetic_unknown_extension(payload TEXT NOT NULL); INSERT INTO synthetic_unknown_extension VALUES ('synthetic required state')").unwrap();
    drop(conn);
    assert_eq!(
        migrate::prepare(request.clone()).unwrap_err().code(),
        "UNSUPPORTED_EXTENSION"
    );
    assert!(!request.destination.exists());
    assert!(Store::open(&request.source).is_ok());
}

#[test]
fn retrying_an_old_rollback_never_clears_a_newer_cutover_fence() {
    let (_dir, first, _) = setup();
    migrate::prepare(first.clone()).unwrap();
    migrate::activate(&first.journal, &first.operation_id).unwrap();
    migrate::rollback(&first.journal, &first.operation_id).unwrap();
    let second = MigrationRequest {
        source: first.source.clone(),
        destination: first.destination.with_file_name("second.redb"),
        journal: first.journal.with_file_name("second-cutover"),
        operation_id: Uuid::new_v4().to_string(),
    };
    migrate::prepare(second.clone()).unwrap();
    migrate::activate(&second.journal, &second.operation_id).unwrap();
    assert_eq!(
        migrate::rollback(&first.journal, &first.operation_id)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        migrate::check_source(&second.source).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert!(migrate::check_destination(&second.destination).is_ok());
    migrate::rollback(&second.journal, &second.operation_id).unwrap();
}

#[test]
fn missing_destination_cannot_be_treated_as_proof_of_no_acknowledged_writes() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    std::fs::remove_file(&request.destination).unwrap();
    assert_eq!(
        migrate::rollback(&request.journal, &request.operation_id)
            .unwrap_err()
            .code(),
        "RECONCILIATION_REQUIRED"
    );
    assert!(migrate::check_source(&request.source).is_err());
    assert_eq!(
        migrate::status(&request.journal).unwrap().phase,
        MigrationPhase::Activated
    );
}

#[test]
fn hardlinked_sources_are_rejected_before_preparation_and_at_later_admission() {
    let (_dir, request, _) = setup();
    let alias = request.source.with_file_name("source-alias.sqlite");
    std::fs::hard_link(&request.source, &alias).unwrap();
    assert_eq!(
        migrate::prepare(request.clone()).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(Store::open(&alias).unwrap_err().code(), "MIGRATION_FENCED");
    std::fs::remove_file(&alias).unwrap();
    migrate::prepare(request.clone()).unwrap();
    let existing = Store::open(&request.source).unwrap();
    std::fs::hard_link(&request.source, &alias).unwrap();
    assert_eq!(
        existing
            .access_for(&mother::contracts::Auth {
                client_id: "synthetic-client".into(),
                token: "0".repeat(64)
            })
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        migrate::activate(&request.journal, &request.operation_id)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    drop(existing);
    std::fs::remove_file(alias).unwrap();
    migrate::rollback(&request.journal, &request.operation_id).unwrap();
}

#[test]
fn hardlinked_prepared_and_rolled_back_destinations_reject_all_alias_admission() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let alias = request.destination.with_file_name("destination-alias.redb");
    std::fs::hard_link(&request.destination, &alias).unwrap();
    assert_eq!(
        migrate::check_destination(&alias).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        migrate::activate(&request.journal, &request.operation_id)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    std::fs::remove_file(&alias).unwrap();
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    migrate::rollback(&request.journal, &request.operation_id).unwrap();
    std::fs::hard_link(&request.destination, &alias).unwrap();
    assert_eq!(
        migrate::check_destination(&alias).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(
        migrate::check_destination(&request.destination)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
}

#[cfg(unix)]
#[test]
fn symlink_destination_alias_cannot_bypass_prepared_or_rollback_fences() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let alias = request
        .destination
        .with_file_name("destination-symlink.redb");
    std::os::unix::fs::symlink(&request.destination, &alias).unwrap();
    assert_eq!(
        migrate::check_destination(&alias).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
    migrate::activate(&request.journal, &request.operation_id).unwrap();
    migrate::rollback(&request.journal, &request.operation_id).unwrap();
    assert_eq!(
        migrate::check_destination(&alias).unwrap_err().code(),
        "MIGRATION_FENCED"
    );
}

#[cfg(unix)]
#[test]
fn permissive_existing_journal_is_rejected_before_private_state_is_read_or_written() {
    use std::os::unix::fs::PermissionsExt;
    let (_dir, request, _) = setup();
    std::fs::create_dir(&request.journal).unwrap();
    std::fs::set_permissions(&request.journal, std::fs::Permissions::from_mode(0o777)).unwrap();
    assert!(migrate::prepare(request.clone()).is_err());
    assert!(migrate::status(&request.journal).is_err());
    assert!(!request.journal.join("000000.json").exists());
    std::fs::set_permissions(&request.journal, std::fs::Permissions::from_mode(0o700)).unwrap();
    migrate::prepare(request.clone()).unwrap();
    std::fs::set_permissions(&request.journal, std::fs::Permissions::from_mode(0o777)).unwrap();
    assert!(migrate::status(&request.journal).is_err());
    assert!(migrate::activate(&request.journal, &request.operation_id).is_err());
}

#[cfg(any(unix, windows))]
#[test]
fn reissuance_rejects_broadly_readable_output_before_publishing_intent_or_credentials() {
    let (_dir, request, _) = setup();
    migrate::prepare(request.clone()).unwrap();
    let active = migrate::activate(&request.journal, &request.operation_id).unwrap();
    let output_directory = request.journal.parent().unwrap().join("broad-output");
    std::fs::create_dir(&output_directory).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&output_directory, std::fs::Permissions::from_mode(0o777))
            .unwrap();
    }
    #[cfg(windows)]
    {
        use windows_permissions::{
            LocalBox, SecurityDescriptor,
            constants::{SeObjectType, SecurityInformation},
        };
        let current = windows_permissions::utilities::current_process_sid().unwrap();
        // Only this owned synthetic directory receives the broad read ACE.
        let descriptor: LocalBox<SecurityDescriptor> =
            format!("D:P(A;;FA;;;{})(A;;FR;;;WD)", current.as_ref())
                .parse()
                .unwrap();
        windows_permissions::wrappers::SetNamedSecurityInfo(
            output_directory.as_os_str(),
            SeObjectType::SE_FILE_OBJECT,
            SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
            None,
            None,
            descriptor.dacl(),
            None,
        )
        .unwrap();
    }
    let output = output_directory.join("client.private.json");
    assert_eq!(
        migrate::reissue(&request.journal, "synthetic-client", &output)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert!(!output.exists());
    assert!(!std::fs::read_dir(&request.journal).unwrap().any(|entry| {
        entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("reissue-")
    }));
    assert_eq!(
        mother::config::check_private_output(&output)
            .unwrap_err()
            .code(),
        "MIGRATION_FENCED"
    );
    assert_eq!(migrate::status(&request.journal).unwrap(), active);
    migrate::rollback(&request.journal, &request.operation_id).unwrap();
}
