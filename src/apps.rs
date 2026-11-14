//! Friendly app connections: find the agent apps on this computer, show whether each can
//! read M.O.T.H.E.R, and connect one in a single step.
//!
//! Status is read from each app's own settings file (fast, no process launch). Changes go
//! through the app's own `mcp add` command where it has one, so M.O.T.H.E.R never rewrites
//! another program's settings format by hand. Claude Desktop has no such command; its JSON
//! file is backed up before the single `mother` entry is replaced.

use mother::{
    config::{data_directory, load_config},
    error::{Error, Result},
    store::Store,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

/// The scopes behind "Knows me": full context, plus checkpoints, tasks and memory proposals
/// that wait in the owner's review inbox. No direct memory writes.
pub const KNOWS_ME_SCOPES: [&str; 4] =
    ["read", "context:propose", "checkpoint:write", "task:write"];
const SERVER: &str = "mother";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppId {
    ClaudeCode,
    Codex,
    Hermes,
    ClaudeDesktop,
}

impl AppId {
    pub const ALL: [AppId; 4] = [
        AppId::ClaudeCode,
        AppId::Codex,
        AppId::Hermes,
        AppId::ClaudeDesktop,
    ];

    /// The grant name and the word used on the command line.
    pub fn key(self) -> &'static str {
        match self {
            AppId::ClaudeCode => "claude-code",
            AppId::Codex => "codex",
            AppId::Hermes => "hermes",
            AppId::ClaudeDesktop => "claude-desktop",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            AppId::ClaudeCode => "Claude Code",
            AppId::Codex => "Codex",
            AppId::Hermes => "Hermes",
            AppId::ClaudeDesktop => "Claude Desktop",
        }
    }
    fn program(self) -> Option<&'static str> {
        match self {
            AppId::ClaudeCode => Some("claude"),
            AppId::Codex => Some("codex"),
            AppId::Hermes => Some("hermes"),
            AppId::ClaudeDesktop => None,
        }
    }
}

/// Where an app's own settings live. Environment overrides keep tests away from real files.
pub struct Paths {
    pub claude_code: PathBuf,
    pub codex: PathBuf,
    pub hermes: PathBuf,
    pub claude_desktop: PathBuf,
}

impl Paths {
    pub fn detect() -> Self {
        let home = std::env::var_os("USERPROFILE")
            .or_else(|| std::env::var_os("HOME"))
            .map(PathBuf::from)
            .unwrap_or_default();
        let env = |name: &str| std::env::var_os(name).map(PathBuf::from);
        let hermes_home = env("HERMES_HOME").unwrap_or_else(|| {
            let local = env("LOCALAPPDATA").map(|p| p.join("hermes"));
            match local {
                Some(dir) if dir.join("config.yaml").is_file() => dir,
                _ => home.join(".hermes"),
            }
        });
        Self {
            claude_code: env("MOTHER_CLAUDE_CODE_CONFIG")
                .unwrap_or_else(|| home.join(".claude.json")),
            codex: env("CODEX_HOME")
                .unwrap_or_else(|| home.join(".codex"))
                .join("config.toml"),
            hermes: hermes_home.join("config.yaml"),
            claude_desktop: env("MOTHER_CLAUDE_DESKTOP_CONFIG").unwrap_or_else(|| {
                env("APPDATA")
                    .unwrap_or_else(|| home.join(".config"))
                    .join("Claude")
                    .join("claude_desktop_config.json")
            }),
        }
    }
    fn of(&self, app: AppId) -> &Path {
        match app {
            AppId::ClaudeCode => &self.claude_code,
            AppId::Codex => &self.codex,
            AppId::Hermes => &self.hermes,
            AppId::ClaudeDesktop => &self.claude_desktop,
        }
    }
}

/// How an app currently launches M.O.T.H.E.R.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub command: String,
    pub args: Vec<String>,
}

