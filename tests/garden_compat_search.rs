use mother::{
    access::ClientAccess,
    contracts::Auth,
    embedding::EmbeddingProfile,
    error::Result,
    garden::{
        store::GardenStore,
        v1_search::{EncodedPassage, EncodedPassages, Encoder, NativeSearch},
    },
};
use serde_json::{Value, json};
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

#[test]
fn corrupt_unselected_derived_cache_row_falls_back_without_affecting_canonical_reads() {
    let (dir, store, auth, req) = fixture();
    let saved = store.v1_call(&auth, "mother_write_record", req).unwrap();
    let search = NativeSearch::new(dir.path().join("missing-local-model"));
    let encoder = Synthetic::default();
    search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &encoder,
        )
        .unwrap();
    drop(store);
    let path = dir.path().join("native.redb");
    let db = redb::Database::open(&path).unwrap();
    let tx = db.begin_write().unwrap();
    {
        let mut table = tx
            .open_table(redb::TableDefinition::<(&str, &str), &[u8]>::new(
                "garden_v1_vectors_v1",
            ))
            .unwrap();
        table
            .insert(("unselected-scope", "unselected-record"), b"{}".as_slice())
            .unwrap();
    }
    tx.commit().unwrap();
    drop(db);
    let store = GardenStore::open(path).unwrap();
    let result = search
        .search_with(
            &store,
            &auth,
            json!({"query":"automobile","mode":"semantic"}),
            &encoder,
        )
        .unwrap();
    assert_eq!(result["search"]["effectiveMode"], "literal");
    assert_eq!(result["search"]["reason"], "cache_unavailable");
    assert_eq!(result["records"][0]["id"], saved["record"]["id"]);
    assert_eq!(
        store
            .v1_call(
                &auth,
                "mother_get_record",
                json!({"id":saved["record"]["id"]})
            )
            .unwrap(),
        saved["record"]
    );
}

fn fixture() -> (tempfile::TempDir, GardenStore, Auth, Value) {
    let dir = tempfile::tempdir().unwrap();
    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let store =
        GardenStore::initialize(dir.path().join("native.redb"), &workspace, &generation).unwrap();
    store.v1_initialize("Synthetic semantic persona").unwrap();
    let auth = store
        .v1_grant(
            "searcher",
            &["read".into(), "memory:write".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    let req = json!({"workspaceId":workspace,"generation":generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Vehicle","content":"An automobile carries passengers.","status":"active","basis":"source"}});
    (dir, store, auth, req)
}
#[derive(Default)]
struct Synthetic {
    queries: AtomicUsize,
    passages: AtomicUsize,
}
impl Encoder for Synthetic {
    fn profile(&self) -> EmbeddingProfile {
        EmbeddingProfile {
            version: 1,
            model: "test:deterministic-v1".into(),
            profile: "test:passages-v1".into(),
            dimensions: 2,
        }
    }
    fn query(&self, _: &str) -> Result<Vec<f32>> {
        self.queries.fetch_add(1, Ordering::SeqCst);
        Ok(vec![1., 0.])
    }
    fn passages(&self, text: &str) -> Result<EncodedPassages> {
        self.passages.fetch_add(1, Ordering::SeqCst);
        Ok(EncodedPassages {
            partial: false,
            passages: vec![EncodedPassage {
                start: 0,
                end: text.len(),
                values: vec![1., 0.],
            }],
        })
    }
}
#[test]
fn learned_native_candidates_cache_and_literal_mode_use_current_canonical_text() {
    let (dir, store, auth, req) = fixture();
    let saved = store.v1_call(&auth, "mother_write_record", req).unwrap();
    let search = NativeSearch::new(dir.path().join("missing-local-model"));
    let encoder = Synthetic::default();
    let literal = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"literal"}),
            &encoder,
        )
        .unwrap();
    assert_eq!(literal["records"], json!([]));
    assert_eq!(encoder.queries.load(Ordering::SeqCst), 0);
    assert_eq!(encoder.passages.load(Ordering::SeqCst), 0);
    let semantic = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &encoder,
        )
        .unwrap();
    assert_eq!(semantic["records"][0]["id"], saved["record"]["id"]);
    assert_eq!(semantic["search"]["semanticUsed"], true);
    assert_eq!(semantic["search"]["indexedRecords"], 1);
    assert_eq!(semantic["search"]["newlyIndexedRecords"], 1);
    assert_eq!(semantic["search"]["model"], "test:deterministic-v1");
    assert!(
        semantic["records"][0]["match"]["text"]
            .as_str()
            .unwrap()
            .contains("automobile")
    );
    let cached = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"hybrid"}),
            &encoder,
        )
        .unwrap();
    assert_eq!(cached["search"]["newlyIndexedRecords"], 0);
    assert_eq!(encoder.passages.load(Ordering::SeqCst), 1);
    let missing = search
        .search(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
        )
        .unwrap();
    assert_eq!(missing["search"]["reason"], "model_unavailable");
    assert_eq!(missing["search"]["semanticUsed"], false);
}

struct Revoking<'a> {
    store: &'a GardenStore,
}
impl Encoder for Revoking<'_> {
    fn profile(&self) -> EmbeddingProfile {
        Synthetic::default().profile()
    }
    fn query(&self, text: &str) -> Result<Vec<f32>> {
        self.store.revoke("searcher").unwrap();
        Synthetic::default().query(text)
    }
    fn passages(&self, text: &str) -> Result<EncodedPassages> {
        Synthetic::default().passages(text)
    }
}
#[test]
fn native_semantic_response_rechecks_revocation_after_embedding() {
    let (dir, store, auth, req) = fixture();
    store.v1_call(&auth, "mother_write_record", req).unwrap();
    let search = NativeSearch::new(dir.path().join("missing-local-model"));
    let error = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &Revoking { store: &store },
        )
        .unwrap_err();
    assert_eq!(error.code(), "UNAUTHORIZED");
}

