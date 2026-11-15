//! Synthetic completed-engine qualification through authenticated public GardenStore methods.

use clap::Parser;
use mother::{
    access::ClientAccess,
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::{
            AssertionObject, BatchItem, BatchRequest, Coverage, Direction, ExternalReference,
            GardenDraft, GardenErrorCode, GardenKind, GetRequest, MAX_BATCH_ITEMS, Mutation,
            ObservedTime, OutcomeResponse, QueryRequest, QueryResponse, RecordRef, SourceLocator,
            TimePrecision, Traversal, ValidTime,
        },
        store::GardenStore,
    },
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    error::Error,
    path::Path,
    sync::{Arc, Barrier},
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
const WORKSPACE: &str = "00000000-0000-4000-8000-000000000001";
const GENERATION: &str = "00000000-0000-4000-8000-000000000002";
const VALID_AT: &str = "2025-01-01T00:00:00Z";

#[derive(Debug, Parser)]
#[command(about = "Completed Garden engine, synthetic data only; no external paths or services")]
struct Options {
    #[arg(long, required = true)]
    synthetic: bool,
    #[arg(long, default_value = "10000", value_parser = parse_entities)]
    entities: usize,
    #[arg(long, default_value = "128", value_parser = clap::value_parser!(u32).range(1..=10000))]
    samples: u32,
    #[arg(long, default_value = "64", value_parser = clap::value_parser!(u32).range(1..=10000))]
    write_samples: u32,
    /// Soft deadline checked between API calls, never cancels an atomic commit or audit.
    #[arg(long, default_value = "3600", value_parser = clap::value_parser!(u64).range(1..))]
    deadline_seconds: u64,
    #[arg(long, default_value = "4096", value_parser = clap::value_parser!(u32).range(129..=100000))]
    high_degree_edges: u32,
    #[arg(long, default_value = "128", value_parser = clap::value_parser!(u32).range(2..=10000))]
    history_revisions: u32,
    /// Retain the owned synthetic database even on successful completion.
    #[arg(long)]
    keep_database: bool,
}

