//! Measurements deliberately separate bulk ingestion from durable small transactions.
use crate::Config;
use garden_store::{
    Batch, Mutation, RedbStore, Result, Traversal, synthetic_assertion, synthetic_entity,
};
use serde_json::{Value, json};
use std::{
    fs,
    hint::black_box,
    io::Read,
    path::Path,
    process::{Child, Command, Stdio},
    sync::{Arc, Barrier, mpsc},
    thread,
    time::{Duration, Instant},
};

const LOOKUPS: u64 = 256;
const QUERIES: u64 = 128;
const BATCHES: u64 = 64;
const ASSERTIONS_PER_BATCH: u64 = 32;
const INGEST_ENTITIES: u64 = 256;

pub(crate) fn stats(mut samples: Vec<Duration>) -> Value {
    samples.sort_unstable();
    let percentile = |percent: usize| -> f64 {
        let rank = (percent * samples.len()).div_ceil(100).saturating_sub(1);
        samples[rank].as_secs_f64() * 1_000.0
    };
    json!({"samples":samples.len(), "unit":"milliseconds", "p50":percentile(50), "p95":percentile(95), "p99":percentile(99), "total":samples.iter().map(Duration::as_secs_f64).sum::<f64>()*1000.0})
}

pub(crate) fn sample_id(index: u64, seed: u64, count: u64) -> u64 {
    let mut value = index.wrapping_add(seed).wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    (value ^ (value >> 31)) % count + 1
}

pub(crate) fn bulk_batch(config: Config, start: u64, operation_id: u64) -> Batch {
    let end = (start + INGEST_ENTITIES).min(config.records + 1);
    Batch {
        operation_id,
        entities: vec![],
        assertions: (start..end)
            .flat_map(|id| {
                (0..config.edges).map(move |edge| Mutation {
                    assertion: synthetic_assertion(
                        id,
                        edge,
                        config.records,
                        config.edges,
                        config.seed,
                    ),
                    expected_revision: 0,
                })
            })
            .collect(),
    }
}

pub(crate) fn entity_batch(config: Config, start: u64, operation_id: u64) -> Batch {
    Batch {
        operation_id,
        entities: (start..(start + INGEST_ENTITIES).min(config.records + 1))
            .map(synthetic_entity)
            .collect(),
        assertions: vec![],
    }
}

pub(crate) fn small_batch(config: Config, batch_index: u64) -> Batch {
    Batch {
        operation_id: 1_000_000 + batch_index,
        entities: vec![],
        assertions: (0..ASSERTIONS_PER_BATCH)
            .map(|offset| {
                let mut assertion = synthetic_assertion(
                    sample_id(
                        batch_index * ASSERTIONS_PER_BATCH + offset,
                        config.seed,
                        config.records,
                    ),
                    offset % config.edges,
                    config.records,
                    config.edges,
                    config.seed,
                );
                assertion.id =
                    config.records * config.edges + batch_index * ASSERTIONS_PER_BATCH + offset + 1;
                Mutation {
                    assertion,
                    expected_revision: 0,
                }
            })
            .collect(),
    }
}

pub(crate) fn traversal(config: Config, sample: u64, sequence: u64) -> Traversal {
    Traversal {
        start: sample_id(sample, config.seed, config.records),
        max_hops: 2,
        max_results: 32,
        max_work: 128,
        valid_at: 5_000,
        known_at: sequence,
        predicate: None,
    }
}

fn reads(
    store: &RedbStore,
    config: Config,
    lookup_count: u64,
    query_count: u64,
    offset: u64,
) -> Result<Value> {
    let sequence = store.counts()?.sequence;
    let mut lookups = Vec::new();
    for sample in 0..lookup_count {
        let id = sample_id(sample + offset, config.seed, config.records * config.edges);
        let start = Instant::now();
        let result = store.snapshot()?.assertion(id)?;
        black_box(result.ok_or("sampled assertion missing")?);
        lookups.push(start.elapsed());
    }
    let mut queries = Vec::new();
    let mut examined = 0;
    let mut truncated = 0;
    let (mut returned, mut visited, mut max_returned, mut max_visited) = (0, 0, 0, 0);
    for sample in 0..query_count {
        let request = traversal(config, sample + offset, sequence);
        let start = Instant::now();
        let result = store.snapshot()?.neighbors(request)?;
        queries.push(start.elapsed());
        if result.work > request.max_work || result.assertions.len() as u64 > request.max_results {
            return Err("traversal exceeded declared bounds".into());
        }
        examined += result.work;
        truncated += u64::from(result.truncated);
        returned += result.assertions.len() as u64;
        visited += result.visited;
        max_returned = max_returned.max(result.assertions.len() as u64);
        max_visited = max_visited.max(result.visited);
        black_box(result);
    }
    Ok(
        json!({"exact_assertion_lookup":stats(lookups), "bounded_temporal_neighbors":stats(queries), "query_work_total":examined, "truncated_queries":truncated, "fanout":{"returned_edges_total":returned,"visited_nodes_total":visited,"max_returned_edges":max_returned,"max_visited_nodes":max_visited}, "bounds":{"hops":2,"results":32,"work":128,"valid_at":5000,"known_at_sequence":sequence}}),
    )
}

