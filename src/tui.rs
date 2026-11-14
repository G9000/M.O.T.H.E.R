//! Interactive terminal home; transport and storage stay behind the connection adapter.
mod animaos;
mod health;
mod inbox;
mod logo;
mod tend;
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
    Inbox,
    Animaos,
    Tend,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Setup,
    ConfirmSave,
    Help,
    ConfirmStartup,
    Inbox,
    Animaos,
    Health,
    Tend,
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
                    && !c.is_control()
                    && self.active().len() + c.len_utf8() <= 4096 =>
            {
                self.active().push(c);
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

/// One row on the home screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Row {
    App(crate::apps::AppId),
    Animaos,
    Chatgpt,
}
const ROWS: [Row; 6] = [
    Row::App(crate::apps::AppId::ClaudeCode),
    Row::App(crate::apps::AppId::Codex),
    Row::App(crate::apps::AppId::Hermes),
    Row::App(crate::apps::AppId::ClaudeDesktop),
    Row::Animaos,
    Row::Chatgpt,
];

/// Has AnimaOS been connected from this workspace before? (Local files only; no network.)
fn animaos_set_up() -> bool {
    mother::config::data_directory()
        .ok()
        .and_then(|d| std::fs::read_dir(d.join("clients")).ok())
        .is_some_and(|entries| {
            entries.filter_map(|e| e.ok()).any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name.starts_with("animaos-") && name.ends_with(".binding.json")
            })
        })
}

