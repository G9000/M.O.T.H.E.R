use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranking {
    pub recall: f64,
    pub precision: f64,
    pub reciprocal_rank: f64,
    pub ndcg: f64,
    pub all_evidence: bool,
}

pub fn rank(actual: &[String], relevant: &BTreeMap<String, u8>, k: usize) -> Option<Ranking> {
    if relevant.is_empty() || k == 0 {
        return None;
    }
    let mut seen = BTreeSet::new();
    let mut found = 0;
    let mut reciprocal_rank = 0.0;
    let mut dcg = 0.0;
    for (index, id) in actual.iter().take(k).enumerate() {
        if !seen.insert(id) {
            continue;
        }
        if let Some(grade) = relevant.get(id) {
            found += 1;
            if reciprocal_rank == 0.0 {
                reciprocal_rank = 1.0 / (index + 1) as f64;
            }
            dcg += gain(*grade) / ((index + 2) as f64).log2();
        }
    }
    let mut ideal = relevant.values().copied().collect::<Vec<_>>();
    ideal.sort_unstable_by(|a, b| b.cmp(a));
    let idcg: f64 = ideal
        .iter()
        .take(k)
        .enumerate()
        .map(|(i, g)| gain(*g) / ((i + 2) as f64).log2())
        .sum();
    Some(Ranking {
        recall: found as f64 / relevant.len() as f64,
        precision: found as f64 / k as f64,
        reciprocal_rank,
        ndcg: if idcg == 0.0 { 0.0 } else { dcg / idcg },
        all_evidence: found == relevant.len(),
    })
}

fn gain(grade: u8) -> f64 {
    2.0_f64.powi(i32::from(grade)) - 1.0
}
