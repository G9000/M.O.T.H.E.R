//! Owner administration and native MCP launcher.

use clap::{Parser, Subcommand, ValueEnum};
use mother::{
    config::{create_private_file, data_directory, load_config, read_json},
    contracts::SCOPES,
    error::{Error, Result},
    limits::MAX_EXPORT_BYTES,
    store::Store,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
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
    /// Initialize a fresh workspace with the current SOUL.md or bundled persona.
    Init {
        #[arg(long)]
        db: Option<PathBuf>,
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
fn execute(command: Command) -> Result<()> {
    match command {
        Command::Init { db } => {
            let db = db_path(db)?;
            private_parent(&db)?;
            let persona = match fs::read_to_string("SOUL.md") {
                Ok(text) => text,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    mother::DEFAULT_PERSONA.to_owned()
                }
                Err(error) => return Err(error.into()),
            };
            let store = Store::initialize(&db, &persona)?;
            print_json(json!({"databasePath":db, "workspace":store.workspace()?}))
        }
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
