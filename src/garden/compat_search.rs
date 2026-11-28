//! Optional learned v1 retrieval over native canonical records and disposable redb vectors.
use super::{
    View, metadata, principal, recallable, recent, unavailable, unsettled, visible_records,
};
use crate::{
    contracts::{self, Auth},
    embedding::{EmbeddingProfile, LocalEmbedder},
    error::{Error, Result, invalid},
    garden::store::GardenStore,
    search::{
        self, Mode, Progress, Query, Record, Snapshot,
        cache::{Entry, Vector},
    },
};
use chrono::SecondsFormat;
use redb::{Durability, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    sync::{
        Mutex, MutexGuard,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

/// Derived vectors shared by every client: keyed by embedding profile and record ID and
/// valid only for the exact canonical revision and text digest they were computed from.
/// Visibility is enforced when a caller's authorized snapshot selects records, never by
/// the cache. Earlier per-client caches lived in `garden_v1_vectors_v1`.
const VECTORS: TableDefinition<(&str, &str), &[u8]> = TableDefinition::new("garden_v1_vectors_v2");
const LEGACY_VECTORS: TableDefinition<(&str, &str), &[u8]> =
    TableDefinition::new("garden_v1_vectors_v1");
/// Durable indexing progress and per-record failures; derived and rebuildable.
const INDEX_STATE: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_index_state");
/// A failed record is retried after this long, or at once when its revision changes.
const RETRY_FAILED_AFTER: chrono::Duration = chrono::Duration::hours(6);
const MODEL_WAIT: Duration = Duration::from_secs(5);
const MAX_RECORDS: usize = 8;
const MAX_PASSAGES: usize = 32;
const MAX_CACHED_PASSAGES: usize = 32_000;
const MAX_CACHE_BYTES: usize = 128 * 1024 * 1024;

/// One normalized vector with byte offsets into the canonical title-newline-content text.
#[derive(Debug, Clone)]
pub struct EncodedPassage {
    pub start: usize,
    pub end: usize,
    pub values: Vec<f32>,
}
/// Bounded encoder passages and truthful coverage of the canonical record text.
#[derive(Debug, Clone)]
pub struct EncodedPassages {
    pub passages: Vec<EncodedPassage>,
    pub partial: bool,
}
/// Trusted local encoder installed by the host; requests never select models or providers.
pub trait Encoder {
    fn profile(&self) -> EmbeddingProfile;
    fn query(&self, text: &str) -> Result<Vec<f32>>;
    fn passages(&self, text: &str) -> Result<EncodedPassages>;
}
impl Encoder for LocalEmbedder {
    fn profile(&self) -> EmbeddingProfile {
        crate::embedding::profile()
    }
    fn query(&self, text: &str) -> Result<Vec<f32>> {
        LocalEmbedder::query(self, text)
    }
    fn passages(&self, text: &str) -> Result<EncodedPassages> {
        let plan = LocalEmbedder::passages(self, text)?;
        Ok(EncodedPassages {
            partial: plan.partial,
            passages: plan
                .passages
                .iter()
                .map(|p| {
                    Ok(EncodedPassage {
                        start: p.start,
                        end: p.end,
                        values: self.embed(p)?,
                    })
                })
                .collect::<Result<_>>()?,
        })
    }
}
/// Reusable optional local model; loading verifies installed assets and never downloads anything.
#[derive(Debug)]
pub struct NativeSearch {
    model_dir: Option<PathBuf>,
    gate: Mutex<()>,
    model: Mutex<Option<LocalEmbedder>>,
    /// Set while an owner service runs the background indexer.
    background: AtomicBool,
    /// Last model load failure, reported by index status without retrying the load.
    model_error: Mutex<Option<String>>,
}
impl NativeSearch {
    /// Select explicit local assets without loading them.
    pub fn new(model_dir: PathBuf) -> Self {
        Self {
            model_dir: Some(model_dir),
            gate: Mutex::new(()),
            model: Mutex::new(None),
            background: AtomicBool::new(false),
            model_error: Mutex::new(None),
        }
    }
    /// Resolve the configured local model directory without accessing a provider or network.
    pub fn from_environment() -> Self {
        Self {
            model_dir: crate::embedding::model_directory().ok(),
            gate: Mutex::new(()),
            model: Mutex::new(None),
            background: AtomicBool::new(false),
            model_error: Mutex::new(None),
        }
    }
    /// Search with the pinned local model and a native derived cache.
    pub fn search(&self, store: &GardenStore, auth: &Auth, args: Value) -> Result<Value> {
        let query = Query::parse(args)?;
        let snapshot = capture(store, auth, &query)?;
        if query.mode == Mode::Literal {
            return snapshot.literal_response(&query, None);
        }
        let Ok(_gate) = self.gate.try_lock() else {
            return fallback(store, auth, &query, "model_busy");
        };
        // The background indexer releases the model between records, so wait briefly.
        let Some(mut model) = self.wait_model(MODEL_WAIT) else {
            return fallback(store, auth, &query, "model_busy");
        };
        if self.load(&mut model).is_err() {
            return fallback(store, auth, &query, "model_unavailable");
        }
        let encoder = model
            .as_ref()
            .ok_or_else(|| Error::domain("MODEL_UNAVAILABLE", "Local model unavailable"))?;
        run(store, auth, &query, snapshot, encoder)
    }
    fn wait_model(&self, wait: Duration) -> Option<MutexGuard<'_, Option<LocalEmbedder>>> {
        let started = Instant::now();
        loop {
            match self.model.try_lock() {
                Ok(guard) => return Some(guard),
                Err(std::sync::TryLockError::Poisoned(poisoned)) => {
                    return Some(poisoned.into_inner());
                }
                Err(std::sync::TryLockError::WouldBlock) if started.elapsed() < wait => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(std::sync::TryLockError::WouldBlock) => return None,
            }
        }
    }
    /// Load the pinned local model from installed assets only; never downloads.
    fn load(&self, model: &mut Option<LocalEmbedder>) -> Result<()> {
        if model.is_some() {
            return Ok(());
        }
        let loaded = self
            .model_dir
            .as_deref()
            .ok_or_else(|| {
                Error::domain(
                    "MODEL_UNAVAILABLE",
                    "No local embedding model directory is configured",
                )
            })
            .and_then(LocalEmbedder::load);
        let mut error = self.model_error.lock().unwrap_or_else(|e| e.into_inner());
        match loaded {
            Ok(loaded) => {
                *model = Some(loaded);
                *error = None;
                Ok(())
            }
            Err(e) => {
                *error = Some(e.to_string());
                Err(e)
            }
        }
    }
    /// Mark whether an owner service is running the background indexer.
    pub fn set_background(&self, running: bool) {
        self.background.store(running, Ordering::Release);
    }
    /// Index at most one pending record with the installed local model, releasing the
    /// model afterwards so searches can use it. Returns `None` when the model is busy.
    /// # Errors
    /// `MODEL_UNAVAILABLE` when no verified local model is installed; storage failures.
    pub fn index_next(&self, store: &GardenStore) -> Result<Option<IndexReport>> {
        let Some(mut model) = self.wait_model(Duration::from_millis(200)) else {
            return Ok(None);
        };
        self.load(&mut model)?;
        let encoder = model
            .as_ref()
            .ok_or_else(|| Error::domain("MODEL_UNAVAILABLE", "Local model unavailable"))?;
        index_with(store, encoder, 1).map(Some)
    }
    /// Index status over the records `auth` may read (all eligible records for `None`).
    /// # Errors
    /// Authorization and storage failures.
    pub fn index_status(&self, store: &GardenStore, auth: Option<&Auth>) -> Result<Value> {
        let mut status = index_status(store, auth, &crate::embedding::profile())?;
        let loaded = self.model.try_lock().map(|m| m.is_some()).unwrap_or(true);
        let error = self
            .model_error
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        status["model"]["state"] = json!(if loaded {
            "loaded"
        } else if self.model_dir.is_none() {
            "not_configured"
        } else if error.is_some() {
            "unavailable"
        } else {
            "not_loaded"
        });
        status["model"]["error"] = json!(error);
        status["backgroundIndexing"] = json!(self.background.load(Ordering::Acquire));
        if !loaded && (self.model_dir.is_none() || error.is_some()) {
            status["limitation"] = json!(
                "No verified local embedding model is installed, so semantic ranking is unavailable. Literal search still covers every authorized record. Mother never downloads models or sends memory content anywhere."
            );
        }
        Ok(status)
    }
    /// Search through an explicitly installed trusted encoder, including synthetic qualification adapters.
    pub fn search_with(
        &self,
        store: &GardenStore,
        auth: &Auth,
        args: Value,
        encoder: &dyn Encoder,
    ) -> Result<Value> {
        let query = Query::parse(args)?;
        let snapshot = capture(store, auth, &query)?;
        if query.mode == Mode::Literal {
            return snapshot.literal_response(&query, None);
        }
        let Ok(_gate) = self.gate.try_lock() else {
            return fallback(store, auth, &query, "model_busy");
        };
        run(store, auth, &query, snapshot, encoder)
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cached {
    revision: u64,
    digest: String,
    entry: Entry,
}

fn capture(store: &GardenStore, auth: &Auth, query: &Query) -> Result<Snapshot> {
    let tx = store.db.begin_read().map_err(unavailable)?;
    capture_in(&tx, store, auth, query)
}
fn capture_in(tx: &impl View, store: &GardenStore, auth: &Auth, query: &Query) -> Result<Snapshot> {
    let workspace = metadata(tx, "workspace")?;
    let access = principal(tx, auth, "read")?.access;
    let persona = if access.mode == crate::access::AccessMode::Shared {
        Value::Null
    } else {
        metadata(tx, "persona")?
    };
    let mut fingerprint =
        workspace.to_string() + &persona.to_string() + &serde_json::to_string(&access)?;
    // Equal grants can see the same ranking. A budgeted continuation still
    // belongs to the client whose previous page ended at that position.
    if query.args.get("maxResponseBytes").is_some() {
        fingerprint.push_str(&serde_json::to_string(&auth.client_id)?);
    }
    let settings = super::settings::Settings::load(tx)?;
    let query_name = super::settings::words(&query.text);
    let names = settings.names(&query_name);
    fingerprint.push_str(&serde_json::to_string(&names)?);
    let mut records = Vec::new();
    for value in visible_records(tx, &access)?.into_iter().filter(recallable) {
        fingerprint.push_str(&value.to_string());
        let text = format!(
            "{}\n{}",
            contracts::string(&value, "title")?,
            contracts::string(&value, "content")?
        );
        let literal = format!(
            "{} {}",
            contracts::string(&value, "title")?,
            contracts::string(&value, "content")?
        )
        .to_ascii_lowercase()
        .contains(&query.text.to_ascii_lowercase());
        let alias = (!literal && names.len() > 1)
            .then(|| super::settings::project_match(&value, &names))
            .flatten()
            .map(str::to_owned);
        records.push(Record {
            id: contracts::string(&value, "id")?.into(),
            revision: contracts::integer(&value, "revision", 1, 9007199254740991)?,
            digest: contracts::hash(&text),
            text,
            value,
            literal: literal || alias.is_some(),
            alias,
        });
    }
    let mut work = records
        .iter()
        .filter(|r| unsettled(&r.value))
        .map(|r| r.value.clone())
        .collect::<Vec<_>>();
    work.sort_by(recent);
    let truncated = work.len() > 20;
    work.truncate(20);
    let work = work
        .iter()
        .map(|r| json!({"id":r["id"],"kind":r["kind"],"status":r["status"]}))
        .collect::<Vec<_>>();
    let mut base = json!({"workspace":workspace,"access":access.metadata(),"capturedAt":store.clock.now().to_rfc3339_opts(SecondsFormat::Millis,true),"records":[],"omitted":0,"unsettledWork":work,"unsettledWorkTruncated":truncated,"note":"Excerpts and similarity scores are navigation, not evidence of truth. Resolve full records before editing or acting. A handoff does not pause running jobs."});
    if access.mode != crate::access::AccessMode::Shared && query.view.include_persona {
        base["persona"] = persona;
        base["contextSections"] = json!({"persona":"included","pinned":"not_applicable"});
    } else if access.mode != crate::access::AccessMode::Shared {
        base["contextSections"] = json!({"persona":"excluded","pinned":"not_applicable"});
    }
    Ok(Snapshot {
        access,
        records,
        base,
        fingerprint: contracts::hash(&fingerprint),
    })
}
fn fallback(store: &GardenStore, auth: &Auth, query: &Query, reason: &str) -> Result<Value> {
    capture(store, auth, query)?.literal_response(query, Some(reason))
}
/// One derived index per embedding profile, shared by all clients. Vectors are a pure
/// function of the canonical text and never select which records a caller may see.
fn scope(profile: &EmbeddingProfile) -> String {
    contracts::hash(&format!(
        "garden-v1-shared-vectors|{}",
        profile.fingerprint()
    ))
}
fn validate_profile(profile: &EmbeddingProfile) -> Result<()> {
    if profile.version != 1
        || profile.dimensions == 0
        || profile.dimensions > 4096
        || profile.model.trim().is_empty()
        || profile.model.len() > 200
        || profile.profile.trim().is_empty()
        || profile.profile.len() > 200
    {
        return Err(invalid("Invalid native embedding profile"));
    }
    Ok(())
}
fn validate_vector(values: &[f32], dimensions: usize) -> Result<()> {
    let norm: f64 = values.iter().map(|v| f64::from(*v).powi(2)).sum();
    if values.len() != dimensions
        || values.iter().any(|v| !v.is_finite())
        || !(0.98..=1.02).contains(&norm)
    {
        return Err(Error::domain(
            "MODEL_UNAVAILABLE",
            "Invalid normalized local embedding",
        ));
    }
    Ok(())
}
fn validate_entry(entry: &Entry, text: &str, dimensions: usize) -> Result<()> {
    if entry.vectors.len() > 32 {
        return Err(invalid("Too many native embedding passages"));
    }
    for vector in &entry.vectors {
        if vector.start >= vector.end
            || vector.end > text.len()
            || !text.is_char_boundary(vector.start)
            || !text.is_char_boundary(vector.end)
        {
            return Err(invalid("Invalid native embedding span"));
        }
        validate_vector(&vector.values, dimensions)?;
    }
    Ok(())
}
fn load_cached(
    store: &GardenStore,
    scope: &str,
    snapshot: &Snapshot,
    profile: &EmbeddingProfile,
) -> Result<HashMap<String, Entry>> {
    let tx = store.db.begin_read().map_err(unavailable)?;
    let table = match tx.open_table(VECTORS) {
        Ok(t) => t,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(HashMap::new()),
        Err(e) => return Err(unavailable(e)),
    };
    let mut cached = HashMap::new();
    for r in &snapshot.records {
        if let Some(bytes) = table.get((scope, r.id.as_str())).map_err(unavailable)? {
            let row: Cached = serde_json::from_slice(bytes.value())?;
            if row.revision == r.revision && row.digest == r.digest {
                validate_entry(&row.entry, &r.text, profile.dimensions)?;
                cached.insert(r.id.clone(), row.entry);
            }
        }
    }
    Ok(cached)
}
fn run(
    store: &GardenStore,
    auth: &Auth,
    query: &Query,
    initial: Snapshot,
    encoder: &dyn Encoder,
) -> Result<Value> {
    let profile = encoder.profile();
    if validate_profile(&profile).is_err() {
        return fallback(store, auth, query, "embedding_failed");
    }
    let scope = scope(&profile);
    let mut cached = match load_cached(store, &scope, &initial, &profile) {
        Ok(cached) => cached,
        Err(_) => return fallback(store, auth, query, "cache_unavailable"),
    };
    let vector = match encoder.query(&query.text).and_then(|v| {
        validate_vector(&v, profile.dimensions)?;
        Ok(v)
    }) {
        Ok(v) => v,
        Err(_) => return fallback(store, auth, query, "embedding_failed"),
    };
    let mut staged = Vec::new();
    let mut progress = Progress::default();
    for r in &initial.records {
        if cached.contains_key(&r.id) {
            continue;
        }
        if progress.new_records >= MAX_RECORDS {
            break;
        }
        let plan = match encoder.passages(&r.text) {
            Ok(plan) => plan,
            Err(_) => return fallback(store, auth, query, "embedding_failed"),
        };
        if plan.passages.len() > 32 {
            return fallback(store, auth, query, "embedding_failed");
        }
        if progress.new_passages + plan.passages.len() > MAX_PASSAGES {
            break;
        }
        let entry = Entry {
            partial: plan.partial,
            vectors: plan
                .passages
                .into_iter()
                .map(|v| Vector {
                    start: v.start,
                    end: v.end,
                    values: v.values,
                })
                .collect(),
        };
        if validate_entry(&entry, &r.text, profile.dimensions).is_err() {
            return fallback(store, auth, query, "embedding_failed");
        }
        progress.new_records += 1;
        progress.new_passages += entry.vectors.len();
        staged.push((
            r.id.clone(),
            Cached {
                revision: r.revision,
                digest: r.digest.clone(),
                entry: entry.clone(),
            },
        ));
        cached.insert(r.id.clone(), entry);
    }
    // Recheck identity, grant, visibility and canonical revisions under the same writer as derived publication.
    let publication = (|| -> Result<(Snapshot, bool)> {
        let mut tx = store.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        let current = capture_in(&tx, store, auth, query)?;
        if current.fingerprint != initial.fingerprint {
            return Ok((current, true));
        }
        let mut table = tx.open_table(VECTORS).map_err(unavailable)?;
        let mut bytes_used = 0usize;
        let mut passages_used = 0usize;
        for row in table.iter().map_err(unavailable)? {
            let (_, bytes) = row.map_err(unavailable)?;
            bytes_used = bytes_used.saturating_add(bytes.value().len());
            let row: Cached = serde_json::from_slice(bytes.value())?;
            passages_used = passages_used.saturating_add(row.entry.vectors.len());
        }
        for (id, row) in staged {
            let bytes = serde_json::to_vec(&row)?;
            let old = table
                .get((scope.as_str(), id.as_str()))
                .map_err(unavailable)?
                .map(|bytes| {
                    let old: Cached = serde_json::from_slice(bytes.value())?;
                    Ok::<_, Error>((bytes.value().len(), old.entry.vectors.len()))
                })
                .transpose()?
                .unwrap_or_default();
            let next_bytes = bytes_used.saturating_sub(old.0).saturating_add(bytes.len());
            let next_passages = passages_used
                .saturating_sub(old.1)
                .saturating_add(row.entry.vectors.len());
            if next_bytes > MAX_CACHE_BYTES || next_passages > MAX_CACHED_PASSAGES {
                progress.capacity = true;
                progress.new_records -= 1;
                progress.new_passages -= row.entry.vectors.len();
                cached.remove(&id);
                continue;
            }
            table
                .insert((scope.as_str(), id.as_str()), bytes.as_slice())
                .map_err(unavailable)?;
            bytes_used = next_bytes;
            passages_used = next_passages;
        }
        drop(table);
        tx.commit().map_err(unavailable)?;
        Ok((current, false))
    })();
    let current = match publication {
        Ok((snapshot, true)) => {
            return snapshot.literal_response(query, Some("workspace_changed"));
        }
        Ok((snapshot, false)) => snapshot,
        Err(e) if matches!(e.code(), "UNAUTHORIZED" | "FORBIDDEN") => return Err(e),
        Err(_) => return fallback(store, auth, query, "cache_unavailable"),
    };
    let mut response =
        search::semantic_response_validated(current, query, &cached, &vector, &progress)?;
    if response["search"]["semanticUsed"] == true {
        response["search"]["model"] = json!(if profile == crate::embedding::profile() {
            crate::embedding::MODEL_ID.to_owned()
        } else {
            profile.model
        });
        response["search"]["profile"] = json!(profile.profile);
    }
    Ok(response)
}

/// What one background indexing step did.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexReport {
    pub indexed: usize,
    pub failed: usize,
    pub skipped_changed: usize,
    pub pending: usize,
    pub pruned: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IndexState {
    #[serde(default)]
    last_run_at: Option<String>,
    #[serde(default)]
    last_success_at: Option<String>,
    #[serde(default)]
    last_error: Option<String>,
    #[serde(default)]
    failures: BTreeMap<String, Failure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Failure {
    revision: u64,
    digest: String,
    error: String,
    at: String,
}

struct Eligible {
    id: String,
    revision: u64,
    digest: String,
    text: String,
    updated_at: String,
}

fn eligible_record(value: &Value) -> Result<Eligible> {
    let text = format!(
        "{}\n{}",
        contracts::string(value, "title")?,
        contracts::string(value, "content")?
    );
    Ok(Eligible {
        id: contracts::string(value, "id")?.into(),
        revision: contracts::integer(value, "revision", 1, 9007199254740991)?,
        digest: contracts::hash(&text),
        text,
        updated_at: value["updatedAt"].as_str().unwrap_or_default().into(),
    })
}

fn eligible(tx: &impl View) -> Result<Vec<Eligible>> {
    let settings = super::settings::Settings::load(tx)?;
    tx.values(super::RECORDS)?
        .iter()
        .filter(|r| recallable(r) && !settings.hidden(r["id"].as_str().unwrap_or_default()))
        .map(eligible_record)
        .collect()
}

fn read_state(tx: &redb::ReadTransaction) -> Result<IndexState> {
    let table = match tx.open_table(INDEX_STATE) {
        Ok(table) => table,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(IndexState::default()),
        Err(e) => return Err(unavailable(e)),
    };
    match table.get("status").map_err(unavailable)? {
        Some(row) => Ok(serde_json::from_slice(row.value())?),
        None => Ok(IndexState::default()),
    }
}

fn write_state(tx: &redb::WriteTransaction, state: &IndexState) -> Result<()> {
    let mut table = tx.open_table(INDEX_STATE).map_err(unavailable)?;
    table
        .insert("status", serde_json::to_vec(state)?.as_slice())
        .map_err(unavailable)?;
    Ok(())
}

fn cached_rows(tx: &redb::ReadTransaction, scope: &str) -> Result<HashMap<String, Cached>> {
    let table = match tx.open_table(VECTORS) {
        Ok(t) => t,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(HashMap::new()),
        Err(e) => return Err(unavailable(e)),
    };
    let mut rows = HashMap::new();
    for row in table.range((scope, "")..).map_err(unavailable)? {
        let (key, value) = row.map_err(unavailable)?;
        let (row_scope, id) = key.value();
        if row_scope != scope {
            break;
        }
        rows.insert(id.to_owned(), serde_json::from_slice(value.value())?);
    }
    Ok(rows)
}

fn retry_due(failure: &Failure, record: &Eligible, now: chrono::DateTime<chrono::Utc>) -> bool {
    failure.revision != record.revision
        || failure.digest != record.digest
        || chrono::DateTime::parse_from_rfc3339(&failure.at)
            .map(|at| now.signed_duration_since(at) >= RETRY_FAILED_AFTER)
            .unwrap_or(true)
}

/// Remove vectors for retracted, superseded, deleted or revised records and for other
/// embedding profiles. Derived data only; canonical records are never touched.
fn prune(store: &GardenStore, scope: &str, keep: &HashMap<String, (u64, String)>) -> Result<usize> {
    let tx = store.db.begin_write().map_err(unavailable)?;
    let removed;
    {
        let mut table = tx.open_table(VECTORS).map_err(unavailable)?;
        let mut stale = Vec::new();
        for row in table.iter().map_err(unavailable)? {
            let (key, value) = row.map_err(unavailable)?;
            let (row_scope, id) = key.value();
            let current = row_scope == scope
                && keep.get(id).is_some_and(|(revision, digest)| {
                    serde_json::from_slice::<Cached>(value.value())
                        .is_ok_and(|c| c.revision == *revision && c.digest == *digest)
                });
            if !current {
                stale.push((row_scope.to_owned(), id.to_owned()));
            }
        }
        for (row_scope, id) in &stale {
            table
                .remove((row_scope.as_str(), id.as_str()))
                .map_err(unavailable)?;
        }
        removed = stale.len();
    }
    if removed > 0 {
        tx.commit().map_err(unavailable)?;
    } else {
        tx.abort().map_err(unavailable)?;
    }
    Ok(removed)
}

/// One durable, incremental indexing step through a trusted local encoder: prune stale
/// vectors, then embed up to `max_records` pending records (newest first). Each vector
/// is published only if the record's revision and text are unchanged at commit time.
/// # Errors
/// Storage failures; encoder failures are recorded per record and retried later.
pub fn index_with(
    store: &GardenStore,
    encoder: &dyn Encoder,
    max_records: usize,
) -> Result<IndexReport> {
    let profile = encoder.profile();
    validate_profile(&profile)?;
    let scope = scope(&profile);
    let now = store.clock.now();
    let (records, rows, mut state) = {
        let tx = store.db.begin_read().map_err(unavailable)?;
        (eligible(&tx)?, cached_rows(&tx, &scope)?, read_state(&tx)?)
    };
    let mut report = IndexReport::default();
    let keep: HashMap<String, (u64, String)> = records
        .iter()
        .map(|r| (r.id.clone(), (r.revision, r.digest.clone())))
        .collect();
    let stale_rows = rows
        .iter()
        .any(|(id, row)| keep.get(id) != Some(&(row.revision, row.digest.clone())));
    if stale_rows || rows.len() > keep.len() {
        report.pruned = prune(store, &scope, &keep)?;
    }
    state.failures.retain(|id, f| {
        keep.get(id)
            .is_some_and(|(revision, digest)| *revision == f.revision && *digest == f.digest)
    });
    let mut pending: Vec<&Eligible> = records
        .iter()
        .filter(|r| {
            rows.get(&r.id)
                .is_none_or(|row| row.revision != r.revision || row.digest != r.digest)
        })
        .filter(|r| {
            state
                .failures
                .get(&r.id)
                .is_none_or(|f| retry_due(f, r, now))
        })
        .collect();
    pending.sort_by(|a, b| b.updated_at.cmp(&a.updated_at).then(a.id.cmp(&b.id)));
    for record in pending.iter().take(max_records) {
        let entry = encoder.passages(&record.text).and_then(|plan| {
            if plan.passages.len() > MAX_PASSAGES {
                return Err(invalid("Too many native embedding passages"));
            }
            let entry = Entry {
                partial: plan.partial,
                vectors: plan
                    .passages
                    .into_iter()
                    .map(|v| Vector {
                        start: v.start,
                        end: v.end,
                        values: v.values,
                    })
                    .collect(),
            };
            validate_entry(&entry, &record.text, profile.dimensions)?;
            Ok(entry)
        });
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                report.failed += 1;
                state.failures.insert(
                    record.id.clone(),
                    Failure {
                        revision: record.revision,
                        digest: record.digest.clone(),
                        error: error.code().to_owned(),
                        at: now.to_rfc3339_opts(SecondsFormat::Secs, true),
                    },
                );
                state.last_error = Some(format!("{}: {}", error.code(), error));
                continue;
            }
        };
        let row = Cached {
            revision: record.revision,
            digest: record.digest.clone(),
            entry,
        };
        let mut tx = store.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        // Publish only for the exact canonical revision and text that were embedded.
        let settings = super::settings::Settings::load(&tx)?;
        let current = tx
            .get(super::RECORDS, &record.id)?
            .filter(|v| recallable(v) && !settings.hidden(&record.id))
            .map(|v| eligible_record(&v))
            .transpose()?;
        if current.is_none_or(|c| c.revision != record.revision || c.digest != record.digest) {
            report.skipped_changed += 1;
            tx.abort().map_err(unavailable)?;
            continue;
        }
        {
            let mut table = tx.open_table(VECTORS).map_err(unavailable)?;
            let bytes = serde_json::to_vec(&row)?;
            if bytes.len() > MAX_CACHE_BYTES / 64 {
                return Err(invalid("Native embedding entry is too large"));
            }
            table
                .insert((scope.as_str(), record.id.as_str()), bytes.as_slice())
                .map_err(unavailable)?;
        }
        state.failures.remove(&record.id);
        tx.commit().map_err(unavailable)?;
        report.indexed += 1;
    }
    report.pending = pending.len().saturating_sub(report.indexed + report.failed);
    state.last_run_at = Some(now.to_rfc3339_opts(SecondsFormat::Secs, true));
    if report.indexed > 0 || (report.pending == 0 && report.failed == 0) {
        state.last_success_at = state.last_run_at.clone();
        if report.failed == 0 {
            state.last_error = None;
        }
    }
    let tx = store.db.begin_write().map_err(unavailable)?;
    write_state(&tx, &state)?;
    tx.commit().map_err(unavailable)?;
    Ok(report)
}

/// Read-only indexing status, scoped to what `auth` may read when given.
/// # Errors
/// Authorization and storage failures.
pub fn index_status(
    store: &GardenStore,
    auth: Option<&Auth>,
    profile: &EmbeddingProfile,
) -> Result<Value> {
    let scope = scope(profile);
    let tx = store.db.begin_read().map_err(unavailable)?;
    let access = auth
        .map(|auth| principal(&tx, auth, "read").map(|p| p.access))
        .transpose()?;
    let records: Vec<Eligible> = eligible(&tx)?
        .into_iter()
        .filter(|r| access.as_ref().is_none_or(|a| a.allows(&r.id)))
        .collect();
    let rows = cached_rows(&tx, &scope)?;
    let state = read_state(&tx)?;
    let (mut indexed, mut partial, mut unembeddable, mut failed, mut pending) = (0, 0, 0, 0, 0);
    let mut failed_ids = Vec::new();
    for r in &records {
        match rows.get(&r.id) {
            Some(row) if row.revision == r.revision && row.digest == r.digest => {
                if row.entry.vectors.is_empty() {
                    unembeddable += 1;
                } else if row.entry.partial {
                    partial += 1;
                } else {
                    indexed += 1;
                }
            }
            _ => match state.failures.get(&r.id) {
                Some(f) if f.revision == r.revision && f.digest == r.digest => {
                    failed += 1;
                    if failed_ids.len() < 10 {
                        failed_ids.push(
                            json!({"id":r.id,"revision":r.revision,"error":f.error,"at":f.at}),
                        );
                    }
                }
                _ => pending += 1,
            },
        }
    }
    let eligible = records.len();
    let done = indexed + partial + unembeddable;
    let stale: usize = rows
        .iter()
        .filter(|(id, row)| {
            !records
                .iter()
                .any(|r| r.id == **id && r.revision == row.revision && r.digest == row.digest)
        })
        .count();
    Ok(json!({
        "eligibleRecords": eligible,
        "indexedRecords": indexed,
        "partiallyIndexedRecords": partial,
        "unembeddableRecords": unembeddable,
        "pendingRecords": pending,
        "failedRecords": failed,
        "failures": failed_ids,
        "staleVectors": if access.is_none() {json!(stale)} else {Value::Null},
        "progress": if eligible == 0 {1.0} else {done as f64 / eligible as f64},
        "complete": pending == 0 && failed == 0,
        "lastRunAt": state.last_run_at,
        "lastSuccessAt": state.last_success_at,
        "lastError": state.last_error,
        "model": {"id": if *profile == crate::embedding::profile() {crate::embedding::MODEL_ID.to_owned()} else {profile.model.clone()},
            "profile": profile.profile, "dimensions": profile.dimensions},
        "scope": if access.is_some() {"authorized_records"} else {"all_recallable_records"},
        "localOnly": true,
    }))
}

/// Owner repair: discard every derived vector and indexing record so the next indexing
/// pass rebuilds from canonical records. Canonical records are never modified.
/// # Errors
/// Storage failures.
pub fn reset_index(store: &GardenStore) -> Result<Value> {
    let tx = store.db.begin_write().map_err(unavailable)?;
    let mut dropped = Vec::new();
    for (name, table) in [
        ("garden_v1_vectors_v2", VECTORS),
        ("garden_v1_vectors_v1", LEGACY_VECTORS),
    ] {
        if tx.delete_table(table).map_err(unavailable)? {
            dropped.push(name);
        }
    }
    if tx.delete_table(INDEX_STATE).map_err(unavailable)? {
        dropped.push("garden_v1_index_state");
    }
    tx.commit().map_err(unavailable)?;
    Ok(json!({"reset": true, "droppedDerivedTables": dropped,
        "note": "Derived semantic vectors were discarded. The owner service rebuilds them in the background from canonical records."}))
}
