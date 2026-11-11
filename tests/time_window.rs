//! Context can be limited to records last updated within a time window.
use mother::{
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
use std::{thread::sleep, time::Duration};
use uuid::Uuid;
fn fixture() -> (tempfile::TempDir, Store, Auth) {
    let dir = tempfile::Builder::new()
        .prefix("mother-time-window-")
        .tempdir()
        .unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic time window").unwrap();
    let grant = store
        .grant("synthetic", &SCOPES.map(str::to_owned))
        .unwrap();
    let auth = Auth {
        client_id: "synthetic".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    (dir, store, auth)
}
fn add(s: &mut Store, a: &Auth, kind: &str, title: &str) -> Value {
    let mut record =
        json!({"kind":kind,"status":"active","title":title,"content":"Synthetic context only"});
    match kind {
        "memory" => record["basis"] = json!("owner_statement"),
        "checkpoint" => record["nextStep"] = json!("Synthetic next step"),
        _ => {}
    }
    let w = s.workspace().unwrap();
    let saved = s.write(a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})).unwrap()["record"].clone();
    sleep(Duration::from_millis(5));
    saved
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
fn since_is_inclusive_and_before_is_exclusive() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "First");
    let second = add(&mut s, &a, "memory", "Second");
    let third = add(&mut s, &a, "memory", "Third");
    let since = &second["updatedAt"];
    let before = &third["updatedAt"];
    let c = s.context(&a, json!({"updatedSince":since})).unwrap();
    assert_eq!(titles(&c), ["Third", "Second"]);
    let c = s.context(&a, json!({"updatedBefore":before})).unwrap();
    assert_eq!(titles(&c), ["Second", "First"]);
    let c = s
        .context(&a, json!({"updatedSince":since,"updatedBefore":before}))
        .unwrap();
    assert_eq!(titles(&c), ["Second"]);
    assert_eq!(c["selection"]["updatedSince"], *since);
    assert_eq!(c["selection"]["updatedBefore"], *before);
    assert_eq!(c["selection"]["matchingRecords"], 1);
    assert_eq!(c["selection"]["order"], "recent");
    assert_eq!(
        c["inventory"]["recallableRecords"], 3,
        "the window narrows previews, not the workspace inventory"
    );
}
#[test]
fn a_window_combines_with_kind_and_query() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "Old tea");
    let start = add(&mut s, &a, "checkpoint", "Recent handoff");
    add(&mut s, &a, "memory", "Recent tea");
    add(&mut s, &a, "memory", "Recent coffee");
    let since = &start["updatedAt"];
    let c = s
        .context(&a, json!({"updatedSince":since,"kind":"memory"}))
        .unwrap();
    assert_eq!(titles(&c), ["Recent coffee", "Recent tea"]);
    let c = s
        .context(&a, json!({"updatedSince":since,"query":"tea"}))
        .unwrap();
    assert_eq!(titles(&c), ["Recent tea"]);
}
#[test]
fn dates_offsets_and_extra_precision_are_normalized_to_stored_utc_milliseconds() {
    let (_dir, mut s, a) = fixture();
    for (given, normalized) in [
        ("2026-11-11", "2026-11-11T00:00:00.000Z"),
        ("2026-11-11T09:30:00Z", "2026-11-11T09:30:00.000Z"),
        ("2026-11-11T11:30:00+02:00", "2026-11-11T09:30:00.000Z"),
        ("2026-11-11T09:30:00.12Z", "2026-11-11T09:30:00.120Z"),
        // Stored times have whole milliseconds, so rounding a bound up keeps
        // "at or after" and "before" exact.
        ("2026-11-11T09:30:00.1201Z", "2026-11-11T09:30:00.121Z"),
    ] {
        let c = s.context(&a, json!({"updatedSince":given})).unwrap();
        assert_eq!(c["selection"]["updatedSince"], normalized, "{given}");
        let c = s.context(&a, json!({"updatedBefore":given})).unwrap();
        assert_eq!(c["selection"]["updatedBefore"], normalized, "{given}");
    }
}
#[test]
fn records_outside_a_future_or_past_window_are_not_previewed() {
    let (_dir, mut s, a) = fixture();
    add(&mut s, &a, "memory", "Now");
    let c = s.context(&a, json!({"updatedSince":"2999-01-01"})).unwrap();
    assert_eq!(c["selection"]["matchingRecords"], 0);
    assert_eq!(c["records"], json!([]));
    let c = s
        .context(&a, json!({"updatedBefore":"2000-01-01"}))
        .unwrap();
    assert_eq!(c["selection"]["matchingRecords"], 0);
}
#[test]
fn malformed_or_empty_windows_are_rejected() {
    let (_dir, mut s, a) = fixture();
    for bad in [
        json!({"updatedSince":"yesterday"}),
        json!({"updatedSince":""}),
        json!({"updatedSince":1_700_000_000}),
        json!({"updatedBefore":"2026-13-01"}),
        json!({"updatedSince":"2026-11-11T09:30:00"}),
        json!({"updatedSince":"2026-11-12","updatedBefore":"2026-11-11"}),
        json!({"updatedSince":"2026-11-11","updatedBefore":"2026-11-11"}),
    ] {
        assert_eq!(
            s.context(&a, bad.clone()).unwrap_err().code(),
            "INVALID_INPUT",
            "{bad}"
        );
    }
}
