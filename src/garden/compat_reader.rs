//! Owner-only narrative reading of single v1 records for the garden explorer's reader.
//!
//! The reader offers three lenses over the same stored evidence: Mother's Eyes (existing
//! assistant reflections, attributed), My Memories (the record's own account, labelled by how it
//! was captured) and The Archive (exact identifiers, provenance and revisions). It is a pure,
//! deterministic projection made in one read transaction. It never writes, never generates text,
//! and never treats a title or guessed ID as a relationship: a reflection is shown only because
//! its stored `references` name the record.
use super::{
    PROPOSALS, RECORDS, View, excerpt, inbox, metadata, recent, unavailable,
    view::{history, revisions},
};
use crate::{
    contracts::{self, string},
    error::{Error, Result},
    garden::store::GardenStore,
};
use redb::ReadableDatabase;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

/// Longest record text one reading carries. The v1 contract allows 12,000 UTF-16 units, so this
/// only bounds records kept from older or imported data.
const MAX_READER_CONTENT: usize = 24_000;
/// Reflections shown with their full text; further ones are counted.
const MAX_REFLECTIONS: usize = 5;
/// Records naming this one listed in the Archive; further ones are counted.
const MAX_LINKED: usize = 60;
const MAX_READER_REVISIONS: usize = 100;

fn missing() -> Error {
    Error::domain("NOT_FOUND", "No such record in this garden.")
}

/// Who wrote a revision and whether the owner approved it, as far as stored evidence shows.
struct Provenance {
    writer: Value,
    review: &'static str,
    attribution: Value,
}

/// Saved v1 inbox proposals, keyed by the record ID and revision their approval wrote.
fn approvals(tx: &redb::ReadTransaction) -> Result<HashMap<(String, u64), Value>> {
    let mut out = HashMap::new();
    for p in tx.values(PROPOSALS)? {
        if p["status"] != "saved" {
            continue;
        }
        let written = &p["receipt"]["receipt"]["record"];
        if let (Some(id), Some(revision)) = (written["id"].as_str(), written["revision"].as_u64()) {
            out.insert((id.to_owned(), revision), p);
        }
    }
    Ok(out)
}

fn provenance(record: &Value, approvals: &HashMap<(String, u64), Value>) -> Provenance {
    let actor = &record["actor"];
    let approved = record["id"]
        .as_str()
        .zip(record["revision"].as_u64())
        .and_then(|(id, revision)| approvals.get(&(id.to_owned(), revision)));
    match approved {
        Some(p) if *actor == inbox::OWNER_ACTOR => Provenance {
            writer: p["actor"].clone(),
            review: "reviewed",
            attribution: p["source"].clone(),
        },
        // Approved by the owner, but the proposal that carried the writer is no longer kept.
        None if *actor == inbox::OWNER_ACTOR => Provenance {
            writer: Value::Null,
            review: "reviewed",
            attribution: Value::Null,
        },
        _ => Provenance {
            writer: actor.clone(),
            review: "unknown",
            attribution: Value::Null,
        },
    }
}

fn text(record: &Value) -> (String, bool, usize) {
    let content = record["content"].as_str().unwrap_or_default();
    let length = content.encode_utf16().count();
    (
        excerpt(content, MAX_READER_CONTENT),
        length > MAX_READER_CONTENT,
        length,
    )
}

fn is_reflection(record: &Value) -> bool {
    record["kind"] == "memory" && record["basis"] == "model_inference"
}

fn list(record: &Value, key: &str) -> Vec<Value> {
    record[key].as_array().cloned().unwrap_or_default()
}

/// A record named from another, in brief.
fn brief(record: &Value) -> Value {
    let mut out = json!({
        "id": record["id"],
        "kind": record["kind"],
        "title": record["title"],
        "status": record["status"],
        "updatedAt": record["updatedAt"],
    });
    if let Some(basis) = record.get("basis").filter(|v| v.is_string()) {
        out["basis"] = basis.clone();
    }
    out
}

/// Mother's Eyes: one attributed reflection.
fn witness(reflection: &Value, relation: &str, approvals: &HashMap<(String, u64), Value>) -> Value {
    let (body, truncated, length) = text(reflection);
    let who = provenance(reflection, approvals);
    json!({
        "recordId": reflection["id"],
        "revision": reflection["revision"],
        "title": reflection["title"],
        "text": body,
        "truncated": truncated,
        "length": length,
        "relation": relation,
        "writer": who.writer,
        "attribution": who.attribution,
        "writtenAt": reflection["updatedAt"],
        "firstWrittenAt": reflection["createdAt"],
        "reviewStatus": who.review,
        "status": reflection["status"],
        "references": list(reflection, "references"),
        "sources": list(reflection, "sources"),
    })
}

/// My Memories: the record's own account, labelled by how it came to be written.
fn account(record: &Value, who: &Provenance) -> Value {
    let form = match (record["kind"].as_str(), record["basis"].as_str()) {
        (Some("memory"), Some("owner_statement")) => "captured_paraphrase",
        (Some("memory"), Some("source")) => "source_summary",
        (Some("memory"), Some("model_inference")) => "interpretation",
        (Some("memory"), _) => "unlabelled",
        _ => "working_record",
    };
    json!({
        "form": form,
        // No stored field marks a record's text as the owner's own exact words, so no account
        // is ever presented as verbatim.
        "ownerWords": false,
        "writer": who.writer,
        "attribution": who.attribution,
        "reviewStatus": who.review,
    })
}

