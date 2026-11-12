//! Read-only revision history, reconstructed from retained mutation receipts.
//!
//! Every committed write stores a receipt holding the full record it produced,
//! so earlier revisions already exist; nothing new is written to read them.
use crate::{
    access::{self, AccessMode},
    contracts::{self, Auth, integer, string},
    error::{Error, Result, invalid},
    store::{self, Store},
};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::collections::BTreeMap;

const DEFAULT_LIMIT: u64 = 20;
const EXCERPT_UNITS: usize = 400;
// Fields a client chooses; identity and timestamps are set by the store.
const COMPARED: [&str; 11] = [
    "title",
    "content",
    "status",
    "references",
    "sources",
    "basis",
    "nextStep",
    "completed",
    "openQuestions",
    "mediaType",
    "destination",
];

struct Snapshot {
    record: Value,
    generation: String,
    committed_at: String,
}

/// Every retained snapshot of one record, keyed by revision. A revision written
/// once per generation is unique; if a restored copy repeats a revision, the
/// current generation wins.
fn snapshots(conn: &Connection, id: &str, generation: &str) -> Result<BTreeMap<u64, Snapshot>> {
    let mut statement = conn.prepare(
        "SELECT generation,receipt FROM operations WHERE instr(receipt,?1)>0 AND json_extract(receipt,'$.record.id')=?1",
    )?;
    let mut found = BTreeMap::new();
    for row in statement.query_map([id], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })? {
        let (row_generation, raw) = row?;
        let receipt: Value = serde_json::from_str(&raw)?;
        let record = contracts::parse_record(receipt["record"].clone())?;
        let revision = integer(&record, "revision", 1, 9007199254740991)?;
        let snapshot = Snapshot {
            committed_at: string(&receipt, "committedAt")?.to_owned(),
            record,
            generation: row_generation,
        };
        let keep = found
            .get(&revision)
            .is_none_or(|kept: &Snapshot| kept.generation != generation);
        if keep {
            found.insert(revision, snapshot);
        }
    }
    Ok(found)
}

fn excerpt(content: &str) -> (String, bool) {
    let mut used = 0;
    let text: String = content
        .chars()
        .take_while(|c| {
            used += c.len_utf16();
            used <= EXCERPT_UNITS
        })
        .collect();
    let truncated = text.len() < content.len();
    (text, truncated)
}

fn changed(before: &Value, after: &Value) -> Value {
    json!(
        COMPARED
            .iter()
            .filter(|key| before.get(**key) != after.get(**key))
            .collect::<Vec<_>>()
    )
}

fn entry(revision: u64, s: &Snapshot, previous: Option<&Snapshot>, generation: &str) -> Value {
    let content = s.record["content"].as_str().unwrap_or_default();
    let (text, truncated) = excerpt(content);
    json!({
        "revision": revision,
        "status": s.record["status"],
        "title": s.record["title"],
        "actor": s.record["actor"],
        "committedAt": s.committed_at,
        "currentGeneration": s.generation == generation,
        "excerpt": text,
        "truncated": truncated,
        "changed": previous.map_or(Value::Null, |p| changed(&p.record, &s.record)),
    })
}

/// A validated history request: the record, one wanted revision, or a list limit.
struct Request {
    id: String,
    wanted: Option<u64>,
    limit: Option<u64>,
}
fn parse(query: &Value) -> Result<Request> {
    contracts::strict(query, &["id", "revision", "limit"])?;
    let id = string(query, "id")?;
    contracts::id(id)?;
    let wanted = query
        .get("revision")
        .map(|_| integer(query, "revision", 1, 9007199254740991))
        .transpose()?;
    let limit = query
        .get("limit")
        .map(|_| integer(query, "limit", 1, 100))
        .transpose()?;
    if wanted.is_some() && limit.is_some() {
        return Err(invalid("Request one revision or a list, not both"));
    }
    Ok(Request {
        id: id.to_owned(),
        wanted,
        limit,
    })
}
/// Read history inside the caller's transaction, after the caller checked its own authority.
fn read(tx: &Connection, request: &Request) -> Result<Value> {
    let (id, wanted, limit) = (request.id.as_str(), request.wanted, request.limit);
    let (current, _) = store::read_record(tx, id)?;
    let current_revision = integer(&current, "revision", 1, 9007199254740991)?;
    let workspace = store::workspace(tx)?;
    let generation = string(&workspace, "generation")?;
    let found = snapshots(tx, id, generation)?;
    let entries: Vec<(u64, &Snapshot, Option<&Snapshot>)> = found
        .iter()
        .map(|(revision, s)| {
            let previous = revision.checked_sub(1).and_then(|p| found.get(&p));
            (*revision, s, previous)
        })
        .collect();
    let result = if let Some(revision) = wanted {
        let (_, snapshot, previous) =
            entries
                .iter()
                .find(|(r, _, _)| *r == revision)
                .ok_or_else(|| {
                    Error::domain(
                        "NOT_FOUND",
                        "No retained snapshot exists for that revision of this record.",
                    )
                })?;
        json!({
            "workspace": workspace,
            "id": id,
            "kind": current["kind"],
            "currentRevision": current_revision,
            "current": revision == current_revision,
            "entry": entry(revision, snapshot, *previous, generation),
            "record": snapshot.record,
        })
    } else {
        let limit = usize::try_from(limit.unwrap_or(DEFAULT_LIMIT))
            .map_err(|_| invalid("Invalid limit"))?;
        let listed: Vec<Value> = entries
            .iter()
            .rev()
            .take(limit)
            .map(|(r, s, p)| entry(*r, s, *p, generation))
            .collect();
        let missing: Vec<u64> = (1..=current_revision)
            .filter(|r| !found.contains_key(r))
            .collect();
        json!({
            "workspace": workspace,
            "id": id,
            "kind": current["kind"],
            "currentRevision": current_revision,
            "currentStatus": current["status"],
            "returnedRevisions": listed.len(),
            "omittedRevisions": found.len() - listed.len(),
            "missingRevisions": missing,
            "revisions": listed,
            "note": "Revisions come from retained write receipts, newest first. changed lists the client-chosen fields that differ from the revision before, and is null when that revision has no retained snapshot. Request one revision to read it in full. Earlier text is history, not current fact.",
        })
    };
    Ok(result)
}

impl Store {
    /// List a record's retained revisions, or read one earlier revision in full.
    ///
    /// Requires the read scope and full-context visibility: shared clients see
    /// only the current version of the records granted to them.
    pub fn history(&mut self, auth: &Auth, query: Value) -> Result<Value> {
        let request = parse(&query)?;
        let tx = self.conn.transaction()?;
        store::authorize(&tx, auth, "read")?;
        if access::load(&tx, &auth.client_id)?.mode == AccessMode::Shared {
            return Err(Error::domain(
                "FORBIDDEN",
                "Shared clients can read only the current version of granted records.",
            ));
        }
        let result = read(&tx, &request)?;
        tx.commit()?;
        Ok(result)
    }
    /// The same history for the local owner, who needs no grant.
    pub fn owner_history(&mut self, query: Value) -> Result<Value> {
        let request = parse(&query)?;
        let tx = self.conn.transaction()?;
        let result = read(&tx, &request)?;
        tx.commit()?;
        Ok(result)
    }
}
