//! Bounded, read-only verification of this executable's local MCP transport.
use mother::config::load_config;
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

const MAX_BYTES: u64 = 2 * 1024 * 1024;
const MAX_MESSAGES: usize = 32;
const KINDS: [&str; 6] = [
    "memory",
    "decision",
    "task",
    "checkpoint",
    "artifact",
    "action",
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Inventory {
    pub total_records: u64,
    pub recallable_records: u64,
    pub by_kind: BTreeMap<String, u64>,
    pub recallable_by_kind: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HealthReport {
    pub scope: &'static str,
    pub read_verified: bool,
    pub status: &'static str,
    pub client_id: Option<String>,
    pub executable: PathBuf,
    pub executable_sha256: Option<String>,
    pub server_version: Option<String>,
    pub workspace_id: Option<String>,
    pub generation: Option<String>,
    pub inventory: Option<Inventory>,
}
impl HealthReport {
    fn new(executable: &Path) -> Self {
        Self {
            scope: "LOCAL MCP read not verified",
            read_verified: false,
            status: "not_started",
            client_id: None,
            executable: executable.into(),
            executable_sha256: None,
            server_version: None,
            workspace_id: None,
            generation: None,
            inventory: None,
        }
    }
}

type ProbeResult<T> = Result<T, &'static str>;

// The child is always the current MOTHER executable, never a command supplied by
// a private config. Its stdout is bounded before JSON parsing; no stderr is kept.
struct Session {
    child: Child,
    stdin: ChildStdin,
    reader: Option<JoinHandle<()>>,
    replies: mpsc::Receiver<ProbeResult<Value>>,
    deadline: Instant,
    cancel: Arc<AtomicBool>,
    messages: usize,
}
impl Session {
    fn start(
        exe: &Path,
        config: &Path,
        deadline: Instant,
        cancel: Arc<AtomicBool>,
    ) -> ProbeResult<Self> {
        let mut command = Command::new(exe);
        command
            .args(["mcp", "--config"])
            .arg(config)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        Self::start_command(command, deadline, cancel)
    }
    fn start_command(
        mut command: Command,
        deadline: Instant,
        cancel: Arc<AtomicBool>,
    ) -> ProbeResult<Self> {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut child = command.spawn().map_err(|_| "spawn_failed")?;
        let Some(stdin) = child.stdin.take() else {
            let _ = child.kill();
            let _ = child.wait();
            return Err("transport_failed");
        };
        let Some(stdout) = child.stdout.take() else {
            let _ = child.kill();
            let _ = child.wait();
            return Err("transport_failed");
        };
        let (tx, replies) = mpsc::sync_channel(MAX_MESSAGES + 1);
        let reader = thread::spawn(move || {
            let mut input = BufReader::new(stdout).take(MAX_BYTES + 1);
            let mut consumed = 0u64;
            for index in 0..=MAX_MESSAGES {
                let mut bytes = Vec::new();
                let result = match input.read_until(b'\n', &mut bytes) {
                    Ok(0) | Err(_) => Err("transport_closed"),
                    Ok(n) => {
                        consumed += n as u64;
                        if consumed > MAX_BYTES {
                            Err("output_limit")
                        } else if index == MAX_MESSAGES {
                            Err("message_limit")
                        } else {
                            serde_json::from_slice(&bytes).map_err(|_| "malformed_response")
                        }
                    }
                };
                let failed = result.is_err();
                // Enough slots for every permitted message plus the limit error;
                // the reader never blocks when cancellation joins it.
                if tx.try_send(result).is_err() || failed {
                    break;
                }
            }
        });
        Ok(Self {
            child,
            stdin,
            reader: Some(reader),
            replies,
            deadline,
            cancel,
            messages: 0,
        })
    }
    fn send(&mut self, message: Value) -> ProbeResult<()> {
        // Four fixed, tiny messages fit within the OS pipe buffer even if a
        // broken server never reads. No record/config text is sent here.
        writeln!(self.stdin, "{message}")
            .and_then(|()| self.stdin.flush())
            .map_err(|_| "transport_closed")
    }
    fn rpc(&mut self, id: u64, method: &str, params: Value) -> ProbeResult<Value> {
        self.send(json!({"jsonrpc":"2.0", "id":id,"method":method,"params":params}))?;
        loop {
            if self.cancel.load(Ordering::Relaxed) {
                return Err("cancelled");
            }
            let remaining = self.deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err("timeout");
            }
            let value = match self
                .replies
                .recv_timeout(remaining.min(Duration::from_millis(25)))
            {
                Ok(result) => result?,
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(_) => return Err("transport_closed"),
            };
            self.messages += 1;
            if self.messages > MAX_MESSAGES {
                return Err("message_limit");
            }
            if value["jsonrpc"] != "2.0" {
                return Err("malformed_response");
            }
            if value["id"] != id {
                continue;
            }
            if value.get("error").is_some() {
                return Err("protocol_error");
            }
            return value.get("result").cloned().ok_or("malformed_response");
        }
    }
    fn stop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        self.stop();
    }
}

