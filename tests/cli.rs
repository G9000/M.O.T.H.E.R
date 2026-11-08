use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::TempDir;

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", dir)
        .output()
        .expect("run native CLI")
}
fn succeeds(dir: &Path, args: &[&str]) -> Value {
    let result = run(dir, args);
    assert!(
        result.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).expect("CLI JSON")
}

#[test]
fn owner_can_initialize_grant_inspect_export_restore_and_revoke_without_leaking_credentials() {
    let dir = TempDir::new().unwrap();
    succeeds(dir.path(), &["init"]);
    assert!(!run(dir.path(), &["init"]).status.success());
    let grant = succeeds(
        dir.path(),
        &[
            "grant",
            "codex",
            "--scopes",
            "read,memory:write,task:write,checkpoint:write",
        ],
    );
    let config_path = dir.path().join("clients").join("codex.private.json");
    let config: Value = serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    let token = config["token"].as_str().unwrap();
    assert!(!grant.to_string().contains(token));
    let config_arg = config_path.to_str().unwrap();
    succeeds(dir.path(), &["context", "--config", config_arg]);
    let connection = succeeds(dir.path(), &["connection", "--config", config_arg]);
    assert!(!connection.to_string().contains(token));
    assert_eq!(
        connection["args"],
        serde_json::json!(["mcp", "--config", config_arg])
    );
    let toml = run(
        dir.path(),
        &["connection", "--config", config_arg, "--format", "codex"],
    );
    assert!(toml.status.success());
    assert!(!String::from_utf8_lossy(&toml.stdout).contains(token));
    assert!(String::from_utf8_lossy(&toml.stdout).contains("[mcp_servers.mother]"));
    assert!(!run(dir.path(), &["grant", "codex"]).status.success());
    let backup = dir.path().join("backup.private.json");
    let backup_arg = backup.to_str().unwrap();
    succeeds(dir.path(), &["export", "--out", backup_arg]);
    assert!(
        !run(dir.path(), &["export", "--out", backup_arg])
            .status
            .success()
    );
    assert!(!fs::read_to_string(&backup).unwrap().contains(token));
    let restored = dir.path().join("restored.sqlite");
    let restored_arg = restored.to_str().unwrap();
    succeeds(
        dir.path(),
        &["restore", "--from", backup_arg, "--db", restored_arg],
    );
    assert!(
        !run(
            dir.path(),
            &["restore", "--from", backup_arg, "--db", restored_arg]
        )
        .status
        .success()
    );
    let mut old_grant = config.clone();
    old_grant["databasePath"] = restored_arg.into();
    let old_path = dir.path().join("old-grant.private.json");
    fs::write(&old_path, serde_json::to_vec(&old_grant).unwrap()).unwrap();
    assert!(
        !run(
            dir.path(),
            &["context", "--config", old_path.to_str().unwrap()]
        )
        .status
        .success()
    );
    succeeds(dir.path(), &["revoke", "codex"]);
    assert!(
        !run(dir.path(), &["context", "--config", config_arg])
            .status
            .success()
    );
}

#[test]
fn invalid_cli_flags_and_unsafe_client_names_fail_before_mutation() {
    let dir = TempDir::new().unwrap();
    assert!(!run(dir.path(), &["init", "--typo"]).status.success());
    assert!(!dir.path().join("context.sqlite").exists());
    succeeds(dir.path(), &["init"]);
    assert!(!run(dir.path(), &["grant", "../escape"]).status.success());
    assert!(
        !run(dir.path(), &["grant", "writer", "--scopes", "read,admin"])
            .status
            .success()
    );
    assert!(
        !run(dir.path(), &["grant", "writer", "--scopes", "memory:write"])
            .status
            .success()
    );
    assert!(!dir.path().join("clients/writer.private.json").exists());
    assert!(
        !run(
            dir.path(),
            &["grant", "writer", "--scopes", "read", "--scopes", "read"]
        )
        .status
        .success()
    );
    assert!(run(dir.path(), &["help"]).status.success());
}
