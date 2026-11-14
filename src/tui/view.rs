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

pub(super) fn safe(text: &str) -> String {
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
        Page::Health => {
            if let Some(health) = &app.health {
                health.render(frame, rows[1]);
            }
        }
        Page::Home => home(frame, rows[1], app),
        Page::Animaos => {
            if let Some(setup) = &app.animaos {
                setup.render(frame, rows[1]);
            }
        }
        Page::Inbox => {
            if let Some(inbox) = &app.inbox {
                inbox.render(frame, rows[1]);
            }
        }
        Page::Tend => {
            if let Some(tend) = &app.tend {
                tend.render(frame, rows[1]);
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
        text(safe(if app.page == Page::Health {
            app.health
                .as_ref()
                .map_or(app.message.as_str(), |health| health.message.as_str())
        } else if app.page == Page::Animaos {
            app.animaos
                .as_ref()
                .map_or(app.message.as_str(), |setup| setup.message.as_str())
        } else if app.page == Page::Inbox {
            app.inbox
                .as_ref()
                .map_or(app.message.as_str(), |inbox| inbox.message.as_str())
        } else if app.page == Page::Tend {
            app.tend
                .as_ref()
                .map_or(app.message.as_str(), |tend| tend.message.as_str())
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
            Page::Health => "Up/Down select   Enter test   PgUp/PgDn scroll   Esc cancel / home",
            Page::Home => {
                "p pinned core   i inbox   t test   n AnimaOS   s setup   c connect   d disconnect   r refresh   a startup   h help   q quit"
            }
            Page::Inbox => app.inbox.as_ref().map_or("Esc home", |inbox| inbox.hints()),
            Page::Tend => app.tend.as_ref().map_or("Esc home", |tend| tend.hints()),
            Page::Animaos => app
                .animaos
                .as_ref()
                .map_or("Esc home", |setup| setup.hints()),
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
    use super::{ROWS, Row};
    use crate::apps::State;
    let columns =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);

    let chatgpt_ready = app
        .status
        .as_ref()
        .is_some_and(|s| s.state == crate::connection::ConnectionState::Ready);
    let mut lines = vec![
        Line::from("Which apps can read your memory").style(Style::default().fg(VIOLET)),
        Line::from(""),
    ];
    for (i, row) in ROWS.iter().enumerate() {
        let (name, mark, status, color) = match row {
            Row::App(id) => {
                let s = app.apps.iter().find(|s| s.app == *id);
                let (mark, color) = match s.map(|s| &s.state) {
                    Some(State::Working) => ("✓", Color::LightGreen),
                    Some(State::Broken(_)) => ("!", GOLD),
                    Some(State::NotConnected) => ("○", INK),
                    _ => (" ", MUTED),
                };
                (
                    id.name().to_owned(),
                    mark,
                    s.map_or("checking".into(), |s| s.summary()),
                    color,
                )
            }
            Row::Animaos => {
                if app.animaos_ready {
                    (
                        "AnimaOS".into(),
                        "•",
                        "set up before · Enter to check or repair".into(),
                        INK,
                    )
                } else {
                    ("AnimaOS".into(), "○", "not connected".into(), INK)
                }
            }
            Row::Chatgpt => {
                if chatgpt_ready {
                    (
                        "ChatGPT".into(),
                        "✓",
                        "tunnel ready · not verified in ChatGPT".into(),
                        Color::LightGreen,
                    )
                } else {
                    let state = app.status.as_ref().map_or("checking".to_owned(), |s| {
                        s.state.to_string().to_lowercase()
                    });
                    ("ChatGPT".into(), "○", state, INK)
                }
            }
        };
        let selected = i == app.row;
        let style = if selected {
            Style::default()
                .fg(BACKGROUND)
                .bg(VIOLET)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };
        lines.push(Line::from(vec![
            Span::styled(format!(" {mark} {name:<16}"), style),
            Span::styled(
                format!(" {}", safe(&status)),
                if selected {
                    style
                } else {
                    Style::default().fg(MUTED)
                },
            ),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(key("↑↓", "  choose an app"));
    lines.push(key("Enter", " connect, or repair if something's wrong"));
    lines.push(key("f", "  fix every app that needs it"));
    frame.render_widget(text(lines).block(panel(" Your apps ")), columns[0]);

    let broken = app
        .apps
        .iter()
        .filter(|s| matches!(s.state, State::Broken(_)))
        .count();
    let mut lines = vec![
        Line::from(safe(&app.workspace)),
        Line::from(""),
        key("i", "  Memory Inbox · review what's waiting for you"),
        key("p", "  pinned core and handoffs"),
        key("t", "  Test connection (deep check)"),
        key("r", "  refresh"),
        key("?", "  help"),
        key("q", "  quit"),
        Line::from(""),
        Line::from(format!(
            "ChatGPT start at sign-in: {}",
            match app.startup {
                Some(true) => "on",
                Some(false) => "off",
                None => "unknown - inspect before changing",
            }
        ))
        .style(Style::default().fg(MUTED)),
        Line::from(""),
    ];
    if broken > 0 {
        lines.push(
            Line::from(format!("{broken} app(s) need fixing. Press f."))
                .style(Style::default().fg(GOLD)),
        );
    } else {
        lines.push(Line::from("Your memory stays on this computer. Each app gets its own access, which you can revoke any time.").style(Style::default().fg(MUTED)));
    }
    frame.render_widget(text(lines).block(panel(" Your memory ")), columns[1]);
}
fn setup(frame: &mut Frame, area: Rect, app: &App) {
    let Some(form) = &app.form else {
        return;
    };
    let labels = [
        "Tunnel ID  (platform.openai.com → Settings → Tunnels)",
        "Organization ID  (optional)",
        "ChatGPT access file  (leave as suggested)",
        "tunnel-client program  (download: github.com/openai/tunnel-client)",
        "M.O.T.H.E.R program  (leave as suggested)",
        "OpenAI runtime key  (blank keeps the saved one; never paste it in a chat)",
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
    frame.render_widget(
        text(vec![
            Line::from("Connect once. Continue in every app.").style(Style::default().fg(VIOLET)),
            Line::from(""),
            Line::from("Claude Code, Codex, Hermes, Claude Desktop").style(Style::default().fg(GOLD)),
            Line::from("  Choose the app on the home screen and press Enter. M.O.T.H.E.R gives it its own"),
            Line::from("  access, adds itself to the app's settings (keeping a backup), and checks it works."),
            Line::from("  Restart the app afterwards. Press f any time something breaks."),
            Line::from(""),
            Line::from("AnimaOS").style(Style::default().fg(GOLD)),
            Line::from("  Start AnimaOS, choose AnimaOS and press Enter. Your owner key is found on this"),
            Line::from("  computer automatically. Pick the agent, choose \"Knows me\", and confirm. If old"),
            Line::from("  connections are in the way, press f to turn them off and connect cleanly."),
            Line::from(""),
            Line::from("ChatGPT (needs two things from OpenAI's website)").style(Style::default().fg(GOLD)),
            Line::from("  1. platform.openai.com → Settings → Tunnels: create a tunnel for your ChatGPT workspace."),
            Line::from("  2. platform.openai.com → API keys: create a runtime key with Tunnels Read + Use."),
            Line::from("  3. Install tunnel-client from github.com/openai/tunnel-client."),
            Line::from("  4. Choose ChatGPT, fill in the tunnel ID and key, save, and wait for \"tunnel ready\"."),
            Line::from("  5. In ChatGPT: Add custom MCP server → Connection: Tunnel → Authentication: none."),
            Line::from(""),
            Line::from("Your memory stays on this computer. Each app has its own access; `mother revoke <name>`")
                .style(Style::default().fg(MUTED)),
            Line::from("removes one without touching the others. Esc returns home.").style(Style::default().fg(MUTED)),
        ])
        .block(panel(" Help ")),
        area,
    );
}
