//! Owner-only read model of the native v1 view for the local memory garden explorer.
//! One read transaction: it never writes, widens a grant, or returns persona or credentials.
use super::{ASSIGNMENTS, OPERATIONS, PINS, RECORDS, View, excerpt, metadata, recent, unavailable};
use crate::{
    contracts::string,
    error::{Error, Result},
    garden::store::GardenStore,
};
use chrono::SecondsFormat;
use redb::{ReadableDatabase, ReadableTable};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap};

/// Most records one explorer snapshot may carry.
pub const MAX_VIEW_RECORDS: usize = 2_000;
const MAX_VIEW_REVISIONS: usize = 20;
const MAX_VIEW_CONTENT: usize = 4_000;
const CHANGED: [&str; 11] = [
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

fn keyed(values: Vec<Value>) -> HashMap<String, Value> {
    values
        .into_iter()
        .filter_map(|v| Some((v["recordId"].as_str()?.to_owned(), v)))
        .collect()
}

impl GardenStore {
    /// Bounded snapshot of current v1 records with pins, handoffs and recent revision history.
    /// Pinned records come first, then the most recently updated, up to `limit`.
    pub fn v1_garden_view(&self, limit: usize) -> Result<Value> {
        if limit == 0 || limit > MAX_VIEW_RECORDS {
            return Err(Error::domain(
                "INVALID_INPUT",
                "The record limit must be between 1 and 2000.",
            ));
        }
        if !self.v1_initialized()? {
            return Err(Error::domain(
                "COMPAT_NOT_INITIALIZED",
                "The owner has not enabled the native v1 compatibility view.",
            ));
        }
        let tx = self.db.begin_read().map_err(unavailable)?;
        let workspace = metadata(&tx, "workspace")?;
        let generation = string(&workspace, "generation")?;
        let pins = keyed(tx.values(PINS)?);
        let assignments = keyed(tx.values(ASSIGNMENTS)?);
        let mut records = tx.values(RECORDS)?;
        let total = records.len();
        records.sort_by(recent);
        records.sort_by_key(|r| !r["id"].as_str().is_some_and(|id| pins.contains_key(id)));
        records.truncate(limit);

        // One pass over the immutable receipts gathers every shown record's revisions.
        let mut snapshots: HashMap<String, BTreeMap<u64, (bool, Value)>> = records
            .iter()
            .filter_map(|r| Some((r["id"].as_str()?.to_owned(), BTreeMap::new())))
            .collect();
        for entry in tx
            .open_table(OPERATIONS)
            .map_err(unavailable)?
            .iter()
            .map_err(unavailable)?
        {
            let (_, bytes) = entry.map_err(unavailable)?;
            let mut op: Value = serde_json::from_slice(bytes.value())?;
            let current = op["generation"] == generation;
            let record = op["receipt"]["record"].take();
            let (Some(id), Some(revision)) = (record["id"].as_str(), record["revision"].as_u64())
            else {
                continue;
            };
            let Some(revisions) = snapshots.get_mut(id) else {
                continue;
            };
            if revisions.get(&revision).is_none_or(|kept| !kept.0) {
                revisions.insert(revision, (current, record));
            }
        }

        let shown = records
            .into_iter()
            .map(|r| {
                let id = string(&r, "id")?;
                let mut revisions = snapshots.remove(id).unwrap_or_default();
                if let Some(revision) = r["revision"].as_u64() {
                    revisions.entry(revision).or_insert((true, r.clone()));
                }
                let history = revisions
                    .iter()
                    .map(|(revision, (_, s))| {
                        let previous = revision.checked_sub(1).and_then(|p| revisions.get(&p));
                        let changed = previous.map_or(Value::Null, |(_, p)| {
                            json!(
                                CHANGED
                                    .into_iter()
                                    .filter(|k| p.get(*k) != s.get(*k))
                                    .collect::<Vec<_>>()
                            )
                        });
                        json!({
                            "revision": revision,
                            "actor": s["actor"],
                            "at": s["updatedAt"],
                            "changed": changed,
                            "excerpt": excerpt(s["content"].as_str().unwrap_or_default(), 400),
                        })
                    })
                    .collect::<Vec<_>>();
                let omitted = history.len().saturating_sub(MAX_VIEW_REVISIONS);
                let content = r["content"].as_str().unwrap_or_default();
                let mut out = json!({
                    "id": id,
                    "kind": r["kind"],
                    "status": r["status"],
                    "title": r["title"],
                    "content": excerpt(content, MAX_VIEW_CONTENT),
                    "truncated": content.encode_utf16().count() > MAX_VIEW_CONTENT,
                    "revision": r["revision"],
                    "actor": r["actor"],
                    "createdAt": r["createdAt"],
                    "updatedAt": r["updatedAt"],
                    "references": r.get("references").cloned().unwrap_or_else(|| json!([])),
                    "history": history[omitted..],
                    "historyOmitted": omitted,
                });
                if let Some(next) = r.get("nextStep").filter(|v| v.is_string()) {
                    out["nextStep"] = next.clone();
                }
                if let Some(pin) = pins.get(id) {
                    out["pinned"] = json!({"actor": pin["actor"], "pinnedAt": pin["pinnedAt"]});
                }
                if let Some(a) = assignments.get(id) {
                    out["assignment"] = json!({
                        "assignee": a["assignee"],
                        "assignedBy": a["assignedBy"],
                        "assignedAt": a["assignedAt"],
                    });
                }
                Ok(out)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(json!({
            "workspaceId": workspace["id"],
            "generatedAt": self.clock.now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "total": total,
            "shown": shown.len(),
            "records": shown,
        }))
    }
}
