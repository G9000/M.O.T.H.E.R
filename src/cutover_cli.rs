//! Explicit owner operations for local service and journaled cutover.
use clap::Subcommand;
use mother::{
    config::{create_private_file, read_json},
    error::Result,
    garden::{config::load_garden_config, migrate},
    recall_profiles::RecallProfiles,
    service::{LocalClient, LocalService, ServiceClientConfig, ServiceInfo},
};
use serde_json::json;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Subcommand)]
pub enum MigrationCommand {
    /// Read a durable cutover journal; this does not retry an uncertain action.
    Status {
        #[arg(long)]
        journal: PathBuf,
    },
    /// Reissue only one original client's captured restrictions, with fresh credentials.
    Reissue {
        #[arg(long)]
        journal: PathBuf,
        #[arg(long)]
        client: String,
        #[arg(long)]
        out: PathBuf,
    },
    /// Roll back only when no destination canonical state would be discarded.
    Rollback {
        #[arg(long)]
        journal: PathBuf,
        #[arg(long)]
        operation_id: String,
    },
}
fn absolute(path: &Path) -> Result<PathBuf> {
    Ok(std::path::absolute(path)?)
}
pub fn service(db: PathBuf, endpoint: PathBuf, recall_profiles: Option<PathBuf>) -> Result<()> {
    let profiles = recall_profiles
        .as_deref()
        .map(RecallProfiles::load)
        .transpose()?
        .unwrap_or_default();
    let service =
        LocalService::bind_with_recall_profiles(&absolute(&db)?, &absolute(&endpoint)?, profiles)?;
    println!("{}", serde_json::to_string(service.info())?);
    std::io::stdout().flush()?;
    service.run()
}
pub fn service_config(config: PathBuf, endpoint: PathBuf, out: PathBuf) -> Result<()> {
    let source = load_garden_config(&absolute(&config)?)?;
    let endpoint = absolute(&endpoint)?;
    let info: ServiceInfo = serde_json::from_value(read_json(&endpoint, 16_384)?)?;
    let config = ServiceClientConfig {
        version: 3,
        backend: "garden_service".into(),
        endpoint_path: endpoint,
        workspace_id: info.workspace_id,
        generation: info.generation,
        client_id: source.client_id,
        token: source.token,
    };
    LocalClient::new(config.clone())?;
    let out = absolute(&out)?;
    mother::config::check_private_output(&out)?;
    let mut file = create_private_file(&out)?;
    serde_json::to_writer_pretty(&mut file, &config)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!(
        "{}",
        json!({"configPath":out,"clientId":config.client_id,"workspaceId":config.workspace_id,"generation":config.generation})
    );
    Ok(())
}
pub fn migration(command: MigrationCommand) -> Result<()> {
    let value = match command {
        MigrationCommand::Status { journal } => {
            serde_json::to_value(migrate::status(&absolute(&journal)?)?)?
        }
        MigrationCommand::Rollback {
            journal,
            operation_id,
        } => serde_json::to_value(migrate::rollback(&absolute(&journal)?, &operation_id)?)?,
        MigrationCommand::Reissue {
            journal,
            client,
            out,
        } => serde_json::to_value(migrate::reissue(
            &absolute(&journal)?,
            &client,
            &absolute(&out)?,
        )?)?,
    };
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
