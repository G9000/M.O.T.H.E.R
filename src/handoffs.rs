//! Addressed handoffs: an open task named for one harness, found when that harness reads context.
//!
//! The harness is named by its grant (client ID). Nothing here runs work: the
//! receiving harness picks the task up under its own permissions and moves it
//! through the ordinary task statuses. Assignments live in an additive table,
//! so an older executable still reads every record.
use crate::{
    access::{self, AccessMode, ClientAccess},
    contracts::{self, Auth, string, valid_client},
    error::{Error, Result, invalid},
    store::{self, Store},
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::{Value, json};
use std::collections::HashSet;

const SHOWN: usize = 10;
const CONTENT_CHARS: usize = 600;
const OPEN: &str = "status IN ('todo','doing','blocked')";
const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS assignments(record_id TEXT PRIMARY KEY, assignee TEXT NOT NULL, assigned_by TEXT NOT NULL, assigned_at TEXT NOT NULL)";

fn exists(conn: &Connection) -> Result<bool> {
    Ok(conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='assignments')",
        [],
        |r| r.get(0),
    )?)
}

/// Active harness names a task could be handed to; shared clients are not told.
pub(crate) fn harnesses(conn: &Connection, access: &ClientAccess) -> Result<Option<Value>> {
    if access.mode == AccessMode::Shared {
        return Ok(None);
    }
    let mut statement = conn.prepare("SELECT id FROM clients WHERE revoked=0 ORDER BY id")?;
    let names = statement
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(Some(json!(names)))
}

