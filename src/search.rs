//! Local semantic retrieval over authorized canonical records, with truthful literal fallback.
mod cache;
use crate::{
    access::{self, AccessMode, ClientAccess},
    contracts::{self, Auth, Kind},
    embedding::{LocalEmbedder, MODEL_ID, MODEL_REVISION, PROFILE},
    error::{Error, Result, invalid},
    store::{self, Store},
};
use cache::{Cache, Entry, Vector};
use serde_json::{Value, json};
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
const MIN_SIMILARITY: f32 = 0.35;
const QUERY_RECORDS: usize = 8;
const QUERY_PASSAGES: usize = 32;
const INDEX_PASSAGES: usize = 1024;

/// Reuses one local model; concurrent searches fall back instead of blocking writes.
pub struct Search {
    model_dir: Option<PathBuf>,
    model: Mutex<Option<LocalEmbedder>>,
}
impl std::fmt::Debug for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Search")
            .field("model_dir", &self.model_dir)
            .finish_non_exhaustive()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Hybrid,
    Semantic,
    Literal,
}
impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Hybrid => "hybrid",
            Self::Semantic => "semantic",
            Self::Literal => "literal",
        }
    }
}
#[derive(Debug)]
struct Query {
    text: String,
    kind: Option<String>,
    limit: usize,
    mode: Mode,
    updated_since: Option<String>,
    updated_before: Option<String>,
}
impl Query {
    fn parse(v: Value) -> Result<Self> {
        contracts::strict(
            &v,
            &[
                "query",
                "kind",
                "limit",
                "mode",
                "updatedSince",
                "updatedBefore",
            ],
        )?;
        let (updated_since, updated_before) = contracts::time_window(&v)?;
        let text = contracts::normalize_search_query(contracts::string(&v, "query")?)?;
        let kind = v
            .get("kind")
            .map(|_| contracts::string(&v, "kind").map(str::to_owned))
            .transpose()?;
        if let Some(k) = &kind {
            Kind::parse(k)?;
        }
        let limit = if v.get("limit").is_some() {
            contracts::integer(&v, "limit", 1, 100)? as usize
        } else {
            20
        };
        let mode = match v
            .get("mode")
            .map(|_| contracts::string(&v, "mode"))
            .transpose()?
            .unwrap_or("hybrid")
        {
            "hybrid" => Mode::Hybrid,
            "semantic" => Mode::Semantic,
            "literal" => Mode::Literal,
            _ => return Err(invalid("Unknown search mode")),
        };
        Ok(Self {
            text,
            kind,
            limit,
            mode,
            updated_since,
            updated_before,
        })
    }
    fn includes(&self, r: &Record) -> bool {
        // Stored and bound times share one fixed-width UTC form, so text order is time order.
        let updated = r.value["updatedAt"].as_str().unwrap_or_default();
        self.kind
            .as_ref()
            .is_none_or(|k| r.value["kind"].as_str() == Some(k))
            && self
                .updated_since
                .as_deref()
                .is_none_or(|since| updated >= since)
            && self
                .updated_before
                .as_deref()
                .is_none_or(|before| updated < before)
    }
}
#[derive(Debug)]
struct Record {
    id: String,
    revision: u64,
    digest: String,
    text: String,
    value: Value,
    literal: bool,
}
#[derive(Debug)]
struct Snapshot {
    access: ClientAccess,
    records: Vec<Record>,
    base: Value,
    fingerprint: String,
}
impl Snapshot {
    fn capture(store: &mut Store, auth: &Auth, query: &Query) -> Result<Self> {
        let tx = store.conn.transaction()?;
        store::authorize(&tx, auth, "read")?;
        let workspace = store::workspace(&tx)?;
        let access = access::load(&tx, &auth.client_id)?;
        let persona = if access.mode == AccessMode::Shared {
            Value::Null
        } else {
            store::persona(&tx)?
        };
        let mut stmt=tx.prepare(&format!("SELECT payload,instr(lower(title || ' ' || content),lower(?1))>0 FROM records WHERE {} AND status NOT IN ('retracted','superseded','cancelled') ORDER BY id LIMIT 10001",access.filter()))?;
        let raw = stmt
            .query_map([&query.text], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, bool>(1)?))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        drop(stmt);
        if raw.len() > 10_000 {
            return Err(Error::domain(
                "RESOURCE_LIMIT",
                "Workspace exceeds supported record count.",
            ));
        }
        let mut fingerprint =
            workspace.to_string() + &persona.to_string() + &serde_json::to_string(&access)?;
        let mut records = Vec::with_capacity(raw.len());
        for (raw, literal) in raw {
            let value = contracts::parse_record(serde_json::from_str(&raw)?)?;
            fingerprint.push_str(&value.to_string());
            let text = format!(
                "{}\n{}",
                contracts::string(&value, "title")?,
                contracts::string(&value, "content")?
            );
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
            .filter(|r| {
                (r.value["kind"] == "task"
                    && matches!(r.value["status"].as_str(), Some("doing" | "blocked")))
                    || (r.value["kind"] == "action"
                        && matches!(r.value["status"].as_str(), Some("pending" | "unknown")))
            })
            .collect::<Vec<_>>();
        work.sort_by(|a, b| {
            b.value["updatedAt"]
                .as_str()
                .cmp(&a.value["updatedAt"].as_str())
                .then(a.id.cmp(&b.id))
        });
        let truncated = work.len() > 20;
        let work = work
            .into_iter()
            .take(20)
            .map(|r| json!({"id":r.id,"kind":r.value["kind"],"status":r.value["status"]}))
            .collect::<Vec<_>>();
        let mut base = json!({"workspace":workspace,"access":access.metadata(),"capturedAt":store::now(),"records":[],"omitted":0,"unsettledWork":work,"unsettledWorkTruncated":truncated,"note":"Excerpts and similarity scores are navigation, not evidence of truth. Resolve full records before editing or acting. A handoff does not pause running jobs."});
        if access.mode != AccessMode::Shared {
            base["persona"] = persona;
        }
        tx.commit()?;
        Ok(Self {
            access,
            records,
            base,
            fingerprint: contracts::hash(&fingerprint),
        })
    }
    fn scope(&self) -> String {
        format!(
            "{}:{}:{MODEL_REVISION}:{PROFILE}",
            self.base["workspace"]["id"], self.base["workspace"]["generation"]
        )
    }
    fn literal_response(self, query: &Query, reason: Option<&str>) -> Value {
        let eligible = self.records.iter().filter(|r| query.includes(r)).count();
        let hits = self
            .records
            .iter()
            .enumerate()
            .filter(|(_, r)| query.includes(r) && r.literal)
            .map(|(ordinal, _)| Hit {
                ordinal,
                similarity: None,
                span: None,
            })
            .collect();
        let metadata = json!({"requestedMode":query.mode.name(),"effectiveMode":"literal","semanticUsed":false,"reason":reason,"model":null,"eligibleRecords":eligible,"indexedRecords":0,"unindexedRecords":eligible,"partialRecords":0,"newlyIndexedRecords":0});
        self.response(query, hits, metadata)
    }
    fn response(mut self, query: &Query, mut hits: Vec<Hit>, metadata: Value) -> Value {
        hits.sort_by(|a, b| {
            let ra = &self.records[a.ordinal];
            let rb = &self.records[b.ordinal];
            let literal = if query.mode == Mode::Hybrid {
                rb.literal.cmp(&ra.literal)
            } else {
                std::cmp::Ordering::Equal
            };
            literal
                .then_with(|| {
                    b.similarity
                        .unwrap_or(-1.)
                        .total_cmp(&a.similarity.unwrap_or(-1.))
                })
                .then_with(|| {
                    rb.value["updatedAt"]
                        .as_str()
                        .cmp(&ra.value["updatedAt"].as_str())
                })
                .then_with(|| ra.id.cmp(&rb.id))
        });
        let omitted = hits.len().saturating_sub(query.limit);
        let records=hits.into_iter().take(query.limit).map(|h|{
            let r=&self.records[h.ordinal];let v=&r.value;let content=v["content"].as_str().unwrap_or_default();
            let mut p=json!({"id":r.id,"kind":v["kind"],"title":v["title"],"status":v["status"],"revision":r.revision,"actor":v["actor"],"updatedAt":v["updatedAt"],"excerpt":excerpt(content),"truncated":content.encode_utf16().count()>400});
            if let Some((embedding_start,embedding_end))=h.span {
                let (start,end)=context_span(&r.text,embedding_start,embedding_end);
                p["match"]=json!({"text":excerpt(&r.text[start..end]),"startByte":start,"endByte":end,"embeddingStartByte":embedding_start,"embeddingEndByte":embedding_end,"source":"title+newline+content","similarity":h.similarity,"truncated":r.text[start..end].encode_utf16().count()>400});
            }
            p
        }).collect::<Vec<_>>();
        self.base["records"] = json!(records);
        self.base["omitted"] = json!(omitted);
        self.base["search"] = metadata;
        self.base["search"]["updatedSince"] = json!(query.updated_since);
        self.base["search"]["updatedBefore"] = json!(query.updated_before);
        if self.base["access"]["mode"] == "shared" {
            self.base["search"]["scope"] = json!("granted_records");
        }
        self.base
    }
}
// A short matched sentence may depend on its neighbour. Include whole adjacent
// sentences only when the resulting navigation excerpt still fits its UTF-16 budget.
fn context_span(text: &str, start: usize, end: usize) -> (usize, usize) {
    // A run of terminators (".\n", "?!", "...") ends one sentence. A segment
    // without any word would otherwise stand in for the neighbouring sentence.
    let mut boundaries = vec![0];
    let mut close = |at: usize| {
        let last = boundaries.len() - 1;
        if text[boundaries[last]..at]
            .chars()
            .any(char::is_alphanumeric)
        {
            boundaries.push(at);
        } else if last > 0 {
            boundaries[last] = at;
        }
    };
    for (i, c) in text.char_indices() {
        if matches!(c, '.' | '!' | '?' | '\n' | '\r' | '。' | '！' | '？') {
            close(i + c.len_utf8());
        }
    }
    close(text.len());
    if boundaries.last().copied() != Some(text.len()) {
        boundaries.push(text.len());
    }
    let first = boundaries
        .partition_point(|b| *b <= start)
        .saturating_sub(1);
    let last = boundaries
        .partition_point(|b| *b < end)
        .min(boundaries.len() - 1);
    let proposed_start = boundaries[first.saturating_sub(1)];
    let expanded_start = if text[proposed_start..end].encode_utf16().count() <= 400 {
        proposed_start
    } else {
        start
    };
    let proposed_end = boundaries[(last + 1).min(boundaries.len() - 1)];
    let expanded_end = if text[expanded_start..proposed_end].encode_utf16().count() <= 400 {
        proposed_end
    } else {
        end
    };
    let span = &text[expanded_start..expanded_end];
    let trimmed = span.trim();
    let offset = span.len() - span.trim_start().len();
    (
        expanded_start + offset,
        expanded_start + offset + trimmed.len(),
    )
}
fn excerpt(content: &str) -> String {
    let mut used = 0;
    content
        .chars()
        .take_while(|c| {
            used += c.len_utf16();
            used <= 400
        })
        .collect()
}
#[derive(Debug)]
struct Hit {
    ordinal: usize,
    similarity: Option<f32>,
    span: Option<(usize, usize)>,
}
#[derive(Debug, Default)]
struct Progress {
    new_records: usize,
    new_passages: usize,
    capacity: bool,
}
fn cache_path(store: &Store, auth: &Auth, snapshot: &Snapshot) -> Result<PathBuf> {
    store.authorize(auth, "read")?;
    let access = &snapshot.access;
    let path = store
        .conn
        .path()
        .map(|p| cache::path(std::path::Path::new(p)))
        .ok_or_else(|| Error::domain("SEARCH_CACHE", "Search requires a file-backed workspace."))?;
    if access.mode == AccessMode::Shared {
        // Scoped snapshots must never prune the workspace-wide derived cache.
        let mut name = path.into_os_string();
        name.push(format!(
            ".{}",
            contracts::hash(&(auth.client_id.clone() + &serde_json::to_string(&access)?))
        ));
        Ok(PathBuf::from(name))
    } else {
        Ok(path)
    }
}
fn entries(cache: &Cache, snapshot: &Snapshot) -> Result<HashMap<String, Entry>> {
    snapshot
        .records
        .iter()
        .filter_map(
            |r| match cache.load(&r.id, r.revision, &r.digest, &r.text) {
                Ok(Some(e)) => Some(Ok((r.id.clone(), e))),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            },
        )
        .collect()
}
fn prepare(
    cache: &mut Cache,
    snapshot: &Snapshot,
    model: &LocalEmbedder,
    max_records: usize,
    max_passages: usize,
) -> Result<(HashMap<String, Entry>, Progress)> {
    let current = snapshot
        .records
        .iter()
        .map(|r| (r.id.as_str(), (r.revision, r.digest.as_str())))
        .collect();
    cache.prune(&current)?;
    let mut cached = entries(cache, snapshot)?;
    let mut progress = Progress::default();
    let mut count = cache.count()?;
    for r in &snapshot.records {
        if cached.contains_key(&r.id) {
            continue;
        }
        if progress.new_records >= max_records {
            break;
        }
        let plan = model.passages(&r.text)?;
        if progress.new_passages + plan.passages.len() > max_passages {
            break;
        }
        if count + plan.passages.len() > cache::MAX_PASSAGES {
            progress.capacity = true;
            break;
        }
        let vectors = plan
            .passages
            .iter()
            .map(|p| {
                Ok(Vector {
                    start: p.start,
                    end: p.end,
                    values: model.embed(p)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let entry = Entry {
            partial: plan.partial,
            vectors,
        };
        if !cache.put(&r.id, r.revision, &r.digest, &entry)? {
            progress.capacity = true;
            break;
        }
        count += entry.vectors.len();
        progress.new_passages += entry.vectors.len();
        progress.new_records += 1;
        cached.insert(r.id.clone(), entry);
    }
    Ok((cached, progress))
}
fn semantic_response(
    snapshot: Snapshot,
    query: &Query,
    cached: &HashMap<String, Entry>,
    query_vector: &[f32],
    progress: &Progress,
) -> Result<Value> {
    cache::validate_vector(query_vector)?;
    let mut hits = vec![];
    let mut eligible = 0;
    let mut indexed = 0;
    let mut partial = 0;
    let mut unembeddable = 0;
    for (ordinal, r) in snapshot
        .records
        .iter()
        .enumerate()
        .filter(|(_, r)| query.includes(r))
    {
        eligible += 1;
        let mut best = None;
        if let Some(e) = cached.get(&r.id) {
            if e.vectors.is_empty() {
                unembeddable += 1;
            } else {
                indexed += 1;
            }
            if e.partial {
                partial += 1;
            }
            for v in &e.vectors {
                let score: f32 = v.values.iter().zip(query_vector).map(|(a, b)| a * b).sum();
                if best.as_ref().is_none_or(|(s, _, _)| score > *s) {
                    best = Some((score, v.start, v.end));
                }
            }
        }
        let semantic = best.filter(|(score, _, _)| *score >= MIN_SIMILARITY);
        if semantic.is_some() || (query.mode == Mode::Hybrid && r.literal) {
            hits.push(Hit {
                ordinal,
                similarity: semantic.map(|(s, _, _)| s),
                span: semantic.map(|(_, start, end)| (start, end)),
            });
        }
    }
    // With no eligible records there is nothing left to index, so the empty
    // result below is complete rather than a fallback.
    if indexed == 0 && eligible > 0 {
        let reason = if progress.capacity {
            "index_capacity"
        } else if unembeddable > 0 {
            "unembeddable_records"
        } else {
            "index_incomplete"
        };
        let mut result = snapshot.literal_response(query, Some(reason));
        result["search"]["unembeddableRecords"] = json!(unembeddable);
        result["search"]["newlyIndexedRecords"] = json!(progress.new_records);
        return Ok(result);
    }
    let reason = if progress.capacity {
        Some("index_capacity")
    } else if indexed < eligible || partial > 0 {
        Some("index_incomplete")
    } else {
        None
    };
    let metadata = json!({"requestedMode":query.mode.name(),"effectiveMode":query.mode.name(),"semanticUsed":true,"reason":reason,"model":MODEL_ID,"profile":PROFILE,"similarityFloor":0.35,"eligibleRecords":eligible,"indexedRecords":indexed,"unindexedRecords":eligible-indexed,"partialRecords":partial,"unembeddableRecords":unembeddable,"newlyIndexedRecords":progress.new_records,"newlyIndexedPassages":progress.new_passages});
    Ok(snapshot.response(query, hits, metadata))
}
fn finish(
    store: &mut Store,
    auth: &Auth,
    query: &Query,
    initial: &Snapshot,
    cached: &HashMap<String, Entry>,
    vector: &[f32],
    progress: &Progress,
) -> Result<Value> {
    let final_snapshot = Snapshot::capture(store, auth, query)?;
    if final_snapshot.fingerprint != initial.fingerprint {
        return Ok(final_snapshot.literal_response(query, Some("workspace_changed")));
    }
    semantic_response(final_snapshot, query, cached, vector, progress)
}
impl Search {
    /// Select local model assets; this does not load or download a model.
    pub fn new(model_dir: PathBuf) -> Self {
        Self {
            model_dir: Some(model_dir),
            model: Mutex::new(None),
        }
    }
    /// Resolve optional local model configuration without disabling canonical context access.
    pub fn from_environment() -> Self {
        Self {
            model_dir: crate::embedding::model_directory().ok(),
            model: Mutex::new(None),
        }
    }
    /// Search canonical records by meaning and/or exact text without sending data remotely.
    pub fn search(&self, store: &mut Store, auth: &Auth, args: Value) -> Result<Value> {
        let query = Query::parse(args)?;
        let initial = Snapshot::capture(store, auth, &query)?;
        if query.mode == Mode::Literal {
            return Ok(Snapshot::capture(store, auth, &query)?.literal_response(&query, None));
        }
        let mut guard = match self.model.try_lock() {
            Ok(g) => g,
            Err(_) => {
                return Ok(Snapshot::capture(store, auth, &query)?
                    .literal_response(&query, Some("model_busy")));
            }
        };
        if guard.is_none() {
            match self
                .model_dir
                .as_deref()
                .ok_or_else(|| {
                    Error::domain("MODEL_UNAVAILABLE", "Local model path is unavailable.")
                })
                .and_then(LocalEmbedder::load)
            {
                Ok(model) => *guard = Some(model),
                Err(_) => {
                    return Ok(Snapshot::capture(store, auth, &query)?
                        .literal_response(&query, Some("model_unavailable")));
                }
            }
        }
        let Some(model) = guard.as_ref() else {
            return Err(Error::domain(
                "MODEL_UNAVAILABLE",
                "Local embedding model is unavailable.",
            ));
        };
        let path = cache_path(store, auth, &initial)?;
        let mut cache = match Cache::open(&path, &initial.scope()) {
            Ok(c) => c,
            Err(_) => {
                return Ok(Snapshot::capture(store, auth, &query)?
                    .literal_response(&query, Some("cache_unavailable")));
            }
        };
        let vector = match model.query(&query.text) {
            Ok(v) => v,
            Err(_) => {
                return Ok(Snapshot::capture(store, auth, &query)?
                    .literal_response(&query, Some("embedding_failed")));
            }
        };
        let (cached, progress) =
            match prepare(&mut cache, &initial, model, QUERY_RECORDS, QUERY_PASSAGES) {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Snapshot::capture(store, auth, &query)?.literal_response(
                        &query,
                        Some(if e.code() == "MODEL_UNAVAILABLE" {
                            "embedding_failed"
                        } else {
                            "cache_unavailable"
                        }),
                    ));
                }
            };
        finish(store, auth, &query, &initial, &cached, &vector, &progress)
    }
    /// Prepare a bounded batch of missing current embeddings; repeat until coverage is complete.
    pub fn index(&self, store: &mut Store, auth: &Auth, max_records: usize) -> Result<Value> {
        if !(1..=100).contains(&max_records) {
            return Err(invalid("Index batch must contain 1 to 100 records"));
        }
        let query = Query::parse(json!({"query":"index"}))?;
        let initial = Snapshot::capture(store, auth, &query)?;
        let mut guard = self
            .model
            .try_lock()
            .map_err(|_| Error::domain("MODEL_BUSY", "Another local search is using the model."))?;
        if guard.is_none() {
            *guard = Some(LocalEmbedder::load(self.model_dir.as_deref().ok_or_else(
                || Error::domain("MODEL_UNAVAILABLE", "Local model path is unavailable."),
            )?)?);
        }
        let Some(model) = guard.as_ref() else {
            return Err(Error::domain(
                "MODEL_UNAVAILABLE",
                "Local embedding model is unavailable.",
            ));
        };
        let mut cache = Cache::open(&cache_path(store, auth, &initial)?, &initial.scope())?;
        let (cached, progress) = prepare(&mut cache, &initial, model, max_records, INDEX_PASSAGES)?;
        let current = Snapshot::capture(store, auth, &query)?;
        if current.fingerprint != initial.fingerprint {
            return Ok(
                json!({"complete":false,"reason":"workspace_changed","eligibleRecords":current.records.len(),"newlyIndexedRecords":0}),
            );
        }
        let partial = cached.values().filter(|e| e.partial).count();
        let unembeddable = cached.values().filter(|e| e.vectors.is_empty()).count();
        let indexed = cached.len() - unembeddable;
        let eligible = current.records.len();
        Ok(
            json!({"complete":indexed==eligible&&partial==0,"reason":if progress.capacity{Some("index_capacity")}else if indexed<eligible||partial>0{Some("index_incomplete")}else{None},"model":MODEL_ID,"profile":PROFILE,"eligibleRecords":eligible,"indexedRecords":indexed,"unindexedRecords":eligible-indexed,"partialRecords":partial,"unembeddableRecords":unembeddable,"newlyIndexedRecords":progress.new_records,"newlyIndexedPassages":progress.new_passages}),
        )
    }
}
#[cfg(test)]
mod tests;