fn parse_entities(value: &str) -> std::result::Result<usize, String> {
    match value.parse::<usize>() {
        Ok(n) if (8..=1000).contains(&n) || matches!(n, 10_000 | 100_000 | 1_000_000) => Ok(n),
        _ => Err("Use 10000, 100000, 1000000, or a smoke size 8..1000".into()),
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct LexicalTarget {
    found: bool,
    coverage_complete: bool,
    coverage_truncated: bool,
    reason: Option<String>,
    has_next_page: bool,
}

#[derive(Debug)]
struct Metric {
    milliseconds: Vec<f64>,
    unsuccessful_milliseconds: Vec<f64>,
    unsuccessful_by_code: BTreeMap<String, u64>,
    budget: f64,
    returned: Vec<usize>,
    truncation: BTreeMap<String, u64>,
    lexical_targets: Vec<LexicalTarget>,
}
impl Metric {
    fn new(budget: f64) -> Self {
        Self {
            milliseconds: Vec::new(),
            unsuccessful_milliseconds: Vec::new(),
            unsuccessful_by_code: BTreeMap::new(),
            budget,
            returned: Vec::new(),
            truncation: BTreeMap::new(),
            lexical_targets: Vec::new(),
        }
    }
    fn record(&mut self, start: Instant) {
        self.milliseconds
            .push(start.elapsed().as_secs_f64() * 1000.0);
    }
    fn failure(&mut self, start: Instant, code: &str) {
        self.unsuccessful_milliseconds
            .push(start.elapsed().as_secs_f64() * 1000.0);
        *self.unsuccessful_by_code.entry(code.into()).or_default() += 1;
    }
    fn merge(&mut self, metric: &Self) {
        self.milliseconds.extend_from_slice(&metric.milliseconds);
        self.unsuccessful_milliseconds
            .extend_from_slice(&metric.unsuccessful_milliseconds);
        self.returned.extend_from_slice(&metric.returned);
        self.lexical_targets
            .extend_from_slice(&metric.lexical_targets);
        for (reason, count) in &metric.truncation {
            *self.truncation.entry(reason.clone()).or_default() += count;
        }
        for (code, count) in &metric.unsuccessful_by_code {
            *self.unsuccessful_by_code.entry(code.clone()).or_default() += count;
        }
    }
    fn lexical_target(&mut self, found: bool, coverage: &Coverage, has_next_page: bool) {
        self.lexical_targets.push(LexicalTarget {
            found,
            coverage_complete: coverage.complete,
            coverage_truncated: coverage.truncated,
            reason: coverage.reason.clone(),
            has_next_page,
        });
    }
    fn query(&mut self, start: Instant, result: &QueryResponse) {
        self.record(start);
        self.returned.push(result.hits.len());
        if result.coverage.truncated {
            *self
                .truncation
                .entry(
                    result
                        .coverage
                        .reason
                        .clone()
                        .unwrap_or_else(|| "unspecified".into()),
                )
                .or_default() += 1;
        }
    }
    fn summary(&self) -> Value {
        let mut times = self.milliseconds.clone();
        times.sort_by(f64::total_cmp);
        let percentile = |percent: usize| {
            times
                .get((times.len() * percent).div_ceil(100).saturating_sub(1))
                .copied()
        };
        json!({"samples":times.len(), "p50_ms":percentile(50), "p95_ms":percentile(95),
            "p99_ms":percentile(99), "p95_budget_ms":self.budget,
            "p95_budget_pass":percentile(95).map(|value|value <= self.budget),
            "raw_ms":self.milliseconds, "returned_hits":self.returned,
            "attempts":times.len()+self.unsuccessful_milliseconds.len(),
            "unsuccessful_requests_by_code":self.unsuccessful_by_code,
            "unsuccessful_raw_ms":self.unsuccessful_milliseconds,
            "availability_pass":if times.is_empty() && self.unsuccessful_milliseconds.is_empty() { None } else { Some(self.unsuccessful_milliseconds.is_empty()) },
            "latency_scope":"successful requests only; unsuccessful request latency is reported separately and never satisfies a latency budget",
            "truncated_samples_by_reason":self.truncation,
            "lexical_recall":{
                "requested_targets":self.lexical_targets.len(),
                "found_targets":self.lexical_targets.iter().filter(|target|target.found).count(),
                "missing_targets":self.lexical_targets.iter().filter(|target|!target.found).count(),
                "missing_on_complete_unpaginated_coverage":self.lexical_targets.iter().filter(|target|!target.found && target.coverage_complete && !target.coverage_truncated && !target.has_next_page).count(),
                "pass":if self.lexical_targets.is_empty() { None } else { Some(self.lexical_targets.iter().all(|target|target.found)) },
                "scope":"first bounded candidate page; no automatic retry, increased budget or subsequent page retrieval",
                "samples":self.lexical_targets}})
    }
}

#[derive(Debug)]
struct Run {
    started: Instant,
    deadline: Duration,
    stage: String,
    operation: u64,
    entities: usize,
    assertions: usize,
    history_updates: usize,
    receipts: usize,
    metrics: BTreeMap<String, Metric>,
    checks: BTreeMap<String, Value>,
    last_operation: Option<String>,
}
impl Run {
    fn new(options: &Options) -> Self {
        Self {
            started: Instant::now(),
            deadline: Duration::from_secs(options.deadline_seconds),
            stage: "initializing".into(),
            operation: 0,
            entities: 0,
            assertions: 0,
            history_updates: 0,
            receipts: 0,
            metrics: BTreeMap::new(),
            checks: BTreeMap::new(),
            last_operation: None,
        }
    }
    fn check_deadline(&self) -> Result<()> {
        if self.started.elapsed() >= self.deadline {
            Err("soft_deadline_exceeded".into())
        } else {
            Ok(())
        }
    }
    fn request(&mut self, items: Vec<BatchItem>) -> BatchRequest {
        self.operation += 1;
        let id = synthetic_uuid(3, self.operation);
        self.last_operation = Some(id.clone());
        BatchRequest {
            version: 2,
            workspace_id: WORKSPACE.into(),
            generation: GENERATION.into(),
            operation_id: id,
            required_extensions: BTreeMap::new(),
            items,
        }
    }
    fn apply(
        &mut self,
        store: &GardenStore,
        auth: &Auth,
        items: Vec<BatchItem>,
    ) -> Result<OutcomeResponse> {
        self.check_deadline()?;
        let request = self.request(items);
        let result = store.apply(auth, &request)?;
        self.receipts += 1;
        for item in &request.items {
            match (item.mutation, item.draft.as_ref().map(GardenDraft::kind)) {
                (Mutation::Create, Some(GardenKind::Entity)) => self.entities += 1,
                (Mutation::Create, Some(GardenKind::Assertion)) => self.assertions += 1,
                (Mutation::Update, _) => self.history_updates += 1,
                _ => (),
            }
        }
        Ok(result)
    }
    fn snapshot(&self, options: &Options) -> Value {
        json!({"implementation":"completed GardenStore public API", "synthetic":options.synthetic,
            "requested_entities":options.entities, "requested_initial_relationships":options.entities * 5,
            "seed":14, "generator":"G2 wrapping endpoint mixing and SplitMix sample order, adapted to public v2 UUIDs/UTC times/evidence",
            "stage":self.stage, "elapsed_seconds":self.started.elapsed().as_secs_f64(),
            "progress":{"committed_entities":self.entities,"committed_assertions":self.assertions,
                "committed_history_updates":self.history_updates,"committed_receipts":self.receipts,
                "last_attempted_operation_id":self.last_operation},
            "metrics":self.metrics.iter().map(|(name,m)|(name,m.summary())).collect::<BTreeMap<_,_>>(),
            "checks":self.checks, "contract_seed_batch_items":MAX_BATCH_ITEMS,
            "durable_measurement_assertions_per_batch":32, "durability":"GardenStore::apply Immediate",
            "percentiles":"nearest-rank; p99 of 64 samples is the maximum",
            "deadline":"cooperative between API calls; active transactions/audit are not cancelled",
            "traversal":{"max_hops":2,"max_edges":32,"work_budget":128,"limit":32,
                "actual_examined_work":"unavailable in public QueryResponse", "visited_nodes":"unavailable in public QueryResponse"},
            "cold_disk":"unavailable; application-cache reopen only, OS caches untouched",
            "ram":"external process monitor required", "service_ipc":"not measured in this storage runner",
            "embeddings":"not requested; no model/network calls",
            "query_cost":"includes current auth, validation, indexes, snapshot leases and pager clock writes",
            "reproducibility":"logical graph and sample order deterministic; canonical UUIDs, grant token, store transaction times are generated by engine"})
    }
    fn progress(&self, options: &Options, root: &Path) -> Result<()> {
        std::fs::write(
            root.join("progress.json"),
            serde_json::to_vec_pretty(&self.snapshot(options))?,
        )?;
        Ok(())
    }
}

fn synthetic_uuid(domain: u16, index: u64) -> String {
    format!("00000000-{domain:04x}-4000-8000-{index:012x}")
}
#[cfg(test)]
fn reference(kind: GardenKind, id: String) -> ExternalReference {
    ExternalReference { kind, id }
}
fn existing(reference: &ExternalReference) -> RecordRef {
    RecordRef::Existing {
        kind: reference.kind,
        id: reference.id.clone(),
    }
}
fn sample_index(sample: u64, count: usize) -> usize {
    let mut value = sample.wrapping_add(14).wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    ((value ^ (value >> 31)) % count as u64) as usize
}
fn create(index: usize, draft: GardenDraft) -> BatchItem {
    BatchItem {
        item_key: format!("i{index}"),
        expected_revision: 0,
        mutation: Mutation::Create,
        id: None,
        draft: Some(draft),
        replacement: None,
    }
}
fn entity(index: usize) -> GardenDraft {
    GardenDraft::Entity {
        entity_type: "synthetic:entity".into(),
        display_name: format!("Synthetic seed14 entity {index:07}"),
        aliases: vec![],
        extensions: BTreeMap::new(),
    }
}
fn observed() -> ObservedTime {
    ObservedTime {
        at: Some("2024-01-01T00:00:00Z".into()),
        source_timezone: Some("UTC".into()),
        precision: TimePrecision::Instant,
    }
}
fn edge_draft(
    entities: &[ExternalReference],
    source: &ExternalReference,
    index: usize,
    high_degree: bool,
) -> GardenDraft {
    let subject = if high_degree {
        0
    } else {
        (index / 5) % entities.len()
    };
    let target = if high_degree {
        (index + 1) % entities.len()
    } else {
        ((subject as u64 + 1)
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(14)
            .wrapping_add((index as u64 % 5).wrapping_mul(1_442_695_040_888_963_407))
            % entities.len() as u64) as usize
    };
    GardenDraft::Assertion {
        subject: existing(&entities[subject]),
        predicate: "synthetic:related".into(),
        object: AssertionObject::EntityRef {
            entity_ref: existing(&entities[target]),
        },
        valid_time: ValidTime {
            valid_from: Some("2020-01-01T00:00:00Z".into()),
            valid_until: Some("2030-01-01T00:00:00Z".into()),
            source_timezone: Some("UTC".into()),
            precision: TimePrecision::Instant,
        },
        observed_time: observed(),
        evidence: vec![existing(source)],
        confidence: None,
        extensions: BTreeMap::new(),
    }
}
fn get_request(reference: ExternalReference, known_at: Option<String>) -> GetRequest {
    GetRequest {
        version: 2,
        workspace_id: WORKSPACE.into(),
        generation: GENERATION.into(),
        required_extensions: BTreeMap::new(),
        reference,
        revision: None,
        known_at,
    }
}
fn query(start: &ExternalReference, known_at: Option<String>) -> QueryRequest {
    QueryRequest {
        version: 2,
        workspace_id: WORKSPACE.into(),
        generation: GENERATION.into(),
        required_extensions: BTreeMap::new(),
        ids: vec![],
        kinds: vec![GardenKind::Assertion],
        entity_type: None,
        predicate: Some("synthetic:related".into()),
        property: None,
        valid_at: Some(VALID_AT.into()),
        known_at,
        text: None,
        vector: None,
        traversal: Some(Traversal {
            start: vec![start.clone()],
            direction: Direction::Outgoing,
            max_hops: 2,
            max_edges: 32,
            work_budget: 128,
        }),
        limit: 32,
        cursor: None,
    }
}
fn check_sources(response: &QueryResponse, source: &ExternalReference) -> Result<()> {
    if response.hits.is_empty() {
        return Err("broad-validity synthetic neighborhood unexpectedly empty".into());
    }
    for hit in &response.hits {
        let GardenDraft::Assertion { evidence, .. } = &hit.record.draft else {
            return Err("query returned non-assertion".into());
        };
        if hit.sources != [source.clone()] || evidence != &[existing(source)] {
            return Err("query source canonical UUID/provenance mismatch".into());
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)] // Keep the measured public API calls explicit in this standalone runner.
fn reads(
    run: &mut Run,
    options: &Options,
    store: &GardenStore,
    auth: &Auth,
    entities: &[ExternalReference],
    source: &ExternalReference,
    known: &str,
    prefix: &str,
) -> Result<()> {
    for (name, count, budget) in [
        ("exact", options.samples * 2, 10.0),
        ("temporal_neighborhood", options.samples, 50.0),
        ("lexical", options.samples, 100.0),
    ] {
        let mut metric = Metric::new(budget);
        let warmups = if prefix == "warm" {
            if name == "exact" { 32 } else { 16 }
        } else {
            0
        };
        for sample in 0..count + warmups {
            run.check_deadline()?;
            let id = sample_index(u64::from(sample), entities.len());
            if name == "exact" {
                let request = get_request(entities[id].clone(), None);
                let start = Instant::now();
                let response = store.get(auth, &request)?;
                if sample >= warmups {
                    metric.record(start);
                }
                if response.record.metadata.id != entities[id].id {
                    return Err("exact ID mismatch".into());
                }
                std::hint::black_box(response);
            } else {
                let mut request = query(&entities[id], Some(known.into()));
                if name == "lexical" {
                    request.traversal = None;
                    request.kinds = vec![GardenKind::Entity];
                    request.predicate = None;
                    request.valid_at = None;
                    request.known_at = None;
                    request.text = Some(format!("entity {id:07}"));
                }
                let start = Instant::now();
                let response = store.query(auth, &request)?;
                if sample >= warmups {
                    metric.query(start, &response);
                }
                if name == "temporal_neighborhood" {
                    check_sources(&response, source)?;
                } else if sample >= warmups {
                    metric.lexical_target(
                        response
                            .hits
                            .iter()
                            .any(|hit| hit.record.metadata.id == entities[id].id),
                        &response.coverage,
                        response.page.next_cursor.is_some(),
                    );
                }
                std::hint::black_box(response);
            }
        }
        run.metrics.insert(format!("{prefix}_{name}"), metric);
    }
    Ok(())
}

fn writer_items(
    entities: &[ExternalReference],
    source: &ExternalReference,
    offset: usize,
) -> Vec<BatchItem> {
    (0..32)
        .map(|index| create(index, edge_draft(entities, source, offset + index, false)))
        .collect()
}

#[derive(Debug)]
struct WorkerOutcome {
    name: String,
    metric: Metric,
    committed: usize,
    fatal: Option<Value>,
}

fn contention(
    run: &mut Run,
    options: &Options,
    store: &GardenStore,
    auth: &Auth,
    entities: &[ExternalReference],
    source: &ExternalReference,
    known: &str,
) -> Result<()> {
    let barrier = Arc::new(Barrier::new(6));
    let samples = options.samples;
    let writes = options.write_samples.div_ceil(2);
    let deadline = run.started + run.deadline;
    // Reserve disjoint operation domains, retaining exact UUIDs for uncertain failure reconciliation.
    let results = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for worker in 0..6 {
            let barrier = barrier.clone();
            handles.push(scope.spawn(move || {
                let writer = worker >= 4;
                let mut metric = Metric::new(if writer { 100.0 } else { 50.0 });
                let mut committed = 0;
                let mut fatal = None;
                barrier.wait();
                for sample in 0..if writer { writes } else { samples } {
                    if Instant::now() >= deadline {
                        break;
                    }
                    if writer {
                        let request = BatchRequest {
                            version: 2,
                            workspace_id: WORKSPACE.into(),
                            generation: GENERATION.into(),
                            operation_id: synthetic_uuid(100 + worker, u64::from(sample)),
                            required_extensions: BTreeMap::new(),
                            items: writer_items(entities, source, sample as usize * 32),
                        };
                        let start = Instant::now();
                        match store.apply(auth, &request) {
                            Ok(_) => {
                                metric.record(start);
                                committed += 1;
                            }
                            Err(error) => {
                                metric.failure(start, &format!("{:?}",error.code));
                                fatal = Some(json!({"error":error.to_string(),"operation_id":request.operation_id,
                                    "outcome_requires_reconciliation":error.code == GardenErrorCode::StoreUnavailable}));
                                break;
                            }
                        }
                    } else {
                        let request = query(
                            &entities[sample_index(
                                u64::from(sample) + u64::from(worker),
                                entities.len(),
                            )],
                            Some(known.into()),
                        );
                        let start = Instant::now();
                        match store.query(auth, &request) {
                            Ok(response) => {
                                metric.query(start, &response);
                                if let Err(error) = check_sources(&response, source) {
                                    fatal = Some(json!({"error":error.to_string(),"sample":sample}));
                                    break;
                                }
                            }
                            Err(error) => {
                                metric.failure(start, &format!("{:?}",error.code));
                                if error.code != GardenErrorCode::RestartRequired {
                                    fatal = Some(json!({"error":error.to_string(),"sample":sample}));
                                    break;
                                }
                                // This sample failed availability. The next iteration is a new sample, not a retry.
                            }
                        }
                    }
                }
                WorkerOutcome {
                    name: format!(
                        "contention_{}_{}",
                        if writer { "writer" } else { "reader" },
                        worker
                    ),
                    metric,
                    committed,
                    fatal,
                }
            }));
        }
        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .map_err(|_| "contention worker panicked".into())
            })
            .collect::<Vec<Result<WorkerOutcome>>>()
    });
    let mut readers = Metric::new(50.0);
    let mut writers = Metric::new(100.0);
    let mut fatal_errors = Vec::new();
    for result in results {
        let WorkerOutcome {
            name,
            metric,
            committed,
            fatal,
        } = match result {
            Ok(outcome) => outcome,
            Err(error) => {
                fatal_errors.push(json!({"error":error.to_string(),"committed_progress_for_panicked_worker":"unknown"}));
                continue;
            }
        };
        if let Some(error) = fatal {
            fatal_errors.push(json!({"worker":name,"failure":error}));
        }
        run.receipts += committed;
        run.assertions += committed * 32;
        let combined = if name.contains("writer") {
            &mut writers
        } else {
            &mut readers
        };
        combined.merge(&metric);
        run.metrics.insert(name, metric);
    }
    run.metrics
        .insert("contention_reader_combined".into(), readers);
    run.metrics
        .insert("contention_writer_combined".into(), writers);
    run.checks.insert("contention".into(), json!({"readers":4,"writers":2,"barrier":true,
        "overlap":"scheduled concurrently; continuous overlap not guaranteed", "writer_acquisition_wait":"not exposed separately by public API",
        "restart_policy":"RestartRequired is a failed availability attempt with separately measured latency; no automatic retry",
        "fatal_worker_errors":fatal_errors}));
    if !fatal_errors.is_empty() {
        return Err(
            "contention worker failure; all available worker outcomes aggregated in report".into(),
        );
    }
    run.check_deadline()
}

