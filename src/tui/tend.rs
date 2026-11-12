//! Owner tending screen: see the pinned core, pin or unpin, and hand tasks to harnesses.
#[cfg(test)]
mod tests;

use super::view::safe;
#[cfg(test)]
use crossterm::event::KeyModifiers;
use crossterm::event::{KeyCode, KeyEvent};
use mother::{error::Result, store::Store};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};

const INK: Color = Color::Rgb(236, 227, 216);
const MUTED: Color = Color::Rgb(162, 150, 173);
const VIOLET: Color = Color::Rgb(188, 160, 215);
const GOLD: Color = Color::Rgb(225, 188, 139);
const NOBODY: &str = "Nobody (clear the handoff)";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Pinned,
    Memories,
    Decisions,
    Tasks,
}
const TABS: [Tab; 4] = [Tab::Pinned, Tab::Memories, Tab::Decisions, Tab::Tasks];

struct Row {
    id: String,
    kind: String,
    title: String,
    status: String,
    excerpt: String,
    truncated: bool,
    actor: String,
    updated: String,
    recallable: bool,
    pinned: Option<String>,
    assignee: Option<String>,
    assigned_by: Option<String>,
}
impl Row {
    fn open_task(&self) -> bool {
        self.kind == "task" && matches!(self.status.as_str(), "todo" | "doing" | "blocked")
    }
}

