//! A task can be handed to a named harness, which finds it when it next reads context.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    store::Store,
};
use serde_json::{Value, json};
use uuid::Uuid;
const WORK: &[&str] = &["read", "task:write", "memory:write", "checkpoint:write"];
fn grant(s: &mut Store, name: &str, scopes: &[&str]) -> Auth {
    let scopes: Vec<String> = scopes.iter().map(|s| (*s).to_owned()).collect();
    let g = s.grant(name, &scopes).unwrap();
    Auth {
        client_id: name.into(),
        token: g["token"].as_str().unwrap().into(),
    }
}
struct Harnesses {
    dir: tempfile::TempDir,
    store: Store,
    claude: Auth,
    codex: Auth,
}
fn fixture() -> Harnesses {
    let dir = tempfile::Builder::new()
        .prefix("mother-handoffs-")
        .tempdir()
        .unwrap();
    let mut store =
        Store::initialize(&dir.path().join("context.sqlite"), "Synthetic handoffs").unwrap();
    let claude = grant(&mut store, "claude-code", WORK);
    let codex = grant(&mut store, "codex", WORK);
    Harnesses {
        dir,
        store,
        claude,
        codex,
    }
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
fn task(title: &str, status: &str) -> Value {
    json!({"kind":"task","status":status,"title":title,"content":"Synthetic handoff detail"})
}
fn assign(s: &mut Store, a: &Auth, record: &Value, to: Value) -> Result<Value, String> {
    s.assign(a, json!({"id":record["id"],"assignee":to}))
        .map_err(|e| e.code().to_owned())
}
fn waiting(s: &mut Store, a: &Auth) -> Vec<String> {
    s.context(a, json!({})).unwrap()["assignedToYou"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["title"].as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn a_task_handed_to_another_harness_waits_for_it_there_only() {
    let mut h = fixture();
    let refactor = save(
        &mut h.store,
        &h.claude,
        task("Refactor the search cache", "todo"),
        None,
    );
    let result = assign(&mut h.store, &h.claude, &refactor, json!("codex")).unwrap();
    assert_eq!(result["assignee"], "codex");
    assert_eq!(result["changed"], true);
    assert_eq!(
        waiting(&mut h.store, &h.codex),
        ["Refactor the search cache"]
    );
    assert!(waiting(&mut h.store, &h.claude).is_empty());
    let c = h.store.context(&h.codex, json!({"kind":"memory"})).unwrap();
    let entry = &c["assignedToYou"][0];
    assert_eq!(entry["id"], refactor["id"]);
    assert_eq!(entry["status"], "todo");
    assert_eq!(entry["revision"], 1);
    assert_eq!(entry["content"], "Synthetic handoff detail");
    assert_eq!(entry["assignedBy"], "claude-code");
    assert!(entry["assignedAt"].as_str().unwrap().ends_with('Z'));
    assert_eq!(c["assignedToYouSummary"], json!({"total":1,"shown":1}));
}
#[test]
fn the_receiving_harness_works_it_through_the_normal_task_lifecycle() {
    let mut h = fixture();
    let job = save(
        &mut h.store,
        &h.claude,
        task("Write release notes", "todo"),
        None,
    );
    assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap();
    let doing = save(
        &mut h.store,
        &h.codex,
        task("Write release notes", "doing"),
        Some(&job),
    );
    assert_eq!(
        h.store.context(&h.codex, json!({})).unwrap()["assignedToYou"][0]["status"],
        "doing"
    );
    save(
        &mut h.store,
        &h.codex,
        task("Write release notes", "done"),
        Some(&doing),
    );
    assert!(
        waiting(&mut h.store, &h.codex).is_empty(),
        "finished work is no longer waiting"
    );
}
#[test]
fn reassigning_and_unassigning_are_explicit_and_repeatable() {
    let mut h = fixture();
    let job = save(&mut h.store, &h.claude, task("Plan the week", "todo"), None);
    assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap();
    let again = assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap();
    assert_eq!(again["changed"], false);
    let moved = assign(&mut h.store, &h.codex, &job, json!("claude-code")).unwrap();
    assert_eq!(moved["changed"], true);
    assert_eq!(waiting(&mut h.store, &h.claude), ["Plan the week"]);
    assert!(waiting(&mut h.store, &h.codex).is_empty());
    let cleared = assign(&mut h.store, &h.claude, &job, Value::Null).unwrap();
    assert_eq!(cleared["assignee"], Value::Null);
    assert!(waiting(&mut h.store, &h.claude).is_empty());
    assert_eq!(
        assign(&mut h.store, &h.claude, &job, Value::Null).unwrap()["changed"],
        false
    );
}
#[test]
fn only_open_tasks_go_to_a_harness_that_exists_and_can_see_them() {
    let mut h = fixture();
    let job = save(&mut h.store, &h.claude, task("Open", "todo"), None);
    let note = save(
        &mut h.store,
        &h.claude,
        json!({"kind":"memory","status":"active","basis":"owner_statement","title":"Note","content":"Synthetic"}),
        None,
    );
    let finished = save(&mut h.store, &h.claude, task("Finished", "done"), None);
    assert_eq!(
        assign(&mut h.store, &h.claude, &note, json!("codex")).unwrap_err(),
        "INVALID_INPUT"
    );
    assert_eq!(
        assign(&mut h.store, &h.claude, &finished, json!("codex")).unwrap_err(),
        "INVALID_INPUT"
    );
    assert_eq!(
        assign(&mut h.store, &h.claude, &job, json!("nobody")).unwrap_err(),
        "INVALID_INPUT"
    );
    grant(&mut h.store, "retired", WORK);
    h.store.revoke("retired").unwrap();
    assert_eq!(
        assign(&mut h.store, &h.claude, &job, json!("retired")).unwrap_err(),
        "INVALID_INPUT"
    );
    h.store
        .grant_with_access(
            "outsider",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: vec![note["id"].as_str().unwrap().into()],
            },
        )
        .unwrap();
    assert_eq!(
        assign(&mut h.store, &h.claude, &job, json!("outsider")).unwrap_err(),
        "INVALID_INPUT",
        "a shared harness that cannot read the task cannot receive it"
    );
    let missing = json!({"id":Uuid::new_v4().to_string()});
    assert_eq!(
        assign(&mut h.store, &h.claude, &missing, json!("codex")).unwrap_err(),
        "NOT_FOUND"
    );
}
#[test]
fn handing_off_needs_the_task_write_scope() {
    let mut h = fixture();
    let job = save(&mut h.store, &h.claude, task("Open", "todo"), None);
    let reader = grant(&mut h.store, "reader", &["read"]);
    assert_eq!(
        assign(&mut h.store, &reader, &job, json!("codex")).unwrap_err(),
        "FORBIDDEN"
    );
    h.store.revoke("claude-code").unwrap();
    assert_eq!(
        assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap_err(),
        "UNAUTHORIZED"
    );
}
#[test]
fn context_names_the_harnesses_a_task_could_go_to() {
    let mut h = fixture();
    grant(&mut h.store, "hermes", WORK);
    grant(&mut h.store, "retired", WORK);
    h.store.revoke("retired").unwrap();
    let c = h.store.context(&h.claude, json!({})).unwrap();
    assert_eq!(c["harnesses"], json!(["claude-code", "codex", "hermes"]));
    let g = h
        .store
        .grant_with_access(
            "outsider",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: vec![],
            },
        )
        .unwrap();
    let outsider = Auth {
        client_id: "outsider".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let c = h.store.context(&outsider, json!({})).unwrap();
    assert!(
        c.get("harnesses").is_none(),
        "shared clients are not told about other harnesses"
    );
}
#[test]
fn many_handoffs_are_bounded_with_an_honest_total() {
    let mut h = fixture();
    for n in 0..14 {
        let job = save(
            &mut h.store,
            &h.claude,
            task(&format!("Job {n}"), "todo"),
            None,
        );
        assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap();
    }
    let c = h.store.context(&h.codex, json!({})).unwrap();
    assert_eq!(c["assignedToYou"].as_array().unwrap().len(), 10);
    assert_eq!(c["assignedToYouSummary"], json!({"total":14,"shown":10}));
}
#[test]
fn handoffs_survive_export_and_restore() {
    let mut h = fixture();
    let job = save(&mut h.store, &h.claude, task("Carry over", "todo"), None);
    assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap();
    let bundle = h.store.export().unwrap();
    assert_eq!(bundle["assignments"].as_array().unwrap().len(), 1);
    let mut restored =
        Store::restore(&h.dir.path().join("restored.sqlite"), bundle.clone()).unwrap();
    // Grants are never exported; re-granting the same name picks the work back up.
    let codex = grant(&mut restored, "codex", WORK);
    assert_eq!(waiting(&mut restored, &codex), ["Carry over"]);
    let mut old = bundle.clone();
    old.as_object_mut().unwrap().remove("assignments");
    assert!(Store::restore(&h.dir.path().join("old.sqlite"), old).is_ok());
    let mut broken = bundle;
    broken["assignments"][0]["recordId"] = json!(Uuid::new_v4().to_string());
    assert!(Store::restore(&h.dir.path().join("broken.sqlite"), broken).is_err());
}
#[test]
fn malformed_handoff_requests_are_rejected() {
    let mut h = fixture();
    let job = save(&mut h.store, &h.claude, task("Open", "todo"), None);
    for bad in [
        json!({"id":job["id"]}),
        json!({"assignee":"codex"}),
        json!({"id":"not-a-uuid","assignee":"codex"}),
        json!({"id":job["id"],"assignee":7}),
        json!({"id":job["id"],"assignee":"bad name!"}),
        json!({"id":job["id"],"assignee":"codex","note":"x"}),
    ] {
        assert_eq!(
            h.store.assign(&h.claude, bad.clone()).unwrap_err().code(),
            "INVALID_INPUT",
            "{bad}"
        );
    }
}
#[test]
fn the_receiving_harness_brief_leads_with_what_is_waiting_for_it() {
    let mut h = fixture();
    let job = save(
        &mut h.store,
        &h.claude,
        task("Review the pins design", "todo"),
        None,
    );
    assign(&mut h.store, &h.claude, &job, json!("codex")).unwrap();
    let brief = mother::brief::compose(&mut h.store, &h.codex).unwrap();
    let section = brief
        .find("## Waiting for you (1)")
        .expect("a waiting section");
    assert!(brief[section..].contains("Review the pins design"));
    assert!(brief[section..].contains("from claude-code"));
    assert!(
        brief.contains("claude-code, codex"),
        "the brief names the harnesses"
    );
    let other = mother::brief::compose(&mut h.store, &h.claude).unwrap();
    assert!(!other.contains("## Waiting for you"));
}
