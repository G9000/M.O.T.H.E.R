//! Owner-created client policies, independent of host or agent implementations.
use crate::{
    contracts,
    error::{Error, Result, invalid},
};
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
