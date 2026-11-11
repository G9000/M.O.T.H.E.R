//! An unfiltered context read keeps every record category visible within its preview limit.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
use std::{thread::sleep, time::Duration};
use uuid::Uuid;
fn fixture() -> (tempfile::TempDir, Store, Auth) {
    let dir = tempfile::Builder::new()
        .prefix("mother-balanced-context-")
        .tempdir()
        .unwrap();
    let mut store = Store::initialize(
        &dir.path().join("context.sqlite"),
        "Synthetic balanced context",
    )
    .unwrap();
    let grant = store
        .grant("synthetic", &SCOPES.map(str::to_owned))
        .unwrap();
    let auth = Auth {
        client_id: "synthetic".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    (dir, store, auth)
}
/// Save one synthetic record and return its ID. Update times have millisecond
/// resolution, so a short pause keeps "newer" unambiguous.
fn add(s: &mut Store, a: &Auth, kind: &str, status: &str, title: &str) -> String {
    let mut record =
        json!({"kind":kind,"status":status,"title":title,"content":"Synthetic context only"});
    match kind {
        "memory" => record["basis"] = json!("owner_statement"),
        "checkpoint" => record["nextStep"] = json!("Synthetic next step"),
        "artifact" => record["mediaType"] = json!("text/plain"),
        "action" => record["destination"] = json!("synthetic evidence"),
        _ => {}
    }
    let w = s.workspace().unwrap();
    let receipt = s.write(a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})).unwrap();
    sleep(Duration::from_millis(2));
    receipt["record"]["id"].as_str().unwrap().to_owned()
}
fn titles(context: &Value) -> Vec<&str> {
    context["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["title"].as_str().unwrap())
        .collect()
}
#[test]
fn many_recent_checkpoints_do_not_crowd_out_older_memories_and_decisions() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "active", "Tea preference");
    add(&mut s, &a, "memory", "active", "Preferred name");
    add(&mut s, &a, "decision", "confirmed", "Keep data local");
    for n in 0..30 {
        add(&mut s, &a, "checkpoint", "active", &format!("Session {n}"));
    }
    let c = s.context(&a, json!({})).unwrap();
    let shown = titles(&c);
    assert_eq!(shown.len(), 20);
    for expected in ["Tea preference", "Preferred name", "Keep data local"] {
        assert!(shown.contains(&expected), "{expected} was crowded out");
    }
    assert_eq!(c["selection"]["order"], "balanced");
    assert_eq!(c["selection"]["matchingRecords"], 33);
    assert_eq!(c["selection"]["returnedRecords"], 20);
    assert_eq!(c["selection"]["omittedRecords"], 13);
    assert_eq!(
        c["selection"]["returnedByKind"],
        json!({"memory":2,"decision":1,"task":0,"checkpoint":17,"artifact":0,"action":0})
    );
    assert_eq!(
        c["selection"]["omittedByKind"],
        json!({"memory":0,"decision":0,"task":0,"checkpoint":13,"artifact":0,"action":0})
    );
    assert_eq!(c["omitted"], 13);
    assert!(c["summary"].as_str().unwrap().contains("take turns"));
    // The checkpoints that remain are the newest ones.
    assert!(shown.contains(&"Session 29"));
    assert!(!shown.contains(&"Session 0"));
}
#[test]
fn balanced_previews_take_the_newest_of_each_category_and_stay_newest_first() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "active", "Older memory");
    add(&mut s, &a, "memory", "active", "Newer memory");
    add(&mut s, &a, "checkpoint", "active", "Older checkpoint");
    add(&mut s, &a, "checkpoint", "active", "Newer checkpoint");
    let c = s.context(&a, json!({"limit":2})).unwrap();
    assert_eq!(titles(&c), ["Newer checkpoint", "Newer memory"]);
    let c = s.context(&a, json!({"limit":3})).unwrap();
    assert_eq!(
        titles(&c),
        ["Newer checkpoint", "Older checkpoint", "Newer memory"]
    );
}
#[test]
fn unsettled_tasks_and_actions_are_previewed_before_settled_ones() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "task", "blocked", "Waiting on a reply");
    add(&mut s, &a, "action", "unknown", "Outcome not yet known");
    for n in 0..5 {
        add(&mut s, &a, "task", "done", &format!("Finished task {n}"));
        add(
            &mut s,
            &a,
            "action",
            "succeeded",
            &format!("Finished action {n}"),
        );
    }
    let c = s.context(&a, json!({"limit":2})).unwrap();
    let mut shown = titles(&c);
    shown.sort_unstable();
    assert_eq!(shown, ["Outcome not yet known", "Waiting on a reply"]);
}
#[test]
fn a_workspace_within_the_limit_is_unchanged_and_fully_shown() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "active", "First");
    add(&mut s, &a, "checkpoint", "active", "Second");
    add(&mut s, &a, "task", "done", "Third");
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(titles(&c), ["Third", "Second", "First"]);
    assert_eq!(c["selection"]["omittedRecords"], 0);
    assert_eq!(
        c["selection"]["omittedByKind"],
        json!({"memory":0,"decision":0,"task":0,"checkpoint":0,"artifact":0,"action":0})
    );
}
#[test]
fn query_and_kind_filters_keep_newest_first_matching() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "active", "Tea preference");
    for n in 0..25 {
        add(&mut s, &a, "checkpoint", "active", &format!("Session {n}"));
    }
    let c = s.context(&a, json!({"query":"Synthetic"})).unwrap();
    assert_eq!(c["selection"]["order"], "recent");
    assert!(
        !titles(&c).contains(&"Tea preference"),
        "a query lists its newest matches, whatever their category"
    );
    assert_eq!(c["selection"]["returnedByKind"]["checkpoint"], 20);
    assert_eq!(c["selection"]["omittedByKind"]["checkpoint"], 5);
    assert_eq!(c["selection"]["omittedByKind"]["memory"], 1);
    let c = s
        .context(&a, json!({"kind":"checkpoint","limit":3}))
        .unwrap();
    assert_eq!(c["selection"]["order"], "recent");
    assert_eq!(titles(&c), ["Session 24", "Session 23", "Session 22"]);
}
#[test]
fn historical_records_never_take_a_preview_slot() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "retracted", "Withdrawn memory");
    add(&mut s, &a, "decision", "superseded", "Replaced decision");
    add(&mut s, &a, "task", "cancelled", "Dropped task");
    add(&mut s, &a, "checkpoint", "active", "Current checkpoint");
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(titles(&c), ["Current checkpoint"]);
    assert_eq!(c["selection"]["matchingRecords"], 1);
}
#[test]
fn shared_clients_are_balanced_within_their_granted_records_only() {
    let (_dir, mut s, a) = fixture();
    let granted_memory = add(&mut s, &a, "memory", "active", "Granted memory");
    add(&mut s, &a, "decision", "confirmed", "Hidden decision");
    let mut granted = vec![granted_memory];
    for n in 0..4 {
        granted.push(add(
            &mut s,
            &a,
            "checkpoint",
            "active",
            &format!("Granted {n}"),
        ));
        add(&mut s, &a, "checkpoint", "active", &format!("Hidden {n}"));
    }
    let grant = s
        .grant_with_access(
            "shared",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: granted,
            },
        )
        .unwrap();
    let shared = Auth {
        client_id: "shared".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    let c = s.context(&shared, json!({"limit":3})).unwrap();
    let shown = titles(&c);
    assert!(shown.contains(&"Granted memory"));
    assert!(shown.iter().all(|title| title.starts_with("Granted")));
    assert_eq!(
        c["selection"]["returnedByKind"],
        json!({"memory":1,"decision":0,"task":0,"checkpoint":2,"artifact":0,"action":0})
    );
    assert_eq!(
        c["selection"]["omittedByKind"],
        json!({"memory":0,"decision":0,"task":0,"checkpoint":2,"artifact":0,"action":0})
    );
}
