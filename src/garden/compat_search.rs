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
use std::{collections::HashMap, path::PathBuf, sync::Mutex};

const VECTORS: TableDefinition<(&str, &str), &[u8]> = TableDefinition::new("garden_v1_vectors_v1");
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
}
impl NativeSearch {
    /// Select explicit local assets without loading them.
    pub fn new(model_dir: PathBuf) -> Self {
        Self {
            model_dir: Some(model_dir),
            gate: Mutex::new(()),
            model: Mutex::new(None),
        }
    }
    /// Resolve the configured local model directory without accessing a provider or network.
    pub fn from_environment() -> Self {
        Self {
            model_dir: crate::embedding::model_directory().ok(),
            gate: Mutex::new(()),
            model: Mutex::new(None),
        }
    }
    /// Search with the pinned local model and a native derived cache.
    pub fn search(&self, store: &GardenStore, auth: &Auth, args: Value) -> Result<Value> {
        let query = Query::parse(args)?;
        let snapshot = capture(store, auth, &query)?;
        if query.mode == Mode::Literal {
            return Ok(snapshot.literal_response(&query, None));
        }
        let Ok(_gate) = self.gate.try_lock() else {
            return fallback(store, auth, &query, "model_busy");
        };
        let Ok(mut model) = self.model.try_lock() else {
            return fallback(store, auth, &query, "model_busy");
        };
        if model.is_none() {
            match self
                .model_dir
                .as_deref()
                .ok_or_else(|| Error::domain("MODEL_UNAVAILABLE", "Local model path unavailable"))
                .and_then(LocalEmbedder::load)
            {
                Ok(loaded) => *model = Some(loaded),
                Err(_) => return fallback(store, auth, &query, "model_unavailable"),
            }
        }
        let encoder = model
            .as_ref()
            .ok_or_else(|| Error::domain("MODEL_UNAVAILABLE", "Local model unavailable"))?;
        run(store, auth, &query, snapshot, encoder)
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
            return Ok(snapshot.literal_response(&query, None));
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
        records.push(Record {
            id: contracts::string(&value, "id")?.into(),
            revision: contracts::integer(&value, "revision", 1, 9007199254740991)?,
            digest: contracts::hash(&text),
            text,
            value,
            literal,
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
    if access.mode != crate::access::AccessMode::Shared {
        base["persona"] = persona;
    }
    Ok(Snapshot {
        access,
        records,
        base,
        fingerprint: contracts::hash(&fingerprint),
    })
}
fn fallback(store: &GardenStore, auth: &Auth, query: &Query, reason: &str) -> Result<Value> {
    Ok(capture(store, auth, query)?.literal_response(query, Some(reason)))
}
fn scope(snapshot: &Snapshot, auth: &Auth, profile: &EmbeddingProfile) -> Result<String> {
    Ok(contracts::hash(
        &(snapshot.base["workspace"].to_string()
            + &auth.client_id
            + &serde_json::to_string(&snapshot.access)?
            + &profile.fingerprint()),
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
    let scope = scope(&initial, auth, &profile)?;
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
            return Ok(snapshot.literal_response(query, Some("workspace_changed")));
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
