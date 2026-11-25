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

/// Clap's generated parser for the full command tree needs more than the 1 MiB Windows
/// main-thread stack in unoptimized builds, so the CLI runs on a thread with an explicit
/// stack. Behaviour is otherwise identical to `#[tokio::main]`.
const CLI_STACK_BYTES: usize = 8 * 1024 * 1024;

fn main() {
    let worker = std::thread::Builder::new()
        .name("mother-cli".into())
        .stack_size(CLI_STACK_BYTES)
        .spawn(|| {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?;
            runtime.block_on(cli::run())
        });
    let outcome = match worker {
        Ok(worker) => worker.join(),
        Err(error) => {
            eprintln!("Could not start the command thread: {error}");
            std::process::exit(1);
        }
    };
    match outcome {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
        Err(panic) => std::panic::resume_unwind(panic),
    }
}
