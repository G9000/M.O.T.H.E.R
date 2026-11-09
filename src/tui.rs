//! Interactive terminal home; transport and storage stay behind the connection adapter.
#[cfg(test)]
mod tests;
mod view;

use crate::connection::{self, Settings, Startup, Status};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use mother::error::{Error, Result};
use std::{io::IsTerminal, path::PathBuf, sync::mpsc, time::Duration};
use zeroize::Zeroizing;

#[derive(Debug, Clone, Copy)]
pub enum Launch {
    Home,
    Setup,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Setup,
    ConfirmSave,
    Help,
    ConfirmStartup,
}

struct Form {
    fields: [String; 5],
    key: Zeroizing<String>,
    selected: usize,
}
impl Form {
    fn new(settings: Settings) -> Self {
        Self {
            fields: [
                settings.tunnel_id,
                settings.organization_id.unwrap_or_default(),
                settings.client_config.display().to_string(),
                settings.tunnel_client.display().to_string(),
                settings.mother_exe.display().to_string(),
            ],
            key: Zeroizing::new(String::new()),
            selected: 0,
        }
    }
    fn active(&mut self) -> &mut String {
        if self.selected == 5 {
            &mut self.key
        } else {
            &mut self.fields[self.selected]
        }
    }
    fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::Down => self.selected = (self.selected + 1) % 6,
            KeyCode::BackTab | KeyCode::Up => self.selected = (self.selected + 5) % 6,
            KeyCode::Backspace => {
                self.active().pop();
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.active().clear()
            }
            KeyCode::Char(c)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
                    && !c.is_control() =>
            {
                if self.active().len() + c.len_utf8() <= 4096 {
                    self.active().push(c);
                }
            }
            _ => {}
        }
    }
    fn settings(&self) -> Settings {
        Settings {
            version: 1,
            tunnel_id: self.fields[0].trim().into(),
            organization_id: (!self.fields[1].trim().is_empty())
                .then(|| self.fields[1].trim().to_owned()),
            client_config: PathBuf::from(self.fields[2].trim()),
            tunnel_client: PathBuf::from(self.fields[3].trim()),
            mother_exe: PathBuf::from(self.fields[4].trim()),
        }
    }
}

struct App {
    page: Page,
    form: Option<Form>,
    status: Option<Status>,
    message: String,
    busy: bool,
    startup: Option<bool>,
    workspace: String,
}
impl App {
    fn new() -> Result<Self> {
        let home = mother::config::data_directory()?;
        Ok(Self {
            page: Page::Home,
            form: None,
            status: None,
            message: "Checking your connection...".into(),
            busy: false,
            startup: connection::startup_enabled().ok(),
            workspace: if home.join("context.sqlite").is_file() {
                "Local workspace found".into()
            } else {
                "No default workspace yet - use mother init --soul <your file>".into()
            },
        })
    }
    fn open_setup(&mut self) -> Result<()> {
        let settings = match connection::load_settings()? {
            Some(s) => s,
            None => connection::suggested_settings()?,
        };
        self.form = Some(Form::new(settings));
        self.page = Page::Setup;
        self.message = "Use your existing tunnel and separate ChatGPT grant. Blank key keeps the saved credential.".into();
        Ok(())
    }
    fn finish(&mut self, result: Result<Status>) {
        self.busy = false;
        match result {
            Ok(status) => {
                if self.page == Page::ConfirmSave {
                    self.form = None;
                    self.page = Page::Home;
                }
                self.message = status.detail.clone();
                self.status = Some(status);
            }
            Err(error) => {
                self.message = error.to_string();
                if self.page == Page::ConfirmSave {
                    self.page = Page::Setup;
                    if let Some(form) = &mut self.form {
                        form.key = Zeroizing::new(String::new());
                    }
                    self.message
                        .push_str(" Your paths are kept; re-enter the key if it was not saved.");
                }
                self.status = None;
            }
        }
        self.startup = connection::startup_enabled().ok();
    }
}

