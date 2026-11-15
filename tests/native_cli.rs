//! The shipped native executable must reject conversion and legacy credentials
//! before opening any legacy path, while keeping its owner command tree usable.
use serde_json::json;
use std::process::Command;

#[test]
fn native_help_and_legacy_refusals_never_initialize_a_store() {
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(args)
            .env("MOTHER_HOME", dir.path())
            .current_dir(dir.path())
            .output()
            .unwrap()
    };
    for args in [vec![], vec!["--help"], vec!["garden", "--help"]] {
        let output = run(&args);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    let db = dir.path().join("never-opened.sqlite");
    let config = dir.path().join("legacy.json");
    std::fs::write(
        &config,
        serde_json::to_vec(
            &json!({"version":1,"databasePath":db,"clientId":"synthetic","token":"a".repeat(64)}),
        )
        .unwrap(),
    )
    .unwrap();
    let rejected = run(&["mcp", "--config", config.to_str().unwrap()]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("LEGACY_UNAVAILABLE"));
    let rejected = run(&[
        "garden",
        "migrate",
        "prepare",
        "--source",
        db.to_str().unwrap(),
        "--db",
        dir.path().join("never-created.redb").to_str().unwrap(),
        "--journal",
        dir.path().join("never-created-journal").to_str().unwrap(),
        "--operation-id",
        &uuid::Uuid::new_v4().to_string(),
    ]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("unrecognized subcommand"));
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    assert!(!db.exists());
}
