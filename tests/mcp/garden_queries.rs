use super::*;

#[test]
fn graph_query_snapshot_and_change_checkpoint_work_over_stdio() {
    let dir = tempfile::tempdir().unwrap();
    let database = dir.path().join("garden.redb");
    let initial: BatchRequest =
        serde_json::from_str(include_str!("../fixtures/garden-v2.json")).unwrap();
    let store =
        GardenStore::initialize(&database, &initial.workspace_id, &initial.generation).unwrap();
    let auth = store
        .grant(
            "reader-writer",
            &["garden:read".into(), "garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    store.apply(&auth, &initial).unwrap();
    let path = dir.path().join("graph.private.json");
    std::fs::write(&path,serde_json::to_vec(&json!({"version":2,"backend":"garden_redb","databasePath":database,"clientId":auth.client_id,"token":auth.token})).unwrap()).unwrap();
    drop(store);
    let mut client = Client::connect(&path);
    let discovery = client.call("mother_get_capabilities", json!({}));
    assert_eq!(
        discovery["garden"]["availableTools"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    let mut query = json!({"version":2,"workspaceId":initial.workspace_id,"generation":initial.generation,"requiredExtensions":{},"ids":[],"kinds":[],"entityType":null,"predicate":null,"property":null,"validAt":null,"knownAt":null,"text":null,"vector":null,"traversal":null,"limit":1,"cursor":null});
    let first = client.call("mother_garden_query", query.clone());
    let mut seen = vec![
        first["hits"][0]["record"]["metadata"]["id"]
            .as_str()
            .unwrap()
            .to_owned(),
    ];
    let store = GardenStore::open(&database).unwrap();
    let mut later: Value = serde_json::to_value(&initial).unwrap();
    later["operationId"] = json!(uuid::Uuid::new_v4().to_string());
    later["items"] = json!([{"itemKey":"later","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":"Added after page one","aliases":[],"extensions":{}}}]);
    let added = store
        .apply(&auth, &serde_json::from_value(later).unwrap())
        .unwrap()
        .receipt
        .results[0]
        .reference
        .clone();
    drop(store);
    let first_cursor = first["page"]["nextCursor"].clone();
    let mut cursor = first_cursor.clone();
    while !cursor.is_null() {
        query["cursor"] = cursor;
        let next = client.call("mother_garden_query", query.clone());
        for hit in next["hits"].as_array().unwrap() {
            seen.push(hit["record"]["metadata"]["id"].as_str().unwrap().to_owned());
        }
        cursor = next["page"]["nextCursor"].clone();
    }
    assert_eq!(seen.len(), 3);
    assert!(!seen.contains(&added.id));
    query["cursor"] = first_cursor;
    query["predicate"] = json!("test:changed-query");
    let error = client.result("mother_garden_query", query);
    assert_eq!(
        error["structuredContent"]["error"]["code"],
        "RESTART_REQUIRED"
    );
    assert_eq!(error["structuredContent"]["error"]["restartRequired"], true);
    let mut changes = json!({"version":2,"workspaceId":initial.workspace_id,"generation":initial.generation,"requiredExtensions":{},"limit":1,"cursor":null});
    let mut events = Vec::new();
    for _ in 0..6 {
        let page = client.call("mother_garden_changes", changes.clone());
        changes["cursor"] = page["page"]["nextCursor"].clone();
        let rows = page["changes"].as_array().unwrap();
        if rows.is_empty() {
            break;
        }
        events.extend(rows.iter().cloned());
    }
    assert_eq!(events.len(), 4);
    assert!(!changes["cursor"].is_null());
    let store = GardenStore::open(&database).unwrap();
    store.apply(&auth,&serde_json::from_value(json!({"version":2,"workspaceId":initial.workspace_id,"generation":initial.generation,"operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"retire","mutation":"retract","id":added,"expectedRevision":1}]})).unwrap()).unwrap();
    drop(store);
    let tail = client.call("mother_garden_changes", changes);
    assert_eq!(tail["changes"].as_array().unwrap().len(), 1);
    assert_eq!(tail["changes"][0]["lifecycle"], "retracted");
    assert!(tail["changes"][0]["record"].is_null());
}
