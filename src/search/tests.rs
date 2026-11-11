use super::*;
use tempfile::TempDir;
use uuid::Uuid;
fn fixture() -> (TempDir, Store, Auth) {
    let d = tempfile::tempdir().unwrap();
    let mut s = Store::initialize(&d.path().join("context.sqlite"), "Synthetic persona").unwrap();
    let g = s
        .grant("test", &["read".into(), "memory:write".into()])
        .unwrap();
    let a = Auth {
        client_id: "test".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    (d, s, a)
}
fn write(s: &mut Store, a: &Auth, content: &str, old: Option<&Value>, status: &str) -> Value {
    let w = s.workspace().unwrap();
    let mut q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":"Synthetic","content":content,"status":status,"basis":"source"}});
    if let Some(r) = old {
        q["id"] = r["id"].clone();
        q["expectedRevision"] = r["revision"].clone();
    }
    s.write(a, q).unwrap()["record"].clone()
}
fn axis() -> Vec<f32> {
    let mut v = vec![0.; crate::embedding::DIMENSIONS];
    v[0] = 1.;
    v
}
fn cached(s: &Snapshot) -> HashMap<String, Entry> {
    s.records
        .iter()
        .map(|r| {
            (
                r.id.clone(),
                Entry {
                    partial: false,
                    vectors: vec![Vector {
                        start: 0,
                        end: r.text.len(),
                        values: axis(),
                    }],
                },
            )
        })
        .collect()
}
#[test]
fn changed_records_and_retractions_never_return_stale_semantic_matches() {
    let (_d, mut s, a) = fixture();
    let original = write(&mut s, &a, "old answer", None, "active");
    let query = Query::parse(json!({"query":"old answer"})).unwrap();
    let before = Snapshot::capture(&mut s, &a, &query).unwrap();
    let cache = cached(&before);
    let corrected = write(&mut s, &a, "corrected answer", Some(&original), "active");
    let result = finish(
        &mut s,
        &a,
        &query,
        &before,
        &cache,
        &axis(),
        &Progress::default(),
    )
    .unwrap();
    assert_eq!(result["search"]["reason"], "workspace_changed");
    assert_eq!(result["records"].as_array().unwrap().len(), 0);
    let query = Query::parse(json!({"query":"corrected answer"})).unwrap();
    let before = Snapshot::capture(&mut s, &a, &query).unwrap();
    let cache = cached(&before);
    write(
        &mut s,
        &a,
        "corrected answer",
        Some(&corrected),
        "retracted",
    );
    let result = finish(
        &mut s,
        &a,
        &query,
        &before,
        &cache,
        &axis(),
        &Progress::default(),
    )
    .unwrap();
    assert_eq!(result["search"]["reason"], "workspace_changed");
    assert!(result["records"].as_array().unwrap().is_empty());
}
#[test]
fn revoked_grants_are_errors_even_after_candidates_were_ranked() {
    let (_d, mut s, a) = fixture();
    write(&mut s, &a, "evidence", None, "active");
    let q = Query::parse(json!({"query":"evidence"})).unwrap();
    let before = Snapshot::capture(&mut s, &a, &q).unwrap();
    let cache = cached(&before);
    s.revoke("test").unwrap();
    let error = finish(
        &mut s,
        &a,
        &q,
        &before,
        &cache,
        &axis(),
        &Progress::default(),
    )
    .unwrap_err();
    assert_eq!(error.code(), "UNAUTHORIZED");
}
#[test]
fn missing_model_is_truthful_and_literal_retains_sqlite_unicode_and_punctuation_rules() {
    let (d, mut s, a) = fixture();
    write(&mut s, &a, "A 100%_ token CAFÉ and cafe", None, "active");
    let search = Search::new(d.path().join("missing-model"));
    for query in ["100%_", "CAFÉ", "café", "A 100", "token"] {
        let expected = s.context(&a, json!({"query":query})).unwrap();
        let result = search.search(&mut s, &a, json!({"query":query})).unwrap();
        assert_eq!(result["records"], expected["records"]);
        assert_eq!(result["search"]["reason"], "model_unavailable");
        assert_eq!(result["search"]["semanticUsed"], false);
    }
    assert!(!d.path().join("missing-model").exists());
    assert!(!cache::path(&d.path().join("context.sqlite")).exists());
}
#[test]
fn ranking_keeps_exact_hits_and_reports_partial_coverage_and_offsets() {
    let (_d, mut s, a) = fixture();
    let literal = write(&mut s, &a, "ZX-731 exact identifier", None, "active");
    write(&mut s, &a, "Another record", None, "active");
    let q = Query::parse(json!({"query":"ZX-731"})).unwrap();
    let snapshot = Snapshot::capture(&mut s, &a, &q).unwrap();
    let mut c = cached(&snapshot);
    c.get_mut(literal["id"].as_str().unwrap()).unwrap().partial = true;
    let result = semantic_response(snapshot, &q, &c, &axis(), &Progress::default()).unwrap();
    assert_eq!(result["records"][0]["id"], literal["id"]);
    assert_eq!(result["search"]["partialRecords"], 1);
    assert_eq!(result["search"]["reason"], "index_incomplete");
    assert_eq!(result["search"]["semanticUsed"], true);
    assert_eq!(
        result["records"][0]["match"]["text"],
        "Synthetic\nZX-731 exact identifier"
    );
}
#[test]
fn nothing_to_index_is_not_reported_as_an_incomplete_index() {
    let (_d, mut s, a) = fixture();
    let q = Query::parse(json!({"query":"anything"})).unwrap();
    let snapshot = Snapshot::capture(&mut s, &a, &q).unwrap();
    let result =
        semantic_response(snapshot, &q, &HashMap::new(), &axis(), &Progress::default()).unwrap();
    assert_eq!(result["search"]["eligibleRecords"], 0);
    assert_eq!(result["search"]["unindexedRecords"], 0);
    assert_eq!(result["search"]["reason"], Value::Null);
    assert!(result["records"].as_array().unwrap().is_empty());
    // Records that exist but are not indexed yet still fall back with a reason.
    write(&mut s, &a, "A record", None, "active");
    let snapshot = Snapshot::capture(&mut s, &a, &q).unwrap();
    let result =
        semantic_response(snapshot, &q, &HashMap::new(), &axis(), &Progress::default()).unwrap();
    assert_eq!(result["search"]["reason"], "index_incomplete");
    assert_eq!(result["search"]["semanticUsed"], false);
}
#[test]
fn a_time_window_limits_search_candidates_and_is_reported() {
    let (d, mut s, a) = fixture();
    let old = write(&mut s, &a, "Synthetic tea note", None, "active");
    std::thread::sleep(std::time::Duration::from_millis(5));
    let recent = write(&mut s, &a, "Synthetic tea plan", None, "active");
    let search = Search::new(d.path().join("missing-model"));
    let result = search
        .search(
            &mut s,
            &a,
            json!({"query":"tea","mode":"literal","updatedSince":recent["updatedAt"]}),
        )
        .unwrap();
    let ids: Vec<&Value> = result["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| &r["id"])
        .collect();
    assert_eq!(ids, [&recent["id"]]);
    assert_eq!(result["search"]["eligibleRecords"], 1);
    assert_eq!(result["search"]["updatedSince"], recent["updatedAt"]);
    assert_eq!(result["search"]["updatedBefore"], Value::Null);
    let result = search
        .search(
            &mut s,
            &a,
            json!({"query":"tea","mode":"literal","updatedBefore":recent["updatedAt"]}),
        )
        .unwrap();
    assert_eq!(result["records"][0]["id"], old["id"]);
    assert_eq!(result["records"].as_array().unwrap().len(), 1);
    assert_eq!(
        search
            .search(&mut s, &a, json!({"query":"tea","updatedSince":"soon"}))
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
}
#[test]
fn similarity_floor_excludes_unrelated_vectors_and_validates_query_vectors() {
    let (_d, mut s, a) = fixture();
    write(&mut s, &a, "A record", None, "active");
    let q = Query::parse(json!({"query":"unrelated query","mode":"semantic"})).unwrap();
    let snapshot = Snapshot::capture(&mut s, &a, &q).unwrap();
    let c = cached(&snapshot);
    let mut orthogonal = vec![0.; crate::embedding::DIMENSIONS];
    orthogonal[1] = 1.;
    let result = semantic_response(snapshot, &q, &c, &orthogonal, &Progress::default()).unwrap();
    assert!(result["records"].as_array().unwrap().is_empty());
    let snapshot = Snapshot::capture(&mut s, &a, &q).unwrap();
    assert!(semantic_response(snapshot, &q, &c, &[f32::NAN], &Progress::default()).is_err());
}
#[test]
fn restored_identity_never_reuses_a_prior_snapshot() {
    let (_d, mut s, a) = fixture();
    write(&mut s, &a, "current answer", None, "active");
    let q = Query::parse(json!({"query":"current answer"})).unwrap();
    let before = Snapshot::capture(&mut s, &a, &q).unwrap();
    let c = cached(&before);
    let mut w = s.workspace().unwrap();
    w["generation"] = json!(Uuid::new_v4().to_string());
    store::set_metadata(&s.conn, "workspace", &w).unwrap();
    let result = finish(&mut s, &a, &q, &before, &c, &axis(), &Progress::default()).unwrap();
    assert_eq!(result["workspace"], w);
    assert_eq!(result["search"]["reason"], "workspace_changed");
    assert_eq!(result["search"]["semanticUsed"], false);
}

#[test]
fn model_busy_fallback_is_current_and_does_not_wait() {
    let (d, mut s, a) = fixture();
    let record = write(&mut s, &a, "available context", None, "active");
    let search = Search::new(d.path().join("missing"));
    let _guard = search.model.lock().unwrap();
    let result = search
        .search(&mut s, &a, json!({"query":"available context"}))
        .unwrap();
    assert_eq!(result["records"][0]["id"], record["id"]);
    assert_eq!(result["search"]["reason"], "model_busy");
    s.revoke("test").unwrap();
    assert_eq!(
        search
            .search(&mut s, &a, json!({"query":"available context"}))
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
}

#[test]
fn short_match_keeps_bounded_neighbouring_sentence_context() {
    let text =
        "Header\nMembership may be terminated by written notice. Further monthly fees then cease.";
    let start = text.find("Further").unwrap();
    let (from, to) = context_span(text, start, text.len());
    assert!(text[from..to].contains("written notice"));
    assert!(from <= start);
    assert_eq!(to, text.len());
    let long = format!("{} End.", "a".repeat(500));
    let start = long.find("End").unwrap();
    let (from, to) = context_span(&long, start, long.len());
    assert_eq!(&long[from..to], "End.");
}

#[test]
fn neighbouring_context_survives_line_breaks_and_repeated_punctuation() {
    for separator in [".\n", ".\r\n", ".\n\n", "?! ", "... "] {
        let text = format!(
            "Header\nMembership may be terminated by written notice{separator}Further monthly fees then cease."
        );
        let start = text.find("Further").unwrap();
        let (from, to) = context_span(&text, start, text.len());
        assert!(
            text[from..to].contains("written notice"),
            "{separator:?} lost the previous sentence"
        );
        assert_eq!(to, text.len());
        let end = text.find("notice").unwrap() + "notice".len() + 1;
        let start = text.find("Membership").unwrap();
        let (from, to) = context_span(&text, start, end);
        assert!(
            text[from..to].ends_with("then cease."),
            "{separator:?} lost the next sentence"
        );
        assert!(text[from..to].starts_with("Header"));
    }
    let text = "Is it done?! Yes it is... Then stop.";
    let start = text.find("Yes").unwrap();
    let (from, to) = context_span(text, start, text.find(" Then").unwrap());
    assert_eq!(&text[from..to], text);
    // Text without any words still yields an in-bounds span.
    for text in ["!!!", "\n\n", "... ?"] {
        let (from, to) = context_span(text, 0, text.len());
        assert!(from <= to && to <= text.len());
        let (from, to) = context_span(text, text.len(), text.len());
        assert!(from <= to && to <= text.len());
    }
}
#[test]
fn owner_persona_change_during_search_returns_the_new_snapshot() {
    let (_dir, mut s, auth) = fixture();
    write(&mut s, &auth, "evidence", None, "active");
    let query = Query::parse(json!({"query":"evidence"})).unwrap();
    let before = Snapshot::capture(&mut s, &auth, &query).unwrap();
    let cache = cached(&before);
    let hash = s.persona().unwrap()["hash"].as_str().unwrap().to_owned();
    s.update_persona("Updated owner persona", &hash).unwrap();
    let result = finish(
        &mut s,
        &auth,
        &query,
        &before,
        &cache,
        &axis(),
        &Progress::default(),
    )
    .unwrap();
    assert_eq!(result["search"]["reason"], "workspace_changed");
    assert_eq!(result["search"]["semanticUsed"], false);
    assert_eq!(result["persona"]["content"], "Updated owner persona");
    assert_eq!(result["records"].as_array().unwrap().len(), 1);
}
#[test]
fn policy_withdrawal_during_semantic_search_discards_old_candidates_and_cache_is_isolated() {
    use crate::access::{AccessMode, ClientAccess};
    let (_d, mut s, a) = fixture();
    let r = write(&mut s, &a, "evidence", None, "active");
    let policy = ClientAccess {
        mode: AccessMode::Shared,
        subject: Some("agent-b".into()),
        records: vec![r["id"].as_str().unwrap().into()],
    };
    let g = s
        .grant_with_access("shared", &["read".into()], &policy)
        .unwrap();
    let b = Auth {
        client_id: "shared".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let q = Query::parse(json!({"query":"evidence"})).unwrap();
    let before = Snapshot::capture(&mut s, &b, &q).unwrap();
    let owner_snapshot = Snapshot::capture(&mut s, &a, &q).unwrap();
    assert_ne!(
        cache_path(&s, &a, &owner_snapshot).unwrap(),
        cache_path(&s, &b, &before).unwrap()
    );
    let vectors = cached(&before);
    s.conn
        .execute(
            "UPDATE client_access SET policy=? WHERE client_id='shared'",
            [json!({"mode":"shared","subject":"agent-b","records":[]}).to_string()],
        )
        .unwrap();
    let v = finish(
        &mut s,
        &b,
        &q,
        &before,
        &vectors,
        &axis(),
        &Progress::default(),
    )
    .unwrap();
    assert_eq!(v["records"], json!([]));
    assert_eq!(v["search"]["eligibleRecords"], 0);
    assert_eq!(v["search"]["newlyIndexedRecords"], 0);
    assert!(v.get("persona").is_none());
}
#[test]
fn in_flight_shared_cache_never_switches_to_global_when_policy_changes() {
    use crate::access::{AccessMode, ClientAccess};
    let (_d, mut s, a) = fixture();
    write(&mut s, &a, "evidence", None, "active");
    let g = s
        .grant_with_access(
            "shared",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: vec![],
            },
        )
        .unwrap();
    let b = Auth {
        client_id: "shared".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let query = Query::parse(json!({"query":"evidence"})).unwrap();
    let snapshot = Snapshot::capture(&mut s, &b, &query).unwrap();
    let original = cache_path(&s, &b, &snapshot).unwrap();
    s.conn
        .execute(
            "UPDATE client_access SET policy=? WHERE client_id='shared'",
            [json!({"mode":"companion","subject":"agent-b","records":[]}).to_string()],
        )
        .unwrap();
    assert_eq!(cache_path(&s, &b, &snapshot).unwrap(), original);
}