impl Registration {
    pub fn config(&self) -> Option<PathBuf> {
        let i = self.args.iter().position(|a| a == "--config")?;
        self.args.get(i + 1).map(PathBuf::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// The app isn't on this computer.
    NotInstalled,
    /// Installed, but it has no M.O.T.H.E.R entry.
    NotConnected,
    /// Its entry points at a working executable and a valid grant.
    Working,
    /// Its entry exists but can't work; the text says why in plain words.
    Broken(String),
}

#[derive(Debug, Clone)]
pub struct AppStatus {
    pub app: AppId,
    pub state: State,
    pub registration: Option<Registration>,
}

impl AppStatus {
    pub fn summary(&self) -> String {
        match &self.state {
            State::NotInstalled => "not on this computer".into(),
            State::NotConnected => "installed, not connected".into(),
            State::Working => "connected".into(),
            State::Broken(why) => format!("needs fixing: {why}"),
        }
    }
}

// ---------- reading each app's settings ----------

fn read_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn json_entry(path: &Path) -> Option<Registration> {
    let value: Value = serde_json::from_str(&read_text(path)?).ok()?;
    let entry = value.get("mcpServers")?.get(SERVER)?;
    Some(Registration {
        command: entry.get("command")?.as_str()?.to_owned(),
        args: entry
            .get("args")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default(),
    })
}

/// Quoted strings from a TOML value: "basic" (with escapes) or 'literal'.
fn toml_strings(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                let mut s = String::new();
                while let Some(c) = chars.next() {
                    match c {
                        '\\' => match chars.next() {
                            Some('n') => s.push('\n'),
                            Some('t') => s.push('\t'),
                            Some(other) => s.push(other),
                            None => {}
                        },
                        '"' => break,
                        other => s.push(other),
                    }
                }
                out.push(s);
            }
            '\'' => out.push(chars.by_ref().take_while(|c| *c != '\'').collect()),
            _ => {}
        }
    }
    out
}

pub fn codex_entry(text: &str) -> Option<Registration> {
    let mut inside = false;
    let (mut command, mut args) = (None, Vec::new());
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            if inside {
                break;
            }
            inside = trimmed == "[mcp_servers.mother]" || trimmed == "[mcp_servers.\"mother\"]";
            continue;
        }
        if !inside {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("command") {
            command = toml_strings(rest.trim_start().trim_start_matches('='))
                .into_iter()
                .next();
        } else if let Some(rest) = trimmed.strip_prefix("args") {
            args = toml_strings(rest.trim_start().trim_start_matches('='));
        }
    }
    Some(Registration {
        command: command?,
        args,
    })
}

fn yaml_scalar(text: &str) -> String {
    let t = text.trim();
    if let Some(inner) = t.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        inner.replace("\\\\", "\\").replace("\\\"", "\"")
    } else if let Some(inner) = t.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        inner.replace("''", "'")
    } else {
        t.to_owned()
    }
}

pub fn hermes_entry(text: &str) -> Option<Registration> {
    let lines: Vec<&str> = text.lines().collect();
    let mut in_servers = false;
    let mut start = None;
    for (i, line) in lines.iter().enumerate() {
        if !line.starts_with(' ') && !line.trim().is_empty() {
            in_servers = line.trim_end() == "mcp_servers:";
            continue;
        }
        if in_servers && line.trim_end() == format!("  {SERVER}:") {
            start = Some(i + 1);
            break;
        }
    }
    let start = start?;
    let (mut command, mut args, mut in_args) = (None, Vec::new(), false);
    for line in &lines[start..] {
        let indent = line.len() - line.trim_start().len();
        if !line.trim().is_empty() && indent <= 2 {
            break;
        }
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("command:") {
            command = Some(yaml_scalar(rest));
            in_args = false;
        } else if let Some(rest) = t.strip_prefix("args:") {
            let rest = rest.trim();
            if let Some(inline) = rest.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
                args = inline
                    .split(',')
                    .map(yaml_scalar)
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            in_args = rest.is_empty();
        } else if in_args && let Some(item) = t.strip_prefix("- ") {
            args.push(yaml_scalar(item));
        } else if !t.is_empty() {
            in_args = false;
        }
    }
    Some(Registration {
        command: command?,
        args,
    })
}

