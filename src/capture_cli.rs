//! Owner-configured entry point for Claude Code capture and backfill.
use clap::Subcommand;
use mother::error::Result;
use std::{io, path::PathBuf};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub(crate) enum CaptureCommand {
    /// Propose one explicitly selected conversation supplied as JSON on stdin.
    Select,
    /// Inspect capture journals and current review receipts. Omit --operation to list this configuration's saves.
    Status {
        #[arg(long)]
        operation: Option<Uuid>,
    },
    /// Reconcile one journal and retry its exact request if the owner confirms the operation is missing.
    Retry {
        #[arg(long)]
        operation: Uuid,
    },
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
        CaptureCommand::Select => CaptureRequest::Select,
        CaptureCommand::Status { operation } => {
            return print(mother::capture::recovery(&policy, operation, false)?);
        }
        CaptureCommand::Retry { operation } => {
            return print(mother::capture::recovery(&policy, Some(operation), true)?);
        }
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
    print(run(&policy, request, io::stdin().lock())?)
}

fn print(result: impl serde::Serialize) -> Result<()> {
    serde_json::to_writer(io::stdout().lock(), &result)?;
    println!();
    Ok(())
}
