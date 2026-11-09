use super::*;
use ratatui::{Terminal, backend::TestBackend};
fn press(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}
fn screen(setup: &Setup) -> String {
    let mut terminal = Terminal::new(TestBackend::new(100, 24)).unwrap();
    terminal.draw(|f| setup.render(f, f.area())).unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect()
}
#[test]
fn owner_secret_is_masked_in_every_page_and_cleared_on_cancel() {
    let mut setup = Setup::new();
    setup.key.push_str("synthetic-owner-secret");
    setup.api_key.push_str("synthetic-api-secret");
    for page in [
        Stage::Credentials,
        Stage::Agents,
        Stage::Confirm,
        Stage::Result,
    ] {
        setup.stage = page;
        assert!(!screen(&setup).contains("synthetic-owner-secret"));
        assert!(!screen(&setup).contains("synthetic-api-secret"));
    }
    setup.stage = Stage::Credentials;
    assert!(setup.input(press(KeyCode::Esc)).unwrap());
    assert!(setup.key.is_empty());
}
#[test]
fn selecting_agent_requires_explicit_review_before_connect() {
    let mut setup = Setup::new();
    setup.stage = Stage::Agents;
    let temp = tempfile::tempdir().unwrap();
    mother::store::Store::initialize(&temp.path().join("context.sqlite"), "Synthetic setup")
        .unwrap();
    setup.root = temp.path().to_path_buf();
    setup.agents = vec![Agent {
        id: "synthetic-a".into(),
        name: "Synthetic companion".into(),
    }];
    setup.session = Some(Session::new("http://127.0.0.1:8080", "synthetic-owner-token").unwrap());
    setup.input(press(KeyCode::Enter)).unwrap();
    assert_eq!(setup.stage, Stage::Confirm);
    assert!(!setup.busy);
    assert!(screen(&setup).contains("Synthetic companion"));
    assert!(screen(&setup).contains("synthetic-a"));
    assert!(screen(&setup).contains("Executable:"));
    assert!(screen(&setup).contains("Workspace:"));
    assert!(screen(&setup).contains("read"));
    setup.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(setup.stage, Stage::Agents);
}
#[test]
fn paste_cannot_trigger_connect_and_controls_are_filtered() {
    let mut setup = Setup::new();
    setup.selected = 1;
    setup.paste("abc\n\u{1b}def");
    assert_eq!(&*setup.key, "abcdef");
    assert!(!setup.busy);
    setup.stage = Stage::Confirm;
    setup.paste("\r\n");
    assert!(!setup.busy);
}