pub(crate) fn reopened(config: Config, path: &Path) -> Result<Value> {
    let start = Instant::now();
    let store = RedbStore::open(path)?;
    let open_ms = start.elapsed().as_secs_f64() * 1000.0;
    let read_stats = reads(&store, config, LOOKUPS, QUERIES, 0)?;
    Ok(
        json!({"label":"reopened process/application cache; OS and disk caches uncontrolled", "open_ms":open_ms,"reads":read_stats}),
    )
}

struct WorkerMeasurements {
    reader: bool,
    elapsed: Vec<Duration>,
    waits: Vec<Duration>,
}

fn concurrent(store: Arc<RedbStore>, config: Config) -> Result<Value> {
    let barrier = Arc::new(Barrier::new(7));
    let result: Result<Vec<WorkerMeasurements>> = thread::scope(|scope| {
        let mut handles = Vec::new();
        for worker in 0..6u64 {
            let store = Arc::clone(&store);
            let barrier = Arc::clone(&barrier);
            handles.push(scope.spawn(move || -> Result<_> {
                let mut elapsed = Vec::new();
                let mut waits = Vec::new();
                barrier.wait();
                if worker < 4 {
                    for sample in 0..128 {
                        let sequence = store.counts()?.sequence;
                        let start = Instant::now();
                        black_box(store.snapshot()?.neighbors(traversal(
                            config,
                            sample + worker * 128,
                            sequence,
                        ))?);
                        elapsed.push(start.elapsed());
                    }
                } else {
                    for batch_index in 0..32 {
                        let batch = small_batch(config, 10_000 + (worker - 4) * 32 + batch_index);
                        let start = Instant::now();
                        let (_, wait) = store.apply_timed(&batch)?;
                        elapsed.push(start.elapsed());
                        waits.push(wait);
                    }
                }
                Ok(WorkerMeasurements {
                    reader: worker < 4,
                    elapsed,
                    waits,
                })
            }));
        }
        barrier.wait();
        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .map_err(|_| "benchmark worker panicked".into())
                    .and_then(|result| result)
            })
            .collect()
    });
    let (mut read, mut write, mut waits) = (vec![], vec![], vec![]);
    for measurements in result? {
        if measurements.reader {
            read.extend(measurements.elapsed);
        } else {
            write.extend(measurements.elapsed);
            waits.extend(measurements.waits);
        }
    }
    Ok(
        json!({"reader_threads":4,"writer_threads":2,"assertions_per_batch":32,"reader_query_latency":stats(read),"writer_batch_latency":stats(write),"writer_begin_transaction_wait":stats(waits),"note":"bounded operations start together; overlap and scheduling are measured, not guaranteed"}),
    )
}

struct ReopenChild(Child);

impl Drop for ReopenChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn reopen_child(config: Config, root: &Path, nonce: &str) -> Result<Value> {
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut child = ReopenChild(
        Command::new(std::env::current_exe()?)
            .args([
                "--internal-reopen-child",
                "--synthetic",
                "--records",
                &config.records.to_string(),
                "--edges-per-record",
                &config.edges.to_string(),
                "--seed",
                &config.seed.to_string(),
            ])
            .env("GARDEN_SYNTHETIC_CHILD_ROOT", root)
            .env("GARDEN_SYNTHETIC_CHILD_NONCE", nonce)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?,
    );
    let mut stdout = child.0.stdout.take().ok_or("reopen stdout missing")?;
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut bytes = Vec::new();
        let result = stdout.read_to_end(&mut bytes).map(|_| bytes);
        let _ = sender.send(result);
    });
    let bytes = receiver.recv_timeout(deadline.saturating_duration_since(Instant::now()))??;
    let status = loop {
        if let Some(status) = child.0.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            return Err("reopen child exceeded 60 seconds".into());
        }
        thread::sleep(Duration::from_millis(10));
    };
    if !status.success() {
        return Err("reopen child failed; see synthetic stderr diagnostic".into());
    }
    Ok(serde_json::from_slice(&bytes)?)
}

