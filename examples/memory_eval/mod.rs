pub mod dataset;
mod fixture;
pub mod metrics;
mod runner;

use fixture::{EvalResult, Fixture};
use runner::ResultRow;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn executable_hash() -> EvalResult<String> {
    let mut file = std::fs::File::open(std::env::current_exe()?)?;
    let mut sha = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha.update(&buffer[..count]);
    }
    Ok(format!("{:x}", sha.finalize()))
}

pub fn run() -> EvalResult<Value> {
    let (corpus, suite) = dataset::load().map_err(std::io::Error::other)?;
    let mut results = Vec::new();
    for mode in ["literal", "hybrid", "semantic"] {
        let fixture = Fixture::new(&corpus)?;
        for pass in ["first_pass", "repeat_pass"] {
            for case in suite.cases.iter().filter(|c| !c.graph()) {
                results.push(runner::run_case(&fixture, case, mode, pass, suite.k));
            }
        }
    }
    let graph = Fixture::new(&corpus)?;
    for case in suite.cases.iter().filter(|c| c.graph()) {
        results.push(runner::run_case(
            &graph,
            case,
            "structured",
            "first_pass",
            suite.k,
        ));
    }
    let hard_pass = results.iter().all(|r| !r.hard_failure);
    let quality_pass = results.iter().all(|r| r.status == "passed");
    let error_count = results.iter().filter(|r| r.status == "error").count();
    let summaries = summarize(&results);
    let inputs = [
        ("corpus", dataset::CORPUS),
        ("cases", dataset::CASES),
        ("evaluator", include_str!("mod.rs")),
        ("metrics", include_str!("metrics.rs")),
        ("dataset_validator", include_str!("dataset.rs")),
        ("fixture", include_str!("fixture.rs")),
        ("runner", include_str!("runner.rs")),
        ("search", include_str!("../../src/search.rs")),
        (
            "native_search",
            include_str!("../../src/garden/compat_search.rs"),
        ),
        ("native_v1", include_str!("../../src/garden/compat.rs")),
        ("graph_query", include_str!("../../src/garden/query.rs")),
        ("graph_store", include_str!("../../src/garden/store.rs")),
        ("graph_history", include_str!("../../src/garden/history.rs")),
        ("contracts", include_str!("../../src/contracts.rs")),
        (
            "graph_contracts",
            include_str!("../../src/garden/contracts.rs"),
        ),
        ("embedding", include_str!("../../src/embedding.rs")),
        ("lockfile", include_str!("../../Cargo.lock")),
    ];
    let hashes: BTreeMap<_, _> = inputs
        .into_iter()
        .map(|(name, source)| (name, hash(source.as_bytes())))
        .collect();
    Ok(json!({
        "reportVersion":1,"suite":suite.id,"corpus":corpus.id,"suiteVersion":suite.version,
        "scope":"Original synthetic Mother API baseline; not LongMemEval/LoCoMo/BEIR benchmark scores, live user memory or agent-answer acceptance",
        "provenance":{"packageVersion":env!("CARGO_PKG_VERSION"),"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"debugAssertions":cfg!(debug_assertions),"executableSha256":executable_hash()?,"sourceManifestScope":"Selected compiled modules and lockfile only, not an exhaustive source/build manifest. Executable SHA identifies the actual evaluated binary.","compiledInputSha256":hashes},
        "configuration":{"k":suite.k,"requestedModes":["literal","hybrid","semantic"],"passes":["first_pass","repeat_pass"],"cacheLifecycle":"Fresh synthetic store per requested mode, authored query order in both passes; no hidden warmup/retry","modelAssets":"deliberately absent; measures literal and honest no-model fallback","timingScope":"Search/graph domain calls exclude fixture setup and full-read verification. denied_write includes before/after backup snapshots and receipt verification. Expected denial latency is reported separately by category; failures cannot satisfy a latency gate.","fixedClock":"Distinct second per canonical v1 write; graph 01:00:00, 01:00:01, correction 01:00:02 UTC on 2026-01-01"},
        "gates":{"hardGatesPass":hard_pass,"qualityPass":quality_pass,"unexpectedErrors":error_count,"failedCases":results.iter().filter(|r|r.status!="passed").count()},
        "lanes":{"v1Retrieval":{"status":"completed","transport":"Direct native domain API used by MCP; not a host/IPC test"},"learnedRetrieval":{"status":"not_run","reason":"Deterministic learned-index scheduling and real-model qualification are a separate slice; no local assets loaded"},"temporalGraph":{"status":"completed","scope":"Structured contract recall only; no natural-language planner"},"animaAnswerAndToolUse":{"status":"not_run","reason":"Separate model/harness evaluation required; no model calls or host changes authorized in this slice"},"captureQuality":{"status":"not_run","reason":"Harness intelligence selects memories; this fixture preloads only selected synthetic records"},"physicalRestart":{"status":"not_run"}},
        "metricDefinitions":{"ranking":"Macro average over successfully scored answerable cases only; attempted/scored/errors disclosed. No cross-mode or cross-pass average.","precision":"Unique relevant hits divided by fixed k=5, including short result lists","recall":"Unique relevant hits at k divided by all positive qrels","mrr":"Reciprocal first relevant rank at k; duplicates consume slots","ndcg":"Gain 2^grade-1; discount log2(rank+1); positive grades 1..3","allEvidence":"All positive qrels must be present; each is required, not an alternative","emptyQrels":"Ranking null; separate retrieval-empty expectation, not model abstention","holdout":"Public regression holdout by scenario, not a secret test or statistical generalization claim"},
        "summaries":summaries,"results":results
    }))
}

fn summarize(results: &[ResultRow]) -> Vec<Value> {
    let mut groups: BTreeMap<(String, String, String, String, String), Vec<&ResultRow>> =
        BTreeMap::new();
    for r in results {
        for (dimension, label) in [
            ("all", "all"),
            ("language", r.language.as_str()),
            ("category", r.category.as_str()),
            ("split", r.split.as_str()),
        ] {
            groups
                .entry((
                    r.lane.clone(),
                    r.mode.clone(),
                    r.pass.clone(),
                    dimension.into(),
                    label.into(),
                ))
                .or_default()
                .push(r);
        }
    }
    groups.into_iter().map(|((lane,mode,pass,dimension,label),rows)|{
        let ranks=rows.iter().filter_map(|r|r.ranking.as_ref()).collect::<Vec<_>>();
        let n=ranks.len() as f64;
        let avg=|f:fn(&metrics::Ranking)->f64| if n==0.0{None}else{Some(ranks.iter().map(|r|f(r)).sum::<f64>()/n)};
        let mut times=rows.iter().filter(|r|r.status!="error").map(|r|r.milliseconds).collect::<Vec<_>>(); times.sort_by(f64::total_cmp);
        let percentile=|p:usize| times.get((times.len()*p).div_ceil(100).saturating_sub(1)).copied();
        json!({"lane":lane,"mode":mode,"pass":pass,"dimension":dimension,"label":label,"attempted":rows.len(),"passed":rows.iter().filter(|r|r.status=="passed").count(),"errors":rows.iter().filter(|r|r.status=="error").count(),"rankingScored":ranks.len(),"recallAt5":avg(|r|r.recall),"precisionAt5":avg(|r|r.precision),"mrrAt5":avg(|r|r.reciprocal_rank),"ndcgAt5":avg(|r|r.ndcg),"allEvidenceRate":avg(|r|f64::from(r.all_evidence)),"p50Ms":percentile(50),"p95Ms":percentile(95),"responseBytesMax":rows.iter().map(|r|r.response_bytes).max()})
    }).collect()
}

pub fn write_new(path: &Path, report: &Value) -> EvalResult<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, report)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

pub fn exit_code(report: &Value, require_quality: bool) -> u8 {
    if report["gates"]["hardGatesPass"] != true {
        2
    } else if require_quality && report["gates"]["qualityPass"] != true {
        1
    } else {
        0
    }
}
