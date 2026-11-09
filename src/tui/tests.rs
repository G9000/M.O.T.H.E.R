use super::*;
use ratatui::{Terminal, backend::TestBackend};
fn fixture(page: Page) -> App {
    App {
        page,
        form: Some(Form::new(Settings {
            version: 1,
            tunnel_id: "tunnel_synthetic".into(),
            organization_id: None,
            client_config: PathBuf::from("/private/config.json"),
            tunnel_client: PathBuf::from("/tools/tunnel-client"),
            mother_exe: PathBuf::from("/tools/mother"),
        })),
        status: None,
        message: "Synthetic test only".into(),
        busy: false,
        startup: Some(false),
        workspace: "Synthetic workspace".into(),
    }
}
fn screen(app: &App, width: u16, height: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|frame| view::render(frame, app)).unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}
#[test]
fn setup_and_confirmation_never_render_secret() {
    let mut app = fixture(Page::Setup);
    app.form
        .as_mut()
        .unwrap()
        .key
        .push_str("synthetic-secret-never-display");
    for page in [Page::Setup, Page::ConfirmSave] {
        app.page = page;
        let output = screen(&app, 110, 32);
        assert!(!output.contains("synthetic-secret"));
        assert!(output.contains("M.O.T.H.E.R"));
    }
}
#[test]
fn small_terminal_does_not_panic_or_show_secrets() {
    for page in [
        Page::Home,
        Page::Setup,
        Page::ConfirmSave,
        Page::Help,
        Page::ConfirmStartup,
    ] {
        let app = fixture(page);
        let output = screen(&app, 25, 8);
        assert!(output.contains("Resize"));
    }
}
#[test]
fn input_handles_unicode_backspace_navigation_and_control_characters() {
    let mut form = fixture(Page::Setup).form.take().unwrap();
    form.selected = 5;
    for c in ['\u{e9}', '\u{3bb}'] {
        form.input(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    form.input(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(&*form.key, "\u{e9}");
    form.input(KeyEvent::new(KeyCode::Char('\u{1b}'), KeyModifiers::NONE));
    assert_eq!(&*form.key, "\u{e9}");
    form.input(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(form.selected, 0);
}
#[test]
fn local_readiness_never_claims_chatgpt_acceptance() {
    let mut app = fixture(Page::Home);
    app.status = Some(Status {
        state: connection::ConnectionState::Ready,
        detail: "Local runtime ready".into(),
        process_running: true,
        healthy: true,
        ready: true,
    });
    let output = screen(&app, 110, 32);
    assert!(output.contains("ChatGPT read/save: not verified here"));
}

#[test]
fn unhealthy_readiness_signal_does_not_offer_plugin_creation() {
    let mut app = fixture(Page::Home);
    app.status = Some(Status {
        state: connection::ConnectionState::Unhealthy,
        detail: "Health failed".into(),
        process_running: true,
        healthy: false,
        ready: true,
    });
    let output = screen(&app, 110, 32);
    assert!(!output.contains("Ready for ChatGPT setup"));
}
#[test]
fn failed_save_returns_to_draft_without_revealing_secret() {
    let mut app = fixture(Page::ConfirmSave);
    app.form.as_mut().unwrap().fields[0] = "tunnel_correctable".into();
    app.finish(Err(Error::domain("CONNECTION", "Invalid config path")));
    assert_eq!(app.page, Page::Setup);
    assert_eq!(app.form.as_ref().unwrap().fields[0], "tunnel_correctable");
    assert!(app.message.contains("key"));
}

#[test]
fn unknown_startup_is_never_displayed_as_off() {
    let mut app = fixture(Page::Home);
    app.startup = None;
    let output = screen(&app, 110, 32);
    assert!(output.contains("Start at sign-in: unknown"));
}
