#[cfg(feature = "legacy-sqlite")]
mod animaos;
#[cfg(feature = "legacy-sqlite")]
mod apps;
#[cfg(feature = "legacy-sqlite")]
mod cli;
#[cfg(not(feature = "legacy-sqlite"))]
#[path = "native_cli.rs"]
mod cli;
#[cfg(feature = "legacy-sqlite")]
mod connection;
mod cutover_cli;
mod garden_cli;
#[cfg(feature = "legacy-sqlite")]
mod health;
#[cfg(feature = "legacy-sqlite")]
mod lock;
mod mcp;
#[cfg(feature = "legacy-sqlite")]
mod tui;

#[tokio::main]
async fn main() {
    if let Err(error) = cli::run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
