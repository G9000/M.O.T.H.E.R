//! Project briefs and private, per-credential reading positions. Only explicit acknowledgements
//! write to VISITS. This table is deliberately excluded from backup and canonical recall.
use super::settings::{self, Settings, project_match};
use super::{
    AccessMode, Auth, ClientAccess, GardenStore, View, contracts, excerpt, identity, integer,
    metadata, preview, principal, put, recallable, recent, record, string, unavailable, view,
    visible_records,
};
use crate::error::{Error, Result, invalid};
use redb::TableDefinition;
use serde_json::{Value, json};

const VISITS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_project_visits");
const PAGE: usize = 3;

fn reading_position(visits: &Value, names: &[&str]) -> Value {
    let mut seen = Value::Null;
    for name in names {
        if let Some(items) = visits[*name].as_object() {
            for (id, rev) in items {
                seen[id] = json!(
                    seen[id]
                        .as_u64()
                        .unwrap_or(0)
                        .max(rev.as_u64().unwrap_or(0))
                );
            }
        }
    }
    seen
}

fn key(auth: &Auth, workspace: &Value) -> String {
    format!(
        "{}/{}/{}",
        workspace["generation"],
        auth.client_id,
        contracts::hash(&auth.token)
    )
}

fn card(r: &Value) -> Value {
    let mut out = preview(r, 200);
    if let Some(step) = r["nextStep"].as_str() {
        out["nextStep"] = json!(excerpt(step, 200));
        out["nextStepTruncated"] = json!(step.encode_utf16().count() > 200);
    }
    out
}

pub(super) fn resume(
    tx: &redb::ReadTransaction,
    auth: &Auth,
    access: &ClientAccess,
    args: &Value,
) -> Result<Value> {
    contracts::strict(args, &["project"])?;
    let name = settings::name(string(args, "project")?)?;
    let settings = Settings::load(tx)?;
    let names = settings.names(&name);
    let project = names[0];
    let workspace = metadata(tx, "workspace")?;
    let visits = match tx.open_table(VISITS) {
        Ok(_) => tx.get(VISITS, &key(auth, &workspace))?.unwrap_or_default(),
        Err(redb::TableError::TableDoesNotExist(_)) => Value::Null,
        Err(e) => return Err(unavailable(e)),
    };
    let seen = reading_position(&visits, &names);
    let mut records = visible_records(tx, access)?;
    records.retain(|r| {
        r["sourceReview"]["state"] != "needs_review"
            && (project_match(r, &names).is_some()
                || seen.get(r["id"].as_str().unwrap_or("")).is_some())
    });
    records.sort_by(recent);
    let current: Vec<_> = records
        .iter()
        .filter(|r| recallable(r) && project_match(r, &names).is_some())
        .collect();
    let project_card = |r: &Value| {
        let mut c = card(r);
        c["recallReason"] = project_match(r, &names)
            .map_or_else(|| json!("previously seen"), |n| json!({"projectName":n}));
        c
    };
    let latest = |kind, status: &[&str]| {
        current
            .iter()
            .copied()
            .find(|r| r["kind"] == kind && status.contains(&r["status"].as_str().unwrap_or("")))
            .map(project_card)
    };
    let changed: Vec<_> = records
        .iter()
        .filter(|r| seen[r["id"].as_str().unwrap_or("")] != r["revision"])
        .collect();
    let history_ids = changed
        .iter()
        .take(PAGE)
        .filter(|r| access.mode != AccessMode::Shared && r["revision"].as_u64().unwrap_or(1) > 1)
        .filter_map(|r| r["id"].as_str())
        .collect::<std::collections::HashSet<_>>();
    let history = if history_ids.is_empty() {
        Default::default()
    } else {
        view::revisions(tx, string(&workspace, "generation")?, history_ids)?
    };
    let changes = changed.iter().take(PAGE).map(|r| {
        let id = r["id"].as_str().unwrap_or("");
        let last = seen[id].as_u64();
        let rev = r["revision"].as_u64().unwrap_or(1);
        let previous = last.unwrap_or(rev.saturating_sub(1));
        let old = history.get(id).and_then(|h| h.get(&previous)).map(|(_, old)| old);
        let wording = old.and_then(|old| {
            let a = old["content"].as_str()?;
            let b = r["content"].as_str()?;
            if a == b {return None;}
            let prefix = a.chars().zip(b.chars()).take_while(|(l,r)| l == r).count();
            let start = prefix.saturating_sub(32);
            let window = |text: &str| text.chars().skip(start).take(160).collect::<String>();
            Some(json!({"before":window(a),"after":window(b),"startCharacter":start}))
        });
        json!({"current":project_card(r),"before":old.map(card),"wording":wording,"lastSeenRevision":last,
            "matchesProjectNow":project_match(r,&names).is_some(),"seen":{"id":id,"revision":rev}})
    }).collect::<Vec<_>>();
    Ok(
        json!({"workspaceId":workspace["id"],"generation":workspace["generation"],"project":project,
        "matchingRecords":current.len(),"confirmedDecision":latest("decision",&["confirmed"]),
        "checkpoint":latest("checkpoint",&["active"]),"openTask":latest("task",&["blocked","doing","todo"]),
        "changes":changes,"remaining":changed.len().saturating_sub(PAGE),"hasPreviousVisit":seen.is_object(),
        "historyAvailable":access.mode != AccessMode::Shared}),
    )
}

pub(super) fn mark(store: &GardenStore, auth: &Auth, args: &Value) -> Result<Value> {
    contracts::strict(args, &["workspaceId", "generation", "project", "seen"])?;
    let name = settings::name(string(args, "project")?)?;
    let items = args["seen"]
        .as_array()
        .filter(|a| (1..=6).contains(&a.len()))
        .ok_or_else(|| invalid("Acknowledge one to six returned record revisions."))?;
    let tx = store.db.begin_write().map_err(unavailable)?;
    let access = principal(&tx, auth, "read")?.access;
    let workspace = identity(&tx, args)?;
    let settings = Settings::load(&tx)?;
    let names = settings.names(&name);
    let project = names[0];
    let key = key(auth, &workspace);
    let mut visits = tx.get(VISITS, &key)?.unwrap_or_else(|| json!({}));
    let seen = reading_position(&visits, &names);
    for item in items {
        contracts::strict(item, &["id", "revision"])?;
        let id = string(item, "id")?;
        contracts::id(id)?;
        access.require(id)?;
        let r = record(&tx, id)?;
        let revision = integer(item, "revision", 1, 9_007_199_254_740_991)?;
        let prior = seen[id]
            .as_u64()
            .unwrap_or(0)
            .max(visits[project][id].as_u64().unwrap_or(0));
        if settings.hidden(id)
            || revision > r["revision"].as_u64().unwrap_or(0)
            || (prior == 0 && project_match(&r, &names).is_none())
        {
            return Err(invalid(
                "This revision is unavailable for this project. Refresh the brief.",
            ));
        }
        visits[project][id] = json!(prior.max(revision));
    }
    if serde_json::to_vec(&visits)?.len() > 1024 * 1024 {
        return Err(Error::domain(
            "RESOURCE_LIMIT",
            "Project reading positions exceed this credential's limit.",
        ));
    }
    put(&tx, VISITS, &key, &visits)?;
    tx.commit().map_err(unavailable)?;
    Ok(json!({"project":project,"acknowledged":items.len()}))
}
