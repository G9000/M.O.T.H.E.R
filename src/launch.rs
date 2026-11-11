//! Owner-declared launchers: the only way M.O.T.H.E.R starts a harness.
//!
//! The owner lists each launcher in a private JSON file: an absolute command,
//! fixed arguments, an optional working folder and a timeout. No shell is
//! involved. `{prompt}` and `{task}` are replaced inside arguments, and the
//! prompt names the task by ID only, so saved text never decides what runs.
//! Every launch is an `action` record that moves from pending to its outcome.
use crate::{
    config::{create_private_file, data_directory, read_json},
    contracts::{self, valid_client},
    error::{Error, Result, invalid},
    store::{self, Store},
};
use rusqlite::{OptionalExtension, TransactionBehavior};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;

/// Actor recorded for launches the owner starts from this computer.
pub const OWNER: &str = "@owner-launch";
const DEFAULT_TIMEOUT: u64 = 1_800;
const MAX_TIMEOUT: u64 = 86_400;

/// One way to start a harness, declared by the owner.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Launcher {
    /// The grant name of the harness this launcher starts; the task is handed to it.
    pub harness: String,
    /// Absolute path of the program to run.
    pub command: PathBuf,
    /// Arguments; `{prompt}` and `{task}` are replaced in each one.
    #[serde(default)]
    pub args: Vec<String>,
    /// Absolute working folder; defaults to the current one.
    pub cwd: Option<PathBuf>,
    /// Seconds before the run is stopped and its outcome recorded as unknown.
    pub timeout_seconds: Option<u64>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LauncherFile {
    version: u32,
    launchers: BTreeMap<String, Launcher>,
}
impl Launcher {
    /// Why this launcher cannot run right now, if anything.
    pub fn problem(&self) -> Option<String> {
        if !valid_client(&self.harness) {
            return Some("harness must be a grant name".into());
        }
        if !self.command.is_absolute() {
            return Some("command must be an absolute path".into());
        }
        if !self.command.is_file() {
            return Some(format!("command not found: {}", self.command.display()));
        }
        if let Some(cwd) = &self.cwd
            && (!cwd.is_absolute() || !cwd.is_dir())
        {
            return Some("cwd must be an existing absolute folder".into());
        }
        if self
            .timeout_seconds
            .is_some_and(|t| t == 0 || t > MAX_TIMEOUT)
        {
            return Some(format!("timeoutSeconds must be 1 to {MAX_TIMEOUT}"));
        }
        if self.args.len() > 64 || self.args.iter().any(|a| a.len() > 4096) {
            return Some("too many or too long arguments".into());
        }
        None
    }
    fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds.unwrap_or(DEFAULT_TIMEOUT))
    }
}

/// Where the owner declares launchers.
pub fn launchers_path() -> Result<PathBuf> {
    Ok(data_directory()?.join("hearth").join("launchers.json"))
}
/// Read the owner's launchers. A missing file is an error that names where to create it.
pub fn load(path: &Path) -> Result<BTreeMap<String, Launcher>> {
    if !path.exists() {
        return Err(Error::domain(
            "LAUNCHERS_MISSING",
            format!(
                "No launchers are declared yet. Create {} as described in docs/hearth.md.",
                path.display()
            ),
        ));
    }
    let file: LauncherFile = serde_json::from_value(read_json(path, 64 * 1024)?)
        .map_err(|_| invalid(format!("{} is not a valid launchers file", path.display())))?;
    if file.version != 1 {
        return Err(invalid("Unsupported launchers file version"));
    }
    if let Some(name) = file.launchers.keys().find(|n| !valid_client(n)) {
        return Err(invalid(format!("Invalid launcher name: {name}")));
    }
    Ok(file.launchers)
}

/// Fixed instructions for the launched harness. Only the task ID varies.
pub fn prompt(task: &str) -> String {
    format!(
        "M.O.T.H.E.R handed you task {task}. It is listed under assignedToYou in mother_get_context. Read it with mother_get_record, do the work under your own permissions, and update the task's status with mother_write_record when you finish or are blocked. Save a checkpoint before you stop."
    )
}

/// What happened to one run.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// `succeeded`, `failed`, or `unknown` when the run was stopped at its timeout.
    pub status: &'static str,
    pub exit_code: Option<i32>,
    pub seconds: f64,
}

