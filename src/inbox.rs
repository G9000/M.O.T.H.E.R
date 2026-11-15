//! Durable, isolated capture proposals and local owner review.
use crate::{
    contracts::{self, Kind, Write, integer, string},
    error::{Result, invalid},
    limits::Usage,
};
use serde_json::{Value, json};
const OWNER: &str = "@owner-review";
const MAX_PROPOSALS: usize = 10000;
const MAX_OPERATIONS: usize = 100000;
pub(crate) fn parse_proposal(mut raw: Value) -> Result<Value> {
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
pub(crate) fn parse_review(mut v: Value) -> Result<Value> {
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
pub(crate) fn candidate(record: &Value) -> Result<Value> {
    let mut v = record.clone();
    let o = v.as_object_mut().ok_or_else(|| invalid("Invalid record"))?;
    for key in ["id", "revision", "actor", "createdAt", "updatedAt"] {
        o.remove(key);
    }
    contracts::normalize_draft(v)
}
pub(crate) fn dedup(p: &Value) -> String {
    contracts::hash(&json!({"actor":p["actor"],"workspaceId":p["workspaceId"],"generation":p["generation"],"source":p["source"],"basis":p["basis"],"record":p["record"],"targetId":p["targetId"],"expectedRevision":p["expectedRevision"]}).to_string())
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
    validate_with_limits(extension, workspace, records, canonical_ops, true)
}
pub(crate) fn validate_with_limits(
    extension: &Value,
    workspace: &Value,
    records: &[Value],
    canonical_ops: &[Value],
    small: bool,
) -> Result<()> {
    use std::collections::{HashMap, HashSet};
    contracts::strict(extension, &["version", "proposals", "operations"])?;
    integer(extension, "version", 1, 1)?;
    let proposals = list(
        extension,
        "proposals",
        if small { MAX_PROPOSALS } else { usize::MAX },
    )?;
    let operations = list(
        extension,
        "operations",
        if small { MAX_OPERATIONS } else { usize::MAX },
    )?;
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
    if small {
        Usage {
            records: (records.len() + proposals.len()) as u64,
            operations: (canonical_ops.len() + operations.len()) as u64,
            payload_bytes: payload,
        }
        .check()?;
    }
    Ok(())
}
