//! Native setup UI for the optional loopback AnimaOS adapter.
#[cfg(test)]
mod tests;
use crate::animaos::{Agent, Session};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mother::error::Result;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::sync::mpsc::{self, Receiver, Sender};
use zeroize::{Zeroize, Zeroizing};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Credentials,
    Agents,
    Confirm,
    Result,
}
enum Reply {
    Agents(Session, Result<Vec<Agent>>),
    Connected(Session, Result<String>),
}
pub(super) struct Setup {
    stage: Stage,
    endpoint: String,
    key: Zeroizing<String>,
    api_key: Zeroizing<String>,
    selected: usize,
    agents: Vec<Agent>,
    session: Option<Session>,
    sender: Sender<Reply>,
    receiver: Receiver<Reply>,
    busy: bool,
    root: std::path::PathBuf,
    exe: std::path::PathBuf,
    workspace: String,
    generation: String,
    scroll: u16,
    pub(super) message: String,
}
impl Setup {
    pub(super) fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            stage: Stage::Credentials,
            endpoint: "http://127.0.0.1:8080".into(),
            key: Zeroizing::new(String::new()),
            api_key: Zeroizing::new(String::new()),
            selected: 0,
            agents: vec![],
            session: None,
            sender,
            receiver,
            busy: false,
            root: mother::config::data_directory().unwrap_or_default(),
            exe: std::env::current_exe().unwrap_or_default(),
            workspace: String::new(),
            generation: String::new(),
            scroll: 0,
            message:
                "Start AnimaOS, then load your agents. Owner credentials stay only in this session."
                    .into(),
        }
    }
    pub(super) fn is_busy(&self) -> bool {
        self.busy
    }
    pub(super) fn poll(&mut self) {
        while let Ok(reply) = self.receiver.try_recv() {
            self.busy = false;
            match reply {
                Reply::Agents(session, result) => {
                    self.session = Some(session);
                    match result {
                        Ok(agents) => {
                            self.key.zeroize();
                            self.api_key.zeroize();
                            self.agents = agents;
                            self.selected = 0;
                            self.stage = Stage::Agents;
                            self.message = if self.agents.is_empty() {
                                "No agents found. Create an agent in AnimaOS, then return and load again.".into()
                            } else {
                                "Choose the AnimaOS agent that should use M.O.T.H.E.R.".into()
                            };
                        }
                        Err(error) => {
                            self.session = None;
                            self.stage = Stage::Credentials;
                            self.message = error.to_string();
                        }
                    }
                }
                Reply::Connected(session, result) => {
                    self.session = Some(session);
                    self.stage = Stage::Result;
                    self.message = match result {
                        Ok(status) => status,
                        Err(error) => format!(
                            "{error} Return to the agent list to inspect/reconcile before another connection attempt."
                        ),
                    };
                }
            }
        }
    }
    pub(super) fn input(&mut self, key: KeyEvent) -> Result<bool> {
        if self.busy {
            return Ok(false);
        }
        match self.stage {
            Stage::Credentials => match key.code {
                KeyCode::Esc => {
                    self.key.zeroize();
                    self.api_key.zeroize();
                    self.session = None;
                    return Ok(true);
                }
                KeyCode::Tab | KeyCode::Down => self.selected = (self.selected + 1) % 3,
                KeyCode::BackTab | KeyCode::Up => self.selected = (self.selected + 2) % 3,
                KeyCode::Enter => {
                    let session = if self.api_key.is_empty() {
                        Session::new(self.endpoint.trim(), self.key.trim())?
                    } else {
                        Session::with_api_key(
                            self.endpoint.trim(),
                            self.key.trim(),
                            self.api_key.trim(),
                        )?
                    };
                    self.busy = true;
                    self.message = "Loading AnimaOS agents…".into();
                    let sender = self.sender.clone();
                    std::thread::spawn(move || {
                        let result = session.agents();
                        let _ = sender.send(Reply::Agents(session, result));
                    });
                }
                KeyCode::Backspace => {
                    if self.selected == 0 {
                        self.endpoint.pop();
                    } else if self.selected == 1 {
                        self.key.pop();
                    } else {
                        self.api_key.pop();
                    }
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if self.selected == 0 {
                        self.endpoint.clear();
                    } else if self.selected == 1 {
                        self.key.zeroize();
                    } else {
                        self.api_key.zeroize();
                    }
                }
                KeyCode::Char(c)
                    if !key
                        .modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
                {
                    self.paste(&c.to_string())
                }
                _ => {}
            },
            Stage::Agents => match key.code {
                KeyCode::Esc => {
                    self.session = None;
                    self.key.zeroize();
                    self.api_key.zeroize();
                    self.selected = 0;
                    self.stage = Stage::Credentials;
                }
                KeyCode::Up => self.selected = self.selected.saturating_sub(1),
                KeyCode::Down => {
                    self.selected = (self.selected + 1).min(self.agents.len().saturating_sub(1))
                }
                KeyCode::Enter if !self.agents.is_empty() => {
                    self.root = self.root.canonicalize()?;
                    self.exe = self.exe.canonicalize()?;
                    let workspace = mother::store::Store::open(&self.root.join("context.sqlite"))?
                        .workspace()?;
                    self.workspace = workspace["id"].as_str().unwrap_or_default().into();
                    self.generation = workspace["generation"].as_str().unwrap_or_default().into();
                    self.scroll = 0;
                    self.stage = Stage::Confirm;
                }
                _ => {}
            },
            Stage::Confirm => {
                match key.code {
                    KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
                    KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
                    KeyCode::Esc => self.stage = Stage::Agents,
                    KeyCode::Enter => {
                        let Some(agent) = self.agents.get(self.selected) else {
                            return Ok(false);
                        };
                        let id = agent.id.clone();
                        let root = self.root.clone();
                        let exe = self.exe.clone();
                        let current = mother::store::Store::open(&root.join("context.sqlite"))?
                            .workspace()?;
                        if current["id"].as_str() != Some(self.workspace.as_str())
                            || current["generation"].as_str() != Some(self.generation.as_str())
                        {
                            return Err(mother::error::Error::domain(
                                "CONNECTION",
                                "Workspace changed; return to the agent list and review again.",
                            ));
                        }
                        let Some(session) = self.session.take() else {
                            return Ok(false);
                        };
                        self.busy = true;
                        self.message="Connecting M.O.T.H.E.R. Keep this window open until the result is known.".into();
                        let sender = self.sender.clone();
                        std::thread::spawn(move || {
                            let result = session.connect(&id, &root, &exe);
                            let _ = sender.send(Reply::Connected(session, result));
                        });
                    }
                    _ => {}
                }
            }
            Stage::Result => match key.code {
                KeyCode::Esc | KeyCode::Enter => self.stage = Stage::Agents,
                _ => {}
            },
        }
        Ok(false)
    }
    pub(super) fn paste(&mut self, text: &str) {
        if self.busy || self.stage != Stage::Credentials {
            return;
        }
        let field = if self.selected == 0 {
            &mut self.endpoint
        } else if self.selected == 1 {
            &mut self.key
        } else {
            &mut self.api_key
        };
        let limit = if self.selected == 0 { 512 } else { 8192 };
        for c in text.chars().filter(|c| !c.is_control()) {
            if field.len() + c.len_utf8() > limit {
                break;
            }
            field.push(c);
        }
    }
    pub(super) fn hints(&self) -> &'static str {
        if self.busy {
            return "Working… no automatic retries. Please wait for the result.";
        }
        match self.stage {
            Stage::Credentials => {
                "Tab / arrows field   Ctrl+U clear   Enter load agents   Esc home"
            }
            Stage::Agents => "↑↓ choose agent   Enter review connection   Esc account",
            Stage::Confirm => "↑↓ scroll details   Enter connect this agent   Esc back",
            Stage::Result => "Enter / Esc agent list",
        }
    }
    pub(super) fn render(&self, frame: &mut Frame, area: Rect) {
        let safe = |s: &str| s.chars().filter(|c| !c.is_control()).collect::<String>();
        let mut lines = vec![
            "Connect your AnimaOS agent to shared context.".to_string(),
            String::new(),
        ];
        match self.stage {
            Stage::Credentials => {
                lines.push(format!(
                    "{} AnimaOS address",
                    if self.selected == 0 { ">" } else { " " }
                ));
                lines.push(safe(&self.endpoint));
                lines.push(format!(
                    "{} AnimaOS owner token (kept only in memory)",
                    if self.selected == 1 { ">" } else { " " }
                ));
                lines.push(if self.key.is_empty() {
                    "(empty)".into()
                } else {
                    "••••••••".into()
                });
                lines.push(format!(
                    "{} API key (optional, if enabled in AnimaOS)",
                    if self.selected == 2 { ">" } else { " " }
                ));
                lines.push(if self.api_key.is_empty() {
                    "(empty)".into()
                } else {
                    "••••••••".into()
                });
                lines.push(String::new());
                lines.push("Use the owner token from your AnimaOS setup. Paste credentials here, never into chat. Both stay only in this session.".into());
            }
            Stage::Agents => {
                if self.agents.is_empty() {
                    lines.push("No agents available.".into());
                }
                let visible = usize::from(area.height.saturating_sub(6)).max(1);
                let start = self.selected.saturating_sub(visible - 1);
                for (index, agent) in self.agents.iter().enumerate().skip(start).take(visible) {
                    let label = format!(
                        "{} {}",
                        if index == self.selected { ">" } else { " " },
                        safe(&agent.name)
                    );
                    lines.push(
                        label
                            .chars()
                            .take(usize::from(area.width.saturating_sub(4)) / 2)
                            .collect(),
                    );
                }
            }
            Stage::Confirm => {
                if let Some(agent) = self.agents.get(self.selected) {
                    lines.push(format!(
                        "Agent: {} ({})",
                        safe(&agent.name),
                        safe(&agent.id)
                    ));
                }
                lines.push(format!("AnimaOS: {}", safe(&self.endpoint)));
                lines.push(format!(
                    "Executable: {}",
                    safe(&self.exe.display().to_string())
                ));
                lines.push(format!("Workspace: {}", safe(&self.workspace)));
                lines.push(format!("Data: {}", safe(&self.root.display().to_string())));
                lines.push(
                    "Access: read shared context + propose memories for owner review.".into(),
                );
                lines.push("M.O.T.H.E.R runs locally using this executable. AnimaOS keeps its own agent permissions.".into());
                lines.push("Enter creates/reuses a separate grant and starts this agent's local MCP connection.".into());
            }
            Stage::Result => {
                lines.push("Connection result".into());
                lines.push(String::new());
                lines.push(safe(&self.message));
            }
        }
        frame.render_widget(
            Paragraph::new(lines.join("\n"))
                .wrap(Wrap { trim: false })
                .scroll((
                    if self.stage == Stage::Confirm {
                        self.scroll
                    } else {
                        0
                    },
                    0,
                ))
                .block(
                    Block::default()
                        .title(" AnimaOS connection ")
                        .borders(Borders::ALL),
                ),
            area,
        );
    }
}
