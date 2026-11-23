//! The garden explorer's three-lens reader, on synthetic narrative fixtures only.
//!
//! The fixtures cover: an owner account quoting the owner, a captured paraphrase, a reflection
//! approved in the owner inbox, an unreviewed reflection, a retracted reflection, a memory with
//! no reflection, a source summary, a checkpoint, a record longer than the snapshot preview, and
//! a private-sounding record with no media attached.
use mother::{access::ClientAccess, contracts::Auth, garden::store::GardenStore};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::Path,
    process::{Child, Command, Stdio},
};
use uuid::Uuid;

struct Garden {
    store: GardenStore,
    writer: Auth,
    witness: Auth,
    workspace: String,
    generation: String,
}
impl Garden {
    fn new(path: &Path) -> Self {
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(path, &workspace, &generation).unwrap();
        store.v1_initialize("Synthetic owner persona.").unwrap();
        let scopes = ["read", "memory:write", "checkpoint:write"].map(String::from);
        let writer = store
            .v1_grant("synthetic-harness", &scopes, &ClientAccess::default(), None)
            .unwrap();
        let witness = store
            .v1_grant(
                "synthetic-witness",
                &["read", "context:propose"].map(String::from),
                &ClientAccess::default(),
                None,
            )
            .unwrap();
        Self {
            store,
            writer,
            witness,
            workspace,
            generation,
        }
    }
    fn request(&self, record: Value) -> Value {
        json!({"workspaceId":self.workspace,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":record})
    }
    fn write(&self, record: Value) -> Value {
        self.store
            .v1_call(&self.writer, "mother_write_record", self.request(record))
            .unwrap()["record"]
            .clone()
    }
    fn revise(&self, id: &str, revision: u64, record: Value) -> Value {
        let mut request = self.request(record);
        request["id"] = json!(id);
        request["expectedRevision"] = json!(revision);
        self.store
            .v1_call(&self.writer, "mother_write_record", request)
            .unwrap()["record"]
            .clone()
    }
    /// A reflection proposed by the witnessing agent and approved by the owner in the inbox.
    fn approved_reflection(&self, title: &str, content: &str, refs: &[&str]) -> String {
        let mut proposal = self.request(reflection(title, content, refs));
        proposal["source"] = json!("Synthetic Witness Journal letter");
        proposal["basis"] = json!("model_inference");
        let p = self
            .store
            .v1_call(&self.witness, "mother_propose_memory", proposal)
            .unwrap();
        let saved = self
            .store
            .v1_review_proposal(
                json!({"workspaceId":self.workspace,"generation":self.generation,
                "operationId":Uuid::new_v4().to_string(),"proposalId":p["proposal"]["id"],
                "expectedRevision":1,"action":"save"}),
            )
            .unwrap();
        saved["receipt"]["record"]["id"]
            .as_str()
            .unwrap()
            .to_owned()
    }
}
#[test]
fn backfill_cli_round_trip_uses_an_explicit_approval() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("synthetic.redb");
    let garden = Garden::new(&db);
    let record = garden.write(memory(
        "CLI screening",
        "Synthetic clip.",
        "owner_statement",
    ));
    drop(garden);
    let clip = include_bytes!("fixtures/attachment-sample.mp4");
    std::fs::write(dir.path().join("video.mp4"), clip).unwrap();
    let run = |command: &str, args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_mother"))
            .current_dir(dir.path())
            .args([
                "garden",
                "media",
                "backfill",
                command,
                "--db",
                "synthetic.redb",
            ])
            .args(args)
            .output()
            .unwrap()
    };
    let output = run(
        "plan",
        &[
            "--root",
            ".",
            "--file",
            "video.mp4",
            "--memory",
            id(&record),
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reviewed: Value = serde_json::from_slice(&output.stdout).unwrap();
    std::fs::write(dir.path().join("review.json"), output.stdout).unwrap();
    let op = reviewed["operationId"].as_str().unwrap();
    assert!(
        !run("apply", &["--root", ".", "--manifest", "review.json"])
            .status
            .success()
    );
    for (command, args) in [
        (
            "apply",
            vec!["--root", ".", "--manifest", "review.json", "--approve", op],
        ),
        ("verify", vec!["--root", ".", "--manifest", "review.json"]),
        ("revoke", vec!["--manifest", "review.json", "--approve", op]),
    ] {
        let output = run(command, &args);
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert!(
        !run("verify", &["--root", ".", "--manifest", "review.json"])
            .status
            .success()
    );
}

#[test]
fn owner_backfill_binds_reviewed_video_and_replays_after_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("garden.redb");
    let garden = Garden::new(&db);
    let record = garden.write(memory(
        "A synthetic screening",
        "A two-frame test.",
        "owner_statement",
    ));
    let memory_id = id(&record);
    let clip = include_bytes!("fixtures/attachment-sample.mp4");
    let path = dir.path().join("clip.mp4");
    std::fs::write(&path, clip).unwrap();
    let before = garden.store.v1_garden_reader(memory_id).unwrap();
    let plan = garden
        .store
        .plan_attachment(
            dir.path(),
            "clip.mp4",
            memory_id,
            Some("synthetic-source".into()),
            false,
        )
        .unwrap();
    assert_eq!(plan.media_type, "video/mp4");
    assert!(
        garden
            .store
            .plan_attachment(dir.path(), "../clip.mp4", memory_id, None, false)
            .is_err()
    );
    assert!(
        garden.store.media_owner_list(None).unwrap()["assets"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        garden
            .store
            .apply_attachment(dir.path(), &plan, "unreviewed")
            .is_err()
    );
    std::fs::write(&path, b"swapped after review").unwrap();
    assert!(
        garden
            .store
            .apply_attachment(dir.path(), &plan, &plan.operation_id)
            .is_err()
    );
    std::fs::write(&path, clip).unwrap();
    let first = garden
        .store
        .apply_attachment(dir.path(), &plan, &plan.operation_id)
        .unwrap();
    assert_eq!(first["status"], "applied");
    assert_eq!(
        garden.store.verify_attachment(dir.path(), &plan).unwrap()["verified"],
        true
    );
    assert_eq!(before, garden.store.v1_garden_reader(memory_id).unwrap());
    let listed = garden.store.media_owner_list(None).unwrap();
    let asset = &listed["assets"][0];
    assert_eq!(asset["memoryLinks"][0]["memoryId"], memory_id);
    assert_eq!(asset["memoryLinks"][0]["operationId"], plan.operation_id);
    assert_eq!(garden.store.media_owner_bytes(&plan.sha256).unwrap(), clip);
    let bundle = dir.path().join("backup");
    garden.store.backup_to(&bundle).unwrap();
    let restored = GardenStore::restore_backup(&bundle, &dir.path().join("restored.redb")).unwrap();
    assert_eq!(listed, restored.media_owner_list(None).unwrap());
    assert!(
        restored
            .apply_attachment(dir.path(), &plan, &plan.operation_id)
            .is_err()
    );
    drop(garden);
    let reopened = GardenStore::open_active(&db).unwrap();
    let replay = reopened
        .apply_attachment(dir.path(), &plan, &plan.operation_id)
        .unwrap();
    assert_eq!(replay["receipt"], first["receipt"]);
    assert_eq!(replay["status"], "already_applied");
    assert_eq!(listed, reopened.media_owner_list(None).unwrap());
    let revoked = reopened
        .revoke_attachment(&plan, &plan.operation_id)
        .unwrap();
    assert_eq!(revoked["status"], "revoked");
    assert_eq!(
        reopened
            .revoke_attachment(&plan, &plan.operation_id)
            .unwrap()["receipt"],
        revoked["receipt"]
    );
    assert!(
        reopened.media_owner_list(None).unwrap()["assets"][0]["memoryLinks"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(reopened.media_owner_bytes(&plan.sha256).unwrap(), clip);
}

fn memory(title: &str, content: &str, basis: &str) -> Value {
    json!({"kind":"memory","title":title,"content":content,"status":"active","basis":basis})
}
fn reflection(title: &str, content: &str, refs: &[&str]) -> Value {
    json!({"kind":"memory","title":title,"content":content,"status":"active","basis":"model_inference","references":refs})
}
fn id(record: &Value) -> &str {
    record["id"].as_str().unwrap()
}

struct Fixtures {
    workshop: String,
    paraphrase: String,
    approved: String,
    unreviewed: String,
    retracted: String,
    walk: String,
    source: String,
    checkpoint: String,
    long: String,
    private: String,
}

/// Plants every narrative fixture in a fresh synthetic garden.
fn plant(garden: &Garden) -> Fixtures {
    let workshop = garden.write(memory(
        "Synthetic lantern workshop",
        "She said: \"The paper lanterns looked like a field of small moons.\" She made three that afternoon.",
        "owner_statement",
    ));
    let paraphrase = garden.write(memory(
        "Synthetic morning tea",
        "The owner mentioned preferring green tea in the mornings.",
        "owner_statement",
    ));
    let paraphrase = garden.revise(
        id(&paraphrase),
        1,
        memory(
            "Synthetic morning tea",
            "The owner mentioned preferring oolong tea in the mornings.",
            "owner_statement",
        ),
    );
    let unreviewed = garden.write(reflection(
        "Synthetic note on the workshop",
        "Mother noticed that the workshop came up again a week later, so it may have mattered.",
        &[id(&workshop)],
    ));
    let retracted = garden.write(reflection(
        "Synthetic retracted note",
        "An interpretation the owner asked to withdraw.",
        &[id(&workshop)],
    ));
    garden.revise(
        id(&retracted),
        1,
        json!({"kind":"memory","title":"Synthetic retracted note","content":"An interpretation the owner asked to withdraw.","status":"retracted","basis":"model_inference","references":[id(&workshop)]}),
    );
    let approved = garden.approved_reflection(
        "Synthetic letter about the lanterns",
        "A dated letter from the witness: she described the lanterns as small moons [S1].",
        &[id(&workshop)],
    );
    let walk = garden.write(memory(
        "Synthetic river walk",
        "A walk by the invented river on a grey afternoon.",
        "owner_statement",
    ));
    let source = garden.write(json!({"kind":"memory","title":"Synthetic program notes",
        "content":"The invented concert programme listed four pieces.","status":"active","basis":"source",
        "sources":[{"label":"Synthetic programme","uri":"https://example.test/programme"}]}));
    let checkpoint = garden.write(json!({"kind":"checkpoint","title":"Synthetic handoff",
        "content":"Indexed the synthetic photos.","status":"active","nextStep":"Check the captions.",
        "completed":["Indexed photos"],"openQuestions":["Which album first?"],"references":[id(&walk)]}));
    let long = garden.write(memory(
        "Synthetic long recollection",
        &format!("{}The end.", "A long invented recollection. ".repeat(300)),
        "owner_statement",
    ));
    let private = garden.write(memory(
        "Synthetic private appointment",
        "An invented private appointment, with no photo attached.",
        "owner_statement",
    ));
    Fixtures {
        workshop: id(&workshop).into(),
        paraphrase: id(&paraphrase).into(),
        approved,
        unreviewed: id(&unreviewed).into(),
        retracted: id(&retracted).into(),
        walk: id(&walk).into(),
        source: id(&source).into(),
        checkpoint: id(&checkpoint).into(),
        long: id(&long).into(),
        private: id(&private).into(),
    }
}

#[test]
fn reflections_are_attributed_and_owner_accounts_are_never_verbatim() {
    let dir = tempfile::tempdir().unwrap();
    let garden = Garden::new(&dir.path().join("garden.redb"));
    let f = plant(&garden);
    let read = |id: &str| garden.store.v1_garden_reader(id).unwrap();

    let workshop = read(&f.workshop);
    assert_eq!(workshop["defaultLens"], "mother");
    assert_eq!(workshop["account"]["form"], "captured_paraphrase");
    assert_eq!(
        workshop["account"]["ownerWords"], false,
        "a quote in the text is not proof of verbatim words"
    );
    assert_eq!(workshop["account"]["writer"], "synthetic-harness");
    assert_eq!(
        workshop["eventDate"],
        Value::Null,
        "no event date is guessed"
    );
    let witnesses = workshop["witnessAccounts"].as_array().unwrap();
    assert_eq!(
        witnesses.len(),
        2,
        "the retracted reflection is not offered"
    );
    assert!(
        witnesses
            .iter()
            .all(|w| w["recordId"] != json!(f.retracted))
    );
    let approved = witnesses
        .iter()
        .find(|w| w["recordId"] == json!(f.approved))
        .unwrap();
    assert_eq!(approved["reviewStatus"], "reviewed");
    assert_eq!(
        approved["writer"], "synthetic-witness",
        "the proposer, not the review actor"
    );
    assert_eq!(approved["attribution"], "Synthetic Witness Journal letter");
    assert_eq!(approved["relation"], "references");
    assert_eq!(approved["references"], json!([f.workshop]));
    let unreviewed = witnesses
        .iter()
        .find(|w| w["recordId"] == json!(f.unreviewed))
        .unwrap();
    assert_eq!(
        unreviewed["reviewStatus"], "unknown",
        "active does not mean approved"
    );
    assert_eq!(unreviewed["writer"], "synthetic-harness");
    // the retracted one is still listed, as history, among the records that name this one
    let named: Vec<&Value> = workshop["referencedBy"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| &r["id"])
        .collect();
    assert_eq!(named.len(), 3);
    assert!(named.contains(&&json!(f.retracted)));

    // a reflection read on its own is Mother's Eyes about itself, and names no owner account
    let letter = read(&f.approved);
    assert_eq!(letter["defaultLens"], "mother");
    assert_eq!(letter["account"]["form"], "interpretation");
    assert_eq!(letter["witnessAccounts"][0]["relation"], "self");
    assert_eq!(
        letter["references"][0]["title"],
        "Synthetic lantern workshop"
    );

    let walk = read(&f.walk);
    assert_eq!(walk["defaultLens"], "owner");
    assert_eq!(
        walk["witnessAccounts"],
        json!([]),
        "no reflection is made up"
    );
    assert_eq!(walk["referencedBy"][0]["id"], json!(f.checkpoint));

    let tea = read(&f.paraphrase);
    assert_eq!(tea["revision"], 2);
    assert_eq!(tea["history"].as_array().unwrap().len(), 2);
    assert_eq!(tea["history"][1]["changed"], json!(["content"]));

    let source = read(&f.source);
    assert_eq!(source["account"]["form"], "source_summary");
    assert_eq!(source["basis"], "source");
    assert_eq!(
        source["sources"][0]["uri"],
        "https://example.test/programme"
    );

    let checkpoint = read(&f.checkpoint);
    assert_eq!(checkpoint["account"]["form"], "working_record");
    assert_eq!(checkpoint["basis"], Value::Null);
    assert_eq!(checkpoint["nextStep"], "Check the captions.");
    assert_eq!(checkpoint["openQuestions"], json!(["Which album first?"]));

    let private = read(&f.private);
    assert!(
        private.get("media").is_none(),
        "no media without a verified association"
    );

    // the snapshot preview is cut short; the reading carries the whole record
    let view = garden.store.v1_garden_view(500).unwrap();
    let preview = view["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == json!(f.long))
        .unwrap();
    assert_eq!(preview["truncated"], true);
    assert_eq!(preview["basis"], "owner_statement");
    let long = read(&f.long);
    assert_eq!(long["truncated"], false);
    assert_eq!(long["length"], 9_008);
    assert_eq!(long["content"].as_str().unwrap().len(), 9_008);
}

#[test]
fn reading_writes_nothing_and_refuses_unknown_records() {
    let dir = tempfile::tempdir().unwrap();
    let garden = Garden::new(&dir.path().join("garden.redb"));
    let f = plant(&garden);
    let export = || {
        let mut all = garden.store.v1_export().unwrap();
        all.as_object_mut().unwrap().remove("exportedAt");
        all
    };
    let before = export();
    let first = garden.store.v1_garden_reader(&f.workshop).unwrap();
    assert_eq!(
        garden.store.v1_garden_reader(&f.workshop).unwrap(),
        first,
        "deterministic"
    );
    assert_eq!(export(), before, "reading changes nothing");
    for unknown in [
        Uuid::new_v4().to_string(),
        "not-an-id".into(),
        String::new(),
    ] {
        assert_eq!(
            garden.store.v1_garden_reader(&unknown).unwrap_err().code(),
            "NOT_FOUND"
        );
    }
    let readings = garden
        .store
        .v1_garden_readers(&[&f.walk, "not-an-id", &f.long])
        .unwrap();
    assert_eq!(readings.len(), 2);
    let text = serde_json::to_string(&readings).unwrap();
    assert!(!text.contains(&garden.writer.token));
    assert!(!text.contains("Synthetic owner persona"));
}

struct Running(Child);
impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn http(port: u16, request: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    let mut reply = String::new();
    stream.read_to_string(&mut reply).unwrap();
    (reply[9..12].parse().unwrap(), reply)
}

#[test]
fn offline_explorer_serves_full_readings_after_releasing_the_database() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("garden.redb");
    let garden = Garden::new(&db);
    let f = plant(&garden);
    drop(garden);

    let mut child = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["garden", "explore", "--db"])
        .arg(&db)
        .args(["--limit", "9"])
        .env("MOTHER_HOME", dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let _running = Running(child);
    let started: Value = serde_json::from_str(&line).unwrap();
    let (address, launch) = started["url"].as_str().unwrap().split_once("/#").unwrap();
    let authority = address.strip_prefix("http://").unwrap();
    let port: u16 = authority.rsplit(':').next().unwrap().parse().unwrap();
    GardenStore::open_active(&db).unwrap();

    let get = |path: &str, bearer: &str| {
        http(
            port,
            &format!(
                "GET {path} HTTP/1.1\r\nHost: {authority}\r\nAuthorization: Bearer {bearer}\r\n\r\n"
            ),
        )
    };
    let reading = format!("/api/garden/records/{}", f.long);
    assert_eq!(
        get(&reading, launch).0,
        401,
        "the launch token is not a session"
    );
    let (status, reply) = http(
        port,
        &format!(
            "POST /api/session HTTP/1.1\r\nHost: {authority}\r\nOrigin: {address}\r\nAuthorization: Bearer {launch}\r\nContent-Length: 0\r\n\r\n"
        ),
    );
    assert_eq!(status, 200);
    let session: Value = serde_json::from_str(reply.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    let session = session["session"].as_str().unwrap();

    let (status, reply) = get(&reading, session);
    assert_eq!(status, 200);
    assert!(reply.contains("Cache-Control: no-store"));
    assert!(reply.contains("Content-Security-Policy: default-src 'none'"));
    let body: Value = serde_json::from_str(reply.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(body["length"], 9_008);
    assert_eq!(body["truncated"], false);

    // the oldest record is outside this nine-record snapshot: it reads as unknown
    assert_eq!(
        get(&format!("/api/garden/records/{}", f.workshop), session).0,
        404
    );
    assert_eq!(
        get(&format!("/api/garden/records/{}", Uuid::new_v4()), session).0,
        404
    );
    assert_eq!(get("/api/garden/records/../garden", session).0, 404);
    assert_eq!(
        http(port, &format!("DELETE {reading} HTTP/1.1\r\nHost: {authority}\r\nAuthorization: Bearer {session}\r\n\r\n")).0,
        405
    );
}

/// Writes the narrative fixtures to a new garden for looking at the reader by hand:
/// `MOTHER_READER_DEMO=<new file> cargo test --locked --test garden_reader demo -- --ignored`
#[test]
#[ignore = "writes a synthetic demonstration garden to the path in MOTHER_READER_DEMO"]
fn demo_reader_garden() {
    let path = std::env::var_os("MOTHER_READER_DEMO").expect("set MOTHER_READER_DEMO");
    plant(&Garden::new(Path::new(&path)));
}