struct App {
    page: Page,
    apps: Vec<crate::apps::AppStatus>,
    row: usize,
    animaos_ready: bool,
    form: Option<Form>,
    status: Option<Status>,
    message: String,
    busy: bool,
    startup: Option<bool>,
    workspace: String,
    inbox: Option<inbox::Inbox>,
    tend: Option<tend::Tend>,
    animaos: Option<animaos::Setup>,
    health: Option<health::Health>,
}
impl App {
    fn new() -> Result<Self> {
        let home = mother::config::data_directory()?;
        Ok(Self {
            page: Page::Home,
            apps: crate::apps::statuses(&crate::apps::Paths::detect()),
            row: 0,
            animaos_ready: animaos_set_up(),
            inbox: None,
            tend: None,
            animaos: None,
            health: None,
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
    fn refresh_apps(&mut self) {
        self.apps = crate::apps::statuses(&crate::apps::Paths::detect());
        self.animaos_ready = animaos_set_up();
    }
    fn open_inbox(&mut self) -> Result<()> {
        let path = mother::config::data_directory()?.join("context.sqlite");
        self.inbox = Some(inbox::Inbox::open(path)?);
        self.page = Page::Inbox;
        Ok(())
    }
    fn open_tend(&mut self) -> Result<()> {
        let path = mother::config::data_directory()?.join("context.sqlite");
        self.tend = Some(tend::Tend::open(path)?);
        self.page = Page::Tend;
        Ok(())
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
/// Connect one app, or repair every broken one, off the UI thread.
fn dispatch_app(app: &mut App, sender: &mpsc::Sender<String>, target: Option<crate::apps::AppId>) {
    use crate::apps;
    app.busy = true;
    app.message = match target {
        Some(id) => format!("Connecting {}… this can take a few seconds.", id.name()),
        None => "Checking and repairing every app…".into(),
    };
    let sender = sender.clone();
    std::thread::spawn(move || {
        let result = (|| -> Result<String> {
            let (root, paths, exe) = (
                apps::workspace_root()?,
                apps::Paths::detect(),
                apps::stable_exe()?,
            );
            Ok(match target {
                Some(id) => apps::connect(&root, &paths, id, &exe, &apps::RealRunner)?,
                None => {
                    let lines = apps::fix(&root, &paths, &exe, &apps::RealRunner);
                    if lines.is_empty() {
                        "Nothing to fix: every connected app can read your memory.".into()
                    } else {
                        lines.join(" ")
                    }
                }
            })
        })();
        let _ = sender.send(result.unwrap_or_else(|e| e.to_string()));
    });
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
        let _ = crossterm::execute!(std::io::stdout(), event::DisableBracketedPaste);
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
    if matches!(launch, Launch::Inbox) {
        app.open_inbox()?;
    }
    if matches!(launch, Launch::Tend) {
        app.open_tend()?;
    }
    if matches!(launch, Launch::Animaos) {
        app.animaos = Some(animaos::Setup::with_local_key());
        app.page = Page::Animaos;
    }
    let mut terminal = ratatui::try_init()?;
    let _guard = TerminalGuard;
    crossterm::execute!(std::io::stdout(), event::EnableBracketedPaste)?;
    let (sender, receiver) = mpsc::channel();
    let (app_sender, app_receiver) = mpsc::channel::<String>();
    if !matches!(launch, Launch::Inbox | Launch::Animaos | Launch::Tend) {
        dispatch(
            &mut app,
            &sender,
            Action::Refresh,
            "Checking connection state...",
        );
    }
    loop {
        if let Some(health) = &mut app.health {
            health.poll();
        }
        if let Some(setup) = &mut app.animaos {
            setup.poll();
        }
        if let Ok(result) = receiver.try_recv() {
            app.finish(result);
        }
        if let Ok(message) = app_receiver.try_recv() {
            app.busy = false;
            app.message = message;
            app.refresh_apps();
        }
        terminal.draw(|frame| view::render(frame, &app))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        let key = match event::read()? {
            Event::Key(key) => key,
            Event::Paste(text) if !app.busy => {
                if app.page == Page::Animaos {
                    if let Some(setup) = &mut app.animaos {
                        setup.paste(&text);
                    }
                } else if app.page == Page::Inbox {
                    if let Some(inbox) = &mut app.inbox {
                        inbox.paste(&text);
                    }
                } else if app.page == Page::Setup
                    && let Some(form) = &mut app.form
                {
                    for c in text.chars().filter(|c| !c.is_control()) {
                        form.input(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
                    }
                }
                continue;
            }
            _ => continue,
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
            if app.page == Page::Animaos
                && app.animaos.as_ref().is_some_and(|setup| setup.is_busy())
            {
                continue;
            }
            break;
        }
        match app.page {
            Page::Health => {
                if app.health.as_mut().is_some_and(|health| health.input(key)) {
                    app.health = None;
                    app.page = Page::Home;
                }
            }
            Page::Animaos => {
                if let Some(setup) = &mut app.animaos {
                    match setup.input(key) {
                        Ok(true) => app.page = Page::Home,
                        Ok(false) => {}
                        Err(error) => setup.message = error.to_string(),
                    }
                }
            }
            Page::Tend => {
                if let Some(tend) = &mut app.tend {
                    match tend.input(key) {
                        Ok(true) => app.page = Page::Home,
                        Ok(false) => {}
                        Err(error) => tend.message = error.to_string(),
                    }
                }
            }
            Page::Inbox => {
                if let Some(inbox) = &mut app.inbox {
                    match inbox.input(key) {
                        Ok(true) => app.page = Page::Home,
                        Ok(false) => {}
                        Err(error) => inbox.message = error.to_string(),
                    }
                }
            }
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
                KeyCode::Up => app.row = app.row.saturating_sub(1),
                KeyCode::Down => app.row = (app.row + 1).min(ROWS.len() - 1),
                KeyCode::Enter => match ROWS[app.row] {
                    Row::App(id) => {
                        let installed = app
                            .apps
                            .iter()
                            .any(|s| s.app == id && s.state != crate::apps::State::NotInstalled);
                        if installed {
                            dispatch_app(&mut app, &app_sender, Some(id));
                        } else {
                            app.message =
                                format!("{} isn't installed on this computer.", id.name());
                        }
                    }
                    Row::Animaos => {
                        app.animaos = Some(animaos::Setup::with_local_key());
                        app.page = Page::Animaos;
                    }
                    Row::Chatgpt => {
                        let configured = connection::load_settings().ok().flatten().is_some();
                        if configured {
                            dispatch(
                                &mut app,
                                &sender,
                                Action::Connect,
                                "Connecting ChatGPT with your saved settings...",
                            );
                        } else if let Err(error) = app.open_setup() {
                            app.message = error.to_string();
                        }
                    }
                },
                KeyCode::Char('f') => dispatch_app(&mut app, &app_sender, None),
                KeyCode::Char('t') => {
                    app.health = Some(health::Health::open(&mother::config::data_directory()?));
                    app.page = Page::Health;
                }
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('n') => {
                    app.animaos = Some(animaos::Setup::with_local_key());
                    app.page = Page::Animaos;
                }
                KeyCode::Char('i') => {
                    if let Err(error) = app.open_inbox() {
                        app.message = error.to_string();
                    }
                }
                KeyCode::Char('p') => {
                    if let Err(error) = app.open_tend() {
                        app.message = error.to_string();
                    }
                }
                KeyCode::Char('s') => {
                    if let Err(error) = app.open_setup() {
                        app.message = error.to_string();
                    }
                }
                KeyCode::Char('r') => {
                    app.refresh_apps();
                    dispatch(
                        &mut app,
                        &sender,
                        Action::Refresh,
                        "Checking connection state...",
                    )
                }
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
