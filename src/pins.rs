//! Pinned core memories: a small owner-endorsed set that every context read includes.
//!
//! Pins live in an additive table rather than as a record field, so an older
//! executable that does not know about pins can still read every record.
use crate::{
    access::{self, ClientAccess},
    contracts::{self, Auth, Kind, string},
    error::{Error, Result, invalid},
    store::{self, Store},
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::{Value, json};
use std::collections::HashSet;

/// Most pins a workspace can hold, so the pinned core stays small enough to read every time.
pub(crate) const MAX_PINS: usize = 12;
const CONTENT_UNITS: usize = 2000;
const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS pins(record_id TEXT PRIMARY KEY, actor TEXT NOT NULL, pinned_at TEXT NOT NULL)";
const RECALLABLE: &str = "status NOT IN ('retracted','superseded','cancelled')";

fn exists(conn: &Connection) -> Result<bool> {
    Ok(conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='pins')",
        [],
        |r| r.get(0),
    )?)
}
fn pinnable(kind: &str) -> bool {
    matches!(kind, "memory" | "decision")
}
fn bounded(content: &str) -> (String, bool) {
    let mut used = 0;
    let text: String = content
        .chars()
        .take_while(|c| {
            used += c.len_utf16();
            used <= CONTENT_UNITS
        })
        .collect();
    let truncated = text.len() < content.len();
    (text, truncated)
}

/// The pinned section of a context read, limited to records this client may see.
pub(crate) fn context(conn: &Connection, access: &ClientAccess) -> Result<(Value, Value)> {
    let summary = |total: usize, shown: usize| json!({"total":total,"shown":shown,"inactive":total - shown,"limit":MAX_PINS});
    if !exists(conn)? {
        return Ok((json!([]), summary(0, 0)));
    }
    let mut statement = conn.prepare(&format!(
        "SELECT r.payload, r.status NOT IN ('retracted','superseded','cancelled'), p.actor, p.pinned_at FROM pins p JOIN records r ON r.id=p.record_id WHERE {} ORDER BY p.pinned_at, p.record_id",
        // The pins table has no id column, so the policy filter applies to records.
        access.filter()
    ))?;
    let rows = statement
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, bool>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let total = rows.len();
    let mut shown = Vec::new();
    for (payload, recallable, actor, pinned_at) in rows {
        if !recallable {
            continue;
        }
        let r = contracts::parse_record(serde_json::from_str(&payload)?)?;
        let (content, truncated) = bounded(string(&r, "content")?);
        shown.push(json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"updatedAt":r["updatedAt"],"content":content,"truncated":truncated,"pinnedBy":actor,"pinnedAt":pinned_at}));
    }
    let count = shown.len();
    Ok((Value::Array(shown), summary(total, count)))
}

