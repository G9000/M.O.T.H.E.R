use mother::{
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
use uuid::Uuid;
fn fixture() -> (tempfile::TempDir, Store, Auth) {
    let dir = tempfile::Builder::new()
        .prefix("mother-context-counts-")
        .tempdir()
        .unwrap();
    let mut store = Store::initialize(
        &dir.path().join("context.sqlite"),
        "Synthetic context counts",
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
fn draft(kind: &str, status: &str, title: &str) -> Value {
    let mut r =
        json!({"kind":kind,"status":status,"title":title,"content":"Synthetic context only"});
    match kind {
        "memory" => r["basis"] = json!("owner_statement"),
        "checkpoint" => {
            r["nextStep"] = json!("An open checkpoint question is not an unsettled task")
        }
        "artifact" => r["mediaType"] = json!("text/plain"),
        "action" => r["destination"] = json!("synthetic evidence"),
        _ => {}
    };
    r
}
fn add(s: &mut Store, a: &Auth, record: Value) {
    let w = s.workspace().unwrap();
    s.write(a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})).unwrap();
}
fn zero() -> Value {
    json!({"memory":0,"decision":0,"task":0,"checkpoint":0,"artifact":0,"action":0})
}
#[test]
fn checkpoint_only_workspace_is_not_reported_as_empty_even_with_memory_filter() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, draft("checkpoint", "active", "Build handoff"));
    let c = s.context(&a, json!({"kind":"memory"})).unwrap();
    assert_eq!(c["inventory"]["totalRecords"], 1);
    assert_eq!(c["inventory"]["recallableRecords"], 1);
    assert_eq!(c["inventory"]["recallableByKind"]["checkpoint"], 1);
    assert_eq!(c["inventory"]["recallableByKind"]["memory"], 0);
    assert_eq!(c["selection"]["matchingRecords"], 0);
    assert_eq!(c["records"], json!([]));
    assert!(c["summary"].as_str().unwrap().contains("1 checkpoints"));
    assert!(
        c["note"]
            .as_str()
            .unwrap()
            .contains("not an empty workspace")
    );
    assert_eq!(c["unsettledWorkSummary"]["total"], 0);
}
#[test]
fn empty_workspace_reports_every_category_as_zero() {
    let (_dir, mut s, a) = fixture();
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(
        c["inventory"],
        json!({"totalRecords":0,"byKind":zero(),"recallableRecords":0,"recallableByKind":zero(),"excludedRecords":0})
    );
    assert_eq!(
        c["selection"],
        json!({"query":null,"kind":null,"matchingRecords":0,"returnedRecords":0,"omittedRecords":0})
    );
}
#[test]
fn counts_cover_all_categories_and_separate_historical_records() {
    let (_dir, mut s, a) = fixture();
    for (kind, state, hidden) in [
        ("memory", "active", "retracted"),
        ("decision", "confirmed", "superseded"),
        ("task", "done", "cancelled"),
        ("checkpoint", "active", "superseded"),
        ("artifact", "active", "retracted"),
        ("action", "succeeded", "failed"),
    ] {
        add(&mut s, &a, draft(kind, state, "Visible"));
        if kind != "action" {
            add(&mut s, &a, draft(kind, hidden, "Historical"));
        }
    }
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(c["inventory"]["totalRecords"], 11);
    assert_eq!(c["inventory"]["recallableRecords"], 6);
    assert_eq!(c["inventory"]["excludedRecords"], 5);
    for kind in [
        "memory",
        "decision",
        "task",
        "checkpoint",
        "artifact",
        "action",
    ] {
        assert_eq!(c["inventory"]["recallableByKind"][kind], 1);
        assert_eq!(
            c["inventory"]["byKind"][kind],
            if kind == "action" { 1 } else { 2 }
        );
    }
    assert_eq!(c["unsettledWorkSummary"]["total"], 0);
}
#[test]
fn query_and_limit_only_affect_selection_not_workspace_inventory() {
    let (_dir, mut s, a) = fixture();
    for title in ["Match one", "Match two", "Other"] {
        add(&mut s, &a, draft("memory", "active", title));
    }
    let c = s.context(&a, json!({"query":"Match","limit":1})).unwrap();
    assert_eq!(c["inventory"]["recallableRecords"], 3);
    assert_eq!(c["selection"]["matchingRecords"], 2);
    assert_eq!(c["selection"]["returnedRecords"], 1);
    assert_eq!(c["selection"]["omittedRecords"], 1);
    assert_eq!(c["omitted"], 1);
}
#[test]
fn unreviewed_proposals_are_not_counted_as_canonical_records() {
    let (_dir, mut s, a) = fixture();
    let w = s.workspace().unwrap();
    s.propose(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"source":"Synthetic owner capture","basis":"owner_statement","record":draft("memory","active","Pending")})).unwrap();
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(c["inventory"]["totalRecords"], 0);
    assert_eq!(c["records"], json!([]));
}
#[test]
fn unsettled_counts_are_global_and_report_truncation_explicitly() {
    let (_dir, mut s, a) = fixture();
    for _ in 0..22 {
        add(&mut s, &a, draft("task", "blocked", "Unsettled"));
    }
    add(&mut s, &a, draft("action", "unknown", "Unknown"));
    let c = s.context(&a, json!({"kind":"memory"})).unwrap();
    assert_eq!(c["unsettledWorkSummary"]["total"], 23);
    assert_eq!(c["unsettledWorkSummary"]["returned"], 20);
    assert_eq!(c["unsettledWorkSummary"]["omitted"], 3);
    assert_eq!(c["unsettledWorkTruncated"], true);
}
#[test]
fn authorization_is_checked_before_inventory_is_returned() {
    let (_dir, mut s, a) = fixture();
    s.revoke(&a.client_id).unwrap();
    assert_eq!(s.context(&a, json!({})).unwrap_err().code(), "UNAUTHORIZED");
}
