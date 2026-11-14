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
        Stage::Access,
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
    assert_eq!(setup.stage, Stage::Access);
    assert_eq!(setup.access_mode, AccessMode::Companion);
    assert!(screen(&setup).contains("empty means none"));
    setup.input(press(KeyCode::Enter)).unwrap();
    assert_eq!(setup.stage, Stage::Confirm);
    assert!(!setup.busy);
    assert!(screen(&setup).contains("Synthetic companion"));
    assert!(screen(&setup).contains("synthetic-a"));
    assert!(screen(&setup).contains("Executable:"));
    assert!(screen(&setup).contains("Workspace:"));
    assert!(screen(&setup).contains("read"));
    setup.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(setup.stage, Stage::Access);
}

#[test]
fn role_selection_and_allowlist_are_explicit_and_no_paste_connects() {
    let mut setup = Setup::new();
    setup.stage = Stage::Access;
    setup.agents = vec![Agent {
        id: "synthetic-a".into(),
        name: "Synthetic".into(),
    }];
    assert!(setup.selected_access().unwrap().records.is_empty());
    let id = "00000000-0000-4000-8000-000000000001";
    setup.paste(&format!("{id}, {id}\n"));
    assert_eq!(setup.selected_access().unwrap().records, vec![id]);
    setup.shareable = vec![(id.into(), "Synthetic record title".into())];
    setup.records = "1".into();
    assert_eq!(setup.selected_access().unwrap().records, vec![id]);
    assert!(screen(&setup).contains("Synthetic record title"));
    setup.records = "0".into();
    assert!(setup.selected_access().is_err());
    // Order: Knows me (companion) → Only what I share (shared) → Old style (legacy).
    setup.input(press(KeyCode::Down)).unwrap();
    assert_eq!(setup.selected_access().unwrap(), ClientAccess::default());
    assert!(screen(&setup).contains("Old style (not recommended)"));
    setup.input(press(KeyCode::Down)).unwrap();
    assert_eq!(setup.access_mode, AccessMode::Companion);
    assert!(setup.selected_access().unwrap().records.is_empty());
    assert!(screen(&setup).contains("Knows me"));
    assert!(screen(&setup).contains("same chat"));
    assert!(!setup.busy);
    setup.input(press(KeyCode::Down)).unwrap();
    setup.records = "bad-id".into();
    assert!(setup.input(press(KeyCode::Enter)).is_err());
    assert_eq!(setup.stage, Stage::Access);
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