fn execute(run: &mut Run, options: &Options, root: &Path) -> Result<()> {
    let path = root.join("synthetic-garden.redb");
    let store = GardenStore::initialize(&path, WORKSPACE, GENERATION)?;
    let auth = store.grant(
        "synthetic-benchmark",
        &["garden:read".into(), "garden:write".into()],
        &ClientAccess::default(),
        &GardenAccess::All {},
    )?;
    let outcome = run.apply(
        &store,
        &auth,
        vec![create(
            0,
            GardenDraft::Evidence {
                source: SourceLocator::Uri {
                    uri: "https://example.test/synthetic/seed14".into(),
                },
                excerpt: Some("Synthetic seed14 source, never a real conversation".into()),
                observed_time: observed(),
                extensions: BTreeMap::new(),
            },
        )],
    )?;
    let source = outcome.receipt.results[0].reference.clone();
    run.checks.insert("source".into(), json!({"canonical_evidence_id":source.id,
        "locator":"https://example.test/synthetic/seed14","assertion_evidence_ids":"exact canonical ID checked on query hits"}));
    let mut entities = Vec::with_capacity(options.entities);
    run.stage = "seed_entities".into();
    let load_start = Instant::now();
    while entities.len() < options.entities {
        let end = (entities.len() + MAX_BATCH_ITEMS).min(options.entities);
        let items = (entities.len()..end)
            .map(|index| create(index, entity(index)))
            .collect();
        let result = run.apply(&store, &auth, items)?;
        entities.extend(
            result
                .receipt
                .results
                .into_iter()
                .map(|item| item.reference),
        );
        run.progress(options, root)?;
        if entities.len().is_multiple_of(4096) {
            eprintln!(
                "seed entities {}/{} elapsed {:.1}s",
                entities.len(),
                options.entities,
                run.started.elapsed().as_secs_f64()
            );
        }
    }
    run.stage = "seed_relationships".into();
    let mut known = String::new();
    let mut first_assertion = None;
    while run.assertions < options.entities * 5 {
        let end = (run.assertions + MAX_BATCH_ITEMS).min(options.entities * 5);
        let items = (run.assertions..end)
            .map(|index| create(index, edge_draft(&entities, &source, index, false)))
            .collect();
        let result = run.apply(&store, &auth, items)?;
        if first_assertion.is_none() {
            first_assertion = Some(result.receipt.results[0].reference.clone());
        }
        known = result.receipt.committed_at.ok_or("missing committed_at")?;
        run.progress(options, root)?;
        if run.assertions.is_multiple_of(4096) {
            eprintln!(
                "seed assertions {}/{} elapsed {:.1}s",
                run.assertions,
                options.entities * 5,
                run.started.elapsed().as_secs_f64()
            );
        }
    }
    run.checks.insert(
        "initial_load".into(),
        json!({"milliseconds":load_start.elapsed().as_secs_f64()*1000.0,
        "entities":entities.len(),"assertions":run.assertions,"evidence":1,"known_at":known}),
    );
    run.stage = "reopened_reads".into();
    drop(store);
    let store = GardenStore::open(&path)?; // Trusted owned synthetic fixture, not a live/raw runtime open.
    reads(
        run,
        options,
        &store,
        &auth,
        &entities,
        &source,
        &known,
        "reopened_application_cache",
    )?;
    run.stage = "warm_reads".into();
    reads(
        run, options, &store, &auth, &entities, &source, &known, "warm",
    )?;
    run.progress(options, root)?;
    run.stage = "durable_32_assertions".into();
    let mut writes = Metric::new(100.0);
    for sample in 0..options.write_samples + 4 {
        run.check_deadline()?;
        let items = writer_items(&entities, &source, sample as usize * 32);
        let start = Instant::now();
        run.apply(&store, &auth, items)?;
        if sample >= 4 {
            writes.record(start);
        }
    }
    run.metrics.insert("durable_32_assertions".into(), writes);
    run.stage = "reader_writer_contention".into();
    contention(run, options, &store, &auth, &entities, &source, &known)?;
    run.progress(options, root)?;
    run.stage = "high_degree_seed".into();
    let before = run.assertions;
    while run.assertions - before < options.high_degree_edges as usize {
        let count =
            MAX_BATCH_ITEMS.min(options.high_degree_edges as usize - (run.assertions - before));
        let offset = run.assertions - before;
        let items = (0..count)
            .map(|index| create(index, edge_draft(&entities, &source, offset + index, true)))
            .collect();
        run.apply(&store, &auth, items)?;
        run.progress(options, root)?;
    }
    let first = first_assertion.ok_or("missing assertion")?;
    run.stage = "history_heavy_seed".into();
    for revision in 1..options.history_revisions {
        let mut draft = edge_draft(&entities, &source, 0, false);
        if let GardenDraft::Assertion { valid_time, .. } = &mut draft {
            valid_time.valid_from = Some(
                if revision % 2 == 0 {
                    "2020-01-01T00:00:00Z"
                } else {
                    "2026-01-01T00:00:00Z"
                }
                .into(),
            );
        }
        let item = BatchItem {
            item_key: "history".into(),
            expected_revision: u64::from(revision),
            mutation: Mutation::Update,
            id: Some(first.clone()),
            draft: Some(draft),
            replacement: None,
        };
        run.apply(&store, &auth, vec![item])?;
        run.progress(options, root)?;
    }
    run.stage = "high_degree_historical_queries".into();
    for (name, cutoff) in [
        ("high_degree_current", None),
        ("history_known_at", Some(known.clone())),
    ] {
        let mut metric = Metric::new(50.0);
        for _ in 0..options.samples {
            run.check_deadline()?;
            let request = query(&entities[0], cutoff.clone());
            let start = Instant::now();
            let response = store.query(&auth, &request)?;
            metric.query(start, &response);
            check_sources(&response, &source)?;
        }
        run.metrics.insert(name.into(), metric);
    }
    let historic = store.get(&auth, &get_request(first.clone(), Some(known.clone())))?;
    let valid = store
        .get_valid_at(&auth, &get_request(first.clone(), Some(known)), VALID_AT)?
        .is_some();
    let excluded = store
        .get_valid_at(&auth, &get_request(first, None), "2030-01-01T00:00:00Z")?
        .is_none();
    if historic.record.metadata.revision != 1 || !valid || !excluded {
        return Err("historic revision or half-open valid-time mismatch".into());
    }
    run.checks.insert("history".into(),json!({"revisions_on_one_assertion":options.history_revisions,
        "original_known_at_revision":historic.record.metadata.revision,"valid_at_included":valid,"half_open_end_excluded":excluded,
        "high_degree_added_edges":options.high_degree_edges}));
    run.stage = "canonical_index_audit".into();
    run.progress(options, root)?;
    run.check_deadline()?;
    let audit_start = Instant::now();
    let stats = store.audit(&auth)?;
    let current = (run.entities + run.assertions + 1) as u64;
    if stats.current_records != current
        || stats.history_records != current + run.history_updates as u64
        || stats.receipts != run.receipts as u64
    {
        return Err("canonical/index audit counts differ from committed generator counts".into());
    }
    run.checks.insert(
        "canonical_index_audit".into(),
        json!({"passed":true,"current_records":stats.current_records,
        "history_records":stats.history_records,"receipts":stats.receipts,"sequence":stats.sequence,
        "milliseconds":audit_start.elapsed().as_secs_f64()*1000.0}),
    );
    drop(store);
    run.stage = "complete".into();
    run.progress(options, root)
}

