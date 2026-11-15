//! Synthetic owner workflow; no installed settings, conversations or live stores.
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

fn run(home: &Path, args: &[&str]) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", home)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
#[test]
fn owner_can_enable_native_v1_back_up_verify_and_restore_without_installation() {
    let dir = tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let db = dir.path().join("garden.redb");
    let soul = dir.path().join("SOUL.md");
    fs::write(&soul, "Synthetic owner persona for qualification.").unwrap();
    run(
        dir.path(),
        &["garden", "init", "--db", db.to_str().unwrap()],
    );
    let workspace = run(
        dir.path(),
        &[
            "garden",
            "compat-init",
            "--db",
            db.to_str().unwrap(),
            "--soul",
            soul.to_str().unwrap(),
        ],
    );
    let config = dir.path().join("client.private.json");
    run(
        dir.path(),
        &[
            "garden",
            "grant-v1",
            "synthetic",
            "--db",
            db.to_str().unwrap(),
            "--out",
            config.to_str().unwrap(),
        ],
    );
    let credential: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
    assert_eq!(credential["backend"], "garden_redb");
    assert_eq!(workspace["schemaVersion"], 1);
    let small = dir.path().join("v1.json");
    run(
        dir.path(),
        &[
            "garden",
            "export-v1",
            "--db",
            db.to_str().unwrap(),
            "--out",
            small.to_str().unwrap(),
        ],
    );
    let bundle = dir.path().join("backup");
    run(
        dir.path(),
        &[
            "garden",
            "backup",
            "--db",
            db.to_str().unwrap(),
            "--out",
            bundle.to_str().unwrap(),
        ],
    );
    run(
        dir.path(),
        &["garden", "verify", "--backup", bundle.to_str().unwrap()],
    );
    let restored = dir.path().join("restored.redb");
    let result = run(
        dir.path(),
        &[
            "garden",
            "restore",
            "--backup",
            bundle.to_str().unwrap(),
            "--db",
            restored.to_str().unwrap(),
        ],
    );
    assert_eq!(result["workspace"]["id"], workspace["id"]);
    assert_ne!(result["workspace"]["generation"], workspace["generation"]);
    assert!(
        !result
            .to_string()
            .contains(credential["token"].as_str().unwrap())
    );
    let store = mother::garden::store::GardenStore::open(&restored).unwrap();
    let old = mother::contracts::Auth {
        client_id: "synthetic".into(),
        token: credential["token"].as_str().unwrap().into(),
    };
    assert_eq!(
        store
            .v1_call(&old, "mother_get_context", json!({}))
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
    assert!(!dir.path().join("context.sqlite").exists());
}
