//! Owner entry point for the standalone native shared garden.
use clap::{CommandFactory, Parser, Subcommand};
use mother::error::Result;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "mother",
    version,
    about = "M.O.T.H.E.R native shared garden (SQLite-free runtime)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}
#[derive(Debug, Subcommand)]
enum Command {
    /// Capture selected Claude Code conversation items for owner review.
    Capture {
        #[arg(long)]
        policy: PathBuf,
        #[command(subcommand)]
        command: crate::capture_cli::CaptureCommand,
    },
    /// Native storage, local owner service and explicit cutover inspection.
    Garden {
        #[command(subcommand)]
        command: crate::garden_cli::DeferredGardenCommand,
    },
    /// Bridge the configured native store or running local owner to MCP stdio.
    Mcp {
        #[arg(long)]
        config: PathBuf,
    },
}

pub async fn run() -> Result<()> {
    match Cli::parse().command {
        None => {
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
        Some(Command::Capture { policy, command }) => crate::capture_cli::run(policy, command),
        Some(Command::Mcp { config }) => crate::mcp::serve(&config).await,
        Some(Command::Garden {
            command:
                crate::garden_cli::DeferredGardenCommand(crate::garden_cli::GardenCommand::Mcp {
                    config,
                }),
        }) => crate::mcp::serve_garden(&config).await,
        Some(Command::Garden { command }) => crate::garden_cli::execute_garden(command.0),
    }
}
