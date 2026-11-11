//! Canonical SQLite state, atomic mutations, and client capability enforcement.
use crate::{
    access::{self, AccessMode, ClientAccess},
    config::{ClientConfig, create_private_file},
    contracts::{self, Auth, Kind, SCOPES},
    error::{Error, Result, invalid},
    limits::Usage,
};
use chrono::{SecondsFormat, Utc};
use rand::RngCore;
use rusqlite::{Connection, OpenFlags, OptionalExtension, TransactionBehavior, params};
use serde_json::{Value, json};
use std::{fs, io::Write as _, path::Path, time::Duration};
use subtle::ConstantTimeEq;
const SCHEMA: &str = include_str!("../contracts/schema-v1.sql");
/// A local connection to the single canonical v1 context store.
#[derive(Debug)]
pub struct Store {
    pub(crate) conn: Connection,
}
pub(crate) fn metadata(conn: &Connection, key: &str) -> Result<Value> {
    let raw: String = conn.query_row("SELECT value FROM metadata WHERE key=?", [key], |r| {
        r.get(0)
    })?;
    Ok(serde_json::from_str(&raw)?)
}
pub(crate) fn set_metadata(conn: &Connection, key: &str, value: &Value) -> Result<()> {
    conn.execute(
        "UPDATE metadata SET value=? WHERE key=?",
        params![value.to_string(), key],
    )?;
    Ok(())
}
pub(crate) fn workspace(conn: &Connection) -> Result<Value> {
    let mut v = metadata(conn, "workspace")?;
    contracts::validate_workspace(&v)?;
    v["schemaVersion"] = json!(1);
    Ok(v)
}
pub(crate) fn persona(conn: &Connection) -> Result<Value> {
    let v = metadata(conn, "persona")?;
    contracts::validate_persona(&v)?;
    Ok(v)
}
pub(crate) fn now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
pub(crate) fn authorize(conn: &Connection, auth: &Auth, scope: &str) -> Result<()> {
    let row: Option<(String, String, i64)> = conn
        .query_row(
            "SELECT token_hash,scopes,revoked FROM clients WHERE id=?",
            [&auth.client_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()?;
    let Some((token, scopes, revoked)) = row else {
        return Err(Error::domain(
            "UNAUTHORIZED",
            "Client is unknown, revoked, or has an invalid token.",
        ));
    };
    if revoked != 0
        || !bool::from(
            token
                .as_bytes()
                .ct_eq(contracts::hash(&auth.token).as_bytes()),
        )
    {
        return Err(Error::domain(
            "UNAUTHORIZED",
            "Client is unknown, revoked, or has an invalid token.",
        ));
    }
    let scopes: Vec<String> = serde_json::from_str(&scopes)?;
    if !scopes.iter().any(|s| s == scope) {
        return Err(Error::domain(
            "FORBIDDEN",
            format!("This client needs {scope}."),
        ));
    }
    if access::load(conn, &auth.client_id)?.mode == AccessMode::Shared
        && !matches!(scope, "read" | "context:propose")
    {
        return Err(Error::domain(
            "FORBIDDEN",
            "Shared clients cannot write canonical records.",
        ));
    }
    Ok(())
}
fn grant(conn: &Connection, client: &str, scopes: &[String]) -> Result<Value> {
    if !contracts::valid_client(client)
        || scopes.is_empty()
        || scopes.iter().any(|s| !SCOPES.contains(&s.as_str()))
    {
        return Err(invalid("Invalid client name or scopes"));
    }
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let token = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let unique: Vec<&str> = scopes
        .iter()
        .map(String::as_str)
        .fold(Vec::new(), |mut acc, s| {
            if !acc.contains(&s) {
                acc.push(s);
            }
            acc
        });
    conn.execute(
        "INSERT INTO clients(id,token_hash,scopes) VALUES (?,?,?)",
        params![
            client,
            contracts::hash(&token),
            serde_json::to_string(&unique)?
        ],
    )?;
    Ok(json!({"clientId":client,"token":token,"scopes":scopes}))
}
pub(crate) fn read_record(conn: &Connection, id: &str) -> Result<(Value, usize)> {
    let row: Option<String> = conn
        .query_row("SELECT payload FROM records WHERE id=?", [id], |r| r.get(0))
        .optional()?;
    let raw = row.ok_or_else(|| Error::domain("NOT_FOUND", "Record does not exist."))?;
    let len = raw.len();
    Ok((contracts::parse_record(serde_json::from_str(&raw)?)?, len))
}
pub(crate) fn insert_record(conn: &Connection, r: &Value) -> Result<()> {
    conn.execute("INSERT INTO records VALUES (?,?,?,?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET kind=excluded.kind,status=excluded.status,revision=excluded.revision,updated_at=excluded.updated_at,title=excluded.title,content=excluded.content,payload=excluded.payload",params![contracts::string(r,"id")?,contracts::string(r,"kind")?,contracts::string(r,"status")?,i64::try_from(contracts::integer(r,"revision",1,9007199254740991)?).map_err(|_|invalid("Revision exceeds SQLite range"))?,contracts::string(r,"updatedAt")?,contracts::string(r,"title")?,contracts::string(r,"content")?,r.to_string()])?;
    Ok(())
}
pub(crate) fn rebuild_usage(conn: &Connection) -> Result<()> {
    let (records, record_bytes): (i64, i64) = conn.query_row(
        "SELECT COUNT(*),COALESCE(SUM(length(CAST(payload AS BLOB))),0) FROM records",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    let (operations, operation_bytes): (i64, i64) = conn.query_row(
        "SELECT COUNT(*),COALESCE(SUM(length(CAST(receipt AS BLOB))),0) FROM operations",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    let count = |n: i64| u64::try_from(n).map_err(|_| invalid("Invalid storage counters"));
    let usage = Usage {
        records: count(records)?,
        operations: count(operations)?,
        payload_bytes: count(record_bytes)? + count(operation_bytes)?,
    };
    usage.check()?;
    set_metadata(conn, "usage", &serde_json::to_value(usage)?)
}
/// Tasks and actions whose outcome is still open.
const UNSETTLED: &str = "((kind='task' AND status IN ('doing','blocked')) OR (kind='action' AND status IN ('pending','unknown')))";
fn kind_counts() -> Value {
    json!({"memory":0,"decision":0,"task":0,"checkpoint":0,"artifact":0,"action":0})
}
fn context_inventory(conn: &Connection, access: &ClientAccess) -> Result<Value> {
    let mut by_kind = kind_counts();
    let mut recallable_by_kind = by_kind.clone();
    let mut total = 0i64;
    let mut recallable = 0i64;
    let mut statement = conn.prepare(&format!("SELECT kind,COUNT(*),SUM(CASE WHEN status NOT IN ('retracted','superseded','cancelled') THEN 1 ELSE 0 END) FROM records WHERE {} GROUP BY kind", access.filter()))?;
    for row in statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })? {
        let (kind, count, available) = row?;
        Kind::parse(&kind)?;
        by_kind[&kind] = json!(count);
        recallable_by_kind[&kind] = json!(available);
        total += count;
        recallable += available;
    }
    Ok(
        json!({"totalRecords":total,"byKind":by_kind,"recallableRecords":recallable,"recallableByKind":recallable_by_kind,"excludedRecords":total-recallable}),
    )
}

impl Store {
    /// Create a new store; an existing destination is never overwritten.
    pub fn initialize(path: &Path, content: &str) -> Result<Self> {
        Self::initialize_with(path, content, |_| Ok(()))
    }
    pub(crate) fn initialize_with(
        path: &Path,
        content: &str,
        populate: impl FnOnce(&Connection) -> Result<()>,
    ) -> Result<Self> {
        let p = json!({"content":content,"hash":contracts::hash(content)});
        contracts::validate_persona(&p)?;
        drop(create_private_file(path)?);
        let build = || -> Result<Self> {
            let mut conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
            conn.busy_timeout(Duration::from_secs(5))?;
            conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;")?;
            let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
            tx.execute_batch(SCHEMA)?;
            let w = json!({"id":uuid::Uuid::new_v4().to_string(),"generation":uuid::Uuid::new_v4().to_string(),"schemaVersion":1,"createdAt":now()});
            for (key, value) in [
                ("workspace", w),
                ("persona", p),
                (
                    "usage",
                    json!({"records":0,"operations":0,"payloadBytes":0}),
                ),
            ] {
                tx.execute(
                    "INSERT INTO metadata VALUES (?,?)",
                    params![key, value.to_string()],
                )?;
            }
            populate(&tx)?;
            tx.commit()?;
            Ok(Self { conn })
        };
        let result = build();
        if result.is_err() {
            for suffix in ["", "-wal", "-shm"] {
                let mut name = path.as_os_str().to_os_string();
                name.push(suffix);
                let _ = fs::remove_file(Path::new(&name));
            }
        }
        result
    }
    /// Open the existing schema without migration, replacement, or identity regeneration.
    pub fn open(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(Error::domain(
                "NOT_INITIALIZED",
                "Initialize this workspace before connecting.",
            ));
        }
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.execute_batch("PRAGMA synchronous=FULL;")?;
        let version: u32 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if version != 1 {
            return Err(Error::domain(
                "SCHEMA_MISMATCH",
                "Unsupported or incomplete database version.",
            ));
        }
        let s = Self { conn };
        s.workspace()?;
        s.persona()?;
        Ok(s)
    }
    /// Retrieve stable workspace identity and current restore generation.
    pub fn workspace(&self) -> Result<Value> {
        workspace(&self.conn)
    }
    /// Retrieve the current personality snapshot and verified digest.
    pub fn persona(&self) -> Result<Value> {
        persona(&self.conn)
    }
    /// Replace the owner's persona atomically if its current content hash still matches.
    ///
    /// This owner-administration operation is not exposed through MCP grants.
    /// Returns `CONFLICT` for a stale hash, including retries after a successful change.
    /// Only persona metadata changes; no record receipt or persona history is created.
    pub fn update_persona(&mut self, content: &str, expected_hash: &str) -> Result<Value> {
        if !contracts::hex_digest(expected_hash) {
            return Err(invalid(
                "Expected persona hash must be a lowercase SHA-256 digest",
            ));
        }
        let new_persona = json!({"content":content,"hash":contracts::hash(content)});
        contracts::validate_persona(&new_persona)?;
        if content.trim_start_matches('\u{feff}').trim().is_empty() {
            return Err(invalid("Persona must contain nonblank text"));
        }
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let current = persona(&tx)?;
        if current["hash"].as_str() != Some(expected_hash) {
            return Err(Error::domain(
                "CONFLICT",
                "Persona changed. Read soul show and review the current version before updating.",
            ));
        }
        let changed = current != new_persona;
        if changed {
            set_metadata(&tx, "persona", &new_persona)?;
        }
        let result = json!({"workspace":workspace(&tx)?,"previousHash":current["hash"],"hash":new_persona["hash"],"changed":changed});
        tx.commit()?;
        Ok(result)
    }
    /// Check the current grant; previously connected clients are not exempt.
    pub fn authorize(&self, auth: &Auth, scope: &str) -> Result<()> {
        authorize(&self.conn, auth, scope)
    }
    /// Verify current read authority and return its owner-selected visibility policy.
    pub fn access_for(&self, auth: &Auth) -> Result<ClientAccess> {
        self.authorize(auth, "read")?;
        access::load(&self.conn, &auth.client_id)
    }
    /// Provision a new local client. The returned token must be kept private.
    pub fn grant(&mut self, client: &str, scopes: &[String]) -> Result<Value> {
        self.grant_with_access(client, scopes, &ClientAccess::default())
    }
    /// Provision an explicitly scoped owner policy without changing existing grants.
    pub fn grant_with_access(
        &mut self,
        client: &str,
        scopes: &[String],
        access: &ClientAccess,
    ) -> Result<Value> {
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut g = grant(&tx, client, scopes)?;
        access::save(&tx, client, scopes, access)?;
        g["access"] = access.metadata();
        tx.commit()?;
        Ok(g)
    }
    /// Persist a grant and its exclusive private file, rolling back ordinary failures.
    pub fn grant_to_file(
        &mut self,
        client: &str,
        scopes: &[String],
        path: &Path,
        database_path: &Path,
    ) -> Result<()> {
        self.grant_to_file_with_access(
            client,
            scopes,
            path,
            database_path,
            &ClientAccess::default(),
        )
    }
    /// Persist a private credential with its owner-selected access policy atomically.
    pub fn grant_to_file_with_access(
        &mut self,
        client: &str,
        scopes: &[String],
        path: &Path,
        database_path: &Path,
        access: &ClientAccess,
    ) -> Result<()> {
        if !contracts::valid_client(client)
            || scopes.is_empty()
            || scopes.iter().any(|s| !SCOPES.contains(&s.as_str()))
        {
            return Err(invalid("Invalid client name or scopes"));
        }
        let mut file = create_private_file(path)?;
        let result = (|| -> Result<()> {
            let tx = self
                .conn
                .transaction_with_behavior(TransactionBehavior::Immediate)?;
            let g = grant(&tx, client, scopes)?;
            access::save(&tx, client, scopes, access)?;
            let config = ClientConfig {
                version: if access.mode == AccessMode::Legacy {
                    1
                } else {
                    2
                },
                database_path: database_path.to_path_buf(),
                client_id: client.into(),
                token: contracts::string(&g, "token")?.into(),
            };
            serde_json::to_writer_pretty(&mut file, &config)?;
            file.write_all(b"\n")?;
            file.sync_all()?;
            tx.commit()?;
            Ok(())
        })();
        drop(file);
        if result.is_err() {
            let _ = fs::remove_file(path);
        }
        result
    }
    /// Revoke access for subsequent requests from a named client.
    pub fn revoke(&mut self, client: &str) -> Result<()> {
        if self
            .conn
            .execute("UPDATE clients SET revoked=1 WHERE id=?", [client])?
            == 0
        {
            return Err(Error::domain("NOT_FOUND", "Unknown client."));
        }
        Ok(())
    }
    /// Read a complete current record, including explicit retractions.
    pub fn get(&mut self, auth: &Auth, id: &str) -> Result<Value> {
        contracts::id(id)?;
        let tx = self.conn.transaction()?;
        authorize(&tx, auth, "read")?;
        let access = access::load(&tx, &auth.client_id)?;
        access.require(id)?;
        let (mut r, _) = read_record(&tx, id)?;
        access.scrub(&mut r);
        tx.commit()?;
        Ok(r)
    }
    /// Retrieve this client's current-generation receipt after an uncertain response.
    pub fn receipt(&mut self, auth: &Auth, operation_id: &str) -> Result<Value> {
        contracts::id(operation_id)?;
        let tx = self.conn.transaction()?;
        authorize(&tx, auth, "read")?;
        let w = workspace(&tx)?;
        let raw: Option<String> = tx
            .query_row(
                "SELECT receipt FROM operations WHERE actor=? AND generation=? AND operation_id=?",
                params![
                    auth.client_id,
                    contracts::string(&w, "generation")?,
                    operation_id
                ],
                |r| r.get(0),
            )
            .optional()?;
        let raw = match raw {
            Some(raw) => Some(raw),
            None => crate::inbox::operation_receipt(
                &tx,
                &auth.client_id,
                contracts::string(&w, "generation")?,
                operation_id,
            )?,
        };
        let raw = raw.ok_or_else(|| {
            Error::domain(
                "NOT_FOUND",
                "No committed receipt for this client and generation.",
            )
        })?;
        let mut value = serde_json::from_str(&raw)?;
        access::load(&tx, &auth.client_id)?.receipt(&auth.client_id, &mut value)?;
        tx.commit()?;
        Ok(value)
    }
    /// Retrieve bounded record previews and unsettled work in a consistent read transaction.
    pub fn context(&mut self, auth: &Auth, query: Value) -> Result<Value> {
        contracts::strict(
            &query,
            &["query", "kind", "limit", "updatedSince", "updatedBefore"],
        )?;
        let (since, before) = contracts::time_window(&query)?;
        let q = query
            .get("query")
            .map(|_| contracts::string(&query, "query"))
            .transpose()?;
        if q.is_some_and(|q| q.encode_utf16().count() > 200) {
            return Err(invalid("Query too long"));
        }
        let kind = query
            .get("kind")
            .map(|_| contracts::string(&query, "kind"))
            .transpose()?;
        if let Some(k) = kind {
            Kind::parse(k)?;
        }
        let limit = if query.get("limit").is_some() {
            contracts::integer(&query, "limit", 1, 100)?
        } else {
            20
        };
        let tx = self.conn.transaction()?;
        authorize(&tx, auth, "read")?;
        let access = access::load(&tx, &auth.client_id)?;
        let inventory = context_inventory(&tx, &access)?;
        let filter = format!(
            "{} AND status NOT IN ('retracted','superseded','cancelled') AND (?1 IS NULL OR kind=?1) AND (?2 IS NULL OR instr(lower(title || ' ' || content),lower(?2))>0) AND (?3 IS NULL OR updated_at>=?3) AND (?4 IS NULL OR updated_at<?4)",
            access.filter()
        );
        let mut matching_by_kind = kind_counts();
        let mut total = 0i64;
        {
            let mut stmt = tx.prepare(&format!(
                "SELECT kind,COUNT(*) FROM records WHERE {filter} GROUP BY kind"
            ))?;
            for row in stmt.query_map(params![kind, q, since, before], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
            })? {
                let (name, count) = row?;
                Kind::parse(&name)?;
                matching_by_kind[&name] = json!(count);
                total += count;
            }
        }
        // Checkpoints are written every session. Without a filter, categories take
        // turns for preview slots so older memories and decisions stay visible.
        let balanced = q.is_none() && kind.is_none() && since.is_none() && before.is_none();
        let records = {
            let mut stmt = tx.prepare(&if balanced {
                format!(
                    "SELECT payload FROM (SELECT payload,updated_at,id,ROW_NUMBER() OVER (PARTITION BY kind ORDER BY {UNSETTLED} DESC,updated_at DESC,id) AS turn,CASE kind WHEN 'checkpoint' THEN 0 WHEN 'task' THEN 1 WHEN 'decision' THEN 2 WHEN 'memory' THEN 3 WHEN 'action' THEN 4 ELSE 5 END AS seat FROM records WHERE {filter} ORDER BY turn,seat LIMIT ?5) ORDER BY updated_at DESC,id"
                )
            } else {
                format!(
                    "SELECT payload FROM records WHERE {filter} ORDER BY updated_at DESC,id LIMIT ?5"
                )
            })?;
            let raw = stmt
                .query_map(
                    params![
                        kind,
                        q,
                        since,
                        before,
                        i64::try_from(limit).map_err(|_| invalid("Invalid limit"))?
                    ],
                    |r| r.get::<_, String>(0),
                )?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            raw.into_iter().map(|raw|->Result<Value>{let r=contracts::parse_record(serde_json::from_str(&raw)?)?;let content=contracts::string(&r,"content")?;
    let mut used=0;let excerpt:String=content.chars().take_while(|c|{used+=c.len_utf16();used<=400}).collect();
    Ok(json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"actor":r["actor"],"updatedAt":r["updatedAt"],"excerpt":excerpt,"truncated":content.encode_utf16().count()>400}))
   }).collect::<Result<Vec<_>>>()?
        };
        let unsettled_total: i64 = tx.query_row(
            &format!(
                "SELECT COUNT(*) FROM records WHERE {} AND {UNSETTLED}",
                access.filter()
            ),
            [],
            |row| row.get(0),
        )?;
        let mut work = {
            let mut stmt=tx.prepare(&format!("SELECT id,kind,status FROM records WHERE {} AND {UNSETTLED} ORDER BY updated_at DESC,id LIMIT 21",access.filter()))?;
            stmt.query_map([],|r|Ok(json!({"id":r.get::<_,String>(0)?,"kind":r.get::<_,String>(1)?,"status":r.get::<_,String>(2)?})))?.collect::<std::result::Result<Vec<_>,_>>()?
        };
        let truncated = work.len() > 20;
        work.truncate(20);
        let counts = &inventory["recallableByKind"];
        let order = if balanced {
            " Previews take turns across categories, newest first within each, so recent checkpoints do not displace older memories and decisions; selection.omittedByKind lists what was left out."
        } else {
            ""
        };
        let summary = format!(
            "Shared context has {} recallable records: {} memories, {} decisions, {} tasks, {} checkpoints, {} artifacts and {} actions. {} retained historical records are excluded from recall. This request matches {total} records and returns {} previews.{order}",
            inventory["recallableRecords"],
            counts["memory"],
            counts["decision"],
            counts["task"],
            counts["checkpoint"],
            counts["artifact"],
            counts["action"],
            inventory["excludedRecords"],
            records.len()
        );
        let mut returned_by_kind = kind_counts();
        let mut omitted_by_kind = matching_by_kind;
        for record in &records {
            let name = contracts::string(record, "kind")?;
            for (counts, step) in [(&mut returned_by_kind, 1), (&mut omitted_by_kind, -1)] {
                counts[name] = json!(counts[name].as_i64().unwrap_or(0) + step);
            }
        }
        let selection = json!({"query":q,"kind":kind,"updatedSince":since,"updatedBefore":before,"order":if balanced {"balanced"} else {"recent"},"matchingRecords":total,"returnedRecords":records.len(),"omittedRecords":total-records.len() as i64,"returnedByKind":returned_by_kind,"omittedByKind":omitted_by_kind});
        let unsettled_summary = json!({"total":unsettled_total,"returned":work.len(),"omitted":unsettled_total-work.len() as i64,"scope":"Workspace-wide tasks in doing/blocked and actions in pending/unknown. Checkpoint next steps and open questions are not counted as unsettled task/action records."});
        let mut result = json!({"workspace":workspace(&tx)?,"access":access.metadata(),"capturedAt":now(),"inventory":inventory,"selection":selection,"summary":summary,"omitted":total-records.len() as i64,"records":records,"unsettledWork":work,"unsettledWorkSummary":unsettled_summary,"unsettledWorkTruncated":truncated,"note":"Inventory counts cover all canonical record categories, independently of this request's query/kind filters and preview limit. Zero memory-category records is not an empty workspace when other categories contain records. Unreviewed inbox proposals are excluded. Excerpts are navigation, not complete records; retrieve IDs before relying on details or editing. Unsettled work counts only the task/action states described in unsettledWorkSummary. A handoff does not pause running jobs."});
        if access.mode != AccessMode::Shared {
            result["persona"] = persona(&tx)?;
        } else {
            result["selection"]["scope"] = json!("granted_records");
            result["unsettledWorkSummary"]["scope"] = json!(
                "Granted tasks in doing/blocked and actions in pending/unknown; hidden records are excluded."
            );
            result["note"] = json!(
                "Inventory, selection and unsettled-work counts cover only this client's granted canonical records. Unreviewed inbox proposals are excluded. Excerpts are navigation; retrieve full records before relying on them. A handoff does not pause running jobs."
            );
        }
        tx.commit()?;
        Ok(result)
    }
    /// Atomically validate a revision, mutate a record and persist its idempotent receipt.
    pub fn write(&mut self, auth: &Auth, raw: Value) -> Result<Value> {
        let input = contracts::Write::parse(raw)?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        authorize(&tx, auth, &input.kind.scope())?;
        let receipt = write_transaction(&tx, &auth.client_id, &input)?;
        tx.commit()?;
        Ok(receipt)
    }
}

