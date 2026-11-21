//! Timeline acceptance using dated fictional revisions and the public command.
use chrono::{DateTime, Utc};
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, ExternalReference, OutcomeResponse},
        store::{Clock, GardenStore},
    },
    service::{LocalService, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};
use uuid::Uuid;

const WS: &str = "11111111-1111-4111-8111-111111111111";
const GENERATION: &str = "22222222-2222-4222-8222-222222222222";

#[derive(Debug)]
struct SyntheticClock(Mutex<DateTime<Utc>>);
impl SyntheticClock {
    fn set(&self, value: &str) {
        *self.0.lock().unwrap() = value.parse().unwrap();
    }
}
impl Clock for SyntheticClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

fn grant(store: &GardenStore, name: &str, access: &GardenAccess) -> Auth {
    let all = matches!(access, GardenAccess::All {});
    let mut scopes = vec!["garden:read".into()];
    if all {
        scopes.push("garden:write".into());
    }
    store
        .grant(
            name,
            &scopes,
            &ClientAccess {
                mode: if all {
                    AccessMode::Companion
                } else {
                    AccessMode::Shared
                },
                subject: Some("fictional-test".into()),
                records: vec![],
            },
            access,
        )
        .unwrap()
}

fn config(directory: &Path, db: &Path, auth: &Auth) -> PathBuf {
    let path = directory.join(format!("{}.json", auth.client_id));
    std::fs::write(
        &path,
        json!({"version":2,"backend":"garden_redb","databasePath":db,
        "clientId":auth.client_id,"token":auth.token})
        .to_string(),
    )
    .unwrap();
    path
}

fn apply(store: &GardenStore, auth: &Auth, items: Value) -> OutcomeResponse {
    let request: BatchRequest = serde_json::from_value(json!({
        "version":2,"workspaceId":WS,"generation":GENERATION,
        "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items,
    }))
    .unwrap();
    store.apply(auth, &request).unwrap()
}

fn created(response: &OutcomeResponse, key: &str) -> ExternalReference {
    response
        .receipt
        .results
        .iter()
        .find(|item| item.item_key == key)
        .unwrap()
        .reference
        .clone()
}

fn new_item(key: &str, draft: Value) -> Value {
    json!({"itemKey":key,"mutation":"create","expectedRevision":0,"draft":draft})
}

fn local(kind: &str, key: &str) -> Value {
    json!({"refType":"item","kind":kind,"itemKey":key})
}

fn existing(reference: &ExternalReference) -> Value {
    json!({"refType":"existing","kind":reference.kind,"id":reference.id})
}

fn milestone(subject: Value, source: Value, text: &str, from: &str, until: &str) -> Value {
    json!({"kind":"assertion","subject":subject,"predicate":"example:milestone",
        "object":{"type":"value","value":{"type":"string","value":text}},
        "validTime":{"validFrom":from,"validUntil":until,"sourceTimezone":"UTC","precision":"day"},
        "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
        "evidence":[source],"confidence":{"score":1.0,"basis":"source"},"extensions":{}})
}

