use super::{
    Auth, ClientAccess, Result, Value, contracts, excerpt, graph, invalid, metadata, recallable,
    recent, settings,
};
use serde_json::json;
use std::collections::BTreeSet;

const STOP: &str = "a an and are at by can day did do find for from i in it me moment my of on please recall remember show that the this to was we when where with you";
fn references(r: &Value, id: &str) -> bool {
    r["references"]
        .as_array()
        .is_some_and(|a| a.iter().any(|v| v == id))
}
pub(super) fn find(
    tx: &redb::ReadTransaction,
    auth: &Auth,
    access: &ClientAccess,
    args: &Value,
) -> Result<Value> {
    contracts::strict(args, &["clue", "includeKeepsakes"])?;
    let clue = contracts::string(args, "clue")?;
    if clue.len() > 320 {
        return Err(invalid("Use a clue of up to 320 bytes."));
    }
    let include = args
        .get("includeKeepsakes")
        .map(|v| {
            v.as_bool()
                .ok_or_else(|| invalid("includeKeepsakes must be a boolean."))
        })
        .transpose()?
        .unwrap_or(false);
    let normalized = settings::words(clue);
    let terms: BTreeSet<_> = normalized
        .split_whitespace()
        .filter(|t| !STOP.split_whitespace().any(|s| s == *t))
        .collect();
    if terms.is_empty() {
        return Err(invalid("Include a distinctive word from the moment."));
    }
    let settings = settings::Settings::load(tx)?;
    let workspace = metadata(tx, "workspace")?;
    let records: Vec<_> = super::summary_review::records(tx, |id| {
        access.allows(id) && (include || !settings.hidden(id))
    })?
    .into_iter()
    .filter(recallable)
    .collect();
    let mut hits = Vec::new();
    for r in &records {
        let title = settings::words(r["title"].as_str().unwrap_or_default());
        let body = settings::words(r["content"].as_str().unwrap_or_default());
        let title: BTreeSet<_> = title.split_whitespace().collect();
        let body: BTreeSet<_> = body.split_whitespace().collect();
        let matched: Vec<_> = terms
            .iter()
            .copied()
            .filter(|t| title.contains(t) || body.contains(t))
            .collect();
        if !matched.is_empty() {
            hits.push((
                matched.len(),
                matched.iter().filter(|t| title.contains(**t)).count(),
                matched,
                r,
            ));
        }
    }
    hits.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then(b.1.cmp(&a.1))
            .then_with(|| recent(a.3, b.3))
    });
    let count = hits.len();
    let mut shown = Vec::new();
    for (_, _, terms, r) in hits.into_iter().take(3) {
        let id = contracts::string(r, "id")?;
        let mut links: Vec<_> = records
            .iter()
            .filter(|v| {
                v["id"] != id
                    && (references(v, id) || references(r, v["id"].as_str().unwrap_or_default()))
            })
            .collect();
        links.sort_by(|a, b| recent(a, b));
        shown.push(json!({"id":id,"revision":r["revision"],"title":excerpt(r["title"].as_str().unwrap_or_default(),120),"excerpt":excerpt(r["content"].as_str().unwrap_or_default(),180),
            "kind":r["kind"],"basis":r["basis"],"recordedAt":r["createdAt"],"eventDate":settings.date(id),"recallReason":{"matchedWords":terms},
            "linkedRecords":links.iter().take(4).map(|v|json!({"id":v["id"],"kind":v["kind"],"title":excerpt(v["title"].as_str().unwrap_or_default(),72)})).collect::<Vec<_>>(),"linkedRecordsOmitted":links.len().saturating_sub(4)}));
    }
    let media = super::super::media::moment_media(tx, auth, &mut shown).map_err(graph)?;
    Ok(
        json!({"workspaceId":workspace["id"],"generation":workspace["generation"],"records":shown,"otherMatches":count.saturating_sub(3),"mediaCoverage":media}),
    )
}
