//! Graph owner review UI; authorization and batch policy remain in the graph writer.
#[cfg(test)]
mod tests;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mother::{
    error::{Error, Result},
    garden::{
        config::domain_error,
        contracts::{
            BatchItem, BatchRequest, GardenError, GardenErrorCode, MAX_JSON_BYTES, OutcomeResponse,
            OutcomeStatus, bounded_json,
        },
        inbox::validate_reviewed_items,
        store::GardenStore,
    },
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use serde_json::{Value, json};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    List,
    Review,
    Edit,
    ConfirmApprove,
    ConfirmDismiss,
}

pub(super) struct Inbox {
    path: PathBuf,
    page: Page,
    workspace: Value,
    items: Vec<Value>,
    omitted: u64,
    page_cursor: Option<String>,
    next_cursor: Option<String>,
    cursor_history: Vec<Option<String>>,
    selected: usize,
    proposal: Value,
    candidate: Value,
    editor: String,
    cursor: usize,
    scroll: u16,
    pending: Option<Value>,
    blocked: bool,
    message: String,
}
impl Inbox {
    fn empty(path: PathBuf) -> Self {
        Self {
            path,
            page: Page::List,
            workspace: Value::Null,
            items: vec![],
            omitted: 0,
            page_cursor: None,
            next_cursor: None,
            cursor_history: vec![],
            selected: 0,
            proposal: Value::Null,
            candidate: Value::Null,
            editor: String::new(),
            cursor: 0,
            scroll: 0,
            pending: None,
            blocked: false,
            message: "Graph proposals stay outside canonical recall until approved.".into(),
        }
    }
    pub(super) fn open(path: PathBuf) -> Result<Self> {
        let mut inbox = Self::empty(path);
        inbox.reload()?;
        Ok(inbox)
    }
    fn reload(&mut self) -> Result<()> {
        let list = self.read_page(self.page_cursor.as_deref())?;
        self.install_page(list);
        Ok(())
    }
    fn read_page(&self, cursor: Option<&str>) -> Result<Value> {
        GardenStore::open(&self.path)
            .and_then(|store| store.owner_inbox_page(cursor))
            .map_err(domain_error)
    }
    fn install_page(&mut self, list: Value) {
        self.workspace = list["workspace"].clone();
        self.items = list["proposals"].as_array().cloned().unwrap_or_default();
        self.omitted = list["omitted"].as_u64().unwrap_or(0);
        self.next_cursor = list["nextCursor"].as_str().map(str::to_owned);
        self.selected = self.selected.min(self.items.len().saturating_sub(1));
    }
    fn next_page(&mut self) -> bool {
        let Some(next) = self.next_cursor.take() else {
            return false;
        };
        self.cursor_history.push(self.page_cursor.take());
        self.page_cursor = Some(next);
        self.selected = 0;
        self.scroll = 0;
        true
    }
    fn previous_page(&mut self) -> bool {
        let Some(previous) = self.cursor_history.pop() else {
            return false;
        };
        self.page_cursor = previous;
        self.selected = 0;
        self.scroll = 0;
        true
    }
    fn reset_page(&mut self) {
        self.page_cursor = None;
        self.next_cursor = None;
        self.cursor_history.clear();
        self.selected = 0;
        self.scroll = 0;
    }
    fn open_proposal(&mut self, proposal: Value) {
        self.candidate = proposal["request"]["items"].clone();
        self.blocked = proposal["status"] != "pending"
            || !proposal["blockedError"].is_null()
            || proposal["generation"] != self.workspace["generation"];
        self.proposal = proposal;
        self.page = Page::Review;
        self.scroll = 0;
        self.message =
            "Review every original and reviewed item, evidence and revision before approval."
                .into();
    }
    fn validate_candidate(&self) -> Result<()> {
        let original: BatchRequest = serde_json::from_value(self.proposal["request"].clone())?;
        let reviewed: Vec<BatchItem> = serde_json::from_value(self.candidate.clone())?;
        validate_reviewed_items(&original, &reviewed).map_err(domain_error)
    }
    fn prepare_request(&mut self, action: &str) -> Result<()> {
        if self.pending.is_some() {
            return Ok(());
        }
        let mut request = json!({"version":2,"workspaceId":self.workspace["id"],
            "generation":self.workspace["generation"],"operationId":uuid::Uuid::new_v4().to_string(),
            "proposalId":self.proposal["id"],"expectedRevision":self.proposal["revision"],"action":action});
        if action == "approve" && self.candidate != self.proposal["request"]["items"] {
            request["reviewedItems"] = self.candidate.clone();
        }
        bounded_json(&request).map_err(domain_error)?;
        self.pending = Some(request);
        Ok(())
    }
    fn commit(&mut self) {
        let action = if self.page == Page::ConfirmDismiss {
            "dismiss"
        } else {
            "approve"
        };
        if let Err(error) = self.prepare_request(action) {
            self.message = error.to_string();
            self.page = Page::Review;
            return;
        }
        if let Some(request) = self.pending.as_ref() {
            let outcome = GardenStore::open(&self.path)
                .and_then(|store| store.review_proposal(request.clone()));
            self.finish(outcome);
        }
    }
    fn finish(&mut self, result: std::result::Result<OutcomeResponse, GardenError>) {
        match result {
            Err(error) if error.code == GardenErrorCode::StoreUnavailable => {
                self.message = "Result uncertain. Enter reconciles the identical operation UUID and request. Navigation and editing are locked.".into();
            }
            Err(error) => {
                self.pending = None;
                self.blocked = !matches!(
                    error.code,
                    GardenErrorCode::InvalidInput | GardenErrorCode::LimitExceeded
                );
                self.page = Page::Review;
                self.message = format!(
                    "Review not applied: {error}. Conflicts require a new proposal; revisions are never refreshed automatically."
                );
            }
            Ok(outcome) if outcome.receipt.status == OutcomeStatus::Blocked => {
                self.pending = None;
                self.blocked = true;
                self.page = Page::Review;
                self.message = "Approval blocked by changed authority, generation or graph state. Return to inspect the proposal; obtain a new proposal to approve.".into();
            }
            Ok(outcome) => {
                self.pending = None;
                self.page = Page::List;
                self.reset_page();
                let text = if outcome.receipt.status == OutcomeStatus::Approved {
                    "Graph batch approved atomically."
                } else {
                    "Proposal dismissed. Canonical graph and evidence history were not erased."
                };
                self.message = text.into();
                if let Err(error) = self.reload() {
                    self.message = format!("{text} Refresh failed: {error}");
                }
            }
        }
    }
    /// Return true only when leaving the graph inbox with no uncertain write.
    pub(super) fn input(&mut self, key: KeyEvent) -> Result<bool> {
        if self.pending.is_some() {
            if key.code == KeyCode::Enter {
                self.commit();
            }
            return Ok(false);
        }
        match self.page {
            Page::List => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => self.selected = self.selected.saturating_sub(1),
                KeyCode::Down => {
                    self.selected = (self.selected + 1).min(self.items.len().saturating_sub(1))
                }
                KeyCode::Char('r') => {
                    let list = self.read_page(None)?;
                    self.reset_page();
                    self.install_page(list);
                }
                KeyCode::Char('n') => {
                    if let Some(cursor) = self.next_cursor.as_deref() {
                        let list = self.read_page(Some(cursor))?;
                        self.next_page();
                        self.install_page(list);
                    }
                }
                KeyCode::Char('p') => {
                    if let Some(cursor) = self.cursor_history.last() {
                        let list = self.read_page(cursor.as_deref())?;
                        self.previous_page();
                        self.install_page(list);
                    }
                }
                KeyCode::Enter => {
                    if let Some(id) = self
                        .items
                        .get(self.selected)
                        .and_then(|item| item["id"].as_str())
                    {
                        let proposal = GardenStore::open(&self.path)
                            .and_then(|store| store.owner_inbox_get(id))
                            .map_err(domain_error)?;
                        self.open_proposal(proposal);
                    }
                }
                _ => {}
            },
            Page::Review => match key.code {
                KeyCode::Esc => {
                    self.reload()?;
                    self.page = Page::List;
                    self.scroll = 0;
                }
                KeyCode::Char('e') if !self.blocked => {
                    self.editor = serde_json::to_string_pretty(&self.candidate)?;
                    self.cursor = self.editor.len();
                    self.page = Page::Edit;
                    self.scroll = 0;
                }
                KeyCode::Char('a') if !self.blocked => match self.validate_candidate() {
                    Ok(()) => {
                        self.page = Page::ConfirmApprove;
                        self.scroll = 0;
                    }
                    Err(error) => self.message = error.to_string(),
                },
                KeyCode::Char('d') if self.proposal["status"] == "pending" => {
                    self.page = Page::ConfirmDismiss;
                    self.scroll = 0;
                }
                KeyCode::Char('r') => {
                    self.message="Return to the inbox and reopen to inspect current state. Existing proposals and expected revisions are never rebased.".into();
                }
                _ => self.scroll_key(key.code),
            },
            Page::Edit => match key.code {
                KeyCode::Esc => {
                    self.page = Page::Review;
                    self.scroll = 0;
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    match serde_json::from_str::<Value>(&self.editor) {
                        Ok(candidate) => {
                            let previous = std::mem::replace(&mut self.candidate, candidate);
                            if let Err(error) = self.validate_candidate() {
                                self.candidate = previous;
                                self.message = error.to_string();
                            } else {
                                self.page = Page::Review;
                                self.scroll = 0;
                                self.message="Owner changes remain local. Review the complete batch before approval.".into();
                            }
                        }
                        Err(error) => self.message = format!("Invalid item-array JSON: {error}"),
                    }
                }
                _ => self.edit(key),
            },
            Page::ConfirmApprove | Page::ConfirmDismiss => match key.code {
                KeyCode::Esc => {
                    self.page = Page::Review;
                    self.scroll = 0;
                }
                KeyCode::Enter => self.commit(),
                _ => self.scroll_key(key.code),
            },
        }
        Ok(false)
    }
    fn scroll_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
            KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
            KeyCode::PageDown => self.scroll = self.scroll.saturating_add(10),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
            KeyCode::Home => self.scroll = 0,
            _ => {}
        }
    }
    fn edit(&mut self, key: KeyEvent) {
        let previous = self.editor[..self.cursor]
            .char_indices()
            .last()
            .map_or(0, |(i, _)| i);
        let next = self.editor[self.cursor..]
            .chars()
            .next()
            .map_or(self.cursor, |c| self.cursor + c.len_utf8());
        match key.code {
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.editor.clear();
                self.cursor = 0;
            }
            KeyCode::Char(c)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.insert(&c.to_string())
            }
            KeyCode::Enter => self.insert("\n"),
            KeyCode::Left => self.cursor = previous,
            KeyCode::Right => self.cursor = next,
            KeyCode::Home => self.cursor = 0,
            KeyCode::End => self.cursor = self.editor.len(),
            KeyCode::Backspace if self.cursor > 0 => {
                self.editor.replace_range(previous..self.cursor, "");
                self.cursor = previous;
            }
            KeyCode::Delete if self.cursor < self.editor.len() => {
                self.editor.replace_range(self.cursor..next, "");
            }
            _ => self.scroll_key(key.code),
        }
    }
    fn insert(&mut self, text: &str) {
        for c in text.chars().filter(|c| !c.is_control() || *c == '\n') {
            if self.editor.len() + c.len_utf8() > MAX_JSON_BYTES {
                break;
            }
            self.editor.insert(self.cursor, c);
            self.cursor += c.len_utf8();
        }
    }
    pub(super) fn paste(&mut self, text: &str) {
        if self.page == Page::Edit && self.pending.is_none() {
            self.insert(text);
        }
    }
    fn body(&self) -> String {
        let mut lines = vec![self.message.clone(), String::new()];
        if self.page == Page::List {
            lines.push("Graph review inbox. Unreviewed proposals are excluded from recall.".into());
            lines.push(format!(
                "Page {}   {} other proposals outside this page   next: {}",
                self.cursor_history.len() + 1,
                self.omitted,
                if self.next_cursor.is_some() {
                    "available"
                } else {
                    "none"
                }
            ));
            lines.push("↑↓ select   Enter review   n next page   p previous page   r first page / refresh   Esc exit".into());
            for (i, item) in self.items.iter().enumerate() {
                lines.push(format!(
                    "{} {}  {}  items:{}  proposer:{}",
                    if i == self.selected { ">" } else { " " },
                    item["id"].as_str().unwrap_or("(missing ID)"),
                    item["status"].as_str().unwrap_or("unknown"),
                    item["itemCount"].as_u64().unwrap_or(0),
                    item["proposer"].as_str().unwrap_or("unknown")
                ));
            }
            return lines.join("\n");
        }
        lines.push("Local edits/discard never erase shared evidence. Shared changes require an explicit retract or supersede, preserving history.".into());
        if self.page == Page::Edit {
            lines.push("Edit the bounded item-array JSON. Keep all original keys, targets and expected revisions; add distinct item keys.".into());
            lines.push("Ctrl+S review   Ctrl+U clear   arrows move/scroll   Home/End start/end   Esc discard edit".into());
            let mut editor = self.editor.clone();
            editor.insert(self.cursor, '▏');
            lines.push(editor);
            return lines.join("\n");
        }
        if self.page == Page::ConfirmApprove {
            lines.push("Confirm approval of ALL reviewed items below: Enter approves atomically; Esc returns.".into());
        } else if self.page == Page::ConfirmDismiss {
            lines.push("Dismiss this proposal only: Enter confirms; Esc returns. Canonical evidence and history remain.".into());
        } else {
            lines.push(
                if self.proposal["status"] != "pending" {
                    "Resolved proposal   ↑↓ / PgUp PgDn scroll   Esc inbox"
                } else if self.blocked {
                    "Approval blocked   d dismiss proposal   ↑↓ / PgUp PgDn scroll   Esc inbox"
                } else {
                    "a approve   e edit/add items   d dismiss   ↑↓ / PgUp PgDn scroll   Esc inbox"
                }
                .into(),
            );
        }
        lines.push(format!(
            "Original proposer: {}   subject: {}",
            self.proposal["proposer"], self.proposal["subject"]
        ));
        lines.push(format!(
            "Proposal {} revision {}   status {}",
            self.proposal["id"], self.proposal["revision"], self.proposal["status"]
        ));
        lines.push(format!(
            "Current targets (no revision rebasing): {}",
            self.proposal["currentTargets"]
        ));
        lines.push(format!("Blocked state: {}", self.proposal["blockedError"]));
        let original = self.proposal["request"]["items"].as_array();
        if let Some(items) = original {
            for item in items {
                lines.push(format!(
                    "Original item {} (including evidence):\n{}",
                    item["itemKey"],
                    pretty(item)
                ));
            }
        }
        if let Some(items) = self.candidate.as_array() {
            for item in items {
                let previous = original
                    .and_then(|items| items.iter().find(|old| old["itemKey"] == item["itemKey"]));
                let label = match previous {
                    None => "owner-added",
                    Some(old) if old != item => "owner-edited",
                    _ => "original unchanged",
                };
                lines.push(format!(
                    "Reviewed item {} [{label}] (including evidence):\n{}",
                    item["itemKey"],
                    pretty(item)
                ));
            }
        }
        lines.join("\n")
    }
    pub(super) fn render(&self, frame: &mut Frame, area: Rect) {
        let body = self
            .body()
            .chars()
            .filter(|c| !c.is_control() || *c == '\n')
            .collect::<String>();
        let scroll = if self.page == Page::List {
            u16::try_from(
                self.selected
                    .saturating_sub(usize::from(area.height.saturating_sub(8))),
            )
            .unwrap_or(u16::MAX)
        } else {
            self.scroll
        };
        let paragraph = Paragraph::new(body).scroll((scroll, 0)).block(
            Block::default()
                .title(" Garden owner inbox ")
                .borders(Borders::ALL),
        );
        let paragraph = if self.page == Page::List {
            paragraph
        } else {
            paragraph.wrap(Wrap { trim: false })
        };
        frame.render_widget(paragraph, area);
    }
    pub(super) fn report(&mut self, error: Error) {
        self.message = error.to_string();
    }
    pub(super) fn uncertain(&self) -> bool {
        self.pending.is_some()
    }
}
fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "Unavailable JSON".into())
}
