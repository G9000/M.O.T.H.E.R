//! Projection used only for clients that request a limit on the complete MCP reply.
use super::{CallToolResult, Value, fits, json, repaginate};

fn fields(value: &Value, keys: &[&str]) -> Value {
    Value::Object(
        keys.iter()
            .filter_map(|k| value.get(k).map(|v| ((*k).into(), v.clone())))
            .collect(),
    )
}

/// Keep a UTF-8 prefix without splitting a scalar value. The final size check also
/// counts JSON escaping and the MCP text copy of the structured response.
fn clip(value: &mut Value, key: &str, bytes: usize) -> bool {
    let Some(text) = value[key].as_str().filter(|s| s.len() > bytes) else {
        return false;
    };
    let end = text.floor_char_boundary(bytes);
    value[key] = json!(&text[..end]);
    true
}

fn preview(source: &Value, bytes: usize) -> Value {
    let mut card = source.clone();
    let title_truncated = clip(&mut card, "title", bytes);
    if title_truncated {
        card["titleTruncated"] = json!(true);
    }
    if clip(&mut card, "excerpt", bytes) {
        card["truncated"] = json!(true);
    }
    if let Some(m) = card.get_mut("match")
        && clip(m, "text", bytes)
    {
        m["truncated"] = json!(true);
    }
    card
}

/// Build the header once per attempt. Field summaries describe any material
/// omitted from the delivery. Compacting a page never modifies stored values.
fn header(source: &Value, bytes: usize, include_extras: bool) -> Value {
    let mut page = fields(
        source,
        &["workspace", "access", "capturedAt", "contextSections"],
    );
    let mut budget = json!({"maxBytes":bytes,"reason":"byte_budget","compacted":true,"originalsTool":"mother_read_original"});
    let omitted: Vec<_> = [
        "note",
        "summary",
        "inventory",
        "unsettledWork",
        "unsettledWorkSummary",
        "unsettledWorkTruncated",
        "assignedToYou",
        "assignedToYouSummary",
        "harnesses",
    ]
    .into_iter()
    .filter(|k| source.get(*k).is_some())
    .collect();
    if !omitted.is_empty() {
        budget["omittedSections"] = json!(omitted);
    }
    if let Some(s) = source.get("selection") {
        page["selection"] = fields(s, &["order", "scope", "profile", "matchingRecords"]);
    }
    if let Some(s) = source.get("search") {
        page["search"] = fields(
            s,
            &[
                "requestedMode",
                "effectiveMode",
                "semanticUsed",
                "reason",
                "scope",
            ],
        );
    }
    if let Some(persona) = source.get("persona") {
        let mut compact = fields(persona, &["hash"]);
        if include_extras {
            compact["content"] = persona["content"].clone();
            if clip(&mut compact, "content", 80) {
                compact["truncated"] = json!(true);
            }
        } else {
            compact["omitted"] = json!(true);
        }
        if compact["omitted"] == true || compact["truncated"] == true {
            page["contextSections"]["persona"] = json!(if include_extras {
                "truncated_budget"
            } else {
                "omitted_budget"
            });
        }
        page["persona"] = compact;
    }
    if let Some(pins) = source["pinned"].as_array() {
        let mut kept = Vec::new();
        if include_extras {
            for pin in pins.iter().take(1) {
                let mut compact = fields(
                    pin,
                    &[
                        "id",
                        "kind",
                        "title",
                        "revision",
                        "pinnedBy",
                        "content",
                        "truncated",
                    ],
                );
                if clip(&mut compact, "title", 64) {
                    compact["titleTruncated"] = json!(true);
                }
                if clip(&mut compact, "content", 64) {
                    compact["truncated"] = json!(true);
                }
                kept.push(compact);
            }
        }
        let mut summary = source["pinnedSummary"].clone();
        summary["shown"] = json!(kept.len());
        if kept.len() < pins.len() {
            summary["omittedForBudget"] = json!(pins.len() - kept.len());
        }
        page["pinnedSummary"] = summary;
        page["pinned"] = json!(kept);
    }
    let pagination = &source["pagination"];
    page["pagination"] = fields(
        pagination,
        &[
            "order",
            "offset",
            "matchingRecords",
            "returnedRecords",
            "remainingAfterPage",
            "hasMore",
            "nextCursor",
            "consistency",
        ],
    );
    if let Some(changed) = pagination["changed"].as_array() {
        let shown_changes = if include_extras {
            changed.as_slice()
        } else {
            &[]
        };
        page["pagination"]["changed"] = json!(
            shown_changes
                .iter()
                .map(|c| fields(c, &["id", "revision"]))
                .collect::<Vec<_>>()
        );
        let shown = page["pagination"]["changed"].as_array().map_or(0, Vec::len);
        if shown < changed.len() {
            page["pagination"]["changedOmittedForBudget"] = json!(changed.len() - shown);
            page["pagination"]["changesRecovery"] =
                json!("Restart without cursor to include changed records.");
        }
    }
    page["responseBudget"] = budget;
    page
}

