//! Owner-created client policies, independent of host or agent implementations.
use crate::{
    contracts,
    error::{Error, Result, invalid},
};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
/// Owner-selected context visibility; scopes remain independently required.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessMode {
    /// Existing full-context behavior, without an external subject binding.
    #[default]
    Legacy,
    /// Full context and persona, bound to an owner-selected external subject.
    Companion,
    /// Explicit canonical record allowlist, without persona or canonical writes.
    Shared,
}
/// A policy bound by the owner to a private client credential.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientAccess {
    /// Visibility selected by the local owner.
    pub mode: AccessMode,
    /// External agent identifier; required for companion and shared policies.
    pub subject: Option<String>,
    /// Canonical record IDs for shared access; an empty list grants no records.
    #[serde(default)]
    pub records: Vec<String>,
}
impl ClientAccess {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.mode == AccessMode::Legacy {
            if self.subject.is_some() || !self.records.is_empty() {
                return Err(invalid("Legacy grants cannot bind subjects or records"));
            }
        } else if self
            .subject
            .as_ref()
            .is_none_or(|s| s.trim().is_empty() || s.len() > 256)
        {
            return Err(invalid("Scoped grants require a subject of 1 to 256 bytes"));
        }
        if self.mode != AccessMode::Shared && !self.records.is_empty() {
            return Err(invalid("Only shared grants accept record IDs"));
        }
        if self.records.len() > 10000 {
            return Err(invalid("Too many shared record IDs"));
        }
        for id in &self.records {
            contracts::id(id)?;
        }
        Ok(())
    }
    pub(crate) fn metadata(&self) -> Value {
        json!({"version":1,"mode":self.mode,"subject":self.subject})
    }
    pub(crate) fn allows(&self, id: &str) -> bool {
        self.mode != AccessMode::Shared || self.records.iter().any(|x| x == id)
    }
    pub(crate) fn require(&self, id: &str) -> Result<()> {
        if self.allows(id) {
            Ok(())
        } else {
            Err(Error::domain("NOT_FOUND", "Record does not exist."))
        }
    }
    pub(crate) fn filter(&self) -> String {
        if self.mode != AccessMode::Shared {
            "1".into()
        } else if self.records.is_empty() {
            "0".into()
        } else {
            // Both provisioning and policy loading validate canonical UUID syntax.
            format!(
                "id IN ({})",
                self.records
                    .iter()
                    .map(|x| format!("'{x}'"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
    pub(crate) fn references(&self, record: &Value) -> Result<()> {
        if let Some(ids) = record["references"].as_array() {
            for id in ids {
                self.require(id.as_str().ok_or_else(|| invalid("Invalid reference"))?)?;
            }
        }
        Ok(())
    }
    pub(crate) fn scrub(&self, value: &mut Value) {
        match value {
            Value::Array(a) => {
                for v in a {
                    self.scrub(v)
                }
            }
            Value::Object(o) => {
                for (k, v) in o {
                    if k == "references" || k == "similarRecordIds" {
                        if let Some(a) = v.as_array_mut() {
                            a.retain(|x| x.as_str().is_some_and(|id| self.allows(id)));
                        }
                    } else if k == "duplicateRecordId" {
                        if v.as_str().is_some_and(|id| !self.allows(id)) {
                            *v = Value::Null;
                        }
                    } else {
                        self.scrub(v);
                    }
                }
            }
            _ => {}
        }
    }
    fn embedded_records(&self, value: &Value) -> Result<()> {
        match value {
            Value::Array(values) => {
                for value in values {
                    self.embedded_records(value)?;
                }
            }
            Value::Object(fields) => {
                if fields.contains_key("kind")
                    && let Some(id) = fields.get("id").and_then(Value::as_str)
                {
                    self.require(id)?;
                }
                for value in fields.values() {
                    self.embedded_records(value)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub(crate) fn proposal(&self, client: &str, p: &mut Value) -> Result<()> {
        if self.mode == AccessMode::Shared {
            if p["actor"].as_str() != Some(client) {
                return Err(Error::domain("NOT_FOUND", "Proposal does not exist."));
            }
            if let Some(id) = p["targetId"].as_str() {
                self.require(id)?;
            }
            self.references(&p["record"])?;
            if let Some(id) = p["receipt"]["record"]["id"].as_str() {
                self.require(id)?;
            }
        }
        self.embedded_records(p)?;
        self.scrub(p);
        Ok(())
    }
    pub(crate) fn receipt(&self, client: &str, v: &mut Value) -> Result<()> {
        if let Some(id) = v["record"]["id"].as_str() {
            self.require(id)?;
        }
        if let Some(p) = v.get_mut("proposal") {
            self.proposal(client, p)?;
        }
        self.embedded_records(v)?;
        self.scrub(v);
        Ok(())
    }
}
pub(crate) fn load(conn: &Connection, client: &str) -> Result<ClientAccess> {
    // This grant anchor is separate from withdrawable policy content. Reads occur
    // inside the caller's canonical transaction, so policy loss cannot become legacy.
    let requirements_exist: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='client_policy_requirements')",
        [], |r| r.get(0),
    )?;
    let required = requirements_exist
        && conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM client_policy_requirements WHERE client_id=?)",
            [client],
            |r| r.get::<_, bool>(0),
        )?;
    let missing_policy = || {
        Error::domain(
            "FORBIDDEN",
            "This scoped grant requires a current companion or shared policy.",
        )
    };
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='client_access')",
        [],
        |r| r.get(0),
    )?;
    if !exists {
        return if required {
            Err(missing_policy())
        } else {
            Ok(ClientAccess::default())
        };
    }
    let raw: Option<String> = conn
        .query_row(
            "SELECT policy FROM client_access WHERE client_id=?",
            [client],
            |r| r.get(0),
        )
        .optional()?;
    let p = raw
        .map(|s| serde_json::from_str::<ClientAccess>(&s))
        .transpose()?
        .unwrap_or_default();
    p.validate()?;
    if required && p.mode == AccessMode::Legacy {
        return Err(missing_policy());
    }
    Ok(p)
}
pub(crate) fn save(
    conn: &Connection,
    client: &str,
    scopes: &[String],
    p: &ClientAccess,
) -> Result<()> {
    p.validate()?;
    if p.mode == AccessMode::Shared
        && scopes
            .iter()
            .any(|s| !matches!(s.as_str(), "read" | "context:propose"))
    {
        return Err(invalid("Shared grants allow only read and context:propose"));
    }
    for id in &p.records {
        crate::store::read_record(conn, id)?;
    }
    if p.mode != AccessMode::Legacy {
        conn.execute_batch("CREATE TABLE IF NOT EXISTS client_access(client_id TEXT PRIMARY KEY,policy TEXT NOT NULL); CREATE TABLE IF NOT EXISTS client_policy_requirements(client_id TEXT PRIMARY KEY)")?;
        conn.execute(
            "INSERT INTO client_policy_requirements VALUES (?)",
            [client],
        )?;
        conn.execute(
            "INSERT INTO client_access VALUES (?,?)",
            params![client, serde_json::to_string(p)?],
        )?;
    }
    Ok(())
}
