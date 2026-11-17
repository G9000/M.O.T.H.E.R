//! Offline synthetic evaluation. No live database/credential paths or provider calls.
#[path = "memory_eval/mod.rs"]
mod evaluation;
use clap::Parser;
use std::{io::Write, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(about = "Evaluate unchanged Mother retrieval on isolated synthetic memories")]
struct Options {
    /// New report file (never overwrites). Omit to print JSON to stdout.
    #[arg(long)]
    output: Option<PathBuf>,
    /// Also fail on quality misses; hard/safety/evaluator failures always fail.
    #[arg(long)]
    require_quality: bool,
}
fn main() -> ExitCode {
    let options = Options::parse();
    let result = (|| -> Result<u8, Box<dyn std::error::Error>> {
        let report = evaluation::run()?;
        if let Some(path) = &options.output {
            evaluation::write_new(path, &report)?;
        } else {
            let mut out = std::io::stdout().lock();
            serde_json::to_writer_pretty(&mut out, &report)?;
            out.write_all(b"\n")?;
        }
        Ok(evaluation::exit_code(&report, options.require_quality))
    })();
    match result {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("Memory evaluation failed: {error}");
            ExitCode::from(2)
        }
    }
}