struct Retracting<'a> {
    store: &'a GardenStore,
    auth: &'a Auth,
    request: Value,
}
impl Encoder for Retracting<'_> {
    fn profile(&self) -> EmbeddingProfile {
        Synthetic::default().profile()
    }
    fn query(&self, text: &str) -> Result<Vec<f32>> {
        Synthetic::default().query(text)
    }
    fn passages(&self, text: &str) -> Result<EncodedPassages> {
        self.store
            .v1_call(self.auth, "mother_write_record", self.request.clone())
            .unwrap();
        Synthetic::default().passages(text)
    }
}
#[test]
fn native_semantic_response_does_not_resurrect_version_changed_during_embedding() {
    let (dir, store, auth, req) = fixture();
    let saved = store
        .v1_call(&auth, "mother_write_record", req.clone())
        .unwrap();
    let mut retract = req;
    retract["operationId"] = json!(Uuid::new_v4().to_string());
    retract["id"] = saved["record"]["id"].clone();
    retract["expectedRevision"] = json!(1);
    retract["record"]["status"] = json!("retracted");
    let search = NativeSearch::new(dir.path().join("missing-local-model"));
    let response = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &Retracting {
                store: &store,
                auth: &auth,
                request: retract,
            },
        )
        .unwrap();
    assert_eq!(response["records"], json!([]));
    assert_eq!(response["search"]["reason"], "workspace_changed");
    let response = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &Synthetic::default(),
        )
        .unwrap();
    assert_eq!(response["records"], json!([]));
    assert_eq!(response["search"]["eligibleRecords"], 0);
}

struct Reentrant<'a> {
    search: &'a NativeSearch,
    store: &'a GardenStore,
    auth: &'a Auth,
    observed: std::sync::Mutex<Option<Value>>,
}
impl Encoder for Reentrant<'_> {
    fn profile(&self) -> EmbeddingProfile {
        Synthetic::default().profile()
    }
    fn query(&self, text: &str) -> Result<Vec<f32>> {
        let response = self.search.search_with(
            self.store,
            self.auth,
            json!({"query":"transport","mode":"semantic"}),
            &Synthetic::default(),
        )?;
        *self.observed.lock().unwrap() = Some(response);
        Synthetic::default().query(text)
    }
    fn passages(&self, text: &str) -> Result<EncodedPassages> {
        Synthetic::default().passages(text)
    }
}
#[test]
fn concurrent_native_model_use_reports_truthful_busy_fallback() {
    let (dir, store, auth, req) = fixture();
    store.v1_call(&auth, "mother_write_record", req).unwrap();
    let search = NativeSearch::new(dir.path().join("missing-local-model"));
    let encoder = Reentrant {
        search: &search,
        store: &store,
        auth: &auth,
        observed: std::sync::Mutex::new(None),
    };
    let outer = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &encoder,
        )
        .unwrap();
    assert_eq!(outer["search"]["semanticUsed"], true);
    let inner = encoder.observed.lock().unwrap();
    assert_eq!(inner.as_ref().unwrap()["search"]["reason"], "model_busy");
    assert_eq!(inner.as_ref().unwrap()["search"]["semanticUsed"], false);
}

#[test]
fn native_cached_vectors_cannot_cross_visibility_shrink_or_profile_identity() {
    let (dir, store, auth, req) = fixture();
    let saved = store.v1_call(&auth, "mother_write_record", req).unwrap();
    let search = NativeSearch::new(dir.path().join("missing-local-model"));
    let encoder = Synthetic::default();
    let shared = store
        .v1_grant(
            "shared",
            &["read".into()],
            &ClientAccess {
                mode: mother::access::AccessMode::Shared,
                subject: Some("synthetic-shared".into()),
                records: vec![saved["record"]["id"].as_str().unwrap().into()],
            },
            None,
        )
        .unwrap();
    assert_eq!(
        search
            .search_with(
                &store,
                &shared,
                json!({"query":"transport","mode":"semantic"}),
                &encoder
            )
            .unwrap()["search"]["indexedRecords"],
        1
    );
    let narrowed = store
        .v1_grant(
            "shared",
            &["read".into()],
            &ClientAccess {
                mode: mother::access::AccessMode::Shared,
                subject: Some("synthetic-shared".into()),
                records: vec![],
            },
            None,
        )
        .unwrap();
    assert_eq!(
        search
            .search_with(
                &store,
                &shared,
                json!({"query":"transport","mode":"semantic"}),
                &encoder
            )
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
    let result = search
        .search_with(
            &store,
            &narrowed,
            json!({"query":"transport","mode":"semantic"}),
            &encoder,
        )
        .unwrap();
    assert_eq!(result["records"], json!([]));
    assert_eq!(result["search"]["eligibleRecords"], 0);
    assert!(result.get("persona").is_none());
    struct OtherProfile;
    impl Encoder for OtherProfile {
        fn profile(&self) -> EmbeddingProfile {
            let mut p = Synthetic::default().profile();
            p.profile = "test:independent-v2".into();
            p
        }
        fn query(&self, text: &str) -> Result<Vec<f32>> {
            Synthetic::default().query(text)
        }
        fn passages(&self, text: &str) -> Result<EncodedPassages> {
            Synthetic::default().passages(text)
        }
    }
    let result = search
        .search_with(
            &store,
            &auth,
            json!({"query":"transport","mode":"semantic"}),
            &OtherProfile,
        )
        .unwrap();
    assert_eq!(result["search"]["newlyIndexedRecords"], 1);
    assert_eq!(result["search"]["profile"], "test:independent-v2");
}
