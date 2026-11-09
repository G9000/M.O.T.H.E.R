//! Owner-only review controller. Pending drafts never enter normal context here.
#[cfg(test)]
mod tests;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mother::{error::Result, store::Store};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use serde_json::{Value, json};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InboxPage {
    List,
    Review,
    Edit,
    ConfirmSave,
    ConfirmDismiss,
}

pub(super) struct Inbox {
    path: PathBuf,
    page: InboxPage,
    workspace: Value,
    items: Vec<Value>,
    omitted: u64,
    selected: usize,
    proposal: Value,
    candidate: Value,
    editor: Option<Editor>,
    scroll: u16,
    // Retain the exact request after uncertain I/O. Enter retries it explicitly.
    pending: Option<Value>,
    pub(super) message: String,
}
impl Inbox {
    fn empty(path: PathBuf) -> Self {
        Self {
            path,
            page: InboxPage::List,
            workspace: Value::Null,
            items: vec![],
            omitted: 0,
            selected: 0,
            proposal: Value::Null,
            candidate: Value::Null,
            editor: None,
            scroll: 0,
            pending: None,
            message: "Ask a connected agent to keep what matters, then review its proposals here."
                .into(),
        }
    }
    pub(super) fn open(path: PathBuf) -> Result<Self> {
        let mut inbox = Self::empty(path);
        inbox.reload()?;
        Ok(inbox)
    }
    fn reload(&mut self) -> Result<()> {
        let list = Store::open(&self.path)?.inbox_list()?;
        self.workspace = list["workspace"].clone();
        self.items = list["proposals"].as_array().cloned().unwrap_or_default();
        self.omitted = list["omitted"].as_u64().unwrap_or(0);
        self.selected = self.selected.min(self.items.len().saturating_sub(1));
        Ok(())
    }
    fn open_proposal(&mut self, proposal: Value) {
        self.candidate = proposal["record"].clone();
        self.proposal = proposal;
        self.page = InboxPage::Review;
        self.scroll = 0;
        self.editor = None;
        self.message =
            "Review source and changes. Save publishes to shared context; Dismiss keeps them out."
                .into();
    }
    fn stale(&self) -> bool {
        self.proposal["conflict"] == true
            || (!self.workspace.is_null()
                && self.proposal["generation"] != self.workspace["generation"])
    }
    fn commit(&mut self, action: &str) -> Result<()> {
        if self.pending.is_none() {
            let mut request = json!({"workspaceId":self.workspace["id"],"generation":self.workspace["generation"],
                "operationId":uuid::Uuid::new_v4().to_string(),"proposalId":self.proposal["id"],
                "expectedRevision":self.proposal["revision"],"action":action});
            if action == "save" {
                request["record"] = self.candidate.clone();
            }
            self.pending = Some(request);
        }
        let request = self
            .pending
            .as_ref()
            .expect("review request initialized")
            .clone();
        let outcome = Store::open(&self.path).and_then(|mut store| store.review_proposal(request));
        match outcome {
            Ok(receipt) => {
                self.pending = None;
                self.page = InboxPage::List;
                self.proposal = Value::Null;
                self.editor = None;
                let outcome = if receipt["status"] == "saved" {
                    "Saved to shared memory."
                } else {
                    "Dismissed. Shared memory was not changed."
                };
                self.message = outcome.into();
                if let Err(error) = self.reload() {
                    self.message = format!("{outcome} Inbox refresh failed: {error}");
                }
            }
            Err(error) => {
                if error.code() == "STORE_UNAVAILABLE" {
                    self.message="Result uncertain. Enter checks/retries the identical operation. Do not submit another review.".into();
                } else {
                    self.pending = None;
                    self.page = InboxPage::Review;
                    self.message = format!(
                        "Not saved: {error}. Refresh to inspect current state; stale proposals need a new capture."
                    );
                }
            }
        }
        Ok(())
    }
    /// Returns true when the owner leaves this inbox for the home screen.
    pub(super) fn input(&mut self, key: KeyEvent) -> Result<bool> {
        match self.page {
            InboxPage::List => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => self.selected = self.selected.saturating_sub(1),
                KeyCode::Down => {
                    self.selected = (self.selected + 1).min(self.items.len().saturating_sub(1))
                }
                KeyCode::Char('r') => {
                    self.reload()?;
                    self.message = "Inbox refreshed.".into();
                }
                KeyCode::Enter => {
                    if let Some(id) = self.items.get(self.selected).and_then(|p| p["id"].as_str()) {
                        let proposal = Store::open(&self.path)?.inbox_get(id)?;
                        self.open_proposal(proposal);
                    }
                }
                _ => {}
            },
            InboxPage::Review => match key.code {
                KeyCode::Esc => {
                    self.reload()?;
                    self.page = InboxPage::List;
                }
                KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
                KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
                KeyCode::PageDown => self.scroll = self.scroll.saturating_add(10),
                KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
                KeyCode::Home => self.scroll = 0,
                KeyCode::Char('r') => {
                    let id = self.proposal["id"].as_str().unwrap_or_default();
                    let current = Store::open(&self.path)?.inbox_get(id)?;
                    // Keep the owner's unsaved edit. Refresh only current-state evidence.
                    self.proposal = current;
                    self.reload()?;
                    self.message = "Current state refreshed; your candidate edit is kept.".into();
                }
                KeyCode::Char('e') => {
                    self.editor = Some(Editor::new(&self.candidate));
                    self.page = InboxPage::Edit;
                }
                KeyCode::Char('s') => {
                    if self.stale() {
                        self.message="The target or workspace changed. Request a fresh proposal; this one can still be dismissed.".into();
                    } else if self.proposal["status"] != "pending" {
                        self.message =
                            "This proposal was already resolved. Return to the inbox.".into();
                    } else {
                        self.page = InboxPage::ConfirmSave;
                    }
                }
                KeyCode::Char('d') => self.page = InboxPage::ConfirmDismiss,
                _ => {}
            },
            InboxPage::Edit => {
                if key.code == KeyCode::Esc {
                    self.editor = None;
                    self.page = InboxPage::Review;
                } else if key.code == KeyCode::Char('s')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    if let Some(editor) = self.editor.take() {
                        self.candidate = editor.draft(&self.candidate);
                    }
                    self.page = InboxPage::Review;
                    self.scroll = 0;
                    self.message = "Edited candidate. Review it, then press s to save.".into();
                } else if let Some(editor) = self.editor.as_mut() {
                    editor.input(key);
                }
            }
            InboxPage::ConfirmSave | InboxPage::ConfirmDismiss => match key.code {
                KeyCode::Esc if self.pending.is_none() => self.page = InboxPage::Review,
                KeyCode::Enter => {
                    let action = if self.page == InboxPage::ConfirmSave {
                        "save"
                    } else {
                        "dismiss"
                    };
                    self.commit(action)?;
                }
                _ => {}
            },
        }
        Ok(false)
    }
    pub(super) fn paste(&mut self, text: &str) {
        if self.page == InboxPage::Edit
            && let Some(editor) = self.editor.as_mut()
        {
            editor.insert(text);
        }
    }
    pub(super) fn hints(&self) -> &'static str {
        match self.page {
            InboxPage::List => "↑↓ select   Enter review   r refresh   Esc home",
            InboxPage::Review => {
                "↑↓ / PgUp PgDn scroll   e edit   s Save   d Dismiss   r refresh   Esc inbox"
            }
            InboxPage::Edit => {
                "Tab field   arrows move/change choice   Enter newline   Ctrl+U clear   Ctrl+S review   Esc cancel"
            }
            InboxPage::ConfirmSave | InboxPage::ConfirmDismiss => {
                "Enter confirm   Esc back (unless result is uncertain)"
            }
        }
    }
    pub(super) fn render(&self, frame: &mut Frame, area: Rect) {
        let title = match self.page {
            InboxPage::List => " Memory Inbox · waiting for you ",
            InboxPage::Edit => " Edit candidate · not saved ",
            _ => " Review memory ",
        };
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(188, 160, 215)));
        let (body, scroll) = match self.page {
            InboxPage::List => {
                let mut lines = vec![
                    "Proposals stay out of shared recall until you Save.".into(),
                    String::new(),
                ];
                if self.items.is_empty() {
                    lines.push(
                        "Nothing waiting. In a connected chat, ask: keep what matters.".into(),
                    );
                }
                let visible = usize::from(area.height.saturating_sub(6)).max(1);
                let start = self.selected.saturating_sub(visible - 1);
                for (i, p) in self.items.iter().enumerate().skip(start).take(visible) {
                    lines.push(fit_line(
                        &format!(
                            "{} {}  ·  {}",
                            if i == self.selected { ">" } else { " " },
                            value(p, "title"),
                            value(p, "actor")
                        ),
                        usize::from(area.width.saturating_sub(2)),
                    ));
                }
                if self.omitted > 0 {
                    lines.push(format!(
                        "{} more pending. Resolve these and refresh to continue.",
                        self.omitted
                    ));
                }
                (lines.join("\n"), 0)
            }
            InboxPage::Review => {
                let mut lines = vec![
                    format!("From: {}", value(&self.proposal, "actor")),
                    format!("Source: {}", value(&self.proposal, "source")),
                    format!("Attribution: {}", value(&self.proposal, "basis")),
                    format!("State: {}", value(&self.proposal, "status")),
                ];
                if self.stale() {
                    lines.push("CONFLICT: target/workspace changed. Save is blocked; request a fresh proposal.".into());
                }
                if self.proposal["duplicateRecordId"].is_string() {
                    lines.push(format!(
                        "Possible duplicate: {}. Inspect the existing record before saving.",
                        value(&self.proposal, "duplicateRecordId")
                    ));
                }
                if let Some(similar) = self.proposal["similarRecordIds"].as_array() {
                    for id in similar {
                        lines.push(format!(
                            "Related title/content: {}. Compare before saving.",
                            id.as_str().unwrap_or("")
                        ));
                    }
                }
                lines.push(String::new());
                lines.push("Before (at capture)".into());
                append_record(&mut lines, &self.proposal["before"]);
                if self.proposal["currentRecord"] != self.proposal["before"] {
                    lines.push("Current record (changed since capture)".into());
                    append_record(&mut lines, &self.proposal["currentRecord"]);
                }
                lines.push(String::new());
                lines.push(
                    if self.candidate != self.proposal["record"] {
                        "After · Edited candidate"
                    } else {
                        "After · Proposed candidate"
                    }
                    .into(),
                );
                append_record(&mut lines, &self.candidate);
                (lines.join("\n"), self.scroll)
            }
            InboxPage::Edit => {
                let body = self.editor.as_ref().map_or_else(String::new, |e| {
                    e.display(
                        usize::from(area.width.saturating_sub(2)).max(1),
                        usize::from(area.height.saturating_sub(2)).max(2),
                    )
                });
                (body, 0)
            }
            InboxPage::ConfirmSave | InboxPage::ConfirmDismiss => {
                let verb = if self.page == InboxPage::ConfirmSave {
                    "Save this candidate to shared context?"
                } else {
                    "Dismiss this proposal?"
                };
                (
                    format!(
                        "{verb}\n\n{}\n\n{}\n\nEnter confirms this one item. Esc returns to review.",
                        value(&self.candidate, "title"),
                        if self.page == InboxPage::ConfirmSave {
                            "Other connected harnesses can retrieve the approved record."
                        } else {
                            "The proposal stays in audit history. It will not become active context."
                        }
                    ),
                    0,
                )
            }
        };
        frame.render_widget(
            Paragraph::new(sanitize(&body))
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0))
                .block(block),
            area,
        );
    }
}
fn value<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap_or("")
}
fn sanitize(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || *c == '\n')
        .collect()
}
fn append_record(lines: &mut Vec<String>, record: &Value) {
    if record.is_null() {
        lines.push("New record; nothing is being replaced.".into());
        return;
    }
    for (key, label) in [
        ("id", "Record"),
        ("actor", "Written by"),
        ("createdAt", "Created"),
        ("updatedAt", "Updated"),
    ] {
        if let Some(text) = record[key].as_str() {
            lines.push(format!("{label}: {text}"));
        }
    }
    lines.push(format!(
        "{} · {} · {}",
        value(record, "kind"),
        value(record, "status"),
        value(record, "basis")
    ));
    if let Some(revision) = record["revision"].as_u64() {
        lines.push(format!("Revision {revision}"));
    }
    lines.push(value(record, "title").into());
    lines.push(value(record, "content").into());
    if let Some(sources) = record["sources"].as_array() {
        for source in sources {
            lines.push(format!(
                "Source link: {} — {}",
                value(source, "label"),
                value(source, "uri")
            ));
        }
    }
    if let Some(references) = record["references"].as_array() {
        for reference in references {
            lines.push(format!(
                "Related record: {}",
                reference.as_str().unwrap_or("")
            ));
        }
    }
}

