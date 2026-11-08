//! Owner administration and native MCP launcher.

use clap::{Parser, Subcommand, ValueEnum};
use mother::{
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
    io::{Read, Write},
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
    /// Serve the workspace's six MCP tools over standard input/output.
    Mcp {
        #[arg(long)]
        config: PathBuf,
    },
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
    Json,
    Codex,
}

/// Parse the owner CLI and dispatch the native stdio service when requested.
pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Mcp { config }) => crate::mcp::serve(&config).await,
        Some(command) => execute(command),
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
fn execute(command: Command) -> Result<()> {
    match command {
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
            store.grant_to_file(&client, &scopes, &out, &db)?;
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
        Command::Connection { config, format } => {
            let config_path = absolute(&config)?;
            let config = load_config(&config_path)?;
            Store::open(&config.database_path)?.authorize(&config.auth(), "read")?;
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
