use super::dataset::{Corpus, Request};
use chrono::{DateTime, Duration, Utc};
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, ExternalReference, GetRequest, QueryRequest},
        store::{Clock, GardenStore},
        v1_search::NativeSearch,
    },
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub type EvalResult<T> = Result<T, Box<dyn std::error::Error>>;
const WORKSPACE: &str = "11111111-1111-4111-8111-111111111111";
const GENERATION: &str = "22222222-2222-4222-8222-222222222222";

#[derive(Debug)]
struct FixedClock(Mutex<DateTime<Utc>>);
impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().expect("fixture clock")
    }
}
impl FixedClock {
    fn set(&self, value: &str) {
        *self.0.lock().expect("fixture clock") = value.parse().expect("authored timestamp");
    }
    fn tick(&self) {
        *self.0.lock().expect("fixture clock") += Duration::seconds(1);
    }
}

pub struct Fixture {
    // Keep the owned directory alive until after the database handle is dropped.
    pub store: GardenStore,
    pub search: NativeSearch,
    pub ids: BTreeMap<String, String>,
    pub graph: BTreeMap<String, ExternalReference>,
    pub canonical: BTreeMap<String, Value>,
    pub historical: BTreeMap<(String, u64), Value>,
    principals: BTreeMap<String, Auth>,
    _dir: tempfile::TempDir,
}
impl Fixture {
    pub fn new(corpus: &Corpus) -> EvalResult<Self> {
        let dir = tempfile::tempdir()?;
        let clock = Arc::new(FixedClock(Mutex::new("2026-01-01T00:00:00Z".parse()?)));
        let store = GardenStore::initialize_with_clock(
            dir.path().join("synthetic.redb"),
            WORKSPACE,
            GENERATION,
            clock.clone(),
        )?;
        store.v1_initialize("Synthetic evaluation persona. No owner data.")?;
        let companion = ClientAccess {
            mode: AccessMode::Companion,
            subject: Some("eval-owner".into()),
            records: vec![],
        };
        let owner = store.v1_grant(
            "eval-owner",
            &[
                "read".into(),
                "memory:write".into(),
                "checkpoint:write".into(),
                "garden:read".into(),
                "garden:write".into(),
            ],
            &companion,
            Some(&GardenAccess::All {}),
        )?;
        let mut ids: BTreeMap<String, String> = BTreeMap::new();
        let mut canonical = BTreeMap::new();
        let mut historical = BTreeMap::new();
        for seed in &corpus.records {
            for (index, revision) in seed.revisions.iter().enumerate() {
                clock.tick();
                let mut record = revision.clone();
                if record.get("kind").is_none() {
                    record["kind"] = json!("memory");
                }
                if record["kind"] == "memory" {
                    record["basis"] = json!("source");
                }
                if record["kind"] == "checkpoint" {
                    record["nextStep"] = json!("Synthetic task only.");
                }
                record["sources"] = json!([{"label":"Synthetic authored fixture", "uri":format!("https://example.invalid/mother-eval/{}", seed.key)}]);
                let mut request = json!({"workspaceId":WORKSPACE,"generation":GENERATION,"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":index,"record":record});
                if index > 0 {
                    request["id"] = json!(ids[&seed.key]);
                }
                let saved =
                    store.v1_call(&owner, "mother_write_record", request)?["record"].clone();
                ids.insert(
                    seed.key.clone(),
                    saved["id"].as_str().ok_or("Missing created ID")?.into(),
                );
                historical.insert((seed.key.clone(), (index + 1) as u64), saved.clone());
                canonical.insert(seed.key.clone(), saved);
            }
        }
        let shared_ids = corpus
            .records
            .iter()
            .filter(|s| s.shared)
            .map(|s| ids[&s.key].clone())
            .collect();
        let shared = store.v1_grant(
            "eval-shared",
            &["read".into(), "garden:read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("eval-shared".into()),
                records: shared_ids,
            },
            Some(&GardenAccess::default()),
        )?;
        let reader = store.v1_grant("eval-reader", &["read".into()], &companion, None)?;
        let proposer = store.v1_grant(
            "eval-proposer",
            &["read".into(), "context:propose".into()],
            &companion,
            None,
        )?;
        clock.tick();
        store.v1_call(&proposer,"mother_propose_memory",json!({"workspaceId":WORKSPACE,"generation":GENERATION,"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"source":"Synthetic selected detail awaiting explicit owner approval","basis":"owner_statement","record":{"kind":"memory","title":"Unapproved synthetic detail","content":"amber-proposal is a proposed preference, not an established memory.","basis":"owner_statement","status":"active"}}))?;
        let revoked = store.v1_grant("eval-revoked", &["read".into()], &companion, None)?;
        store.revoke("eval-revoked")?;
        clock.set("2026-01-01T01:00:00Z");
        let mut graph = BTreeMap::new();
        let mut graph_write = |items: Value| -> EvalResult<()> {
            let request: BatchRequest = serde_json::from_value(
                json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items}),
            )?;
            let result = store.apply(&owner, &request)?;
            for item in &result.receipt.results {
                ids.insert(item.item_key.clone(), item.reference.id.clone());
                graph.insert(item.item_key.clone(), item.reference.clone());
            }
            Ok(())
        };
        graph_write(
            json!([{"itemKey":"person","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"eval:person","displayName":"Alia before correction","aliases":[],"extensions":{}}}]),
        )?;
        clock.set("2026-01-01T01:00:01Z");
        let person_id = &ids["person"];
        let subject = json!({"refType":"existing","kind":"entity","id":person_id});
        let assertion = |city: &str, from: &str, until: Option<&str>| json!({"kind":"assertion","subject":subject,"predicate":"eval:lives_in","object":{"type":"value","value":{"type":"string","value":city}},"validTime":{"validFrom":from,"validUntil":until,"sourceTimezone":null,"precision":"instant"},"observedTime":{"at":from,"sourceTimezone":null,"precision":"instant"},"evidence":[{"refType":"item","kind":"evidence","itemKey":"evidence"}],"confidence":null,"extensions":{}});
        let request: BatchRequest = serde_json::from_value(
            json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
                {"itemKey":"evidence","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence","source":{"type":"uri","uri":"urn:mother:eval:graph-source"},"excerpt":"Synthetic city timeline, not a real person's history.","observedTime":{"at":"2024-01-01T00:00:00Z","sourceTimezone":null,"precision":"instant"},"extensions":{}}},
                {"itemKey":"old_city","mutation":"create","expectedRevision":0,"draft":assertion("Cedar Bay","2024-01-01T00:00:00Z",Some("2025-01-01T00:00:00Z"))},
                {"itemKey":"new_city","mutation":"create","expectedRevision":0,"draft":assertion("Willow Port","2025-01-01T00:00:00Z",None)}
            ]}),
        )?;
        for item in store.apply(&owner, &request)?.receipt.results {
            ids.insert(item.item_key.clone(), item.reference.id.clone());
            graph.insert(item.item_key, item.reference);
        }
        for (key, reference) in &graph {
            let req: GetRequest = serde_json::from_value(
                json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"requiredExtensions":{},"reference":reference,"revision":null,"knownAt":null}),
            )?;
            let record = serde_json::to_value(store.get(&owner, &req)?.record)?;
            historical.insert((key.clone(), 1), record.clone());
            canonical.insert(key.clone(), record);
        }
        clock.set("2026-01-01T01:00:02Z");
        let update: BatchRequest = serde_json::from_value(
            json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"person","mutation":"update","id":graph["person"],"expectedRevision":1,"draft":{"kind":"entity","entityType":"eval:person","displayName":"Alia corrected","aliases":[],"extensions":{}}}]}),
        )?;
        store.apply(&owner, &update)?;
        let get: GetRequest = serde_json::from_value(
            json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"requiredExtensions":{},"reference":graph["person"],"revision":null,"knownAt":null}),
        )?;
        let record = serde_json::to_value(store.get(&owner, &get)?.record)?;
        historical.insert(("person".into(), 2), record.clone());
        canonical.insert("person".into(), record);
        Ok(Self {
            store,
            search: NativeSearch::new(dir.path().join("deliberately-absent-model")),
            ids,
            graph,
            canonical,
            historical,
            principals: BTreeMap::from([
                ("owner".into(), owner),
                ("shared".into(), shared),
                ("reader".into(), reader),
                ("revoked".into(), revoked),
            ]),
            _dir: dir,
        })
    }
    pub fn alias(&self, id: &str) -> Option<String> {
        self.ids
            .iter()
            .find(|(_, v)| v.as_str() == id)
            .map(|(k, _)| k.clone())
    }
    pub fn auth(&self, principal: &str) -> &Auth {
        &self.principals[principal]
    }
    pub fn graph_query(&self, req: &Request) -> EvalResult<Value> {
        let mut args = json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"requiredExtensions":{},"ids":[],"kinds":[],"limit":5,"cursor":null});
        if let Some(fields) = req.args.as_object() {
            for (k, v) in fields {
                args[k] = v.clone();
            }
        }
        if let Some(key) = req.target.as_deref() {
            args["traversal"]["start"] = json!([self.graph[key]]);
        }
        let query: QueryRequest = serde_json::from_value(args)?;
        Ok(serde_json::to_value(
            self.store.query(self.auth(&req.principal), &query)?,
        )?)
    }
    pub fn graph_get(&self, req: &Request) -> EvalResult<Value> {
        let key = req.target.as_deref().ok_or("Missing graph target")?;
        let mut args = json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,"requiredExtensions":{},"reference":self.graph[key],"revision":null,"knownAt":null});
        if let Some(fields) = req.args.as_object() {
            for (k, v) in fields {
                args[k] = v.clone();
            }
        }
        let get: GetRequest = serde_json::from_value(args)?;
        Ok(serde_json::to_value(
            self.store.get(self.auth(&req.principal), &get)?,
        )?)
    }
    pub fn denied_write(&self, req: &Request) -> EvalResult<Value> {
        // Compare full canonical graph/v1 backup before and after, even on an error.
        let snapshot = |label: &str| -> EvalResult<BTreeMap<String, Vec<u8>>> {
            let out = self
                ._dir
                .path()
                .join(format!("{label}-{}", uuid::Uuid::new_v4()));
            self.store.backup_to(&out)?;
            let mut files = BTreeMap::new();
            for entry in std::fs::read_dir(out)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    files.insert(
                        entry.file_name().to_string_lossy().into_owned(),
                        std::fs::read(entry.path())?,
                    );
                }
            }
            Ok(files)
        };
        let before = snapshot("before-denied-write")?;
        let operation = uuid::Uuid::new_v4().to_string();
        let result=self.store.v1_call(self.auth(&req.principal),"mother_write_record",json!({"workspaceId":WORKSPACE,"generation":GENERATION,"operationId":operation,"expectedRevision":0,"record":{"kind":"memory","title":"Unapproved synthetic publication","content":"This must never become canonical.","basis":"source","status":"active"}}));
        let after = snapshot("after-denied-write")?;
        if before != after {
            return Err("SAFETY: denied write changed canonical backup".into());
        }
        let receipt = self.store.v1_call(
            self.auth(&req.principal),
            "mother_get_receipt",
            json!({"operationId":operation}),
        );
        if !receipt.as_ref().is_err_and(|e| e.code() == "NOT_FOUND") {
            return Err("SAFETY: denied write has a receipt".into());
        }
        Ok(result?)
    }
}
