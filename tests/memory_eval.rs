#[path = "../examples/memory_eval/mod.rs"]
mod evaluation;
use evaluation::metrics;
use std::collections::BTreeMap;

fn keys(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_owned()).collect()
}

#[test]
fn independently_computed_ranking_penalizes_partial_evidence_and_fixed_budget() {
    let gold = BTreeMap::from([("a".into(), 2), ("b".into(), 1)]);
    let got = metrics::rank(&keys(&["noise", "a"]), &gold, 5).unwrap();
    assert_eq!(got.recall, 0.5);
    assert_eq!(got.precision, 0.2);
    assert_eq!(got.reciprocal_rank, 0.5);
    let expected = (3.0 / 3.0_f64.log2()) / (3.0 + 1.0 / 3.0_f64.log2());
    assert!((got.ndcg - expected).abs() < 1e-12);
    assert!(!got.all_evidence);
}

#[test]
fn duplicates_keep_their_positions_but_never_earn_repeated_credit() {
    let gold = BTreeMap::from([("a".into(), 1), ("b".into(), 1)]);
    let got = metrics::rank(&keys(&["a", "a", "b"]), &gold, 2).unwrap();
    assert_eq!(got.recall, 0.5);
    assert_eq!(got.precision, 0.5);
    assert!(got.ndcg < 1.0);
    assert!(!got.all_evidence);
}

#[test]
fn no_answer_cases_have_no_ranking_score_and_wrong_retriever_cannot_pass() {
    assert!(metrics::rank(&[], &BTreeMap::new(), 5).is_none());
    let gold = BTreeMap::from([("a".into(), 1)]);
    let right = metrics::rank(&keys(&["a"]), &gold, 5).unwrap();
    let wrong = metrics::rank(&keys(&["b"]), &gold, 5).unwrap();
    assert!(right.ndcg > wrong.ndcg);
    assert!(right.all_evidence);
    assert_eq!(wrong.recall, 0.0);
}

#[test]
fn fixture_validation_rejects_contamination_contradictions_and_unknown_ids() {
    let (corpus, mut suite) = evaluation::dataset::load().unwrap();
    suite.cases[1].split = "holdout".into();
    assert!(
        evaluation::dataset::validate(&corpus, &suite)
            .unwrap_err()
            .contains("leakage")
    );
    let (_, mut suite) = evaluation::dataset::load().unwrap();
    suite.cases[0].forbidden.push("pet".into());
    assert!(evaluation::dataset::validate(&corpus, &suite).is_err());
    suite.cases[0].forbidden = vec!["nonexistent".into()];
    assert!(
        evaluation::dataset::validate(&corpus, &suite)
            .unwrap_err()
            .contains("Unknown")
    );
    suite.cases[0].forbidden.clear();
    suite.cases[0].relevant.insert("pet".into(), 0);
    assert!(evaluation::dataset::validate(&corpus, &suite).is_err());
}

fn remove_timings(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(fields) => {
            // This contract case permits either equally eligible edge. Membership
            // and count are graded; an arbitrary UUID tie order is not a rank oracle.
            if fields.get("id").is_some_and(|id| id == "graph-budget") {
                let count = fields["actual"].as_array().unwrap().len();
                fields.insert(
                    "actual".into(),
                    serde_json::json!({"eligibleEdgeCount":count}),
                );
                fields.insert(
                    "revisions".into(),
                    serde_json::json!({"eligibleEdgeCount":count}),
                );
            }
            // Snapshot lease IDs are opaque random capabilities, not ranked results.
            for key in [
                "milliseconds",
                "p50Ms",
                "p95Ms",
                "responseBytes",
                "responseBytesMax",
                "snapshotId",
                "visibilityEpoch",
            ] {
                fields.remove(key);
            }
            for child in fields.values_mut() {
                remove_timings(child);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                remove_timings(child);
            }
        }
        _ => {}
    }
}

fn first_difference(a: &serde_json::Value, b: &serde_json::Value, path: &str) -> Option<String> {
    if a == b {
        return None;
    }
    match (a, b) {
        (serde_json::Value::Object(left), serde_json::Value::Object(right)) => {
            for key in left.keys().chain(right.keys()) {
                let empty = serde_json::Value::Null;
                if let Some(diff) = first_difference(
                    left.get(key).unwrap_or(&empty),
                    right.get(key).unwrap_or(&empty),
                    &format!("{path}.{key}"),
                ) {
                    return Some(diff);
                }
            }
        }
        (serde_json::Value::Array(left), serde_json::Value::Array(right))
            if left.len() == right.len() =>
        {
            for (i, (l, r)) in left.iter().zip(right).enumerate() {
                if let Some(diff) = first_difference(l, r, &format!("{path}[{i}]")) {
                    return Some(diff);
                }
            }
        }
        _ => return Some(format!("{path}: {a} != {b}")),
    }
    Some(format!("Different structures at {path}"))
}

#[test]
fn independent_fresh_stores_produce_identical_baselines_and_do_not_hide_quality_misses() {
    let mut first = evaluation::run().unwrap();
    let mut second = evaluation::run().unwrap();
    // Include the report in a failure message to diagnose real API behavior, not tune gold.
    assert_eq!(
        first["gates"]["hardGatesPass"],
        true,
        "{}",
        serde_json::to_string_pretty(&first["results"]).unwrap()
    );
    assert_eq!(first["gates"]["qualityPass"], false);
    assert_eq!(first["lanes"]["animaAnswerAndToolUse"]["status"], "not_run");
    assert_eq!(evaluation::exit_code(&first, false), 0);
    assert_eq!(evaluation::exit_code(&first, true), 1);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("report.json");
    evaluation::write_new(&path, &first).unwrap();
    let original = std::fs::read(&path).unwrap();
    assert!(evaluation::write_new(&path, &second).is_err());
    assert_eq!(original, std::fs::read(&path).unwrap());
    remove_timings(&mut first);
    remove_timings(&mut second);
    assert!(
        first == second,
        "{}",
        first_difference(&first, &second, "report").unwrap()
    );
    first["gates"]["hardGatesPass"] = serde_json::json!(false);
    assert_eq!(evaluation::exit_code(&first, false), 2);
}
