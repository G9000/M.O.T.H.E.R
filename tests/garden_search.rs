use mother::{
    access::ClientAccess,
    embedding::EmbeddingProfile,
    garden::{
        access::GardenAccess,
        contracts::*,
        search::{EmbeddingVectors, Vectorizer},
        store::GardenStore,
    },
};
use serde_json::json;

struct Synthetic;
impl Vectorizer for Synthetic {
    fn profile(&self) -> EmbeddingProfile {
        EmbeddingProfile {
            version: 1,
            model: "synthetic@immutable-v1".into(),
            profile: "synthetic:unit-v1".into(),
            dimensions: 2,
        }
    }
    fn vectors(&self, text: &str) -> Result<EmbeddingVectors, GardenError> {
        Ok(EmbeddingVectors {
            vectors: vec![if text.contains("tea") {
                vec![1., 0.]
            } else {
                vec![0., 1.]
            }],
            partial: false,
        })
    }
}
fn batch(workspace: &str, generation: &str, name: &str) -> BatchRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"person","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}}]})).unwrap()
}
fn query(workspace: &str, generation: &str) -> QueryRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":workspace,"generation":generation,"requiredExtensions":{},"ids":[],"kinds":[],"entityType":null,"predicate":null,"property":null,"validAt":null,"knownAt":null,"text":null,"vector":{"model":"synthetic@immutable-v1","profile":"synthetic:unit-v1","dimensions":2,"values":[1.,0.]},"traversal":null,"limit":100,"cursor":null})).unwrap()
}

#[test]
fn explicit_vectors_are_profile_bound_and_stale_revisions_cannot_resurrect() {
    let dir = tempfile::tempdir().unwrap();
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(dir.path().join("garden.redb"), &ws, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let tea = store
        .apply(&auth, &batch(&ws, &generation, "Tea preference"))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let coffee = store
        .apply(&auth, &batch(&ws, &generation, "Coffee preference"))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let prepared = store
        .prepare_search_with(&auth, &[tea.clone(), coffee], &Synthetic)
        .unwrap();
    assert_eq!(prepared.prepared, 2);
    let mut q = query(&ws, &generation);
    let hits = store.query(&auth, &q).unwrap();
    assert_eq!(hits.hits.len(), 1);
    assert_eq!(hits.hits[0].record.metadata.id, tea.id);
    assert!(hits.hits[0].scores.vector.unwrap() > 0.99);
    assert_eq!(hits.coverage.index_watermark, 0);
    assert!(!hits.coverage.complete);
    q.vector.as_mut().unwrap().model = "synthetic@different-v2".into();
    let fallback = store.query(&auth, &q).unwrap();
    assert!(fallback.hits.is_empty());
    assert_eq!(fallback.coverage.search_mode, SearchMode::LiteralFallback);
    q.vector.as_mut().unwrap().model = "synthetic@immutable-v1".into();
    let retract:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"retired","mutation":"retract","id":tea,"expectedRevision":1}]})).unwrap();
    store.apply(&auth, &retract).unwrap();
    assert!(store.query(&auth, &q).unwrap().hits.is_empty());
}

#[test]
fn literal_fallback_is_explicit_and_prepare_rechecks_current_authority() {
    let dir = tempfile::tempdir().unwrap();
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(dir.path().join("garden.redb"), &ws, &generation).unwrap();
    let auth = store
        .grant(
            "writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let reference = store
        .apply(&auth, &batch(&ws, &generation, "Tea preference"))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let mut q = query(&ws, &generation);
    q.text = Some("TEA".into());
    let result = store.query(&auth, &q).unwrap();
    assert_eq!(result.hits.len(), 1);
    assert_eq!(result.coverage.search_mode, SearchMode::LiteralFallback);
    assert!(result.hits[0].scores.vector.is_none());
    struct Revoking<'a>(&'a GardenStore);
    impl Vectorizer for Revoking<'_> {
        fn profile(&self) -> EmbeddingProfile {
            Synthetic.profile()
        }
        fn vectors(&self, _: &str) -> Result<EmbeddingVectors, GardenError> {
            self.0.revoke("writer")?;
            Ok(EmbeddingVectors {
                vectors: vec![vec![1., 0.]],
                partial: false,
            })
        }
    }
    assert_eq!(
        store
            .prepare_search_with(&auth, &[reference], &Revoking(&store))
            .unwrap_err()
            .code,
        GardenErrorCode::Unauthorized
    );
}

