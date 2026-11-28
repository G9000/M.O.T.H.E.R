//! Context retrieval, pagination, filtering and semantic indexing lifecycle over a
//! synthetic native workspace. No real records, models or network are used.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    embedding::EmbeddingProfile,
    error::{Error, Result},
    garden::{
        store::GardenStore,
        v1_search::{
            EncodedPassage, EncodedPassages, Encoder, NativeSearch, index_status, index_with,
            reset_index,
        },
    },
};
use serde_json::{Value, json};
use std::{
    collections::HashSet,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use uuid::Uuid;

struct Fixture {
    dir: tempfile::TempDir,
    store: GardenStore,
    workspace: String,
    generation: String,
    chatgpt: Auth,
    codex: Auth,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store =
            GardenStore::initialize(dir.path().join("native.redb"), &workspace, &generation)
                .unwrap();
        // A persona as large as the live one that starved previews.
        store
            .v1_initialize(&"Synthetic persona paragraph. ".repeat(300))
            .unwrap();
        let scopes = ["read".to_owned(), "memory:write".to_owned()];
        let chatgpt = store
            .v1_grant("synthetic-chatgpt", &scopes, &ClientAccess::default(), None)
            .unwrap();
        let codex = store
            .v1_grant("synthetic-codex", &scopes, &ClientAccess::default(), None)
            .unwrap();
        Self {
            dir,
            store,
            workspace,
            generation,
            chatgpt,
            codex,
        }
    }
    fn write(&self, auth: &Auth, title: &str, basis: &str) -> Value {
        let request = json!({"workspaceId":self.workspace,"generation":self.generation,
            "operationId":Uuid::new_v4().to_string(),"expectedRevision":0,
            "record":{"kind":"memory","title":title,"content":format!("{title} happened in a synthetic story."),
                "status":"active","basis":basis}});
        self.store
            .v1_call(auth, "mother_write_record", request)
            .unwrap()["record"]
            .clone()
    }
    fn revise(&self, auth: &Auth, record: &Value, content: &str, status: &str) -> Value {
        let request = json!({"workspaceId":self.workspace,"generation":self.generation,
            "operationId":Uuid::new_v4().to_string(),"id":record["id"],"expectedRevision":record["revision"],
            "record":{"kind":"memory","title":record["title"],"content":content,"status":status,"basis":record["basis"]}});
        self.store
            .v1_call(auth, "mother_write_record", request)
            .unwrap()["record"]
            .clone()
    }
    fn context(&self, auth: &Auth, args: Value) -> Result<Value> {
        self.store.v1_call(auth, "mother_get_context", args)
    }
}

