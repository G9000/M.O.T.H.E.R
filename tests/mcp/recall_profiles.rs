use super::*;

use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{config::GardenClientConfig, store::GardenStore},
    recall_profiles::RecallProfiles,
    service::{LocalService, ServiceClientConfig},
};
use std::collections::BTreeSet;
use std::{
    fs,
    sync::atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

/// Exercise the real MCP envelope: its text copy can more than double the structured JSON.
fn traverse_with_budget(
    client: &mut Client,
    tool: &str,
    budget: usize,
    expected: &BTreeSet<String>,
    decision_id: &str,
) {
    let catalog = client.rpc("tools/list", json!({}));
    let properties = &catalog["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == tool)
        .unwrap()["inputSchema"]["properties"];
    assert_eq!(properties["maxResponseBytes"]["minimum"], 4096);
    assert_eq!(properties["maxResponseBytes"]["maximum"], 12288);
    assert_eq!(properties["cursor"]["type"], "string");

    let mut args = json!({"query":"Synthetic recall fixture", "limit":7,
        "maxResponseBytes":budget, "previewMaxChars":2000});
    if tool == "mother_get_context" {
        args["profile"] = json!("all");
    } else {
        args["mode"] = json!("literal");
    }
    let mut seen = BTreeSet::new();
    for page_number in 1..=expected.len() {
        let reply = client.result(tool, args.clone());
        assert_ne!(reply["isError"], true, "{tool} page {page_number}: {reply}");
        let size = serde_json::to_vec(&reply).unwrap().len();
        assert!(
            size <= budget,
            "{tool} returned {size} bytes for a {budget} byte budget"
        );
        let page = result_body(&reply);
        assert_eq!(page["responseBudget"]["maxBytes"], budget);
        if tool == "mother_get_context" {
            if page["access"]["mode"] == "shared" {
                assert!(page.get("persona").is_none());
            } else {
                assert!(page["persona"]["omitted"] == true || page["persona"]["truncated"] == true);
            }
            let pins = page["pinned"].as_array().unwrap();
            assert_eq!(page["pinnedSummary"]["shown"], pins.len());
            let total = page["pinnedSummary"]["total"].as_u64().unwrap();
            assert_eq!(
                page["pinnedSummary"]["omittedForBudget"]
                    .as_u64()
                    .unwrap_or(0),
                total - pins.len() as u64
            );
        }
        let results = page["records"].as_array().unwrap();
        assert!(
            !results.is_empty(),
            "{tool} returned an empty matching page"
        );
        for preview in results {
            let id = preview["id"].as_str().unwrap().to_owned();
            assert!(
                expected.contains(&id),
                "unauthorized or unrelated preview {id}"
            );
            assert!(!preview["title"].as_str().unwrap().is_empty());
            assert!(!preview["excerpt"].as_str().unwrap().is_empty());
            let writer = if id == decision_id {
                "decision-writer"
            } else {
                "harness-a"
            };
            assert_eq!(preview["actor"], writer);
            assert!(seen.insert(id), "duplicate preview on page {page_number}");
        }
        if page["pagination"]["hasMore"] == false {
            assert_eq!(&seen, expected, "{tool} skipped available records");
            return;
        }
        assert_eq!(page["pagination"]["hasMore"], true);
        args["cursor"] = json!(page["pagination"]["nextCursor"].as_str().unwrap());
    }
    panic!("{tool} did not terminate pagination");
}

#[test]
fn explicit_wire_budget_keeps_project_decision_and_support_resolvable_across_clients() {
    let fixture = RecallFixture::new(&"Synthetic persona with quoted text. ".repeat(250), 12);
    let store = GardenStore::open_active(&fixture.fixture.database).unwrap();
    let owner = mother::garden::config::load_garden_config(&fixture.fixture.a_config)
        .unwrap()
        .auth();
    let additional = store
        .v1_call(&owner, "mother_get_context", json!({"query":title(12)}))
        .unwrap();
    let last = additional["records"][0]["id"].as_str().unwrap().to_owned();
    let persona = &additional["persona"];
    let supports = &fixture.ids[..2];
    let writer = store
        .v1_grant(
            "decision-writer",
            &["read".into(), "decision:write".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    let text = "Use the two cited rehearsal notes as the release checklist.";
    let decision = store.v1_call(&writer,"mother_write_record", json!({
        "workspaceId":fixture.workspace["id"],"generation":fixture.workspace["generation"],
        "expectedRevision":0,"operationId":Uuid::new_v4().to_string(),
        "record":{"kind":"decision","status":"confirmed","title":"Synthetic recall fixture decision",
            "content":text,"references":supports}
    })).unwrap()["record"].clone();
    let id = decision["id"].as_str().unwrap();
    let mut authorized: BTreeSet<String> = fixture.ids.iter().cloned().collect();
    authorized.extend([last, id.to_owned()]);
    let mut shared_ids = fixture.ids[..8].to_vec();
    shared_ids.push(id.to_owned());
    let guest = store
        .v1_grant(
            "recall-guest",
            &["read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("synthetic-guest".into()),
                records: shared_ids.clone(),
            },
            None,
        )
        .unwrap();
    drop(store);
    let verify = |client: &mut Client, budget: usize, expected: &BTreeSet<String>| {
        for tool in ["mother_get_context", "mother_search_context"] {
            traverse_with_budget(client, tool, budget, expected, id);
        }
        let original = client.call("mother_get_record", json!({"id":id}));
        assert_eq!(original["content"], text);
        assert_eq!(original["actor"], "decision-writer");
        assert_eq!(original["references"], json!(supports));
        for (index, support) in supports.iter().enumerate() {
            let evidence = client.call("mother_get_record", json!({"id":support}));
            assert_eq!(evidence["content"], full_content(index));
            assert_eq!(evidence["actor"], "harness-a");
            if index == 0 {
                assert_eq!(
                    read_original_pages(client, json!({"source":"record","id":support})),
                    evidence
                );
            }
        }
    };
    {
        let mut direct = fixture.direct(owner.clone());
        verify(&mut direct, 4096, &authorized);
        assert_eq!(
            read_original_pages(&mut direct, json!({"source":"persona"})),
            *persona
        );
    }
    fixture.service(|service| {
        let mut companion = fixture.bridge(service, owner);
        let mut shared = fixture.bridge(service, guest);
        verify(&mut companion, 12288, &authorized);
        assert_eq!(
            read_original_pages(&mut companion, json!({"source":"persona"})),
            *persona
        );
        verify(&mut shared, 4096, &shared_ids.into_iter().collect());
        let denied = shared.result(
            "mother_read_original",
            json!({"source":"persona","maxResponseBytes":4096}),
        );
        assert_eq!(denied["structuredContent"]["error"]["code"], "FORBIDDEN");
        assert_eq!(
            shared.result("mother_get_record", json!({"id":fixture.ids[11]}))["isError"],
            true
        );
    });
}

fn read_original_pages(client: &mut Client, mut args: Value) -> Value {
    args["maxResponseBytes"] = json!(4096);
    let mut serialized = String::new();
    let mut cursors = BTreeSet::new();
    loop {
        let reply = client.result("mother_read_original", args.clone());
        assert_ne!(reply["isError"], true, "{reply}");
        assert!(serde_json::to_vec(&reply).unwrap().len() <= 4096);
        let page = result_body(&reply);
        assert_eq!(page["startByte"], serialized.len());
        serialized.push_str(page["fragment"].as_str().unwrap());
        assert_eq!(page["endByte"], serialized.len());
        if page["nextCursor"].is_null() {
            assert_eq!(page["totalBytes"], serialized.len());
            assert!(!cursors.is_empty(), "fixture should span several pages");
            return serde_json::from_str(&serialized).unwrap();
        }
        let cursor = page["nextCursor"].as_str().unwrap();
        assert!(
            cursors.insert(cursor.to_owned()),
            "repeated original cursor"
        );
        args["cursor"] = json!(cursor);
    }
}

fn title(index: usize) -> String {
    format!("Synthetic recall fixture {index:02} 記憶")
}

fn full_content(index: usize) -> String {
    format!(
        "hidden-proof-{index:02}: {}",
        "Escaped 記憶🪽 \\ \" \n ".repeat(255).trim()
    )
}

struct RecallFixture {
    fixture: Fixture,
    policy: std::path::PathBuf,
    workspace: Value,
    ids: Vec<String>,
}

impl RecallFixture {
    fn new(persona: &str, pins: usize) -> Self {
        let fixture = Fixture::new(persona);
        let config = mother::garden::config::load_garden_config(&fixture.a_config).unwrap();
        let auth = config.auth();
        let store = GardenStore::open_active(&fixture.database).unwrap();
        let context = store
            .v1_call(&auth, "mother_get_context", json!({}))
            .unwrap();
        let workspace = context["workspace"].clone();
        let mut ids = Vec::new();
        for index in 0..=pins {
            let reply = store.v1_call(&auth, "mother_write_record", json!({
                "workspaceId":workspace["id"], "generation":workspace["generation"], "expectedRevision":0,
                "operationId":Uuid::new_v4().to_string(),
                "record":{"kind":"memory","title":title(index),"content":full_content(index),
                    "status":"active","basis":"owner_statement"}
            })).unwrap();
            let id = reply["record"]["id"].as_str().unwrap().to_owned();
            if index < pins {
                store
                    .v1_call(&auth, "mother_pin_record", json!({"id":id,"pinned":true}))
                    .unwrap();
                ids.push(id);
            }
        }
        let policy = fixture
            .dir
            .path()
            .canonicalize()
            .unwrap()
            .join("recall.private.json");
        let mut profiles = vec![
            json!({"name":"all","pins":ids}),
            json!({"name":"empty","pins":[]}),
        ];
        if pins >= 3 {
            profiles.push(json!({"name":"work","pins":[ids[0],ids[1]]}));
            profiles.push(json!({"name":"personal","pins":[ids[0],ids[2]]}));
        }
        let file = mother::config::create_private_file(&policy).unwrap();
        serde_json::to_writer(
            file,
            &json!({"version":1,"workspaceId":workspace["id"],"profiles":profiles}),
        )
        .unwrap();
        Self {
            fixture,
            policy,
            workspace,
            ids,
        }
    }

    fn direct(&self, auth: Auth) -> Client {
        let config = GardenClientConfig {
            version: 2,
            backend: "garden_redb".into(),
            database_path: self.fixture.database.clone(),
            client_id: auth.client_id.clone(),
            token: auth.token,
        };
        let path = self
            .fixture
            .dir
            .path()
            .join(format!("{}.direct.json", auth.client_id));
        fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
        Client::connect_with_options(
            &path,
            &self.fixture.dir.path().join("missing-model"),
            Some(&self.policy),
        )
    }

    fn service(&self, run: impl FnOnce(&LocalService)) {
        let endpoint = self
            .fixture
            .dir
            .path()
            .canonicalize()
            .unwrap()
            .join("owner.json");
        let policy = RecallProfiles::load(&self.policy).unwrap();
        let service =
            LocalService::bind_with_recall_profiles(&self.fixture.database, &endpoint, policy)
                .unwrap();
        let stopped = AtomicBool::new(false);
        std::thread::scope(|scope| {
            struct Stop<'a>(&'a AtomicBool);
            impl Drop for Stop<'_> {
                fn drop(&mut self) {
                    self.0.store(true, Ordering::Release);
                }
            }
            scope.spawn(|| service.serve(&stopped).unwrap());
            let _stop = Stop(&stopped);
            run(&service);
        });
    }

    fn bridge(&self, service: &LocalService, auth: Auth) -> Client {
        let config = ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: self.policy.with_file_name("owner.json"),
            workspace_id: service.info().workspace_id.clone(),
            generation: service.info().generation.clone(),
            client_id: auth.client_id.clone(),
            token: auth.token,
        };
        let path = self
            .fixture
            .dir
            .path()
            .join(format!("{}.bridge.json", auth.client_id));
        fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
        Client::connect(&path)
    }
}

fn result_body(result: &Value) -> &Value {
    assert!(serde_json::to_vec(result).unwrap().len() <= 48 * 1024);
    let parsed: Value =
        serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(parsed, result["structuredContent"]);
    assert!(serde_json::to_vec(&parsed).unwrap().len() <= 12 * 1024);
    &result["structuredContent"]
}

fn read_context(client: &mut Client, args: Value) -> Value {
    let result = client.result("mother_get_context", args);
    assert_ne!(result["isError"], true, "{result}");
    result_body(&result).clone()
}

fn pin_ids(value: &Value) -> Vec<&str> {
    value["pinned"]
        .as_array()
        .unwrap()
        .iter()
        .map(|pin| pin["id"].as_str().unwrap())
        .collect()
}

fn assert_profile(
    fixture: &RecallFixture,
    client: &mut Client,
    name: &str,
    included: &[usize],
    unpinned: usize,
) {
    let value = read_context(client, json!({"profile":name,"limit":100}));
    if value["access"]["mode"] == "shared" {
        assert!(value.get("persona").is_none());
        assert!(value.get("harnesses").is_none());
    }
    let mut actual_ids = pin_ids(&value);
    actual_ids.sort_unstable();
    let mut expected_ids = included
        .iter()
        .map(|index| fixture.ids[*index].as_str())
        .collect::<Vec<_>>();
    expected_ids.sort_unstable();
    assert_eq!(actual_ids, expected_ids);
    assert_eq!(value["workspace"], fixture.workspace);
    assert_eq!(value["selection"]["profile"], name);
    assert_eq!(value["selection"]["scope"], "profiled_granted_records");
    assert_eq!(
        value["inventory"]["totalRecords"],
        included.len() + unpinned
    );
    assert_eq!(
        value["inventory"]["byKind"]["memory"],
        included.len() + unpinned
    );
    assert_eq!(
        value["selection"]["matchingRecords"],
        included.len() + unpinned
    );
    assert_eq!(value["pinnedSummary"]["total"], included.len());
    assert_eq!(value["pinnedSummary"]["inactive"], 0);
    assert!(
        value["unsettledWorkSummary"]["scope"]
            .as_str()
            .unwrap()
            .contains("current grant and recall profile")
    );
    for pin in value["pinned"].as_array().unwrap() {
        assert_eq!(pin["pinnedBy"], "harness-a");
        assert!(pin["pinnedAt"].is_string());
        assert_eq!(pin["truncated"], true);
    }
    let serialized = value.to_string();
    for (index, id) in fixture.ids.iter().enumerate() {
        if !included.contains(&index) {
            for forbidden in [
                id.as_str(),
                &title(index),
                &format!("hidden-proof-{index:02}"),
            ] {
                assert!(
                    !serialized.contains(forbidden),
                    "profile leaked pin {index}"
                );
            }
        }
    }
    if name == "all" && included.len() == 12 {
        assert_eq!(value["responseBudget"]["pinnedContentTruncated"], true);
    }
}

fn rotate_grant(store: &GardenStore, ids: &[String], restricted: bool) -> Auth {
    let access = if restricted {
        ClientAccess {
            mode: AccessMode::Shared,
            subject: Some("synthetic-subject".into()),
            records: vec![ids[0].clone(), ids[2].clone()],
        }
    } else {
        ClientAccess::default()
    };
    store
        .v1_grant("profile-reader", &["read".into()], &access, None)
        .unwrap()
}

fn profiles_before_grant_change(fixture: &RecallFixture, client: &mut Client) {
    let legacy = read_context(client, json!({"limit":100}));
    assert_eq!(legacy["pinnedSummary"]["total"], 12);
    assert_eq!(legacy["inventory"]["totalRecords"], 13);
    assert!(legacy["selection"].get("profile").is_none());
    assert_profile(fixture, client, "work", &[0, 1], 1);
    assert_profile(fixture, client, "personal", &[0, 2], 1);
    assert_profile(fixture, client, "all", &(0..12).collect::<Vec<_>>(), 1);
    assert_profile(fixture, client, "empty", &[], 1);
    let hidden = read_context(client, json!({"profile":"work","query":"hidden-proof-11"}));
    assert_eq!(hidden["records"], json!([]));
    assert_eq!(hidden["selection"]["matchingRecords"], 0);
    assert_eq!(hidden["selection"]["omittedRecords"], 0);
    assert_eq!(hidden["pinnedSummary"]["shown"], 2);
    let unknown = client.result("mother_get_context", json!({"profile":"unknown-profile"}));
    assert_eq!(unknown["isError"], true);
    assert_eq!(
        result_body(&unknown)["error"]["code"],
        "RECALL_PROFILE_UNAVAILABLE"
    );
    assert!(result_body(&unknown).get("pinned").is_none());
    let record = client.call("mother_get_record", json!({"id":fixture.ids[0]}));
    assert_eq!(record["content"], full_content(0));
}

fn profiles_after_grant_change(fixture: &RecallFixture, client: &mut Client) {
    assert_profile(fixture, client, "work", &[0], 0);
    assert_profile(fixture, client, "personal", &[0, 2], 0);
    let unauthorized = client.result("mother_get_record", json!({"id":fixture.ids[1]}));
    assert_eq!(unauthorized["isError"], true);
    let record = client.call("mother_get_record", json!({"id":fixture.ids[0]}));
    assert_eq!(record["content"], full_content(0));
}

#[test]
fn profile_scope_and_budgets_match_on_direct_and_owner_bridge_even_after_grant_change() {
    let fixture = RecallFixture::new(&"Synthetic persona. ".repeat(280), 12);
    let store = GardenStore::open_active(&fixture.fixture.database).unwrap();
    let auth = rotate_grant(&store, &fixture.ids, false);
    drop(store);
    let mut direct = fixture.direct(auth);
    profiles_before_grant_change(&fixture, &mut direct);
    let store = GardenStore::open_active(&fixture.fixture.database).unwrap();
    let auth = rotate_grant(&store, &fixture.ids, true);
    drop(store);
    let old = direct.result("mother_get_context", json!({"profile":"work"}));
    assert_eq!(old["isError"], true);
    drop(direct);
    profiles_after_grant_change(&fixture, &mut fixture.direct(auth));

    fixture.service(|service| {
        let auth = rotate_grant(service.store(), &fixture.ids, false);
        let mut bridge = fixture.bridge(service, auth);
        profiles_before_grant_change(&fixture, &mut bridge);
        let auth = rotate_grant(service.store(), &fixture.ids, true);
        let old = bridge.result("mother_get_context", json!({"profile":"work"}));
        assert_eq!(old["isError"], true);
        drop(bridge);
        profiles_after_grant_change(&fixture, &mut fixture.bridge(service, auth));
    });
}

#[test]
fn profile_marks_an_oversized_persona_omitted_instead_of_emptying_records() {
    let fixture = RecallFixture::new(&"Escaped persona 記憶🪽 \\ \" \n ".repeat(570), 1);
    let store = GardenStore::open_active(&fixture.fixture.database).unwrap();
    let auth = rotate_grant(&store, &fixture.ids, false);
    drop(store);
    // A persona larger than the whole budget is replaced by an explicit marker that says
    // how to read it alone; matching records still come back.
    let assert_marked = |client: &mut Client| {
        let value = client.result("mother_get_context", json!({"profile":"all"}));
        assert_ne!(value["isError"], true);
        let body = result_body(&value);
        assert_eq!(body["persona"]["omitted"], true);
        assert_eq!(body["responseBudget"]["personaOmitted"], true);
        assert_eq!(body["contextSections"]["persona"], "omitted_budget");
        assert!(!body["records"].as_array().unwrap().is_empty());
    };
    assert_marked(&mut fixture.direct(auth.clone()));
    fixture.service(|service| assert_marked(&mut fixture.bridge(service, auth)));
}
