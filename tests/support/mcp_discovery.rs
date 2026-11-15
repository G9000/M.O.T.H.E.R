//! Collect actual native discovery frames within the admitted host's transport limits.
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub fn collect(mut rpc: impl FnMut(Value) -> Value) -> Vec<Value> {
    let mut cursor = Value::Null;
    let mut cursors = BTreeSet::new();
    let mut tools = Vec::new();
    let mut pages = 0;
    let mut first_cursor = None;
    loop {
        let params = if cursor.is_null() {
            json!({})
        } else {
            json!({"cursor":cursor})
        };
        let reply = rpc(params);
        assert!(
            reply.get("error").is_none(),
            "discovery RPC failed: {:?}",
            reply.get("error")
        );
        assert!(
            serde_json::to_vec(&reply).unwrap().len() < 64 * 1024,
            "native discovery must fit the admitted host's 64 KiB JSON-RPC frame"
        );
        let page = reply["result"]["tools"].as_array().unwrap();
        assert!(
            !page.is_empty(),
            "native discovery cannot loop on empty pages"
        );
        tools.extend(page.iter().cloned());
        pages += 1;
        cursor = reply["result"]["nextCursor"].clone();
        if cursor.is_null() {
            break;
        }
        let text = cursor.as_str().unwrap();
        assert!(text.len() <= 96 && cursors.insert(text.to_owned()));
        first_cursor.get_or_insert_with(|| text.to_owned());
        assert!(
            pages < 4,
            "native discovery must complete within the host page budget"
        );
    }
    if tools
        .iter()
        .any(|tool| tool["name"].as_str().unwrap().starts_with("mother_garden_"))
    {
        assert!(pages > 1, "the current graph catalog needs bounded pages");
    }
    let names: BTreeSet<_> = tools
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names.len(),
        tools.len(),
        "discovery must not duplicate tools"
    );
    let mut invalid = vec!["invalid".to_owned()];
    if let Some(cursor) = first_cursor {
        let mut stale = cursor.into_bytes();
        stale[3] = if stale[3] == b'0' { b'1' } else { b'0' };
        invalid.push(String::from_utf8(stale).unwrap());
    }
    for cursor in invalid {
        let reply = rpc(json!({"cursor":cursor}));
        assert_eq!(
            reply["error"]["code"], -32602,
            "malformed or stale discovery cursor must fail closed"
        );
    }
    tools
}
