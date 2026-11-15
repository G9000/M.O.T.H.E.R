//! Audit an explicitly marked disposable copy of a retained synthetic benchmark.
//! Never accepts an ordinary/live database or prints the temporary credential.
use clap::Parser;
use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, store::GardenStore},
};
use serde_json::json;
use std::{path::PathBuf, time::Instant};

#[derive(Parser)]
struct Options {
    #[arg(long, required = true)]
    synthetic_copy: bool,
    #[arg(long)]
    database: PathBuf,
    #[arg(long)]
    expected_current: u64,
    #[arg(long)]
    expected_history: u64,
    #[arg(long)]
    expected_receipts: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    let parent = options
        .database
        .parent()
        .ok_or("missing synthetic parent")?;
    if !options.synthetic_copy
        || !options.database.is_absolute()
        || options.database.file_name().and_then(|name| name.to_str()) != Some("audit-copy.redb")
        || std::fs::symlink_metadata(parent)?.file_type().is_symlink()
        || std::fs::symlink_metadata(&options.database)?
            .file_type()
            .is_symlink()
        || std::fs::read_to_string(parent.join(".mother-synthetic-audit-copy"))?.trim()
            != "Disposable synthetic G9 audit qualification copy"
    {
        return Err("explicit marked synthetic copy required".into());
    }
    let store = GardenStore::open_active(&options.database)?;
    // Only this disposable copy receives a fresh synthetic credential. Canonical
    // records, history, receipts and generation stay unchanged; the original
    // benchmark database is never opened by this executable.
    let auth = store.grant(
        &format!("audit-{}", uuid::Uuid::new_v4()),
        &["garden:read".into()],
        &ClientAccess::default(),
        &GardenAccess::All {},
    )?;
    let started = Instant::now();
    let result = store.audit(&auth);
    let elapsed = started.elapsed().as_secs_f64();
    let actual = result.as_ref().ok().map(|stats| {
        json!({"current_records":stats.current_records,"history_records":stats.history_records,
            "receipts":stats.receipts,"sequence":stats.sequence})
    });
    let expected = json!({"current_records":options.expected_current,
        "history_records":options.expected_history,"receipts":options.expected_receipts,
        "sequence":options.expected_receipts});
    let passed = actual.as_ref() == Some(&expected);
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "synthetic":true,"fixture":"retained_benchmark_disposable_copy",
            "api":"GardenStore::audit with full graph read authority",
            "audit_seconds":elapsed,"passed":passed,"actual":actual,"expected":expected,
            "failure":result.as_ref().err().map(ToString::to_string),
            "preparation":"fresh synthetic read grant on copy only; original benchmark database untouched",
            "memory_scope":"external process monitor includes store open, grant preparation and audit"
        }))?
    );
    result?;
    if !passed {
        return Err("audited cardinalities differ from retained benchmark acknowledgements".into());
    }
    Ok(())
}