/// Compact scope, change and omission metadata still travel with the record and
/// its cursor when the smallest client cannot accommodate the regular header.
fn minimum_header(mut page: Value) -> Value {
    let mut out = fields(&page, &["selection", "search", "responseBudget"]);
    out["access"] = fields(&page["access"], &["mode"]);
    if page["access"]["subject"].is_string() {
        out["access"]["subjectOmittedForBudget"] = json!(true);
    }
    out["workspace"] = fields(&page["workspace"], &["id", "generation"]);
    out["pagination"] = fields(
        &page["pagination"],
        &[
            "order",
            "returnedRecords",
            "remainingAfterPage",
            "hasMore",
            "nextCursor",
            "changedOmittedForBudget",
            "changesRecovery",
        ],
    );
    if let Some(persona) = page.get("persona") {
        out["persona"] = fields(persona, &["omitted"]);
    }
    if let Some(pins) = page.get("pinnedSummary") {
        out["pinnedSummary"] = fields(pins, &["shown", "total", "omittedForBudget", "excluded"]);
        out["pinned"] = page["pinned"].take();
    }
    out
}

/// Rebuild pagination from the untrimmed domain page for every prefix tested.
/// This also works on the final domain page, which carries a repositionable cursor.
fn candidate(base: &Value, source: &Value, cards: &[Value], shown: usize) -> Option<Value> {
    let mut page = base.clone();
    page["records"] = json!(&cards[..shown]);
    let dropped = cards.len() - shown;
    page["omitted"] = json!(source["omitted"].as_u64()? + dropped as u64);
    page["responseBudget"]["omittedItems"] = json!(dropped);
    if page.get("selection").is_some() {
        page["selection"]["returnedRecords"] = json!(shown);
        page["selection"]["omittedRecords"] = page["omitted"].clone();
    }
    if dropped > 0 {
        repaginate(&mut page, dropped)?;
    }
    if page["pagination"]["hasMore"] == false {
        page["pagination"]["nextCursor"] = Value::Null;
    }
    Some(page)
}

pub(super) fn reply(mut source: Value, bytes: usize) -> Option<CallToolResult> {
    source["responseBudget"] = json!({"maxBytes":bytes,"compacted":false,"omittedItems":0});
    let full = CallToolResult::structured(source);
    if fits(&full, bytes) {
        return Some(full);
    }
    let source = full.structured_content?;
    let records = source["records"].as_array()?;
    // Give the best match a useful excerpt before allocating space to subsequent hits.
    // Shorter prefixes and omission of optional sections are fallbacks for tight budgets.
    for (extras, excerpt_bytes) in [(true, 240), (false, 120), (false, 32)] {
        let mut base = header(&source, bytes, extras);
        if excerpt_bytes == 32 {
            base = minimum_header(base);
        }
        let cards: Vec<_> = records
            .iter()
            .map(|r| {
                let card = preview(r, excerpt_bytes);
                if excerpt_bytes == 32 {
                    // Preserve the ordering keys so the continuation starts precisely after
                    // this record. Omit detailed ranking spans in the smallest presentation.
                    fields(
                        &card,
                        &[
                            "id",
                            "kind",
                            "revision",
                            "actor",
                            "basis",
                            "title",
                            "titleTruncated",
                            "excerpt",
                            "truncated",
                            "createdAt",
                            "updatedAt",
                        ],
                    )
                } else {
                    card
                }
            })
            .collect();
        let minimum = usize::from(!cards.is_empty());
        let first = CallToolResult::structured(candidate(&base, &source, &cards, minimum)?);
        let mut best = if fits(&first, bytes) {
            Some(first)
        } else {
            None
        };
        // The last page can be smaller because it does not need a continuation token.
        if cards.len() > minimum {
            let last = CallToolResult::structured(candidate(&base, &source, &cards, cards.len())?);
            if fits(&last, bytes) {
                return Some(last);
            }
        }
        if best.is_none() {
            continue;
        }
        let mut lo = minimum + 1;
        let mut hi = cards.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let reply = CallToolResult::structured(candidate(&base, &source, &cards, mid)?);
            if fits(&reply, bytes) {
                best = Some(reply);
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return best;
    }
    None
}