pub fn registration(paths: &Paths, app: AppId) -> Option<Registration> {
    let path = paths.of(app);
    match app {
        AppId::ClaudeCode | AppId::ClaudeDesktop => json_entry(path),
        AppId::Codex => codex_entry(&read_text(path)?),
        AppId::Hermes => hermes_entry(&read_text(path)?),
    }
}

/// The full path of a program on PATH, trying Windows launcher extensions too.
pub fn find_program(name: &str) -> Option<PathBuf> {
    let exts: &[&str] = if cfg!(windows) {
        &[".exe", ".cmd", ".bat", ""]
    } else {
        &[""]
    };
    std::env::split_paths(&std::env::var_os("PATH")?).find_map(|dir| {
        exts.iter()
            .map(|ext| dir.join(format!("{name}{ext}")))
            .find(|p| p.is_file())
    })
}

fn installed(paths: &Paths, app: AppId) -> bool {
    app.program().is_some_and(|p| find_program(p).is_some())
        || paths.of(app).is_file()
        || (app == AppId::ClaudeDesktop && paths.of(app).parent().is_some_and(Path::is_dir))
}

/// Can this grant read the workspace right now? Plain-language reason if not.
pub fn grant_problem(config: &Path) -> Option<String> {
    let config = match load_config(config) {
        Ok(c) => c,
        Err(_) => return Some("its access file is missing or unreadable".into()),
    };
    let store = match Store::open(&config.database_path) {
        Ok(s) => s,
        Err(_) => return Some("its memory database can't be opened".into()),
    };
    match config.authorize(&store) {
        Ok(()) => None,
        Err(_) => Some("its access was revoked or is no longer valid".into()),
    }
}

pub fn status(paths: &Paths, app: AppId) -> AppStatus {
    let registration = registration(paths, app);
    let state = match &registration {
        None if !installed(paths, app) => State::NotInstalled,
        None => State::NotConnected,
        Some(r) if !Path::new(&r.command).is_file() => {
            State::Broken("it points at a M.O.T.H.E.R program that no longer exists".into())
        }
        Some(r) => match r.config() {
            None => State::Broken("its entry has no access file".into()),
            Some(config) => grant_problem(&config).map_or(State::Working, State::Broken),
        },
    };
    AppStatus {
        app,
        state,
        registration,
    }
}

pub fn statuses(paths: &Paths) -> Vec<AppStatus> {
    AppId::ALL.iter().map(|&a| status(paths, a)).collect()
}

// ---------- connecting ----------

/// Runs another program; swapped out in tests.
pub trait Runner {
    fn run(&self, program: &Path, args: &[String]) -> Result<()>;
}

pub struct RealRunner;

impl Runner for RealRunner {
    fn run(&self, program: &Path, args: &[String]) -> Result<()> {
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| {
                Error::domain("CONNECT", format!("Could not start {}.", program.display()))
            })?;
        use wait_timeout::ChildExt;
        match child.wait_timeout(Duration::from_secs(90))? {
            Some(status) if status.success() => Ok(()),
            Some(_) => {
                let mut err = String::new();
                if let Some(mut e) = child.stderr.take() {
                    use std::io::Read;
                    let _ = e.read_to_string(&mut err);
                }
                let first = err
                    .lines()
                    .find(|l| !l.trim().is_empty())
                    .unwrap_or("no details");
                Err(Error::domain(
                    "CONNECT",
                    format!("{} refused the change: {first}", program.display()),
                ))
            }
            None => {
                let _ = child.kill();
                Err(Error::domain(
                    "CONNECT",
                    format!("{} took too long and was stopped.", program.display()),
                ))
            }
        }
    }
}

