//! Project the freshness of derived accounts from their explicit record references.
//! Canonical records, history and proposal receipts are never changed by a read.
use super::{ClientAccess, PROPOSALS, RECORDS, Result, Value, View, inbox, record, settings};
use serde_json::json;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

fn derived(r: &Value) -> bool {
    r["kind"] == "checkpoint" || (r["kind"] == "memory" && r["basis"] == "model_inference")
}

type RevisionBindings = HashMap<(String, u64), HashMap<String, u64>>;
#[derive(Default)]
pub(super) struct Review {
    bindings: Option<RevisionBindings>,
}

impl Review {
    /// Reviewed gardening proposals preserve the exact source revision the owner accepted.
    fn bound_revision(&mut self, tx: &impl View, summary: &Value, id: &str) -> Result<Option<u64>> {
        if summary["actor"] != inbox::OWNER_ACTOR {
            return Ok(None);
        }
        if self.bindings.is_none() {
            let mut index = HashMap::new();
            for prop in tx.values(PROPOSALS)? {
                if prop["status"] != "saved" {
                    continue;
                }
                let written = &prop["receipt"]["receipt"]["record"];
                let (Some(key), Some(rev)) = (written["id"].as_str(), written["revision"].as_u64())
                else {
                    continue;
                };
                if let Some(bindings) =
                    crate::gardening::bindings(prop["source"].as_str().unwrap_or_default())?
                {
                    index.insert(
                        (key.to_owned(), rev),
                        bindings
                            .into_iter()
                            .map(|(id, v)| (id.to_owned(), v))
                            .collect(),
                    );
                }
            }
            self.bindings = Some(index);
        }
        Ok(self
            .bindings
            .as_ref()
            .and_then(|i| {
                i.get(&(
                    summary["id"].as_str()?.to_owned(),
                    summary["revision"].as_u64()?,
                ))
            })
            .and_then(|b| b.get(id))
            .copied())
    }

    pub(super) fn annotate(
        &mut self,
        tx: &impl View,
        r: &mut Value,
        visible: &impl Fn(&str) -> bool,
    ) -> Result<()> {
        if let Some(flag) = self.flag(tx, r, visible)? {
            r["sourceReview"] = flag;
        }
        Ok(())
    }

    fn flag(
        &mut self,
        tx: &impl View,
        r: &Value,
        visible: &impl Fn(&str) -> bool,
    ) -> Result<Option<Value>> {
        if !derived(r)
            || r["references"]
                .as_array()
                .is_none_or(|refs| refs.is_empty())
        {
            return Ok(None);
        }
        let mut stack = vec![Cow::Borrowed(r)];
        let mut visited = HashSet::new();
        let mut edges = 0;
        while let Some(summary) = stack.pop() {
            if !visited.insert(summary["id"].as_str().unwrap_or_default().to_owned()) {
                continue;
            }
            for id in summary["references"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|v| v.as_str())
            {
                edges += 1;
                if edges > 1024 {
                    return Ok(Some(
                        json!({"state":"needs_review","reason":"dependency_limit"}),
                    ));
                }
                if !visible(id) {
                    return Ok(Some(
                        json!({"state":"needs_review","reason":"unverified_reference"}),
                    ));
                }
                let Some(mut source) = tx.get(RECORDS, id)? else {
                    return Ok(Some(
                        json!({"state":"needs_review","reason":"unverified_reference"}),
                    ));
                };
                let rev = source["revision"].as_u64().unwrap_or(1);
                let retired = !super::recallable(&source);
                let needs_review = if retired {
                    true
                } else if rev == 1 {
                    false
                } else {
                    match self.bound_revision(tx, &summary, id)? {
                        Some(bound) => bound != rev,
                        // Older plain references have no revision. At equal clock precision
                        // chronology is ambiguous, so conservatively require review.
                        None => source["updatedAt"].as_str() >= summary["updatedAt"].as_str(),
                    }
                };
                if needs_review {
                    return Ok(Some(
                        json!({"state":"needs_review","reason":if retired {"retired_reference"} else {"revised_reference"},"sourceId":id,"sourceRevision":rev}),
                    ));
                }
                if derived(&source) && !visited.contains(id) {
                    if let Some(fields) = source.as_object_mut() {
                        fields.retain(|key, _| {
                            matches!(
                                key.as_str(),
                                "id" | "revision" | "actor" | "updatedAt" | "references"
                            )
                        });
                    }
                    stack.push(Cow::Owned(source));
                }
            }
        }
        Ok(None)
    }
}

pub(super) fn inspect(tx: &impl View, access: &ClientAccess, id: &str) -> Result<Value> {
    access.require(id)?;
    let mut r = record(tx, id)?;
    if derived(&r) {
        let settings = settings::Settings::load(tx)?;
        Review::default().annotate(tx, &mut r, &|source| {
            access.allows(source) && !settings.hidden(source)
        })?;
    }
    Ok(r)
}

pub(super) fn records(tx: &impl View, visible: impl Fn(&str) -> bool) -> Result<Vec<Value>> {
    let mut review = Review::default();
    let mut records = tx.values(RECORDS)?;
    records.retain(|r| r["id"].as_str().is_some_and(&visible));
    for r in &mut records {
        review.annotate(tx, r, &visible)?;
    }
    Ok(records)
}
