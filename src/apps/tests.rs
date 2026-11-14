use super::*;
use std::cell::RefCell;

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mother-apps-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn workspace(dir: &Path) -> PathBuf {
    let root = dir.join("MOTHER");
    fs::create_dir_all(&root).unwrap();
    Store::initialize(&root.join("context.sqlite"), "I am a test persona.").unwrap();
    root
}

fn paths(dir: &Path) -> Paths {
    Paths {
        claude_code: dir.join("claude.json"),
        codex: dir.join("codex/config.toml"),
        hermes: dir.join("hermes/config.yaml"),
        claude_desktop: dir.join("Claude/claude_desktop_config.json"),
    }
}

/// Pretends to be `claude` / `codex`: records calls and writes what the real tool would.
struct Fake<'a> {
    paths: &'a Paths,
    calls: RefCell<Vec<Vec<String>>>,
}

impl Runner for Fake<'_> {
    fn run(&self, _program: &Path, args: &[String]) -> Result<()> {
        self.calls.borrow_mut().push(args.to_vec());
        if args.get(1).map(String::as_str) == Some("add") {
            let dash = args.iter().position(|a| a == "--").unwrap();
            let (cmd, rest) = (&args[dash + 1], &args[dash + 2..]);
            if args.iter().any(|a| a == "user") {
                let v = json!({"mcpServers": {"other": {"command": "x"}, "mother": {"command": cmd, "args": rest}}});
                fs::write(&self.paths.claude_code, v.to_string()).unwrap();
            } else {
                fs::create_dir_all(self.paths.codex.parent().unwrap()).unwrap();
                let quoted: Vec<String> = rest.iter().map(|a| format!("'{a}'")).collect();
                fs::write(
                    &self.paths.codex,
                    format!("model = \"o3\"\n\n[mcp_servers.mother]\ncommand = '{cmd}'\nargs = [{}]\n\n[other]\nx = 1\n", quoted.join(", ")),
                )
                .unwrap();
            }
        }
        Ok(())
    }
}

#[test]
fn reads_codex_toml() {
    let text = "model = \"o3\"\n[mcp_servers.pencil]\ncommand = \"p\"\n[mcp_servers.mother]\ncommand = \"C:/M/mother.exe\"\nargs = [\"mcp\", \"--config\", 'C:\\Users\\x\\codex.private.json']\nenabled = true\n[profiles.a]\ncommand = \"nope\"\n";
    let r = codex_entry(text).unwrap();
    assert_eq!(r.command, "C:/M/mother.exe");
    assert_eq!(
        r.config().unwrap(),
        PathBuf::from(r"C:\Users\x\codex.private.json")
    );
    assert!(codex_entry("[mcp_servers.pencil]\ncommand = \"p\"\n").is_none());
}

#[test]
fn reads_hermes_yaml() {
    let text = "model: x\nmcp_servers:\n  other:\n    command: y\n  mother:\n    command: \"C:/M/mother.exe\"\n    args:\n      - mcp\n      - --config\n      - C:\\Users\\x\\hermes.private.json\n    enabled: true\n  after:\n    command: z\nother_top: 1\n";
    let r = hermes_entry(text).unwrap();
    assert_eq!(r.command, "C:/M/mother.exe");
    assert_eq!(
        r.args,
        vec!["mcp", "--config", r"C:\Users\x\hermes.private.json"]
    );
    let inline = hermes_entry(
        "mcp_servers:\n  mother:\n    command: m\n    args: [mcp, --config, 'c.json']\n",
    )
    .unwrap();
    assert_eq!(inline.config().unwrap(), PathBuf::from("c.json"));
    assert!(hermes_entry("mcp_servers:\n  other:\n    command: y\n").is_none());
    assert!(hermes_entry("other:\n  mother:\n    command: y\n").is_none());
}

