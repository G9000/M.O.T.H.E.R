//! Versioned source revision bindings for Memory Gardening proposals.
use crate::{
    contracts,
    error::{Result, invalid},
};
use std::collections::BTreeSet;

const PREFIX: &str = "Memory Gardening v2 evidence: ";

/// Produce the existing inbox source attribution string with exact evidence bindings.
pub fn source<'a>(evidence: impl IntoIterator<Item = (&'a str, u64)>) -> String {
    let list = evidence
        .into_iter()
        .map(|(id, revision)| format!("{id}@{revision}"))
        .collect::<Vec<_>>();
    format!("{PREFIX}{}", list.join(","))
}

pub(crate) fn bindings(source: &str) -> Result<Option<Vec<(&str, u64)>>> {
    let Some(raw) = source.strip_prefix(PREFIX) else {
        return Ok(None);
    };
    let mut bindings = Vec::new();
    let mut seen = BTreeSet::new();
    for reference in raw.split(',') {
        let (id, revision) = reference.split_once('@').ok_or_else(invalid_binding)?;
        contracts::id(id).map_err(|_| invalid_binding())?;
        let revision = revision
            .parse::<u64>()
            .ok()
            .filter(|rev| (1..=9_007_199_254_740_991).contains(rev))
            .ok_or_else(invalid_binding)?;
        if !seen.insert(id) || bindings.len() == 2 {
            return Err(invalid_binding());
        }
        bindings.push((id, revision));
    }
    Ok(Some(bindings))
}

fn invalid_binding() -> crate::error::Error {
    invalid("Invalid Memory Gardening source revision binding.")
}