#[test]
fn hidden_vector_preparation_does_not_change_restricted_coverage() {
    let dir = tempfile::tempdir().unwrap();
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(dir.path().join("garden.redb"), &ws, &generation).unwrap();
    let owner = store
        .grant(
            "owner",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let visible = store
        .apply(&owner, &batch(&ws, &generation, "Tea preference"))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let hidden = store
        .apply(&owner, &batch(&ws, &generation, "Private coffee"))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let reader = store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::Allowlist {
                objects: vec![visible],
            },
        )
        .unwrap();
    let mut q = query(&ws, &generation);
    q.text = Some("tea".into());
    let before = store.query(&reader, &q).unwrap();
    store
        .prepare_search_with(&owner, &[hidden], &Synthetic)
        .unwrap();
    let after = store.query(&reader, &q).unwrap();
    assert_eq!(before.coverage, after.coverage);
    assert_eq!(before.hits, after.hits);
    assert_eq!(after.coverage.search_mode, SearchMode::LiteralFallback);
}

#[test]
fn preparation_rejects_unauthorized_inputs_and_invalid_vectors_before_cache_write() {
    let dir = tempfile::tempdir().unwrap();
    let ws = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let store = GardenStore::initialize(dir.path().join("garden.redb"), &ws, &generation).unwrap();
    let owner = store
        .grant(
            "owner",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let reference = store
        .apply(&owner, &batch(&ws, &generation, "Tea"))
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    let reader = store
        .grant(
            "reader",
            &["garden:read".into()],
            &ClientAccess::default(),
            &GardenAccess::default(),
        )
        .unwrap();
    // Both fail before any model asset can be opened, even if this machine has a local model.
    assert_eq!(
        store.prepare_search(&owner, &[]).unwrap_err().code,
        GardenErrorCode::LimitExceeded
    );
    assert_eq!(
        store
            .prepare_search(&reader, std::slice::from_ref(&reference))
            .unwrap_err()
            .code,
        GardenErrorCode::NotFound
    );
    struct Invalid(f32);
    impl Vectorizer for Invalid {
        fn profile(&self) -> EmbeddingProfile {
            Synthetic.profile()
        }
        fn vectors(&self, _: &str) -> Result<EmbeddingVectors, GardenError> {
            Ok(EmbeddingVectors {
                vectors: vec![vec![self.0, self.0]],
                partial: false,
            })
        }
    }
    for value in [0., f32::NAN, f32::INFINITY] {
        assert_eq!(
            store
                .prepare_search_with(&owner, std::slice::from_ref(&reference), &Invalid(value))
                .unwrap_err()
                .code,
            GardenErrorCode::InvalidInput
        );
    }
    assert_eq!(
        store
            .query(&owner, &query(&ws, &generation))
            .unwrap()
            .coverage
            .search_mode,
        SearchMode::LiteralFallback
    );
    assert_eq!(store.audit(&owner).unwrap().sequence, 1);
}

#[test]
fn optional_profile_fingerprint_covers_model_policy_dimensions_and_version() {
    let profile = mother::search::embedding_profile();
    assert_eq!(
        profile.fingerprint(),
        mother::search::embedding_profile_fingerprint()
    );
    let original = profile.fingerprint();
    let mut changed = profile.clone();
    changed.model.push_str("changed");
    assert_ne!(changed.fingerprint(), original);
    let mut changed = profile.clone();
    changed.profile.push_str("changed");
    assert_ne!(changed.fingerprint(), original);
    let mut changed = profile.clone();
    changed.dimensions += 1;
    assert_ne!(changed.fingerprint(), original);
    let mut changed = profile;
    changed.version += 1;
    assert_ne!(changed.fingerprint(), original);
}
