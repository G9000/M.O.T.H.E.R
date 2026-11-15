//! Pinned core memories: a small owner-endorsed set that every context read includes.
//!
//! Shared validators preserve pin identity and attribution in native exports.
use crate::{
    contracts::{self, string},
    error::{Error, Result, invalid},
};
use serde_json::Value;
use std::collections::HashSet;

/// Most pins a workspace can hold, so the pinned core stays small enough to read every time.
pub(crate) const MAX_PINS: usize = 12;

fn pinnable(kind: &str) -> bool {
    matches!(kind, "memory" | "decision")
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