fn identity(value: &Value) -> ProbeResult<String> {
    let text = value.as_str().ok_or("malformed_response")?;
    uuid::Uuid::parse_str(text).map_err(|_| "malformed_response")?;
    Ok(text.to_owned())
}
fn inventory(value: &Value) -> ProbeResult<Inventory> {
    let counts = |field: &str| -> ProbeResult<BTreeMap<String, u64>> {
        if value[field]
            .as_object()
            .is_none_or(|map| map.len() != KINDS.len())
        {
            return Err("malformed_inventory");
        }
        KINDS
            .into_iter()
            .map(|kind| {
                Ok((
                    kind.into(),
                    value[field][kind].as_u64().ok_or("malformed_inventory")?,
                ))
            })
            .collect()
    };
    let inventory = Inventory {
        total_records: value["totalRecords"]
            .as_u64()
            .ok_or("malformed_inventory")?,
        recallable_records: value["recallableRecords"]
            .as_u64()
            .ok_or("malformed_inventory")?,
        by_kind: counts("byKind")?,
        recallable_by_kind: counts("recallableByKind")?,
    };
    let sum = |map: &BTreeMap<String, u64>| {
        map.values()
            .try_fold(0u64, |total, n| total.checked_add(*n))
    };
    if sum(&inventory.by_kind) != Some(inventory.total_records)
        || sum(&inventory.recallable_by_kind) != Some(inventory.recallable_records)
        || KINDS
            .iter()
            .any(|kind| inventory.recallable_by_kind[*kind] > inventory.by_kind[*kind])
    {
        return Err("malformed_inventory");
    }
    Ok(inventory)
}
fn run_probe(
    report: &mut HealthReport,
    config: &Path,
    timeout: Duration,
    cancel: Arc<AtomicBool>,
    selected_database: Option<&Path>,
) -> ProbeResult<()> {
    let deadline = Instant::now() + timeout;
    let config = std::path::absolute(config).map_err(|_| "invalid_config")?;
    let parsed = client_config(&config)?;
    if let Some(database) = selected_database {
        let selected = database
            .canonicalize()
            .map_err(|_| "workspace_unavailable")?;
        let configured = parsed
            .database_path
            .canonicalize()
            .map_err(|_| "workspace_unavailable")?;
        if selected != configured {
            return Err("workspace_mismatch");
        }
    }
    let expected = expected_workspace(&parsed.database_path)?;
    report.client_id = Some(parsed.client_id);
    // Stream hashing rather than loading the executable into memory.
    let mut file = File::open(&report.executable).map_err(|_| "executable_unreadable")?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err("cancelled");
        }
        if Instant::now() >= deadline {
            return Err("timeout");
        }
        let size = file
            .read(&mut buffer)
            .map_err(|_| "executable_unreadable")?;
        if size == 0 {
            break;
        }
        hash.update(&buffer[..size]);
    }
    report.executable_sha256 = Some(format!("{:x}", hash.finalize()));
    let mut session = Session::start(&report.executable, &config, deadline, cancel)?;
    check_session(report, &mut session, &expected)
}
pub(crate) fn client_config(path: &Path) -> ProbeResult<mother::config::ClientConfig> {
    if !std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file()) {
        return Err("invalid_config");
    }
    load_config(path).map_err(|_| "invalid_config")
}
fn expected_workspace(database: &Path) -> ProbeResult<Value> {
    if !std::fs::symlink_metadata(database).is_ok_and(|metadata| metadata.file_type().is_file()) {
        return Err("workspace_unavailable");
    }
    let connection =
        rusqlite::Connection::open_with_flags(database, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|_| "workspace_unavailable")?;
    connection
        .busy_timeout(Duration::from_millis(100))
        .map_err(|_| "workspace_unavailable")?;
    let raw: String = connection
        .query_row(
            "SELECT value FROM metadata WHERE key='workspace' AND length(value)<=4096",
            [],
            |row| row.get(0),
        )
        .map_err(|_| "workspace_unavailable")?;
    let value: Value = serde_json::from_str(&raw).map_err(|_| "workspace_unavailable")?;
    identity(&value["id"])?;
    identity(&value["generation"])?;
    Ok(value)
}
fn check_session(
    report: &mut HealthReport,
    session: &mut Session,
    expected: &Value,
) -> ProbeResult<()> {
    let initialized = session.rpc(1, "initialize", json!({"protocolVersion":"2025-06-18", "capabilities":{}, "clientInfo":{"name":"mother-local-health", "version":env!("CARGO_PKG_VERSION")}}))?;
    if initialized["serverInfo"]["name"] != "mother-workspace" {
        return Err("unexpected_server");
    }
    let version = initialized["serverInfo"]["version"]
        .as_str()
        .ok_or("malformed_response")?;
    if version.len() > 80
        || !version
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b".-+".contains(&c))
    {
        return Err("malformed_response");
    }
    report.server_version = Some(version.into());
    session.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;
    let catalog = session.rpc(2, "tools/list", json!({}))?;
    let tools = catalog["tools"].as_array().ok_or("malformed_response")?;
    if !tools
        .iter()
        .any(|tool| tool["name"] == "mother_get_context")
    {
        return Err("context_tool_unavailable");
    }
    let result = session.rpc(
        3,
        "tools/call",
        json!({"name":"mother_get_context","arguments":{}}),
    )?;
    if result["isError"] == true {
        return Err("read_denied");
    }
    let context = result
        .get("structuredContent")
        .ok_or("malformed_response")?;
    report.workspace_id = Some(identity(&context["workspace"]["id"])?);
    report.generation = Some(identity(&context["workspace"]["generation"])?);
    if context["workspace"]["id"] != expected["id"]
        || context["workspace"]["generation"] != expected["generation"]
    {
        return Err("workspace_mismatch");
    }
    report.read_verified = true;
    report.scope = "LOCAL MCP read verified";
    let Some(counts) = context.get("inventory") else {
        return Err("inventory_unsupported");
    };
    report.inventory = Some(inventory(counts)?);
    Ok(())
}
fn probe_with(
    executable: &Path,
    config: &Path,
    timeout: Duration,
    cancel: Arc<AtomicBool>,
    selected_database: Option<&Path>,
) -> HealthReport {
    let mut report = HealthReport::new(executable);
    report.status = match run_probe(&mut report, config, timeout, cancel, selected_database) {
        Ok(()) => "verified",
        Err(status) => status,
    };
    report
}
pub(crate) fn probe(config: &Path) -> HealthReport {
    match std::env::current_exe() {
        Ok(exe) => probe_with(
            &exe,
            config,
            Duration::from_secs(10),
            Arc::new(AtomicBool::new(false)),
            None,
        ),
        Err(_) => {
            let mut report = HealthReport::new(Path::new(""));
            report.status = "executable_unavailable";
            report
        }
    }
}

