use super::{
    dataset::{Case, Request},
    fixture::{EvalResult, Fixture},
    metrics::{self, Ranking},
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, time::Instant};

fn coverage_failures(value: &Value, case: &Case) -> Vec<String> {
    let mut failures = Vec::new();
    match serde_json::from_value::<mother::garden::contracts::Coverage>(value.clone()) {
        Err(_) => failures.push("Missing or malformed typed graph coverage".into()),
        Ok(coverage) => {
            if coverage.index_watermark > coverage.canonical_watermark
                || (coverage.complete && coverage.truncated)
                || (coverage.truncated && coverage.reason.as_ref().is_none_or(String::is_empty))
            {
                failures.push("Inconsistent graph coverage/watermark".into());
            }
            if case
                .expected_graph_truncated
                .is_some_and(|expected| coverage.truncated != expected)
                || case
                    .expected_coverage_reason
                    .as_ref()
                    .is_some_and(|reason| coverage.reason.as_ref() != Some(reason))
            {
                failures.push("Expected graph budget exhaustion was not reported".into());
            }
        }
    }
    failures
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultRow {
    pub id: String,
    pub scenario: String,
    pub split: String,
    pub category: String,
    pub language: String,
    pub lane: String,
    pub mode: String,
    pub pass: String,
    pub gate: String,
    pub request: Value,
    pub expected: Value,
    pub status: String,
    pub actual: Vec<String>,
    pub revisions: BTreeMap<String, u64>,
    pub ranking: Option<Ranking>,
    pub search: Value,
    pub coverage: Value,
    pub response_bytes: usize,
    pub milliseconds: f64,
    pub error: Option<String>,
    pub failures: Vec<String>,
    pub hard_failure: bool,
}

fn code(error: &(dyn std::error::Error + 'static)) -> String {
    if let Some(e) = error.downcast_ref::<mother::error::Error>() {
        return e.code().into();
    }
    if let Some(e) = error.downcast_ref::<mother::garden::contracts::GardenError>() {
        return serde_json::to_value(e.code)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned))
            .unwrap_or_else(|| "UNKNOWN_GRAPH_ERROR".into());
    }
    format!("EVALUATOR_ERROR: {error}")
}
fn execute(f: &Fixture, request: &Request, mode: &str) -> EvalResult<Value> {
    let auth = f.auth(&request.principal);
    match request.operation.as_str() {
        "search" => {
            let mut args = json!({"query":request.query,"mode":mode,"limit":5});
            if let Some(kind) = &request.kind {
                args["kind"] = json!(kind);
            }
            Ok(f.search.search(&f.store, auth, args)?)
        }
        "get" => Ok(f.store.v1_call(
            auth,
            "mother_get_record",
            json!({"id":f.ids[request.target.as_deref().ok_or("Missing target")?]}),
        )?),
        "denied_write" => f.denied_write(request),
        "graph_get" => f.graph_get(request),
        "graph_query" => f.graph_query(request),
        _ => Err("Unsupported evaluation request".into()),
    }
}

