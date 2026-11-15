//! Shipped artifact and runtime boundaries for the single native backend.
use serde_json::json;
use std::process::Command;

#[test]
fn manifest_has_no_sqlite_backend_or_feature() {
    let manifest = include_str!("../Cargo.toml");
    for obsolete in ["legacy-sqlite", "rusqlite", "libsqlite3-sys"] {
        assert!(
            !manifest.contains(obsolete),
            "obsolete backend remains: {obsolete}"
        );
    }
}

#[test]
fn default_cli_is_native_and_rejects_old_config_without_touching_its_database() {
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(args)
            .env("MOTHER_HOME", dir.path())
            .current_dir(dir.path())
            .output()
            .unwrap()
    };
    let help = run(&["--help"]);
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("garden"));
    assert!(
        !help.contains("setup"),
        "obsolete setup entry point remains"
    );
    assert!(!help.contains("tend"), "obsolete TUI entry point remains");
    let empty = run(&[]);
    assert!(empty.status.success(), "default CLI must show native help");
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);

    let database = dir.path().join("old.sqlite");
    let original = b"SQLite format 3\0synthetic preserved fixture";
    std::fs::write(&database, original).unwrap();
    let config = dir.path().join("legacy.json");
    std::fs::write(
        &config,
        serde_json::to_vec(&json!({"version":1,"databasePath":database,
            "clientId":"synthetic","token":"a".repeat(64)}))
        .unwrap(),
    )
    .unwrap();
    let rejected = run(&["mcp", "--config", config.to_str().unwrap()]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("LEGACY_UNAVAILABLE"));
    assert_eq!(std::fs::read(&database).unwrap(), original);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
}
