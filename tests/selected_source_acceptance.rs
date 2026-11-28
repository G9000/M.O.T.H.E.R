use mother::{
    access::ClientAccess,
    garden::store::GardenStore,
    service::{LocalClient, LocalService, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

const OWNER_WORDS: &str = "I've DECIDED to keep the café cover...";
const ASSISTANT_WORDS: &str = "Observation [Cover lettering]: The lettering seemed easy to read.";
const UNSELECTED: &str = "The teapot is beside the window.";

struct StopService<'a>(&'a AtomicBool);

impl Drop for StopService<'_> {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

fn capture(policy: &Path, args: &[&str], input: Option<&Value>) -> Value {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["capture", "--policy"])
        .arg(policy)
        .args(args)
        .env("MOTHER_HOME", policy.parent().unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut pipe = child.stdin.take().unwrap();
    if let Some(input) = input {
        serde_json::to_writer(&mut pipe, input).unwrap();
    }
    drop(pipe);
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn a_second_client_can_read_the_selected_words_and_attribution_after_owner_review() {
    let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    let transcripts = root.path().join("transcripts");
    let state = root.path().join("capture-state");
    for path in [&transcripts, &state] {
        fs::create_dir(path).unwrap();
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for path in [root.path(), &transcripts, &state] {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
        }
    }
    let session = Uuid::new_v4();
    let selected = [
        (
            Uuid::new_v4(),
            "user",
            OWNER_WORDS,
            "Keep the café cover",
            "2026-09-04T19:01:00Z",
        ),
        (
            Uuid::new_v4(),
            "assistant",
            ASSISTANT_WORDS,
            "Cover lettering",
            "2026-09-04T19:03:00Z",
        ),
    ];
    let mut file = fs::File::create(transcripts.join(format!("{session}.jsonl"))).unwrap();
    for (turn, role, words, _, timestamp) in selected.into_iter().chain([(
        Uuid::new_v4(),
        "user",
        UNSELECTED,
        "",
        "2026-09-04T19:04:00Z",
    )]) {
        let row = json!({"uuid": turn.to_string(), "sessionId":session.to_string(), "timestamp":timestamp,
            "type":role, "userType":"external", "message":{"role":role,"content":words}});
        writeln!(file, "{row}").unwrap();
    }
    drop(file);

    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let db = root.path().join("garden.redb");
    let endpoint = root.path().join("service.json");
    let store = GardenStore::initialize(&db, &workspace, &generation).unwrap();
    store.v1_initialize("Synthetic source reviewer").unwrap();
    let grant = |actor, scopes: &[String]| {
        let auth = store
            .v1_grant(actor, scopes, &ClientAccess::default(), None)
            .unwrap();
        ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: endpoint.clone(),
            workspace_id: workspace.clone(),
            generation: generation.clone(),
            client_id: auth.client_id,
            token: auth.token,
        }
    };
    let writer_config = root.path().join("capture-client.json");
    let writer = grant(
        "capture-witness",
        &["read".into(), "context:propose".into()],
    );
    serde_json::to_writer(
        mother::config::create_private_file(&writer_config).unwrap(),
        &writer,
    )
    .unwrap();
    let reader_config = grant("independent-reader", &["read".into()]);
    drop(store);
    let service = LocalService::bind(&db, &endpoint).unwrap();
    let policy = root.path().join("capture.json");
    fs::write(
        &policy,
        json!({"enabled":true,"transcript_dir":transcripts,"state_dir":state,
        "service_config":writer_config})
        .to_string(),
    )
    .unwrap();

    let stop = AtomicBool::new(false);
    std::thread::scope(|scope| {
        scope.spawn(|| service.serve(&stop).unwrap());
        let _stop = StopService(&stop);
        let report = capture(
            &policy,
            &["session-end", "--session", &session.to_string()],
            None,
        );
        assert_eq!(report["submitted"], 2, "{report}");
        let selection = json!({
            "operationId": Uuid::new_v4().to_string(), "title":"Lantern launch",
            "originals":[
                {"speaker":"user","author":"River","text":"We said “after dusk.”\nI meant dusk on Friday."},
                {"speaker":"other","author":"Sam","text":"Friday works for me.","spokenAt":"2026-09-04T19:06:00Z"}
            ],
            "sourceRef":"synthetic:meeting-42","eventDate":"2026-09-04",
            "summary":{"author":"Recorder","text":"River and Sam agreed to Friday."},
            "whyItMattered":{"author":"River","text":"The lighting test needs dusk."}
        });
        capture(&policy, &["select"], Some(&selection));
        let reader = LocalClient::new(reader_config).unwrap();
        let search = |query: &str| {
            reader
                .call(
                    "mother_search_context",
                    &json!({"query":query,"mode":"literal"}),
                )
                .unwrap()
        };
        assert_eq!(search("cover")["records"], json!([]));
        assert_eq!(search("Lantern launch")["records"], json!([]));
        let inbox = service.store().v1_inbox_list().unwrap();
        let proposals = inbox["proposals"].as_array().unwrap();
        assert_eq!(proposals.len(), 3);
        let review_and_read = |title: &str| {
            let proposal = proposals.iter().find(|p| p["title"] == title).unwrap();
            let detail = service
                .store()
                .v1_inbox_get(proposal["id"].as_str().unwrap())
                .unwrap();
            assert_eq!(detail["actor"], "capture-witness");
            let approval = service
                .store()
                .v1_review_proposal(json!({"workspaceId": workspace,
                "generation":generation,"operationId":Uuid::new_v4().to_string(),"proposalId":proposal["id"],
                "expectedRevision":proposal["revision"],"action":"save"}))
                .unwrap();
            let found = search(title);
            let rows = found["records"].as_array().unwrap();
            assert_eq!(rows.len(), 1, "{found}");
            assert_eq!(rows[0]["id"], approval["receipt"]["record"]["id"]);
            let record = reader
                .call("mother_get_record", &json!({"id":rows[0]["id"]}))
                .unwrap();
            let page = reader
                .call(
                    "mother_read_original",
                    &json!({
                        "source":"record","id":rows[0]["id"],"maxResponseBytes":12288
                    }),
                )
                .unwrap();
            assert!(
                page["nextCursor"].is_null(),
                "A short synthetic record should fit in one page"
            );
            assert_eq!(
                serde_json::from_str::<Value>(page["fragment"].as_str().unwrap()).unwrap(),
                record
            );
            let content = record["content"].as_str().unwrap();
            assert!(!content.contains(UNSELECTED));
            let source: Value = serde_json::from_str(content).unwrap();
            assert_eq!(source["format"], "mother.selected-conversation.v1");
            chrono::DateTime::parse_from_rfc3339(source["capturedAt"].as_str().unwrap()).unwrap();
            source
        };
        for (turn, role, words, title, timestamp) in selected {
            let source = review_and_read(title);
            assert_eq!(
                source["sourceRef"],
                format!("claude-code://session/{session}/turn/{turn}")
            );
            let originals = source["originals"].as_array().unwrap();
            assert_eq!(originals.len(), 1);
            assert_eq!(originals[0]["speaker"], role);
            assert_eq!(originals[0]["text"], words);
            assert_eq!(
                chrono::DateTime::parse_from_rfc3339(originals[0]["spokenAt"].as_str().unwrap())
                    .unwrap(),
                chrono::DateTime::parse_from_rfc3339(timestamp).unwrap()
            );
            assert!(
                source["summary"]["author"]
                    .as_str()
                    .is_some_and(|s| !s.is_empty())
            );
            assert!(
                source["whyItMattered"]["text"]
                    .as_str()
                    .is_some_and(|s| !s.is_empty())
            );
        }
        let source = review_and_read("Lantern launch");
        for field in [
            "originals",
            "sourceRef",
            "eventDate",
            "summary",
            "whyItMattered",
        ] {
            assert_eq!(source[field], selection[field], "{field}");
        }
        assert_eq!(search("teapot")["records"], json!([]));
    });
}