/// Reuse a working access file for this app, or create a new "Knows me" grant.
/// Returns the access file path and whether it was newly created.
pub fn ensure_grant(
    root: &Path,
    app: AppId,
    current: Option<&Registration>,
) -> Result<(PathBuf, bool)> {
    if let Some(config) = current.and_then(Registration::config)
        && config.is_file()
        && grant_problem(&config).is_none()
    {
        return Ok((config, false));
    }
    let clients = root.join("clients");
    let preferred = clients.join(format!("{}.private.json", app.key()));
    if preferred.is_file() && grant_problem(&preferred).is_none() {
        return Ok((preferred, false));
    }
    let database = root.join("context.sqlite");
    let mut store = Store::open(&database).map_err(|_| {
        Error::domain("CONNECT", "M.O.T.H.E.R has no memory workspace yet. Run `mother init --soul <your SOUL.md>` first.")
    })?;
    fs::create_dir_all(&clients)?;
    let scopes: Vec<String> = KNOWS_ME_SCOPES.iter().map(|s| (*s).to_owned()).collect();
    for n in 1..=20 {
        let name = if n == 1 {
            app.key().to_owned()
        } else {
            format!("{}-{n}", app.key())
        };
        let path = clients.join(format!("{name}.private.json"));
        if path.exists() {
            continue;
        }
        if store
            .grant_to_file(&name, &scopes, &path, &database)
            .is_ok()
        {
            return Ok((path, true));
        }
    }
    Err(Error::domain(
        "CONNECT",
        "Could not create a new access file for this app.",
    ))
}

fn backup(root: &Path, file: &Path) -> Result<Option<PathBuf>> {
    if !file.is_file() {
        return Ok(None);
    }
    let dir = root.join("setup-backups");
    fs::create_dir_all(&dir)?;
    let stamp = chrono::Utc::now().format("%Y%m%dT%H%M%S%3f");
    let name = file
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let target = dir.join(format!("{stamp}-{name}"));
    fs::copy(file, &target)?;
    Ok(Some(target))
}

fn write_claude_desktop(root: &Path, path: &Path, exe: &str, config: &str) -> Result<()> {
    let mut value: Value = match read_text(path) {
        Some(text) => serde_json::from_str(&text).map_err(|_| {
            Error::domain(
                "CONNECT",
                "Claude Desktop's settings file isn't valid JSON, so it was left alone.",
            )
        })?,
        None => json!({}),
    };
    if !value.is_object() {
        return Err(Error::domain(
            "CONNECT",
            "Claude Desktop's settings file has an unexpected shape; left alone.",
        ));
    }
    backup(root, path)?;
    let servers = value
        .as_object_mut()
        .expect("checked object")
        .entry("mcpServers")
        .or_insert_with(|| json!({}));
    let Some(servers) = servers.as_object_mut() else {
        return Err(Error::domain(
            "CONNECT",
            "Claude Desktop's mcpServers entry has an unexpected shape; left alone.",
        ));
    };
    servers.insert(
        SERVER.into(),
        json!({"command": exe, "args": ["mcp", "--config", config]}),
    );
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut tmp = tempfile::NamedTempFile::new_in(path.parent().unwrap_or(Path::new(".")))?;
    tmp.write_all(serde_json::to_string_pretty(&value)?.as_bytes())?;
    tmp.write_all(b"\n")?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| Error::from(e.error))?;
    Ok(())
}