struct Editor {
    fields: [String; 4],
    cursors: [usize; 4],
    selected: usize,
    kind: String,
}
impl Editor {
    fn new(draft: &Value) -> Self {
        let fields = ["title", "content", "status", "basis"].map(|k| value(draft, k).to_owned());
        let cursors = fields.each_ref().map(|s| s.len());
        Self {
            fields,
            cursors,
            selected: 0,
            kind: value(draft, "kind").into(),
        }
    }
    fn count(&self) -> usize {
        if self.kind == "memory" { 4 } else { 3 }
    }
    fn choices(&self) -> &'static [&'static str] {
        if self.selected == 3 {
            &["owner_statement", "source", "model_inference"]
        } else {
            match self.kind.as_str() {
                "memory" => &["active", "retracted"],
                "decision" => &["proposed", "confirmed", "superseded", "retracted"],
                _ => &["todo", "doing", "blocked", "done", "cancelled"],
            }
        }
    }
    fn insert(&mut self, text: &str) {
        if self.selected >= 2 {
            return;
        }
        let max = if self.selected == 0 { 240 } else { 12000 };
        let field = &mut self.fields[self.selected];
        let cursor = &mut self.cursors[self.selected];
        let mut used = field.encode_utf16().count();
        for c in text
            .chars()
            .filter(|c| !c.is_control() || (*c == '\n' && self.selected == 1))
        {
            if used + c.len_utf16() > max {
                break;
            }
            field.insert(*cursor, c);
            *cursor += c.len_utf8();
            used += c.len_utf16();
        }
    }
    fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => self.selected = (self.selected + 1) % self.count(),
            KeyCode::BackTab => self.selected = (self.selected + self.count() - 1) % self.count(),
            KeyCode::Left | KeyCode::Right if self.selected >= 2 => {
                let choices = self.choices();
                let index = choices
                    .iter()
                    .position(|s| *s == self.fields[self.selected])
                    .unwrap_or(0);
                let next = if key.code == KeyCode::Left {
                    (index + choices.len() - 1) % choices.len()
                } else {
                    (index + 1) % choices.len()
                };
                self.fields[self.selected] = choices[next].into();
            }
            KeyCode::Char('u')
                if key.modifiers.contains(KeyModifiers::CONTROL) && self.selected < 2 =>
            {
                self.fields[self.selected].clear();
                self.cursors[self.selected] = 0;
            }
            KeyCode::Char(c)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.insert(&c.to_string())
            }
            KeyCode::Enter if self.selected == 1 => self.insert("\n"),
            _ if self.selected < 2 => {
                let field = &mut self.fields[self.selected];
                let cursor = &mut self.cursors[self.selected];
                let previous = field[..*cursor].char_indices().last().map_or(0, |(i, _)| i);
                let next = field[*cursor..]
                    .chars()
                    .next()
                    .map_or(*cursor, |c| *cursor + c.len_utf8());
                match key.code {
                    KeyCode::Left => *cursor = previous,
                    KeyCode::Right => *cursor = next,
                    KeyCode::Backspace if *cursor > 0 => {
                        field.replace_range(previous..*cursor, "");
                        *cursor = previous;
                    }
                    KeyCode::Delete if *cursor < field.len() => {
                        field.replace_range(*cursor..next, "");
                    }
                    KeyCode::Home => *cursor = field[..*cursor].rfind('\n').map_or(0, |i| i + 1),
                    KeyCode::End => {
                        *cursor += field[*cursor..].find('\n').unwrap_or(field.len() - *cursor)
                    }
                    KeyCode::Up | KeyCode::Down => {
                        let start = field[..*cursor].rfind('\n').map_or(0, |i| i + 1);
                        let column = field[start..*cursor].chars().count();
                        let target = if key.code == KeyCode::Up {
                            start
                                .checked_sub(1)
                                .map(|end| (field[..end].rfind('\n').map_or(0, |i| i + 1), end))
                        } else {
                            field[*cursor..].find('\n').map(|offset| {
                                let begin = *cursor + offset + 1;
                                (
                                    begin,
                                    begin
                                        + field[begin..].find('\n').unwrap_or(field.len() - begin),
                                )
                            })
                        };
                        if let Some((start, end)) = target {
                            *cursor = start
                                + field[start..end]
                                    .char_indices()
                                    .nth(column)
                                    .map_or(end - start, |(i, _)| i);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn draft(self, original: &Value) -> Value {
        let mut draft = original.clone();
        draft["title"] = json!(self.fields[0]);
        draft["content"] = json!(self.fields[1]);
        draft["status"] = json!(self.fields[2]);
        if self.kind == "memory" {
            draft["basis"] = json!(self.fields[3]);
        }
        draft
    }
    fn display(&self, width: usize, height: usize) -> String {
        let labels = ["Title", "Content", "Status", "Basis"];
        let mut lines = vec![fit_line(
            "Edits stay local until you review and Save.",
            width,
        )];
        // One line per inactive field leaves a bounded viewport for the active field.
        let available = height.saturating_sub(self.count() + 1).max(1);
        for (i, label) in labels.iter().enumerate().take(self.count()) {
            if i != self.selected {
                let preview = if i == 1 {
                    format!("{} characters", self.fields[i].chars().count())
                } else {
                    self.fields[i].replace(['\n', '\r'], " ")
                };
                lines.push(fit_line(&format!("  {label}: {preview}"), width));
                continue;
            }
            lines.push(fit_line(&format!("> {label}"), width));
            if i >= 2 {
                lines.push(fit_line(&format!("< {} >", self.fields[i]), width));
                continue;
            }
            let mut shown = sanitize(&self.fields[i]);
            let at = sanitize(&self.fields[i][..self.cursors[i]]).len();
            shown.insert(at, '\u{258f}');
            let wrapped = wrap_cells(&shown, width);
            let cursor_line = wrap_cells(&shown[..at + '\u{258f}'.len_utf8()], width).len() - 1;
            let start = cursor_line.saturating_sub(available - 1);
            lines.extend(wrapped.into_iter().skip(start).take(available));
        }
        lines.join("\n")
    }
}

fn fit_line(text: &str, width: usize) -> String {
    let plain = sanitize(text).replace('\n', " ");
    if ratatui::text::Span::raw(&plain).width() <= width {
        return plain;
    }
    let mut out = String::new();
    let mut used = 0;
    for c in plain.chars() {
        let size = ratatui::text::Span::raw(c.to_string()).width();
        if used + size >= width {
            break;
        }
        out.push(c);
        used += size;
    }
    out.push('\u{2026}');
    out
}
fn wrap_cells(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![String::new()];
    let mut used = 0;
    for c in text.chars() {
        if c == '\n' {
            lines.push(String::new());
            used = 0;
            continue;
        }
        let size = ratatui::text::Span::raw(c.to_string()).width();
        if used + size > width {
            lines.push(String::new());
            used = 0;
        }
        if let Some(line) = lines.last_mut() {
            line.push(c);
        }
        used += size;
    }
    lines
}