fn ids(page: &Value) -> Vec<String> {
    page["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn explicit_budget_requires_valid_size_and_pageable_order() {
    let f = Fixture::new();
    let auth = &f.codex;
    assert_eq!(
        f.context(auth, json!({})).unwrap()["pagination"]["order"],
        "balanced"
    );
    assert_eq!(
        f.context(auth, json!({"maxResponseBytes":4096})).unwrap()["pagination"]["order"],
        "recent"
    );
    assert_eq!(
        f.context(auth, json!({"maxResponseBytes":12288,"order":"created"}))
            .unwrap()["pagination"]["order"],
        "created"
    );
    assert_eq!(
        f.context(auth, json!({"maxResponseBytes":4096,"order":"balanced"}))
            .unwrap_err()
            .code(),
        "INVALID_INPUT"
    );
    for invalid in [
        json!(4095),
        json!(12289),
        json!(4096.5),
        json!("6000"),
        Value::Null,
    ] {
        for (tool, mut args) in [
            ("mother_get_context", json!({})),
            (
                "mother_search_context",
                json!({"query":"sample","mode":"literal"}),
            ),
        ] {
            args["maxResponseBytes"] = invalid.clone();
            assert_eq!(
                f.store.v1_call(auth, tool, args).unwrap_err().code(),
                "INVALID_INPUT"
            );
        }
    }
}

#[test]
fn project_visits_compare_exact_seen_revisions_and_respect_current_grants() {
    let f = Fixture::new();
    let writer = f
        .store
        .v1_grant(
            "work-writer",
            &["read", "decision:write", "task:write", "checkpoint:write"].map(String::from),
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    let create = |record| {
        f.store.v1_call(&writer,"mother_write_record",json!({
        "workspaceId":f.workspace,"generation":f.generation,"operationId":Uuid::new_v4().to_string(),
        "expectedRevision":0,"record":record})).unwrap()["record"].clone()
    };
    let decision = create(
        json!({"kind":"decision","title":"North Star launch","content":"Use a local build","status":"confirmed"}),
    );
    create(
        json!({"kind":"decision","title":"North Star idea","content":"Switch the build","status":"proposed"}),
    );
    let checkpoint = create(
        json!({"kind":"checkpoint","title":"North-Star progress","content":"Build completed","status":"active","nextStep":"Test installation"}),
    );
    let task = create(
        json!({"kind":"task","title":"Inspect delivery","content":"North Star packaging","status":"blocked"}),
    );
    f.write(&f.codex, "North Starling excluded", "source");
    let memory = f.write(&f.chatgpt, "North Star meeting", "source");
    let resume = |auth| {
        f.store
            .v1_call(
                auth,
                "mother_project_resume",
                json!({"project":"NORTH star"}),
            )
            .unwrap()
    };
    let mark = |auth, seen| {
        f.store.v1_call(auth,"mother_mark_project_seen",json!({"project":"north-star","workspaceId":f.workspace,"generation":f.generation,"seen":seen}))
    };
    let first = resume(&f.codex);
    assert_eq!(first["matchingRecords"], 5);
    assert_eq!(first["confirmedDecision"]["id"], decision["id"]);
    assert_eq!(first["checkpoint"]["id"], checkpoint["id"]);
    assert_eq!(first["openTask"]["id"], task["id"]);
    assert_eq!(first["remaining"], 2);
    let ack = |page: &Value| {
        Value::Array(
            page["changes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| r["seen"].clone())
                .collect(),
        )
    };
    mark(&f.codex, ack(&first)).unwrap();
    mark(&f.codex, ack(&resume(&f.codex))).unwrap();
    assert_eq!(resume(&f.codex)["changes"], json!([]));
    assert_eq!(resume(&f.chatgpt)["hasPreviousVisit"], false);
    let second = f.revise(&f.chatgpt, &memory, "Intermediate wording", "active");
    f.revise(&f.chatgpt, &second, "Corrected account", "active");
    let diff = &resume(&f.codex)["changes"][0];
    assert_eq!(diff["before"]["revision"], 1);
    assert_eq!(diff["current"]["revision"], 3);
    assert_eq!(diff["wording"]["after"], "Corrected account");
    mark(&f.codex, json!([diff["seen"]])).unwrap();
    mark(&f.codex, json!([{"id":memory["id"],"revision":1}])).unwrap();
    assert_eq!(resume(&f.codex)["changes"], json!([]));
    let guest = f
        .store
        .v1_grant(
            "shared-visitor",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("sample-reader".into()),
                records: vec![memory["id"].as_str().unwrap().into()],
            },
            None,
        )
        .unwrap();
    let restricted = resume(&guest);
    assert_eq!(restricted["matchingRecords"], 1);
    assert!(restricted["changes"][0]["before"].is_null());
    assert!(!restricted.to_string().contains("Intermediate wording"));
    assert!(
        mark(
            &guest,
            json!([{"id":memory["id"],"revision":3},{"id":task["id"],"revision":1}])
        )
        .is_err()
    );
    assert_eq!(resume(&guest)["hasPreviousVisit"], false);
}

#[test]
fn aliases_keepsakes_and_clues_follow_grants_and_saved_positions() {
    let f = Fixture::new();
    let r = f.write(&f.chatgpt, "Hearth lantern rehearsal", "owner_statement");
    let other = f.write(&f.chatgpt, "Paperweight rehearsal", "source");
    let id = r["id"].as_str().unwrap();
    let resume = |project| {
        f.store
            .v1_call(
                &f.codex,
                "mother_project_resume",
                json!({"project":project}),
            )
            .unwrap()
    };
    let seen = resume("hearth")["changes"][0]["seen"].clone();
    f.store.v1_call(&f.codex,"mother_mark_project_seen",json!({"workspaceId":f.workspace,"generation":f.generation,"project":"hearth","seen":[seen]})).unwrap();
    f.store.project_alias("Ember", "Hearth", false).unwrap();
    assert!(f.store.project_alias("Hearth", "Ember", false).is_err());
    assert_eq!(resume("ember")["changes"], json!([]));
    assert_eq!(resume("ember")["matchingRecords"], 1);
    let search = || {
        f.store
            .v1_call(
                &f.codex,
                "mother_search_context",
                json!({"query":"Ember","mode":"literal"}),
            )
            .unwrap()
    };
    assert_eq!(
        search()["records"][0]["recallReason"]["lexical"]["projectName"],
        "hearth"
    );
    assert_eq!(
        f.context(&f.codex, json!({"query":"Ember"})).unwrap()["records"][0]["id"],
        id
    );
    let guest = f
        .store
        .v1_grant(
            "visitor",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("visitor".into()),
                records: vec![other["id"].as_str().unwrap().into()],
            },
            None,
        )
        .unwrap();
    let find = |auth: &Auth, include| {
        f.store.v1_call(auth,"mother_find_moment",json!({"clue":"show me the day of the lantern rehearsal","includeKeepsakes":include})).unwrap()
    };
    assert!(find(&f.codex, false)["records"][0]["eventDate"].is_null());
    f.store.set_event_date(id, Some("2024-02-29")).unwrap();
    assert!(f.store.set_event_date(id, Some("2023-02-29")).is_err());
    f.store.set_keepsake(id, true).unwrap();
    for auth in [&f.chatgpt, &f.codex] {
        assert!(!ids(&f.context(auth, json!({})).unwrap()).contains(&id.to_string()));
        assert!(!ids(&find(auth, false)).contains(&id.to_string()));
        let moment = &find(auth, true)["records"][0];
        assert_eq!(moment["id"], id);
        assert_eq!(moment["recordedAt"], r["createdAt"]);
        assert_eq!(moment["eventDate"]["value"], "2024-02-29");
    }
    assert!(!ids(&find(&guest, true)).contains(&id.to_string()));
    assert_eq!(resume("hearth")["matchingRecords"], 0);
    assert_eq!(search()["records"], json!([]));
    assert_eq!(f.store.v1_garden_reader(id).unwrap()["keepsake"], true);
    assert_eq!(
        f.store.v1_export().unwrap_err().code(),
        "FULL_BACKUP_REQUIRED"
    );
    f.store.set_keepsake(id, false).unwrap();
    assert_eq!(resume("ember")["matchingRecords"], 1);
    f.store.project_alias("Ember", "Hearth", true).unwrap();
    assert_eq!(resume("ember")["matchingRecords"], 0);
    assert_eq!(resume("hearth")["changes"], json!([]));
}

/// Follow `nextCursor` until `hasMore` is false, collecting every returned ID.
fn enumerate(f: &Fixture, auth: &Auth, mut args: Value) -> Vec<String> {
    let mut seen = Vec::new();
    for _ in 0..1000 {
        let page = f.context(auth, args.clone()).unwrap();
        seen.extend(ids(&page));
        if page["pagination"]["hasMore"] == false {
            return seen;
        }
        args["cursor"] = page["pagination"]["nextCursor"].clone();
    }
    panic!("pagination did not terminate");
}

#[test]
fn actor_filtered_pages_enumerate_every_record_once_in_stable_order() {
    let f = Fixture::new();
    let mut chatgpt = HashSet::new();
    for i in 0..140 {
        chatgpt.insert(
            f.write(
                &f.chatgpt,
                &format!("ChatGPT memory {i}"),
                "owner_statement",
            )["id"]
                .as_str()
                .unwrap()
                .to_owned(),
        );
        if i % 2 == 0 {
            f.write(&f.codex, &format!("Codex memory {i}"), "model_inference");
        }
    }
    for order in ["created", "recent"] {
        let seen = enumerate(
            &f,
            &f.codex,
            json!({"kind":"memory","actor":"synthetic-chatgpt","limit":25,"order":order,
                "includePersona":false,"includePinned":false,"previewMaxChars":300}),
        );
        assert_eq!(seen.len(), 140, "{order}: no record skipped or repeated");
        assert_eq!(seen.iter().cloned().collect::<HashSet<_>>(), chatgpt);
    }
    // Defaults follow the example contract: filters imply newest-first pages with cursors.
    let first = f
        .context(
            &f.chatgpt,
            json!({"kind":"memory","actor":"synthetic-chatgpt","limit":25,"includePersona":false,"includePinned":false,"previewMaxChars":300}),
        )
        .unwrap();
    assert_eq!(first["pagination"]["order"], "recent");
    assert_eq!(first["pagination"]["matchingRecords"], 140);
    assert_eq!(first["pagination"]["returnedRecords"], 25);
    assert_eq!(first["pagination"]["hasMore"], true);
    assert_eq!(first["contextSections"]["persona"], "excluded");
    assert_eq!(first["contextSections"]["pinned"], "excluded");
    assert!(first.get("persona").is_none());
    assert!(
        first["records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["actor"] == "synthetic-chatgpt"
                && r["excerpt"].as_str().unwrap().encode_utf16().count() <= 300)
    );
    // basis distinguishes owner statements from assistant interpretations server-side.
    let inferred = enumerate(
        &f,
        &f.chatgpt,
        json!({"basis":"model_inference","order":"created","limit":100}),
    );
    assert_eq!(inferred.len(), 70);
    let repeat = f
        .context(
            &f.chatgpt,
            json!({"kind":"memory","actor":"synthetic-chatgpt","limit":25,"includePersona":false,"includePinned":false,"previewMaxChars":300}),
        )
        .unwrap();
    assert_eq!(ids(&first), ids(&repeat), "stable ordering");
}

#[test]
fn concurrent_changes_during_pagination_are_reported_not_skipped() {
    let f = Fixture::new();
    let mut records = Vec::new();
    for i in 0..30 {
        records.push(f.write(&f.chatgpt, &format!("Story {i}"), "owner_statement"));
    }
    for order in ["recent", "created"] {
        let args = json!({"kind":"memory","order":order,"limit":10});
        let first = f.context(&f.chatgpt, args.clone()).unwrap();
        let shown: HashSet<String> = ids(&first).into_iter().collect();
        // Revise one record that was not shown yet, add a new one and retract one more.
        let unseen = records
            .iter()
            .find(|r| !shown.contains(r["id"].as_str().unwrap()))
            .unwrap()
            .clone();
        let revised = f.revise(&f.chatgpt, &unseen, "Revised while paging", "active");
        let added = f.write(&f.chatgpt, &format!("Added during {order}"), "source");
        let retired = records
            .iter()
            .rev()
            .find(|r| !shown.contains(r["id"].as_str().unwrap()) && r["id"] != unseen["id"])
            .unwrap()
            .clone();
        let current = f
            .store
            .v1_call(&f.chatgpt, "mother_get_record", json!({"id":retired["id"]}))
            .unwrap();
        f.revise(&f.chatgpt, &current, "Retracted", "retracted");
        let mut seen: Vec<String> = ids(&first);
        let mut changed = Vec::new();
        let mut next = args.clone();
        next["cursor"] = first["pagination"]["nextCursor"].clone();
        loop {
            let page = f.context(&f.chatgpt, next.clone()).unwrap();
            seen.extend(ids(&page));
            for c in page["pagination"]["changed"].as_array().unwrap() {
                changed.push(c["id"].as_str().unwrap().to_owned());
            }
            if page["pagination"]["hasMore"] == false {
                break;
            }
            next["cursor"] = page["pagination"]["nextCursor"].clone();
        }
        let unique: HashSet<_> = seen.iter().cloned().collect();
        assert_eq!(unique.len(), seen.len(), "{order}: a record repeated");
        let revised_id = revised["id"].as_str().unwrap().to_owned();
        let added_id = added["id"].as_str().unwrap().to_owned();
        assert!(
            unique.contains(&revised_id) || changed.contains(&revised_id),
            "{order}: revised record lost"
        );
        assert!(
            unique.contains(&added_id) || changed.contains(&added_id),
            "{order}: added record lost"
        );
        assert!(!unique.contains(retired["id"].as_str().unwrap()));
        records.retain(|r| r["id"] != retired["id"]);
        records.push(added);
        for r in records.iter_mut() {
            *r = f
                .store
                .v1_call(&f.chatgpt, "mother_get_record", json!({"id":r["id"]}))
                .unwrap();
        }
    }
}

#[test]
fn cursors_are_bound_to_their_query_and_client() {
    let f = Fixture::new();
    for i in 0..5 {
        f.write(&f.chatgpt, &format!("Bound {i}"), "owner_statement");
    }
    let page = f
        .context(&f.chatgpt, json!({"order":"created","limit":2}))
        .unwrap();
    let cursor = page["pagination"]["nextCursor"].clone();
    let other_filter = f.context(
        &f.chatgpt,
        json!({"order":"created","limit":2,"actor":"synthetic-codex","cursor":cursor}),
    );
    assert_eq!(other_filter.unwrap_err().code(), "INVALID_INPUT");
    let other_client = f.context(
        &f.codex,
        json!({"order":"created","limit":2,"cursor":cursor}),
    );
    assert_eq!(other_client.unwrap_err().code(), "INVALID_INPUT");
    let tampered = f.context(
        &f.chatgpt,
        json!({"order":"created","limit":2,"cursor":"not-a-cursor"}),
    );
    assert_eq!(tampered.unwrap_err().code(), "INVALID_INPUT");
    let historical = f.context(&f.chatgpt, json!({"status":"retracted"}));
    assert_eq!(historical.unwrap_err().code(), "INVALID_INPUT");
    let balanced = f.context(&f.chatgpt, json!({})).unwrap();
    assert_eq!(balanced["pagination"]["order"], "balanced");
    assert!(balanced["pagination"]["nextCursor"].is_null());
    assert_eq!(balanced["contextSections"]["persona"], "included");
}

#[test]
fn shared_clients_paginate_only_their_granted_records() {
    let f = Fixture::new();
    let mut granted = Vec::new();
    for i in 0..12 {
        let r = f.write(&f.chatgpt, &format!("Shared {i}"), "owner_statement");
        if i % 3 == 0 {
            granted.push(r["id"].as_str().unwrap().to_owned());
        }
    }
    let access = ClientAccess {
        mode: AccessMode::Shared,
        subject: Some("synthetic-guest".into()),
        records: granted.clone(),
    };
    let guest = f
        .store
        .v1_grant("synthetic-guest", &["read".into()], &access, None)
        .unwrap();
    let seen = enumerate(
        &f,
        &guest,
        json!({"order":"created","limit":2,"actor":"synthetic-chatgpt"}),
    );
    assert_eq!(
        seen.into_iter().collect::<HashSet<_>>(),
        granted.into_iter().collect::<HashSet<_>>()
    );
    let page = f.context(&guest, json!({"order":"created"})).unwrap();
    assert!(page.get("persona").is_none());
    assert_eq!(page["pagination"]["matchingRecords"], 4);
}

#[derive(Default)]
struct Synthetic {
    passages: AtomicUsize,
    fail: AtomicBool,
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
        Ok(vec![1., 0.])
    }
    fn passages(&self, text: &str) -> Result<EncodedPassages> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(Error::domain("MODEL_UNAVAILABLE", "synthetic failure"));
        }
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
fn background_indexing_is_incremental_durable_and_independent_of_searches() {
    let mut f = Fixture::new();
    let mut records = Vec::new();
    for i in 0..20 {
        records.push(f.write(&f.chatgpt, &format!("Indexed {i}"), "owner_statement"));
    }
    let encoder = Synthetic::default();
    let profile = encoder.profile();
    let status = index_status(&f.store, None, &profile).unwrap();
    assert_eq!(status["eligibleRecords"], 20);
    assert_eq!(status["pendingRecords"], 20);
    // No searches: the owner-side indexer alone completes the index.
    let mut steps = 0;
    while index_with(&f.store, &encoder, 4).unwrap().indexed > 0 {
        steps += 1;
    }
    assert_eq!(steps, 5);
    let status = index_status(&f.store, None, &profile).unwrap();
    assert_eq!(status["indexedRecords"], 20);
    assert_eq!(status["complete"], true);
    assert!(status["lastSuccessAt"].is_string());
    // Revisions invalidate exactly one vector; retraction removes one from eligibility.
    let revised = f.revise(&f.chatgpt, &records[3], "A new chapter", "active");
    f.revise(&f.chatgpt, &records[4], "Retracted story", "retracted");
    let status = index_status(&f.store, None, &profile).unwrap();
    assert_eq!(status["eligibleRecords"], 19);
    assert_eq!(status["pendingRecords"], 1);
    assert_eq!(status["staleVectors"], 2);
    let report = index_with(&f.store, &encoder, 10).unwrap();
    assert_eq!(report.indexed, 1);
    assert_eq!(report.pruned, 2);
    // Restart recovery: progress lives in the database, not in memory.
    let path = f.dir.path().join("native.redb");
    drop(std::mem::replace(
        &mut f.store,
        GardenStore::initialize(
            f.dir.path().join("scratch.redb"),
            &Uuid::new_v4().to_string(),
            &Uuid::new_v4().to_string(),
        )
        .unwrap(),
    ));
    f.store = GardenStore::open(&path).unwrap();
    let before = encoder.passages.load(Ordering::SeqCst);
    assert_eq!(index_with(&f.store, &encoder, 10).unwrap().indexed, 0);
    assert_eq!(encoder.passages.load(Ordering::SeqCst), before);
    let status = index_status(&f.store, None, &profile).unwrap();
    assert_eq!(status["indexedRecords"], 19);
    assert_eq!(status["staleVectors"], 0);
    // Searches reuse the shared index for every client: nothing new is embedded.
    let search = NativeSearch::new(f.dir.path().join("missing-local-model"));
    for auth in [&f.chatgpt, &f.codex] {
        let result = search
            .search_with(
                &f.store,
                auth,
                json!({"query":"chapter","mode":"semantic","includePersona":false}),
                &encoder,
            )
            .unwrap();
        assert_eq!(result["search"]["indexedRecords"], 19);
        assert_eq!(result["search"]["newlyIndexedRecords"], 0);
        assert!(result.get("persona").is_none());
    }
    assert_eq!(encoder.passages.load(Ordering::SeqCst), before);
    // Failures are recorded explicitly and do not block other records.
    encoder.fail.store(true, Ordering::SeqCst);
    f.revise(&f.chatgpt, &revised, "Another revision", "active");
    let report = index_with(&f.store, &encoder, 10).unwrap();
    assert_eq!(report.failed, 1);
    let status = index_status(&f.store, Some(&f.codex), &profile).unwrap();
    assert_eq!(status["failedRecords"], 1);
    assert_eq!(status["complete"], false);
    assert_eq!(status["scope"], "authorized_records");
    // Owner repair discards derived data only; canonical records are untouched.
    let canonical = f
        .context(&f.chatgpt, json!({"order":"created","limit":100}))
        .unwrap();
    reset_index(&f.store).unwrap();
    let status = index_status(&f.store, None, &profile).unwrap();
    assert_eq!(status["indexedRecords"], 0);
    assert_eq!(status["pendingRecords"], 19);
    let after = f
        .context(&f.chatgpt, json!({"order":"created","limit":100}))
        .unwrap();
    assert_eq!(ids(&canonical), ids(&after));
}

