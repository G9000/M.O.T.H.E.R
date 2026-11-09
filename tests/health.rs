use mother::{config::load_config, store::Store};
use serde_json::Value;
use std::{path::Path, process::Command};
use tempfile::TempDir;

fn fixture() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("context.sqlite");
    let config = dir.path().join("reader.private.json");
    let mut store = Store::initialize(&db, "SYNTHETIC PERSONA MUST NOT APPEAR").unwrap();
    store
        .grant_to_file("synthetic-reader", &["read".into()], &config, &db)
        .unwrap();
    drop(store);
    (dir, config)
}
fn doctor(config: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["doctor", "--config"])
        .arg(config)
        .arg("--json")
        .env(
            "MOTHER_MODEL_DIR",
            config
                .parent()
                .unwrap()
                .join("model-must-not-be-downloaded"),
        )
        .output()
        .unwrap()
}
#[test]
fn doctor_verifies_real_mcp_counts_and_keeps_canonical_state_and_secrets_private() {
    let (_dir, config) = fixture();
    let cfg = load_config(&config).unwrap();
    let clients_before = grant_snapshot(&cfg.database_path);
    let mut before = Store::open(&cfg.database_path).unwrap().export().unwrap();
    before.as_object_mut().unwrap().remove("exportedAt");
    let output = doctor(&config);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["readVerified"], true);
    assert_eq!(report["scope"], "LOCAL MCP read verified");
    assert_eq!(report["inventory"]["totalRecords"], 0);
    assert_eq!(report["inventory"]["byKind"]["memory"], 0);
    assert_eq!(report["inventory"]["recallableByKind"]["checkpoint"], 0);
    assert_eq!(report["serverVersion"], env!("CARGO_PKG_VERSION"));
    assert_eq!(report["executableSha256"].as_str().unwrap().len(), 64);
    let mut after = Store::open(&cfg.database_path).unwrap().export().unwrap();
    after.as_object_mut().unwrap().remove("exportedAt");
    assert_eq!(after, before);
    assert_eq!(grant_snapshot(&cfg.database_path), clients_before);
    assert!(
        !config
            .parent()
            .unwrap()
            .join("model-must-not-be-downloaded")
            .exists()
    );
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(!rendered.contains(&cfg.token));
    assert!(!rendered.contains("SYNTHETIC PERSONA"));
    assert!(!rendered.contains("records"));
}

fn grant_snapshot(db: &Path) -> Vec<(String, String, String, i64)> {
    let conn =
        rusqlite::Connection::open_with_flags(db, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .unwrap();
    conn.prepare("SELECT id, token_hash, scopes, revoked FROM clients ORDER BY id")
        .unwrap()
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}
#[test]
fn doctor_redacts_invalid_private_json_and_revoked_grants() {
    let (dir, config) = fixture();
    let cfg = load_config(&config).unwrap();
    Store::open(&cfg.database_path)
        .unwrap()
        .revoke(&cfg.client_id)
        .unwrap();
    let output = doctor(&config);
    assert!(!output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["readVerified"], false);
    assert!(!String::from_utf8_lossy(&output.stdout).contains(&cfg.token));
    let invalid = dir.path().join("invalid.json");
    std::fs::write(&invalid, r#"{"token":"SECRET_PRIVATE_JSON_NOT_AN_ERROR"}"#).unwrap();
    let output = doctor(&invalid);
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!combined.contains("SECRET_PRIVATE_JSON"));
    assert!(serde_json::from_slice::<Value>(&output.stdout).is_ok());
}
