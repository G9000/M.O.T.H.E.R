//! Optional SQLite graph-kernel baseline; it is not the production M.O.T.H.E.R Store API.
use crate::{Config, bench};
use garden_store::{
    Assertion, Batch, Receipt, RedbStore, Result, StoredAssertion, Traversal, TraversalResult,
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    hint::black_box,
    path::Path,
    time::{Duration, Instant},
};

struct Sqlite {
    connection: Connection,
}

fn sql_integer(value: u64) -> Result<i64> {
    Ok(i64::try_from(value)?)
}

impl Sqlite {
    fn open(path: &Path) -> Result<Self> {
        let connection = Connection::open(path)?;
        connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;
            CREATE TABLE entities(id INTEGER PRIMARY KEY,payload BLOB NOT NULL);
            CREATE TABLE assertions(id INTEGER PRIMARY KEY,subject INTEGER NOT NULL,predicate INTEGER NOT NULL,object INTEGER NOT NULL,valid_from INTEGER NOT NULL,valid_until INTEGER NOT NULL,source INTEGER NOT NULL,namespace INTEGER NOT NULL,sequence INTEGER NOT NULL,payload BLOB NOT NULL);
            CREATE TABLE history(id INTEGER NOT NULL,sequence INTEGER NOT NULL,subject INTEGER NOT NULL,payload BLOB NOT NULL,PRIMARY KEY(id,sequence));
            CREATE INDEX outgoing_history ON history(subject,id,sequence);
            CREATE INDEX incoming_current ON assertions(object,id);
            CREATE INDEX predicate_current ON assertions(predicate,id);
            CREATE INDEX valid_current ON assertions(valid_from,valid_until,id);
            CREATE INDEX source_current ON assertions(source,id);
            CREATE INDEX namespace_current ON assertions(namespace,id);
            CREATE TABLE receipts(operation_id INTEGER PRIMARY KEY,sequence INTEGER NOT NULL,receipt BLOB NOT NULL,batch BLOB NOT NULL);
            CREATE TABLE metadata(id INTEGER PRIMARY KEY CHECK(id=1),sequence INTEGER NOT NULL);
            INSERT INTO metadata VALUES(1,0);")?;
        Ok(Self { connection })
    }

    fn apply(&mut self, batch: &Batch) -> Result<Duration> {
        let start = Instant::now();
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let wait = start.elapsed();
        let sql_sequence: i64 =
            transaction.query_row("SELECT sequence+1 FROM metadata WHERE id=1", [], |row| {
                row.get(0)
            })?;
        let sequence = u64::try_from(sql_sequence)?;
        {
            let mut insert = transaction.prepare_cached("INSERT INTO entities VALUES(?1,?2)")?;
            for entity in &batch.entities {
                insert.execute(params![
                    sql_integer(entity.id)?,
                    serde_json::to_vec(entity)?
                ])?;
            }
        }
        {
            let mut current = transaction
                .prepare_cached("INSERT INTO assertions VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)")?;
            let mut history =
                transaction.prepare_cached("INSERT INTO history VALUES(?1,?2,?3,?4)")?;
            for mutation in &batch.assertions {
                if mutation.expected_revision != 0 {
                    return Err("SQLite benchmark baseline supports generated creates only".into());
                }
                let assertion = mutation.assertion;
                let stored = StoredAssertion {
                    assertion,
                    revision: 1,
                    sequence,
                };
                let payload = serde_json::to_vec(&stored)?;
                current.execute(params![
                    sql_integer(assertion.id)?,
                    sql_integer(assertion.subject)?,
                    sql_integer(assertion.predicate)?,
                    sql_integer(assertion.object)?,
                    sql_integer(assertion.valid_from)?,
                    sql_integer(assertion.valid_until)?,
                    sql_integer(assertion.source)?,
                    sql_integer(assertion.namespace)?,
                    sql_sequence,
                    &payload
                ])?;
                history.execute(params![
                    sql_integer(assertion.id)?,
                    sql_sequence,
                    sql_integer(assertion.subject)?,
                    &payload
                ])?;
            }
        }
        let receipt = Receipt {
            operation_id: batch.operation_id,
            sequence,
            entity_count: batch.entities.len() as u64,
            assertion_count: batch.assertions.len() as u64,
        };
        transaction.execute(
            "INSERT INTO receipts VALUES(?1,?2,?3,?4)",
            params![
                sql_integer(batch.operation_id)?,
                sql_sequence,
                serde_json::to_vec(&receipt)?,
                serde_json::to_vec(batch)?
            ],
        )?;
        transaction.execute("UPDATE metadata SET sequence=?1 WHERE id=1", [sql_sequence])?;
        transaction.commit()?;
        Ok(wait)
    }

    fn lookup(&self, id: u64) -> Result<Option<StoredAssertion>> {
        let payload: Option<Vec<u8>> = self
            .connection
            .query_row(
                "SELECT payload FROM assertions WHERE id=?1",
                [sql_integer(id)?],
                |row| row.get(0),
            )
            .optional()?;
        payload
            .map(|bytes| serde_json::from_slice(&bytes).map_err(Into::into))
            .transpose()
    }

