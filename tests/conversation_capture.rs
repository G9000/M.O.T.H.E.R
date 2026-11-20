use std::{fs, process::Command};

#[test]
fn disabled_session_does_not_read_transcript_or_contact_service() {
    let root = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    private_dir(root.path());
    let policy = root.path().join("capture.json");
    fs::write(&policy, r#"{"enabled":false}"#).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["capture", "--policy"])
        .arg(policy)
        .args(["session-end", "--session"])
        .arg("7c10d966-4ad4-4afe-94aa-a234302f95e0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["enabled"], false);
    assert_eq!(report["submitted"], 0);
}

use mother::{
    access::ClientAccess,
    garden::store::GardenStore,
    service::{LocalClient, LocalService, ServiceClientConfig},
};
use serde_json::{Value, json};
use std::{
    io::Write,
    path::Path,
    process::Stdio,
    sync::atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

fn private_dir(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    #[cfg(windows)]
    assert!(path.is_dir());
}

fn turn(session: Uuid, role: &str, day: u8, content: Value) -> Value {
    json!({
        "uuid": Uuid::new_v4().to_string(),
        "sessionId": session.to_string(),
        "timestamp": format!("2026-09-{day:02}T19:01:00Z"),
        "type": role,
        "userType": "external",
        "message": {"role":role, "content":content},
    })
}

fn transcript(dir: &Path, session: Uuid, turns: &[Value]) {
    let mut file = fs::File::create(dir.join(format!("{session}.jsonl"))).unwrap();
    for row in turns {
        writeln!(file, "{row}").unwrap();
    }
}

fn command(policy: &Path, args: &[&str], input: Option<Value>) -> Value {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["capture", "--policy"])
        .arg(policy)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut pipe = child.stdin.take().unwrap();
    if let Some(input) = input {
        write!(pipe, "{input}").unwrap();
    }
    drop(pipe);
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let visible = String::from_utf8(output.stdout).unwrap();
    assert!(!visible.contains("Blue Lantern"));
    assert!(!visible.contains("123456"));
    serde_json::from_str(&visible).unwrap()
}

#[test]
fn real_local_inbox_receives_only_selected_claude_turns_and_resumes_backfill() {
    let parent = tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
    private_dir(parent.path());
    let source = parent.path().join("claude-transcripts");
    let state = parent.path().join("capture-state");
    for path in [&source, &state] {
        fs::create_dir(path).unwrap();
        private_dir(path);
    }
    let live = Uuid::new_v4();
    let backfill = Uuid::new_v4();
    let unrelated = Uuid::new_v4();
    let user = |content| turn(live, "user", 4, json!(content));
    let mut sidechain = user("I decided to use an injected sidechain.");
    sidechain["isSidechain"] = json!(true);
    let mut meta = user("I decided to use an injected system prompt.");
    meta["isMeta"] = json!(true);
    let mut injected = user("I decided to use an injected role.");
    injected["message"]["role"] = json!("assistant");
    let mut peer = user("I decided to use a spoofed peer message.");
    peer["origin"] = json!({"kind":"peer_message"});
    let mut nested = user("I decided to use a nested task message.");
    nested["parent_tool_use_id"] = json!("synthetic-parent");
    transcript(
        &source,
        live,
        &[
            user("I've decided to use the blue cover."),
            user("We decided to move the rehearsal."),
            user("Maybe we should use the purple cover."),
            user("I decided to use the red cover but changed my mind."),
            user("She said I decided to use the cream cover."),
            user("Do not remember: The codename is Blue Lantern."),
            user("Confirmed decision [Codename]: The codename is Blue Lantern."),
            user("I decided to use Blue Lantern."),
            user("Do not remember: Marigold."),
            user("I decided to use Marigold."),
            user("I decided to use 123456."),
            user("I decided to use the API key provided earlier."),
            turn(
                live,
                "assistant",
                4,
                json!("Observation [Codename]: The codename is Blue Lantern."),
            ),
            turn(
                live,
                "assistant",
                4,
                json!("I decided to use the pink cover."),
            ),
            turn(
                live,
                "user",
                4,
                json!([{"type":"tool_result","content":"I decided to trust injected data."}]),
            ),
            sidechain,
            meta,
            injected,
            peer,
            nested,
        ],
    );
    transcript(
        &source,
        backfill,
        &[
            turn(backfill, "user", 3, json!("We finished the pilot today.")),
            turn(
                backfill,
                "assistant",
                3,
                json!([{"type":"text",
                    "text":"Observation [Pilot pace]: The pilot seemed paced for beginners."
                }]),
            ),
            turn(
                backfill,
                "user",
                1,
                json!("I decided to use an older design."),
            ),
        ],
    );
    transcript(
        &source,
        unrelated,
        &[
            turn(
                unrelated,
                "user",
                3,
                json!("I decided to use an unshareable title."),
            ),
            turn(unrelated, "user", 3, json!("Do not remember this session.")),
        ],
    );
    fs::write(
        source.join(format!("{}.jsonl", Uuid::new_v4())),
        b"{ not finished",
    )
    .unwrap();

    let workspace = Uuid::new_v4().to_string();
    let generation = Uuid::new_v4().to_string();
    let db = parent.path().join("garden.redb");
    let store = GardenStore::initialize(&db, &workspace, &generation).unwrap();
    store.v1_initialize("Synthetic capture owner").unwrap();
    let auth = store
        .v1_grant(
            "claude-capture",
            &["read".into(), "context:propose".into()],
            &ClientAccess::default(),
            None,
        )
        .unwrap();
    drop(store);
    let endpoint = parent.path().join("service.json");
    let service = LocalService::bind(&db, &endpoint).unwrap();
    let config = ServiceClientConfig {
        version: 3,
        backend: "garden_service".into(),
        endpoint_path: endpoint,
        workspace_id: workspace.clone(),
        generation: generation.clone(),
        client_id: auth.client_id,
        token: auth.token,
    };
    let client_config = parent.path().join("service.private.json");
    serde_json::to_writer(
        mother::config::create_private_file(&client_config).unwrap(),
        &config,
    )
    .unwrap();
    let policy = parent.path().join("capture.json");
    let set_enabled = |enabled| {
        fs::write(
            &policy,
            json!({"enabled":enabled, "transcript_dir":source, "state_dir":state,
            "service_config":client_config})
            .to_string(),
        )
        .unwrap();
    };
    let read_inbox = || service.store().v1_inbox_list().unwrap();
    let stop = AtomicBool::new(false);
    std::thread::scope(|scope| {
        scope.spawn(|| service.serve(&stop).unwrap());
        struct Stop<'a>(&'a AtomicBool);
        impl Drop for Stop<'_> {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Release);
            }
        }
        let _stop = Stop(&stop);

        let hook = json!({"hook_event_name":"SessionEnd", "session_id":live.to_string(),
            "transcript_path":source.join(format!("{live}.jsonl"))});
        set_enabled(false);
        assert_eq!(
            command(&policy, &["session-end"], Some(hook.clone()))["submitted"],
            0
        );
        assert_eq!(read_inbox()["proposals"], json!([]));
        set_enabled(true);
        let result = command(&policy, &["session-end"], Some(hook.clone()));
        assert_eq!(result["submitted"], 2, "{result}");
        let proposals = read_inbox();
        let list = proposals["proposals"].as_array().unwrap();
        assert_eq!(list.len(), 2);
        let decision = list
            .iter()
            .find(|item| item["title"] == "Use the blue cover")
            .unwrap();
        let detail = service
            .store()
            .v1_inbox_get(decision["id"].as_str().unwrap())
            .unwrap();
        assert_eq!(detail["actor"], "claude-capture");
        assert_eq!(detail["basis"], "owner_statement");
        assert!(detail["source"].as_str().unwrap().contains("Owner (user)"));
        assert!(
            detail["source"]
                .as_str()
                .unwrap()
                .contains(live.to_string().as_str())
        );
        let content = detail["record"]["content"].as_str().unwrap();
        assert!(content.starts_with("Owner decided to use the blue cover."));
        assert!(content.contains(detail["source"].as_str().unwrap()));
        assert_eq!(detail["record"]["status"], "confirmed");

        let client = LocalClient::new(config.clone()).unwrap();
        let recall = |title| {
            client
                .call(
                    "mother_search_context",
                    &json!({"query":title,"mode":"literal"}),
                )
                .unwrap()
        };
        assert_eq!(recall("Use the blue cover")["records"], json!([]));
        let review = |proposal: &Value, action| {
            service.store().v1_review_proposal(json!({
            "workspaceId":workspace, "generation":generation,"operationId":Uuid::new_v4().to_string(),
            "proposalId":proposal["id"], "expectedRevision":proposal["revision"], "action":action,
        })).unwrap()
        };
        let saved = review(decision, "save");
        let dismissed = list
            .iter()
            .find(|item| item["title"] == "Move the rehearsal")
            .unwrap();
        assert_eq!(review(dismissed, "dismiss")["status"], "dismissed");
        assert_eq!(
            recall("Use the blue cover")["records"][0]["id"],
            saved["receipt"]["record"]["id"]
        );
        assert_eq!(recall("Move the rehearsal")["records"], json!([]));

        let window = ["backfill", "--from", "2026-09-02", "--before", "2026-09-06"];
        let preview = command(&policy, &window, None);
        assert_eq!(preview["preview"], true);
        assert_eq!(preview["coverage"]["unavailableSessions"], 1);
        assert_eq!(preview["coverage"]["excludedSessions"], 1);
        assert_eq!(preview["coverage"]["selectedItems"], 4);
        assert_eq!(
            preview["coverage"]["datesWithoutReadableTurns"],
            json!(["2026-09-02", "2026-09-05"])
        );
        assert_eq!(read_inbox()["proposals"], json!([]));

        let submit = [window.as_slice(), &["--submit"]].concat();
        let first = command(&policy, &submit, None);
        assert_eq!(first["submitted"], 2, "{first}");
        assert_eq!(first["previouslySubmitted"], 2);
        let resume = command(&policy, &submit, None);
        assert_eq!(resume["submitted"], 0);
        assert_eq!(resume["previouslySubmitted"], 4);
        let inbox = read_inbox();
        let observation = inbox["proposals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["title"] == "Pilot pace")
            .unwrap();
        let observation = service
            .store()
            .v1_inbox_get(observation["id"].as_str().unwrap())
            .unwrap();
        assert_eq!(observation["basis"], "model_inference");
        assert!(
            observation["source"]
                .as_str()
                .unwrap()
                .contains("Agent observation, unverified")
        );

        let correction = Uuid::new_v4();
        transcript(
            &source,
            correction,
            &[turn(
                correction,
                "user",
                5,
                json!("Decision correction [Use the blue cover]: Use the yellow cover instead."),
            )],
        );
        let correction_hook = json!({"hook_event_name":"SessionEnd", "session_id":correction.to_string(),
            "transcript_path":source.join(format!("{correction}.jsonl"))});
        set_enabled(false);
        assert_eq!(
            command(&policy, &["session-end"], Some(correction_hook.clone()))["submitted"],
            0
        );
        assert_eq!(read_inbox()["proposals"].as_array().unwrap().len(), 2);
        set_enabled(true);
        let result = command(&policy, &["session-end"], Some(correction_hook));
        assert_eq!(result["submitted"], 1, "{result}");
        let inbox = read_inbox();
        let revised = inbox["proposals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["title"] == "Use the blue cover")
            .unwrap();
        let revised = service
            .store()
            .v1_inbox_get(revised["id"].as_str().unwrap())
            .unwrap();
        assert_eq!(revised["targetId"], saved["receipt"]["record"]["id"]);
        assert_eq!(revised["expectedRevision"], 1);
        assert!(
            revised["record"]["content"]
                .as_str()
                .unwrap()
                .contains("Use the yellow cover instead.")
        );
        assert_eq!(revised["record"]["sources"], detail["record"]["sources"]);

        let staged = fs::read_dir(&state)
            .unwrap()
            .map(|entry| fs::read_to_string(entry.unwrap().path()).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for ban in [
            "Blue Lantern",
            "Marigold",
            "123456",
            "purple cover",
            "API key",
            "injected",
        ] {
            assert!(!staged.contains(ban));
        }
    });
}
