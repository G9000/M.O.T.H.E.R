use super::*;
use ratatui::{Terminal, backend::TestBackend};
use serde_json::json;

fn proposal() -> Value {
    json!({"id":"11111111-1111-4111-8111-111111111111","revision":1,"status":"pending","actor":"synthetic-chatgpt","source":"Synthetic conversation, explicitly requested capture","basis":"owner_statement","generation":"22222222-2222-4222-8222-222222222222","record":{"kind":"memory","title":"Tea","content":"Green tea","status":"active","basis":"owner_statement","references":[],"sources":[]},"before":null,"currentRecord":null,"conflict":false,"duplicateRecordId":null})
}
fn press(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}
fn render(inbox: &Inbox, width: u16, height: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|f| inbox.render(f, f.area())).unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect()
}
#[test]
fn review_shows_attribution_before_after_and_confirmation_is_explicit() {
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(proposal());
    let output = render(&inbox, 100, 30);
    assert!(output.contains("synthetic-chatgpt"));
    assert!(output.contains("Synthetic conversation"));
    assert!(output.contains("Before"));
    assert!(output.contains("Green tea"));
    inbox.input(press(KeyCode::Char('s'))).unwrap();
    assert_eq!(inbox.page, InboxPage::ConfirmSave);
    assert!(
        inbox.pending.is_none(),
        "request is created only on confirmation"
    );
    inbox.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(inbox.page, InboxPage::Review);
}
#[test]
fn editing_unicode_multiline_and_cancel_preserves_original() {
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(proposal());
    inbox.input(press(KeyCode::Char('e'))).unwrap();
    inbox.input(press(KeyCode::Tab)).unwrap();
    inbox
        .input(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL))
        .unwrap();
    inbox.paste("\u{3bb}\nsecond\u{1b}line");
    assert_eq!(
        inbox.editor.as_ref().unwrap().fields[1],
        "\u{3bb}\nsecondline"
    );
    inbox.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(inbox.candidate["content"], "Green tea");
}
#[test]
fn edit_returns_to_review_without_saving_and_retains_status_basis() {
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(proposal());
    inbox.input(press(KeyCode::Char('e'))).unwrap();
    inbox.input(press(KeyCode::Tab)).unwrap();
    inbox
        .input(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL))
        .unwrap();
    inbox.paste("Black tea");
    inbox
        .input(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL))
        .unwrap();
    assert_eq!(inbox.page, InboxPage::Review);
    assert_eq!(inbox.candidate["content"], "Black tea");
    assert_eq!(inbox.candidate["basis"], "owner_statement");
    assert!(inbox.pending.is_none());
    assert!(render(&inbox, 100, 30).contains("Edited"));
}
#[test]
fn long_review_can_scroll_to_tail_and_never_renders_control_sequences() {
    let mut p = proposal();
    p["record"]["content"] = json!(format!(
        "{}\nFINAL-MARKER\u{1b}[2J",
        "Long synthetic passage\n".repeat(80)
    ));
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(p);
    assert!(!render(&inbox, 80, 24).contains("FINAL-MARKER"));
    for _ in 0..85 {
        inbox.input(press(KeyCode::Down)).unwrap();
    }
    let output = render(&inbox, 80, 24);
    assert!(output.contains("FINAL-MARKER"));
    assert!(!output.contains('\u{1b}'));
}
#[test]
fn conflict_blocks_save_but_keeps_dismiss_and_before_visible() {
    let mut p = proposal();
    p["conflict"] = json!(true);
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(p);
    inbox.input(press(KeyCode::Char('s'))).unwrap();
    assert_eq!(inbox.page, InboxPage::Review);
    assert!(inbox.message.contains("changed"));
    inbox.input(press(KeyCode::Char('d'))).unwrap();
    assert_eq!(inbox.page, InboxPage::ConfirmDismiss);
}
#[test]
fn pasted_shortcuts_never_approve_and_editor_bounds_are_enforced() {
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(proposal());
    inbox.paste("s\nd\n");
    assert_eq!(inbox.page, InboxPage::Review);
    let mut editor = Editor::new(&inbox.candidate);
    editor.selected = 0;
    editor.fields[0].clear();
    editor.cursors[0] = 0;
    editor.insert(&"x".repeat(500));
    assert_eq!(editor.fields[0].len(), 240);
}

