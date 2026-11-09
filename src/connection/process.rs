//! Bounded, shell-free tunnel-client execution.
use super::{Result, Settings, failure};
use std::{
    ffi::OsString,
    io::Read,
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;
use zeroize::Zeroizing;
const LIMIT: usize = 128 * 1024;
const DEADLINE: Duration = Duration::from_secs(25);
pub(super) struct Output {
    pub stdout: Vec<u8>,
    pub stderr: String,
    pub uncertain: bool,
    pub success: bool,
}
fn preserved_environment() -> Vec<(OsString, OsString)> {
    [
        "SYSTEMROOT",
        "WINDIR",
        "USERPROFILE",
        "LOCALAPPDATA",
        "APPDATA",
        "TEMP",
        "TMP",
        "HOME",
        "PATH",
        "LANG",
        "DBUS_SESSION_BUS_ADDRESS",
        "XDG_RUNTIME_DIR",
        "MOTHER_HOME",
    ]
    .into_iter()
    .filter_map(|key| std::env::var_os(key).map(|value| (OsString::from(key), value)))
    .collect()
}
fn build_command(settings: &Settings, args: &[String], key: Option<&str>) -> Command {
    let mut cmd = Command::new(&settings.tunnel_client);
    cmd.args(args)
        .env_clear()
        .envs(preserved_environment())
        .env("HEALTH_LISTEN_ADDR", "127.0.0.1:0")
        .env("MCP_STDIO_SEND_INITIALIZED_NOTIFICATION", "true")
        .env("LOG_HTTP_RAW_UNSAFE", "false")
        .env("CONTROL_PLANE_BASE_URL", "https://api.openai.com")
        .env(
            "CONTROL_PLANE_ORGANIZATION_ID",
            settings.organization_id.as_deref().unwrap_or(""),
        )
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(key) = key {
        cmd.env("MOTHER_TUNNEL_RUNTIME_KEY", key);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    cmd
}
pub(super) fn run(settings: &Settings, args: &[String], key: Option<&str>) -> Result<Output> {
    run_with_timeout(settings, args, key, DEADLINE)
}
pub(super) fn run_with_timeout(
    settings: &Settings,
    args: &[String],
    key: Option<&str>,
    deadline: Duration,
) -> Result<Output> {
    if deadline.is_zero() {
        return Ok(Output {
            stdout: Vec::new(),
            stderr: String::new(),
            uncertain: true,
            success: false,
        });
    }
    let started = Instant::now();
    let mut child = build_command(settings, args, key)
        .spawn()
        .map_err(|_| failure("Unable to launch the configured official tunnel-client."))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| failure("Cannot capture runtime status."))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| failure("Cannot capture runtime diagnostics."))?;
    let (tx, rx) = mpsc::channel();
    for (index, stream) in [
        (0, Box::new(stdout) as Box<dyn Read + Send>),
        (1, Box::new(stderr) as Box<dyn Read + Send>),
    ] {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut bytes = Zeroizing::new(Vec::new());
            let result = stream.take((LIMIT + 1) as u64).read_to_end(&mut bytes);
            let _ = tx.send((index, bytes, result.is_ok()));
        });
    }
    drop(tx);
    let exited = child
        .wait_timeout(deadline)
        .map_err(|_| failure("Runtime process outcome is unknown; inspect before retrying."))?;
    if exited.is_none() {
        let _ = child.kill();
        let _ = child.wait();
        return Ok(Output {
            stdout: Vec::new(),
            stderr: String::new(),
            uncertain: true,
            success: false,
        });
    }
    let mut out = [Zeroizing::new(Vec::new()), Zeroizing::new(Vec::new())];
    for _ in 0..2 {
        let remaining = deadline.saturating_sub(started.elapsed());
        match rx.recv_timeout(remaining) {
            Ok((i, bytes, true)) if bytes.len() <= LIMIT => out[i] = bytes,
            _ => {
                return Ok(Output {
                    stdout: Vec::new(),
                    stderr: String::new(),
                    uncertain: true,
                    success: false,
                });
            }
        }
    }
    Ok(Output {
        stdout: std::mem::take(&mut out[0]),
        stderr: String::from_utf8_lossy(&out[1]).into_owned(),
        uncertain: false,
        success: exited.is_some_and(|status| status.success()),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runtime_secret_is_child_environment_only_and_controls_are_explicit() {
        let s = Settings {
            version: 1,
            tunnel_id: "tunnel_test".into(),
            organization_id: Some("org-test".into()),
            client_config: "C:/client.json".into(),
            tunnel_client: "C:/tunnel.exe".into(),
            mother_exe: "C:/mother.exe".into(),
        };
        let cmd = build_command(
            &s,
            &["runtimes".into(), "connect".into()],
            Some("synthetic-secret"),
        );
        assert!(
            !cmd.get_args()
                .any(|a| a.to_string_lossy().contains("synthetic-secret"))
        );
        let env: std::collections::HashMap<_, _> = cmd
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.map(|v| v.to_string_lossy().into_owned()),
                )
            })
            .collect();
        assert_eq!(
            env["MOTHER_TUNNEL_RUNTIME_KEY"].as_deref(),
            Some("synthetic-secret")
        );
        assert_eq!(env["LOG_HTTP_RAW_UNSAFE"].as_deref(), Some("false"));
        assert_eq!(env["HEALTH_LISTEN_ADDR"].as_deref(), Some("127.0.0.1:0"));
        assert!(!env.contains_key("OPENAI_API_KEY"));
    }
}