    fn neighbors(&self, request: Traversal) -> Result<TraversalResult> {
        let mut queue = VecDeque::from([(request.start, 0)]);
        let mut seen = BTreeSet::new();
        let mut returned = BTreeSet::new();
        let mut results = Vec::new();
        let mut work = 0;
        let mut truncated = false;
        let mut statement = self
            .connection
            .prepare_cached("SELECT payload FROM history WHERE subject=?1 ORDER BY id,sequence")?;
        'walk: while let Some((subject, depth)) = queue.pop_front() {
            if !seen.insert(subject) {
                continue;
            }
            if depth >= request.max_hops {
                continue;
            }
            let mut candidates = BTreeMap::new();
            let mut rows = statement.query([sql_integer(subject)?])?;
            while let Some(row) = rows.next()? {
                if work >= request.max_work {
                    truncated = true;
                    break 'walk;
                }
                work += 1;
                let payload: Vec<u8> = row.get(0)?;
                let stored: StoredAssertion = serde_json::from_slice(&payload)?;
                if stored.sequence <= request.known_at {
                    candidates.insert(stored.assertion.id, stored);
                }
            }
            for stored in candidates.into_values() {
                let assertion: Assertion = stored.assertion;
                if assertion.valid_from > request.valid_at
                    || request.valid_at >= assertion.valid_until
                    || request
                        .predicate
                        .is_some_and(|predicate| predicate != assertion.predicate)
                    || !returned.insert(assertion.id)
                {
                    continue;
                }
                if results.len() as u64 >= request.max_results {
                    truncated = true;
                    break 'walk;
                }
                results.push(stored);
                queue.push_back((assertion.object, depth + 1));
            }
        }
        Ok(TraversalResult {
            assertions: results,
            visited: seen.len() as u64,
            work,
            truncated,
        })
    }
}

pub(crate) fn run(config: Config, root: &Path, redb: &RedbStore) -> Result<Value> {
    let path = root.join("baseline.sqlite");
    let mut sqlite = Sqlite::open(&path)?;
    let start = Instant::now();
    let mut transaction = 0;
    for id in (1..=config.records).step_by(256) {
        transaction += 1;
        sqlite.apply(&bench::entity_batch(config, id, transaction))?;
    }
    for id in (1..=config.records).step_by(256) {
        transaction += 1;
        sqlite.apply(&bench::bulk_batch(config, id, transaction))?;
    }
    let ingest_ms = start.elapsed().as_secs_f64() * 1000.0;
    let sequence = transaction;
    // The read comparison occurs before either kernel's benchmark mutations.
    let mut warmup_lookup = Vec::new();
    let mut warmup_query = Vec::new();
    for sample in 10_000..10_032 {
        let start = Instant::now();
        black_box(sqlite.lookup(bench::sample_id(
            sample,
            config.seed,
            config.records * config.edges,
        ))?);
        warmup_lookup.push(start.elapsed());
    }
    for sample in 10_000..10_016 {
        let start = Instant::now();
        black_box(sqlite.neighbors(bench::traversal(config, sample, sequence))?);
        warmup_query.push(start.elapsed());
    }
    let mut lookup_times = Vec::new();
    for sample in 0..256 {
        let id = bench::sample_id(sample, config.seed, config.records * config.edges);
        let start = Instant::now();
        let actual = sqlite.lookup(id)?;
        lookup_times.push(start.elapsed());
        if actual != redb.snapshot()?.assertion(id)? {
            return Err("SQLite/redb generated lookup payload mismatch".into());
        }
        black_box(actual);
    }
    let mut query_times = Vec::new();
    for sample in 0..128 {
        let request = bench::traversal(config, sample, sequence);
        let start = Instant::now();
        let actual = sqlite.neighbors(request)?;
        query_times.push(start.elapsed());
        let expected = redb.snapshot()?.neighbors(request)?;
        if actual != expected {
            return Err(format!("SQLite/redb bounded query mismatch at sample {sample}: sqlite={actual:?}, redb={expected:?}").into());
        }
        black_box(actual);
    }
    let warmup_start = Instant::now();
    for batch in 0..4 {
        sqlite.apply(&bench::small_batch(config, batch))?;
    }
    let warmup_write_ms = warmup_start.elapsed().as_secs_f64() * 1000.0;
    let mut commits = Vec::new();
    let mut waits = Vec::new();
    for batch in 4..68 {
        let batch = bench::small_batch(config, batch);
        let start = Instant::now();
        waits.push(sqlite.apply(&batch)?);
        commits.push(start.elapsed());
    }
    sqlite
        .connection
        .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")?;
    let disk_bytes = fs::metadata(path)?.len();
    Ok(
        json!({"engine":"SQLite via optional bundled rusqlite", "scope":"semantic graph reference; timings are not comparable engine costs or the existing authenticated M.O.T.H.E.R Store API", "durability":"WAL synchronous=FULL; immediate transactions", "equivalence":{"exact_generated_payloads_checked":256,"bounded_temporal_queries_checked":128,"same_bulk_chunks":256,"same_assertions_per_durable_batch":32,"current_secondary_indexes":"incoming,predicate,validity,source,namespace; outgoing append-only history"},"bulk_ingest_ms":ingest_ms,"warmup":{"lookup":bench::stats(warmup_lookup),"query":bench::stats(warmup_query),"write_batches":4,"write_ms":warmup_write_ms},"warm_exact_lookup":bench::stats(lookup_times),"warm_bounded_temporal_neighbors":bench::stats(query_times),"durable_32_assertion_batch":bench::stats(commits),"begin_transaction_wait":bench::stats(waits),"database_file_bytes_after_wal_checkpoint":disk_bytes,"limitations":["JSON versus redb fixed binary encoding; index sets, validation and receipt replay costs differ; do not rank engine speed from these timings","generated creates only; no claim about update/authentication/grant/receipt replay API equivalence","SQLite concurrency and reopened-process latency not measured","file size measured after explicit WAL checkpoint"]}),
    )
}
