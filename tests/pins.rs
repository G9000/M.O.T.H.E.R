//! Pinned core memories are always part of context, outside the preview limit.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
use uuid::Uuid;
fn fixture() -> (tempfile::TempDir, Store, Auth) {
    let dir = tempfile::Builder::new()
        .prefix("mother-pins-")
        .tempdir()
        .unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic pins").unwrap();
    let auth = grant(&mut store, "synthetic", &SCOPES.map(str::to_owned));
    (dir, store, auth)
}
fn grant(s: &mut Store, name: &str, scopes: &[String]) -> Auth {
    let g = s.grant(name, scopes).unwrap();
    Auth {
        client_id: name.into(),
        token: g["token"].as_str().unwrap().into(),
    }
}
fn draft(kind: &str, title: &str, content: &str) -> Value {
    let mut r = json!({"kind":kind,"title":title,"content":content,"status":"active"});
    match kind {
        "memory" => r["basis"] = json!("owner_statement"),
        "decision" => r["status"] = json!("confirmed"),
        "checkpoint" => r["nextStep"] = json!("Synthetic next step"),
        _ => {}
    }
    r
}
fn save(s: &mut Store, a: &Auth, record: Value, previous: Option<&Value>) -> Value {
    let w = s.workspace().unwrap();
    let mut q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record});
    if let Some(p) = previous {
        q["id"] = p["id"].clone();
        q["expectedRevision"] = p["revision"].clone();
    }
    s.write(a, q).unwrap()["record"].clone()
}
fn pin(s: &mut Store, a: &Auth, record: &Value, pinned: bool) -> Result<Value, String> {
    s.pin(a, json!({"id":record["id"],"pinned":pinned}))
        .map_err(|e| e.code().to_owned())
}
fn pinned_titles(c: &Value) -> Vec<&str> {
    c["pinned"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["title"].as_str().unwrap())
        .collect()
}
#[test]
fn a_pinned_memory_stays_in_context_when_newer_records_crowd_it_out() {
    let (_dir, mut s, a) = fixture();
    let name = save(
        &mut s,
        &a,
        draft("memory", "Preferred name", "Call me Leo"),
        None,
    );
    for n in 0..30 {
        save(
            &mut s,
            &a,
            draft("memory", &format!("Note {n}"), "Synthetic"),
            None,
        );
    }
    let result = pin(&mut s, &a, &name, true).unwrap();
    assert_eq!(result["pinned"], true);
    assert_eq!(result["id"], name["id"]);
    let c = s.context(&a, json!({"limit":5})).unwrap();
    let previews: Vec<&str> = c["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["title"].as_str().unwrap())
        .collect();
    assert!(!previews.contains(&"Preferred name"));
    assert_eq!(pinned_titles(&c), ["Preferred name"]);
    let entry = &c["pinned"][0];
    assert_eq!(entry["content"], "Call me Leo");
    assert_eq!(entry["truncated"], false);
    assert_eq!(entry["pinnedBy"], "synthetic");
    assert!(entry["pinnedAt"].as_str().unwrap().ends_with('Z'));
    assert_eq!(
        c["pinnedSummary"],
        json!({"total":1,"shown":1,"inactive":0,"limit":12})
    );
    // Filters narrow previews, never the pinned core.
    let c = s.context(&a, json!({"kind":"checkpoint"})).unwrap();
    assert_eq!(pinned_titles(&c), ["Preferred name"]);
}
#[test]
fn pinning_is_idempotent_and_unpinning_removes_it() {
    let (_dir, mut s, a) = fixture();
    let m = save(&mut s, &a, draft("memory", "Tea", "Green tea"), None);
    let first = pin(&mut s, &a, &m, true).unwrap();
    let again = pin(&mut s, &a, &m, true).unwrap();
    assert_eq!(again["pinnedAt"], first["pinnedAt"]);
    assert_eq!(again["changed"], false);
    assert_eq!(first["changed"], true);
    let off = pin(&mut s, &a, &m, false).unwrap();
    assert_eq!(off["pinned"], false);
    assert_eq!(off["changed"], true);
    assert_eq!(pin(&mut s, &a, &m, false).unwrap()["changed"], false);
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(c["pinned"], json!([]));
    assert_eq!(c["pinnedSummary"]["total"], 0);
}
#[test]
fn memories_and_decisions_can_be_pinned_but_not_other_kinds() {
    let (_dir, mut s, a) = fixture();
    let decision = save(
        &mut s,
        &a,
        draft("decision", "Keep data local", "No cloud copies"),
        None,
    );
    let checkpoint = save(
        &mut s,
        &a,
        draft("checkpoint", "Handoff", "Synthetic"),
        None,
    );
    assert!(pin(&mut s, &a, &decision, true).is_ok());
    assert_eq!(
        pin(&mut s, &a, &checkpoint, true).unwrap_err(),
        "INVALID_INPUT"
    );
    let missing = json!({"id":Uuid::new_v4().to_string()});
    assert_eq!(pin(&mut s, &a, &missing, true).unwrap_err(), "NOT_FOUND");
}
#[test]
fn pinning_needs_the_write_scope_for_that_kind() {
    let (_dir, mut s, a) = fixture();
    let m = save(&mut s, &a, draft("memory", "Tea", "Green tea"), None);
    let reader = grant(&mut s, "reader", &["read".into()]);
    let decider = grant(&mut s, "decider", &["read".into(), "decision:write".into()]);
    assert_eq!(pin(&mut s, &reader, &m, true).unwrap_err(), "FORBIDDEN");
    assert_eq!(pin(&mut s, &decider, &m, true).unwrap_err(), "FORBIDDEN");
    pin(&mut s, &a, &m, true).unwrap();
    assert_eq!(
        pin(&mut s, &reader, &m, false).unwrap_err(),
        "FORBIDDEN",
        "unpinning is a change too"
    );
    s.revoke("synthetic").unwrap();
    assert_eq!(pin(&mut s, &a, &m, false).unwrap_err(), "UNAUTHORIZED");
}
#[test]
fn the_pinned_core_is_capped_and_unpinning_frees_a_place() {
    let (_dir, mut s, a) = fixture();
    let mut saved = Vec::new();
    for n in 0..13 {
        saved.push(save(
            &mut s,
            &a,
            draft("memory", &format!("Core {n}"), "Synthetic"),
            None,
        ));
    }
    for m in &saved[..12] {
        pin(&mut s, &a, m, true).unwrap();
    }
    assert_eq!(
        pin(&mut s, &a, &saved[12], true).unwrap_err(),
        "RESOURCE_LIMIT"
    );
    pin(&mut s, &a, &saved[0], false).unwrap();
    assert!(pin(&mut s, &a, &saved[12], true).is_ok());
}
#[test]
fn pins_follow_revisions_and_retired_records_drop_out_but_stay_counted() {
    let (_dir, mut s, a) = fixture();
    let tea = save(&mut s, &a, draft("memory", "Tea", "Green tea"), None);
    let pet = save(&mut s, &a, draft("memory", "Pet", "A cat named Miso"), None);
    pin(&mut s, &a, &tea, true).unwrap();
    pin(&mut s, &a, &pet, true).unwrap();
    let tea = save(&mut s, &a, draft("memory", "Tea", "Oolong"), Some(&tea));
    let mut withdrawn = draft("memory", "Pet", "A cat named Miso");
    withdrawn["status"] = json!("retracted");
    save(&mut s, &a, withdrawn, Some(&pet));
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(pinned_titles(&c), ["Tea"]);
    assert_eq!(c["pinned"][0]["content"], "Oolong");
    assert_eq!(c["pinned"][0]["revision"], tea["revision"]);
    assert_eq!(c["pinnedSummary"]["total"], 2);
    assert_eq!(c["pinnedSummary"]["inactive"], 1);
    assert!(
        pin(&mut s, &a, &pet, false).is_ok(),
        "a retired record can be unpinned"
    );
    assert_eq!(
        pin(&mut s, &a, &pet, true).unwrap_err(),
        "INVALID_INPUT",
        "a retracted record cannot be pinned again"
    );
}
#[test]
fn long_pinned_content_is_bounded() {
    let (_dir, mut s, a) = fixture();
    let long = save(&mut s, &a, draft("memory", "Long", &"a".repeat(5000)), None);
    pin(&mut s, &a, &long, true).unwrap();
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(c["pinned"][0]["content"].as_str().unwrap().len(), 2000);
    assert_eq!(c["pinned"][0]["truncated"], true);
}
#[test]
fn shared_clients_see_only_granted_pins_and_cannot_pin() {
    let (_dir, mut s, a) = fixture();
    let granted = save(&mut s, &a, draft("memory", "Granted", "Synthetic"), None);
    let hidden = save(&mut s, &a, draft("memory", "Hidden", "Synthetic"), None);
    pin(&mut s, &a, &granted, true).unwrap();
    pin(&mut s, &a, &hidden, true).unwrap();
    let g = s
        .grant_with_access(
            "shared",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: vec![granted["id"].as_str().unwrap().into()],
            },
        )
        .unwrap();
    let shared = Auth {
        client_id: "shared".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let c = s.context(&shared, json!({})).unwrap();
    assert_eq!(pinned_titles(&c), ["Granted"]);
    assert_eq!(c["pinnedSummary"]["total"], 1);
    assert_eq!(
        pin(&mut s, &shared, &granted, false).unwrap_err(),
        "FORBIDDEN"
    );
}
#[test]
fn pins_survive_export_and_restore() {
    let (dir, mut s, a) = fixture();
    let m = save(&mut s, &a, draft("memory", "Tea", "Green tea"), None);
    pin(&mut s, &a, &m, true).unwrap();
    let bundle = s.export().unwrap();
    assert_eq!(bundle["pins"].as_array().unwrap().len(), 1);
    let mut restored = Store::restore(&dir.path().join("restored.sqlite"), bundle.clone()).unwrap();
    let r = grant(&mut restored, "restored", &["read".into()]);
    let c = restored.context(&r, json!({})).unwrap();
    assert_eq!(pinned_titles(&c), ["Tea"]);
    assert_eq!(c["pinned"][0]["pinnedBy"], "synthetic");
    // Bundles written before pins existed still restore.
    let mut old = bundle.clone();
    old.as_object_mut().unwrap().remove("pins");
    assert!(Store::restore(&dir.path().join("old.sqlite"), old).is_ok());
    // A pin pointing at a record the bundle does not contain is rejected.
    let mut broken = bundle;
    broken["pins"][0]["recordId"] = json!(Uuid::new_v4().to_string());
    assert!(Store::restore(&dir.path().join("broken.sqlite"), broken).is_err());
    assert!(!dir.path().join("broken.sqlite").exists());
}
#[test]
fn malformed_pin_requests_are_rejected() {
    let (_dir, mut s, a) = fixture();
    let m = save(&mut s, &a, draft("memory", "Tea", "Green tea"), None);
    for bad in [
        json!({"id":m["id"]}),
        json!({"pinned":true}),
        json!({"id":"not-a-uuid","pinned":true}),
        json!({"id":m["id"],"pinned":"yes"}),
        json!({"id":m["id"],"pinned":true,"note":"x"}),
    ] {
        assert_eq!(
            s.pin(&a, bad.clone()).unwrap_err().code(),
            "INVALID_INPUT",
            "{bad}"
        );
    }
}
