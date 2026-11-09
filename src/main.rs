mod animaos;
mod cli;
mod connection;
mod health;
mod mcp;
mod tui;

#[tokio::main]
async fn main() {
    if let Err(error) = cli::run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
