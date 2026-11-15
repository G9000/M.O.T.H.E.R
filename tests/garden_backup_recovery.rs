//! Process-kill qualification of atomic fresh-file publication with synthetic data.
use mother::{
    access::ClientAccess,
    garden::{
        access::GardenAccess, backup::BackupRestoreStage, contracts::BatchRequest,
        store::GardenStore,
    },
};
use serde_json::json;
use std::{
    fs,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn killed_restore_never_publishes_partial_or_credentialed_state() {
    for stage in [
        "validated",
        "staged",
        "before_commit",
        "committed",
        "before_publish",
        "published",
    ] {
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let root = dir.path().canonicalize().unwrap();
        let nonce = uuid::Uuid::new_v4().to_string();
        fs::write(root.join("owned"), &nonce).unwrap();
        let ws = uuid::Uuid::new_v4().to_string();
        let generation = uuid::Uuid::new_v4().to_string();
        let store = GardenStore::initialize(root.join("source.redb"), &ws, &generation).unwrap();
        store.v1_initialize("Synthetic owner only.").unwrap();
        let auth = store
            .v1_grant(
                "synthetic",
                &[
                    "read".into(),
                    "memory:write".into(),
                    "garden:read".into(),
                    "garden:write".into(),
                ],
                &ClientAccess::default(),
                Some(&GardenAccess::All {}),
            )
            .unwrap();
        let original=store.v1_call(&auth,"mother_write_record",json!({"workspaceId":ws,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Kept","content":"Synthetic durable context","status":"active","basis":"owner_statement","references":[],"sources":[]}})).unwrap();
        let request:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"e","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":"Synthetic","aliases":[],"extensions":{}}}]})).unwrap();
        store.apply(&auth, &request).unwrap();
        store.backup_to(&root.join("bundle")).unwrap();
        drop(store);
        let mut child = ChildGuard(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "backup_restore_child",
                    "--ignored",
                    "--nocapture",
                ])
                .env("MOTHER_BACKUP_TEST_ROOT", &root)
                .env("MOTHER_BACKUP_TEST_NONCE", &nonce)
                .env("MOTHER_BACKUP_TEST_STAGE", stage)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap(),
        );
        let deadline = Instant::now() + Duration::from_secs(30);
        while !root.join("ready").exists() {
            assert!(
                child.0.try_wait().unwrap().is_none(),
                "child exited before {stage}"
            );
            assert!(Instant::now() < deadline, "child never reached {stage}");
            std::thread::sleep(Duration::from_millis(10));
        }
        child.0.kill().unwrap();
        child.0.wait().unwrap();
        let destination = root.join("restored.redb");
        assert_eq!(destination.exists(), stage == "published", "{stage}");
        let restored = if destination.exists() {
            GardenStore::open(&destination).unwrap()
        } else {
            GardenStore::restore_backup(&root.join("bundle"), &destination).unwrap()
        };
        assert!(restored.authorize(&auth).is_err());
        let fresh = restored
            .v1_grant(
                "synthetic",
                &["read".into(), "garden:read".into()],
                &ClientAccess::default(),
                Some(&GardenAccess::All {}),
            )
            .unwrap();
        assert_eq!(restored.audit(&fresh).unwrap().sequence, 1);
        let retained = restored
            .v1_call(
                &fresh,
                "mother_get_record",
                json!({"id":original["record"]["id"]}),
            )
            .unwrap();
        assert_eq!(retained, original["record"]);
        let source = GardenStore::open(root.join("source.redb")).unwrap();
        assert_eq!(source.audit(&auth).unwrap().sequence, 1);
        let current = restored.v1_identity(&fresh).unwrap();
        assert_ne!(current["generation"], generation);
    }
}

#[test]
#[ignore = "private process-kill child; only the parent supplies a verified synthetic root"]
fn backup_restore_child() {
    let root =
        PathBuf::from(std::env::var_os("MOTHER_BACKUP_TEST_ROOT").expect("parent-owned root"))
            .canonicalize()
            .unwrap();
    assert!(root.starts_with(std::env::temp_dir().canonicalize().unwrap()));
    let nonce = std::env::var("MOTHER_BACKUP_TEST_NONCE").unwrap();
    uuid::Uuid::parse_str(&nonce).unwrap();
    assert_eq!(fs::read_to_string(root.join("owned")).unwrap(), nonce);
    let stage = std::env::var("MOTHER_BACKUP_TEST_STAGE").unwrap();
    GardenStore::restore_backup_with_hook(
        &root.join("bundle"),
        &root.join("restored.redb"),
        |point| {
            let name = match point {
                BackupRestoreStage::Validated => "validated",
                BackupRestoreStage::Staged => "staged",
                BackupRestoreStage::BeforeCommit => "before_commit",
                BackupRestoreStage::Committed => "committed",
                BackupRestoreStage::BeforePublish => "before_publish",
                BackupRestoreStage::Published => "published",
            };
            if name == stage {
                fs::write(root.join("ready"), b"ready").unwrap();
                loop {
                    std::thread::park();
                }
            }
        },
    )
    .unwrap();
}