pub fn run_case(f: &Fixture, case: &Case, mode: &str, pass: &str, k: usize) -> ResultRow {
    let input = case.input();
    let start = Instant::now();
    // No oracle fields reach execute/NativeSearch/graph requests.
    let result = execute(f, &input, mode);
    let milliseconds = start.elapsed().as_secs_f64() * 1000.0;
    let mut row = ResultRow {
        id: case.id.clone(),
        scenario: case.scenario.clone(),
        split: case.split.clone(),
        category: case.category.clone(),
        language: case.language.clone(),
        lane: if case.graph() {
            "graph_contract"
        } else {
            "v1_retrieval"
        }
        .into(),
        mode: mode.into(),
        pass: pass.into(),
        gate: case.gate.clone(),
        request: json!({"operation":input.operation,"query":input.query,"kind":input.kind,"target":input.target,"principal":input.principal,"args":input.args}),
        expected: json!({"relevant":case.relevant,"forbidden":case.forbidden,"revisions":case.expected_revisions,"error":case.expected_error,"empty":case.expect_empty,"minimumOmitted":case.minimum_omitted,"previewTruncated":case.expected_preview_truncated,"requireTopHit":case.require_top_hit}),
        status: "passed".into(),
        actual: vec![],
        revisions: BTreeMap::new(),
        ranking: None,
        search: Value::Null,
        coverage: Value::Null,
        response_bytes: 0,
        milliseconds,
        error: None,
        failures: vec![],
        hard_failure: false,
    };
    row.expected["graphBudget"] = json!({"allowedResults":case.allowed_results,"maximumHits":case.maximum_hits,"truncated":case.expected_graph_truncated,"reason":case.expected_coverage_reason});
    let value = match result {
        Ok(value) => {
            if case.expected_error.is_some() {
                row.failures.push("Expected request to fail".into());
            }
            value
        }
        Err(error) => {
            let code = code(error.as_ref());
            row.error = Some(code.clone());
            if case.expected_error.as_deref() != Some(&code) {
                row.status = "error".into();
                row.hard_failure = true;
                row.failures
                    .push("Unexpected request error; no ranking assigned".into());
            }
            return row;
        }
    };
    row.response_bytes = serde_json::to_vec(&value)
        .map(|b| b.len())
        .unwrap_or(usize::MAX);
    row.search = value.get("search").cloned().unwrap_or(Value::Null);
    row.coverage = if case.graph() {
        json!({"coverage":value["coverage"],"page":value["page"]})
    } else {
        json!({"omitted":value["omitted"],"truncated":value["truncated"]})
    };
    if input.operation == "search" && !value["records"].is_array() {
        row.failures.push("Missing records array".into());
    }
    if case.graph() {
        row.failures
            .extend(coverage_failures(&value["coverage"], case));
    }
    if let Some(minimum) = case.minimum_omitted
        && value["omitted"].as_u64().is_none_or(|n| n < minimum)
    {
        row.failures
            .push("Expected explicit omitted candidates".into());
    }
    let records = match input.operation.as_str() {
        "search" => value["records"].as_array().cloned().unwrap_or_default(),
        "graph_query" => value["hits"]
            .as_array()
            .map(|a| a.iter().map(|h| h["record"].clone()).collect())
            .unwrap_or_default(),
        "graph_get" => vec![value["record"].clone()],
        "get" => vec![value.clone()],
        _ => vec![],
    };
    if records.len() > k {
        row.failures
            .push("Response exceeds the fixed hit budget".into());
    }
    if row.response_bytes > 262_144 {
        row.failures
            .push("Response exceeds evaluation byte budget".into());
    }
    if case.maximum_hits.is_some_and(|max| records.len() > max) {
        row.failures
            .push("Graph result exceeds declared work-budget hit bound".into());
    }
    for record in &records {
        let (id, revision) = if case.graph() {
            (&record["metadata"]["id"], &record["metadata"]["revision"])
        } else {
            (&record["id"], &record["revision"])
        };
        let Some(alias) = id.as_str().and_then(|s| f.alias(s)) else {
            row.failures.push("Unknown returned fixture ID".into());
            continue;
        };
        let Some(revision) = revision.as_u64() else {
            row.failures.push("Missing revision".into());
            continue;
        };
        row.actual.push(alias.clone());
        if !case.allowed_results.is_empty() && !case.allowed_results.contains(&alias) {
            row.failures
                .push("Graph budget returned an ineligible candidate".into());
        }
        row.revisions.insert(alias.clone(), revision);
        if case.forbidden.contains(&alias) {
            row.failures.push(format!("Forbidden result: {alias}"));
        }
        if let Some(expected) = case.expected_revisions.get(&alias)
            && *expected != revision
        {
            row.failures.push(format!("Wrong revision for {alias}"));
        }
        let verification: EvalResult<Value> = if case.graph() {
            Ok(record.clone())
        } else {
            f.store
                .v1_call(
                    f.auth(&input.principal),
                    "mother_get_record",
                    json!({"id":id}),
                )
                .map_err(Into::into)
        };
        match verification {
            Ok(full) => {
                let expected = if case.graph() {
                    f.historical.get(&(alias.clone(), revision))
                } else {
                    f.canonical.get(&alias)
                };
                if expected != Some(&full) {
                    row.failures.push(format!(
                        "Full source/revision/attribution mismatch: {alias}"
                    ));
                }
                if !case.graph() && full["status"] != "active" {
                    row.failures
                        .push(format!("Inactive record returned: {alias}"));
                }
                if !case.graph() && record.get("excerpt").is_some() {
                    let content = full["content"].as_str().unwrap_or_default();
                    let excerpt = record["excerpt"].as_str().unwrap_or_default();
                    let expected_truncated = content.encode_utf16().count() > 400;
                    if record["truncated"].as_bool() != Some(expected_truncated)
                        || excerpt.encode_utf16().count() > 400
                        || !content.starts_with(excerpt.trim_end_matches('…'))
                    {
                        row.failures
                            .push(format!("Preview bounds/coverage mismatch: {alias}"));
                    }
                    if let Some(required) = case.expected_preview_truncated
                        && record["truncated"] != required
                    {
                        row.failures
                            .push("Expected preview truncation was not reported".into());
                    }
                }
            }
            Err(_) => row.failures.push(format!(
                "Returned reference cannot be fully resolved: {alias}"
            )),
        }
    }
    if input.operation == "search" {
        if !row.search.is_object()
            || !row.search["semanticUsed"].is_boolean()
            || !value["omitted"].is_u64()
        {
            row.failures.push("Missing search/omission metadata".into());
        }
        if row.search["effectiveMode"] == "literal" && row.search["semanticUsed"] != false {
            row.failures
                .push("Literal fallback claims semantic participation".into());
        }
        if row.search["requestedMode"] != mode
            || row.search["effectiveMode"] != "literal"
            || row.search["semanticUsed"] != false
            || !row.search["model"].is_null()
            || (mode != "literal" && row.search["reason"] != "model_unavailable")
        {
            row.failures
                .push("No-model run misreported mode or fallback reason".into());
        }
    }
    row.hard_failure = !row.failures.is_empty();
    if case.expect_empty && !row.actual.is_empty() {
        row.failures
            .push("Expected no evidence; retrieval returned candidates".into());
    }
    row.ranking = metrics::rank(&row.actual, &case.relevant, k);
    if case.require_top_hit
        && row
            .actual
            .first()
            .is_none_or(|id| !case.relevant.contains_key(id))
    {
        row.failures.push("Top hit is not relevant evidence".into());
    }
    if row.ranking.as_ref().is_some_and(|r| !r.all_evidence) {
        row.failures
            .push("Required evidence missing at fixed cutoff".into());
    }
    if !row.failures.is_empty() {
        row.status = "failed".into();
        row.hard_failure |= case.gate != "quality";
    }
    row
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coverage_grader_rejects_missing_fields_and_false_budget_success() {
        let (_, suite) = crate::evaluation::dataset::load().unwrap();
        let case = suite.cases.iter().find(|c| c.id == "graph-budget").unwrap();
        assert!(!coverage_failures(&json!({}), case).is_empty());
        let dishonest = json!({"complete":true,"truncated":false,"reason":null,"canonicalWatermark":4,"indexWatermark":4,"searchMode":"literal","model":null,"profile":null});
        assert!(!coverage_failures(&dishonest, case).is_empty());
    }
    #[test]
    fn grading_catches_wrong_time_candidates_and_denials_with_effects() {
        let (corpus, suite) = crate::evaluation::dataset::load().unwrap();
        let fixture = Fixture::new(&corpus).unwrap();
        let mut temporal = suite
            .cases
            .iter()
            .find(|c| c.id == "graph-valid-before")
            .unwrap()
            .clone();
        temporal.args["validAt"] = Value::Null;
        let row = run_case(&fixture, &temporal, "structured", "control", 5);
        assert_eq!(row.status, "failed");
        assert!(row.hard_failure);
        assert!(
            row.failures
                .iter()
                .any(|s| s == "Forbidden result: new_city")
        );
        let mut write = suite
            .cases
            .iter()
            .find(|c| c.id == "read-only-write")
            .unwrap()
            .clone();
        write.principal = Some("owner".into());
        let row = run_case(&fixture, &write, "literal", "control", 5);
        assert_eq!(row.status, "error");
        assert!(row.hard_failure);
        assert!(
            row.error
                .unwrap()
                .contains("denied write changed canonical backup")
        );
    }
}
