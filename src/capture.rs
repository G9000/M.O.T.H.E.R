//! Opt-in Claude Code adapter. The source reader never sends a transcript to the workspace.
mod source;
mod submission;

use crate::{
    config::{check_private_output, read_json},
    error::{Error, Result},
    private_paths::owner_directory,
    service::{LocalClient, load_service_config},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use source::{Candidate, Scan, read_hook, scan};
use std::{
    io::Read,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Policy {
    #[serde(default)]
    enabled: bool,
    transcript_dir: Option<PathBuf>,
    state_dir: Option<PathBuf>,
    service_config: Option<PathBuf>,
}

fn policy(path: &Path) -> Result<Policy> {
    check_private_output(path)?;
    serde_json::from_value(read_json(path, 16_384)?).map_err(Into::into)
}

/// An explicit Claude Code end-of-session invocation or bounded historical scan.
pub enum CaptureRequest {
    SessionEnd {
        session: Option<Uuid>,
    },
    Backfill {
        from: NaiveDate,
        before: NaiveDate,
        submit: bool,
    },
}

/// Content-free coverage and submission counts for the selected source and window.
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureReport {
    enabled: bool,
    preview: bool,
    from: Option<NaiveDate>,
    before: Option<NaiveDate>,
    coverage: Scan,
    submitted: usize,
    previously_submitted: usize,
    skipped_conflicts: usize,
    paused: bool,
}

fn capture_error(message: &str) -> Error {
    Error::domain("CAPTURE_UNAVAILABLE", message)
}

/// Read only the selected Claude Code project directory and optionally submit to the review inbox.
/// Each invocation reloads owner policy and checks the same policy before each submission.
pub fn run(path: &Path, request: CaptureRequest, hook_input: impl Read) -> Result<CaptureReport> {
    let initial = policy(path)?;
    let mut report = CaptureReport {
        enabled: initial.enabled,
        ..CaptureReport::default()
    };
    let mut window = None;
    if let CaptureRequest::Backfill {
        from,
        before,
        submit,
    } = &request
    {
        let days = (*before - *from).num_days();
        if !(1..=366).contains(&days) {
            return Err(capture_error(
                "Choose a UTC date range of one through 366 days.",
            ));
        }
        window = Some((*from, *before));
        report.from = Some(*from);
        report.before = Some(*before);
        report.preview = !submit;
    }
    if !initial.enabled {
        return Ok(report);
    }
    if policy(path)? != initial {
        return Err(capture_error(
            "The policy changed during startup; restart capture.",
        ));
    }
    let root = initial
        .transcript_dir
        .as_deref()
        .ok_or_else(|| capture_error("Select one Claude Code project transcript directory."))?;
    if !root.is_absolute() {
        return Err(capture_error(
            "The selected transcript directory must be absolute.",
        ));
    }
    owner_directory(root)?;
    let session = match request {
        CaptureRequest::SessionEnd {
            session: Some(session),
        } => Some(session),
        CaptureRequest::SessionEnd { session: None } => Some(read_hook(hook_input, root)?),
        CaptureRequest::Backfill { .. } => None,
    };
    let (coverage, candidates) = scan(root, session, window)?;
    report.coverage = coverage;
    if report.preview || candidates.is_empty() {
        return Ok(report);
    }
    let state = initial
        .state_dir
        .as_deref()
        .ok_or_else(|| capture_error("Select a private capture state directory."))?;
    if !state.is_absolute() {
        return Err(capture_error(
            "The capture state directory must be absolute.",
        ));
    }
    owner_directory(state)?;
    let config = initial
        .service_config
        .as_deref()
        .ok_or_else(|| capture_error("Select a private service client configuration."))?;
    check_private_output(config)?;
    let service = LocalClient::new(load_service_config(config)?)?;
    submit(&mut report, path, &initial, &service, state, candidates)?;
    Ok(report)
}

fn submit(
    report: &mut CaptureReport,
    path: &Path,
    initial: &Policy,
    service: &LocalClient,
    state: &Path,
    candidates: Vec<Candidate>,
) -> Result<()> {
    let call = |name: &str, args: &serde_json::Value| service.call(name, args);
    let config = service.config();
    let sink = submission::Inbox::new(
        &call,
        state,
        &config.client_id,
        &config.workspace_id,
        &config.generation,
    );
    for candidate in candidates {
        if policy(path)? != *initial {
            report.paused = true;
            break;
        }
        match sink.submit(&candidate, || Ok(policy(path)? == *initial))? {
            submission::Outcome::Submitted => report.submitted += 1,
            submission::Outcome::AlreadyReceived => report.previously_submitted += 1,
            submission::Outcome::NoUniqueCurrentMatch => report.skipped_conflicts += 1,
            submission::Outcome::Paused => {
                report.paused = true;
                break;
            }
        }
    }
    Ok(())
}