pub(super) struct Tend {
    path: PathBuf,
    tab: Tab,
    rows: Vec<Row>,
    omitted: u64,
    pin_limit: u64,
    harnesses: Vec<String>,
    selected: usize,
    /// The highlighted entry while choosing who takes a task; 0 is nobody.
    choosing: Option<usize>,
    pub(super) message: String,
}
fn text(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or_default().to_owned()
}
impl Tend {
    pub(super) fn open(path: PathBuf) -> Result<Self> {
        let mut tend = Self {
            path,
            tab: Tab::Pinned,
            rows: vec![],
            omitted: 0,
            pin_limit: 12,
            harnesses: vec![],
            selected: 0,
            choosing: None,
            message: "The pinned core is what every harness reads first.".into(),
        };
        tend.reload()?;
        Ok(tend)
    }
    fn reload(&mut self) -> Result<()> {
        let mut store = Store::open(&self.path)?;
        let records = store.owner_records(serde_json::json!({"limit":500}))?;
        let pins = store.owner_pins()?;
        let handoffs = store.owner_handoffs()?;
        self.pin_limit = pins["limit"].as_u64().unwrap_or(12);
        let pinned_by: HashMap<String, String> = pins["pins"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|p| (text(p, "id"), text(p, "pinnedBy")))
            .collect();
        let assigned_by: HashMap<String, String> = handoffs["handoffs"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|h| (text(h, "id"), text(h, "assignedBy")))
            .collect();
        self.harnesses = handoffs["harnesses"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|h| h.as_str().map(str::to_owned))
            .collect();
        self.omitted = records["omitted"].as_u64().unwrap_or(0);
        self.rows = records["records"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|r| {
                let id = text(r, "id");
                Row {
                    kind: text(r, "kind"),
                    title: text(r, "title"),
                    status: text(r, "status"),
                    excerpt: text(r, "excerpt"),
                    truncated: r["truncated"] == true,
                    actor: text(r, "actor"),
                    updated: text(r, "updatedAt"),
                    recallable: r["recallable"] == true,
                    pinned: pinned_by.get(&id).cloned(),
                    assignee: r["assignee"].as_str().map(str::to_owned),
                    assigned_by: assigned_by.get(&id).cloned(),
                    id,
                }
            })
            .collect();
        self.selected = self.selected.min(self.visible().len().saturating_sub(1));
        Ok(())
    }
    fn in_tab(&self, tab: Tab, row: &Row) -> bool {
        match tab {
            Tab::Pinned => row.pinned.is_some(),
            Tab::Memories => row.kind == "memory" && row.recallable,
            Tab::Decisions => row.kind == "decision" && row.recallable,
            Tab::Tasks => row.kind == "task",
        }
    }
    /// Rows of the current tab; open tasks come before finished ones.
    fn visible(&self) -> Vec<&Row> {
        let mut rows: Vec<&Row> = self
            .rows
            .iter()
            .filter(|r| self.in_tab(self.tab, r))
            .collect();
        if self.tab == Tab::Tasks {
            rows.sort_by_key(|r| !r.open_task());
        }
        rows
    }
    fn count(&self, tab: Tab) -> usize {
        self.rows.iter().filter(|r| self.in_tab(tab, r)).count()
    }
    fn current(&self) -> Option<&Row> {
        self.visible().get(self.selected).copied()
    }
    fn choices(&self) -> Vec<&str> {
        std::iter::once(NOBODY)
            .chain(self.harnesses.iter().map(String::as_str))
            .collect()
    }
    fn switch(&mut self, step: isize) {
        let at = TABS.iter().position(|t| *t == self.tab).unwrap_or(0) as isize;
        self.tab = TABS[(at + step).rem_euclid(TABS.len() as isize) as usize];
        self.selected = 0;
    }
    fn toggle_pin(&mut self) -> Result<()> {
        let Some(row) = self.current() else {
            self.message = "Nothing is selected.".into();
            return Ok(());
        };
        let (id, title, pin) = (row.id.clone(), row.title.clone(), row.pinned.is_none());
        match Store::open(&self.path)?.owner_pin(&id, pin) {
            Ok(_) => {
                self.message = if pin {
                    format!("Pinned “{title}”. Every harness now reads it first.")
                } else {
                    format!("Unpinned “{title}”. It stays saved as an ordinary record.")
                };
                self.reload()?;
            }
            Err(error) => self.message = error.to_string(),
        }
        Ok(())
    }
    fn start_choosing(&mut self) {
        let Some(row) = self.current() else {
            self.message = "Nothing is selected.".into();
            return;
        };
        if row.kind != "task" {
            self.message = "Only tasks can be handed to a harness.".into();
            return;
        }
        if !row.open_task() {
            self.message = "Only open tasks (todo, doing or blocked) can be handed off.".into();
            return;
        }
        if self.harnesses.is_empty() {
            self.message = "No harness has a grant yet. Create one with mother grant.".into();
            return;
        }
        let at = row
            .assignee
            .as_ref()
            .and_then(|a| self.harnesses.iter().position(|h| h == a))
            .map_or(1, |i| i + 1);
        self.choosing = Some(at);
        self.message = "Choose who picks this task up next.".into();
    }
    fn apply_choice(&mut self, at: usize) -> Result<()> {
        self.choosing = None;
        let Some(row) = self.current() else {
            return Ok(());
        };
        let (id, title) = (row.id.clone(), row.title.clone());
        let assignee = at
            .checked_sub(1)
            .and_then(|i| self.harnesses.get(i))
            .cloned();
        match Store::open(&self.path)?.owner_assign(&id, assignee.as_deref()) {
            Ok(_) => {
                self.message = match &assignee {
                    Some(name) => format!("“{title}” is waiting for {name}."),
                    None => format!("“{title}” is no longer handed to anyone."),
                };
                self.reload()?;
            }
            Err(error) => self.message = error.to_string(),
        }
        Ok(())
    }
    /// Handle one key. Returns true when the owner leaves for the home screen.
    pub(super) fn input(&mut self, key: KeyEvent) -> Result<bool> {
        if let Some(at) = self.choosing {
            let last = self.choices().len() - 1;
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.choosing = Some(at.saturating_sub(1)),
                KeyCode::Down | KeyCode::Char('j') => self.choosing = Some((at + 1).min(last)),
                KeyCode::Enter => self.apply_choice(at)?,
                KeyCode::Esc => {
                    self.choosing = None;
                    self.message = "Nothing changed.".into();
                }
                _ => {}
            }
            return Ok(false);
        }
        let last = self.visible().len().saturating_sub(1);
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return Ok(true),
            KeyCode::Tab | KeyCode::Right => self.switch(1),
            KeyCode::BackTab | KeyCode::Left => self.switch(-1),
            KeyCode::Up | KeyCode::Char('k') => self.selected = self.selected.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => self.selected = (self.selected + 1).min(last),
            KeyCode::Char('p') => self.toggle_pin()?,
            KeyCode::Char('a') => self.start_choosing(),
            KeyCode::Char('r') => {
                self.reload()?;
                self.message = "Refreshed.".into();
            }
            _ => {}
        }
        Ok(false)
    }
    pub(super) fn hints(&self) -> &'static str {
        if self.choosing.is_some() {
            "↑↓ choose   Enter hand over   Esc cancel"
        } else {
            "Tab kind   ↑↓ select   p pin / unpin   a hand a task to a harness   r refresh   Esc home"
        }
    }
    pub(super) fn render(&self, frame: &mut Frame, area: Rect) {
        let columns = Layout::horizontal([Constraint::Percentage(46), Constraint::Percentage(54)])
            .split(area);
        let panel = |title: &'static str| {
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(VIOLET))
                .padding(Padding::horizontal(1))
        };
        // Left: tabs and the list.
        let mut tabs = Vec::new();
        for tab in TABS {
            let label = match tab {
                Tab::Pinned => format!("Pinned {}/{}", self.count(tab), self.pin_limit),
                Tab::Memories => format!("Memories {}", self.count(tab)),
                Tab::Decisions => format!("Decisions {}", self.count(tab)),
                Tab::Tasks => format!("Tasks {}", self.count(tab)),
            };
            let style = if tab == self.tab {
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(MUTED)
            };
            tabs.push(Span::styled(label, style));
            tabs.push(Span::raw("  "));
        }
        let mut lines = vec![Line::from(tabs), Line::from("")];
        let rows = self.visible();
        if rows.is_empty() {
            let empty = match self.tab {
                Tab::Pinned => {
                    "Nothing is pinned yet. Press Tab to browse memories and decisions, then p to pin what she should never lose."
                }
                Tab::Memories => "No memories yet.",
                Tab::Decisions => "No decisions yet.",
                Tab::Tasks => "No tasks yet.",
            };
            lines.push(Line::from(empty).style(Style::default().fg(MUTED)));
        }
        let width = usize::from(columns[0].width.saturating_sub(6));
        let visible = usize::from(columns[0].height.saturating_sub(5)).max(1);
        let start = self.selected.saturating_sub(visible - 1);
        for (i, row) in rows.iter().enumerate().skip(start).take(visible) {
            let mut label = format!(
                "{} {}{}",
                if i == self.selected { ">" } else { " " },
                if row.pinned.is_some() && self.tab != Tab::Pinned {
                    "★ "
                } else {
                    ""
                },
                safe(&row.title)
            );
            if let Some(assignee) = &row.assignee {
                label.push_str(&format!("  → {}", safe(assignee)));
            }
            let label: String = label.chars().take(width.max(8)).collect();
            let style = if i == self.selected {
                Style::default().fg(GOLD)
            } else if !row.recallable || (row.kind == "task" && !row.open_task()) {
                Style::default().fg(MUTED)
            } else {
                Style::default().fg(INK)
            };
            lines.push(Line::from(label).style(style));
        }
        if self.omitted > 0 {
            lines.push(
                Line::from(format!(
                    "{} older records are not listed here. Use mother records.",
                    self.omitted
                ))
                .style(Style::default().fg(MUTED)),
            );
        }
        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .block(panel(" Tend · pinned core and handoffs ")),
            columns[0],
        );
        // Right: the chooser, or the selected record.
        if let Some(at) = self.choosing {
            let mut lines = vec![
                Line::from("Who picks this task up next?").style(Style::default().fg(GOLD)),
                Line::from(""),
            ];
            for (i, choice) in self.choices().iter().enumerate() {
                let style = if i == at {
                    Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(INK)
                };
                lines.push(
                    Line::from(format!(
                        "{} {}",
                        if i == at { ">" } else { " " },
                        safe(choice)
                    ))
                    .style(style),
                );
            }
            lines.push(Line::from(""));
            lines.push(
                Line::from(
                    "The harness finds the task when it next reads her context. Nothing runs by itself.",
                )
                .style(Style::default().fg(MUTED)),
            );
            frame.render_widget(
                Paragraph::new(lines)
                    .wrap(Wrap { trim: false })
                    .block(panel(" Hand off ")),
                columns[1],
            );
            return;
        }
        let mut lines = Vec::new();
        if let Some(row) = self.current() {
            lines.push(
                Line::from(safe(&row.title))
                    .style(Style::default().fg(VIOLET).add_modifier(Modifier::BOLD)),
            );
            lines.push(
                Line::from(format!(
                    "{} · {} · last saved by {} · {}",
                    safe(&row.kind),
                    safe(&row.status),
                    safe(&row.actor),
                    safe(row.updated.get(..16).unwrap_or(&row.updated)).replace('T', " ")
                ))
                .style(Style::default().fg(MUTED)),
            );
            if let Some(by) = &row.pinned {
                lines.push(
                    Line::from(format!("★ Pinned by {}", safe(by)))
                        .style(Style::default().fg(GOLD)),
                );
            }
            if let Some(assignee) = &row.assignee {
                lines.push(
                    Line::from(format!(
                        "→ Waiting for {}{}",
                        safe(assignee),
                        row.assigned_by
                            .as_ref()
                            .map(|by| format!(", handed over by {}", safe(by)))
                            .unwrap_or_default()
                    ))
                    .style(Style::default().fg(GOLD)),
                );
            }
            lines.push(Line::from(""));
            for part in row.excerpt.split('\n') {
                lines.push(Line::from(safe(part)));
            }
            if row.truncated {
                lines.push(
                    Line::from("… more with mother history <id> or in a harness.")
                        .style(Style::default().fg(MUTED)),
                );
            }
            lines.push(Line::from(""));
            lines.push(Line::from(safe(&row.id)).style(Style::default().fg(MUTED)));
        } else {
            lines.push(
                Line::from("Select a record to see it here.").style(Style::default().fg(MUTED)),
            );
        }
        frame.render_widget(
            Paragraph::new(lines)
                .style(Style::default().fg(INK))
                .wrap(Wrap { trim: false })
                .block(panel(" Record ")),
            columns[1],
        );
    }
}