fn evidence(excerpt: &str) -> Value {
    json!({"kind":"evidence","source":{"type":"uri","uri":"https://example.invalid/milestone"},
        "excerpt":excerpt,"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}})
}

fn command(config: &Path, subcommand: &str, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", "timeline", subcommand, "--config"])
        .arg(config)
        .args(args)
        .output()
        .unwrap()
}

fn timeline(config: &Path, subcommand: &str, args: &[&str]) -> Value {
    let output = command(config, subcommand, args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn timeline_uses_current_grants_and_real_dated_revisions() {
    let temp_root = std::fs::canonicalize(std::env::temp_dir()).unwrap();
    let directory = tempfile::tempdir_in(temp_root).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let db = directory.path().join("garden.redb");
    let time = Arc::new(SyntheticClock(Mutex::new(
        "2025-03-03T10:00:00Z".parse().unwrap(),
    )));
    let store = GardenStore::initialize_with_clock(&db, WS, GENERATION, time.clone()).unwrap();
    let author = grant(&store, "first-author", &GardenAccess::All {});
    let reviewer = grant(&store, "later-author", &GardenAccess::All {});
    let original = apply(
        &store,
        &author,
        json!([
            new_item(
                "project",
                json!({"kind":"entity","entityType":"example:project","displayName":"Fictional opening","aliases":[],"extensions":{}})
            ),
            new_item("original", evidence("March opening note.")),
            new_item(
                "milestone",
                milestone(
                    local("entity", "project"),
                    local("evidence", "original"),
                    "First opening",
                    "2025-03-01T00:00:00Z",
                    "2025-03-02T00:00:00Z"
                )
            ),
            new_item("concealed", evidence("PRIVATE_EVIDENCE_SENTINEL")),
            new_item(
                "concealed-milestone",
                milestone(
                    local("entity", "project"),
                    local("evidence", "concealed"),
                    "PRIVATE_MILESTONE_SENTINEL",
                    "2025-03-01T00:00:00Z",
                    "2025-03-02T00:00:00Z"
                )
            ),
        ]),
    );
    let claim = created(&original, "milestone");
    let project = created(&original, "project");
    let original_source = created(&original, "original");
    let hidden_source = created(&original, "concealed");
    let hidden_claim = created(&original, "concealed-milestone");
    time.set("2025-03-10T12:00:00Z");
    let corrected = apply(
        &store,
        &reviewer,
        json!([
            new_item("corrected", evidence("February opening correction.")),
            {"itemKey":"milestone","mutation":"update","expectedRevision":1,"id":claim,
                "draft":milestone(existing(&project),local("evidence","corrected"),
                    "Corrected opening","2025-02-28T00:00:00Z","2025-03-01T00:00:00Z")},
        ]),
    );
    let corrected_source = created(&corrected, "corrected");
    let reader = grant(
        &store,
        "narrow-reader",
        &GardenAccess::Allowlist {
            objects: vec![
                project,
                claim.clone(),
                original_source,
                corrected_source,
                hidden_claim.clone(),
            ],
        },
    );
    let config = config(directory.path(), &db, &reader);
    let original_audit = store.audit(&author).unwrap();
    drop(store);

    let request = |known| {
        timeline(
            &config,
            "list",
            &[
                "--known-on",
                known,
                "--kind",
                "assertion",
                "--from",
                "2025-02-28",
                "--through",
                "2025-03-01",
            ],
        )
    };
    let original = request("2025-03-05");
    let corrected = request("2025-03-11");
    assert_eq!(original["entries"].as_array().unwrap().len(), 1);
    assert_eq!(corrected["entries"].as_array().unwrap().len(), 1);
    let before = &original["entries"][0];
    let after = &corrected["entries"][0];
    assert_eq!(before["eventTime"]["from"], "2025-03-01T00:00:00Z");
    assert_eq!(after["eventTime"]["from"], "2025-02-28T00:00:00Z");
    assert_eq!(before["eventTime"]["precision"], "day");
    assert!(
        before["recordedAt"]
            .as_str()
            .unwrap()
            .starts_with("2025-03-03")
    );
    assert!(
        after["recordedAt"]
            .as_str()
            .unwrap()
            .starts_with("2025-03-10")
    );
    assert!(
        before["evidence"][0]["recordedAt"]
            .as_str()
            .unwrap()
            .starts_with("2025-03-03")
    );
    assert!(
        after["evidence"][0]["recordedAt"]
            .as_str()
            .unwrap()
            .starts_with("2025-03-10")
    );
    assert_eq!(before["actor"], "first-author");
    assert_eq!(after["actor"], "later-author");
    let compare = timeline(
        &config,
        "compare",
        &[
            "--earlier",
            "2025-03-05",
            "--later",
            "2025-03-11",
            "--kind",
            "assertion",
        ],
    );
    assert_eq!(compare["changes"].as_array().unwrap().len(), 1);
    assert_eq!(compare["changes"][0]["change"], "corrected");
    assert_eq!(compare["changes"][0]["before"]["revision"], 1);
    assert_eq!(compare["changes"][0]["after"]["revision"], 2);
    for response in [&original, &corrected, &compare] {
        let text = response.to_string();
        for forbidden in [
            "PRIVATE_EVIDENCE_SENTINEL",
            "PRIVATE_MILESTONE_SENTINEL",
            &hidden_source.id,
            &hidden_claim.id,
        ] {
            assert!(!text.contains(forbidden));
        }
    }
    let inspected = timeline(
        &config,
        "inspect",
        &["--kind", "assertion", "--id", &claim.id, "--revision", "1"],
    );
    assert_eq!(inspected["record"]["metadata"]["revision"], 1);
    assert_eq!(
        inspected["entry"]["evidence"][0]["excerpt"],
        "March opening note."
    );
    let unknown = timeline(
        &config,
        "list",
        &[
            "--kind",
            "entity",
            "--from",
            "2025-02-28",
            "--through",
            "2025-03-01",
        ],
    );
    assert_eq!(unknown["entries"][0]["eventTime"]["status"], "unknown");
    let page = timeline(&config, "list", &["--limit", "1"]);
    assert_eq!(page["coverage"]["reason"], "more_pages");
    let cursor = page["nextCursor"].as_str().unwrap();
    let next = timeline(&config, "list", &["--limit", "1", "--cursor", cursor]);
    assert_ne!(page["entries"][0]["id"], next["entries"][0]["id"]);
    let endpoint = directory.path().join("owner.json");
    let service = LocalService::bind(&db, &endpoint).unwrap();
    let service_config = directory.path().join("timeline-service.json");
    serde_json::to_writer(
        std::fs::File::create(&service_config).unwrap(),
        &ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: endpoint,
            workspace_id: WS.into(),
            generation: GENERATION.into(),
            client_id: reader.client_id,
            token: reader.token,
        },
    )
    .unwrap();
    let stop = AtomicBool::new(false);
    std::thread::scope(|scope| {
        struct Stop<'a>(&'a AtomicBool);
        impl Drop for Stop<'_> {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Release);
            }
        }
        scope.spawn(|| service.serve(&stop).unwrap());
        let _stop = Stop(&stop);
        let remote = timeline(
            &service_config,
            "list",
            &[
                "--known-on",
                "2025-03-11",
                "--kind",
                "assertion",
                "--from",
                "2025-02-28",
                "--through",
                "2025-03-01",
            ],
        );
        assert_eq!(remote, corrected);
        service.store().revoke("narrow-reader").unwrap();
        let denied = command(
            &service_config,
            "list",
            &["--limit", "1", "--cursor", cursor],
        );
        assert!(!denied.status.success());
        assert!(denied.stdout.is_empty());
    });
    drop(service);
    let store = GardenStore::open(&db).unwrap();
    assert_eq!(store.audit(&author).unwrap(), original_audit);
    drop(store);
    let denied = command(&config, "list", &["--limit", "1", "--cursor", cursor]);
    assert!(!denied.status.success());
    assert!(denied.stdout.is_empty());
    assert!(String::from_utf8_lossy(&denied.stderr).contains("UNAUTHORIZED"));
}

