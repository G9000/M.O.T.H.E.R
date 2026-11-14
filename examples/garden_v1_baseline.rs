//! Synthetic measurements of the unchanged v1 Store API, not a graph comparison.
use mother::{contracts::Auth, store::Store};
use serde_json::{Value, json};
use std::time::Instant;

fn distribution(mut samples: Vec<f64>) -> Value {
    samples.sort_by(f64::total_cmp);
    let pick = |percent: usize| samples[(samples.len() * percent).div_ceil(100).saturating_sub(1)];
    json!({"samples":samples.len(),"p50_ms":pick(50),"p95_ms":pick(95),"p99_ms":pick(99)})
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 || args[0] != "--synthetic" || args[1] != "--records" {
        return Err("Use --synthetic --records <1..10000>; no database path is accepted".into());
    }
    let records: usize = args[2].parse()?;
    if !(1..=10_000).contains(&records) {
        return Err("v1 supports at most 10000 canonical records".into());
    }
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("synthetic-v1.sqlite");
    let mut store = Store::initialize(&path, "Synthetic storage benchmark persona")?;
    let workspace = store.workspace()?;
    let grant = store.grant(
        "synthetic-benchmark",
        &["read".into(), "memory:write".into()],
    )?;
    let auth = Auth {
        client_id: "synthetic-benchmark".into(),
        token: grant["token"]
            .as_str()
            .ok_or("Missing synthetic token")?
            .into(),
    };
    let mut ids = Vec::with_capacity(records);
    let mut writes = Vec::with_capacity(records);
    let load_start = Instant::now();
    for id in 1..=records {
        let request = json!({"workspaceId":workspace["id"],"generation":workspace["generation"],
            "operationId":format!("00000000-0000-4000-8000-{id:012x}"),"expectedRevision":0,
            "record":{"kind":"memory","title":format!("Synthetic entity {id}"),
                "content":format!("Synthetic benchmark assertion number {id}; source seed 14."),
                "basis":"source","status":"active"}});
        let start = Instant::now();
        let receipt = store.write(&auth, request)?;
        writes.push(start.elapsed().as_secs_f64() * 1000.0);
        ids.push(
            receipt["record"]["id"]
                .as_str()
                .ok_or("Missing canonical ID")?
                .to_owned(),
        );
    }
    let load_ms = load_start.elapsed().as_secs_f64() * 1000.0;
    drop(store);
    let mut store = Store::open(&path)?;
    let mut first_pass = Vec::new();
    let mut warm = Vec::new();
    for (pass, output) in [(0, &mut first_pass), (1, &mut warm)] {
        for sample in 0..512 {
            let id = &ids[(sample * 7919 + 14) % records];
            let start = Instant::now();
            let record = store.get(&auth, id)?;
            std::hint::black_box(record);
            output.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        std::hint::black_box(pass);
    }
    let search = mother::search::Search::new(dir.path().join("absent-model"));
    let mut lexical = Vec::new();
    for sample in 0..128 {
        let query = format!("assertion number {}", 1 + sample * 7919 % records);
        let start = Instant::now();
        std::hint::black_box(search.search(
            &mut store,
            &auth,
            json!({"query":query,"limit":20,"mode":"literal"}),
        )?);
        lexical.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    let bytes = std::fs::read_dir(dir.path())?.try_fold(0_u64, |total, entry| {
        Ok::<_, std::io::Error>(total + entry?.metadata()?.len())
    })?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "implementation":"unchanged Mother v1 Store API", "records":records,
            "durability":"existing WAL + synchronous FULL", "load_ms":load_ms,
            "individual_authenticated_writes":distribution(writes),
            "reopened_first_pass_lookup":distribution(first_pass),"warm_lookup":distribution(warm),
            "literal_substring_candidates":distribution(lexical),"disk_bytes_including_wal":bytes,
            "scope":"v1 auth, validation, receipts and capacity limits included; no graph semantics or cross-engine speed ratio",
            "cold_cache":"not OS-cache flushed; reopen only"
        }))?
    );
    Ok(())
}
