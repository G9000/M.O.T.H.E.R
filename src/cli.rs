//! Owner administration and native MCP launcher.

use clap::{Parser, Subcommand, ValueEnum};
use mother::{
    access::{AccessMode, ClientAccess},
    config::{create_private_file, data_directory, load_config, read_json},
    contracts::SCOPES,
    embedding,
    error::{Error, Result},
    limits::MAX_EXPORT_BYTES,
    search::Search,
    store::Store,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::{IsTerminal, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(
    name = "mother",
    version,
    about = "M.O.T.H.E.R — your independent personal workspace",
    after_help = "MOTHER_HOME selects an absolute private data directory. Grants default to read-only. Exports omit grants. Each harness needs its own grant/config."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}
#[derive(Debug, Subcommand)]
enum Command {
    /// Open the interactive workspace and connection dashboard.
    Tui,
    /// Open the owner review inbox for proposed memories.
    Inbox,
    /// Connect a saved harness, or guide first-time setup in a terminal.
    Connect {
        #[arg(value_enum)]
        harness: Harness,
        #[arg(long)]
        setup: bool,
    },
    /// Inspect connection readiness without modifying settings or memories.
    Status {
        #[arg(long)]
        json: bool,
    },
    /// Test this executable's local MCP read with an existing private grant.
    Doctor {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Stop this workspace's managed harness connection.
    Disconnect {
        #[arg(value_enum)]
        harness: Harness,
    },
    /// Opt in or out of starting the connection at Windows sign-in.
    Startup {
        #[arg(value_enum)]
        action: StartupAction,
    },
    /// Initialize a fresh workspace from your own SOUL.md file.
    Init {
        #[arg(long)]
        db: Option<PathBuf>,
        /// Your persona file; defaults to SOUL.md beside the selected database.
        #[arg(long)]
        soul: Option<PathBuf>,
    },
    /// Inspect or explicitly update this workspace's stored persona as its owner.
    Soul {
        #[command(subcommand)]
        command: SoulCommand,
    },
    /// Create a client grant and exclusive private configuration file.
    Grant {
        client: String,
        /// Context policy; omission preserves legacy access.
        #[arg(long, value_parser = ["companion", "shared"])]
        access: Option<String>,
        /// External agent identity bound by the owner to this credential.
        #[arg(long, requires = "access")]
        subject: Option<String>,
        /// Canonical record IDs visible to a shared client (empty means none).
        #[arg(long, value_delimiter = ',', requires = "access")]
        records: Vec<String>,
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long, default_value = "read", value_delimiter = ',', action = clap::ArgAction::Set)]
        scopes: Vec<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Revoke a client, including its already connected MCP processes.
    Revoke {
        client: String,
        #[arg(long)]
        db: Option<PathBuf>,
    },
    /// Inspect bounded context using a client grant.
    Context {
        #[arg(long)]
        config: PathBuf,
    },
    /// Search bounded record previews with local semantic or literal retrieval.
    Search {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "hybrid", value_parser = ["hybrid", "semantic", "literal"])]
        mode: String,
        #[arg(long)]
        kind: Option<String>,
        #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u16).range(1..=100))]
        limit: u16,
    },
    /// Prepare a bounded batch of local derived embeddings using a client grant.
    Index {
        #[arg(long)]
        config: PathBuf,
        #[arg(long, default_value_t = 32, value_parser = clap::value_parser!(u16).range(1..=100))]
        max_records: u16,
    },
    /// Install or inspect the pinned local embedding model.
    Model {
        #[command(subcommand)]
        command: ModelCommand,
    },
    /// Start a harness on an open task through a launcher you declared.
    ///
    /// Launchers live in hearth/launchers.json in the private data folder. The
    /// task is handed to the launcher's harness, the run is recorded as an
    /// action, and its output goes to a private log.
    Launch {
        /// The task to work on.
        #[arg(required_unless_present = "list")]
        task: Option<String>,
        /// The launcher to use.
        #[arg(long, required_unless_present = "list")]
        launcher: Option<String>,
        /// List declared launchers and whether each is ready.
        #[arg(long, conflicts_with_all = ["task", "launcher", "retry_unknown"])]
        list: bool,
        /// Launch again although an earlier launch of this task is pending or unknown.
        #[arg(long)]
        retry_unknown: bool,
        #[arg(long)]
        db: Option<PathBuf>,
    },
    /// Print a short briefing for a harness to load at session start.
    ///
    /// Always exits successfully: if the workspace cannot be read, it prints a
    /// short note saying so instead of private content.
    Brief {
        #[arg(long)]
        config: PathBuf,
    },
    /// Print token-free harness connection configuration.
    Connection {
        #[arg(long)]
        config: PathBuf,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
    /// Export portable state without client credentials.
    Export {
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long)]
        out: PathBuf,
    },
    /// Restore portable state to a fresh database path.
    Restore {
        #[arg(long)]
        from: PathBuf,
        #[arg(long)]
        db: PathBuf,
    },
    /// Serve the workspace's MCP tools over standard input/output.
    Mcp {
        #[arg(long)]
        config: PathBuf,
    },
}
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Harness {
    Chatgpt,
    Animaos,
}
#[derive(Debug, Clone, Copy, ValueEnum)]
enum StartupAction {
    Enable,
    Disable,
    Status,
}

