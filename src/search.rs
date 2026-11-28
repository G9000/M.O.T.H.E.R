//! Local semantic retrieval over authorized canonical records, with truthful literal fallback.
pub(crate) mod cache;
use crate::{
    access::ClientAccess,
    contracts::{self, Kind},
    embedding::{MODEL_ID, PROFILE},
    error::{Result, invalid},
};
use cache::Entry;
use serde_json::{Value, json};
use std::collections::HashMap;
const MIN_SIMILARITY: f32 = 0.35;

/// Describe the optional local candidate space without changing v1 search defaults.
pub fn embedding_profile() -> crate::embedding::EmbeddingProfile {
    crate::embedding::profile()
}
/// Shared cache identity for callers keeping separate rebuildable derived indexes.
pub fn embedding_profile_fingerprint() -> String {
    cache::profile_fingerprint()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    Hybrid,
    Semantic,
    Literal,
}
impl Mode {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Hybrid => "hybrid",
            Self::Semantic => "semantic",
            Self::Literal => "literal",
        }
    }
}
#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) text: String,
    pub(crate) kind: Option<String>,
    pub(crate) limit: usize,
    pub(crate) mode: Mode,
    pub(crate) updated_since: Option<String>,
    pub(crate) updated_before: Option<String>,
    pub(crate) filters: crate::pagination::Filters,
    pub(crate) view: crate::pagination::View,
    /// The request, kept to validate its optional cursor against the final ranking.
    pub(crate) args: Value,
}
impl Query {
    pub(crate) fn parse(v: Value) -> Result<Self> {
        contracts::strict(
            &v,
            &[
                "query",
                "kind",
                "limit",
                "mode",
                "updatedSince",
                "updatedBefore",
                "actor",
                "status",
                "basis",
                "cursor",
                "includePersona",
                "includePinned",
                "previewMaxChars",
                "maxResponseBytes",
            ],
        )?;
        let filters = crate::pagination::Filters::parse(&v)?;
        let view = crate::pagination::View::parse(&v)?;
        crate::pagination::response_budget(&v)?;
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
            filters,
            view,
            args: v,
        })
    }
    pub(crate) fn includes(&self, r: &Record) -> bool {
        // Stored and bound times share one fixed-width UTC form, so text order is time order.
        let updated = r.value["updatedAt"].as_str().unwrap_or_default();
        self.kind
            .as_ref()
            .is_none_or(|k| r.value["kind"].as_str() == Some(k))
            && self.filters.matches(&r.value)
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
pub(crate) struct Record {
    pub(crate) id: String,
    pub(crate) revision: u64,
    pub(crate) digest: String,
    pub(crate) text: String,
    pub(crate) value: Value,
    pub(crate) literal: bool,
    pub(crate) alias: Option<String>,
}
#[derive(Debug)]
pub(crate) struct Snapshot {
    pub(crate) access: ClientAccess,
    pub(crate) records: Vec<Record>,
    pub(crate) base: Value,
    pub(crate) fingerprint: String,
}
impl Snapshot {
    pub(crate) fn literal_response(self, query: &Query, reason: Option<&str>) -> Result<Value> {
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
    fn response(mut self, query: &Query, mut hits: Vec<Hit>, metadata: Value) -> Result<Value> {
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
        // Pages are offsets into one ranking; the stamp changes with records, grant,
        // index coverage and effective mode, and an outdated cursor is refused.
        let binding = crate::pagination::binding(&json!([
            self.base["workspace"]["generation"],
            serde_json::to_value(&self.access)?,
            query.text,
            query.kind,
            query.filters.describe(),
            query.updated_since,
            query.updated_before,
            query.mode.name()
        ]));
        let stamp = crate::contracts::hash(&format!(
            "{}|{}|{}|{}",
            self.fingerprint,
            metadata["effectiveMode"],
            metadata["indexedRecords"],
            metadata["partialRecords"]
        ))[..32]
            .to_owned();
        let start = crate::pagination::search_offset(&query.args, &binding, &stamp)?;
        let total = hits.len();
        let preview = query.view.preview_chars;
        let records=hits.into_iter().skip(start).take(query.limit).map(|h|{
            let r=&self.records[h.ordinal];let v=&r.value;let content=v["content"].as_str().unwrap_or_default();
            let mut p=json!({"id":r.id,"kind":v["kind"],"title":v["title"],"status":v["status"],"revision":r.revision,"actor":v["actor"],"createdAt":v["createdAt"],"updatedAt":v["updatedAt"],"excerpt":crate::pagination::excerpt(content,preview),"truncated":content.encode_utf16().count()>preview});
            let lexical = if r.literal && (query.mode!=Mode::Semantic || metadata["effectiveMode"]=="literal") {Some(r.alias.as_ref().map_or_else(||json!("query phrase"),|name|json!({"projectName":name})))} else {None};
            p["recallReason"]=json!({"lexical":lexical,"semantic":h.span.is_some()});
            if let Some(basis)=v.get("basis") {p["basis"]=basis.clone();}
            if let Some((embedding_start,embedding_end))=h.span {
                let (start,end)=context_span(&r.text,embedding_start,embedding_end);
                p["match"]=json!({"text":excerpt(&r.text[start..end]),"startByte":start,"endByte":end,"embeddingStartByte":embedding_start,"embeddingEndByte":embedding_end,"source":"title+newline+content","similarity":h.similarity,"truncated":r.text[start..end].encode_utf16().count()>400});
            }
            p
        }).collect::<Vec<_>>();
        let end = (start + records.len()).min(total);
        let omitted = total - records.len();
        let has_more = end < total;
        self.base["pagination"] = json!({"order":"ranked","matchingRecords":total,"offset":start,
            "returnedRecords":records.len(),"remainingAfterPage":total-end,"hasMore":has_more,
            "nextCursor":crate::pagination::search_cursor(&binding,&stamp,end),
            "consistency":"ranking_snapshot"});
        self.base["records"] = json!(records);
        self.base["omitted"] = json!(omitted);
        self.base["search"] = metadata;
        self.base["search"]["updatedSince"] = json!(query.updated_since);
        self.base["search"]["updatedBefore"] = json!(query.updated_before);
        if self.base["access"]["mode"] == "shared" {
            self.base["search"]["scope"] = json!("granted_records");
        }
        Ok(self.base)
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
    crate::pagination::excerpt(content, 400)
}
#[derive(Debug)]
struct Hit {
    ordinal: usize,
    similarity: Option<f32>,
    span: Option<(usize, usize)>,
}
#[derive(Debug, Default)]
pub(crate) struct Progress {
    pub(crate) new_records: usize,
    pub(crate) new_passages: usize,
    pub(crate) capacity: bool,
}
pub(crate) fn semantic_response_validated(
    snapshot: Snapshot,
    query: &Query,
    cached: &HashMap<String, Entry>,
    query_vector: &[f32],
    progress: &Progress,
) -> Result<Value> {
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
        let mut result = snapshot.literal_response(query, Some(reason))?;
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
    snapshot.response(query, hits, metadata)
}
#[cfg(test)]
mod tests;
