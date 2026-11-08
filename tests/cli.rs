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
        .env("MOTHER_MODEL_DIR", dir.join("models"))
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

fn initialize(dir: &Path) {
    fs::write(dir.join("SOUL.md"), "Synthetic CLI persona").unwrap();
    succeeds(dir, &["init"]);
}

#[test]
fn owner_can_initialize_grant_inspect_export_restore_and_revoke_without_leaking_credentials() {
    let dir = TempDir::new().unwrap();
    initialize(dir.path());
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
    initialize(dir.path());
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

#[test]
fn search_cli_and_model_status_work_without_downloading_a_model() {
    let dir = TempDir::new().unwrap();
    initialize(dir.path());
    succeeds(dir.path(), &["grant", "codex"]);
    let config = dir.path().join("clients/codex.private.json");
    let result = succeeds(
        dir.path(),
        &[
            "search",
            "--config",
            config.to_str().unwrap(),
            "--query",
            "unfinished work",
        ],
    );
    assert_eq!(result["search"]["effectiveMode"], "literal");
    assert_eq!(result["search"]["reason"], "model_unavailable");
    let status = succeeds(dir.path(), &["model", "status"]);
    assert_eq!(status["ready"], false);
}

#[test]
fn search_and_index_reject_invalid_options_and_revoked_grants() {
    let dir = TempDir::new().unwrap();
    initialize(dir.path());
    succeeds(dir.path(), &["grant", "reader"]);
    let config = dir.path().join("clients/reader.private.json");
    let config = config.to_str().unwrap();
    for (flag, value) in [
        ("--mode", "remote"),
        ("--limit", "0"),
        ("--limit", "101"),
        ("--kind", "unknown"),
    ] {
        assert!(
            !run(
                dir.path(),
                &["search", "--config", config, "--query", "tea", flag, value]
            )
            .status
            .success()
        );
    }
    for max_records in ["0", "101"] {
        assert!(
            !run(
                dir.path(),
                &["index", "--config", config, "--max-records", max_records]
            )
            .status
            .success()
        );
    }
    let literal = succeeds(
        dir.path(),
        &[
            "search", "--config", config, "--query", "tea", "--mode", "literal", "--kind",
            "memory", "--limit", "1",
        ],
    );
    assert_eq!(literal["search"]["requestedMode"], "literal");
    assert_eq!(literal["search"]["effectiveMode"], "literal");
    assert_eq!(literal["search"]["semanticUsed"], false);
    succeeds(dir.path(), &["revoke", "reader"]);
    for args in [
        vec!["search", "--config", config, "--query", "tea"],
        vec!["index", "--config", config],
    ] {
        let result = run(dir.path(), &args);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("UNAUTHORIZED"));
    }
    assert!(
        !dir.path().join("models").exists(),
        "search, index and status must not download model files"
    );
}

#[test]
fn invalid_model_directory_preserves_cli_context_and_search_but_rejects_model_commands() {
    let dir = TempDir::new().unwrap();
    initialize(dir.path());
    succeeds(dir.path(), &["grant", "reader"]);
    let config = dir.path().join("clients/reader.private.json");
    let config = config.to_str().unwrap();
    let run_relative = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .args(args)
            .env("MOTHER_HOME", dir.path())
            .env("MOTHER_MODEL_DIR", "relative-model")
            .current_dir(dir.path())
            .output()
            .unwrap()
    };
    let context = run_relative(&["context", "--config", config]);
    assert!(
        context.status.success(),
        "{}",
        String::from_utf8_lossy(&context.stderr)
    );
    let context: Value = serde_json::from_slice(&context.stdout).unwrap();
    for mode in ["literal", "hybrid", "semantic"] {
        let found = run_relative(&[
            "search", "--config", config, "--query", "tea", "--mode", mode,
        ]);
        assert!(
            found.status.success(),
            "{}",
            String::from_utf8_lossy(&found.stderr)
        );
        let found: Value = serde_json::from_slice(&found.stdout).unwrap();
        assert_eq!(found["workspace"], context["workspace"]);
        assert_eq!(found["search"]["requestedMode"], mode);
        assert_eq!(found["search"]["effectiveMode"], "literal");
        assert_eq!(found["search"]["semanticUsed"], false);
        if mode != "literal" {
            assert_eq!(found["search"]["reason"], "model_unavailable");
        }
    }
    for command in ["install", "status"] {
        let result = run_relative(&["model", command]);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("MODEL_UNAVAILABLE"));
    }
    assert!(!dir.path().join("relative-model").exists());
}

