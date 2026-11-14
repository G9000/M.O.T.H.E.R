use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::Duration,
};

use garden_store::{Batch, Mutation, RedbStore, synthetic_assertion, synthetic_entity};

struct KillOnDrop(Child);
impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn cli_requires_synthetic_and_rejects_live_paths() {
    for args in [
        vec![],
        vec!["--synthetic", "--path", "owner.db"],
        vec!["--synthetic", "--records", "0"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_garden-store"))
            .args(args)
            .output()
            .unwrap();
        assert!(
            !output.status.success(),
            "unsafe/incomplete CLI arguments were accepted"
        );
        assert!(
            output.stdout.is_empty(),
            "errors must not masquerade as JSON benchmark output"
        );
    }
}

#[test]
fn actual_process_kill_preserves_all_or_none_of_each_transaction() {
    for stage_name in [
        "before-write",
        "staged",
        "before-commit",
        "committed",
        "commit-sync",
    ] {
        let root = tempfile::tempdir().unwrap();
        let nonce = format!("synthetic-{}-{stage_name}", std::process::id());
        std::fs::write(root.path().join("garden-synthetic-owner"), &nonce).unwrap();
        let mut child = KillOnDrop(
            Command::new(env!("CARGO_BIN_EXE_garden-store"))
                .args(["--internal-crash-child", stage_name])
                .env("GARDEN_SYNTHETIC_CHILD_ROOT", root.path())
                .env("GARDEN_SYNTHETIC_CHILD_NONCE", &nonce)
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap(),
        );
        let stdout = child.0.stdout.take().unwrap();
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let mut ready = String::new();
            let result = BufReader::new(stdout).read_line(&mut ready).map(|_| ready);
            let _ = sender.send(result);
        });
        let ready = receiver
            .recv_timeout(Duration::from_secs(15))
            .expect("child did not reach barrier within 15 seconds")
            .unwrap();
        assert_eq!(
            ready.trim(),
            format!("ready:{stage_name}"),
            "child must signal exact commit barrier"
        );
        child.0.kill().unwrap();
        assert!(
            !child.0.wait().unwrap().success(),
            "child must be terminated, not exit cleanly"
        );
        let store = RedbStore::open(root.path().join("crash.redb")).unwrap();
        let counts = store.verify_indexes().unwrap();
        let committed = if stage_name == "commit-sync" {
            counts.receipts == 1
        } else {
            stage_name == "committed"
        };
        assert_eq!(counts.entities, if committed { 32 } else { 0 });
        assert_eq!(counts.assertions, if committed { 32 } else { 0 });
        assert_eq!(counts.history, if committed { 32 } else { 0 });
        assert_eq!(counts.receipts, u64::from(committed));
        assert_eq!(counts.sequence, u64::from(committed));
        assert_eq!(store.receipt(1).unwrap().is_some(), committed);
        let batch = Batch {
            operation_id: 1,
            entities: (1..=32).map(synthetic_entity).collect(),
            assertions: (1..=32)
                .map(|id| Mutation {
                    assertion: synthetic_assertion(id, 0, 32, 1, 14),
                    expected_revision: 0,
                })
                .collect(),
        };
        store.apply(&batch).unwrap();
        let final_counts = store.verify_indexes().unwrap();
        assert_eq!(
            final_counts.entities, 32,
            "retry must not duplicate records"
        );
        assert_eq!(final_counts.assertions, 32);
        assert_eq!(final_counts.history, 32);
        assert_eq!(final_counts.receipts, 1);
    }
}
