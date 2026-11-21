mod capture_cli;
#[path = "native_cli.rs"]
mod cli;
mod cutover_cli;
mod explorer;
mod garden_cli;
mod mcp;
mod media_cli;
mod relationships;
mod tending;
mod timeline;
mod tui;

#[tokio::main]
async fn main() {
    if let Err(error) = cli::run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
