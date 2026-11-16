//! MCP read presentation. Canonical values and mutation receipts are not rewritten.
use rmcp::model::CallToolResult;
use serde_json::{Value, json};

// Leaves room for the receiving harness's 16 KiB result envelope. A structured
// reply can also contain the same JSON as escaped text on the MCP wire.
const PREVIEW_BYTES: usize = 12 * 1024;
const READ_REPLY_BYTES: usize = 48 * 1024;

fn fits(value: &impl serde::Serialize, budget: usize) -> bool {
    serde_json::to_vec(value).is_ok_and(|bytes| bytes.len() <= budget)
}

fn size_error() -> CallToolResult {
    CallToolResult::structured_error(json!({"error":{
        "code":"READ_RESULT_TOO_LARGE",
        "message":"Read result exceeds the MCP response budget. Use a smaller targeted query; use owner inspection/export for an oversized full record or required context. Persona, pinned/assigned context and canonical records were not silently truncated. No memory was changed.",
        "restartRequired":false
    }}))
}

fn increment(value: &mut Value, key: &str) -> Option<()> {
    let next = value.get(key)?.as_u64()?.checked_add(1)?;
    value[key] = json!(next);
    Some(())
}

fn removal_index(value: &Value, collection: &str) -> Option<usize> {
    let items = value[collection].as_array()?;
    if collection == "records" && value["selection"]["order"] == "balanced" {
        // Domain selection grants seats by category, then displays newest first.
        // Trim the most represented category, not the globally oldest record.
        let seats = [
            "checkpoint",
            "task",
            "decision",
            "memory",
            "action",
            "artifact",
        ];
        let (kind, _) = seats
            .iter()
            .enumerate()
            .map(|(seat, kind)| {
                let count = items.iter().filter(|item| item["kind"] == *kind).count();
                (*kind, (count, seat))
            })
            .filter(|(_, (count, _))| *count > 0)
            .max_by_key(|(_, score)| *score)?;
        items.iter().rposition(|item| item["kind"] == kind)
    } else {
        items.len().checked_sub(1)
    }
}

fn bound_previews(mut value: Value, collection: &str) -> Option<Value> {
    let mut removed = 0;
    while !fits(&value, PREVIEW_BYTES) {
        // Remove whole navigation items only. Required context stays untouched;
        // if it alone is too large, return an explicit error instead of a fragment.
        let index = removal_index(&value, collection)?;
        let item = value.get_mut(collection)?.as_array_mut()?.remove(index);
        removed += 1;
        let shown = value[collection].as_array()?.len();
        if collection == "revisions" {
            value["returnedRevisions"] = json!(shown);
            increment(&mut value, "omittedRevisions")?;
        } else {
            increment(&mut value, "omitted")?;
            if let Some(selection) = value.get_mut("selection") {
                selection["returnedRecords"] = json!(shown);
                increment(selection, "omittedRecords")?;
                let kind = item.get("kind")?.as_str()?;
                let remaining = selection["returnedByKind"][kind].as_u64()?.checked_sub(1)?;
                selection["returnedByKind"][kind] = json!(remaining);
                increment(&mut selection["omittedByKind"], kind)?;
                let matching = selection["matchingRecords"].as_u64()?;
                let total = value["inventory"]["recallableRecords"].as_u64()?;
                value["summary"] = json!(format!(
                    "Shared context has {total} recallable records. This request matches {matching} records and returns {shown} previews within the response byte budget."
                ));
            }
        }
        value["responseBudget"] =
            json!({"maxBytes":PREVIEW_BYTES,"omittedItems":removed,"reason":"byte_budget"});
    }
    Some(value)
}

fn is_read(name: &str) -> bool {
    matches!(
        name,
        "mother_get_context"
            | "mother_search_context"
            | "mother_get_record"
            | "mother_get_record_history"
            | "mother_get_receipt"
            | "mother_get_proposal"
            | "mother_get_capabilities"
            | "mother_garden_get"
            | "mother_garden_query"
            | "mother_garden_get_proposal"
            | "mother_garden_get_receipt"
            | "mother_garden_changes"
    )
}