#[derive(Debug, Subcommand)]
enum SoulCommand {
    /// Read the stored persona and its hash; output may contain private text.
    Show {
        #[arg(long)]
        db: Option<PathBuf>,
    },
    /// Import your edited persona only if the current stored hash still matches.
    Update {
        #[arg(long)]
        db: Option<PathBuf>,
        #[arg(long)]
        file: PathBuf,
        /// Current persona hash from soul show; stale hashes fail without changing state.
        #[arg(long)]
        expected_hash: String,
    },
}

#[derive(Debug, Subcommand)]
enum ModelCommand {
    /// Download and verify the pinned model files; no record text leaves this device.
    Install,
    /// Inspect installed model files without downloading anything.
    Status,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    /// The command and arguments, as JSON.
    Json,
    /// A Codex config.toml entry.
    Codex,
    /// A `claude mcp add` command and a SessionStart hook that runs `mother brief`.
    ClaudeCode,
    /// An entry for `mcp_servers` in ~/.hermes/config.yaml.
    Hermes,
    /// An `openclaw mcp set` command.
    Openclaw,
    /// An `mcpServers` JSON block, as used by Claude Desktop and many other clients.
    McpServers,
}

/// Parse the owner CLI and dispatch the native stdio service when requested.
pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Mcp { config }) => crate::mcp::serve(&config).await,
        Some(command) => execute(command),
        None if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() => {
            crate::tui::run(crate::tui::Launch::Home)
        }
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
    }
}
fn absolute(path: &Path) -> Result<PathBuf> {
    Ok(std::path::absolute(path)?)
}
fn db_path(path: Option<PathBuf>) -> Result<PathBuf> {
    path.map_or_else(
        || Ok(data_directory()?.join("context.sqlite")),
        |p| absolute(&p),
    )
}
fn print_json(value: Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
fn private_parent(path: &Path) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::domain("INVALID_INPUT", "The file path needs a parent directory."))?;
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(parent)?;
    Ok(())
}
fn valid_client(client: &str) -> Result<()> {
    if client.is_empty()
        || client.len() > 80
        || !client.as_bytes()[0].is_ascii_alphanumeric()
        || !client
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
    {
        return Err(Error::domain("INVALID_INPUT", "Invalid client name."));
    }
    Ok(())
}
fn exclusive_json(path: &Path, value: Value) -> Result<()> {
    let mut text = serde_json::to_vec(&value)?;
    text.push(b'\n');
    if text.len() > MAX_EXPORT_BYTES {
        return Err(Error::domain(
            "RESOURCE_LIMIT",
            "Export exceeds the 64 MiB portable file limit.",
        ));
    }
    let mut file = create_private_file(path)?;
    let result = file.write_all(&text).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = result {
        let _ = fs::remove_file(path);
        return Err(error.into());
    }
    Ok(())
}
// A 32,000 UTF-16-unit persona needs at most 128,000 UTF-8 bytes.
const MAX_SOUL_BYTES: u64 = 128_000;
fn read_soul(path: &Path) -> Result<String> {
    let file = fs::File::open(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            Error::domain(
                "SOUL_REQUIRED",
                "SOUL.md was not found. Use an existing file with init --soul <path> or soul update --file <path>; init also accepts SOUL.md beside the database.",
            )
        } else {
            error.into()
        }
    })?;
    let mut content = String::new();
    file.take(MAX_SOUL_BYTES + 1).read_to_string(&mut content)?;
    if content.len() as u64 > MAX_SOUL_BYTES
        || content.trim_start_matches('\u{feff}').trim().is_empty()
        || content.encode_utf16().count() > 32_000
    {
        return Err(Error::domain(
            "INVALID_INPUT",
            "SOUL.md must contain nonblank UTF-8 text of at most 32,000 UTF-16 units.",
        ));
    }
    Ok(content)
}
/// A resolved path for harness configuration, without Windows' verbatim `\\?\` prefix.
fn stable_path(path: &Path) -> Result<String> {
    let resolved = fs::canonicalize(path)?;
    let text = resolved
        .to_str()
        .ok_or_else(|| Error::domain("INVALID_INPUT", "The path must be valid Unicode."))?;
    Ok(match text.strip_prefix(r"\\?\") {
        Some(rest) if rest.as_bytes().get(1) == Some(&b':') => rest.to_owned(),
        _ => text.to_owned(),
    })
}
/// Refuse paths that would change meaning inside a pasted shell command.
fn shell_safe(path: &str) -> Result<&str> {
    if path
        .chars()
        .any(|c| c.is_control() || matches!(c, '"' | '\'' | '$' | '`' | '%'))
    {
        return Err(Error::domain(
            "INVALID_INPUT",
            "This path contains characters a shell would misread (quotes, $, `, % or control characters). Move the executable or client config to a plain path and try again.",
        ));
    }
    Ok(path)
}
const STANDING_INSTRUCTION: &str = "At the start of each session, call mother_get_context and treat its pinned records as the owner's core context.";
fn print_harness_setup(format: Format, config_path: &Path) -> Result<()> {
    let exe = stable_path(&std::env::current_exe()?)?;
    let config = stable_path(config_path)?;
    let args = json!(["mcp", "--config", config]);
    match format {
        Format::ClaudeCode => {
            let (exe, config) = (shell_safe(&exe)?, shell_safe(&config)?);
            let hook = json!({"hooks":{"SessionStart":[{"hooks":[{"type":"command","command":format!("\"{exe}\" brief --config \"{config}\""),"timeout":30}]}]}});
            println!(
                "# 1. Register M.O.T.H.E.R for every project (run once in a terminal):\nclaude mcp add --transport stdio --scope user mother -- \"{exe}\" mcp --config \"{config}\"\n\n# 2. Have her brief every session before the first message. Merge this into\n#    the \"hooks\" section of ~/.claude/settings.json:\n{}\n\n# Then start a new session and check with /mcp that \"mother\" is connected.",
                serde_json::to_string_pretty(&hook)?
            );
        }
        Format::Hermes => {
            println!(
                "# Add under mcp_servers in ~/.hermes/config.yaml, then run /reload-mcp in a session.\n# Standing instruction for Hermes: {STANDING_INSTRUCTION}\nmcp_servers:\n  mother:\n    command: {}\n    args: {}",
                serde_json::to_string(&exe)?,
                serde_json::to_string(&args)?
            );
        }
        Format::Openclaw => {
            shell_safe(&exe)?;
            shell_safe(&config)?;
            println!(
                "# With the OpenClaw Gateway stopped, run:\nopenclaw mcp set mother '{}'\n# Then start the Gateway and check with: openclaw mcp doctor --probe\n# Standing instruction for OpenClaw: {STANDING_INSTRUCTION}",
                serde_json::to_string(&json!({"command":exe,"args":args}))?
            );
        }
        Format::McpServers => {
            print_json(json!({"mcpServers":{"mother":{"command":exe,"args":args}}}))?
        }
        Format::Json | Format::Codex => unreachable!("handled by the connection command"),
    }
    Ok(())
}
fn execute(command: Command) -> Result<()> {
    match command {
        Command::Doctor { config, json } => {
            let report = crate::health::probe(&config);
            if json {
                print_json(serde_json::to_value(&report)?)?;
            } else {
                println!(
                    "{}\n{}",
                    report.scope,
                    serde_json::to_string_pretty(&report)?
                );
            }
            if report.status == "verified" {
                Ok(())
            } else {
                Err(Error::domain(
                    "HEALTH_CHECK",
                    "Local MCP check incomplete; inspect the redacted report.",
                ))
            }
        }
        Command::Inbox => crate::tui::run(crate::tui::Launch::Inbox),
        Command::Tui => crate::tui::run(crate::tui::Launch::Home),
        Command::Connect {
            harness: Harness::Animaos,
            ..
        } => crate::tui::run(crate::tui::Launch::Animaos),
        Command::Disconnect {
            harness: Harness::Animaos,
        } => Err(Error::domain(
            "CONNECTION",
            "Stop the AnimaOS connection in its Connectors > MCP screen.",
        )),
        Command::Connect {
            harness: Harness::Chatgpt,
            setup,
        } => {
            let interactive = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
            if setup || crate::connection::load_settings()?.is_none() {
                if !interactive {
                    return Err(Error::domain(
                        "SETUP_REQUIRED",
                        "Run mother connect chatgpt --setup in an interactive terminal.",
                    ));
                }
                return crate::tui::run(crate::tui::Launch::Setup);
            }
            let status = crate::connection::connect()?;
            println!("{}: {}", status.state, status.detail);
            if status.state == crate::connection::ConnectionState::Ready {
                Ok(())
            } else {
                Err(Error::domain(
                    "NOT_READY",
                    "Connection is not ready; run mother status before retrying.",
                ))
            }
        }
        Command::Status { json: machine } => {
            let status = crate::connection::status()?;
            if machine {
                print_json(serde_json::to_value(status)?)
            } else {
                println!("ChatGPT: {}\n{}", status.state, status.detail);
                Ok(())
            }
        }
        Command::Disconnect {
            harness: Harness::Chatgpt,
        } => {
            let status = crate::connection::disconnect()?;
            println!("{}: {}", status.state, status.detail);
            if matches!(
                status.state,
                crate::connection::ConnectionState::Stopped
                    | crate::connection::ConnectionState::Unconfigured
            ) {
                Ok(())
            } else {
                Err(Error::domain(
                    "NOT_STOPPED",
                    "The connection was not verified stopped; inspect mother status before another action.",
                ))
            }
        }
        Command::Startup { action } => {
            use crate::connection::Startup;
            match action {
                StartupAction::Enable => crate::connection::set_startup(Startup::Enabled)?,
                StartupAction::Disable => crate::connection::set_startup(Startup::Disabled)?,
                StartupAction::Status => {}
            }
            print_json(json!({"startupEnabled":crate::connection::startup_enabled()?}))
        }
        Command::Init { db, soul } => {
            let db = db_path(db)?;
            let soul = soul.unwrap_or_else(|| db.with_file_name("SOUL.md"));
            let persona = read_soul(&soul)?;
            private_parent(&db)?;
            let store = Store::initialize(&db, &persona)?;
            print_json(json!({"databasePath":db, "workspace":store.workspace()?}))
        }
        Command::Soul { command } => match command {
            SoulCommand::Show { db } => {
                let store = Store::open(&db_path(db)?)?;
                print_json(json!({"workspace":store.workspace()?,"persona":store.persona()?}))
            }
            SoulCommand::Update {
                db,
                file,
                expected_hash,
            } => {
                let content = read_soul(&file)?;
                let mut store = Store::open(&db_path(db)?)?;
                print_json(store.update_persona(&content, &expected_hash)?)
            }
        },
        Command::Grant {
            client,
            access,
            subject,
            records,
            db,
            scopes,
            out,
        } => {
            valid_client(&client)?;
            if scopes.is_empty() || scopes.iter().any(|s| !SCOPES.contains(&s.as_str())) {
                return Err(Error::domain("INVALID_INPUT", "Invalid client scopes."));
            }
            if !scopes.iter().any(|s| s == "read") {
                return Err(Error::domain(
                    "INVALID_INPUT",
                    "MCP clients require the read scope.",
                ));
            }
            let db = db_path(db)?;
            let out = match out {
                Some(path) => absolute(&path)?,
                None => db
                    .with_file_name("clients")
                    .join(format!("{client}.private.json")),
            };
            let mut store = Store::open(&db)?;
            private_parent(&out)?;
            let policy = ClientAccess {
                mode: match access.as_deref() {
                    Some("companion") => AccessMode::Companion,
                    Some("shared") => AccessMode::Shared,
                    _ => AccessMode::Legacy,
                },
                subject,
                records,
            };
            store.grant_to_file_with_access(&client, &scopes, &out, &db, &policy)?;
            print_json(json!({"clientId":client, "scopes":scopes, "configPath":out}))
        }
        Command::Revoke { client, db } => {
            valid_client(&client)?;
            Store::open(&db_path(db)?)?.revoke(&client)?;
            print_json(json!({"clientId":client, "revoked":true}))
        }
        Command::Context { config } => {
            let config = load_config(&absolute(&config)?)?;
            let mut store = Store::open(&config.database_path)?;
            store.authorize(&config.auth(), "read")?;
            print_json(store.context(&config.auth(), json!({}))?)
        }
        Command::Search {
            config,
            query,
            mode,
            kind,
            limit,
        } => {
            let config = load_config(&absolute(&config)?)?;
            let mut store = Store::open(&config.database_path)?;
            let mut args = json!({"query":query, "mode":mode, "limit":limit});
            if let Some(kind) = kind {
                args["kind"] = kind.into();
            }
            let search = Search::from_environment();
            print_json(search.search(&mut store, &config.auth(), args)?)
        }
        Command::Index {
            config,
            max_records,
        } => {
            let config = load_config(&absolute(&config)?)?;
            let mut store = Store::open(&config.database_path)?;
            let search = Search::from_environment();
            print_json(search.index(&mut store, &config.auth(), usize::from(max_records))?)
        }
        Command::Model { command } => {
            let dir = embedding::model_directory()?;
            print_json(match command {
                ModelCommand::Install => embedding::install(&dir)?,
                ModelCommand::Status => embedding::status(&dir)?,
            })
        }
        Command::Launch {
            task,
            launcher,
            list,
            retry_unknown,
            db,
        } => {
            use mother::launch;
            let path = launch::launchers_path()?;
            let launchers = launch::load(&path)?;
            if list {
                let rows: Vec<Value> = launchers
                    .iter()
                    .map(|(name, l)| {
                        let problem = l.problem();
                        json!({"name":name,"harness":l.harness,"command":l.command,"ready":problem.is_none(),"problem":problem})
                    })
                    .collect();
                return print_json(json!({"launchersFile":path,"launchers":rows}));
            }
            let (Some(task), Some(name)) = (task, launcher) else {
                unreachable!("clap requires a task and a launcher without --list")
            };
            let chosen = launchers.get(&name).ok_or_else(|| {
                Error::domain(
                    "INVALID_INPUT",
                    format!("No launcher named {name} in {}", path.display()),
                )
            })?;
            if let Some(problem) = chosen.problem() {
                return Err(Error::domain(
                    "INVALID_INPUT",
                    format!("Launcher {name} cannot run: {problem}"),
                ));
            }
            let mut store = Store::open(&db_path(db)?)?;
            let action = store.begin_launch(&task, &name, chosen, retry_unknown)?;
            let action_id = action["id"].as_str().unwrap_or_default().to_owned();
            let log = launch::log_path(&action_id)?;
            let outcome = launch::run(chosen, &task, &log)?;
            store.finish_launch(&action, &outcome, &log)?;
            print_json(
                json!({"action":action_id,"task":task,"launcher":name,"harness":chosen.harness,"status":outcome.status,"exitCode":outcome.exit_code,"seconds":outcome.seconds,"log":log}),
            )?;
            if outcome.status == "succeeded" {
                Ok(())
            } else {
                Err(Error::domain(
                    "LAUNCH",
                    format!(
                        "The launch {}; see its log and action record.",
                        if outcome.status == "unknown" {
                            "hit its timeout and its outcome is unknown"
                        } else {
                            "failed"
                        }
                    ),
                ))
            }
        }
        Command::Brief { config } => {
            let brief = (|| -> Result<String> {
                let config = load_config(&absolute(&config)?)?;
                let mut store = Store::open(&config.database_path)?;
                mother::brief::compose(&mut store, &config.auth())
            })();
            match brief {
                Ok(text) => print!("{text}"),
                Err(error) => print!("{}", mother::brief::unavailable(error.code())),
            }
            Ok(())
        }
        Command::Connection { config, format } => {
            let config_path = absolute(&config)?;
            let config = load_config(&config_path)?;
            Store::open(&config.database_path)?.authorize(&config.auth(), "read")?;
            if !matches!(format, Format::Json | Format::Codex) {
                return print_harness_setup(format, &config_path);
            }
            let command = std::env::current_exe()?;
            let args = json!(["mcp", "--config", config_path]);
            match format {
                Format::Json => print_json(json!({"command":command, "args":args})),
                Format::Codex => {
                    println!(
                        "[mcp_servers.mother]\ncommand = {}\nargs = {}",
                        serde_json::to_string(&command)?,
                        args
                    );
                    Ok(())
                }
                _ => unreachable!("harness setup formats are handled above"),
            }
        }
        Command::Export { db, out } => {
            let out = absolute(&out)?;
            let bundle = Store::open(&db_path(db)?)?.export()?;
            exclusive_json(&out, bundle)?;
            print_json(json!({"exportedTo":out, "credentialsIncluded":false}))
        }
        Command::Restore { from, db } => {
            let db = absolute(&db)?;
            let bundle = read_json(&absolute(&from)?, MAX_EXPORT_BYTES)?;
            let store = Store::restore(&db, bundle)?;
            print_json(
                json!({"databasePath":db, "workspace":store.workspace()?, "next":"Create new client grants for this restored database."}),
            )
        }
        Command::Mcp { .. } => unreachable!("MCP is dispatched before synchronous owner commands"),
    }
}