pub(crate) fn write_transaction(
    tx: &Connection,
    actor: &str,
    input: &contracts::Write,
) -> Result<Value> {
    let v = &input.value;
    let w = workspace(tx)?;
    if v["workspaceId"] != w["id"] {
        return Err(Error::domain(
            "WORKSPACE_MISMATCH",
            "Fetch the current workspace.",
        ));
    }
    if v["generation"] != w["generation"] {
        return Err(Error::domain(
            "STALE_GENERATION",
            "Workspace was restored; reconnect and reconcile current records.",
        ));
    }
    let generation = contracts::string(v, "generation")?;
    let op = contracts::string(v, "operationId")?;
    let fingerprint = contracts::hash(&contracts::canonical(v));
    crate::inbox::reject_operation_collision(tx, actor, generation, op)?;
    let prior:Option<(String,String)>=tx.query_row("SELECT request_hash,receipt FROM operations WHERE actor=? AND generation=? AND operation_id=?",params![actor,generation,op],|r|Ok((r.get(0)?,r.get(1)?))).optional()?;
    if let Some((old_hash, receipt)) = prior {
        if old_hash != fingerprint {
            return Err(Error::domain(
                "OPERATION_REUSED",
                "An operation ID cannot be reused for a different request.",
            ));
        }
        let receipt = serde_json::from_str(&receipt)?;
        return Ok(receipt);
    }
    let previous = v
        .get("id")
        .map(|_| read_record(tx, contracts::string(v, "id")?))
        .transpose()?;
    if let Some((p, _)) = &previous {
        if p["revision"] != v["expectedRevision"] {
            return Err(Error::domain(
                "CONFLICT",
                "Record changed; retrieve it before proposing a new edit.",
            ));
        }
        if p["kind"] != v["record"]["kind"] {
            return Err(Error::domain("KIND_MISMATCH", "Record kind cannot change."));
        }
    }
    for id in v["record"]["references"]
        .as_array()
        .ok_or_else(|| invalid("Invalid references"))?
    {
        let found: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM records WHERE id=?)",
            [id.as_str().ok_or_else(|| invalid("Invalid reference"))?],
            |r| r.get(0),
        )?;
        if !found {
            return Err(Error::domain(
                "REFERENCE_NOT_FOUND",
                "Referenced record does not exist in this workspace.",
            ));
        }
    }
    let prior_time = previous
        .as_ref()
        .map(|(p, _)| contracts::timestamp(contracts::string(p, "updatedAt")?))
        .transpose()?;
    let clock = Utc::now();
    let stamp = prior_time
        .map_or(clock, |p| clock.max(p))
        .to_rfc3339_opts(SecondsFormat::Millis, true);
    let mut record = v["record"].clone();
    record["id"] = previous.as_ref().map_or_else(
        || json!(uuid::Uuid::new_v4().to_string()),
        |(p, _)| p["id"].clone(),
    );
    record["revision"] = json!(
        previous
            .as_ref()
            .map_or(Ok(0), |(p, _)| contracts::integer(
                p,
                "revision",
                1,
                9007199254740990
            ))?
            + 1
    );
    record["actor"] = json!(actor);
    record["createdAt"] = previous
        .as_ref()
        .map_or_else(|| json!(stamp), |(p, _)| p["createdAt"].clone());
    record["updatedAt"] = json!(stamp);
    let receipt =
        json!({"operationId":op,"generation":generation,"committedAt":stamp,"record":record});
    let mut usage: Usage = serde_json::from_value(metadata(tx, "usage")?)?;
    usage.records += u64::from(previous.is_none());
    usage.operations += 1;
    usage.payload_bytes = usage
        .payload_bytes
        .checked_sub(previous.as_ref().map_or(0, |(_, n)| *n as u64))
        .and_then(|n| {
            n.checked_add(record.to_string().len() as u64 + receipt.to_string().len() as u64)
        })
        .ok_or_else(|| Error::domain("CORRUPT_STORE", "Invalid usage accounting"))?;
    usage.check()?;
    insert_record(tx, &record)?;
    set_metadata(tx, "usage", &serde_json::to_value(usage)?)?;
    tx.execute(
        "INSERT INTO operations VALUES (?,?,?,?,?)",
        params![actor, generation, op, fingerprint, receipt.to_string()],
    )?;
    crate::inbox::check_capacity(tx)?;
    if let Some(target) = v.get("supersedes") {
        supersede(tx, actor, v, target)?;
    }
    Ok(receipt)
}
/// Retire the checkpoint a new one replaces, inside the caller's transaction.
///
/// The retirement is an ordinary attributed revision with its own receipt, so a
/// failure here rolls back the new checkpoint as well.
fn supersede(tx: &Connection, actor: &str, v: &Value, target: &Value) -> Result<()> {
    let id = contracts::string(target, "id")?;
    let (current, _) = read_record(tx, id)?;
    if current["kind"] != "checkpoint" {
        return Err(Error::domain(
            "KIND_MISMATCH",
            "Only a checkpoint can be superseded by a checkpoint.",
        ));
    }
    if current["revision"] != target["expectedRevision"] || current["status"] != "active" {
        return Err(Error::domain(
            "CONFLICT",
            "The checkpoint to supersede changed or is no longer active; read current context before saving.",
        ));
    }
    let mut draft = current;
    if let Some(fields) = draft.as_object_mut() {
        for key in ["id", "revision", "actor", "createdAt", "updatedAt"] {
            fields.remove(key);
        }
    }
    draft["status"] = json!("superseded");
    let retirement = contracts::Write::parse(
        json!({"workspaceId":v["workspaceId"],"generation":v["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"id":id,"expectedRevision":target["expectedRevision"],"record":draft}),
    )?;
    write_transaction(tx, actor, &retirement)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restore_crash_child() {
        let Some(path) = std::env::var_os("MOTHER_TEST_CRASH_PATH") else {
            return;
        };
        let _ = Store::initialize_with(Path::new(&path), "Synthetic crash fixture", |conn| {
            conn.execute(
                "INSERT INTO records VALUES ('x','memory','active',1,'x','x','x','{}')",
                [],
            )?;
            std::process::exit(73);
        });
        panic!("Expected termination before commit");
    }
    #[test]
    fn interrupted_restore_never_opens_as_empty_workspace() {
        let dir = tempfile::tempdir().expect("temporary directory");
        let path = dir.path().join("interrupted.sqlite");
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .args([
                "--exact",
                "store::tests::restore_crash_child",
                "--nocapture",
            ])
            .env("MOTHER_TEST_CRASH_PATH", &path)
            .status()
            .expect("run crash child");
        assert_eq!(status.code(), Some(73));
        assert_eq!(
            Store::open(&path)
                .expect_err("incomplete state must fail")
                .code(),
            "SCHEMA_MISMATCH"
        );
    }
}