/// Connect one app: reuse or create its access, register M.O.T.H.E.R in the app, then check
/// that the entry reads back and the access works. Returns a plain-language result.
pub fn connect(
    root: &Path,
    paths: &Paths,
    app: AppId,
    exe: &Path,
    runner: &dyn Runner,
) -> Result<String> {
    let before = status(paths, app);
    if before.state == State::NotInstalled {
        return Err(Error::domain(
            "CONNECT",
            format!("{} isn't installed on this computer.", app.name()),
        ));
    }
    let (config, created) = ensure_grant(root, app, before.registration.as_ref())?;
    let exe_text = exe.to_string_lossy().to_string();
    let config_text = config.to_string_lossy().to_string();
    let want = Registration {
        command: exe_text.clone(),
        args: vec!["mcp".into(), "--config".into(), config_text.clone()],
    };

    if before.state != State::Working || before.registration.as_ref() != Some(&want) {
        match app.program() {
            Some(name) => {
                let program = find_program(name).ok_or_else(|| {
                    Error::domain(
                        "CONNECT",
                        format!(
                            "Couldn't find the `{name}` command to update {}.",
                            app.name()
                        ),
                    )
                })?;
                if app == AppId::Codex || app == AppId::Hermes {
                    backup(root, paths.of(app))?;
                }
                let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
                let (remove, add) = match app {
                    AppId::ClaudeCode => (
                        s(&["mcp", "remove", SERVER, "-s", "user"]),
                        s(&[
                            "mcp",
                            "add",
                            "-s",
                            "user",
                            SERVER,
                            "--",
                            &exe_text,
                            "mcp",
                            "--config",
                            &config_text,
                        ]),
                    ),
                    AppId::Codex => (
                        s(&["mcp", "remove", SERVER]),
                        s(&[
                            "mcp",
                            "add",
                            SERVER,
                            "--",
                            &exe_text,
                            "mcp",
                            "--config",
                            &config_text,
                        ]),
                    ),
                    AppId::Hermes => (
                        s(&["mcp", "remove", SERVER]),
                        s(&[
                            "mcp",
                            "add",
                            SERVER,
                            "--command",
                            &exe_text,
                            "--args",
                            "mcp",
                            "--config",
                            &config_text,
                        ]),
                    ),
                    AppId::ClaudeDesktop => unreachable!("no program"),
                };
                if before.registration.is_some() {
                    runner.run(&program, &remove)?;
                }
                runner.run(&program, &add)?;
            }
            None => write_claude_desktop(root, paths.of(app), &exe_text, &config_text)?,
        }
    }

    let after = status(paths, app);
    match after.state {
        State::Working => Ok(format!(
            "{} is connected{}. Restart {} if it was open.",
            app.name(),
            if created { " with new access" } else { "" },
            app.name()
        )),
        State::Broken(why) => Err(Error::domain(
            "CONNECT",
            format!("{} was updated, but {why}.", app.name()),
        )),
        _ => Err(Error::domain(
            "CONNECT",
            format!(
                "{} didn't keep the new entry. Try again, or check its settings.",
                app.name()
            ),
        )),
    }
}

/// Reconnect every app whose entry is broken. Returns one line per app it touched.
pub fn fix(root: &Path, paths: &Paths, exe: &Path, runner: &dyn Runner) -> Vec<String> {
    statuses(paths)
        .into_iter()
        .filter_map(|s| match &s.state {
            State::Broken(why) => Some(match connect(root, paths, s.app, exe, runner) {
                Ok(done) => format!("Fixed {} ({why}). {done}", s.app.name()),
                Err(e) => format!("Couldn't fix {}: {e}", s.app.name()),
            }),
            _ => None,
        })
        .collect()
}

/// The program path to register: this executable, unless it is a development build.
pub fn stable_exe() -> Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let exe = fs::canonicalize(&exe).unwrap_or(exe);
    let text = exe.to_string_lossy();
    let clean = text
        .strip_prefix(r"\\?\")
        .map(PathBuf::from)
        .unwrap_or_else(|| exe.clone());
    Ok(clean)
}

pub fn workspace_root() -> Result<PathBuf> {
    data_directory()
}

#[cfg(test)]
mod tests;