#[test]
fn initialization_requires_owner_soul_and_never_uses_the_working_directory_persona() {
    let dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();
    fs::write(
        cwd.path().join("SOUL.md"),
        "Unrelated repository personality",
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["init"])
        .env("MOTHER_HOME", dir.path())
        .current_dir(cwd.path())
        .output()
        .unwrap();
    assert!(
        !result.status.success(),
        "Missing workspace SOUL must not select a repository persona"
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("--soul"));
    assert!(!dir.path().join("context.sqlite").exists());
    let persona = "# My workspace\nUse my own preferences.\n";
    fs::write(dir.path().join("SOUL.md"), persona).unwrap();
    succeeds(dir.path(), &["init"]);
    let db = dir.path().join("context.sqlite");
    assert_eq!(
        mother::store::Store::open(&db).unwrap().persona().unwrap()["content"],
        persona
    );
    fs::write(
        dir.path().join("SOUL.md"),
        "Changed file after initialization",
    )
    .unwrap();
    assert_eq!(
        mother::store::Store::open(&db).unwrap().persona().unwrap()["content"],
        persona
    );
}

#[test]
fn explicit_soul_is_portable_and_invalid_sources_never_fall_back() {
    let dir = TempDir::new().unwrap();
    let source = TempDir::new().unwrap();
    let file = source.path().join("custom-persona.md");
    fs::write(
        dir.path().join("SOUL.md"),
        "Default that must not mask explicit errors",
    )
    .unwrap();
    let args = ["init", "--soul", file.to_str().unwrap()];
    assert!(!run(dir.path(), &args).status.success());
    assert!(!dir.path().join("context.sqlite").exists());
    for bytes in [
        vec![],
        vec![0xff],
        vec![b'x'; 128_001],
        b" \n\t".to_vec(),
        "\u{feff}".as_bytes().to_vec(),
        "\u{feff} \n\t".as_bytes().to_vec(),
    ] {
        fs::write(&file, bytes).unwrap();
        assert!(!run(dir.path(), &args).status.success());
        assert!(!dir.path().join("context.sqlite").exists());
    }
    let persona = "# A different companion\nKeep this user's language and preferences.\n";
    fs::write(&file, persona).unwrap();
    succeeds(dir.path(), &args);
    let export = dir.path().join("portable.private.json");
    succeeds(dir.path(), &["export", "--out", export.to_str().unwrap()]);
    let restored = dir.path().join("restored.sqlite");
    succeeds(
        dir.path(),
        &[
            "restore",
            "--from",
            export.to_str().unwrap(),
            "--db",
            restored.to_str().unwrap(),
        ],
    );
    assert_eq!(
        mother::store::Store::open(&restored)
            .unwrap()
            .persona()
            .unwrap()["content"],
        persona
    );
}

#[test]
fn default_soul_follows_an_explicit_database_location() {
    let home = TempDir::new().unwrap();
    let workspace = TempDir::new().unwrap();
    fs::write(home.path().join("SOUL.md"), "Unselected home persona").unwrap();
    fs::write(
        workspace.path().join("SOUL.md"),
        "Selected workspace persona",
    )
    .unwrap();
    let db = workspace.path().join("custom.sqlite");
    succeeds(home.path(), &["init", "--db", db.to_str().unwrap()]);
    assert_eq!(
        mother::store::Store::open(&db).unwrap().persona().unwrap()["content"],
        "Selected workspace persona"
    );
    assert!(!home.path().join("context.sqlite").exists());
}

#[test]
fn owner_updates_existing_soul_with_hash_precondition_and_preserves_workspace() {
    let dir = TempDir::new().unwrap();
    initialize(dir.path());
    succeeds(dir.path(), &["grant", "reader"]);
    let before = succeeds(dir.path(), &["soul", "show"]);
    let old_hash = before["persona"]["hash"].as_str().unwrap();
    let file = dir.path().join("edited.md");
    let file_arg = file.to_str().unwrap();
    let update = [
        "soul",
        "update",
        "--file",
        file_arg,
        "--expected-hash",
        old_hash,
    ];
    fs::write(&file, "# Updated companion\nUse concise replies.\n").unwrap();
    let result = succeeds(dir.path(), &update);
    assert_eq!(result["changed"], true);
    assert_eq!(result["previousHash"], old_hash);
    assert_eq!(result["workspace"], before["workspace"]);
    let after = succeeds(dir.path(), &["soul", "show"]);
    assert_eq!(
        after["persona"]["content"],
        fs::read_to_string(&file).unwrap()
    );
    assert_eq!(after["persona"]["hash"], result["hash"]);
    let stale = run(dir.path(), &update);
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("CONFLICT"));
    let new_hash = after["persona"]["hash"].as_str().unwrap();
    assert_eq!(
        succeeds(
            dir.path(),
            &[
                "soul",
                "update",
                "--file",
                file_arg,
                "--expected-hash",
                new_hash
            ]
        )["changed"],
        false
    );
    fs::write(&file, "\u{feff} \n").unwrap();
    assert!(
        !run(
            dir.path(),
            &[
                "soul",
                "update",
                "--file",
                file_arg,
                "--expected-hash",
                new_hash
            ]
        )
        .status
        .success()
    );
    assert_eq!(succeeds(dir.path(), &["soul", "show"]), after);
    let config = dir.path().join("clients/reader.private.json");
    let context = succeeds(
        dir.path(),
        &["context", "--config", config.to_str().unwrap()],
    );
    assert_eq!(context["persona"], after["persona"]);
    assert!(
        !run(dir.path(), &["soul", "update", "--file", file_arg])
            .status
            .success()
    );
}