/// Open tasks handed to this client, oldest handoff first, limited to what it may see.
pub(crate) fn assigned_to(
    conn: &Connection,
    client: &str,
    access: &ClientAccess,
) -> Result<(Value, Value)> {
    if !exists(conn)? {
        return Ok((json!([]), json!({"total":0,"shown":0})));
    }
    // The assignments table has no id column, so the policy filter applies to records.
    let filter = format!(
        "a.assignee=?1 AND r.kind='task' AND r.{OPEN} AND {}",
        access.filter()
    );
    let total: i64 = conn.query_row(
        &format!(
            "SELECT COUNT(*) FROM assignments a JOIN records r ON r.id=a.record_id WHERE {filter}"
        ),
        [client],
        |r| r.get(0),
    )?;
    let mut statement = conn.prepare(&format!(
        "SELECT r.payload,a.assigned_by,a.assigned_at FROM assignments a JOIN records r ON r.id=a.record_id WHERE {filter} ORDER BY a.assigned_at,a.record_id LIMIT {SHOWN}"
    ))?;
    let rows = statement
        .query_map([client], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut shown = Vec::with_capacity(rows.len());
    for (payload, by, at) in rows {
        let r = contracts::parse_record(serde_json::from_str(&payload)?)?;
        let content = string(&r, "content")?;
        let text: String = content.chars().take(CONTENT_CHARS).collect();
        shown.push(json!({"id":r["id"],"title":r["title"],"status":r["status"],"revision":r["revision"],"updatedAt":r["updatedAt"],"content":text,"truncated":text.len() < content.len(),"assignedBy":by,"assignedAt":at}));
    }
    let count = shown.len();
    Ok((Value::Array(shown), json!({"total":total,"shown":count})))
}

/// All assignments, for a portable export. `None` when nothing was ever handed off.
pub(crate) fn export(conn: &Connection) -> Result<Option<Value>> {
    if !exists(conn)? {
        return Ok(None);
    }
    let mut statement = conn.prepare(
        "SELECT record_id,assignee,assigned_by,assigned_at FROM assignments ORDER BY record_id",
    )?;
    let rows = statement
        .query_map([], |r| {
            Ok(json!({"recordId":r.get::<_, String>(0)?,"assignee":r.get::<_, String>(1)?,"assignedBy":r.get::<_, String>(2)?,"assignedAt":r.get::<_, String>(3)?}))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(Some(Value::Array(rows)))
}

/// Check an export's assignments against its records before anything is restored.
pub(crate) fn validate(assignments: &Value, records: &[Value]) -> Result<()> {
    let list = assignments
        .as_array()
        .ok_or_else(|| invalid("Invalid assignments"))?;
    if list.len() > 10_000 {
        return Err(invalid("Too many assignments"));
    }
    let mut seen = HashSet::new();
    for a in list {
        contracts::strict(a, &["recordId", "assignee", "assignedBy", "assignedAt"])?;
        let id = string(a, "recordId")?;
        contracts::id(id)?;
        if !valid_client(string(a, "assignee")?) {
            return Err(invalid("Invalid assignee"));
        }
        let by = string(a, "assignedBy")?;
        if by.is_empty() || by.encode_utf16().count() > 80 {
            return Err(invalid("Invalid assigner"));
        }
        contracts::timestamp(string(a, "assignedAt")?)?;
        let is_task = records.iter().any(|r| r["id"] == id && r["kind"] == "task");
        if !is_task || !seen.insert(id) {
            return Err(Error::domain(
                "INVALID_EXPORT",
                "Assignment references a missing, non-task or duplicate record",
            ));
        }
    }
    Ok(())
}

/// Write validated assignments into a fresh destination inside the restore transaction.
pub(crate) fn restore(conn: &Connection, assignments: &Value) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    for a in assignments
        .as_array()
        .ok_or_else(|| invalid("Invalid assignments"))?
    {
        conn.execute(
            "INSERT INTO assignments VALUES (?,?,?,?)",
            params![
                string(a, "recordId")?,
                string(a, "assignee")?,
                string(a, "assignedBy")?,
                string(a, "assignedAt")?
            ],
        )?;
    }
    Ok(())
}

/// Validate and apply an assignment inside the caller's transaction; true if it changed.
///
/// The caller has already checked its own authority. `by` is recorded as the assigner.
pub(crate) fn set(tx: &Connection, id: &str, assignee: Option<&str>, by: &str) -> Result<bool> {
    let (record, _) = store::read_record(tx, id)?;
    if record["kind"] != "task" {
        return Err(invalid("Only tasks can be handed to a harness"));
    }
    if let Some(name) = assignee {
        if !matches!(
            record["status"].as_str(),
            Some("todo" | "doing" | "blocked")
        ) {
            return Err(invalid(
                "Only open tasks (todo, doing or blocked) can be handed off",
            ));
        }
        let revoked: Option<i64> = tx
            .query_row("SELECT revoked FROM clients WHERE id=?", [name], |r| {
                r.get(0)
            })
            .optional()?;
        if revoked != Some(0) {
            return Err(invalid(
                "No current harness has that name; use one listed in context",
            ));
        }
        if !access::load(tx, name)?.allows(id) {
            return Err(invalid("That harness is not allowed to read this task"));
        }
    }
    tx.execute_batch(SCHEMA)?;
    let current: Option<String> = tx
        .query_row(
            "SELECT assignee FROM assignments WHERE record_id=?",
            [id],
            |r| r.get(0),
        )
        .optional()?;
    let changed = current.as_deref() != assignee;
    if changed {
        match assignee {
            Some(name) => {
                tx.execute(
                    "INSERT INTO assignments VALUES (?1,?2,?3,?4) ON CONFLICT(record_id) DO UPDATE SET assignee=excluded.assignee,assigned_by=excluded.assigned_by,assigned_at=excluded.assigned_at",
                    params![id, name, by, store::now()],
                )?;
            }
            None => {
                tx.execute("DELETE FROM assignments WHERE record_id=?", [id])?;
            }
        }
    }
    Ok(changed)
}

impl Store {
    /// Hand an open task to a named harness, move it to another, or clear it with `null`.
    ///
    /// Requires `task:write`. The receiving harness must hold a current grant
    /// that can read the task. Repeating a request is harmless.
    pub fn assign(&mut self, auth: &Auth, request: Value) -> Result<Value> {
        contracts::strict(&request, &["id", "assignee"])?;
        let id = string(&request, "id")?;
        contracts::id(id)?;
        let assignee = match request.get("assignee") {
            Some(Value::Null) => None,
            Some(Value::String(name)) if valid_client(name) => Some(name.clone()),
            _ => {
                return Err(invalid(
                    "assignee must be a harness name from context, or null to clear",
                ));
            }
        };
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        store::authorize(&tx, auth, "task:write")?;
        access::load(&tx, &auth.client_id)?.require(id)?;
        let changed = set(&tx, id, assignee.as_deref(), &auth.client_id)?;
        tx.commit()?;
        Ok(json!({"id":id,"assignee":assignee,"changed":changed}))
    }
}
