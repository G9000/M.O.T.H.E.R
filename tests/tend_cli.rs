//! Owner commands for records, pins, handoffs and history.
use mother::{config::load_config, store::Store};
use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::TempDir;
use uuid::Uuid;

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", dir)
        .env("MOTHER_MODEL_DIR", dir.join("models"))
        .output()
        .expect("run native CLI")
}
fn ok(dir: &Path, args: &[&str]) -> Value {
    let out = run(dir, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
fn fails(dir: &Path, args: &[&str]) -> String {
    let out = run(dir, args);
    assert!(!out.status.success(), "{args:?} should fail");
    String::from_utf8_lossy(&out.stderr).into_owned()
}
/// An initialized workspace with two harnesses and a few synthetic records.
struct Home {
    dir: TempDir,
    memory: String,
    task: String,
}
fn home() -> Home {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("SOUL.md"), "Synthetic persona").unwrap();
    ok(dir.path(), &["init"]);
    ok(
        dir.path(),
        &[
            "grant",
            "claude-code",
            "--scopes",
            "read,memory:write,task:write",
        ],
    );
    ok(
        dir.path(),
        &["grant", "codex", "--scopes", "read,task:write"],
    );
    let c = load_config(&dir.path().join("clients/claude-code.private.json")).unwrap();
    let mut s = Store::open(&c.database_path).unwrap();
    let w = s.workspace().unwrap();
    let mut save = |record: Value| {
        s.write(&c.auth(), json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})).unwrap()["record"].clone()
    };
    let memory = save(
        json!({"kind":"memory","status":"active","basis":"owner_statement","title":"Preferred name","content":"Call me Sol"}),
    );
    let revised = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"id":memory["id"],"expectedRevision":1,"record":{"kind":"memory","status":"active","basis":"owner_statement","title":"Preferred name","content":"Call me Sol, not Solomon"}});
    let task = save(
        json!({"kind":"task","status":"todo","title":"Refactor the cache","content":"Synthetic"}),
    );
    s.write(&c.auth(), revised).unwrap();
    Home {
        dir,
        memory: memory["id"].as_str().unwrap().into(),
        task: task["id"].as_str().unwrap().into(),
    }
}
#[test]
fn records_lists_what_she_holds_so_ids_can_be_found() {
    let h = home();
    let all = ok(h.dir.path(), &["records"]);
    assert_eq!(all["records"].as_array().unwrap().len(), 2);
    let memories = ok(h.dir.path(), &["records", "--kind", "memory"]);
    assert_eq!(memories["records"][0]["id"], h.memory.as_str());
    assert_eq!(memories["records"][0]["pinned"], false);
    let found = ok(
        h.dir.path(),
        &["records", "--query", "cache", "--limit", "5"],
    );
    assert_eq!(found["records"][0]["title"], "Refactor the cache");
    assert!(fails(h.dir.path(), &["records", "--kind", "poem"]).contains("INVALID_INPUT"));
}
#[test]
fn pin_unpin_and_pins_manage_the_core_from_the_terminal() {
    let h = home();
    let pinned = ok(h.dir.path(), &["pin", &h.memory]);
    assert_eq!(pinned["pinned"], true);
    assert_eq!(pinned["pinnedBy"], "@owner");
    let pins = ok(h.dir.path(), &["pins"]);
    assert_eq!(pins["pins"][0]["title"], "Preferred name");
    assert_eq!(pins["limit"], 12);
    assert_eq!(
        ok(h.dir.path(), &["records", "--kind", "memory"])["records"][0]["pinned"],
        true
    );
    assert_eq!(ok(h.dir.path(), &["unpin", &h.memory])["pinned"], false);
    assert_eq!(ok(h.dir.path(), &["pins"])["pins"], json!([]));
    assert!(fails(h.dir.path(), &["pin", &h.task]).contains("INVALID_INPUT"));
    assert!(fails(h.dir.path(), &["pin", "not-a-uuid"]).contains("INVALID_INPUT"));
}
#[test]
fn assign_and_handoffs_move_work_between_harnesses() {
    let h = home();
    let handed = ok(h.dir.path(), &["assign", &h.task, "--to", "codex"]);
    assert_eq!(handed["assignee"], "codex");
    let list = ok(h.dir.path(), &["handoffs"]);
    assert_eq!(list["harnesses"], json!(["claude-code", "codex"]));
    assert_eq!(list["handoffs"][0]["assignee"], "codex");
    assert_eq!(list["handoffs"][0]["assignedBy"], "@owner");
    assert_eq!(
        ok(h.dir.path(), &["assign", &h.task, "--clear"])["assignee"],
        Value::Null
    );
    assert_eq!(ok(h.dir.path(), &["handoffs"])["handoffs"], json!([]));
    assert!(fails(h.dir.path(), &["assign", &h.task, "--to", "nobody"]).contains("INVALID_INPUT"));
    assert!(!run(h.dir.path(), &["assign", &h.task]).status.success());
    assert!(
        !run(
            h.dir.path(),
            &["assign", &h.task, "--to", "codex", "--clear"]
        )
        .status
        .success()
    );
}
#[test]
fn history_shows_what_a_record_said_before() {
    let h = home();
    let list = ok(h.dir.path(), &["history", &h.memory]);
    assert_eq!(list["currentRevision"], 2);
    assert_eq!(list["revisions"][0]["changed"], json!(["content"]));
    let first = ok(h.dir.path(), &["history", &h.memory, "--revision", "1"]);
    assert_eq!(first["record"]["content"], "Call me Sol");
    assert!(fails(h.dir.path(), &["history", &h.memory, "--revision", "9"]).contains("NOT_FOUND"));
}
