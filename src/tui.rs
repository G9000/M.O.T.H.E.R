//! Native owner review interfaces for canonical and graph proposals.
mod garden_inbox;
mod inbox;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use mother::error::{Error, Result};
use std::{io::IsTerminal, path::PathBuf, time::Duration};

struct TerminalGuard;
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::execute!(std::io::stdout(), event::DisableBracketedPaste);
        ratatui::restore();
    }
}

/// Open the explicit graph owner inbox without loading the v1 connection dashboard.
pub fn run_garden(path: PathBuf) -> Result<()> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return Err(Error::domain(
            "TERMINAL_REQUIRED",
            "The garden inbox needs an interactive terminal.",
        ));
    }
    let mut inbox = garden_inbox::Inbox::open(path)?;
    let mut terminal = ratatui::try_init()?;
    let _guard = TerminalGuard;
    crossterm::execute!(std::io::stdout(), event::EnableBracketedPaste)?;
    loop {
        terminal.draw(|frame| inbox.render(frame, frame.area()))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Paste(text) => inbox.paste(&text),
            Event::Key(key) if key.kind != KeyEventKind::Release => {
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                    && !inbox.uncertain()
                {
                    break;
                }
                match inbox.input(key) {
                    Ok(true) => break,
                    Ok(false) => {}
                    Err(error) => inbox.report(error),
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Open the native canonical inbox only when input and output are real terminals.
pub fn run_native_inbox(path: PathBuf) -> Result<()> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return Err(Error::domain(
            "TERMINAL_REQUIRED",
            "The native v1 inbox needs an interactive terminal.",
        ));
    }
    let mut inbox = inbox::Inbox::open_native(path)?;
    let mut terminal = ratatui::try_init()?;
    let _guard = TerminalGuard;
    crossterm::execute!(std::io::stdout(), event::EnableBracketedPaste)?;
    loop {
        terminal.draw(|frame| inbox.render(frame, frame.area()))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Paste(text) => inbox.paste(&text),
            Event::Key(key) if key.kind != KeyEventKind::Release => {
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                    && !inbox.uncertain()
                {
                    break;
                }
                match inbox.input(key) {
                    Ok(true) => break,
                    Ok(false) => {}
                    Err(error) => inbox.message = error.to_string(),
                }
            }
            _ => {}
        }
    }
    Ok(())
}