impl GardenStore {
    /// The reader's projection of one v1 record: its full text, provenance, the existing
    /// reflections that reference it, its links and its revisions. Read-only.
    pub fn v1_garden_reader(&self, id: &str) -> Result<Value> {
        self.v1_garden_readers(&[id])?.pop().ok_or_else(missing)
    }

    /// Readings of several records from one read transaction; IDs that do not name a current
    /// v1 record are left out.
    pub fn v1_garden_readers(&self, ids: &[&str]) -> Result<Vec<Value>> {
        if !self.v1_initialized()? {
            return Err(Error::domain(
                "COMPAT_NOT_INITIALIZED",
                "The owner has not enabled the native v1 compatibility view.",
            ));
        }
        let wanted: HashSet<&str> = ids
            .iter()
            .copied()
            .filter(|id| contracts::id(id).is_ok())
            .collect();
        if wanted.is_empty() {
            return Ok(Vec::new());
        }
        let tx = self.db.begin_read().map_err(unavailable)?;
        let settings = super::settings::Settings::load(&tx)?;
        let workspace = metadata(&tx, "workspace")?;
        let generation = string(&workspace, "generation")?;
        let mut all = tx.values(RECORDS)?;
        all.sort_by(recent);
        let by_id: HashMap<&str, &Value> = all
            .iter()
            .filter_map(|r| Some((r["id"].as_str()?, r)))
            .collect();
        // Who names each wanted record, newest first.
        let mut named_by: HashMap<&str, Vec<&Value>> = HashMap::new();
        for r in &all {
            let mut seen = HashSet::new();
            for target in r["references"].as_array().into_iter().flatten() {
                if let Some(target) = target.as_str().filter(|t| wanted.contains(t))
                    && target != r["id"]
                    && seen.insert(target)
                {
                    named_by.entry(target).or_default().push(r);
                }
            }
        }
        let found: Vec<&Value> = ids
            .iter()
            .filter(|id| wanted.contains(*id))
            .filter_map(|id| by_id.get(id).copied())
            .collect();
        let mut snapshots = revisions(
            &tx,
            generation,
            found.iter().filter_map(|r| r["id"].as_str()).collect(),
        )?;
        let approvals = approvals(&tx)?;

        found
            .into_iter()
            .map(|r| {
                let id = string(r, "id")?;
                let (body, truncated, length) = text(r);
                let who = provenance(r, &approvals);
                let naming = named_by.remove(id).unwrap_or_default();

                let mut witnesses = Vec::new();
                if is_reflection(r) {
                    witnesses.push(witness(r, "self", &approvals));
                }
                let candidates: Vec<&&Value> = naming
                    .iter()
                    .filter(|n| is_reflection(n) && n["status"] == "active")
                    .collect();
                let witness_omitted = candidates.len().saturating_sub(MAX_REFLECTIONS);
                witnesses.extend(
                    candidates
                        .iter()
                        .take(MAX_REFLECTIONS)
                        .map(|n| witness(n, "references", &approvals)),
                );

                let lens = if witnesses.is_empty() { "owner" } else { "mother" };
                let references = list(r, "references")
                    .iter()
                    .filter_map(Value::as_str)
                    .map(|target| match by_id.get(target) {
                        Some(found) => brief(found),
                        None => json!({"id": target, "available": false}),
                    })
                    .collect::<Vec<_>>();
                let linked_omitted = naming.len().saturating_sub(MAX_LINKED);
                let referenced_by = naming
                    .iter()
                    .take(MAX_LINKED)
                    .map(|n| brief(n))
                    .collect::<Vec<_>>();
                let (history, history_omitted) =
                    history(snapshots.remove(id), r, MAX_READER_REVISIONS);

                let mut out = json!({
                    "recordId": id,
                    "kind": r["kind"],
                    "status": r["status"],
                    "title": r["title"],
                    "revision": r["revision"],
                    "actor": r["actor"],
                    "basis": r.get("basis").filter(|v| v.is_string()).cloned().unwrap_or(Value::Null),
                    "createdAt": r["createdAt"],
                    "updatedAt": r["updatedAt"],
                    "eventDate": settings.date(id), "keepsake":settings.hidden(id),
                    "content": body,
                    "truncated": truncated,
                    "length": length,
                    "sources": list(r, "sources"),
                    "account": account(r, &who),
                    "witnessAccounts": witnesses,
                    "witnessOmitted": witness_omitted,
                    "references": references,
                    "referencedBy": referenced_by,
                    "referencedByOmitted": linked_omitted,
                    "history": history,
                    "historyOmitted": history_omitted,
                    "defaultLens": lens,
                    "readOnly": true,
                });
                for key in [
                    "nextStep",
                    "completed",
                    "openQuestions",
                    "mediaType",
                    "destination",
                ] {
                    if let Some(value) = r.get(key).filter(|v| !v.is_null()) {
                        out[key] = value.clone();
                    }
                }
                Ok(out)
            })
            .collect()
    }
}
