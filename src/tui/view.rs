//! Terminal presentation with no credential or process access.
use super::{App, Page};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

const INK: Color = Color::Rgb(236, 227, 216);
const MUTED: Color = Color::Rgb(162, 150, 173);
const VIOLET: Color = Color::Rgb(188, 160, 215);
const GOLD: Color = Color::Rgb(225, 188, 139);
const BACKGROUND: Color = Color::Rgb(25, 23, 32);

fn safe(text: &str) -> String {
    text.chars().filter(|c| !c.is_control()).collect()
}
fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(MUTED))
        .padding(Padding::horizontal(1))
}
fn text<'a>(lines: impl Into<ratatui::text::Text<'a>>) -> Paragraph<'a> {
    Paragraph::new(lines)
        .style(Style::default().fg(INK))
        .wrap(Wrap { trim: false })
}
fn key(label: &'static str, description: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            label,
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
        ),
        Span::raw(description),
    ])
}
pub(super) fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(BACKGROUND).fg(INK)),
        area,
    );
    if area.width < 72 || area.height < 24 {
        frame.render_widget(
            text("Resize to at least 72 x 24 for M.O.T.H.E.R.\nCtrl+C exits when idle."),
            area,
        );
        return;
    }
    let width = area.width.min(112);
    let area = Rect::new(
        area.x + (area.width - width) / 2,
        area.y,
        width,
        area.height,
    );
    let rows = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(14),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .split(area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(
                "M.O.T.H.E.R",
                Style::default().fg(VIOLET).add_modifier(Modifier::BOLD),
            )),
            Line::from("Memory · Ontology · Temporal · Hearth · Experience · Remembrance")
                .style(Style::default().fg(MUTED)),
        ])
        .alignment(Alignment::Center),
        rows[0],
    );
    match app.page {
        Page::Home => home(frame, rows[1], app),
        Page::Inbox => {
            if let Some(inbox) = &app.inbox {
                inbox.render(frame, rows[1]);
            }
        }
        Page::Setup => setup(frame, rows[1], app),
        Page::ConfirmSave => confirm_save(frame, rows[1], app),
        Page::Help => help(frame, rows[1]),
        Page::ConfirmStartup => {
            let verb = if app.startup == Some(true) {
                "Disable"
            } else {
                "Enable"
            };
            frame.render_widget(text(vec![Line::from(format!("{verb} connection at Windows sign-in?")).style(Style::default().fg(GOLD)), Line::from(""), Line::from("Enabling starts this saved ChatGPT connection in the background when you sign in."), Line::from("The runtime key stays in your OS credential store. No key is written into the launcher."), Line::from("Use a stable installed executable. Other platforms need their own startup integration."), Line::from(""), key("Enter", "  apply this change"), key("Esc", "    return without changing startup")]).block(panel(" Sign-in startup ")), rows[1]);
        }
    }
    frame.render_widget(
        text(safe(if app.page == Page::Inbox {
            app.inbox
                .as_ref()
                .map_or(app.message.as_str(), |inbox| inbox.message.as_str())
        } else {
            &app.message
        }))
        .style(Style::default().fg(if app.busy { GOLD } else { MUTED }))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(MUTED)),
        ),
        rows[2],
    );
    let hints = if app.busy {
        "Working... status will update here. No automatic retry."
    } else {
        match app.page {
            Page::Home => {
                "i inbox   s setup   c connect   d disconnect   r refresh   a startup   h help   q quit"
            }
            Page::Inbox => app.inbox.as_ref().map_or("Esc home", |inbox| inbox.hints()),
            Page::Setup => "Tab / arrows move   Ctrl+U clear   Enter review   Esc cancel",
            Page::ConfirmSave | Page::ConfirmStartup => "Enter confirm   Esc back",
            Page::Help => "Esc / h return",
        }
    };
    frame.render_widget(
        Paragraph::new(hints)
            .alignment(Alignment::Center)
            .style(Style::default().fg(GOLD))
            .wrap(Wrap { trim: true }),
        rows[3],
    );
}
fn home(frame: &mut Frame, area: Rect, app: &App) {
    let columns =
        Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)]).split(area);
    let (state, detail) = app.status.as_ref().map_or(
        (
            "Checking".to_owned(),
            "Use refresh to inspect your saved connection.".to_owned(),
        ),
        |s| (s.state.to_string(), safe(&s.detail)),
    );
    let status_color = if app
        .status
        .as_ref()
        .is_some_and(|s| s.state == crate::connection::ConnectionState::Ready)
    {
        Color::LightGreen
    } else {
        GOLD
    };
    let lines = vec![
        Line::from(safe(&app.workspace)),
        key("i", "  Memory Inbox · review what matters"),
        Line::from(""),
        Line::from(vec![
            Span::raw("ChatGPT   "),
            Span::styled(
                state,
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(detail),
        Line::from(""),
        Line::from(format!(
            "Start at sign-in: {}",
            match app.startup {
                Some(true) => "on",
                Some(false) => "off",
                None => "unknown - inspect before changing",
            }
        )),
        Line::from(""),
        Line::from("ChatGPT read/save: not verified here").style(Style::default().fg(MUTED)),
        Line::from("Use a real ChatGPT tool call to verify shared context."),
    ];
    frame.render_widget(text(lines).block(panel(" Your workspace ")), columns[0]);
    let ready = app
        .status
        .as_ref()
        .is_some_and(|s| s.state == crate::connection::ConnectionState::Ready);
    let lines = if ready {
        vec![
            Line::from("Ready for ChatGPT setup").style(Style::default().fg(VIOLET)),
            Line::from(""),
            Line::from("Open chatgpt.com/plugins"),
            Line::from("Add custom MCP server"),
            Line::from("Connection: Tunnel"),
            Line::from("Authentication: No authentication"),
            Line::from(""),
            Line::from(
                "Select this tunnel and install the plugin. Then test reading and saving a synthetic memory.",
            ),
            Line::from(""),
            key("h", "  full guidance"),
        ]
    } else {
        vec![Line::from("A place to continue").style(Style::default().fg(VIOLET)), Line::from(""), key("s", "  set up your connection"), key("c", "  connect with saved settings"), key("r", "  check what is running"), Line::from(""), Line::from("Wait for local readiness before creating the ChatGPT plugin."), Line::from(""), Line::from("Your memories and persona stay in your local workspace. Context returned through tools is shared with ChatGPT.").style(Style::default().fg(MUTED))]
    };
    frame.render_widget(text(lines).block(panel(" Next step ")), columns[1]);
}
fn setup(frame: &mut Frame, area: Rect, app: &App) {
    let Some(form) = &app.form else {
        return;
    };
    let labels = [
        "Tunnel ID",
        "Platform organization ID (optional)",
        "Separate ChatGPT client config",
        "Official tunnel-client executable",
        "Stable M.O.T.H.E.R executable",
        "Runtime API key (blank keeps saved key)",
    ];
    let mut lines = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let selected = i == form.selected;
        lines.push(
            Line::from(format!("{} {}", if selected { ">" } else { " " }, label))
                .style(Style::default().fg(if selected { GOLD } else { MUTED })),
        );
        let value = if i == 5 {
            if form.key.is_empty() {
                "(not entered)".into()
            } else {
                "**************** (masked)".into()
            }
        } else {
            safe(&form.fields[i])
        };
        let max = area.width.saturating_sub(8) as usize;
        let value: String = value
            .chars()
            .rev()
            .take(max)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        lines.push(
            Line::from(format!("  {value}{}", if selected { "_" } else { "" }))
                .style(Style::default().fg(INK)),
        );
    }
    frame.render_widget(
        Paragraph::new(lines).block(panel(" ChatGPT connection · local settings only ")),
        area,
    );
}
fn confirm_save(frame: &mut Frame, area: Rect, app: &App) {
    let Some(form) = &app.form else {
        return;
    };
    frame.render_widget(
        text(vec![
            Line::from("Save these connection settings?").style(Style::default().fg(GOLD)),
            Line::from(""),
            Line::from(format!("Tunnel: {}", safe(&form.fields[0]))),
            Line::from(format!("Client config: {}", safe(&form.fields[2]))),
            Line::from(format!("Tunnel executable: {}", safe(&form.fields[3]))),
            Line::from(format!("M.O.T.H.E.R executable: {}", safe(&form.fields[4]))),
            Line::from(""),
            Line::from(if form.key.is_empty() {
                "Runtime key: keep the existing OS-vault entry."
            } else {
                "Runtime key: save the entered value in your OS credential store."
            }),
            Line::from(""),
            Line::from("This saves local setup. Press c on the home screen to connect."),
            Line::from("Existing memory, persona and grants are preserved."),
        ])
        .block(panel(" Review ")),
        area,
    );
}
fn help(frame: &mut Frame, area: Rect) {
    frame.render_widget(text(vec![Line::from("Connect once. Continue across harnesses.").style(Style::default().fg(VIOLET)), Line::from(""), Line::from("1. Create a private tunnel at platform.openai.com/settings/organization/tunnels."), Line::from("   Associate your Platform organization AND ChatGPT workspace."), Line::from("2. Create a runtime API key with Tunnels Read + Use. Keep it out of chat."), Line::from("3. Use mother grant chatgpt --scopes read,memory:write,checkpoint:write"), Line::from("   once, or reuse your existing separate ChatGPT config."), Line::from("4. Set up here, save, and connect. Wait for Ready before adding the plugin."), Line::from("5. In ChatGPT: Connection = Tunnel; Authentication = No authentication."), Line::from("   The private tunnel and local M.O.T.H.E.R grant still authenticate access."), Line::from("6. Ask ChatGPT to save a synthetic memory and retrieve its record ID in Codex."), Line::from(""), Line::from("Quitting this dashboard leaves the managed connection running. Your computer must stay on."), Line::from("Uncertain effects are never automatically replayed. Refresh and inspect before retrying."), Line::from("More: docs/terminal.md and docs/chatgpt.md in the project repository.").style(Style::default().fg(MUTED))]).block(panel(" Connection guide ")), area);
}
