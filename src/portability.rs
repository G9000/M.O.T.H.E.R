//! Credential-free portable v1 recovery, separate from executable/runtime replacement.
use crate::{
    contracts::{self, integer, parse_record, string},
    error::{Error, Result, invalid},
    limits::Usage,
};
use serde_json::{Value, json};
use std::collections::{BTreeSet, HashMap, HashSet};
fn export_error(message: &str) -> Error {
    Error::domain("INVALID_EXPORT", message)
}
fn array<'a>(v: &'a Value, key: &str, max: usize) -> Result<&'a Vec<Value>> {
    let a = v
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| invalid(format!("Invalid {key}")))?;
    if a.len() > max {
        return Err(invalid(format!("Too many {key}")));
    }
    Ok(a)
}
fn sources(records: &[Value], operations: &[Value]) -> Result<Vec<String>> {
    let mut out = BTreeSet::new();
    for r in records
        .iter()
        .chain(operations.iter().map(|op| &op["receipt"]["record"]))
    {
        for s in array(r, "sources", 20)? {
            out.insert(string(s, "uri")?.to_owned());
        }
    }
    Ok(out.into_iter().collect())
}
fn bundle_sources(
    records: &[Value],
    operations: &[Value],
    inbox: Option<&Value>,
) -> Result<Vec<String>> {
    let mut all = records.to_vec();
    if let Some(inbox) = inbox {
        for p in array(inbox, "proposals", 10000)? {
            all.push(p["record"].clone());
            if !p["before"].is_null() {
                all.push(p["before"].clone());
            }
        }
    }
    sources(&all, operations)
}
pub(crate) fn validate(mut v: Value) -> Result<Value> {
    contracts::strict(
        &v,
        &[
            "schemaVersion",
            "exportedAt",
            "workspace",
            "persona",
            "records",
            "operations",
            "omittedExternalSources",
            "inbox",
            "pins",
            "assignments",
        ],
    )?;
    integer(&v, "schemaVersion", 1, 1)?;
    contracts::timestamp(string(&v, "exportedAt")?)?;
    contracts::validate_workspace(&v["workspace"])?;
    v["schemaVersion"] = json!(1);
    v["workspace"]["schemaVersion"] = json!(1);
    contracts::validate_persona(&v["persona"])?;
    let records = array(&v, "records", 10000)?
        .iter()
        .cloned()
        .map(parse_record)
        .collect::<Result<Vec<_>>>()?;
    let mut operations = array(&v, "operations", 100000)?.clone();
    for op in &mut operations {
        contracts::strict(
            op,
            &[
                "actor",
                "generation",
                "operationId",
                "requestHash",
                "receipt",
            ],
        )?;
        let actor = string(op, "actor")?;
        if actor.is_empty() || actor.encode_utf16().count() > 80 {
            return Err(invalid("Invalid operation actor"));
        }
        for key in ["generation", "operationId"] {
            contracts::id(string(op, key)?)?;
        }
        if !contracts::hex_digest(string(op, "requestHash")?) {
            return Err(invalid("Invalid operation hash"));
        }
        let receipt = &mut op["receipt"];
        contracts::strict(
            receipt,
            &["operationId", "generation", "committedAt", "record"],
        )?;
        contracts::id(string(receipt, "operationId")?)?;
        contracts::id(string(receipt, "generation")?)?;
        contracts::timestamp(string(receipt, "committedAt")?)?;
        receipt["record"] = parse_record(receipt["record"].clone())?;
    }
    let by_id: HashMap<&str, &Value> = records
        .iter()
        .map(|r| Ok((string(r, "id")?, r)))
        .collect::<Result<_>>()?;
    if by_id.len() != records.len() {
        return Err(export_error("Duplicate record IDs"));
    }
    for r in records
        .iter()
        .chain(operations.iter().map(|o| &o["receipt"]["record"]))
    {
        for id in array(r, "references", 30)? {
            if !by_id.contains_key(id.as_str().ok_or_else(|| invalid("Invalid reference"))?) {
                return Err(export_error("Unresolved record reference"));
            }
        }
    }
    let mut keys = HashSet::new();
    for op in &operations {
        let key = (
            string(op, "actor")?,
            string(op, "generation")?,
            string(op, "operationId")?,
        );
        if !keys.insert(key) {
            return Err(export_error("Duplicate operation IDs"));
        }
        let receipt = &op["receipt"];
        let record = &receipt["record"];
        let latest = by_id
            .get(string(record, "id")?)
            .ok_or_else(|| export_error("Receipt references missing record"))?;
        if op["actor"] != record["actor"]
            || op["generation"] != receipt["generation"]
            || op["operationId"] != receipt["operationId"]
            || receipt["committedAt"] != record["updatedAt"]
            || integer(record, "revision", 1, 9007199254740991)?
                > integer(latest, "revision", 1, 9007199254740991)?
        {
            return Err(export_error("Invalid operation receipt"));
        }
        if record["revision"] == latest["revision"] && record != *latest {
            return Err(export_error("Receipt disagrees with current record"));
        }
    }
    let payload_bytes = records
        .iter()
        .map(|r| r.to_string().len() as u64)
        .sum::<u64>()
        + operations
            .iter()
            .map(|o| o["receipt"].to_string().len() as u64)
            .sum::<u64>();
    Usage {
        records: records.len() as u64,
        operations: operations.len() as u64,
        payload_bytes,
    }
    .check()?;
    if let Some(inbox) = v.get("inbox") {
        crate::inbox::validate(inbox, &v["workspace"], &records, &operations)?;
    }
    if let Some(pins) = v.get("pins") {
        crate::pins::validate(pins, &records)?;
    }
    if let Some(assignments) = v.get("assignments") {
        crate::handoffs::validate(assignments, &records)?;
    }
    let omitted = bundle_sources(&records, &operations, v.get("inbox"))?;
    let supplied = array(&v, "omittedExternalSources", usize::MAX)?
        .iter()
        .map(|s| {
            s.as_str()
                .map(str::to_owned)
                .ok_or_else(|| invalid("Invalid omitted source"))
        })
        .collect::<Result<BTreeSet<_>>>()?;
    if supplied.into_iter().collect::<Vec<_>>() != omitted {
        return Err(export_error("External content omissions are incomplete"));
    }
    v["records"] = json!(records);
    v["operations"] = json!(operations);
    v["omittedExternalSources"] = json!(omitted);
    Ok(v)
}
