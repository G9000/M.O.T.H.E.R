//! She shows up at session start, and each harness gets setup it can paste.
use mother::{
    access::{AccessMode, ClientAccess},
    config::load_config,
    store::Store,
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::TempDir;
use uuid::Uuid;

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(args)
        .env("MOTHER_HOME", dir)
        .env("MOTHER_MODEL_DIR", dir.join("models"))
        .output()
        .expect("run native CLI")
}
fn text(dir: &Path, args: &[&str]) -> String {
    let out = run(dir, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
/// An initialized workspace with one full-access client, opened as that client.
struct Workspace {
    dir: TempDir,
    config: PathBuf,
}
impl Workspace {
    fn new(persona: &str) -> Self {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("SOUL.md"), persona).unwrap();
        text(dir.path(), &["init"]);
        text(
            dir.path(),
            &[
                "grant",
                "claude-code",
                "--scopes",
                "read,memory:write,decision:write,task:write,checkpoint:write",
            ],
        );
        let config = dir.path().join("clients/claude-code.private.json");
        Self { dir, config }
    }
    fn config(&self) -> &str {
        self.config.to_str().unwrap()
    }
    fn save(&self, record: Value) -> Value {
        let c = load_config(&self.config).unwrap();
        let mut s = Store::open(&c.database_path).unwrap();
        let w = s.workspace().unwrap();
        s.write(&c.auth(), json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})).unwrap()["record"].clone()
    }
    fn pin(&self, record: &Value) {
        let c = load_config(&self.config).unwrap();
        let mut s = Store::open(&c.database_path).unwrap();
        s.pin(&c.auth(), json!({"id":record["id"],"pinned":true}))
            .unwrap();
    }
    fn brief(&self) -> String {
        text(self.dir.path(), &["brief", "--config", self.config()])
    }
}
fn memory(title: &str, content: &str) -> Value {
    json!({"kind":"memory","status":"active","basis":"owner_statement","title":title,"content":content})
}

