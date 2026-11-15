//! Addressed handoffs: an open task named for one harness, found when that harness reads context.
//!
//! The harness is named by its grant (client ID). Nothing here runs work: the
//! receiving harness picks the task up under its own permissions and moves it
//! through the ordinary task statuses. Shared validators preserve assignment attribution
//! when exporting and restoring native records.
use crate::{
    contracts::{self, string, valid_client},
    error::{Error, Result, invalid},
};
use serde_json::Value;
use std::collections::HashSet;

/// Check an export's assignments against its records before anything is restored.
pub(crate) fn validate(assignments: &Value, records: &[Value]) -> Result<()> {
    validate_with_limits(assignments, records, true)
}
pub(crate) fn validate_with_limits(
    assignments: &Value,
    records: &[Value],
    small: bool,
) -> Result<()> {
    let list = assignments
        .as_array()
        .ok_or_else(|| invalid("Invalid assignments"))?;
    if small && list.len() > 10_000 {
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
