use super::*;
use mother::contracts::Auth;
use ratatui::{Terminal, backend::TestBackend};
use serde_json::{Value, json};
use uuid::Uuid;

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}
fn render(tend: &Tend, width: u16, height: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|f| tend.render(f, f.area())).unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect()
}
/// A synthetic workspace with two harnesses, two memories, a decision and two tasks.
fn workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("context.sqlite");
    let mut s = Store::initialize(&path, "Synthetic tend screen").unwrap();
    let scopes: Vec<String> = ["read", "memory:write", "decision:write", "task:write"]
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    let g = s.grant("claude-code", &scopes).unwrap();
    s.grant("codex", &scopes).unwrap();
    let a = Auth {
        client_id: "claude-code".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let w = s.workspace().unwrap();
    for record in [
        json!({"kind":"task","status":"done","title":"Finished job","content":"Synthetic"}),
        json!({"kind":"task","status":"todo","title":"Refactor the cache","content":"Synthetic"}),
        json!({"kind":"decision","status":"confirmed","title":"Keep everything local","content":"One file on this machine"}),
        json!({"kind":"memory","status":"active","basis":"owner_statement","title":"Evening tea","content":"Green tea after nine"}),
        json!({"kind":"memory","status":"active","basis":"owner_statement","title":"Preferred name","content":"Call me Sol"}),
    ] {
        s.write(&a, json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    (dir, path)
}
fn pins(path: &std::path::Path) -> Vec<String> {
    Store::open(path).unwrap().owner_pins().unwrap()["pins"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["title"].as_str().unwrap().to_owned())
        .collect()
}
fn handoffs(path: &std::path::Path) -> Value {
    Store::open(path).unwrap().owner_handoffs().unwrap()["handoffs"].clone()
}
#[test]
fn an_empty_core_explains_itself_and_tabs_list_each_kind() {
    let (_dir, path) = workspace();
    let mut tend = Tend::open(path).unwrap();
    let screen = render(&tend, 100, 30);
    assert!(screen.contains("Pinned 0/12"));
    assert!(screen.contains("Nothing is pinned yet"));
    tend.input(press(KeyCode::Tab)).unwrap();
    let screen = render(&tend, 100, 30);
    assert!(screen.contains("Preferred name"));
    assert!(screen.contains("Evening tea"));
    assert!(
        screen.contains("Call me Sol"),
        "the selected record's text is shown"
    );
    assert!(!screen.contains("Refactor the cache"));
    tend.input(press(KeyCode::Tab)).unwrap();
    assert!(render(&tend, 100, 30).contains("Keep everything local"));
    tend.input(press(KeyCode::Tab)).unwrap();
    let screen = render(&tend, 100, 30);
    assert!(screen.contains("Refactor the cache"));
    assert!(screen.contains("Finished job"));
    tend.input(press(KeyCode::BackTab)).unwrap();
    assert!(render(&tend, 100, 30).contains("Keep everything local"));
}
#[test]
fn p_pins_and_unpins_the_selected_memory() {
    let (_dir, path) = workspace();
    let mut tend = Tend::open(path.clone()).unwrap();
    tend.input(press(KeyCode::Tab)).unwrap();
    tend.input(press(KeyCode::Down)).unwrap();
    tend.input(press(KeyCode::Char('p'))).unwrap();
    assert_eq!(pins(&path), ["Evening tea"]);
    assert!(tend.message.contains("Pinned"));
    let screen = render(&tend, 100, 30);
    assert!(screen.contains("Pinned 1/12"));
    assert!(screen.contains("@owner"));
    tend.input(press(KeyCode::BackTab)).unwrap();
    assert!(render(&tend, 100, 30).contains("Evening tea"));
    tend.input(press(KeyCode::Char('p'))).unwrap();
    assert!(pins(&path).is_empty());
    assert!(tend.message.contains("Unpinned"));
}
#[test]
fn a_task_cannot_be_pinned_and_says_why() {
    let (_dir, path) = workspace();
    let mut tend = Tend::open(path.clone()).unwrap();
    for _ in 0..3 {
        tend.input(press(KeyCode::Tab)).unwrap();
    }
    tend.input(press(KeyCode::Char('p'))).unwrap();
    assert!(pins(&path).is_empty());
    assert!(tend.message.contains("Only memories and decisions"));
}
#[test]
fn a_opens_a_chooser_that_hands_a_task_over_or_clears_it() {
    let (_dir, path) = workspace();
    let mut tend = Tend::open(path.clone()).unwrap();
    for _ in 0..3 {
        tend.input(press(KeyCode::Tab)).unwrap();
    }
    tend.input(press(KeyCode::Char('a'))).unwrap();
    let screen = render(&tend, 100, 30);
    assert!(screen.contains("claude-code"));
    assert!(screen.contains("codex"));
    assert!(screen.contains("Nobody"));
    tend.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(
        handoffs(&path),
        json!([]),
        "Esc leaves the chooser without a change"
    );
    tend.input(press(KeyCode::Char('a'))).unwrap();
    tend.input(press(KeyCode::Down)).unwrap();
    assert!(!tend.input(press(KeyCode::Enter)).unwrap());
    let list = handoffs(&path);
    assert_eq!(list[0]["title"], "Refactor the cache");
    assert_eq!(list[0]["assignee"], "codex");
    assert!(render(&tend, 100, 30).contains("codex"));
    tend.input(press(KeyCode::Char('a'))).unwrap();
    tend.input(press(KeyCode::Up)).unwrap();
    tend.input(press(KeyCode::Up)).unwrap();
    tend.input(press(KeyCode::Up)).unwrap();
    tend.input(press(KeyCode::Enter)).unwrap();
    assert_eq!(handoffs(&path), json!([]), "Nobody clears the handoff");
}
#[test]
fn a_finished_task_or_a_memory_cannot_be_handed_off() {
    let (_dir, path) = workspace();
    let mut tend = Tend::open(path.clone()).unwrap();
    tend.input(press(KeyCode::Tab)).unwrap();
    tend.input(press(KeyCode::Char('a'))).unwrap();
    assert!(tend.message.contains("Only tasks"));
    tend.input(press(KeyCode::Tab)).unwrap();
    tend.input(press(KeyCode::Tab)).unwrap();
    tend.input(press(KeyCode::Down)).unwrap();
    tend.input(press(KeyCode::Char('a'))).unwrap();
    tend.input(press(KeyCode::Enter)).unwrap();
    assert_eq!(handoffs(&path), json!([]));
    assert!(tend.message.contains("open tasks"));
}
#[test]
fn escape_goes_home_and_control_characters_never_reach_the_terminal() {
    let (_dir, path) = workspace();
    let mut s = Store::open(&path).unwrap();
    let g = s
        .grant("writer", &["read".into(), "memory:write".into()])
        .unwrap();
    let a = Auth {
        client_id: "writer".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let w = s.workspace().unwrap();
    s.write(&a, json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","status":"active","basis":"source","title":"Odd\u{1b}[31mtitle","content":"Line one\u{7}\nLine two"}})).unwrap();
    drop(s);
    let mut tend = Tend::open(path).unwrap();
    tend.input(press(KeyCode::Tab)).unwrap();
    let screen = render(&tend, 100, 30);
    assert!(screen.contains("Odd[31mtitle"));
    assert!(!screen.contains('\u{1b}'));
    assert!(!screen.contains('\u{7}'));
    assert!(tend.input(press(KeyCode::Esc)).unwrap());
}