fn main() -> Result<()> {
    let options = Options::parse();
    let root = tempfile::Builder::new()
        .prefix("mother-garden-acceptance-synthetic-")
        .tempdir()?;
    eprintln!("owned synthetic output: {}", root.path().display());
    let mut run = Run::new(&options);
    let result = execute(&mut run, &options, root.path());
    let mut report = run.snapshot(&options);
    let bytes = std::fs::read_dir(root.path())?.try_fold(0_u64, |sum, entry| {
        Ok::<_, std::io::Error>(sum + entry?.metadata()?.len())
    })?;
    report["disk_bytes_owned_directory"] = json!(bytes);
    report["disk_bytes_database"] = json!(
        std::fs::metadata(root.path().join("synthetic-garden.redb"))
            .map(|metadata| metadata.len())
            .ok()
    );
    report["completed"] = json!(result.is_ok());
    report["latency_budgets_pass"] = if result.is_ok() {
        json!(
            run.metrics
                .values()
                .all(|metric| metric.summary()["p95_budget_pass"] == true)
        )
    } else {
        Value::Null
    };
    report["availability_pass"] = if result.is_ok() {
        json!(
            run.metrics
                .values()
                .all(|metric| metric.unsuccessful_milliseconds.is_empty())
        )
    } else {
        Value::Null
    };
    report["semantic_gates_pass"] = if result.is_ok() {
        json!(
            run.metrics
                .values()
                .all(|metric| metric.lexical_targets.iter().all(|target| target.found))
        )
    } else {
        Value::Null
    };
    report["workload_gates_pass"] = json!(
        result.is_ok()
            && report["latency_budgets_pass"] == true
            && report["availability_pass"] == true
            && report["semantic_gates_pass"] == true
    );
    report["scale_class"] = json!(if options.entities < 10000 {
        "smoke_only"
    } else if options.entities == 1000000 {
        "separate_stress"
    } else {
        "acceptance_scale"
    });
    report["failure"] = json!(result.as_ref().err().map(ToString::to_string));
    let retained = options.keep_database || report["workload_gates_pass"] != true;
    report["database_retained"] = json!(retained);
    if retained {
        report["owned_output_directory"] = json!(root.path());
    }
    std::fs::write(
        root.path().join("result.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    if retained {
        let _retained_path = root.keep();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_synthetic_consent_and_unbounded_samples() {
        assert!(Options::try_parse_from(["benchmark", "--entities", "10000"]).is_err());
        assert!(Options::try_parse_from(["benchmark", "--synthetic", "--samples", "0"]).is_err());
        assert!(
            Options::try_parse_from(["benchmark", "--synthetic", "--samples", "10001"]).is_err()
        );
        assert!(
            Options::try_parse_from(["benchmark", "--synthetic", "--entities", "9999"]).is_err()
        );
    }

    #[test]
    fn nearest_rank_retains_outliers_and_empty_samples_are_unavailable() {
        let mut metric = Metric::new(10.0);
        assert!(metric.summary()["p95_ms"].is_null());
        for sample in 1..=64 {
            metric.milliseconds.push(f64::from(sample));
        }
        let summary = metric.summary();
        assert_eq!(summary["p50_ms"], 32.0);
        assert_eq!(summary["p95_ms"], 61.0);
        assert_eq!(summary["p99_ms"], 64.0);
        assert_eq!(summary["p95_budget_pass"], false);
    }

    #[test]
    fn unsuccessful_requests_are_measured_without_becoming_successful_latency_samples() {
        let mut metric = Metric::new(50.0);
        metric.milliseconds.push(1.0);
        metric.failure(Instant::now(), "RestartRequired");
        let summary = metric.summary();
        assert_eq!(summary["samples"], 1);
        assert_eq!(summary["attempts"], 2);
        assert_eq!(
            summary["unsuccessful_requests_by_code"]["RestartRequired"],
            1
        );
        assert_eq!(summary["p95_ms"], 1.0);
        assert_eq!(summary["availability_pass"], false);
        assert_eq!(summary["p95_budget_pass"], true);
        let mut combined = Metric::new(50.0);
        combined.merge(&metric);
        combined.merge(&metric);
        assert_eq!(combined.summary()["attempts"], 4);
        assert_eq!(
            combined.summary()["unsuccessful_requests_by_code"]["RestartRequired"],
            2
        );
    }

    #[test]
    fn lexical_missing_targets_fail_recall_and_preserve_coverage_without_failing_execution() {
        let mut metric = Metric::new(100.0);
        let mut coverage = mother::garden::contracts::Coverage {
            complete: false,
            truncated: true,
            reason: Some("index_work_limit".into()),
            canonical_watermark: 1,
            index_watermark: 1,
            search_mode: mother::garden::contracts::SearchMode::Literal,
            model: None,
            profile: None,
        };
        metric.lexical_target(false, &coverage, false);
        coverage.complete = true;
        coverage.truncated = false;
        coverage.reason = None;
        metric.lexical_target(false, &coverage, false);
        metric.lexical_target(true, &coverage, false);
        let summary = metric.summary();
        assert_eq!(summary["lexical_recall"]["requested_targets"], 3);
        assert_eq!(summary["lexical_recall"]["found_targets"], 1);
        assert_eq!(summary["lexical_recall"]["missing_targets"], 2);
        assert_eq!(
            summary["lexical_recall"]["missing_on_complete_unpaginated_coverage"],
            1
        );
        assert_eq!(summary["lexical_recall"]["pass"], false);
        assert_eq!(
            summary["lexical_recall"]["samples"][0]["reason"],
            "index_work_limit"
        );
    }

    #[test]
    fn seeded_edges_have_exact_evidence_and_wrap_without_missing_endpoints() {
        let entities: Vec<_> = (0..7)
            .map(|i| reference(GardenKind::Entity, synthetic_uuid(1, i)))
            .collect();
        let source = reference(GardenKind::Evidence, synthetic_uuid(2, 0));
        for edge in 0..35 {
            let draft = edge_draft(&entities, &source, edge, false);
            draft.validate().unwrap();
            let GardenDraft::Assertion {
                subject,
                object,
                evidence,
                ..
            } = draft
            else {
                panic!("assertion expected")
            };
            assert_eq!(subject, existing(&entities[edge / 5]));
            assert!(matches!(object, AssertionObject::EntityRef { .. }));
            assert_eq!(evidence, vec![existing(&source)]);
        }
    }
}
