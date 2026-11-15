//! Existing cutover journals remain authoritative without a SQLite converter.
use mother::garden::{
    migrate::{self, MigrationPhase, MigrationRequest, MigrationStatus},
    store::GardenStore,
};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

struct Fixture {
    _root: tempfile::TempDir,
    state: MigrationStatus,
}
impl Fixture {
    fn new(phase: MigrationPhase) -> Self {
        let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let journal = root.path().join("journal");
        fs::create_dir(&journal).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&journal, fs::Permissions::from_mode(0o700)).unwrap();
        }
        let destination = root.path().join("garden.redb");
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        drop(GardenStore::initialize(&destination, &workspace, &generation).unwrap());
        let state = MigrationStatus {
            version: 1,
            revision: 0,
            request: MigrationRequest {
                source: root.path().join("archived.sqlite"),
                destination,
                journal,
                operation_id: Uuid::new_v4().to_string(),
            },
            phase,
            workspace_id: workspace,
            source_generation: Uuid::new_v4().to_string(),
            destination_generation: Some(generation),
            source_fingerprint: "synthetic retained source fingerprint".into(),
            destination_fingerprint: None,
            source_archive_fingerprint: None,
            archive: root.path().join("archive.sqlite"),
            source_readonly: false,
            restrictions: vec![],
        };
        let fixture = Self { _root: root, state };
        fixture.write_state();
        fs::write(fixture.marker(),serde_json::to_vec(&json!({"version":1,"journal":fixture.state.request.journal,"operationId":fixture.state.request.operation_id,"role":"destination"})).unwrap()).unwrap();
        fixture
    }
    fn marker(&self) -> PathBuf {
        let mut marker = self.state.request.destination.as_os_str().to_os_string();
        marker.push(".mother-cutover");
        marker.into()
    }
    fn write_state(&self) {
        fs::write(
            self.state
                .request
                .journal
                .join(format!("{:06}.json", self.state.revision)),
            serde_json::to_vec(&self.state).unwrap(),
        )
        .unwrap();
    }
    fn assert_fenced(&self, path: &Path) {
        let error = match GardenStore::open_active(path) {
            Ok(_) => panic!("ordinary native open must honor the cutover fence"),
            Err(error) => error,
        };
        assert_eq!(error.code(), "MIGRATION_FENCED");
    }
}

#[test]
fn prepared_and_rolled_back_native_destinations_remain_fenced_without_modifying_bytes() {
    for phase in [
        MigrationPhase::Prepared,
        MigrationPhase::RolledBack,
        MigrationPhase::NeedsReconciliation,
    ] {
        let fixture = Fixture::new(phase);
        let path = &fixture.state.request.destination;
        let before = fs::read(path).unwrap();
        fixture.assert_fenced(path);
        assert_eq!(fs::read(path).unwrap(), before);
        assert_eq!(
            migrate::status(&fixture.state.request.journal)
                .unwrap()
                .phase,
            phase
        );
    }
}

#[test]
fn only_an_activated_matching_cutover_identity_admits_the_native_destination() {
    let mut fixture = Fixture::new(MigrationPhase::Activated);
    drop(GardenStore::open_active(&fixture.state.request.destination).unwrap());
    fixture.state.request.operation_id = Uuid::new_v4().to_string();
    fixture.write_state();
    fixture.assert_fenced(&fixture.state.request.destination);
}

#[test]
fn missing_malformed_or_gapped_existing_cutover_journals_fail_closed() {
    for damage in ["missing", "malformed", "gap"] {
        let fixture = Fixture::new(MigrationPhase::Activated);
        let transition = fixture.state.request.journal.join("000000.json");
        match damage {
            "missing" => fs::remove_file(transition).unwrap(),
            "malformed" => fs::write(transition, b"{malformed synthetic journal").unwrap(),
            "gap" => fs::rename(
                transition,
                fixture.state.request.journal.join("000001.json"),
            )
            .unwrap(),
            _ => unreachable!(),
        }
        fixture.assert_fenced(&fixture.state.request.destination);
    }
}

#[test]
fn hardlinked_native_destination_alias_cannot_bypass_its_cutover_marker() {
    let fixture = Fixture::new(MigrationPhase::Prepared);
    let alias = fixture
        .state
        .request
        .destination
        .with_file_name("alias.redb");
    fs::hard_link(&fixture.state.request.destination, &alias).unwrap();
    fixture.assert_fenced(&alias);
    fixture.assert_fenced(&fixture.state.request.destination);
}
