use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{access::GardenAccess, contracts::*, store::GardenStore},
    service::{LocalService, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

fn run(config: &Path, subcommand: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", subcommand, "--config"])
        .arg(config)
        .args(args)
        .output()
        .unwrap()
}

fn output(result: Output) -> Value {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}

fn existing(kind: &str, id: &str) -> Value {
    json!({"refType":"existing","kind":kind,"id":id})
}

struct Fixture {
    root: tempfile::TempDir,
    db: PathBuf,
    ws: String,
    generation: String,
    writer: Auth,
    viewer: Auth,
    config: PathBuf,
    memory: String,
    mapping: PathBuf,
    ids: std::collections::BTreeMap<String, String>,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        let db = root.path().join("synthetic.redb");
        let ws = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(&db, &ws, &generation).unwrap();
        store.v1_initialize("Synthetic graph owner").unwrap();
        let writer = store
            .v1_grant(
                "writer",
                &[
                    "read".into(),
                    "memory:write".into(),
                    "garden:read".into(),
                    "garden:write".into(),
                ],
                &ClientAccess::default(),
                Some(&GardenAccess::All {}),
            )
            .unwrap();
        let viewer = store
            .v1_grant(
                "link-proposer",
                &["read".into(), "garden:read".into(), "garden:propose".into()],
                &ClientAccess::default(),
                Some(&GardenAccess::All {}),
            )
            .unwrap();
        let memory = store.v1_call(&writer,"mother_write_record",json!({
            "workspaceId":ws,"generation":generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,
            "record":{"kind":"memory","title":"Early lantern memory","content":"Synthetic source.","status":"active","basis":"owner_statement"}
        })).unwrap()["record"]["id"].as_str().unwrap().to_owned();
        let items = [("a","Earlier lantern"),("b","Later lantern"),("blank","Empty lantern"),("h","HIDDEN LANTERN")]
            .into_iter().map(|(key,label)| json!({"itemKey":key,"mutation":"create","expectedRevision":0,
                "draft":{"kind":"entity","entityType":"test:idea","displayName":label,"aliases":[],"extensions":{}}
            })).chain([("s1","Original workshop note"),("s2","Updated workshop note"),("private","REVOKED EXCERPT")]
                .into_iter().map(|(key,text)| json!({"itemKey":key,"mutation":"create","expectedRevision":0,
                    "draft":{"kind":"evidence","source":{"type":"uri","uri":format!("urn:synthetic:{key}")},
                        "excerpt":text,"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}
                 }))).collect::<Vec<_>>();
        let seeded = store
            .apply(&writer, &batch(&ws, &generation, items))
            .unwrap();
        let ids = seeded
            .receipt
            .results
            .into_iter()
            .map(|r| (r.item_key, r.reference.id))
            .collect::<std::collections::BTreeMap<_, _>>();
        let config = root.path().join("graph.private.json");
        write_direct(&config, &db, &viewer);
        let mapping = root.path().join("mapping.private.json");
        let output = mother::config::create_private_file(&mapping).unwrap();
        serde_json::to_writer(output,&json!({"version":1,"workspaceId":ws,"generation":generation,"mappings":[{"memoryId":memory,"entityId":ids["a"]}]})).unwrap();
        Self {
            root,
            db,
            ws,
            generation,
            writer,
            viewer,
            config,
            memory,
            mapping,
            ids,
        }
    }
    fn store(&self) -> GardenStore {
        GardenStore::open_active(&self.db).unwrap()
    }
    fn view(&self) -> Value {
        output(run(
            &self.config,
            "relationships",
            &[
                "--memory",
                &self.memory,
                "--mapping",
                self.mapping.to_str().unwrap(),
            ],
        ))
    }
    fn add(&self, from: &str, to: &str, predicate: &str, evidence: &str) -> String {
        let req = batch(
            &self.ws,
            &self.generation,
            vec![
                json!({"itemKey":"extra","mutation":"create","expectedRevision":0,
                    "draft":{"kind":"assertion","predicate":predicate,"subject":existing("entity",from),
                        "object":{"type":"entityRef","entityRef":existing("entity",to)},
                        "validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},
                        "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
                        "evidence":[existing("evidence",evidence)],"confidence":null,"extensions":{}}
                }),
            ],
        );
        self.store()
            .apply(&self.writer, &req)
            .unwrap()
            .receipt
            .results[0]
            .reference
            .id
            .clone()
    }
    fn review(&self, proposal: &Value, action: &str) -> OutcomeResponse {
        self.store().review_proposal(json!({"version":2,"workspaceId":self.ws,"generation":self.generation,
            "operationId":Uuid::new_v4().to_string(),"proposalId":proposal["proposalId"],"expectedRevision":1,"action":action
        })).unwrap()
    }
}

fn write_direct(path: &Path, db: &Path, auth: &Auth) {
    fs::write(
        path,
        json!({"version":2,"backend":"garden_redb","databasePath":db,
        "clientId":auth.client_id,"token":auth.token})
        .to_string(),
    )
    .unwrap();
}

fn batch(ws: &str, generation: &str, items: Vec<Value>) -> BatchRequest {
    serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,
        "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items}))
    .unwrap()
}

#[test]
fn mapped_relationships_are_reviewed_attributed_and_permission_bounded() {
    let f = Fixture::new();
    let a = &f.ids["a"];
    let b = &f.ids["b"];
    let s1 = &f.ids["s1"];
    let s2 = &f.ids["s2"];
    assert_eq!(
        output(run(&f.config, "relationships", &["--memory", &f.memory]))["status"],
        "mapping_missing"
    );
    assert_eq!(f.view()["status"], "empty");
    let isolated = output(run(
        &f.config,
        "relationships",
        &["--entity", &f.ids["blank"]],
    ));
    assert_eq!(isolated["status"], "empty");
    assert_eq!(isolated["coverage"], "complete_within_neighborhood");
    let op = Uuid::new_v4().to_string();
    let initial_args = [
        "--from",
        b,
        "--to",
        a,
        "--relationship",
        "supersedes",
        "--evidence",
        s1,
        "--operation-id",
        &op,
    ];
    let proposal = output(run(&f.config, "suggest-link", &initial_args));
    assert_eq!(proposal["status"], "admission_recorded");
    assert_eq!(f.view()["status"], "empty");
    let lookup = OutcomeRequest {
        version: 2,
        workspace_id: f.ws.clone(),
        generation: f.generation.clone(),
        operation_id: op.clone(),
        operation_generation: f.generation.clone(),
        required_extensions: Default::default(),
    };
    let store = f.store();
    assert_eq!(
        store
            .get_proposal(&f.viewer, &lookup)
            .unwrap()
            .receipt
            .status,
        OutcomeStatus::Pending
    );
    drop(store);
    let approved = f.review(&proposal, "approve");
    let edge = &approved.receipt.results[0].reference.id;
    let first = f.view();
    assert_eq!(first["memoryTitle"], "Early lantern memory");
    assert_eq!(first["anchor"]["label"], "Earlier lantern");
    assert_eq!(first["edges"].as_array().unwrap().len(), 1);
    let first = &first["edges"][0];
    assert_eq!(first["direction"], "incoming");
    assert_eq!(first["predicate"], "living-memory:supersedes");
    assert_eq!(first["from"]["id"], b.as_str());
    assert_eq!(first["to"]["id"], a.as_str());
    assert_eq!(first["contributor"], "link-proposer");
    assert_eq!(first["approver"], "@owner-review");
    assert_eq!(first["evidence"][0]["excerpt"], "Original workshop note");
    assert_eq!(
        output(run(&f.config, "suggest-link", &initial_args)),
        proposal
    );
    assert_eq!(f.view()["edges"].as_array().unwrap().len(), 1);

    let correction_op = Uuid::new_v4().to_string();
    let correction = output(run(
        &f.config,
        "suggest-link",
        &[
            "--from",
            b,
            "--to",
            a,
            "--relationship",
            "inspired_by",
            "--evidence",
            s2,
            "--operation-id",
            &correction_op,
            "--correct",
            edge,
            "--expected-revision",
            "1",
        ],
    ));
    assert_eq!(f.view()["edges"][0]["revision"], 1);
    f.review(&correction, "approve");
    let current = f.view();
    assert_eq!(current["edges"][0]["id"], edge.as_str());
    assert_eq!(current["edges"][0]["revision"], 2);
    assert_eq!(current["edges"][0]["previousRevision"], 1);
    assert_eq!(current["edges"][0]["contributor"], "link-proposer");
    assert_eq!(current["edges"][0]["evidence"][0]["id"], s2.as_str());
    let store = f.store();
    let history = |id: &str, kind, revision| {
        store
            .get(
                &f.writer,
                &GetRequest {
                    version: 2,
                    workspace_id: f.ws.clone(),
                    generation: f.generation.clone(),
                    required_extensions: Default::default(),
                    reference: ExternalReference {
                        kind,
                        id: id.into(),
                    },
                    revision,
                    known_at: None,
                },
            )
            .unwrap()
            .record
    };
    let earlier = history(edge, GardenKind::Assertion, Some(1));
    assert_eq!(earlier.metadata.contributor, "link-proposer");
    assert_eq!(
        serde_json::to_value(earlier.draft).unwrap()["evidence"][0]["id"],
        s1.as_str()
    );
    for id in [a, b] {
        assert_eq!(
            history(id, GardenKind::Entity, None).metadata.lifecycle,
            Lifecycle::Active
        );
    }
    drop(store);

    let dismissed = output(run(
        &f.config,
        "suggest-link",
        &[
            "--from",
            a,
            "--to",
            b,
            "--relationship",
            "contradicts",
            "--operation-id",
            &Uuid::new_v4().to_string(),
        ],
    ));
    assert_eq!(
        f.review(&dismissed, "dismiss").receipt.status,
        OutcomeStatus::Dismissed
    );
    assert_eq!(f.view()["edges"].as_array().unwrap().len(), 1);

    let cycle = f.add(a, b, "living-memory:evolved_into", s1);
    let hidden = f.add(a, &f.ids["h"], "living-memory:related_to", s1);
    let revoked = f.add(a, b, "living-memory:related_to", &f.ids["private"]);
    assert_eq!(f.view()["edges"].as_array().unwrap().len(), 4);
    let store = f.store();
    let objects = [
        (GardenKind::Entity, a),
        (GardenKind::Entity, b),
        (GardenKind::Evidence, s1),
        (GardenKind::Evidence, s2),
        (GardenKind::Assertion, edge),
        (GardenKind::Assertion, &cycle),
        (GardenKind::Assertion, &hidden),
        (GardenKind::Assertion, &revoked),
    ]
    .into_iter()
    .map(|(kind, id)| ExternalReference {
        kind,
        id: id.clone(),
    })
    .collect();
    let restricted = store
        .v1_grant(
            "restricted",
            &["read".into(), "garden:read".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("test:hidden".into()),
                records: vec![f.memory.clone()],
            },
            Some(&GardenAccess::Allowlist { objects }),
        )
        .unwrap();
    let context = store
        .v1_grant(
            "context-only",
            &["read".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    drop(store);
    let cfg = f.root.path().join("context.private.json");
    write_direct(&cfg, &f.db, &context);
    let unavailable = run(&cfg, "relationships", &["--entity", a]);
    assert!(!unavailable.status.success());
    assert!(String::from_utf8_lossy(&unavailable.stderr).contains("GRAPH_UNAVAILABLE"));
    let cfg = f.root.path().join("restricted.private.json");
    write_direct(&cfg, &f.db, &restricted);
    let direct = output(run(&cfg, "relationships", &["--entity", a]));
    assert_eq!(direct["edges"].as_array().unwrap().len(), 2);
    assert_eq!(direct["coverage"], "complete_within_neighborhood");
    let shown = direct.to_string();
    for secret in [&f.ids["h"], &f.ids["private"], &hidden, &revoked] {
        assert!(!shown.contains(secret));
    }
    for secret in [
        "HIDDEN LANTERN",
        "REVOKED EXCERPT",
        "canonicalWatermark",
        "visibilityEpoch",
    ] {
        assert!(!shown.contains(secret));
    }
    let endpoint = f.root.path().join("owner.json");
    let service = LocalService::bind(&f.db, &endpoint).unwrap();
    let bridge = f.root.path().join("bridge.private.json");
    fs::write(
        &bridge,
        serde_json::to_vec(&ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: endpoint,
            workspace_id: f.ws.clone(),
            generation: f.generation.clone(),
            client_id: restricted.client_id,
            token: restricted.token,
        })
        .unwrap(),
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
        assert_eq!(
            output(run(&bridge, "relationships", &["--entity", a])),
            direct
        );
    });
    drop(service);
    for _ in 0..25 {
        f.add(&f.ids["blank"], a, "living-memory:related_to", s1);
    }
    let partial = output(run(
        &f.config,
        "relationships",
        &["--entity", &f.ids["blank"]],
    ));
    assert_eq!(partial["coverage"], "partial");
    assert!(partial["edges"].as_array().unwrap().len() <= 24);
}