/// Run a launcher for a task, sending its output to `log`, and stop it at its timeout.
pub fn run(launcher: &Launcher, task: &str, log: &Path) -> Result<Outcome> {
    let text = prompt(task);
    let args: Vec<String> = launcher
        .args
        .iter()
        .map(|a| a.replace("{prompt}", &text).replace("{task}", task))
        .collect();
    let out = create_private_file(log)?;
    let err = out.try_clone()?;
    let mut command = Command::new(&launcher.command);
    command
        .args(&args)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    if let Some(cwd) = &launcher.cwd {
        command.current_dir(cwd);
    }
    let started = Instant::now();
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            fs::write(log, format!("Could not start the launcher: {error}\n"))?;
            return Ok(Outcome {
                status: "failed",
                exit_code: None,
                seconds: 0.0,
            });
        }
    };
    let finished = child.wait_timeout(launcher.timeout())?;
    let outcome = match finished {
        Some(status) => Outcome {
            status: if status.success() {
                "succeeded"
            } else {
                "failed"
            },
            exit_code: status.code(),
            seconds: started.elapsed().as_secs_f64(),
        },
        None => {
            let _ = child.kill();
            let _ = child.wait();
            Outcome {
                status: "unknown",
                exit_code: None,
                seconds: started.elapsed().as_secs_f64(),
            }
        }
    };
    Ok(outcome)
}

/// Private folder for run logs.
pub fn log_path(action: &str) -> Result<PathBuf> {
    let dir = data_directory()?.join("hearth").join("runs");
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&dir)?;
    Ok(dir.join(format!("{action}.log")))
}

impl Store {
    /// Check the task, hand it to the launcher's harness and record a pending launch.
    ///
    /// Refuses while an earlier launch of the same task is pending or unknown,
    /// unless `retry_unresolved` is set.
    pub fn begin_launch(
        &mut self,
        task: &str,
        name: &str,
        launcher: &Launcher,
        retry_unresolved: bool,
    ) -> Result<Value> {
        contracts::id(task)?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (record, _) = store::read_record(&tx, task)?;
        if record["kind"] != "task"
            || !matches!(
                record["status"].as_str(),
                Some("todo" | "doing" | "blocked")
            )
        {
            return Err(invalid(
                "Only open tasks (todo, doing or blocked) can be launched",
            ));
        }
        let unresolved: Option<String> = tx
            .query_row(
                "SELECT status FROM records WHERE kind='action' AND status IN ('pending','unknown') AND json_extract(payload,'$.actor')=?1 AND EXISTS(SELECT 1 FROM json_each(json_extract(payload,'$.references')) WHERE value=?2) ORDER BY status DESC LIMIT 1",
                [OWNER, task],
                |r| r.get(0),
            )
            .optional()?;
        if let Some(status) = unresolved
            && !retry_unresolved
        {
            return Err(Error::domain(
                "LAUNCH_UNRESOLVED",
                if status == "unknown" {
                    "An earlier launch of this task has an unknown outcome. Check what it did, then pass --retry-unknown to launch again."
                } else {
                    "An earlier launch of this task is still pending: it may be running, or it was interrupted. Check it, then pass --retry-unknown to launch again."
                },
            ));
        }
        crate::handoffs::set(&tx, task, Some(&launcher.harness), OWNER)?;
        let workspace = store::workspace(&tx)?;
        let short: String = task.chars().take(8).collect();
        let write = contracts::Write::parse(json!({
            "workspaceId": workspace["id"],
            "generation": workspace["generation"],
            "operationId": uuid::Uuid::new_v4().to_string(),
            "expectedRevision": 0,
            "record": {
                "kind": "action",
                "status": "pending",
                "title": format!("Launch {name} for task {short}"),
                "content": format!(
                    "The owner launched {name} ({}) for task {task}, handed to {}. Started {}.",
                    launcher.command.display(),
                    launcher.harness,
                    store::now()
                ),
                "destination": format!("launcher:{name}"),
                "references": [task],
            }
        }))?;
        let receipt = store::write_transaction(&tx, OWNER, &write)?;
        tx.commit()?;
        Ok(receipt["record"].clone())
    }

    /// Record how a launch ended, as the next revision of its action record.
    pub fn finish_launch(
        &mut self,
        action: &Value,
        outcome: &Outcome,
        log: &Path,
    ) -> Result<Value> {
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let workspace = store::workspace(&tx)?;
        let ending = match outcome.status {
            "unknown" => format!(
                "Stopped at its timeout after {:.0} seconds; what it did before stopping is unknown.",
                outcome.seconds
            ),
            status => format!(
                "Finished: {status}, exit code {}, after {:.0} seconds.",
                outcome
                    .exit_code
                    .map_or_else(|| "none".to_owned(), |c| c.to_string()),
                outcome.seconds
            ),
        };
        let mut record = action.clone();
        if let Some(fields) = record.as_object_mut() {
            for key in ["id", "revision", "actor", "createdAt", "updatedAt"] {
                fields.remove(key);
            }
        }
        record["status"] = json!(outcome.status);
        record["content"] = json!(format!(
            "{}\n{ending} Log: {}",
            action["content"].as_str().unwrap_or_default(),
            log.display()
        ));
        let write = contracts::Write::parse(json!({
            "workspaceId": workspace["id"],
            "generation": workspace["generation"],
            "operationId": uuid::Uuid::new_v4().to_string(),
            "id": action["id"],
            "expectedRevision": action["revision"],
            "record": record,
        }))?;
        let receipt = store::write_transaction(&tx, OWNER, &write)?;
        tx.commit()?;
        Ok(receipt["record"].clone())
    }
}
