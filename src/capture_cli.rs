//! Owner-configured entry point for Claude Code capture and backfill.
use clap::Subcommand;
use mother::error::Result;
use std::{io, path::PathBuf};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub(crate) enum CaptureCommand {
    /// Capture one opted-in session. Without --session, read Claude Code SessionEnd JSON on stdin.
    SessionEnd {
        #[arg(long)]
        session: Option<Uuid>,
    },
    /// Preview an inclusive UTC start date and exclusive UTC end date. Add --submit to run or resume.
    Backfill {
        #[arg(long)]
        from: chrono::NaiveDate,
        #[arg(long)]
        before: chrono::NaiveDate,
        #[arg(long)]
        submit: bool,
    },
}

pub(crate) fn run(policy: PathBuf, command: CaptureCommand) -> Result<()> {
    use mother::capture::{CaptureRequest, run};
    let request = match command {
        CaptureCommand::SessionEnd { session } => CaptureRequest::SessionEnd { session },
        CaptureCommand::Backfill {
            from,
            before,
            submit,
        } => CaptureRequest::Backfill {
            from,
            before,
            submit,
        },
    };
    let result = run(&policy, request, io::stdin().lock())?;
    serde_json::to_writer(io::stdout().lock(), &result)?;
    println!();
    Ok(())
}