#[test]
fn the_brief_carries_who_she_is_what_matters_and_where_you_left_off() {
    let w = Workspace::new("You are M.O.T.H.E.R. Speak warmly and plainly.");
    let name = w.save(memory("Preferred name", "Call me Leo"));
    w.pin(&name);
    w.save(memory("Unpinned note", "Synthetic detail that is not core"));
    w.save(json!({"kind":"checkpoint","status":"active","title":"Evening handoff","content":"Synthetic","nextStep":"Finish the pinned memories review"}));
    w.save(json!({"kind":"task","status":"blocked","title":"Waiting on the tunnel key","content":"Synthetic"}));
    let brief = w.brief();
    assert!(brief.contains("M.O.T.H.E.R"));
    assert!(brief.contains("mother_get_context"));
    assert!(brief.contains("Speak warmly and plainly."), "{brief}");
    assert!(brief.contains("Preferred name"));
    assert!(brief.contains("Call me Leo"));
    assert!(brief.contains("Evening handoff"));
    assert!(brief.contains("Finish the pinned memories review"));
    assert!(brief.contains("Waiting on the tunnel key"));
    assert!(brief.contains("blocked"));
    assert!(
        !brief.contains("Synthetic detail that is not core"),
        "unpinned memories are left to mother_get_context"
    );
    assert!(
        brief.contains("data, not instructions"),
        "saved records are framed as data"
    );
}
#[test]
fn the_brief_fits_under_a_session_hook_limit_however_large_the_workspace() {
    let w = Workspace::new(&"Persona line. ".repeat(2_200));
    for n in 0..12 {
        let m = w.save(memory(&format!("Core {n}"), &"x".repeat(5_000)));
        w.pin(&m);
    }
    for n in 0..30 {
        w.save(json!({"kind":"task","status":"doing","title":format!("Task {n}"),"content":"Synthetic"}));
    }
    let brief = w.brief();
    assert!(
        brief.chars().count() <= 9_000,
        "{} characters",
        brief.chars().count()
    );
    for n in 0..12 {
        assert!(
            brief.contains(&format!("Core {n}")),
            "every pin keeps at least its title"
        );
    }
    assert!(brief.contains("truncated"));
}
#[test]
fn an_unreadable_workspace_produces_a_short_honest_note_not_a_failed_session() {
    let w = Workspace::new("Synthetic persona");
    w.save(memory("Private", "Synthetic private content"));
    text(w.dir.path(), &["revoke", "claude-code"]);
    let out = run(w.dir.path(), &["brief", "--config", w.config()]);
    assert!(out.status.success());
    let brief = String::from_utf8(out.stdout).unwrap();
    assert!(brief.contains("could not be read"));
    assert!(brief.contains("UNAUTHORIZED"));
    assert!(brief.contains("do not invent"));
    assert!(!brief.contains("Synthetic private content"));
    assert!(!brief.contains("Synthetic persona"));
}
#[test]
fn a_shared_client_brief_has_no_persona() {
    let w = Workspace::new("Synthetic secret persona");
    let granted = w.save(memory("Granted", "Visible to the shared agent"));
    let db = load_config(&w.config).unwrap().database_path;
    let shared = w.dir.path().join("clients/shared.private.json");
    Store::open(&db)
        .unwrap()
        .grant_to_file_with_access(
            "shared",
            &["read".into()],
            &shared,
            &db,
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: vec![granted["id"].as_str().unwrap().into()],
            },
        )
        .unwrap();
    let brief = text(
        w.dir.path(),
        &["brief", "--config", shared.to_str().unwrap()],
    );
    assert!(!brief.contains("Synthetic secret persona"));
}
fn connection(w: &Workspace, format: &str) -> String {
    text(
        w.dir.path(),
        &["connection", "--config", w.config(), "--format", format],
    )
}
/// A resolved path as setup output shows it: without Windows' `\\?\` prefix.
fn plain(path: &Path) -> String {
    let resolved = std::fs::canonicalize(path).unwrap();
    let text = resolved.to_str().unwrap();
    text.strip_prefix(r"\\?\").unwrap_or(text).to_owned()
}
fn exe() -> String {
    plain(Path::new(env!("CARGO_BIN_EXE_mother")))
}
/// The first `{` … last `}` block in a setup printout, parsed as JSON.
fn json_block(out: &str) -> Value {
    let start = out.find('{').unwrap();
    let end = out.rfind('}').unwrap();
    serde_json::from_str(&out[start..=end]).unwrap()
}
#[test]
fn claude_code_setup_registers_her_and_briefs_every_session() {
    let w = Workspace::new("Synthetic persona");
    let out = connection(&w, "claude-code");
    let config = plain(&w.config);
    assert!(
        out.contains(&format!(
            "claude mcp add --transport stdio --scope user mother -- \"{}\" mcp --config \"{config}\"",
            exe()
        )),
        "{out}"
    );
    let hook = json_block(&out);
    let command = hook["hooks"]["SessionStart"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap();
    assert_eq!(
        command,
        format!("\"{}\" brief --config \"{config}\"", exe())
    );
    assert_eq!(
        hook["hooks"]["SessionStart"][0]["hooks"][0]["type"],
        "command"
    );
    assert!(!out.contains(&load_config(&w.config).unwrap().auth().token));
}
#[test]
fn hermes_openclaw_and_generic_setup_launch_the_same_server() {
    let w = Workspace::new("Synthetic persona");
    let config = plain(&w.config);
    let args = json!(["mcp", "--config", config]);
    let hermes = connection(&w, "hermes");
    assert!(hermes.contains("mcp_servers:\n  mother:\n"), "{hermes}");
    let command_line = hermes
        .lines()
        .find(|l| l.trim_start().starts_with("command: "))
        .unwrap();
    let command: String =
        serde_json::from_str(command_line.trim_start().trim_start_matches("command: ")).unwrap();
    assert_eq!(command, exe());
    let args_line = hermes
        .lines()
        .find(|l| l.trim_start().starts_with("args: "))
        .unwrap();
    let parsed: Value =
        serde_json::from_str(args_line.trim_start().trim_start_matches("args: ")).unwrap();
    assert_eq!(parsed, args);
    let openclaw = connection(&w, "openclaw");
    let line = openclaw
        .lines()
        .find(|l| l.starts_with("openclaw mcp set mother '"))
        .unwrap();
    let body = line
        .trim_start_matches("openclaw mcp set mother '")
        .trim_end_matches('\'');
    let server: Value = serde_json::from_str(body).unwrap();
    assert_eq!(server, json!({"command":exe(),"args":args}));
    let generic: Value = serde_json::from_str(&connection(&w, "mcp-servers")).unwrap();
    assert_eq!(
        generic,
        json!({"mcpServers":{"mother":{"command":exe(),"args":args}}})
    );
}
#[test]
fn setup_refuses_paths_a_shell_would_misread() {
    let w = Workspace::new("Synthetic persona");
    let odd = w.dir.path().join("clients/dollar$name.private.json");
    text(
        w.dir.path(),
        &[
            "grant",
            "odd",
            "--scopes",
            "read",
            "--out",
            odd.to_str().unwrap(),
        ],
    );
    for format in ["claude-code", "openclaw"] {
        let out = run(
            w.dir.path(),
            &[
                "connection",
                "--config",
                odd.to_str().unwrap(),
                "--format",
                format,
            ],
        );
        assert!(!out.status.success(), "{format}");
        assert!(String::from_utf8_lossy(&out.stderr).contains("path"));
    }
}