/// All pins, for a portable export. `None` when the workspace has never pinned anything.
pub(crate) fn export(conn: &Connection) -> Result<Option<Value>> {
    if !exists(conn)? {
        return Ok(None);
    }
    let mut statement =
        conn.prepare("SELECT record_id,actor,pinned_at FROM pins ORDER BY record_id")?;
    let pins = statement
        .query_map([], |r| {
            Ok(json!({"recordId":r.get::<_, String>(0)?,"actor":r.get::<_, String>(1)?,"pinnedAt":r.get::<_, String>(2)?}))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(Some(Value::Array(pins)))
}

/// Check an export's pins against the records it carries, before anything is restored.
pub(crate) fn validate(pins: &Value, records: &[Value]) -> Result<()> {
    let list = pins.as_array().ok_or_else(|| invalid("Invalid pins"))?;
    if list.len() > MAX_PINS {
        return Err(invalid("Too many pins"));
    }
    let mut seen = HashSet::new();
    for pin in list {
        contracts::strict(pin, &["recordId", "actor", "pinnedAt"])?;
        let id = string(pin, "recordId")?;
        contracts::id(id)?;
        let actor = string(pin, "actor")?;
        if actor.is_empty() || actor.encode_utf16().count() > 80 {
            return Err(invalid("Invalid pin actor"));
        }
        contracts::timestamp(string(pin, "pinnedAt")?)?;
        let record = records
            .iter()
            .find(|r| r["id"] == id)
            .ok_or_else(|| Error::domain("INVALID_EXPORT", "Pin references a missing record"))?;
        if !record["kind"].as_str().is_some_and(pinnable) || !seen.insert(id) {
            return Err(Error::domain("INVALID_EXPORT", "Invalid or duplicate pin"));
        }
    }
    Ok(())
}

/// Write validated pins into a fresh destination inside the restore transaction.
pub(crate) fn restore(conn: &Connection, pins: &Value) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    for pin in pins.as_array().ok_or_else(|| invalid("Invalid pins"))? {
        conn.execute(
            "INSERT INTO pins VALUES (?,?,?)",
            params![
                string(pin, "recordId")?,
                string(pin, "actor")?,
                string(pin, "pinnedAt")?
            ],
        )?;
    }
    Ok(())
}

/// Apply a pin change inside the caller's transaction, after the caller checked its own authority.
pub(crate) fn set(tx: &Connection, id: &str, pinned: bool, actor: &str) -> Result<Value> {
    let (record, _) = store::read_record(tx, id)?;
    let kind = string(&record, "kind")?;
    if !pinnable(kind) {
        return Err(invalid("Only memories and decisions can be pinned"));
    }
    tx.execute_batch(SCHEMA)?;
    let current: Option<(String, String)> = tx
        .query_row(
            "SELECT actor,pinned_at FROM pins WHERE record_id=?",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    let changed = current.is_some() != pinned;
    let (actor, pinned_at) = if pinned {
        if let Some(existing) = current {
            existing
        } else {
            let recallable: bool = tx.query_row(
                &format!("SELECT {RECALLABLE} FROM records WHERE id=?"),
                [id],
                |r| r.get(0),
            )?;
            if !recallable {
                return Err(invalid(
                    "Retracted, superseded or cancelled records cannot be pinned",
                ));
            }
            let count: i64 = tx.query_row("SELECT COUNT(*) FROM pins", [], |r| r.get(0))?;
            if usize::try_from(count).unwrap_or(usize::MAX) >= MAX_PINS {
                return Err(Error::domain(
                    "RESOURCE_LIMIT",
                    format!("At most {MAX_PINS} records can be pinned. Unpin one first."),
                ));
            }
            let stamp = store::now();
            tx.execute("INSERT INTO pins VALUES (?,?,?)", params![id, actor, stamp])?;
            (actor.to_owned(), stamp)
        }
    } else {
        tx.execute("DELETE FROM pins WHERE record_id=?", [id])?;
        (String::new(), String::new())
    };
    let mut result = json!({"id":id,"kind":kind,"pinned":pinned,"changed":changed});
    if pinned {
        result["pinnedBy"] = json!(actor);
        result["pinnedAt"] = json!(pinned_at);
    }
    Ok(result)
}

impl Store {
    /// Pin or unpin a memory or decision so every context read includes it.
    ///
    /// Requires the write scope for that record's kind. Repeating a request is
    /// harmless: the response says whether anything changed.
    pub fn pin(&mut self, auth: &Auth, request: Value) -> Result<Value> {
        contracts::strict(&request, &["id", "pinned"])?;
        let id = string(&request, "id")?;
        contracts::id(id)?;
        let pinned = request
            .get("pinned")
            .and_then(Value::as_bool)
            .ok_or_else(|| invalid("pinned must be true or false"))?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        store::authorize(&tx, auth, "read")?;
        let access = access::load(&tx, &auth.client_id)?;
        access.require(id)?;
        let (record, _) = store::read_record(&tx, id)?;
        let kind = string(&record, "kind")?;
        store::authorize(&tx, auth, &Kind::parse(kind)?.scope())?;
        if !pinnable(kind) {
            return Err(invalid("Only memories and decisions can be pinned"));
        }
        let result = set(&tx, id, pinned, &auth.client_id)?;
        tx.commit()?;
        Ok(result)
    }
    /// Pin or unpin as the local owner, who needs no grant. The same rules apply.
    pub fn owner_pin(&mut self, id: &str, pinned: bool) -> Result<Value> {
        contracts::id(id)?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let result = set(&tx, id, pinned, crate::tend::OWNER)?;
        tx.commit()?;
        Ok(result)
    }
    /// Every pin with its record's current state, for the owner.
    pub fn owner_pins(&mut self) -> Result<Value> {
        let tx = self.conn.transaction()?;
        let mut pins = Vec::new();
        if exists(&tx)? {
            let mut statement = tx.prepare(&format!(
                "SELECT r.payload, r.{RECALLABLE}, p.actor, p.pinned_at FROM pins p JOIN records r ON r.id=p.record_id ORDER BY p.pinned_at, p.record_id"
            ))?;
            let rows = statement
                .query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, bool>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                    ))
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            for (payload, active, actor, at) in rows {
                let r = contracts::parse_record(serde_json::from_str(&payload)?)?;
                pins.push(json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"active":active,"pinnedBy":actor,"pinnedAt":at}));
            }
        }
        drop(tx);
        Ok(json!({"limit":MAX_PINS,"pins":pins}))
    }
}
