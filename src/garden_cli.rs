//! Native Garden owner commands.
use clap::Subcommand;
use mother::{
    access::{AccessMode, ClientAccess},
    config::{create_private_file, read_json},
    error::{Error, Result},
    limits::MAX_EXPORT_BYTES,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::{IsTerminal, Read, Write},
    path::{Path, PathBuf},
};
const MAX_SOUL_BYTES: u64 = 128_000;
/// Build the Garden subtree only when selected, keeping the parent CLI's debug
/// builder frame off the Windows main thread's bounded stack.
#[derive(Debug)]
pub(crate) struct DeferredGardenCommand(pub GardenCommand);
impl clap::FromArgMatches for DeferredGardenCommand {
    fn from_arg_matches(matches: &clap::ArgMatches) -> std::result::Result<Self, clap::Error> {
        GardenCommand::from_arg_matches(matches).map(Self)
    }
    fn update_from_arg_matches(
        &mut self,
        matches: &clap::ArgMatches,
    ) -> std::result::Result<(), clap::Error> {
        self.0.update_from_arg_matches(matches)
    }
}
impl clap::Subcommand for DeferredGardenCommand {
    fn augment_subcommands(command: clap::Command) -> clap::Command {
        command.defer(GardenCommand::augment_subcommands)
    }
    fn augment_subcommands_for_update(command: clap::Command) -> clap::Command {
        command.defer(GardenCommand::augment_subcommands_for_update)
    }
    fn has_subcommand(name: &str) -> bool {
        GardenCommand::has_subcommand(name)
    }
}
#[derive(Debug, Subcommand)]
pub(crate) enum GardenCommand {
    /// Apply an explicitly reviewed owner request; this operation is never exposed through MCP.
    Review {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        graph: bool,
    },
    /// Start the sole local owner for a native Garden database.
    Service {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        endpoint: PathBuf,
    },
    /// Bind an existing native credential to the running local owner.
    ServiceConfig {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        endpoint: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Inspect or execute an explicit journaled cutover as the owner.
    Migrate {
        #[command(subcommand)]
        command: crate::cutover_cli::MigrationCommand,
    },
    /// Enable the native v1 compatibility view with an explicit owner persona.
    CompatInit {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        soul: PathBuf,
    },
    /// Provision native v1 scopes and an optional independent graph policy.
    GrantV1 {
        client: String,
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value = "read", value_delimiter = ',', action = clap::ArgAction::Set)]
        scopes: Vec<String>,
        #[arg(long, value_parser = ["companion", "shared"])]
        access: Option<String>,
        #[arg(long, requires = "access")]
        subject: Option<String>,
        #[arg(long, value_delimiter = ',', requires = "access")]
        records: Vec<String>,
        /// Explicit JSON array of typed graph references, independent of v1 record IDs.
        #[arg(long, conflicts_with = "all")]
        objects: Option<PathBuf>,
        /// Explicit full graph visibility, requiring --access companion.
        #[arg(long)]
        all: bool,
    },
    /// Stream a credential-free full snapshot to a new directory.
    Backup {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Verify every chunk and canonical relationship without publishing a store.
    Verify {
        #[arg(long)]
        backup: PathBuf,
    },
    /// Restore a full snapshot to a nonexistent file with fresh generation and no grants.
    Restore {
        #[arg(long)]
        backup: PathBuf,
        #[arg(long)]
        db: PathBuf,
    },
    /// Ingest owner-selected artifact bytes into this database by content address.
    Blob {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        file: PathBuf,
    },
    /// Export the small v1 format, refusing state that requires a full backup.
    ExportV1 {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Import a small v1 bundle into an empty view with the same explicit identity.
    ImportV1 {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        file: PathBuf,
    },
    /// Review native v1 proposals using the owner inbox.
    V1Inbox {
        #[arg(long)]
        db: PathBuf,
    },
    /// Prepare optional local vectors for selected authorized graph objects.
    Index {
        #[arg(long)]
        config: PathBuf,
        /// Bounded JSON array of typed {kind,id} source references.
        #[arg(long)]
        objects: PathBuf,
    },
    /// Initialize a fresh graph file with a new workspace identity.
    Init {
        #[arg(long)]
        db: PathBuf,
    },
    /// Create graph-only credentials in an exclusive private file.
    Grant {
        client: String,
        #[arg(long)]
        db: PathBuf,
        #[arg(long, default_value = "garden:read,garden:propose", value_delimiter = ',', action = clap::ArgAction::Set)]
        scopes: Vec<String>,
        #[arg(long, value_parser = ["companion", "shared"])]
        access: String,
        #[arg(long)]
        subject: String,
        /// JSON array of typed {kind,id} references; omitted means no objects.
        #[arg(long, conflicts_with = "all")]
        objects: Option<PathBuf>,
        /// Explicit full graph visibility, available to companion clients only.
        #[arg(long)]
        all: bool,
        #[arg(long)]
        out: PathBuf,
    },
    /// Revoke a graph credential without changing v1 grants.
    Revoke {
        client: String,
        #[arg(long)]
        db: PathBuf,
    },
    /// Review graph proposals locally as the owner.
    Inbox {
        #[arg(long)]
        db: PathBuf,
    },
    /// Open the read-only memory garden: a local web view of one snapshot of the v1 view.
    Explore {
        #[arg(long)]
        db: PathBuf,
        /// Most records to show; pinned records first, then the most recently updated.
        #[arg(long, default_value_t = 500, value_parser = clap::value_parser!(u16).range(1..=2000))]
        limit: u16,
        /// Also open the one-time link in the default browser.
        #[arg(long)]
        open: bool,
    },
    /// Serve graph MCP tools using graph-only private credentials.
    Mcp {
        #[arg(long)]
        config: PathBuf,
    },
}

fn absolute(path: &Path) -> Result<PathBuf> {
    Ok(std::path::absolute(path)?)
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
fn run_v1_inbox(path: PathBuf) -> Result<()> {
    if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
        return crate::tui::run_native_inbox(path);
    }
    print_json(mother::garden::store::GardenStore::open_active(path)?.v1_inbox_list()?)
}
fn run_graph_inbox(path: PathBuf) -> Result<()> {
    if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
        return crate::tui::run_garden(path);
    }
    print_json(
        mother::garden::store::GardenStore::open_active(path)?
            .owner_inbox_list()
            .map_err(mother::garden::config::domain_error)?,
    )
}
pub(crate) fn execute_garden(command: GardenCommand) -> Result<()> {
    use mother::garden::{
        access::GardenAccess, config::domain_error, contracts::ExternalReference,
        store::GardenStore,
    };
    let bound_database = match &command {
        GardenCommand::Review { db, .. }
        | GardenCommand::CompatInit { db, .. }
        | GardenCommand::GrantV1 { db, .. }
        | GardenCommand::Grant { db, .. }
        | GardenCommand::Revoke { db, .. }
        | GardenCommand::Backup { db, .. }
        | GardenCommand::Blob { db, .. }
        | GardenCommand::ExportV1 { db, .. }
        | GardenCommand::ImportV1 { db, .. }
        | GardenCommand::Inbox { db, .. }
        | GardenCommand::V1Inbox { db, .. }
        | GardenCommand::Explore { db, .. }
        | GardenCommand::Service { db, .. } => Some(db),
        _ => None,
    };
    if let Some(path) = bound_database {
        mother::garden::migrate::check_destination(&absolute(path)?)?;
    }
    match command {
        GardenCommand::Review { db, request, graph } => {
            let store = GardenStore::open_active(absolute(&db)?)?;
            let request = read_json(&absolute(&request)?, 2_097_152)?;
            if graph {
                print_json(serde_json::to_value(
                    store.review_proposal(request).map_err(domain_error)?,
                )?)
            } else {
                print_json(store.v1_review_proposal(request)?)
            }
        }
        GardenCommand::Service { db, endpoint } => crate::cutover_cli::service(db, endpoint),
        GardenCommand::ServiceConfig {
            config,
            endpoint,
            out,
        } => crate::cutover_cli::service_config(config, endpoint, out),
        GardenCommand::Migrate { command } => crate::cutover_cli::migration(command),
        GardenCommand::CompatInit { db, soul } => {
            let store = GardenStore::open_active(absolute(&db)?)?;
            print_json(store.v1_initialize(&read_soul(&absolute(&soul)?)?)?)
        }
        GardenCommand::GrantV1 {
            client,
            db,
            out,
            scopes,
            access,
            subject,
            records,
            objects,
            all,
        } => {
            valid_client(&client)?;
            if all && access.as_deref() != Some("companion") {
                return Err(Error::domain(
                    "FORBIDDEN",
                    "Full graph visibility requires explicit companion access.",
                ));
            }
            let policy = ClientAccess {
                mode: match access.as_deref() {
                    Some("companion") => AccessMode::Companion,
                    Some("shared") => AccessMode::Shared,
                    _ => AccessMode::Legacy,
                },
                subject,
                records,
            };
            let garden = if all {
                Some(GardenAccess::All {})
            } else {
                objects
                    .map(|path| -> Result<GardenAccess> {
                        let objects: Vec<ExternalReference> =
                            serde_json::from_value(read_json(&absolute(&path)?, 1_048_576)?)?;
                        Ok(GardenAccess::Allowlist { objects })
                    })
                    .transpose()?
            };
            let db = absolute(&db)?;
            let out = absolute(&out)?;
            let store = GardenStore::open_active(&db)?;
            private_parent(&out)?;
            store.v1_grant_to_file_with_access(
                &client,
                &scopes,
                &out,
                &db,
                &policy,
                garden.as_ref(),
            )?;
            print_json(json!({"clientId":client,"scopes":scopes,"configPath":out}))
        }
        GardenCommand::Backup { db, out } => {
            let store = GardenStore::open_active(absolute(&db)?)?;
            let out = absolute(&out)?;
            private_parent(&out)?;
            print_json(serde_json::to_value(
                store.backup_to(&out).map_err(domain_error)?,
            )?)
        }
        GardenCommand::Verify { backup } => print_json(serde_json::to_value(
            GardenStore::verify_backup(&absolute(&backup)?).map_err(domain_error)?,
        )?),
        GardenCommand::Restore { backup, db } => {
            let db = absolute(&db)?;
            private_parent(&db)?;
            let store =
                GardenStore::restore_backup(&absolute(&backup)?, &db).map_err(domain_error)?;
            let identity = store.owner_inbox_list().map_err(domain_error)?;
            print_json(json!({"databasePath":db,"workspace":identity["workspace"]}))
        }
        GardenCommand::Blob { db, file } => {
            let store = GardenStore::open_active(absolute(&db)?)?;
            print_json(serde_json::to_value(
                store
                    .put_blob_file(&absolute(&file)?)
                    .map_err(domain_error)?,
            )?)
        }
        GardenCommand::ExportV1 { db, out } => {
            let store = GardenStore::open_active(absolute(&db)?)?;
            let bundle = store.v1_export()?;
            let out = absolute(&out)?;
            private_parent(&out)?;
            exclusive_json(&out, bundle)?;
            print_json(json!({"path":out,"formatVersion":1}))
        }
        GardenCommand::ImportV1 { db, file } => {
            let bundle = read_json(&absolute(&file)?, MAX_EXPORT_BYTES)?;
            let store = GardenStore::open_active(absolute(&db)?)?;
            store.import_v1_bundle(bundle)?;
            print_json(json!({"imported":true,"formatVersion":1}))
        }
        GardenCommand::V1Inbox { db } => run_v1_inbox(absolute(&db)?),
        GardenCommand::Init { db } => {
            let db = absolute(&db)?;
            private_parent(&db)?;
            let workspace = uuid::Uuid::new_v4().to_string();
            let generation = uuid::Uuid::new_v4().to_string();
            let store =
                GardenStore::initialize(&db, &workspace, &generation).map_err(domain_error)?;
            let identity = store.owner_inbox_list().map_err(domain_error)?;
            print_json(json!({"databasePath":db,"workspace":identity["workspace"]}))
        }
        GardenCommand::Grant {
            client,
            db,
            scopes,
            access,
            subject,
            objects,
            all,
            out,
        } => {
            valid_client(&client)?;
            let db = absolute(&db)?;
            let out = absolute(&out)?;
            let policy = ClientAccess {
                mode: if access == "companion" {
                    AccessMode::Companion
                } else {
                    AccessMode::Shared
                },
                subject: Some(subject),
                records: vec![],
            };
            let garden = if all {
                GardenAccess::All {}
            } else {
                let objects: Vec<ExternalReference> = match objects {
                    Some(path) => {
                        let file = fs::File::open(path)?;
                        let mut bytes = Vec::new();
                        file.take(1_048_577).read_to_end(&mut bytes)?;
                        if bytes.len() > 1_048_576 {
                            return Err(Error::domain(
                                "LIMIT_EXCEEDED",
                                "Graph allowlist file is too large.",
                            ));
                        }
                        serde_json::from_slice(&bytes)?
                    }
                    None => vec![],
                };
                GardenAccess::Allowlist { objects }
            };
            let store = GardenStore::open_active(&db)?;
            private_parent(&out)?;
            store
                .grant_to_file_with_access(&client, &scopes, &out, &db, &policy, &garden)
                .map_err(domain_error)?;
            print_json(json!({"clientId":client,"scopes":scopes,"configPath":out}))
        }
        GardenCommand::Revoke { client, db } => {
            valid_client(&client)?;
            GardenStore::open_active(absolute(&db)?)?
                .revoke(&client)
                .map_err(domain_error)?;
            print_json(json!({"clientId":client,"revoked":true}))
        }
        GardenCommand::Index { config, objects } => {
            let config = mother::garden::config::load_garden_config(&absolute(&config)?)?;
            let references: Vec<ExternalReference> =
                serde_json::from_value(read_json(&absolute(&objects)?, 16_384)?)?;
            let store = GardenStore::open_active(&config.database_path)?;
            config.authorize(&store)?;
            let report = store
                .prepare_search(&config.auth(), &references)
                .map_err(domain_error)?;
            print_json(serde_json::to_value(report)?)
        }
        GardenCommand::Inbox { db } => run_graph_inbox(absolute(&db)?),
        GardenCommand::Explore { db, limit, open } => {
            crate::explorer::run(&absolute(&db)?, usize::from(limit), open)
        }
        GardenCommand::Mcp { .. } => unreachable!("Graph MCP is dispatched asynchronously"),
    }
}
