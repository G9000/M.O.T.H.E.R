use mother::{
    contracts::{Auth, SCOPES},
    store::Store,
};
use serde_json::{Value, json};
#[test]
fn shared_policy_filters_inventory_persona_and_get() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("s.sqlite");
    let mut s = Store::initialize(&p, "Synthetic private persona").unwrap();
    let g = s.grant("owner", &SCOPES.map(str::to_owned)).unwrap();
    let a = Auth {
        client_id: "owner".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let w = s.workspace().unwrap();
    let mut ids = Vec::new();
    for title in ["Visible", "Hidden"] {
        let r=s.write(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","status":"active","basis":"owner_statement","title":title,"content":"Synthetic"}})).unwrap();
        ids.push(r["record"]["id"].as_str().unwrap().to_owned());
    }
    let g = s.grant("shared", &["read".into()]).unwrap();
    let b = Auth {
        client_id: "shared".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let c = rusqlite::Connection::open(&p).unwrap();
    c.execute_batch(
        "CREATE TABLE IF NOT EXISTS client_access(client_id TEXT PRIMARY KEY,policy TEXT NOT NULL)",
    )
    .unwrap();
    c.execute(
        "INSERT INTO client_access VALUES (?,?)",
        rusqlite::params![
            "shared",
            json!({"mode":"shared","subject":"agent-b","records":[ids[0]]}).to_string()
        ],
    )
    .unwrap();
    let v = s.context(&b, json!({})).unwrap();
    assert_eq!(
        v["inventory"]["totalRecords"], 1,
        "hidden record counts must not escape"
    );
    assert!(v.get("persona").is_none());
    assert_eq!(
        v["access"],
        json!({"version":1,"mode":"shared","subject":"agent-b"})
    );
    assert_eq!(s.get(&b, &ids[1]).unwrap_err().code(), "NOT_FOUND");
}
fn fixture() -> (tempfile::TempDir, Store, Auth, Vec<String>) {
    let d = tempfile::tempdir().unwrap();
    let mut s =
        Store::initialize(&d.path().join("state.sqlite"), "Private synthetic persona").unwrap();
    let g = s.grant("owner", &SCOPES.map(str::to_owned)).unwrap();
    let a = Auth {
        client_id: "owner".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let w = s.workspace().unwrap();
    let mut ids = vec![];
    for title in ["Visible", "Hidden"] {
        let r=s.write(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"task","status":"doing","title":title,"content":"Synthetic"}})).unwrap();
        ids.push(r["record"]["id"].as_str().unwrap().into());
    }
    (d, s, a, ids)
}
fn shared(s: &mut Store, ids: Vec<String>) -> Auth {
    let g = s
        .grant_with_access(
            "shared",
            &["read".into(), "context:propose".into()],
            &mother::access::ClientAccess {
                mode: mother::access::AccessMode::Shared,
                subject: Some("agent-b".into()),
                records: ids,
            },
        )
        .unwrap();
    Auth {
        client_id: "shared".into(),
        token: g["token"].as_str().unwrap().into(),
    }
}
#[test]
fn shared_search_filters_before_counts_and_fallback() {
    let (d, mut s, _a, ids) = fixture();
    let b = shared(&mut s, vec![ids[0].clone()]);
    let search = mother::search::Search::new(d.path().join("no-model"));
    for mode in ["literal", "hybrid", "semantic"] {
        let v = search
            .search(&mut s, &b, json!({"query":"Synthetic","mode":mode}))
            .unwrap();
        assert_eq!(v["search"]["eligibleRecords"], 1, "{mode}");
        assert_eq!(v["records"].as_array().unwrap().len(), 1);
        assert!(v.get("persona").is_none());
        assert_eq!(v["unsettledWork"].as_array().unwrap().len(), 1);
    }
}
#[test]
fn shared_proposals_do_not_reveal_other_clients_or_duplicate_metadata() {
    let (_d, mut s, a, ids) = fixture();
    let b = shared(&mut s, vec![ids[0].clone()]);
    let w = s.workspace().unwrap();
    let proposal = |id: Option<&str>| {
        let mut v = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":if id.is_some(){1}else{0},"source":"Synthetic","basis":"owner_statement","record":{"kind":"task","status":"doing","title":"Hidden","content":"Synthetic"}});
        if let Some(id) = id {
            v["id"] = json!(id);
        }
        v
    };
    let own = s.propose(&a, proposal(None)).unwrap();
    assert_eq!(
        s.get_proposal(&b, own["proposal"]["id"].as_str().unwrap())
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    assert_eq!(
        s.propose(&b, proposal(Some(&ids[1]))).unwrap_err().code(),
        "NOT_FOUND"
    );
    let p = s.propose(&b, proposal(None)).unwrap();
    assert_eq!(p["proposal"]["duplicateRecordId"], Value::Null);
    assert_eq!(p["proposal"]["similarRecordIds"], json!([ids[0]]));
    let r = s.receipt(&b, p["operationId"].as_str().unwrap()).unwrap();
    assert_eq!(r["proposal"]["duplicateRecordId"], Value::Null);
}
#[test]
fn withdrawal_blocks_saved_proposal_receipt_and_reference_history() {
    let (d, mut s, _a, ids) = fixture();
    let b = shared(&mut s, vec![ids[0].clone()]);
    let w = s.workspace().unwrap();
    let q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":1,"id":ids[0],"source":"Synthetic","basis":"owner_statement","record":{"kind":"task","status":"done","title":"Updated","content":"Synthetic"}});
    let p = s.propose(&b, q.clone()).unwrap();
    let pid = p["proposal"]["id"].as_str().unwrap();
    s.review_proposal(json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":1,"proposalId":pid,"action":"save"})).unwrap();
    let c = rusqlite::Connection::open(d.path().join("state.sqlite")).unwrap();
    c.execute(
        "UPDATE client_access SET policy=? WHERE client_id='shared'",
        [json!({"mode":"shared","subject":"agent-b","records":[]}).to_string()],
    )
    .unwrap();
    assert_eq!(s.get_proposal(&b, pid).unwrap_err().code(), "NOT_FOUND");
    assert_eq!(
        s.receipt(&b, q["operationId"].as_str().unwrap())
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
    assert_eq!(
        s.context(&b, json!({})).unwrap()["inventory"]["totalRecords"],
        0
    );
    s.revoke("shared").unwrap();
    assert_eq!(s.context(&b, json!({})).unwrap_err().code(), "UNAUTHORIZED");
    assert_eq!(s.access_for(&b).unwrap_err().code(), "UNAUTHORIZED");
}
#[test]
fn companion_scopes_remain_orthogonal_and_policies_do_not_restore() {
    use mother::access::{AccessMode, ClientAccess};
    let (d, mut s, a, _ids) = fixture();
    let w = s.workspace().unwrap();
    let g = s
        .grant_with_access(
            "companion",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Companion,
                subject: Some("agent-a".into()),
                records: vec![],
            },
        )
        .unwrap();
    let b = Auth {
        client_id: "companion".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let c = s.context(&b, json!({})).unwrap();
    assert_eq!(c["access"]["mode"], "companion");
    assert!(c.get("persona").is_some());
    assert_eq!(c["inventory"]["totalRecords"], 2);
    assert_eq!(
        s.authorize(&b, "task:write").unwrap_err().code(),
        "FORBIDDEN"
    );
    let before = s.context(&a, json!({})).unwrap();
    assert_eq!(before["access"]["mode"], "legacy");
    assert_eq!(before["workspace"], w);
    let export = s.export().unwrap();
    assert!(!export.to_string().contains("agent-a"));
    let mut restored = Store::restore(&d.path().join("restored.sqlite"), export).unwrap();
    assert_eq!(
        restored.context(&b, json!({})).unwrap_err().code(),
        "UNAUTHORIZED"
    );
    let c = rusqlite::Connection::open(d.path().join("restored.sqlite")).unwrap();
    let n: i64 = c
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE name IN ('client_access','client_policy_requirements')",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(n, 0);
    let bad = ClientAccess {
        mode: AccessMode::Shared,
        subject: Some("agent-b".into()),
        records: vec![uuid::Uuid::new_v4().to_string()],
    };
    assert_eq!(
        s.grant_with_access("bad", &["read".into()], &bad)
            .unwrap_err()
            .code(),
        "NOT_FOUND"
    );
}
#[test]
fn saved_new_proposal_does_not_implicitly_share_the_canonical_record() {
    let (_d, mut s, _a, _ids) = fixture();
    let b = shared(&mut s, vec![]);
    let w = s.workspace().unwrap();
    let q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"source":"Synthetic","basis":"owner_statement","record":{"kind":"memory","basis":"owner_statement","status":"active","title":"New","content":"Synthetic new memory"}});
    let p = s.propose(&b, q).unwrap();
    let pid = p["proposal"]["id"].as_str().unwrap();
    s.review_proposal(json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":1,"proposalId":pid,"action":"save"})).unwrap();
    assert_eq!(s.get_proposal(&b, pid).unwrap_err().code(), "NOT_FOUND");
}
#[test]
fn withdrawn_target_cannot_be_approved_through_an_old_shared_proposal() {
    let (d, mut s, _a, ids) = fixture();
    let b = shared(&mut s, vec![ids[0].clone()]);
    let w = s.workspace().unwrap();
    let q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":1,"id":ids[0],"source":"Synthetic","basis":"owner_statement","record":{"kind":"task","status":"done","title":"Updated","content":"Synthetic"}});
    let p = s.propose(&b, q).unwrap();
    let c = rusqlite::Connection::open(d.path().join("state.sqlite")).unwrap();
    c.execute(
        "UPDATE client_access SET policy=? WHERE client_id='shared'",
        [json!({"mode":"shared","subject":"agent-b","records":[]}).to_string()],
    )
    .unwrap();
    assert_eq!(s.review_proposal(json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":1,"proposalId":p["proposal"]["id"],"action":"save"})).unwrap_err().code(),"NOT_FOUND");
}
#[test]
fn proposal_similar_metadata_filters_before_its_limit() {
    let (_d, mut s, a, mut ids) = fixture();
    let w = s.workspace().unwrap();
    for n in 0..8 {
        let r=s.write(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"task","status":"doing","title":format!("Task {n}"),"content":"Synthetic"}})).unwrap();
        ids.push(r["record"]["id"].as_str().unwrap().into());
    }
    ids.sort();
    let visible = ids.last().unwrap().clone();
    let b = shared(&mut s, vec![visible.clone()]);
    let p=s.propose(&b,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":0,"source":"Synthetic","basis":"owner_statement","record":{"kind":"task","status":"doing","title":"Unique proposal","content":"Synthetic"}})).unwrap();
    assert_eq!(p["proposal"]["similarRecordIds"], json!([visible]));
}
#[test]
fn reference_redaction_keeps_internal_proposal_audit_portable() {
    let (d, mut s, a, ids) = fixture();
    let w = s.workspace().unwrap();
    s.write(&a,json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":1,"id":ids[0],"record":{"kind":"task","status":"doing","title":"Visible","content":"Synthetic","references":[ids[1]]}})).unwrap();
    let b = shared(&mut s, vec![ids[0].clone()]);
    assert_eq!(s.get(&b, &ids[0]).unwrap()["references"], json!([]));
    let q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":uuid::Uuid::new_v4().to_string(),"expectedRevision":2,"id":ids[0],"source":"Synthetic","basis":"owner_statement","record":{"kind":"task","status":"done","title":"Updated","content":"Synthetic"}});
    let p = s.propose(&b, q).unwrap();
    assert_eq!(p["proposal"]["before"]["references"], json!([]));
    let bundle = s.export().unwrap();
    Store::restore(&d.path().join("restored.sqlite"), bundle).unwrap();
}
#[test]
fn scoped_private_configs_require_policy_aware_loader_and_authenticated_policy() {
    use mother::{
        access::{AccessMode, ClientAccess},
        config::load_config,
    };
    let (d, mut s, a, _ids) = fixture();
    let path = d.path().join("shared.private.json");
    let db = d.path().join("state.sqlite");
    s.grant_to_file_with_access(
        "scoped-file",
        &["read".into()],
        &path,
        &db,
        &ClientAccess {
            mode: AccessMode::Shared,
            subject: Some("agent-b".into()),
            records: vec![],
        },
    )
    .unwrap();
    let raw: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(
        raw["version"], 2,
        "old v1-only executables must reject scoped credentials"
    );
    // This is the original loader's version gate, including integral JSON numbers.
    let old_loader_accepts = raw["version"].as_f64().is_some_and(|n| n == 1.0);
    assert!(!old_loader_accepts);
    assert_eq!(
        raw.as_object().unwrap().keys().cloned().collect::<Vec<_>>(),
        vec!["clientId", "databasePath", "token", "version"]
    );
    let config = load_config(&path).unwrap();
    assert_eq!(config.version, 2);
    assert_eq!(
        s.context(&config.auth(), json!({})).unwrap()["access"]["mode"],
        "shared"
    );
    let c = rusqlite::Connection::open(&db).unwrap();
    c.execute(
        "DELETE FROM client_access WHERE client_id='scoped-file'",
        [],
    )
    .unwrap();
    assert_eq!(load_config(&path).unwrap_err().code(), "FORBIDDEN");
    assert_eq!(
        s.context(&a, json!({})).unwrap()["access"]["mode"],
        "legacy"
    );
    let legacy = d.path().join("legacy.private.json");
    s.grant_to_file("legacy-file", &["read".into()], &legacy, &db)
        .unwrap();
    assert_eq!(load_config(&legacy).unwrap().version, 1);
}
#[test]
fn scoped_policy_requirement_survives_withdrawal_between_precheck_and_transaction() {
    let (d, mut s, a, ids) = fixture();
    let b = shared(&mut s, vec![ids[0].clone()]);
    // Simulate predispatch authorization succeeding before another owner connection withdraws policy.
    assert_eq!(
        s.access_for(&b).unwrap().mode,
        mother::access::AccessMode::Shared
    );
    let db = rusqlite::Connection::open(d.path().join("state.sqlite")).unwrap();
    db.execute("DELETE FROM client_access WHERE client_id='shared'", [])
        .unwrap();
    assert_eq!(s.context(&b, json!({})).unwrap_err().code(), "FORBIDDEN");
    assert_eq!(s.get(&b, &ids[1]).unwrap_err().code(), "FORBIDDEN");
    let search = mother::search::Search::new(d.path().join("no-model"));
    for mode in ["literal", "hybrid", "semantic"] {
        assert_eq!(
            search
                .search(&mut s, &b, json!({"query":"Synthetic","mode":mode}))
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
    }
    assert_eq!(
        s.context(&a, json!({})).unwrap()["access"]["mode"],
        "legacy"
    );
    db.execute(
        "INSERT INTO client_access VALUES ('shared',?)",
        [json!({"mode":"legacy","subject":null,"records":[]}).to_string()],
    )
    .unwrap();
    assert_eq!(s.context(&b, json!({})).unwrap_err().code(), "FORBIDDEN");
    db.execute_batch("DROP TABLE client_access").unwrap();
    assert_eq!(s.context(&b, json!({})).unwrap_err().code(), "FORBIDDEN");
}