#[test]
fn missing_local_model_keeps_literal_search_and_reports_the_limitation() {
    let f = Fixture::new();
    f.write(&f.chatgpt, "Lighthouse", "owner_statement");
    let search = NativeSearch::new(f.dir.path().join("missing-local-model"));
    assert_eq!(
        search.index_next(&f.store).unwrap_err().code(),
        "MODEL_UNAVAILABLE"
    );
    let status = search.index_status(&f.store, Some(&f.chatgpt)).unwrap();
    assert_eq!(status["model"]["state"], "unavailable");
    assert!(
        status["limitation"]
            .as_str()
            .unwrap()
            .contains("Literal search")
    );
    assert_eq!(status["localOnly"], true);
    let result = search
        .search(&f.store, &f.chatgpt, json!({"query":"lighthouse"}))
        .unwrap();
    assert_eq!(result["search"]["effectiveMode"], "literal");
    assert_eq!(result["records"].as_array().unwrap().len(), 1);
}

#[test]
fn search_pages_share_one_ranking_and_refuse_stale_cursors() {
    let f = Fixture::new();
    for i in 0..9 {
        f.write(&f.chatgpt, &format!("Harbor walk {i}"), "owner_statement");
    }
    let search = NativeSearch::new(f.dir.path().join("missing-local-model"));
    let args = json!({"query":"harbor","mode":"literal","limit":4,"actor":"synthetic-chatgpt","previewMaxChars":40});
    let mut budgeted = args.clone();
    budgeted["maxResponseBytes"] = json!(4096);
    let first = search
        .search(&f.store, &f.chatgpt, budgeted.clone())
        .unwrap();
    budgeted["cursor"] = first["pagination"]["nextCursor"].clone();
    assert_eq!(
        search
            .search(&f.store, &f.codex, budgeted)
            .unwrap_err()
            .code(),
        "CURSOR_STALE"
    );
    let mut seen = Vec::new();
    let mut next = args.clone();
    loop {
        let page = search.search(&f.store, &f.chatgpt, next.clone()).unwrap();
        assert_eq!(page["pagination"]["matchingRecords"], 9);
        seen.extend(ids(&page));
        if page["pagination"]["hasMore"] == false {
            break;
        }
        next["cursor"] = page["pagination"]["nextCursor"].clone();
    }
    assert_eq!(seen.iter().collect::<HashSet<_>>().len(), 9);
    assert_eq!(seen.len(), 9);
    let page = search.search(&f.store, &f.chatgpt, args.clone()).unwrap();
    f.write(&f.chatgpt, "Harbor walk late", "owner_statement");
    let mut stale = args.clone();
    stale["cursor"] = page["pagination"]["nextCursor"].clone();
    assert_eq!(
        search
            .search(&f.store, &f.chatgpt, stale)
            .unwrap_err()
            .code(),
        "CURSOR_STALE"
    );
    let none = search
        .search(
            &f.store,
            &f.chatgpt,
            json!({"query":"harbor","actor":"synthetic-codex"}),
        )
        .unwrap();
    assert_eq!(none["pagination"]["matchingRecords"], 0);
}
