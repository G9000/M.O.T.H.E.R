//! Synthetic owner provisioning through the explicit graph command tree.
use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::TempDir;

fn run(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", home)
        .env("MOTHER_MODEL_DIR", home.join("models"))
        .output()
        .expect("run synthetic owner CLI")
}
fn success(home: &Path, args: &[&str]) -> Value {
    let out = run(home, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("owner JSON response")
}

#[test]
fn graph_index_is_selected_local_and_reports_missing_model_without_canonical_changes() {
    use mother::{
        access::ClientAccess,
        garden::{access::GardenAccess, contracts::BatchRequest, store::GardenStore},
    };
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("garden.redb");
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(&db, &ws, &generation).unwrap();
    let auth = store
        .grant(
            "selected-indexer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let batch:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"person","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":"Selected source","aliases":[],"extensions":{}}}]})).unwrap();
    let reference = store.apply(&auth, &batch).unwrap().receipt.results[0]
        .reference
        .clone();
    let config = dir.path().join("graph.private.json");
    fs::write(&config,serde_json::to_vec(&json!({"version":2,"backend":"garden_redb","databasePath":db,"clientId":auth.client_id,"token":auth.token})).unwrap()).unwrap();
    let selected = dir.path().join("selected.json");
    fs::write(&selected, serde_json::to_vec(&vec![reference]).unwrap()).unwrap();
    drop(store);
    let report = success(
        dir.path(),
        &[
            "garden",
            "index",
            "--config",
            config.to_str().unwrap(),
            "--objects",
            selected.to_str().unwrap(),
        ],
    );
    assert_eq!(report["prepared"], 0);
    assert_eq!(report["reason"], "local_model_unavailable");
    assert!(!report.to_string().contains(&auth.token));
    assert_eq!(
        GardenStore::open(&db)
            .unwrap()
            .audit(&auth)
            .unwrap()
            .sequence,
        1
    );
    assert!(!dir.path().join("context.sqlite").exists());
    assert!(!dir.path().join("models").exists());
}

#[test]
fn graph_owner_provisioning_is_explicit_private_and_does_not_initialize_v1() {
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("garden.redb");
    let config = dir.path().join("graph.private.json");
    let db_arg = db.to_str().unwrap();
    let out_arg = config.to_str().unwrap();
    let initialized = success(dir.path(), &["garden", "init", "--db", db_arg]);
    assert!(initialized["workspace"]["id"].as_str().is_some());
    assert!(!dir.path().join("context.sqlite").exists());
    assert!(
        !run(dir.path(), &["garden", "init", "--db", db_arg])
            .status
            .success()
    );
    let grant_args = [
        "garden",
        "grant",
        "synthetic",
        "--db",
        db_arg,
        "--access",
        "companion",
        "--subject",
        "synthetic-agent",
        "--out",
        out_arg,
    ];
    let granted = success(dir.path(), &grant_args);
    assert_eq!(granted["scopes"], json!(["garden:read", "garden:propose"]));
    let bytes = fs::read(&config).unwrap();
    let private: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(private["backend"], "garden_redb");
    assert_eq!(private["version"], 2);
    let token = private["token"].as_str().unwrap();
    assert!(!token.is_empty());
    assert!(!granted.to_string().contains(token));
    {
        use mother::{
            access::ClientAccess,
            garden::{
                access::GardenAccess, config::GardenClientConfig, contracts::*, store::GardenStore,
            },
        };
        let graph_config: GardenClientConfig = serde_json::from_value(private.clone()).unwrap();
        let store = GardenStore::open(&db).unwrap();
        let writer = store
            .grant(
                "synthetic-writer",
                &["garden:read".into(), "garden:write".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        let request: BatchRequest = serde_json::from_value(json!({"version":2,
            "workspaceId":initialized["workspace"]["id"],"generation":initialized["workspace"]["generation"],
            "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
            {"itemKey":"private","mutation":"create","expectedRevision":0,"draft":
            {"kind":"entity","entityType":"test:person","displayName":"Synthetic private object","aliases":[],"extensions":{}}}]})).unwrap();
        let made = store.apply(&writer, &request).unwrap();
        let get: GetRequest = serde_json::from_value(json!({"version":2,
            "workspaceId":request.workspace_id,"generation":request.generation,"requiredExtensions":{},
            "reference":made.receipt.results[0].reference,"revision":null,"knownAt":null})).unwrap();
        assert!(store.get(&writer, &get).is_ok());
        assert_eq!(
            store.get(&graph_config.auth(), &get).unwrap_err().code,
            GardenErrorCode::NotFound,
            "default companion grant must not imply full graph visibility"
        );
    }
    assert!(!run(dir.path(), &grant_args).status.success());
    assert_eq!(fs::read(&config).unwrap(), bytes);
    let revoked = success(
        dir.path(),
        &["garden", "revoke", "synthetic", "--db", db_arg],
    );
    assert_eq!(revoked["revoked"], true);
    assert!(!revoked.to_string().contains(token));
}

#[test]
fn graph_cli_rejects_shared_full_access_v1_scopes_and_implicit_paths() {
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("garden.redb");
    let config = dir.path().join("graph.private.json");
    let db_arg = db.to_str().unwrap();
    let out_arg = config.to_str().unwrap();
    success(dir.path(), &["garden", "init", "--db", db_arg]);
    for suffix in [
        vec!["--all"],
        vec!["--scopes", "read,context:propose"],
        vec!["--scopes", "garden:read,garden:write"],
    ] {
        let mut args = vec![
            "garden",
            "grant",
            "shared",
            "--db",
            db_arg,
            "--access",
            "shared",
            "--subject",
            "synthetic-agent",
            "--out",
            out_arg,
        ];
        args.extend(suffix);
        assert!(
            !run(dir.path(), &args).status.success(),
            "accepted {args:?}"
        );
        assert!(!config.exists());
    }
    assert!(!run(dir.path(), &["garden", "init"]).status.success());
    assert!(
        !run(
            dir.path(),
            &[
                "garden",
                "grant",
                "implicit",
                "--db",
                db_arg,
                "--access",
                "companion",
                "--subject",
                "agent"
            ]
        )
        .status
        .success()
    );
}

#[test]
fn graph_allowlists_are_typed_files_and_full_write_is_an_explicit_companion_choice() {
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("garden.redb");
    let db_arg = db.to_str().unwrap();
    success(dir.path(), &["garden", "init", "--db", db_arg]);
    let objects = dir.path().join("objects.json");
    fs::write(
        &objects,
        serde_json::to_vec(&json!([{"kind":"entity","id":uuid::Uuid::new_v4().to_string()}]))
            .unwrap(),
    )
    .unwrap();
    let config = dir.path().join("shared.private.json");
    success(
        dir.path(),
        &[
            "garden",
            "grant",
            "typed-shared",
            "--db",
            db_arg,
            "--access",
            "shared",
            "--subject",
            "test-agent",
            "--objects",
            objects.to_str().unwrap(),
            "--out",
            config.to_str().unwrap(),
        ],
    );
    let writer = dir.path().join("writer.private.json");
    success(
        dir.path(),
        &[
            "garden",
            "grant",
            "explicit-writer",
            "--db",
            db_arg,
            "--access",
            "companion",
            "--subject",
            "test-agent",
            "--scopes",
            "garden:read,garden:write",
            "--all",
            "--out",
            writer.to_str().unwrap(),
        ],
    );
    fs::write(&objects, b"[\"untyped-record-id\"]").unwrap();
    let rejected = dir.path().join("rejected.private.json");
    assert!(
        !run(
            dir.path(),
            &[
                "garden",
                "grant",
                "bad-allowlist",
                "--db",
                db_arg,
                "--access",
                "shared",
                "--subject",
                "test-agent",
                "--objects",
                objects.to_str().unwrap(),
                "--out",
                rejected.to_str().unwrap()
            ]
        )
        .status
        .success()
    );
    assert!(!rejected.exists());
}
