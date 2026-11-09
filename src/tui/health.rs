//! Existing client selection and a cancellable local MCP health report.
use crate::health::{HealthReport, ProbeTask};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::path::{Path, PathBuf};

struct Client {
    path: PathBuf,
    label: String,
    other_workspace: bool,
}
pub(super) struct Health {
    clients: Vec<Client>,
    selected: usize,
    report: Option<HealthReport>,
    task: Option<ProbeTask>,
    scroll: u16,
    database: PathBuf,
    pub message: String,
}
impl Health {
    pub fn open(home: &Path) -> Self {
        let extra = crate::connection::load_settings()
            .ok()
            .flatten()
            .map(|settings| settings.client_config);
        Self::from_directory(home, extra)
    }
    fn from_directory(home: &Path, extra: Option<PathBuf>) -> Self {
        let mut paths = Vec::new();
        // Directory entries and displayed metadata are bounded, including when
        // the private clients directory contains unrelated/malformed files.
        for root in [home.join("clients")] {
            if let Ok(entries) = std::fs::read_dir(root) {
                for entry in entries.take(128).flatten() {
                    let path = entry.path();
                    if path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .is_some_and(|s| s.ends_with(".private.json"))
                        && entry.file_type().is_ok_and(|kind| kind.is_file())
                    {
                        paths.push(path);
                    }
                }
            }
        }
        // A saved ChatGPT config may live outside the standard clients folder.
        if let Some(path) = extra {
            paths.push(path);
        }
        paths.sort();
        paths.dedup();
        let clients = paths
            .into_iter()
            .take(128)
            .enumerate()
            .map(|(index, path)| {
                let (label, other_workspace) = crate::health::client_config(&path).map_or_else(
                    |_| (format!("Invalid private config #{}", index + 1), false),
                    |config| {
                        let other = config.database_path.canonicalize().ok()
                            != home.join("context.sqlite").canonicalize().ok();
                        (config.client_id, other)
                    },
                );
                Client {
                    path,
                    label,
                    other_workspace,
                }
            })
            .collect();
        Self {
            clients,
            selected: 0,
            report: None,
            task: None,
            scroll: 0,
            database: home.join("context.sqlite"),
            message:
                "Select an existing grant. Enter tests this executable locally (up to 10 seconds)."
                    .into(),
        }
    }
    pub fn poll(&mut self) {
        if let Some(report) = self.task.as_mut().and_then(ProbeTask::poll) {
            self.message = format!("{}: {}", report.scope, report.status);
            self.report = Some(report);
            self.task = None;
        }
    }
    pub fn input(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.task = None;
                return true;
            }
            KeyCode::Up if self.task.is_none() => {
                self.selected = self.selected.saturating_sub(1);
                self.report = None;
                self.scroll = 0;
            }
            KeyCode::Down if self.task.is_none() => {
                self.selected = (self.selected + 1).min(self.clients.len().saturating_sub(1));
                self.report = None;
                self.scroll = 0;
            }
            KeyCode::PageDown => self.scroll = self.scroll.saturating_add(8),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(8),
            KeyCode::Enter if self.task.is_none() => {
                if let Some(client) = self.clients.get(self.selected) {
                    if client.other_workspace {
                        self.message = "This grant targets another workspace. Use mother doctor --config <path> explicitly to test it.".into();
                        return false;
                    }
                    self.task = Some(ProbeTask::start(client.path.clone(), self.database.clone()));
                    self.report = None;
                    self.scroll = 0;
                    self.message = "Checking local initialize, tools/list and context read... Esc cancels and reaps the child.".into();
                }
            }
            _ => {}
        }
        false
    }
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mut lines = vec![
            Line::from("Test connection · current MOTHER executable + existing client grant"),
            Line::from("Actual ChatGPT / Hermes / AnimaOS conversations: not verified here."),
            Line::from(""),
        ];
        if self.clients.is_empty() {
            lines.push(Line::from(
                "No private client configs found in the private clients directory.",
            ));
        }
        let first = self.selected.saturating_sub(2);
        for (index, client) in self.clients.iter().enumerate().skip(first).take(5) {
            lines.push(Line::from(format!(
                "{} {}{}",
                if index == self.selected { ">" } else { " " },
                client.label,
                if client.other_workspace {
                    " (other workspace)"
                } else {
                    ""
                }
            )));
        }
        lines.push(Line::from(format!(
            "{} private config(s) · arrows select",
            self.clients.len()
        )));
        if let Some(report) = &self.report {
            lines.extend([
                Line::from(""),
                Line::from(format!("{} · {}", report.scope, report.status)),
                Line::from(format!("Executable: {}", report.executable.display())),
                Line::from(format!(
                    "SHA-256: {}",
                    report.executable_sha256.as_deref().unwrap_or("unavailable")
                )),
                Line::from(format!(
                    "Server version: {}",
                    report.server_version.as_deref().unwrap_or("unavailable")
                )),
                Line::from(format!(
                    "Workspace: {}",
                    report.workspace_id.as_deref().unwrap_or("unavailable")
                )),
                Line::from(format!(
                    "Generation: {}",
                    report.generation.as_deref().unwrap_or("unavailable")
                )),
            ]);
            if let Some(inventory) = &report.inventory {
                lines.push(Line::from(format!(
                    "Canonical: {}   Recallable: {}",
                    inventory.total_records, inventory.recallable_records
                )));
                for (kind, total) in &inventory.by_kind {
                    lines.push(Line::from(format!(
                        "{kind}: {total} canonical / {} recallable",
                        inventory.recallable_by_kind[kind]
                    )));
                }
            } else {
                lines.push(Line::from(
                    "Inventory: unavailable / unsupported; no zero counts inferred.",
                ));
            }
        }
        // All external display strings pass through the same terminal filter.
        let lines: Vec<Line<'_>> = lines
            .into_iter()
            .map(|line| Line::from(super::view::safe(&line.to_string())))
            .collect();
        frame.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .title(" Connection health ")
                        .borders(Borders::ALL),
                )
                .wrap(Wrap { trim: false })
                .scroll((self.scroll, 0)),
            area,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;
    use mother::store::Store;
    use ratatui::{Terminal, backend::TestBackend};
    use tempfile::TempDir;

    #[test]
    fn existing_configs_are_redacted_and_foreign_workspaces_are_not_probed() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir(dir.path().join("clients")).unwrap();
        let foreign = TempDir::new().unwrap();
        let db = foreign.path().join("context.sqlite");
        let config = dir.path().join("clients/foreign.private.json");
        let mut store = Store::initialize(&db, "SECRET PERSONA").unwrap();
        store
            .grant_to_file("synthetic-foreign", &["read".into()], &config, &db)
            .unwrap();
        let token = crate::health::client_config(&config).unwrap().token;
        let mut health = Health::from_directory(dir.path(), None);
        assert_eq!(health.clients.len(), 1);
        assert!(health.clients[0].other_workspace);
        health.input(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(health.task.is_none());
        assert!(health.message.contains("another workspace"));
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        terminal
            .draw(|frame| health.render(frame, frame.area()))
            .unwrap();
        let output: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol())
            .collect();
        assert!(output.contains("other workspace"));
        assert!(!output.contains(&token));
        assert!(!output.contains("SECRET PERSONA"));
    }
    #[test]
    fn leaving_health_page_cancels_the_owned_worker() {
        let dir = TempDir::new().unwrap();
        let mut health = Health::from_directory(dir.path(), None);
        health.task = Some(ProbeTask::start(
            dir.path().join("missing.private.json"),
            dir.path().join("context.sqlite"),
        ));
        assert!(health.input(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)));
        assert!(health.task.is_none());
    }

    #[test]
    fn config_rotation_cannot_change_the_dashboard_workspace_at_probe_time() {
        let home = TempDir::new().unwrap();
        std::fs::create_dir(home.path().join("clients")).unwrap();
        let config = home.path().join("clients/reader.private.json");
        let db = home.path().join("context.sqlite");
        let mut store = Store::initialize(&db, "Synthetic owner").unwrap();
        store
            .grant_to_file("synthetic-reader", &["read".into()], &config, &db)
            .unwrap();
        let mut health = Health::from_directory(home.path(), None);
        assert!(!health.clients[0].other_workspace);
        let foreign = TempDir::new().unwrap();
        let foreign_db = foreign.path().join("context.sqlite");
        let foreign_config = foreign.path().join("foreign.private.json");
        let mut foreign_store = Store::initialize(&foreign_db, "Synthetic foreign").unwrap();
        foreign_store
            .grant_to_file(
                "synthetic-foreign",
                &["read".into()],
                &foreign_config,
                &foreign_db,
            )
            .unwrap();
        std::fs::copy(&foreign_config, &config).unwrap();
        health.input(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while health.task.is_some() && std::time::Instant::now() < deadline {
            health.poll();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(health.report.as_ref().unwrap().status, "workspace_mismatch");
        assert!(!health.report.as_ref().unwrap().read_verified);
    }
}