fn workspace_fixture() -> (tempfile::TempDir, Store, mother::contracts::Auth, Value) {
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::initialize(
        &dir.path().join("context.sqlite"),
        "Synthetic inbox persona",
    )
    .unwrap();
    let grant = store
        .grant("synthetic-chatgpt", &["read".into(), "memory:write".into()])
        .unwrap();
    let auth = mother::contracts::Auth {
        client_id: "synthetic-chatgpt".into(),
        token: grant["token"].as_str().unwrap().into(),
    };
    let w = store.workspace().unwrap();
    let request = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"source":"Synthetic owner statement in test chat","basis":"owner_statement","record":proposal()["record"]});
    (dir, store, auth, request)
}
#[test]
fn owner_controller_edits_then_saves_to_shared_store_only_after_confirmation() {
    let (dir, mut store, auth, request) = workspace_fixture();
    store.propose(&auth, request).unwrap();
    let mut inbox = Inbox::open(dir.path().join("context.sqlite")).unwrap();
    assert_eq!(inbox.items.len(), 1);
    inbox.input(press(KeyCode::Enter)).unwrap();
    inbox.input(press(KeyCode::Char('e'))).unwrap();
    inbox.input(press(KeyCode::Tab)).unwrap();
    inbox
        .input(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL))
        .unwrap();
    inbox.paste("Black tea — owner edited");
    inbox
        .input(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL))
        .unwrap();
    assert!(
        store.context(&auth, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    inbox.input(press(KeyCode::Char('s'))).unwrap();
    assert!(
        store.context(&auth, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    inbox.input(press(KeyCode::Enter)).unwrap();
    assert_eq!(inbox.page, InboxPage::List);
    assert!(inbox.message.starts_with("Saved"));
    assert!(inbox.items.is_empty());
    let context = store.context(&auth, json!({})).unwrap();
    let record = store
        .get(&auth, context["records"][0]["id"].as_str().unwrap())
        .unwrap();
    assert_eq!(record["content"], "Black tea — owner edited");
    assert_eq!(record["actor"], "@owner-review");
}
#[test]
fn owner_controller_dismiss_keeps_proposal_out_of_recall() {
    let (dir, mut store, auth, request) = workspace_fixture();
    store.propose(&auth, request).unwrap();
    let mut inbox = Inbox::open(dir.path().join("context.sqlite")).unwrap();
    inbox.input(press(KeyCode::Enter)).unwrap();
    inbox.input(press(KeyCode::Char('d'))).unwrap();
    inbox.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(inbox.page, InboxPage::Review);
    inbox.input(press(KeyCode::Char('d'))).unwrap();
    inbox.input(press(KeyCode::Enter)).unwrap();
    assert!(inbox.message.starts_with("Dismissed"));
    assert!(inbox.items.is_empty());
    assert!(
        store.context(&auth, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn retry_uses_frozen_operation_and_does_not_duplicate_a_committed_save() {
    let (dir, mut store, auth, request) = workspace_fixture();
    store.propose(&auth, request).unwrap();
    let mut inbox = Inbox::open(dir.path().join("context.sqlite")).unwrap();
    inbox.input(press(KeyCode::Enter)).unwrap();
    let review = json!({"workspaceId":inbox.workspace["id"],"generation":inbox.workspace["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"proposalId":inbox.proposal["id"],"expectedRevision":1,"action":"save","record":inbox.candidate});
    store.review_proposal(review.clone()).unwrap(); // commit succeeded but imagine its response was lost
    inbox.pending = Some(review);
    inbox.page = InboxPage::ConfirmSave;
    inbox.input(press(KeyCode::Esc)).unwrap();
    assert_eq!(inbox.page, InboxPage::ConfirmSave);
    inbox.input(press(KeyCode::Enter)).unwrap();
    assert_eq!(inbox.page, InboxPage::List);
    assert_eq!(
        store.context(&auth, json!({})).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn focused_editor_field_remains_visible_with_multiline_wide_title_at_minimum_size() {
    let mut p = proposal();
    p["record"]["title"] = json!("界\n".repeat(90));
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(p);
    inbox.input(press(KeyCode::Char('e'))).unwrap();
    for field in [1, 2, 3] {
        inbox.editor.as_mut().unwrap().selected = field;
        let output = render(&inbox, 72, 14);
        let expected = match field {
            1 => "Green tea",
            2 => "active",
            _ => "owner_statement",
        };
        assert!(
            output.contains(expected),
            "Focused field {field} is hidden at minimum size"
        );
    }
}
#[test]
fn before_review_includes_record_identity_and_author() {
    let mut p = proposal();
    let mut before = p["record"].clone();
    before["id"] = json!("33333333-3333-4333-8333-333333333333");
    before["revision"] = json!(4);
    before["actor"] = json!("synthetic-hermes");
    before["createdAt"] = json!("2026-01-01T00:00:00.000Z");
    before["updatedAt"] = before["createdAt"].clone();
    p["before"] = before.clone();
    p["currentRecord"] = before;
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(p);
    let output = render(&inbox, 100, 36);
    assert!(output.contains("33333333-3333-4333-8333-333333333333"));
    assert!(output.contains("synthetic-hermes"));
}

#[test]
fn literal_cursor_marker_in_content_does_not_hide_actual_edit_position() {
    let mut p = proposal();
    p["record"]["content"] = json!(format!(
        "literal ▏ in source\n{}\nCURSOR-END",
        "earlier line\n".repeat(40)
    ));
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.open_proposal(p);
    inbox.input(press(KeyCode::Char('e'))).unwrap();
    inbox.input(press(KeyCode::Tab)).unwrap();
    assert!(render(&inbox, 72, 14).contains("CURSOR-END"));
}
#[test]
fn inbox_selection_stays_visible_when_other_titles_are_multiline() {
    let mut inbox = Inbox::empty(PathBuf::from("unused"));
    inbox.items = vec![
        json!({"title":"Long\n".repeat(30),"actor":"test"}),
        json!({"title":"Long\n".repeat(30),"actor":"test"}),
        json!({"title":"Selected proposal","actor":"test"}),
    ];
    inbox.selected = 2;
    assert!(render(&inbox, 72, 14).contains("> Selected proposal"));
}
