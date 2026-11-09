//! Durable, isolated capture proposals and local owner review.
use crate::{
    contracts::{self, Auth, Kind, Write, integer, string},
    error::{Error, Result, invalid},
    limits::Usage,
    store::{self, Store},
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::{Value, json};
const OWNER: &str = "@owner-review";
const MAX_PROPOSALS: usize = 10000;
const MAX_OPERATIONS: usize = 100000;
const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS inbox_proposals(id TEXT PRIMARY KEY, status TEXT NOT NULL, dedup_hash TEXT NOT NULL, payload TEXT NOT NULL); CREATE INDEX IF NOT EXISTS inbox_pending ON inbox_proposals(status,dedup_hash); CREATE TABLE IF NOT EXISTS inbox_operations(actor TEXT NOT NULL,generation TEXT NOT NULL,operation_id TEXT NOT NULL,family TEXT NOT NULL,request_hash TEXT NOT NULL,request TEXT NOT NULL,receipt TEXT NOT NULL,PRIMARY KEY(actor,generation,operation_id));";
fn exists(conn: &Connection) -> Result<bool> {
    Ok(conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='inbox_proposals')",
        [],
        |r| r.get(0),
    )?)
}
pub(crate) fn operation_receipt(
    conn: &Connection,
    actor: &str,
    generation: &str,
    op: &str,
) -> Result<Option<String>> {
    if !exists(conn)? {
        return Ok(None);
    }
    Ok(conn.query_row("SELECT receipt FROM inbox_operations WHERE actor=? AND generation=? AND operation_id=?",params![actor,generation,op],|r|r.get(0)).optional()?)
}
pub(crate) fn reject_operation_collision(
    conn: &Connection,
    actor: &str,
    generation: &str,
    op: &str,
) -> Result<()> {
    if operation_receipt(conn, actor, generation, op)?.is_some() {
        return Err(reused());
    }
    Ok(())
}
fn reused() -> Error {
    Error::domain(
        "OPERATION_REUSED",
        "An operation ID cannot be reused for another request or operation family.",
    )
}
fn identity(conn: &Connection, v: &Value) -> Result<Value> {
    let w = store::workspace(conn)?;
    if v["workspaceId"] != w["id"] {
        return Err(Error::domain(
            "WORKSPACE_MISMATCH",
            "Fetch the current workspace.",
        ));
    }
    if v["generation"] != w["generation"] {
        return Err(Error::domain(
            "STALE_GENERATION",
            "Workspace was restored; re-propose against the current generation.",
        ));
    }
    Ok(w)
}
fn eligible(conn: &Connection, actor: &str, kind: Kind) -> Result<()> {
    let row: Option<(String, i64)> = conn
        .query_row(
            "SELECT scopes,revoked FROM clients WHERE id=?",
            [actor],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    let Some((scopes, revoked)) = row else {
        return Err(Error::domain(
            "UNAUTHORIZED",
            "Proposer no longer has a current grant.",
        ));
    };
    if revoked != 0 {
        return Err(Error::domain("UNAUTHORIZED", "Proposer grant was revoked."));
    }
    let scopes: Vec<String> = serde_json::from_str(&scopes)?;
    if !scopes
        .iter()
        .any(|s| s == "context:propose" || *s == kind.scope())
    {
        return Err(Error::domain(
            "FORBIDDEN",
            "Proposer needs context:propose or the corresponding kind write scope.",
        ));
    }
    Ok(())
}
fn authenticate_proposal(conn: &Connection, auth: &Auth, kind: Kind) -> Result<()> {
    match store::authorize(conn, auth, "context:propose") {
        Ok(()) => Ok(()),
        Err(e) if e.code() == "FORBIDDEN" => store::authorize(conn, auth, &kind.scope()),
        Err(e) => Err(e),
    }
}
fn parse_proposal(mut raw: Value) -> Result<Value> {
    let source = string(&raw, "source")?.trim().to_owned();
    if source.is_empty() || source.encode_utf16().count() > 500 {
        return Err(invalid("Source must contain 1 to 500 characters"));
    }
    let basis = string(&raw, "basis")?.to_owned();
    if !["owner_statement", "source", "model_inference"].contains(&basis.as_str()) {
        return Err(invalid("Invalid proposal basis"));
    }
    let obj = raw
        .as_object_mut()
        .ok_or_else(|| invalid("Expected object"))?;
    obj.remove("source");
    obj.remove("basis");
    let input = Write::parse(raw)?;
    if !matches!(input.kind, Kind::Memory | Kind::Decision | Kind::Task) {
        return Err(invalid(
            "Only memory, decision and task proposals are supported",
        ));
    }
    let mut v = input.value;
    if input.kind == Kind::Memory && v["record"]["basis"] != basis {
        return Err(invalid("Memory basis must match proposal basis"));
    }
    v["source"] = json!(source);
    v["basis"] = json!(basis);
    if v.to_string().len() > 65536 {
        return Err(invalid("Proposal is too large"));
    }
    Ok(v)
}
fn parse_review(mut v: Value) -> Result<Value> {
    contracts::strict(
        &v,
        &[
            "workspaceId",
            "generation",
            "operationId",
            "proposalId",
            "expectedRevision",
            "action",
            "record",
        ],
    )?;
    for key in ["workspaceId", "generation", "operationId", "proposalId"] {
        contracts::id(string(&v, key)?)?;
    }
    v["expectedRevision"] = json!(integer(&v, "expectedRevision", 1, 9007199254740990)?);
    match string(&v, "action")? {
        "save" => {
            if let Some(record) = v.get("record") {
                v["record"] = contracts::normalize_draft(record.clone())?;
            }
        }
        "dismiss" => {
            if v.get("record").is_some() {
                return Err(invalid("Dismiss cannot edit a record"));
            }
        }
        _ => return Err(invalid("Unknown review action")),
    }
    Ok(v)
}
fn refs(conn: &Connection, draft: &Value) -> Result<()> {
    for id in draft["references"]
        .as_array()
        .ok_or_else(|| invalid("Invalid references"))?
    {
        let found: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM records WHERE id=?)",
            [id.as_str().ok_or_else(|| invalid("Invalid reference"))?],
            |r| r.get(0),
        )?;
        if !found {
            return Err(Error::domain(
                "REFERENCE_NOT_FOUND",
                "Referenced record does not exist.",
            ));
        }
    }
    Ok(())
}
fn candidate(record: &Value) -> Result<Value> {
    let mut v = record.clone();
    let o = v.as_object_mut().ok_or_else(|| invalid("Invalid record"))?;
    for key in ["id", "revision", "actor", "createdAt", "updatedAt"] {
        o.remove(key);
    }
    contracts::normalize_draft(v)
}
fn duplicate(conn: &Connection, draft: &Value) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT payload FROM records WHERE kind=?")?;
    for row in stmt.query_map([string(draft, "kind")?], |r| r.get::<_, String>(0))? {
        let r: Value = serde_json::from_str(&row?)?;
        if candidate(&r)? == *draft {
            return Ok(Some(string(&r, "id")?.to_owned()));
        }
    }
    Ok(None)
}
fn dedup(p: &Value) -> String {
    contracts::hash(&json!({"actor":p["actor"],"workspaceId":p["workspaceId"],"generation":p["generation"],"source":p["source"],"basis":p["basis"],"record":p["record"],"targetId":p["targetId"],"expectedRevision":p["expectedRevision"]}).to_string())
}
fn read(conn: &Connection, id: &str) -> Result<Value> {
    contracts::id(id)?;
    if !exists(conn)? {
        return Err(Error::domain("NOT_FOUND", "Proposal does not exist."));
    }
    let raw: Option<String> = conn
        .query_row(
            "SELECT payload FROM inbox_proposals WHERE id=?",
            [id],
            |r| r.get(0),
        )
        .optional()?;
    Ok(serde_json::from_str(&raw.ok_or_else(|| {
        Error::domain("NOT_FOUND", "Proposal does not exist.")
    })?)?)
}
fn full(conn: &Connection, mut p: Value) -> Result<Value> {
    let w = store::workspace(conn)?;
    let current = match p["targetId"].as_str() {
        Some(id) => Some(store::read_record(conn, id)?.0),
        None => None,
    };
    p["conflict"] = json!(
        p["generation"] != w["generation"]
            || current
                .as_ref()
                .is_some_and(|r| r["revision"] != p["expectedRevision"])
    );
    p["currentRecord"] = json!(current);
    p["duplicateRecordId"] = if p["targetId"].is_null() {
        json!(duplicate(conn, &p["record"])?)
    } else {
        Value::Null
    };
    let mut stmt = conn.prepare("SELECT id FROM records WHERE kind=?1 AND (lower(title)=lower(?2) OR lower(content)=lower(?3)) AND (?4 IS NULL OR id<>?4) AND (?5 IS NULL OR id<>?5) ORDER BY id LIMIT 5")?;
    let similar = stmt
        .query_map(
            params![
                string(&p["record"], "kind")?,
                string(&p["record"], "title")?,
                string(&p["record"], "content")?,
                p["targetId"].as_str(),
                p["duplicateRecordId"].as_str()
            ],
            |r| r.get::<_, String>(0),
        )?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    p["similarRecordIds"] = json!(similar);
    Ok(p)
}
fn prior(conn: &Connection, actor: &str, family: &str, v: &Value) -> Result<Option<Value>> {
    let generation = string(v, "generation")?;
    let op = string(v, "operationId")?;
    if exists(conn)? {
        let row:Option<(String,String,String)>=conn.query_row("SELECT family,request_hash,receipt FROM inbox_operations WHERE actor=? AND generation=? AND operation_id=?",params![actor,generation,op],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
        if let Some((f, h, r)) = row {
            if f != family || h != contracts::hash(&v.to_string()) {
                return Err(reused());
            }
            return Ok(Some(serde_json::from_str(&r)?));
        }
    }
    let used: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM operations WHERE actor=? AND generation=? AND operation_id=?)",
        params![actor, generation, op],
        |r| r.get(0),
    )?;
    if used {
        return Err(reused());
    }
    Ok(None)
}
fn persist_operation(
    conn: &Connection,
    actor: &str,
    family: &str,
    v: &Value,
    receipt: &Value,
) -> Result<()> {
    conn.execute(
        "INSERT INTO inbox_operations VALUES(?,?,?,?,?,?,?)",
        params![
            actor,
            string(v, "generation")?,
            string(v, "operationId")?,
            family,
            contracts::hash(&v.to_string()),
            v.to_string(),
            receipt.to_string()
        ],
    )?;
    Ok(())
}
fn persist_proposal(conn: &Connection, p: &Value) -> Result<()> {
    conn.execute("INSERT INTO inbox_proposals VALUES(?,?,?,?) ON CONFLICT(id) DO UPDATE SET status=excluded.status,payload=excluded.payload",params![string(p,"id")?,string(p,"status")?,dedup(p),p.to_string()])?;
    Ok(())
}
pub(crate) fn check_capacity(conn: &Connection) -> Result<()> {
    if !exists(conn)? {
        return Ok(());
    }
    let (p, b): (i64, i64) = conn.query_row(
        "SELECT COUNT(*),COALESCE(SUM(length(CAST(payload AS BLOB))),0) FROM inbox_proposals",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    let(o,c):(i64,i64)=conn.query_row("SELECT COUNT(*),COALESCE(SUM(length(CAST(request AS BLOB))+length(CAST(receipt AS BLOB))),0) FROM inbox_operations",[],|r|Ok((r.get(0)?,r.get(1)?)))?;
    let mut u: Usage = serde_json::from_value(store::metadata(conn, "usage")?)?;
    u.records += u64::try_from(p).map_err(|_| invalid("Invalid inbox counter"))?;
    u.operations += u64::try_from(o).map_err(|_| invalid("Invalid inbox counter"))?;
    u.payload_bytes += u64::try_from(b + c).map_err(|_| invalid("Invalid inbox counter"))?;
    u.check()
}
impl Store {
    /// Submit one attributed memory, decision or task for local owner review.
    pub fn propose(&mut self, auth: &Auth, raw: Value) -> Result<Value> {
        let v = parse_proposal(raw)?;
        let kind = Kind::parse(string(&v["record"], "kind")?)?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        authenticate_proposal(&tx, auth, kind)?;
        identity(&tx, &v)?;
        if let Some(r) = prior(&tx, &auth.client_id, "propose", &v)? {
            tx.commit()?;
            return Ok(r);
        }
        let before = match v.get("id") {
            Some(id) => {
                let r =
                    store::read_record(&tx, id.as_str().ok_or_else(|| invalid("Invalid ID"))?)?.0;
                if r["revision"] != v["expectedRevision"] {
                    return Err(Error::domain("CONFLICT", "Target revision changed."));
                }
                if r["kind"] != v["record"]["kind"] {
                    return Err(Error::domain("KIND_MISMATCH", "Target kind cannot change."));
                }
                Some(r)
            }
            None => None,
        };
        refs(&tx, &v["record"])?;
        let stamp = store::now();
        let mut p = json!({"id":uuid::Uuid::new_v4().to_string(),"revision":1,"status":"pending","actor":auth.client_id,"workspaceId":v["workspaceId"],"generation":v["generation"],"source":v["source"],"basis":v["basis"],"record":v["record"],"targetId":v.get("id"),"expectedRevision":v["expectedRevision"],"before":before,"createdAt":stamp,"updatedAt":stamp,"receipt":null});
        tx.execute_batch(SCHEMA)?;
        let hash = dedup(&p);
        let old:Option<String>=tx.query_row("SELECT payload FROM inbox_proposals WHERE status='pending' AND dedup_hash=? LIMIT 1",[&hash],|r|r.get(0)).optional()?;
        if let Some(old) = old {
            p = serde_json::from_str(&old)?;
        } else {
            persist_proposal(&tx, &p)?;
        }
        let result = json!({"proposal":full(&tx,p)?,"operationId":v["operationId"],"generation":v["generation"]});
        persist_operation(&tx, &auth.client_id, "propose", &v, &result)?;
        check_capacity(&tx)?;
        tx.commit()?;
        Ok(result)
    }
    /// Return up to one hundred pending summaries without creating extension tables.
    pub fn inbox_list(&self) -> Result<Value> {
        let tx = self.conn.unchecked_transaction()?;
        let mut proposals = Vec::new();
        let mut count = 0i64;
        if exists(&tx)? {
            count = tx.query_row(
                "SELECT COUNT(*) FROM inbox_proposals WHERE status='pending'",
                [],
                |r| r.get(0),
            )?;
            let mut q = tx.prepare(
                "SELECT payload FROM inbox_proposals WHERE status='pending' ORDER BY id LIMIT 100",
            )?;
            for row in q.query_map([], |r| r.get::<_, String>(0))? {
                let p: Value = serde_json::from_str(&row?)?;
                proposals.push(json!({"id":p["id"],"title":p["record"]["title"],"actor":p["actor"],"status":p["status"],"revision":p["revision"]}));
            }
        }
        let result = json!({"workspace":store::workspace(&tx)?,"omitted":count-proposals.len() as i64,"proposals":proposals});
        tx.commit()?;
        Ok(result)
    }
    /// Read all original proposal fields and current conflict and duplicate information.
    pub fn inbox_get(&self, id: &str) -> Result<Value> {
        let tx = self.conn.unchecked_transaction()?;
        let p = full(&tx, read(&tx, id)?)?;
        tx.commit()?;
        Ok(p)
    }
    /// Read a proposal after authenticating the client's current read grant.
    pub fn get_proposal(&mut self, auth: &Auth, id: &str) -> Result<Value> {
        let tx = self.conn.transaction()?;
        store::authorize(&tx, auth, "read")?;
        let p = full(&tx, read(&tx, id)?)?;
        tx.commit()?;
        Ok(p)
    }
    /// Atomically save or dismiss a pending proposal through local owner authority only.
    pub fn review_proposal(&mut self, raw: Value) -> Result<Value> {
        let v = parse_review(raw)?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        identity(&tx, &v)?;
        if let Some(r) = prior(&tx, OWNER, "review", &v)? {
            tx.commit()?;
            return Ok(r);
        }
        let mut p = read(&tx, string(&v, "proposalId")?)?;
        if p["revision"] != v["expectedRevision"] || p["status"] != "pending" {
            return Err(Error::domain(
                "CONFLICT",
                "Proposal already resolved or changed.",
            ));
        }
        let save = v["action"] == "save";
        let receipt = if save {
            identity(&tx, &p)?;
            let draft = v.get("record").unwrap_or(&p["record"]).clone();
            if draft["kind"] != p["record"]["kind"] {
                return Err(Error::domain(
                    "KIND_MISMATCH",
                    "Review cannot change proposal kind.",
                ));
            }
            eligible(
                &tx,
                string(&p, "actor")?,
                Kind::parse(string(&draft, "kind")?)?,
            )?;
            if p["targetId"].is_null() && duplicate(&tx, &draft)?.is_some() {
                return Err(Error::domain(
                    "DUPLICATE_RECORD",
                    "An exact canonical record already exists. Reconcile before saving.",
                ));
            }
            let mut write = json!({"workspaceId":v["workspaceId"],"generation":v["generation"],"operationId":v["operationId"],"expectedRevision":p["expectedRevision"],"record":draft});
            if !p["targetId"].is_null() {
                write["id"] = p["targetId"].clone();
            }
            Some(store::write_transaction(&tx, OWNER, &Write::parse(write)?)?)
        } else {
            None
        };
        let status = if save { "saved" } else { "dismissed" };
        let result = json!({"proposalId":p["id"],"status":status,"operationId":v["operationId"],"generation":v["generation"],"receipt":receipt});
        p["status"] = json!(status);
        p["revision"] = json!(2);
        let previous_time = contracts::timestamp(string(&p, "updatedAt")?)?;
        p["updatedAt"] = json!(
            chrono::Utc::now()
                .max(previous_time)
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
        );
        p["receipt"] = result.clone();
        persist_proposal(&tx, &p)?;
        persist_operation(&tx, OWNER, "review", &v, &result)?;
        check_capacity(&tx)?;
        tx.commit()?;
        Ok(result)
    }
}
pub(crate) fn export(conn: &Connection) -> Result<Option<Value>> {
    if !exists(conn)? {
        return Ok(None);
    }
    let mut q = conn.prepare("SELECT payload FROM inbox_proposals ORDER BY id")?;
    let proposals = q
        .query_map([], |r| r.get::<_, String>(0))?
        .map(|r| Ok(serde_json::from_str::<Value>(&r?)?))
        .collect::<Result<Vec<_>>>()?;
    let mut q=conn.prepare("SELECT actor,generation,operation_id,family,request_hash,request,receipt FROM inbox_operations ORDER BY actor,generation,operation_id")?;
    let operations=q.query_map([],|r|Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?,r.get::<_,String>(3)?,r.get::<_,String>(4)?,r.get::<_,String>(5)?,r.get::<_,String>(6)?)))?.map(|r|{let(a,g,o,f,h,q,r)=r?;Ok(json!({"actor":a,"generation":g,"operationId":o,"family":f,"requestHash":h,"request":serde_json::from_str::<Value>(&q)?,"receipt":serde_json::from_str::<Value>(&r)?}))}).collect::<Result<Vec<_>>>()?;
    Ok(Some(
        json!({"version":1,"proposals":proposals,"operations":operations}),
    ))
}
fn list<'a>(v: &'a Value, key: &str, max: usize) -> Result<&'a Vec<Value>> {
    let a = v[key]
        .as_array()
        .ok_or_else(|| invalid(format!("Invalid inbox {key}")))?;
    if a.len() > max {
        return Err(invalid("Inbox capacity exceeded"));
    }
    Ok(a)
}
fn validate_refs(draft: &Value, records: &[Value]) -> Result<()> {
    for id in list(draft, "references", 30)? {
        if !records.iter().any(|r| r["id"] == *id) {
            return Err(invalid("Unresolved inbox reference"));
        }
    }
    Ok(())
}
fn validate_proposal(p: &Value, workspace: &Value, records: &[Value]) -> Result<()> {
    contracts::strict(
        p,
        &[
            "id",
            "revision",
            "status",
            "actor",
            "workspaceId",
            "generation",
            "source",
            "basis",
            "record",
            "targetId",
            "expectedRevision",
            "before",
            "createdAt",
            "updatedAt",
            "receipt",
        ],
    )?;
    for key in ["id", "workspaceId", "generation"] {
        contracts::id(string(p, key)?)?;
    }
    if p["workspaceId"] != workspace["id"] || !contracts::valid_client(string(p, "actor")?) {
        return Err(invalid("Invalid proposal identity"));
    }
    let revision = integer(p, "revision", 1, 2)?;
    let status = string(p, "status")?;
    if !["pending", "saved", "dismissed"].contains(&status)
        || (status == "pending") != (revision == 1)
    {
        return Err(invalid("Invalid proposal resolution"));
    }
    let created = contracts::timestamp(string(p, "createdAt")?)?;
    let updated = contracts::timestamp(string(p, "updatedAt")?)?;
    if created > updated {
        return Err(invalid("Invalid proposal timestamps"));
    }
    let mut request = json!({"workspaceId":p["workspaceId"],"generation":p["generation"],"operationId":p["id"],"expectedRevision":p["expectedRevision"],"source":p["source"],"basis":p["basis"],"record":p["record"]});
    if !p["targetId"].is_null() {
        request["id"] = p["targetId"].clone();
    }
    let normalized = parse_proposal(request.clone())?;
    if normalized != request {
        return Err(invalid("Proposal must be normalized"));
    }
    validate_refs(&p["record"], records)?;
    if let Some(id) = p["targetId"].as_str() {
        let before = contracts::parse_record(p["before"].clone())?;
        let current = records
            .iter()
            .find(|r| r["id"] == id)
            .ok_or_else(|| invalid("Proposal target missing"))?;
        if before != p["before"]
            || before["id"] != id
            || before["revision"] != p["expectedRevision"]
            || before["kind"] != p["record"]["kind"]
            || before["kind"] != current["kind"]
            || integer(&before, "revision", 1, 9007199254740991)?
                > integer(current, "revision", 1, 9007199254740991)?
        {
            return Err(invalid("Invalid proposal target snapshot"));
        }
        validate_refs(&before, records)?;
        if before["revision"] == current["revision"] && before != *current {
            return Err(invalid("Snapshot disagrees with current record"));
        }
    } else if !p["before"].is_null() {
        return Err(invalid("New proposal cannot have a before snapshot"));
    }
    if status == "pending" && !p["receipt"].is_null() {
        return Err(invalid("Pending proposal cannot have final receipt"));
    }
    Ok(())
}
/// Validate the entire optional extension before restore creates any filesystem state.
pub(crate) fn validate(
    extension: &Value,
    workspace: &Value,
    records: &[Value],
    canonical_ops: &[Value],
) -> Result<()> {
    use std::collections::{HashMap, HashSet};
    contracts::strict(extension, &["version", "proposals", "operations"])?;
    integer(extension, "version", 1, 1)?;
    let proposals = list(extension, "proposals", MAX_PROPOSALS)?;
    let operations = list(extension, "operations", MAX_OPERATIONS)?;
    let mut by_id = HashMap::new();
    let mut pending = HashSet::new();
    for p in proposals {
        validate_proposal(p, workspace, records)?;
        if by_id.insert(string(p, "id")?, p).is_some() {
            return Err(invalid("Duplicate proposal ID"));
        }
        if p["status"] == "pending" && !pending.insert(dedup(p)) {
            return Err(invalid("Duplicate pending proposal"));
        }
    }
    let mut keys = HashSet::new();
    let mut submitted = HashSet::new();
    let mut reviewed = HashSet::new();
    for op in operations {
        contracts::strict(
            op,
            &[
                "actor",
                "generation",
                "operationId",
                "family",
                "requestHash",
                "request",
                "receipt",
            ],
        )?;
        let actor = string(op, "actor")?;
        let family = string(op, "family")?;
        for key in ["generation", "operationId"] {
            contracts::id(string(op, key)?)?;
        }
        if !keys.insert((actor, string(op, "generation")?, string(op, "operationId")?)) {
            return Err(invalid("Duplicate inbox operation"));
        }
        let q = &op["request"];
        let r = &op["receipt"];
        if string(op, "requestHash")? != contracts::hash(&q.to_string())
            || q["operationId"] != op["operationId"]
            || q["generation"] != op["generation"]
            || q["workspaceId"] != workspace["id"]
            || r["operationId"] != op["operationId"]
            || r["generation"] != op["generation"]
        {
            return Err(invalid("Invalid inbox operation hash or identity"));
        }
        let canonical = canonical_ops.iter().find(|c| {
            c["actor"] == op["actor"]
                && c["generation"] == op["generation"]
                && c["operationId"] == op["operationId"]
        });
        match family {
            "propose" => {
                if !contracts::valid_client(actor)
                    || parse_proposal(q.clone())? != *q
                    || canonical.is_some()
                {
                    return Err(invalid("Invalid proposal operation"));
                }
                contracts::strict(r, &["proposal", "operationId", "generation"])?;
                let mut snapshot = r["proposal"].clone();
                contracts::strict(
                    &snapshot,
                    &[
                        "id",
                        "revision",
                        "status",
                        "actor",
                        "workspaceId",
                        "generation",
                        "source",
                        "basis",
                        "record",
                        "targetId",
                        "expectedRevision",
                        "before",
                        "createdAt",
                        "updatedAt",
                        "receipt",
                        "currentRecord",
                        "conflict",
                        "duplicateRecordId",
                        "similarRecordIds",
                    ],
                )?;
                if snapshot["conflict"] != false {
                    return Err(invalid("Invalid proposal conflict flag"));
                }
                if !snapshot["duplicateRecordId"].is_null() {
                    let id = string(&snapshot, "duplicateRecordId")?;
                    contracts::id(id)?;
                    if !records.iter().any(|r| r["id"] == id) {
                        return Err(invalid("Duplicate warning record missing"));
                    }
                }
                for id in list(&snapshot, "similarRecordIds", 5)? {
                    let id = id
                        .as_str()
                        .ok_or_else(|| invalid("Invalid similarity warning ID"))?;
                    contracts::id(id)?;
                    if !records.iter().any(|r| r["id"] == id) {
                        return Err(invalid("Similarity warning record missing"));
                    }
                }
                if !snapshot["currentRecord"].is_null() {
                    contracts::parse_record(snapshot["currentRecord"].clone())?;
                    if snapshot["currentRecord"] != snapshot["before"] {
                        return Err(invalid("Submission current snapshot disagrees"));
                    }
                }
                let obj = snapshot
                    .as_object_mut()
                    .ok_or_else(|| invalid("Invalid proposal receipt"))?;
                for key in [
                    "currentRecord",
                    "conflict",
                    "duplicateRecordId",
                    "similarRecordIds",
                ] {
                    obj.remove(key);
                }
                validate_proposal(&snapshot, workspace, records)?;
                let p = by_id
                    .get(string(&snapshot, "id")?)
                    .ok_or_else(|| invalid("Submission proposal missing"))?;
                if snapshot["status"] != "pending"
                    || snapshot["actor"] != actor
                    || snapshot["generation"] != op["generation"]
                {
                    return Err(invalid("Invalid submitted proposal"));
                }
                for key in [
                    "id",
                    "actor",
                    "workspaceId",
                    "generation",
                    "source",
                    "basis",
                    "record",
                    "targetId",
                    "expectedRevision",
                    "before",
                    "createdAt",
                ] {
                    if snapshot[key] != p[key] {
                        return Err(invalid("Proposal attribution changed"));
                    }
                }
                for key in [
                    "workspaceId",
                    "generation",
                    "source",
                    "basis",
                    "record",
                    "expectedRevision",
                ] {
                    if q[key] != p[key] {
                        return Err(invalid("Proposal request disagrees"));
                    }
                }
                if q.get("id").unwrap_or(&Value::Null) != &p["targetId"] {
                    return Err(invalid("Proposal request target disagrees"));
                }
                submitted.insert(string(p, "id")?);
            }
            "review" => {
                if actor != OWNER || parse_review(q.clone())? != *q {
                    return Err(invalid("Invalid owner review"));
                }
                contracts::strict(
                    r,
                    &[
                        "proposalId",
                        "status",
                        "operationId",
                        "generation",
                        "receipt",
                    ],
                )?;
                let p = by_id
                    .get(string(q, "proposalId")?)
                    .ok_or_else(|| invalid("Reviewed proposal missing"))?;
                if q["expectedRevision"] != 1
                    || p["revision"] != 2
                    || r["proposalId"] != p["id"]
                    || r["status"] != p["status"]
                    || p["receipt"] != *r
                    || !reviewed.insert(string(p, "id")?)
                {
                    return Err(invalid("Invalid resolved proposal receipt"));
                }
                if q["action"] == "save" {
                    if p["status"] != "saved" || p["generation"] != q["generation"] {
                        return Err(invalid("Invalid saved generation"));
                    }
                    let canonical =
                        canonical.ok_or_else(|| invalid("Canonical approval receipt missing"))?;
                    if canonical["receipt"] != r["receipt"] {
                        return Err(invalid("Approval receipts disagree"));
                    }
                    let approved = &r["receipt"]["record"];
                    let draft = q.get("record").unwrap_or(&p["record"]);
                    let mut write = json!({"workspaceId":q["workspaceId"],"generation":q["generation"],"operationId":q["operationId"],"expectedRevision":p["expectedRevision"],"record":draft});
                    if !p["targetId"].is_null() {
                        write["id"] = p["targetId"].clone();
                    }
                    if canonical["requestHash"]
                        != contracts::hash(&Write::parse(write)?.value.to_string())
                    {
                        return Err(invalid("Canonical approval request hash disagrees"));
                    }

                    if candidate(approved)? != *draft
                        || approved["kind"] != p["record"]["kind"]
                        || approved["revision"]
                            != json!(integer(p, "expectedRevision", 0, 9007199254740990)? + 1)
                        || (!p["targetId"].is_null() && approved["id"] != p["targetId"])
                    {
                        return Err(invalid("Approved record disagrees with review"));
                    }
                } else if p["status"] != "dismissed"
                    || !r["receipt"].is_null()
                    || canonical.is_some()
                {
                    return Err(invalid("Invalid dismissal receipt"));
                }
            }
            _ => return Err(invalid("Invalid inbox operation family")),
        }
    }
    for p in proposals {
        let id = string(p, "id")?;
        if !submitted.contains(id) || (p["status"] != "pending" && !reviewed.contains(id)) {
            return Err(invalid("Proposal audit receipts missing"));
        }
    }
    let payload = proposals
        .iter()
        .map(|p| p.to_string().len() as u64)
        .sum::<u64>()
        + operations
            .iter()
            .map(|o| o["request"].to_string().len() as u64 + o["receipt"].to_string().len() as u64)
            .sum::<u64>()
        + records
            .iter()
            .map(|r| r.to_string().len() as u64)
            .sum::<u64>()
        + canonical_ops
            .iter()
            .map(|o| o["receipt"].to_string().len() as u64)
            .sum::<u64>();
    Usage {
        records: (records.len() + proposals.len()) as u64,
        operations: (canonical_ops.len() + operations.len()) as u64,
        payload_bytes: payload,
    }
    .check()
}
pub(crate) fn restore(conn: &Connection, extension: &Value) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    for p in list(extension, "proposals", MAX_PROPOSALS)? {
        persist_proposal(conn, p)?;
    }
    for op in list(extension, "operations", MAX_OPERATIONS)? {
        persist_operation(
            conn,
            string(op, "actor")?,
            string(op, "family")?,
            &op["request"],
            &op["receipt"],
        )?;
    }
    Ok(())
}
