//! Owner tending: the person at this computer lists records and manages handoffs
//! directly, without a harness or a grant.
//!
//! Like the review inbox, these methods are reachable only from the owner CLI
//! and terminal, never from MCP. Owner pins and history live beside the
//! harness versions in their own modules and share the same rules.
use crate::{
    contracts::{self, Kind, integer, string},
    error::{Result, invalid},
    handoffs,
    store::{self, Store},
};
use rusqlite::{TransactionBehavior, params};
use serde_json::{Value, json};

/// Actor recorded when the owner pins or hands off from this computer.
pub const OWNER: &str = "@owner";
const DEFAULT_LIMIT: u64 = 200;
const MAX_LIMIT: u64 = 500;
const EXCERPT_CHARS: usize = 160;

fn has_table(conn: &rusqlite::Connection, name: &str) -> Result<bool> {
    Ok(conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?)",
        [name],
        |r| r.get(0),
    )?)
}

impl Store {
    /// Hand an open task to a harness, move it, or clear it with `None`, as the owner.
    pub fn owner_assign(&mut self, id: &str, assignee: Option<&str>) -> Result<Value> {
        contracts::id(id)?;
        if assignee.is_some_and(|name| !contracts::valid_client(name)) {
            return Err(invalid("assignee must be a harness name"));
        }
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let changed = handoffs::set(&tx, id, assignee, OWNER)?;
        tx.commit()?;
        Ok(json!({"id":id,"assignee":assignee,"changed":changed}))
    }

    /// Every handoff with its task's current state, and the harnesses a task can go to.
    pub fn owner_handoffs(&mut self) -> Result<Value> {
        let tx = self.conn.transaction()?;
        let mut names = tx.prepare("SELECT id FROM clients WHERE revoked=0 ORDER BY id")?;
        let harnesses = names
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        drop(names);
        let mut list = Vec::new();
        if has_table(&tx, "assignments")? {
            let mut statement = tx.prepare(
                "SELECT r.payload,a.assignee,a.assigned_by,a.assigned_at FROM assignments a JOIN records r ON r.id=a.record_id ORDER BY a.assigned_at,a.record_id",
            )?;
            let rows = statement
                .query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                    ))
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            for (payload, assignee, by, at) in rows {
                let r = contracts::parse_record(serde_json::from_str(&payload)?)?;
                let open = matches!(r["status"].as_str(), Some("todo" | "doing" | "blocked"));
                list.push(json!({"id":r["id"],"title":r["title"],"status":r["status"],"revision":r["revision"],"open":open,"assignee":assignee,"assignedBy":by,"assignedAt":at}));
            }
        }
        drop(tx);
        Ok(json!({"harnesses":harnesses,"handoffs":list}))
    }

    /// List records with their pin and handoff state, newest first, for the owner.
    ///
    /// Accepts optional `kind`, `query` (a case-insensitive substring of title or
    /// content) and `limit` (1 to 500, default 200). Retired records are included
    /// and marked not recallable.
    pub fn owner_records(&mut self, query: Value) -> Result<Value> {
        contracts::strict(&query, &["kind", "query", "limit"])?;
        let kind = query
            .get("kind")
            .map(|_| string(&query, "kind"))
            .transpose()?;
        if let Some(k) = kind {
            Kind::parse(k)?;
        }
        let needle = query
            .get("query")
            .map(|_| string(&query, "query"))
            .transpose()?;
        if needle.is_some_and(|q| q.encode_utf16().count() > 200) {
            return Err(invalid("Query too long"));
        }
        let limit = query
            .get("limit")
            .map(|_| integer(&query, "limit", 1, MAX_LIMIT))
            .transpose()?
            .unwrap_or(DEFAULT_LIMIT);
        let tx = self.conn.transaction()?;
        let pins = has_table(&tx, "pins")?;
        let assignments = has_table(&tx, "assignments")?;
        let filter = "(?1 IS NULL OR r.kind=?1) AND (?2 IS NULL OR instr(lower(r.title || ' ' || r.content),lower(?2))>0)";
        let total: i64 = tx.query_row(
            &format!("SELECT COUNT(*) FROM records r WHERE {filter}"),
            params![kind, needle],
            |r| r.get(0),
        )?;
        let pinned = if pins {
            "EXISTS(SELECT 1 FROM pins p WHERE p.record_id=r.id)"
        } else {
            "0"
        };
        let assignee = if assignments {
            "(SELECT a.assignee FROM assignments a WHERE a.record_id=r.id)"
        } else {
            "NULL"
        };
        let mut statement = tx.prepare(&format!(
            "SELECT r.payload,{pinned},{assignee} FROM records r WHERE {filter} ORDER BY r.updated_at DESC,r.id LIMIT ?3"
        ))?;
        let rows = statement
            .query_map(
                params![
                    kind,
                    needle,
                    i64::try_from(limit).map_err(|_| invalid("Invalid limit"))?
                ],
                |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, bool>(1)?,
                        r.get::<_, Option<String>>(2)?,
                    ))
                },
            )?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        drop(statement);
        let mut records = Vec::with_capacity(rows.len());
        for (payload, pinned, assignee) in rows {
            let r = contracts::parse_record(serde_json::from_str(&payload)?)?;
            let content = string(&r, "content")?;
            let excerpt: String = content.chars().take(EXCERPT_CHARS).collect();
            let recallable = !matches!(
                r["status"].as_str(),
                Some("retracted" | "superseded" | "cancelled")
            );
            records.push(json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"actor":r["actor"],"updatedAt":r["updatedAt"],"excerpt":excerpt,"truncated":excerpt.len() < content.len(),"recallable":recallable,"pinned":pinned,"assignee":assignee}));
        }
        let workspace = store::workspace(&tx)?;
        drop(tx);
        let shown = records.len() as i64;
        Ok(json!({"workspace":workspace,"records":records,"omitted":total - shown}))
    }
}
