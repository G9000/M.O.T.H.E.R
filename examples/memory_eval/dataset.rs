use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const CORPUS: &str = include_str!("../../evals/memory-v1/corpus.json");
pub const CASES: &str = include_str!("../../evals/memory-v1/cases.json");
pub const GRAPH_KEYS: &[&str] = &["person", "old_city", "new_city", "evidence"];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Corpus {
    pub version: u32,
    pub id: String,
    pub records: Vec<Seed>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub key: String,
    pub scenario: String,
    pub shared: bool,
    pub revisions: Vec<Value>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Suite {
    pub version: u32,
    pub id: String,
    pub k: usize,
    pub cases: Vec<Case>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub scenario: String,
    pub split: String,
    pub category: String,
    pub language: String,
    // Flattening request fields is avoided so fixture validation can reject typos.
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub principal: Option<String>,
    #[serde(default)]
    pub args: Value,
    #[serde(default)]
    pub relevant: BTreeMap<String, u8>,
    #[serde(default)]
    pub forbidden: Vec<String>,
    #[serde(default)]
    pub expected_revisions: BTreeMap<String, u64>,
    #[serde(default)]
    pub expected_error: Option<String>,
    #[serde(default)]
    pub expect_empty: bool,
    #[serde(default = "quality")]
    pub gate: String,
    #[serde(default)]
    pub minimum_omitted: Option<u64>,
    #[serde(default)]
    pub expected_preview_truncated: Option<bool>,
    #[serde(default)]
    pub require_top_hit: bool,
    #[serde(default)]
    pub allowed_results: Vec<String>,
    #[serde(default)]
    pub maximum_hits: Option<usize>,
    #[serde(default)]
    pub expected_graph_truncated: Option<bool>,
    #[serde(default)]
    pub expected_coverage_reason: Option<String>,
}
fn quality() -> String {
    "quality".into()
}

/// Adapter input deliberately excludes qrels, forbidden keys and expected answers.
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub operation: String,
    pub query: Option<String>,
    pub kind: Option<String>,
    pub target: Option<String>,
    pub principal: String,
    pub args: Value,
}
impl Case {
    pub fn input(&self) -> Request {
        Request {
            operation: self.operation.clone().unwrap_or_else(|| "search".into()),
            query: self.query.clone(),
            kind: self.kind.clone(),
            target: self.target.clone(),
            principal: self.principal.clone().unwrap_or_else(|| "owner".into()),
            args: self.args.clone(),
        }
    }
    pub fn graph(&self) -> bool {
        self.input().operation.starts_with("graph_")
    }
}

pub fn load() -> Result<(Corpus, Suite), String> {
    let corpus: Corpus = serde_json::from_str(CORPUS).map_err(|e| e.to_string())?;
    let suite: Suite = serde_json::from_str(CASES).map_err(|e| e.to_string())?;
    validate(&corpus, &suite)?;
    Ok((corpus, suite))
}
pub fn validate(corpus: &Corpus, suite: &Suite) -> Result<(), String> {
    if corpus.version != 1 || suite.version != 1 || suite.k != 5 || suite.cases.is_empty() {
        return Err("Unsupported or empty evaluation dataset".into());
    }
    let mut keys = BTreeSet::new();
    for seed in &corpus.records {
        if seed.key.is_empty()
            || seed.scenario.is_empty()
            || seed.revisions.is_empty()
            || !keys.insert(seed.key.as_str())
        {
            return Err("Invalid or duplicate corpus key".into());
        }
    }
    for key in GRAPH_KEYS {
        if !keys.insert(key) {
            return Err("Graph/corpus key collision".into());
        }
    }
    let mut ids = BTreeSet::new();
    let mut scenarios = BTreeMap::new();
    for case in &suite.cases {
        let req = case.input();
        if case.id.is_empty()
            || !ids.insert(&case.id)
            || !["dev", "holdout"].contains(&case.split.as_str())
            || case.scenario.is_empty()
            || case.category.is_empty()
            || case.language.is_empty()
            || !["quality", "contract", "safety"].contains(&case.gate.as_str())
        {
            return Err("Invalid case identity, split or gate".into());
        }
        if scenarios
            .insert(&case.scenario, &case.split)
            .is_some_and(|prior| prior != &case.split)
        {
            return Err("Scenario split leakage".into());
        }
        if !["search", "get", "denied_write", "graph_query", "graph_get"]
            .contains(&req.operation.as_str())
            || !["owner", "shared", "reader", "revoked"].contains(&req.principal.as_str())
            || (req.operation == "search" && req.query.as_ref().is_none_or(|q| q.trim().is_empty()))
            || (req.operation.ends_with("get") && req.target.is_none())
        {
            return Err("Invalid adapter request".into());
        }
        for key in case
            .relevant
            .keys()
            .chain(case.forbidden.iter())
            .chain(case.allowed_results.iter())
            .chain(case.expected_revisions.keys())
            .chain(req.target.iter())
        {
            if !keys.contains(key.as_str()) {
                return Err(format!("Unknown fixture key: {key}"));
            }
        }
        if case.relevant.values().any(|g| !(1..=3).contains(g))
            || case
                .forbidden
                .iter()
                .any(|id| case.relevant.contains_key(id))
            || case
                .expected_revisions
                .iter()
                .any(|(id, r)| *r == 0 || !case.relevant.contains_key(id))
            || (case.expect_empty && !case.relevant.is_empty())
            || (case.expected_error.is_some() && (!case.relevant.is_empty() || case.expect_empty))
            || (case.relevant.is_empty()
                && !case.expect_empty
                && case.expected_error.is_none()
                && case.allowed_results.is_empty())
            || (!case.allowed_results.is_empty()
                && (!case.graph()
                    || case.maximum_hits.is_none()
                    || case.expected_graph_truncated != Some(true)
                    || !case.relevant.is_empty()))
        {
            return Err("Contradictory or absent grading oracle".into());
        }
    }
    Ok(())
}