/// Owns its worker until completion or cancellation; leaving the page joins it.
pub(crate) struct ProbeTask {
    cancel: Arc<AtomicBool>,
    receiver: mpsc::Receiver<HealthReport>,
    worker: Option<JoinHandle<()>>,
}
impl ProbeTask {
    pub fn start(config: PathBuf, database: PathBuf) -> Self {
        let cancel = Arc::new(AtomicBool::new(false));
        let worker_cancel = Arc::clone(&cancel);
        let (sender, receiver) = mpsc::channel();
        let worker = thread::spawn(move || {
            let report = match std::env::current_exe() {
                Ok(exe) => probe_with(
                    &exe,
                    &config,
                    Duration::from_secs(10),
                    worker_cancel,
                    Some(&database),
                ),
                Err(_) => {
                    let mut report = HealthReport::new(Path::new(""));
                    report.status = "executable_unavailable";
                    report
                }
            };
            let _ = sender.send(report);
        });
        Self {
            cancel,
            receiver,
            worker: Some(worker),
        }
    }
    pub fn poll(&mut self) -> Option<HealthReport> {
        self.receiver.try_recv().ok()
    }
}
impl Drop for ProbeTask {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace() -> Value {
        json!({"id":"11111111-1111-4111-8111-111111111111","generation":"22222222-2222-4222-8222-222222222222"})
    }
    fn context() -> Value {
        let counts =
            json!({"memory":0,"decision":1,"task":2,"checkpoint":0,"artifact":0,"action":0});
        json!({"workspace":workspace(),"persona":{"content":"SECRET PERSONA"},"records":[{"content":"SECRET CONTENT"}],"inventory":{"totalRecords":3,"recallableRecords":3,"byKind":counts,"recallableByKind":counts}})
    }
    fn replies(context: Value) -> Vec<Value> {
        vec![
            json!({"jsonrpc":"2.0","id":1,"result":{"serverInfo":{"name":"mother-workspace","version":"0.4.0"}}}),
            json!({"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"mother_get_context"}]}}),
            json!({"jsonrpc":"2.0","id":3,"result":{"structuredContent":context}}),
        ]
    }
    // An upper bound only: each scripted child answers at once, but a cold
    // PowerShell start on a loaded Windows runner can take several seconds.
    const REPLY: Duration = Duration::from_secs(25);
    fn script_command(windows: &str, unix: &str) -> Command {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            let _ = unix;
            let mut command = Command::new("powershell.exe");
            command.args(["-NoProfile", "-NonInteractive", "-Command", windows]);
            command.creation_flags(0x08000000);
            command
        }
        #[cfg(not(windows))]
        {
            let _ = windows;
            let mut command = Command::new("/bin/sh");
            command.args(["-c", unix]);
            command
        }
    }
    fn session(messages: &[Value], timeout: Duration) -> Session {
        let mut windows = String::new();
        let mut unix = String::new();
        for message in messages {
            let text = message.to_string();
            windows.push_str(&format!(
                "[Console]::WriteLine('{}');",
                text.replace('\'', "''")
            ));
            unix.push_str(&format!(
                "printf '%s\\n' '{}';",
                text.replace('\'', "'\\''")
            ));
        }
        windows.push_str("[Threading.Thread]::Sleep(30000)");
        unix.push_str("exec sleep 30");
        Session::start_command(
            script_command(&windows, &unix),
            Instant::now() + timeout,
            Arc::new(AtomicBool::new(false)),
        )
        .unwrap()
    }
    #[test]
    fn protocol_projects_only_validated_identity_and_inventory() {
        let mut messages = replies(context());
        messages.insert(0, json!({"jsonrpc":"2.0","method":"notifications/message","params":{"data":"SECRET LOG"}}));
        let mut session = session(&messages, REPLY);
        let mut report = HealthReport::new(Path::new("synthetic-mother"));
        assert_eq!(
            check_session(&mut report, &mut session, &workspace()),
            Ok(())
        );
        assert!(report.read_verified);
        let inventory = report.inventory.as_ref().unwrap();
        assert_eq!(inventory.total_records, 3);
        assert_eq!(inventory.by_kind["memory"], 0);
        assert_eq!(inventory.by_kind["task"], 2);
        assert!(!serde_json::to_string(&report).unwrap().contains("SECRET"));
    }
    #[test]
    fn old_context_is_verified_read_but_counts_are_unsupported() {
        let mut value = context();
        value.as_object_mut().unwrap().remove("inventory");
        let mut session = session(&replies(value), REPLY);
        let mut report = HealthReport::new(Path::new("synthetic"));
        assert_eq!(
            check_session(&mut report, &mut session, &workspace()),
            Err("inventory_unsupported")
        );
        assert!(report.read_verified);
        assert!(report.inventory.is_none());
    }
    #[test]
    fn foreign_workspace_and_generation_fail_verification() {
        for field in ["id", "generation"] {
            let mut value = context();
            value["workspace"][field] = json!("33333333-3333-4333-8333-333333333333");
            let mut session = session(&replies(value), REPLY);
            let mut report = HealthReport::new(Path::new("synthetic"));
            assert_eq!(
                check_session(&mut report, &mut session, &workspace()),
                Err("workspace_mismatch")
            );
            assert!(!report.read_verified);
        }
    }
    #[test]
    fn malformed_counts_never_become_zero() {
        for mutation in 0..5 {
            let mut value = context()["inventory"].clone();
            match mutation {
                0 => {
                    value["byKind"].as_object_mut().unwrap().remove("memory");
                }
                1 => value["byKind"]["unknown"] = json!(0),
                2 => value["totalRecords"] = json!(4),
                3 => value["byKind"]["memory"] = json!(-1),
                _ => value["recallableByKind"]["memory"] = json!(u64::MAX),
            }
            assert!(inventory(&value).is_err(), "mutation {mutation}");
        }
    }
    #[test]
    fn timeout_and_cancellation_kill_reap_child_and_join_reader() {
        let start = Instant::now();
        for cancelled in [false, true] {
            let mut session = session(&[], Duration::from_millis(80));
            session.cancel.store(cancelled, Ordering::Relaxed);
            assert_eq!(
                session.rpc(1, "initialize", json!({})),
                Err(if cancelled { "cancelled" } else { "timeout" })
            );
            // Exercise the same cleanup operation that Drop calls on every
            // production return path, rather than pre-cleaning the child.
            session.stop();
            assert!(session.child.try_wait().unwrap().is_some());
            assert!(session.reader.is_none());
            drop(session);
        }
        assert!(start.elapsed() < Duration::from_secs(3));
    }
    #[test]
    fn malformed_and_excessive_messages_fail_closed() {
        let messages =
            vec![json!({"jsonrpc":"2.0","method":"notifications/message"}); MAX_MESSAGES + 1];
        let mut session = session(&messages, REPLY);
        assert_eq!(
            session.rpc(1, "initialize", json!({})),
            Err("message_limit")
        );
        drop(session);
        let command = script_command(
            "[Console]::WriteLine('SECRET malformed');[Threading.Thread]::Sleep(30000)",
            "printf '%s\\n' 'SECRET malformed'; exec sleep 30",
        );
        let mut session = Session::start_command(
            command,
            Instant::now() + REPLY,
            Arc::new(AtomicBool::new(false)),
        )
        .unwrap();
        assert_eq!(
            session.rpc(1, "initialize", json!({})),
            Err("malformed_response")
        );
    }
    #[test]
    fn newline_free_output_is_bounded_and_reaped() {
        let command = script_command(
            "[Console]::Write(('x' * 2100000));[Threading.Thread]::Sleep(30000)",
            "head -c 2100000 /dev/zero; exec sleep 30",
        );
        let mut session = Session::start_command(
            command,
            Instant::now() + REPLY,
            Arc::new(AtomicBool::new(false)),
        )
        .unwrap();
        assert_eq!(session.rpc(1, "initialize", json!({})), Err("output_limit"));
    }
}
