//! The owner tends pins, handoffs and history directly, without going through a harness.
use mother::{contracts::Auth, store::Store};
use serde_json::{Value, json};
use uuid::Uuid;
const WORK: &[&str] = &["read", "memory:write", "decision:write", "task:write"];
fn grant(s: &mut Store, name: &str, scopes: &[&str]) -> Auth {
    let scopes: Vec<String> = scopes.iter().map(|s| (*s).to_owned()).collect();
    let g = s.grant(name, &scopes).unwrap();
    Auth {
        client_id: name.into(),
        token: g["token"].as_str().unwrap().into(),
    }
}
fn fixture() -> (tempfile::TempDir, Store, Auth) {
    let dir = tempfile::Builder::new()
        .prefix("mother-tend-")
        .tempdir()
        .unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic tend").unwrap();
    let auth = grant(&mut store, "claude-code", WORK);
    (dir, store, auth)
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
fn memory(title: &str, content: &str, status: &str) -> Value {
    json!({"kind":"memory","status":status,"basis":"owner_statement","title":title,"content":content})
}
fn task(title: &str, status: &str) -> Value {
    json!({"kind":"task","status":status,"title":title,"content":"Synthetic"})
}
fn id(record: &Value) -> &str {
    record["id"].as_str().unwrap()
}
fn code<T: std::fmt::Debug>(result: mother::error::Result<T>) -> String {
    result.unwrap_err().code().to_owned()
}
#[test]
fn the_owner_pins_and_unpins_without_a_grant_and_harnesses_see_it() {
    let (_dir, mut s, a) = fixture();
    let name = save(
        &mut s,
        &a,
        memory("Preferred name", "Call me Sol", "active"),
        None,
    );
    let result = s.owner_pin(id(&name), true).unwrap();
    assert_eq!(result["pinned"], true);
    assert_eq!(result["changed"], true);
    assert_eq!(result["pinnedBy"], "@owner");
    assert_eq!(s.owner_pin(id(&name), true).unwrap()["changed"], false);
    let c = s.context(&a, json!({})).unwrap();
    assert_eq!(c["pinned"][0]["title"], "Preferred name");
    assert_eq!(c["pinned"][0]["pinnedBy"], "@owner");
    let pins = s.owner_pins().unwrap();
    assert_eq!(pins["limit"], 12);
    assert_eq!(pins["pins"][0]["id"], name["id"]);
    assert_eq!(pins["pins"][0]["active"], true);
    assert_eq!(s.owner_pin(id(&name), false).unwrap()["pinned"], false);
    assert_eq!(s.owner_pins().unwrap()["pins"], json!([]));
}
#[test]
fn owner_pins_follow_the_same_rules_as_harness_pins() {
    let (_dir, mut s, a) = fixture();
    let job = save(&mut s, &a, task("Synthetic job", "todo"), None);
    let gone = save(&mut s, &a, memory("Old", "Withdrawn", "retracted"), None);
    assert_eq!(code(s.owner_pin(id(&job), true)), "INVALID_INPUT");
    assert_eq!(code(s.owner_pin(id(&gone), true)), "INVALID_INPUT");
    assert_eq!(
        code(s.owner_pin(&Uuid::new_v4().to_string(), true)),
        "NOT_FOUND"
    );
    assert_eq!(code(s.owner_pin("not-a-uuid", true)), "INVALID_INPUT");
    for n in 0..12 {
        let m = save(
            &mut s,
            &a,
            memory(&format!("Core {n}"), "Synthetic", "active"),
            None,
        );
        s.owner_pin(id(&m), true).unwrap();
    }
    let extra = save(
        &mut s,
        &a,
        memory("One too many", "Synthetic", "active"),
        None,
    );
    assert_eq!(code(s.owner_pin(id(&extra), true)), "RESOURCE_LIMIT");
    // A pinned record that is later retracted stays listed, marked inactive, until unpinned.
    let pins = s.owner_pins().unwrap();
    let first = pins["pins"][0].clone();
    let current = s.get(&a, first["id"].as_str().unwrap()).unwrap();
    save(
        &mut s,
        &a,
        memory("Core 0", "Synthetic", "retracted"),
        Some(&current),
    );
    let pins = s.owner_pins().unwrap();
    let entry = pins["pins"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == first["id"])
        .unwrap();
    assert_eq!(entry["active"], false);
}
#[test]
fn the_owner_hands_a_task_to_a_harness_and_takes_it_back() {
    let (_dir, mut s, a) = fixture();
    let codex = grant(&mut s, "codex", WORK);
    let job = save(&mut s, &a, task("Refactor the cache", "todo"), None);
    let result = s.owner_assign(id(&job), Some("codex")).unwrap();
    assert_eq!(result["assignee"], "codex");
    assert_eq!(result["changed"], true);
    let waiting = s.context(&codex, json!({})).unwrap()["assignedToYou"].clone();
    assert_eq!(waiting[0]["title"], "Refactor the cache");
    assert_eq!(waiting[0]["assignedBy"], "@owner");
    let list = s.owner_handoffs().unwrap();
    assert_eq!(list["harnesses"], json!(["claude-code", "codex"]));
    assert_eq!(list["handoffs"][0]["title"], "Refactor the cache");
    assert_eq!(list["handoffs"][0]["assignee"], "codex");
    assert_eq!(list["handoffs"][0]["open"], true);
    assert_eq!(
        code(s.owner_assign(id(&job), Some("nobody"))),
        "INVALID_INPUT"
    );
    let cleared = s.owner_assign(id(&job), None).unwrap();
    assert_eq!(cleared["assignee"], Value::Null);
    assert_eq!(s.owner_handoffs().unwrap()["handoffs"], json!([]));
}
#[test]
fn finished_handoffs_stay_listed_as_closed() {
    let (_dir, mut s, a) = fixture();
    grant(&mut s, "codex", WORK);
    let job = save(&mut s, &a, task("Write notes", "todo"), None);
    s.owner_assign(id(&job), Some("codex")).unwrap();
    save(&mut s, &a, task("Write notes", "done"), Some(&job));
    let list = s.owner_handoffs().unwrap();
    assert_eq!(list["handoffs"][0]["status"], "done");
    assert_eq!(list["handoffs"][0]["open"], false);
}
#[test]
fn the_owner_lists_records_with_their_pin_and_handoff_state() {
    let (_dir, mut s, a) = fixture();
    grant(&mut s, "codex", WORK);
    let name = save(
        &mut s,
        &a,
        memory("Preferred name", "Call me Sol", "active"),
        None,
    );
    save(
        &mut s,
        &a,
        memory("Evening tea", "Green tea", "active"),
        None,
    );
    save(
        &mut s,
        &a,
        memory("Old address", "Withdrawn", "retracted"),
        None,
    );
    let job = save(&mut s, &a, task("Refactor the cache", "todo"), None);
    s.owner_pin(id(&name), true).unwrap();
    s.owner_assign(id(&job), Some("codex")).unwrap();
    let all = s.owner_records(json!({})).unwrap();
    let rows = all["records"].as_array().unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(all["omitted"], 0);
    let find = |title: &str| rows.iter().find(|r| r["title"] == title).unwrap();
    assert_eq!(find("Preferred name")["pinned"], true);
    assert_eq!(find("Evening tea")["pinned"], false);
    assert_eq!(find("Old address")["recallable"], false);
    assert_eq!(find("Refactor the cache")["assignee"], "codex");
    assert_eq!(find("Evening tea")["excerpt"], "Green tea");
    let memories = s.owner_records(json!({"kind":"memory"})).unwrap();
    assert_eq!(memories["records"].as_array().unwrap().len(), 3);
    let tea = s.owner_records(json!({"query":"tea"})).unwrap();
    assert_eq!(tea["records"].as_array().unwrap().len(), 1);
    let limited = s.owner_records(json!({"limit":1})).unwrap();
    assert_eq!(limited["records"].as_array().unwrap().len(), 1);
    assert_eq!(limited["omitted"], 3);
    assert_eq!(
        code(s.owner_records(json!({"kind":"poem"}))),
        "INVALID_INPUT"
    );
    assert_eq!(
        code(s.owner_records(json!({"owner":true}))),
        "INVALID_INPUT"
    );
}
#[test]
fn the_owner_reads_history_without_a_grant() {
    let (_dir, mut s, a) = fixture();
    let first = save(&mut s, &a, memory("Tea", "Green tea", "active"), None);
    save(&mut s, &a, memory("Tea", "Oolong", "active"), Some(&first));
    let h = s.owner_history(json!({"id":first["id"]})).unwrap();
    assert_eq!(h["currentRevision"], 2);
    assert_eq!(h["revisions"][0]["changed"], json!(["content"]));
    let old = s
        .owner_history(json!({"id":first["id"],"revision":1}))
        .unwrap();
    assert_eq!(old["record"]["content"], "Green tea");
    assert_eq!(
        code(s.owner_history(json!({"id":"not-a-uuid"}))),
        "INVALID_INPUT"
    );
}
#[test]
fn owner_pins_and_handoffs_survive_export_and_restore() {
    let (dir, mut s, a) = fixture();
    grant(&mut s, "codex", WORK);
    let name = save(
        &mut s,
        &a,
        memory("Preferred name", "Call me Sol", "active"),
        None,
    );
    let job = save(&mut s, &a, task("Refactor the cache", "todo"), None);
    s.owner_pin(id(&name), true).unwrap();
    s.owner_assign(id(&job), Some("codex")).unwrap();
    let bundle = s.export().unwrap();
    let mut restored = Store::restore(&dir.path().join("restored.sqlite"), bundle).unwrap();
    assert_eq!(
        restored.owner_pins().unwrap()["pins"][0]["pinnedBy"],
        "@owner"
    );
    assert_eq!(
        restored.owner_handoffs().unwrap()["handoffs"][0]["assignedBy"],
        "@owner"
    );
}