#[test]
fn comparison_distinguishes_new_and_superseded_from_inaccessible_old_history() {
    let directory = tempfile::tempdir().unwrap();
    let db = directory.path().join("garden.redb");
    let time = Arc::new(SyntheticClock(Mutex::new(
        "2025-03-03T10:00:00Z".parse().unwrap(),
    )));
    let store = GardenStore::initialize_with_clock(&db, WS, GENERATION, time.clone()).unwrap();
    let writer = grant(&store, "fixture-writer", &GardenAccess::All {});
    let original = apply(
        &store,
        &writer,
        json!([
            new_item(
                "project",
                json!({"kind":"entity","entityType":"example:project",
            "displayName":"Fictional neighborhood","aliases":[],"extensions":{}})
            ),
            new_item("old-hidden-source", evidence("HIDDEN_HISTORY_SENTINEL")),
            new_item("allowed-source", evidence("Authorized source.")),
            new_item(
                "later-visible",
                milestone(
                    local("entity", "project"),
                    local("evidence", "old-hidden-source"),
                    "HIDDEN_OLD_REVISION",
                    "2025-03-01T00:00:00Z",
                    "2025-03-02T00:00:00Z"
                )
            ),
            new_item(
                "retired",
                milestone(
                    local("entity", "project"),
                    local("evidence", "allowed-source"),
                    "Earlier claim",
                    "2025-03-01T00:00:00Z",
                    "2025-03-02T00:00:00Z"
                )
            ),
        ]),
    );
    let project = created(&original, "project");
    let source = created(&original, "allowed-source");
    let hidden_source = created(&original, "old-hidden-source");
    let visible_later = created(&original, "later-visible");
    let retired = created(&original, "retired");
    let event = |label| {
        milestone(
            existing(&project),
            existing(&source),
            label,
            "2025-03-01T00:00:00Z",
            "2025-03-02T00:00:00Z",
        )
    };
    time.set("2025-03-07T10:00:00Z");
    let later = apply(
        &store,
        &writer,
        json!([
            new_item("added", event("Claim first entered during the window")),
            {"itemKey":"retired","expectedRevision":1,"mutation":"supersede","id":retired,
                "replacement":local("assertion","successor")},
            new_item("successor", event("Replacement claim")),
        ]),
    );
    let added = created(&later, "added");
    let successor = created(&later, "successor");
    time.set("2025-03-10T10:00:00Z");
    let mut dated = event("Ongoing approximate milestone");
    dated["predicate"] = json!("example:approximate");
    dated["validTime"] = json!({"validFrom":"2025-03-01T00:00:00Z","validUntil":null,
        "sourceTimezone":"UTC","precision":"month"});
    let last = apply(
        &store,
        &writer,
        json!([
            {"itemKey":"visible","mutation":"update","id":visible_later,"expectedRevision":1,
                "draft":event("This revision is visible")},
            {"itemKey":"added","mutation":"update","id":added,"expectedRevision":1,
                "draft":event("New claim edited later the same week")},
            new_item("approximate", dated),
        ]),
    );
    let reader = grant(
        &store,
        "fixture-reader",
        &GardenAccess::Allowlist {
            objects: vec![
                project.clone(),
                source.clone(),
                visible_later.clone(),
                retired.clone(),
                added.clone(),
                successor.clone(),
                created(&last, "approximate"),
            ],
        },
    );
    let config = config(directory.path(), &db, &reader);
    drop(store);
    let old = command(
        &config,
        "inspect",
        &[
            "--kind",
            "assertion",
            "--id",
            &visible_later.id,
            "--revision",
            "1",
        ],
    );
    assert!(!old.status.success());
    assert!(old.stdout.is_empty());
    assert!(String::from_utf8_lossy(&old.stderr).contains("NOT_FOUND"));
    let visible = timeline(
        &config,
        "inspect",
        &[
            "--kind",
            "assertion",
            "--id",
            &visible_later.id,
            "--known-on",
            "2025-03-11",
        ],
    );
    assert_eq!(
        visible["entry"]["label"],
        "example:milestone · This revision is visible"
    );
    let comparison = timeline(
        &config,
        "compare",
        &[
            "--earlier",
            "2025-03-05",
            "--later",
            "2025-03-11",
            "--predicate",
            "example:milestone",
        ],
    );
    let changes = comparison["changes"].as_array().unwrap();
    assert_eq!(changes.len(), 3);
    let find = |id: &str| {
        changes
            .iter()
            .find(|change| change["after"]["id"] == id)
            .unwrap()
    };
    assert_eq!(find(&added.id)["change"], "added");
    assert_eq!(find(&added.id)["after"]["revision"], 2);
    assert_eq!(find(&successor.id)["change"], "added");
    assert_eq!(find(&retired.id)["change"], "superseded");
    assert_eq!(find(&retired.id)["before"]["revision"], 1);
    assert_eq!(find(&retired.id)["after"]["revision"], 2);
    let text = comparison.to_string();
    for forbidden in [
        "HIDDEN_HISTORY_SENTINEL",
        "HIDDEN_OLD_REVISION",
        &visible_later.id,
        &hidden_source.id,
    ] {
        assert!(!text.contains(forbidden));
    }
    let range = |date| {
        timeline(
            &config,
            "list",
            &[
                "--predicate",
                "example:approximate",
                "--from",
                date,
                "--through",
                date,
            ],
        )
    };
    assert!(
        range("2025-02-28")["entries"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let open = range("2025-05-01");
    assert_eq!(open["entries"][0]["eventTime"]["status"], "approximate");
    assert_eq!(
        open["entries"][0]["eventTime"]["untilExclusive"],
        Value::Null
    );
}