enum Action {
    Refresh,
    Connect,
    Disconnect,
    Save(Settings, Zeroizing<String>),
    Startup(Startup),
}
fn work(action: Action) -> Result<Status> {
    match action {
        Action::Refresh => connection::status(),
        Action::Connect => connection::connect(),
        Action::Disconnect => connection::disconnect(),
        Action::Save(settings, key) => {
            connection::save_settings(&settings)?;
            if !key.trim().is_empty() {
                connection::store_key(&settings, key.trim())?;
            }
            connection::status()
        }
        Action::Startup(value) => {
            connection::set_startup(value)?;
            connection::status()
        }
    }
}
fn dispatch(app: &mut App, sender: &mpsc::Sender<Result<Status>>, action: Action, message: &str) {
    app.busy = true;
    app.message = message.into();
    let sender = sender.clone();
    std::thread::spawn(move || {
        let _ = sender.send(work(action));
    });
}

struct TerminalGuard;
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

/// Run the owner dashboard only when both input and output are real terminals.
pub fn run(launch: Launch) -> Result<()> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return Err(Error::domain(
            "TERMINAL_REQUIRED",
            "The TUI needs an interactive terminal. Use mother status --json for scripts.",
        ));
    }
    let mut app = App::new()?;
    if matches!(launch, Launch::Setup) {
        app.open_setup()?;
    }
    let mut terminal = ratatui::try_init()?;
    let _guard = TerminalGuard;
    let (sender, receiver) = mpsc::channel();
    dispatch(
        &mut app,
        &sender,
        Action::Refresh,
        "Checking connection state...",
    );
    loop {
        if let Ok(result) = receiver.try_recv() {
            app.finish(result);
        }
        terminal.draw(|frame| view::render(frame, &app))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        if app.busy {
            if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
            {
                app.message = "Waiting for the bounded connection check. An unfinished action will not be repeated automatically.".into();
            }
            continue;
        }
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            break;
        }
        match app.page {
            Page::Setup => match key.code {
                KeyCode::Esc => {
                    app.form = None;
                    app.page = Page::Home;
                    app.message = "Setup cancelled; nothing saved.".into();
                }
                KeyCode::Enter => {
                    app.page = Page::ConfirmSave;
                }
                _ => {
                    if let Some(form) = &mut app.form {
                        form.input(key);
                    }
                }
            },
            Page::ConfirmSave => match key.code {
                KeyCode::Esc => app.page = Page::Setup,
                KeyCode::Enter => {
                    if let Some(form) = &mut app.form {
                        let settings = form.settings();
                        let key = std::mem::take(&mut form.key);
                        dispatch(
                            &mut app,
                            &sender,
                            Action::Save(settings, key),
                            "Saving private settings and credential. Press c afterward to connect.",
                        );
                    }
                }
                _ => {}
            },
            Page::ConfirmStartup => match key.code {
                KeyCode::Esc => app.page = Page::Home,
                KeyCode::Enter => {
                    let action = if app.startup == Some(true) {
                        Startup::Disabled
                    } else {
                        Startup::Enabled
                    };
                    app.page = Page::Home;
                    dispatch(
                        &mut app,
                        &sender,
                        Action::Startup(action),
                        "Updating sign-in startup...",
                    );
                }
                _ => {}
            },
            Page::Help => match key.code {
                KeyCode::Esc | KeyCode::Char('h') | KeyCode::Char('q') => app.page = Page::Home,
                _ => {}
            },
            Page::Home => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('s') => {
                    if let Err(error) = app.open_setup() {
                        app.message = error.to_string();
                    }
                }
                KeyCode::Char('r') => dispatch(
                    &mut app,
                    &sender,
                    Action::Refresh,
                    "Checking connection state...",
                ),
                KeyCode::Char('c') => dispatch(
                    &mut app,
                    &sender,
                    Action::Connect,
                    "Connecting with your saved credential...",
                ),
                KeyCode::Char('d') => dispatch(
                    &mut app,
                    &sender,
                    Action::Disconnect,
                    "Stopping this workspace's connection...",
                ),
                KeyCode::Char('a') => match connection::startup_enabled() {
                    Ok(enabled) => {
                        app.startup = Some(enabled);
                        app.page = Page::ConfirmStartup;
                    }
                    Err(error) => {
                        app.startup = None;
                        app.message = error.to_string();
                    }
                },
                KeyCode::Char('h') | KeyCode::Char('?') => app.page = Page::Help,
                _ => {}
            },
        }
    }
    Ok(())
}