pub(super) fn result(name: &str, value: Value) -> CallToolResult {
    let collection = match name {
        "mother_get_context" | "mother_search_context" => Some("records"),
        "mother_get_record_history" if value["revisions"].is_array() => Some("revisions"),
        _ => None,
    };
    let value = match collection {
        Some(collection) => match bound_previews(value, collection) {
            Some(value) => value,
            None => return size_error(),
        },
        None => value,
    };
    let reply = CallToolResult::structured(value);
    if is_read(name) && !fits(&reply, READ_REPLY_BYTES) {
        return size_error();
    }
    reply
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const BUDGET: usize = 12 * 1024;

    fn context() -> Value {
        let counts =
            json!({"memory":70,"checkpoint":70,"decision":0,"task":0,"artifact":0,"action":0});
        let shown =
            json!({"memory":50,"checkpoint":50,"decision":0,"task":0,"artifact":0,"action":0});
        let omitted =
            json!({"memory":20,"checkpoint":20,"decision":0,"task":0,"artifact":0,"action":0});
        let records: Vec<_> = (0..100).map(|i| json!({
            "id":uuid::Uuid::new_v4().to_string(),"kind":if i%2==0 {"memory"} else {"checkpoint"},
            "title":format!("Synthetic memory {i}"),"status":"active","revision":1,
            "actor":"synthetic","updatedAt":"2026-11-16T00:00:00Z",
            "excerpt":"記憶\\\"".repeat(70),"truncated":true
        })).collect();
        json!({"workspace":{"id":uuid::Uuid::new_v4().to_string(),"generation":uuid::Uuid::new_v4().to_string(),"schemaVersion":1},
            "access":{"version":1,"mode":"companion","subject":"synthetic-agent"},
            "capturedAt":"2026-11-16T00:00:00Z","persona":{"content":"p".repeat(7_600),"hash":"synthetic"},
            "inventory":{"totalRecords":140,"recallableRecords":140,"excludedRecords":0,"byKind":counts,"recallableByKind":counts},
            "selection":{"query":null,"kind":null,"order":"balanced","matchingRecords":140,
                "returnedRecords":100,"omittedRecords":40,"returnedByKind":shown,"omittedByKind":omitted},
            "summary":"140 recallable records; this request returns 100 previews.","records":records,"omitted":40,
            "pinned":[],"pinnedSummary":{"total":0,"shown":0,"inactive":0,"limit":12},
            "assignedToYou":[],"assignedToYouSummary":{"total":0,"shown":0},
            "unsettledWork":[],"unsettledWorkSummary":{"total":0,"returned":0,"omitted":0},"unsettledWorkTruncated":false,
            "harnesses":["synthetic-first","synthetic-second"]})
    }

    fn data(reply: &CallToolResult) -> Value {
        let reply = serde_json::to_value(reply).unwrap();
        assert_ne!(reply["isError"], true);
        reply["structuredContent"].clone()
    }

    #[test]
    fn preview_context_fits_actual_wire_and_keeps_counts_and_complete_items() {
        let original = context();
        let reply = result("mother_get_context", original.clone());
        let value = data(&reply);
        assert!(serde_json::to_vec(&value).unwrap().len() <= BUDGET);
        let frame = json!({"jsonrpc":"2.0","id":"x".repeat(12*1024),"result":reply});
        assert!(serde_json::to_vec(&frame).unwrap().len() < 64 * 1024);
        let records = value["records"].as_array().unwrap();
        assert!(!records.is_empty() && records.len() < 100);
        assert_eq!(value["inventory"], original["inventory"]);
        for key in [
            "workspace",
            "access",
            "persona",
            "pinned",
            "assignedToYou",
            "harnesses",
            "unsettledWorkSummary",
        ] {
            assert_eq!(value[key], original[key], "required field {key}");
        }
        let original_records = original["records"].as_array().unwrap();
        let mut positions = Vec::new();
        for actual in records {
            let position = original_records
                .iter()
                .position(|record| record["id"] == actual["id"])
                .unwrap();
            assert_eq!(
                actual, &original_records[position],
                "never cut a preview mid-value"
            );
            positions.push(position);
        }
        assert!(
            positions.windows(2).all(|pair| pair[0] < pair[1]),
            "retained display order must stay deterministic"
        );
        assert_eq!(value["selection"]["returnedRecords"], records.len());
        assert_eq!(value["selection"]["omittedRecords"], 140 - records.len());
        assert_eq!(value["omitted"], 140 - records.len());
        for kind in ["memory", "checkpoint"] {
            let count = records.iter().filter(|r| r["kind"] == kind).count();
            assert_eq!(value["selection"]["returnedByKind"][kind], count);
            assert_eq!(value["selection"]["omittedByKind"][kind], 70 - count);
        }
        assert!(
            value["summary"]
                .as_str()
                .unwrap()
                .contains(&format!("{} previews", records.len()))
        );
        assert_eq!(value["responseBudget"]["reason"], "byte_budget");
    }

    #[test]
    fn preview_search_preserves_index_coverage_and_reports_byte_omissions() {
        let mut original = context();
        original.as_object_mut().unwrap().remove("selection");
        original["search"] = json!({"effectiveMode":"literal","eligibleRecords":140,"indexedRecords":13,"unindexedRecords":127,"partialRecords":2});
        let reply = result("mother_search_context", original.clone());
        let value = data(&reply);
        assert!(serde_json::to_vec(&value).unwrap().len() <= BUDGET);
        assert_eq!(value["search"], original["search"]);
        assert_eq!(
            value["omitted"],
            140 - value["records"].as_array().unwrap().len()
        );
    }

    #[test]
    fn preview_balanced_categories_survive_newest_first_display_order() {
        let mut original = context();
        let kinds = [
            "checkpoint",
            "task",
            "decision",
            "memory",
            "action",
            "artifact",
        ];
        let mut records = Vec::new();
        for (group, kind) in kinds.iter().enumerate() {
            for index in 0..16 {
                records.push(json!({"id":uuid::Uuid::new_v4().to_string(),"kind":kind,"title":format!("Synthetic {kind} {index}"),
                    "updatedAt":format!("2026-11-{:02}T00:00:00Z",16-group),"excerpt":"a".repeat(128),"truncated":true}));
            }
        }
        let counts: serde_json::Map<String, Value> = kinds
            .iter()
            .map(|kind| ((*kind).into(), json!(16)))
            .collect();
        let zero: serde_json::Map<String, Value> = kinds
            .iter()
            .map(|kind| ((*kind).into(), json!(0)))
            .collect();
        original["records"] = json!(records);
        original["inventory"] = json!({"totalRecords":96,"recallableRecords":96,"excludedRecords":0,"byKind":counts,"recallableByKind":counts});
        original["selection"] = json!({"order":"balanced","matchingRecords":96,"returnedRecords":96,"omittedRecords":0,"returnedByKind":counts,"omittedByKind":zero});
        original["omitted"] = json!(0);
        let value = data(&result("mother_get_context", original));
        let records = value["records"].as_array().unwrap();
        assert!(records.len() >= 6 && records.len() < 96);
        let shown: Vec<_> = kinds
            .iter()
            .map(|kind| records.iter().filter(|r| r["kind"] == *kind).count())
            .collect();
        assert!(
            shown.iter().all(|count| *count > 0),
            "older categories disappeared: {shown:?}"
        );
        assert!(shown.iter().max().unwrap() - shown.iter().min().unwrap() <= 1);
        assert_eq!(value["selection"]["order"], "balanced");
    }

    #[test]
    fn preview_required_persona_pins_and_assignments_fail_explicitly_when_too_large() {
        for key in ["persona", "pinned", "assignedToYou", "unsettledWork"] {
            let mut original = context();
            if key == "persona" {
                original[key]["content"] = json!("人".repeat(12_000));
            } else {
                original[key] =
                    json!([{"id":uuid::Uuid::new_v4().to_string(),"content":"人".repeat(12_000)}]);
            }
            let reply = serde_json::to_value(result("mother_get_context", original)).unwrap();
            assert_eq!(reply["isError"], true, "{key}");
            assert_eq!(
                reply["structuredContent"]["error"]["code"],
                "READ_RESULT_TOO_LARGE"
            );
            assert!(serde_json::to_vec(&reply).unwrap().len() < 2048);
        }
    }

    #[test]
    fn preview_history_list_is_bounded_but_full_revision_is_complete() {
        let revisions:Vec<_>=(1..=100).map(|revision|json!({"revision":revision,"excerpt":"過去\\\"".repeat(70),"changed":["content"]})).collect();
        let original = json!({"id":uuid::Uuid::new_v4().to_string(),"returnedRevisions":100,"omittedRevisions":40,"revisions":revisions});
        let value = data(&result("mother_get_record_history", original));
        assert!(serde_json::to_vec(&value).unwrap().len() <= BUDGET);
        let shown = value["revisions"].as_array().unwrap().len();
        assert_eq!(value["returnedRevisions"], shown);
        assert_eq!(value["omittedRevisions"], 140 - shown);
        let revision = json!({"id":uuid::Uuid::new_v4().to_string(),"record":{"revision":1,"content":"人".repeat(2_000)}});
        assert_eq!(
            data(&result("mother_get_record_history", revision.clone())),
            revision
        );
    }

    #[test]
    fn preview_full_reads_stay_complete_or_return_a_small_size_error() {
        let record = json!({"id":uuid::Uuid::new_v4().to_string(),"kind":"memory","content":"人".repeat(2_000)});
        assert_eq!(data(&result("mother_get_record", record.clone())), record);
        let oversized = json!({"id":uuid::Uuid::new_v4().to_string(),"kind":"memory","content":"\"".repeat(12_000)});
        let reply = serde_json::to_value(result("mother_get_record", oversized)).unwrap();
        assert_eq!(reply["isError"], true);
        assert_eq!(
            reply["structuredContent"]["error"]["code"],
            "READ_RESULT_TOO_LARGE"
        );
        assert!(serde_json::to_vec(&reply).unwrap().len() < 2048);
    }

    #[test]
    fn preview_small_values_and_mutation_receipts_are_never_changed() {
        let small = json!({"records":[],"omitted":0,"inventory":{"totalRecords":0}});
        assert_eq!(data(&result("mother_get_context", small.clone())), small);
        let receipt = json!({"operationId":uuid::Uuid::new_v4().to_string(),"record":{"content":"x".repeat(80_000)},"revision":1});
        for name in [
            "mother_save_memory",
            "mother_propose_memory",
            "mother_garden_apply",
        ] {
            assert_eq!(data(&result(name, receipt.clone())), receipt);
        }
    }
}
