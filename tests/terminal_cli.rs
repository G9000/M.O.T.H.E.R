use std::process::{Command, Stdio};
use tempfile::TempDir;

fn run(home: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", home)
        .stdin(Stdio::null())
        .output()
        .expect("run synthetic terminal CLI")
}

#[test]
fn status_without_configuration_is_read_only_and_machine_readable() {
    let home = TempDir::new().unwrap();
    let result = run(home.path(), &["status", "--json"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let status: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(status["ready"], false);
    assert_eq!(
        std::fs::read_dir(home.path()).unwrap().count(),
        0,
        "inspection must not create settings or records"
    );
}

#[test]
fn explicit_tui_rejects_piped_input_without_control_sequences() {
    let home = TempDir::new().unwrap();
    let result = run(home.path(), &["tui"]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("terminal"));
    assert!(!result.stdout.contains(&0x1b));
}

#[test]
fn bare_noninteractive_cli_keeps_help_and_does_not_initialize() {
    let home = TempDir::new().unwrap();
    let result = run(home.path(), &[]);
    assert!(result.status.success());
    assert!(String::from_utf8_lossy(&result.stdout).contains("Usage:"));
    assert_eq!(std::fs::read_dir(home.path()).unwrap().count(), 0);
}

#[test]
fn noninteractive_connect_without_settings_never_prompts_or_creates_state() {
    let home = TempDir::new().unwrap();
    let result = run(home.path(), &["connect", "chatgpt"]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("setup"));
    assert_eq!(std::fs::read_dir(home.path()).unwrap().count(), 0);
}

#[test]
fn inbox_shortcut_requires_terminal_and_does_not_create_workspace() {
    let dir = tempfile::tempdir().unwrap();
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_mother"))
        .arg("inbox")
        .env("MOTHER_HOME", dir.path())
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("TERMINAL_REQUIRED"));
    assert!(!dir.path().join("context.sqlite").exists());
}

#[test]
fn animaos_setup_requires_terminal_without_creating_workspace() {
    let home = TempDir::new().unwrap();
    let out = run(home.path(), &["connect", "animaos"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("TERMINAL_REQUIRED"));
    assert_eq!(std::fs::read_dir(home.path()).unwrap().count(), 0);
}