pub(crate) fn run(config: Config) -> Result<Value> {
    let root = tempfile::Builder::new()
        .prefix("garden-synthetic-")
        .tempdir()?;
    let nonce = format!("synthetic-{}-{}", std::process::id(), config.seed);
    fs::write(root.path().join("garden-synthetic-owner"), &nonce)?;
    let path = root.path().join("garden.redb");
    let store = Arc::new(RedbStore::open(&path)?);
    let ingest_start = Instant::now();
    let mut bulk_transactions = 0;
    for start in (1..=config.records).step_by(INGEST_ENTITIES as usize) {
        bulk_transactions += 1;
        store.apply(&entity_batch(config, start, bulk_transactions))?;
    }
    let entity_ingest_ms = ingest_start.elapsed().as_secs_f64() * 1000.0;
    for start in (1..=config.records).step_by(INGEST_ENTITIES as usize) {
        bulk_transactions += 1;
        store.apply(&bulk_batch(config, start, bulk_transactions))?;
    }
    let ingest_ms = ingest_start.elapsed().as_secs_f64() * 1000.0;
    let warmup = reads(&store, config, 32, 16, 10_000)?;
    let warm = reads(&store, config, LOOKUPS, QUERIES, 0)?;
    drop(store);
    let reopen = reopen_child(config, root.path(), &nonce)?;
    let store = Arc::new(RedbStore::open(&path)?);
    #[cfg(feature = "sqlite-baseline")]
    let sqlite = if config.sqlite {
        crate::sqlite::run(config, root.path(), &store)?
    } else {
        Value::Null
    };
    #[cfg(not(feature = "sqlite-baseline"))]
    let sqlite = Value::Null;
    let warmup_start = Instant::now();
    for batch in 0..4 {
        store.apply(&small_batch(config, batch))?;
    }
    let warmup_write_ms = warmup_start.elapsed().as_secs_f64() * 1000.0;
    let mut commits = Vec::new();
    let mut waits = Vec::new();
    for batch in 4..4 + BATCHES {
        let batch = small_batch(config, batch);
        let start = Instant::now();
        let (_, wait) = store.apply_timed(&batch)?;
        commits.push(start.elapsed());
        waits.push(wait);
    }
    let contention = concurrent(Arc::clone(&store), config)?;
    let counts = store.verify_indexes()?;
    drop(store);
    let disk_bytes = fs::metadata(&path)?.len();
    Ok(
        json!({"experiment":"G2 synthetic graph storage kernel qualification", "engine":"redb 4.6.0", "durability":"Immediate transaction commit", "dataset":{"entities":config.records,"edges_per_entity":config.edges,"seed":config.seed,"generated_edges":config.records*config.edges,"streaming_entity_chunk":INGEST_ENTITIES}, "storage":"owned temporary directory; removed on exit", "bulk_ingest":{"milliseconds":ingest_ms,"entity_phase_ms":entity_ingest_ms,"assertion_phase_ms":ingest_ms-entity_ingest_ms,"transactions":bulk_transactions,"note":"bulk timing is not a small durable batch measurement"}, "warmup":{"reads":warmup,"write_batches":4,"assertions_per_batch":32,"write_ms":warmup_write_ms},"warm_reads":warm,"durable_32_assertion_batch":stats(commits),"uncontended_begin_transaction_wait":stats(waits),"contention":contention,"reopened_process":reopen,"final_counts":counts,"database_file_bytes_before_cleanup":disk_bytes,"sqlite_baseline":sqlite,"lexical_candidates":{"status":"deferred","reason":"no text corpus or lexical index implemented; no lexical performance claim"},"rss":{"status":"external monitor required","reason":"portable Rust std does not expose peak process RSS"},"limitations":["synthetic unsigned IDs and logical known-at transaction sequence are not a promoted wire contract","kernel microbenchmark excludes authentication, grants, sources validation and existing M.O.T.H.E.R API overhead","OS cache state is uncontrolled; reopened-process figures are not cold-disk measurements","power loss, torn filesystem writes and hardware fsync behavior are not established by process-kill tests"]}),
    )
}