#[test]
fn claude_desktop_connects_breaks_and_gets_fixed() {
    let dir = temp("desktop");
    let root = workspace(&dir);
    let p = paths(&dir);
    fs::create_dir_all(p.claude_desktop.parent().unwrap()).unwrap();
    fs::write(
        &p.claude_desktop,
        r#"{"mcpServers":{"other":{"command":"o"}},"theme":"dark"}"#,
    )
    .unwrap();
    let exe = dir.join("mother.exe");
    fs::write(&exe, "").unwrap();
    assert_eq!(status(&p, AppId::ClaudeDesktop).state, State::NotConnected);

    let done = connect(&root, &p, AppId::ClaudeDesktop, &exe, &RealRunner).unwrap();
    assert!(done.contains("connected with new access"), "{done}");
    let saved: Value =
        serde_json::from_str(&fs::read_to_string(&p.claude_desktop).unwrap()).unwrap();
    assert_eq!(saved["theme"], "dark");
    assert_eq!(saved["mcpServers"]["other"]["command"], "o");
    assert_eq!(status(&p, AppId::ClaudeDesktop).state, State::Working);
    assert_eq!(fs::read_dir(root.join("setup-backups")).unwrap().count(), 1);

    // Connecting again reuses the same access instead of piling up grants.
    connect(&root, &p, AppId::ClaudeDesktop, &exe, &RealRunner).unwrap();
    assert!(!root.join("clients/claude-desktop-2.private.json").exists());

    // Revoked access shows as broken in plain words, and fix replaces it.
    Store::open(&root.join("context.sqlite"))
        .unwrap()
        .revoke("claude-desktop")
        .unwrap();
    let s = status(&p, AppId::ClaudeDesktop);
    assert!(
        matches!(&s.state, State::Broken(why) if why.contains("revoked")),
        "{s:?}"
    );
    let fixed = fix(&root, &p, &exe, &RealRunner);
    assert_eq!(fixed.len(), 1, "{fixed:?}");
    assert!(fixed[0].starts_with("Fixed Claude Desktop"), "{fixed:?}");
    assert!(root.join("clients/claude-desktop-2.private.json").is_file());
    assert_eq!(status(&p, AppId::ClaudeDesktop).state, State::Working);

    // A missing program is also explained.
    fs::remove_file(&exe).unwrap();
    assert!(
        matches!(status(&p, AppId::ClaudeDesktop).state, State::Broken(why) if why.contains("no longer exists"))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn command_line_apps_use_their_own_add_command() {
    let dir = temp("cli");
    let root = workspace(&dir);
    let p = paths(&dir);
    let exe = dir.join("mother.exe");
    fs::write(&exe, "").unwrap();
    let fake = Fake {
        paths: &p,
        calls: RefCell::new(Vec::new()),
    };

    // Only runs when the real programs exist on PATH; the fake does the writing.
    if find_program("claude").is_some() {
        connect(&root, &p, AppId::ClaudeCode, &exe, &fake).unwrap();
        let calls = fake.calls.borrow().clone();
        assert_eq!(
            calls.len(),
            1,
            "no remove when nothing was registered: {calls:?}"
        );
        assert_eq!(&calls[0][..5], ["mcp", "add", "-s", "user", "mother"]);
        assert_eq!(status(&p, AppId::ClaudeCode).state, State::Working);
        let kept: Value =
            serde_json::from_str(&fs::read_to_string(&p.claude_code).unwrap()).unwrap();
        assert_eq!(kept["mcpServers"]["other"]["command"], "x");
    }
    if find_program("codex").is_some() {
        fake.calls.borrow_mut().clear();
        fs::create_dir_all(p.codex.parent().unwrap()).unwrap();
        fs::write(&p.codex, "[mcp_servers.mother]\ncommand = 'C:/gone/mother.exe'\nargs = ['mcp', '--config', 'C:/gone.json']\n").unwrap();
        assert!(matches!(status(&p, AppId::Codex).state, State::Broken(_)));
        connect(&root, &p, AppId::Codex, &exe, &fake).unwrap();
        let calls = fake.calls.borrow().clone();
        assert_eq!(calls[0], vec!["mcp", "remove", "mother"]);
        assert_eq!(&calls[1][..3], ["mcp", "add", "mother"]);
        assert_eq!(status(&p, AppId::Codex).state, State::Working);
        assert!(fs::read_dir(root.join("setup-backups")).unwrap().count() >= 1);
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn nothing_installed_means_nothing_to_connect() {
    let dir = temp("none");
    let root = workspace(&dir);
    let p = paths(&dir);
    assert_eq!(status(&p, AppId::ClaudeDesktop).state, State::NotInstalled);
    let exe = dir.join("mother.exe");
    fs::write(&exe, "").unwrap();
    let e = connect(&root, &p, AppId::ClaudeDesktop, &exe, &RealRunner).unwrap_err();
    assert!(e.to_string().contains("isn't installed"), "{e}");
    let _ = fs::remove_dir_all(&dir);
}
