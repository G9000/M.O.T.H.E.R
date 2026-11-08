use mother::{contracts::Auth, store::Store};
use serde_json::{Value, json};
#[test]
fn integral_json_numbers_replay_like_typescript_numbers() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("legacy.sqlite");
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute_batch(include_str!("fixtures/v1.sql")).unwrap();
    drop(db);
    let f: Value = serde_json::from_str(include_str!("fixtures/v1.json")).unwrap();
    let auth = Auth {
        client_id: f["auth"]["clientId"].as_str().unwrap().into(),
        token: f["auth"]["token"].as_str().unwrap().into(),
    };
    let mut s = Store::open(&path).unwrap();
    let mut q = f["requests"][1].clone();
    q["expectedRevision"] = json!(1.0);
    assert_eq!(s.write(&auth, q).unwrap(), f["receipts"][1]);
    assert_eq!(
        s.context(&auth, json!({"limit":1.0})).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let config = dir.path().join("client.private.json");
    std::fs::write(
        &config,
        json!({"version":1.0,"databasePath":path,"clientId":auth.client_id,"token":auth.token})
            .to_string(),
    )
    .unwrap();
    assert_eq!(mother::config::load_config(&config).unwrap().version, 1);
}
