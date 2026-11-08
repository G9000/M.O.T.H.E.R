//! Opt-in acceptance with the actual pinned local model, never private records or remote inference.
use mother::{contracts::Auth, search::Search, store::Store};
use serde_json::{Value, json};
use std::{path::PathBuf, process::Command, time::Instant};
use uuid::Uuid;
fn write(
    s: &mut Store,
    a: &Auth,
    title: &str,
    content: &str,
    old: Option<&Value>,
    status: &str,
) -> Value {
    let w = s.workspace().unwrap();
    let mut q = json!({"workspaceId":w["id"],"generation":w["generation"],"operationId":Uuid::new_v4().to_string(),"expectedRevision":0,"record":{"kind":"memory","title":title,"content":content,"basis":"source","status":status}});
    if let Some(r) = old {
        q["id"] = r["id"].clone();
        q["expectedRevision"] = r["revision"].clone();
    }
    s.write(a, q).unwrap()["record"].clone()
}
#[test]
#[ignore = "requires MOTHER_TEST_MODEL_DIR with the explicitly installed verified model; run in release mode"]
fn actual_model_recalls_paraphrases_late_passages_and_current_records() {
    let model = PathBuf::from(
        std::env::var_os("MOTHER_TEST_MODEL_DIR").expect("set the verified model directory"),
    );
    let d = tempfile::tempdir().unwrap();
    let db = d.path().join("context.sqlite");
    let mut s = Store::initialize(&db, "Synthetic acceptance persona").unwrap();
    let g = s
        .grant("test", &["read".into(), "memory:write".into()])
        .unwrap();
    let a = Auth {
        client_id: "test".into(),
        token: g["token"].as_str().unwrap().into(),
    };
    let food = write(
        &mut s,
        &a,
        "Meals",
        "I avoid meat and fish and choose plant based meals.",
        None,
        "active",
    );
    let seat = write(
        &mut s,
        &a,
        "Travel",
        "For air travel I prefer seats beside the window rather than the aisle.",
        None,
        "active",
    );
    let backup = write(
        &mut s,
        &a,
        "Recovery",
        "Nightly copies of the project files are stored on an external disk so lost documents can be recovered.",
        None,
        "active",
    );
    write(
        &mut s,
        &a,
        "Gardening",
        "The garden needs fresh soil and regular watering for healthy flowers.",
        None,
        "active",
    );
    write(
        &mut s,
        &a,
        "Supplies",
        "Order printer cartridges using reference ZX-731.",
        None,
        "active",
    );
    write(&mut s, &a, "\u{fffd}", "\u{fffd}", None, "active");
    let long = format!(
        "{}\nMembership may be terminated at any time by sending written notice to the service provider. Further monthly fees then cease.",
        "Garden paths are lined with flowering plants and carefully watered soil. ".repeat(100)
    );
    let cancellation = write(&mut s, &a, "Service notes", &long, None, "active");
    let search = Search::new(model.clone());
    let start = Instant::now();
    let index = search.index(&mut s, &a, 100).unwrap();
    eprintln!("initial index {} {:?}", index, start.elapsed());
    assert_eq!(index["unembeddableRecords"], 1);
    assert_eq!(index["indexedRecords"], 6);
    for (query, expected) in [
        ("What can I cook for a vegetarian dinner?", &food),
        ("Where do I like to sit on a plane?", &seat),
        ("How do we restore accidentally deleted documents?", &backup),
        ("How can I stop paying for the service?", &cancellation),
    ] {
        assert!(
            s.context(&a, json!({"query":query})).unwrap()["records"]
                .as_array()
                .unwrap()
                .is_empty(),
            "fixture must have no literal query hit"
        );
        let start = Instant::now();
        let result = search
            .search(&mut s, &a, json!({"query":query,"mode":"semantic"}))
            .unwrap();
        eprintln!(
            "query={query:?} took={:?} records={}",
            start.elapsed(),
            result["records"]
        );
        assert_eq!(result["search"]["semanticUsed"], true);
        assert_eq!(
            result["records"][0]["id"], expected["id"],
            "{query}: {result}"
        );
        let matched = &result["records"][0]["match"];
        let text = format!(
            "{}\n{}",
            expected["title"].as_str().unwrap(),
            expected["content"].as_str().unwrap()
        );
        let span = &text[matched["startByte"].as_u64().unwrap() as usize
            ..matched["endByte"].as_u64().unwrap() as usize];
        assert!(span.starts_with(matched["text"].as_str().unwrap()));
        if expected["id"] == cancellation["id"] {
            assert!(
                matched["text"].as_str().unwrap().contains("written notice"),
                "late evidence must appear in the actual result snippet"
            );
        }
    }
    let unrelated=search.search(&mut s,&a,json!({"query":"What is quantum chromodynamics and the strong nuclear force?","mode":"semantic"})).unwrap();
    assert!(
        unrelated["records"].as_array().unwrap().is_empty(),
        "unrelated query: {unrelated}"
    );
    let exact = search
        .search(&mut s, &a, json!({"query":"ZX-731"}))
        .unwrap();
    assert_eq!(exact["records"][0]["title"], "Supplies");
    drop(search);
    let search = Search::new(model.clone());
    let reused = search
        .search(&mut s, &a, json!({"query":"vegetarian dinner"}))
        .unwrap();
    assert_eq!(reused["search"]["newlyIndexedRecords"], 0);
    assert_eq!(reused["records"][0]["id"], food["id"]);
    let corrected = write(
        &mut s,
        &a,
        "Travel",
        "For air travel I now prefer seats beside the aisle and no longer want a window seat.",
        Some(&seat),
        "active",
    );
    let changed = search
        .search(
            &mut s,
            &a,
            json!({"query":"Where do I like to sit on a plane?"}),
        )
        .unwrap();
    assert_eq!(changed["records"][0]["id"], seat["id"]);
    assert_eq!(changed["records"][0]["revision"], 2);
    write(
        &mut s,
        &a,
        "Travel",
        corrected["content"].as_str().unwrap(),
        Some(&corrected),
        "retracted",
    );
    let retracted = search
        .search(
            &mut s,
            &a,
            json!({"query":"Where do I like to sit on a plane?"}),
        )
        .unwrap();
    assert!(
        retracted["records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["id"] != seat["id"])
    );
    let config = d.path().join("client.private.json");
    std::fs::write(
        &config,
        json!({"version":1,"databasePath":db,"clientId":a.client_id,"token":a.token}).to_string(),
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args([
            "search",
            "--config",
            config.to_str().unwrap(),
            "--query",
            "vegetarian dinner",
            "--mode",
            "semantic",
        ])
        .env("MOTHER_MODEL_DIR", model)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let result: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(result["records"][0]["id"], food["id"]);
    assert_eq!(result["search"]["newlyIndexedRecords"], 0);
    s.revoke("test").unwrap();
    assert_eq!(
        search
            .search(&mut s, &a, json!({"query":"vegetarian dinner"}))
            .unwrap_err()
            .code(),
        "UNAUTHORIZED"
    );
}
