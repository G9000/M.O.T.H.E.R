//! Opt-in capture of explicitly selected passages and narrowly selected Claude Code turns.
mod selected;
mod source;
mod submission;

use crate::{
    config::{check_private_output, read_json},
    error::{Error, Result},
    private_paths::owner_directory,
    service::{LocalClient, ServiceClientConfig, load_service_config},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use source::{Scan, read_hook, scan};
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
    /// Read one owner-selected conversation from stdin.
    Select,
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
    operation_ids: Vec<String>,
    skipped_conflicts: usize,
    paused: bool,
}

enum Candidate {
    Claude(source::Candidate),
    Selected(selected::Selection),
}

impl Candidate {
    fn operation(&self) -> String {
        match self {
            Self::Claude(turn) => turn.operation().to_string(),
            Self::Selected(selection) => selection.operation_id.clone(),
        }
    }
}

fn capture_error(message: &str) -> Error {
    Error::domain("CAPTURE_UNAVAILABLE", message)
}

/// Inspect journaled capture operations, or explicitly retry a single original
/// request. Only counts and operation/proposal/record identifiers leave this path.
pub fn recovery(path: &Path, operation: Option<Uuid>, retry: bool) -> Result<serde_json::Value> {
    let initial = policy(path)?;
    if retry && (!initial.enabled || operation.is_none()) {
        return Err(capture_error(
            "Retry requires an enabled capture policy and one original operation UUID.",
        ));
    }
    let state = initial
        .state_dir
        .as_deref()
        .filter(|p| p.is_absolute())
        .ok_or_else(|| capture_error("Select an absolute private capture state directory."))?;
    owner_directory(state)?;
    let config_path = initial
        .service_config
        .as_deref()
        .ok_or_else(|| capture_error("Select a private service configuration."))?;
    check_private_output(config_path)?;
    let config = load_service_config(config_path)?;
    let call = capture_transport(&config)?;
    let sink = submission::Inbox::new(
        &call,
        state,
        &config.client_id,
        &config.workspace_id,
        &config.generation,
    );
    sink.recover(operation, retry, || Ok(policy(path)? == initial))
}

fn capture_transport(
    config: &ServiceClientConfig,
) -> Result<impl Fn(&str, &serde_json::Value) -> Result<serde_json::Value>> {
    let service = match LocalClient::new(config.clone()) {
        Ok(service) => Some(service),
        Err(e) if e.code() == "STORE_UNAVAILABLE" => None,
        Err(e) => return Err(e),
    };
    Ok(move |name: &str, args: &serde_json::Value| {
        service
            .as_ref()
            .ok_or_else(|| {
                Error::domain(
                    "STORE_UNAVAILABLE",
                    "Cannot reach the configured owner service.",
                )
            })?
            .call(name, args)
    })
}

/// Read selected input or one configured Claude Code source and optionally submit for review.
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
    let candidates = if matches!(request, CaptureRequest::Select) {
        report.coverage.selected_items = 1;
        vec![Candidate::Selected(selected::read(hook_input)?)]
    } else {
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
            CaptureRequest::Select => unreachable!(),
        };
        let (coverage, candidates) = scan(root, session, window)?;
        report.coverage = coverage;
        candidates.into_iter().map(Candidate::Claude).collect()
    };
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
    let config = load_service_config(config)?;
    let call = capture_transport(&config)?;
    let sink = submission::Inbox::new(
        &call,
        state,
        &config.client_id,
        &config.workspace_id,
        &config.generation,
    );
    for candidate in candidates {
        if policy(path)? != initial {
            report.paused = true;
            break;
        }
        match sink.submit(&candidate, || Ok(policy(path)? == initial))? {
            submission::Outcome::Submitted => {
                report.submitted += 1;
                report.operation_ids.push(candidate.operation());
            }
            submission::Outcome::AlreadyReceived => {
                report.previously_submitted += 1;
                report.operation_ids.push(candidate.operation());
            }
            submission::Outcome::NoUniqueCurrentMatch => report.skipped_conflicts += 1,
            submission::Outcome::Paused => {
                report.paused = true;
                break;
            }
        }
    }
    Ok(report)
}
