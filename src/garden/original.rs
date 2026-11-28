//! Read a caller's authorized view of an original across bounded MCP replies.
use super::{
    AccessMode, Auth, ClientAccess, Error, Result, View, contracts, invalid, metadata, record,
    string,
};
use rmcp::model::CallToolResult;
use serde_json::{Value, json};

pub(super) fn read(
    tx: &impl View,
    auth: &Auth,
    access: &ClientAccess,
    args: &Value,
) -> Result<Value> {
    contracts::strict(args, &["source", "id", "cursor", "maxResponseBytes"])?;
    let budget = crate::pagination::response_budget(args)?.unwrap_or(12_288);
    let source = string(args, "source")?;
    let id = args.get("id").map(|_| string(args, "id")).transpose()?;
    let mut original = match (source, id) {
        ("record", Some(id)) => {
            contracts::id(id)?;
            access.require(id)?;
            record(tx, id)?
        }
        ("persona", None) if access.mode == AccessMode::Shared => {
            return Err(Error::domain(
                "FORBIDDEN",
                "This credential cannot read the stored persona.",
            ));
        }
        ("persona", None) => metadata(tx, "persona")?,
        _ => {
            return Err(invalid(
                "Choose a record with its ID, or the persona without an ID.",
            ));
        }
    };
    // Match mother_get_record's grant filtering before the data becomes opaque text.
    access.scrub(&mut original);
    let serialized = serde_json::to_string(&original)?;
    let digest = contracts::hash(&serialized);
    let workspace = metadata(tx, "workspace")?;
    let binding = crate::pagination::binding(&json!([
        workspace["id"],
        workspace["generation"],
        auth.client_id,
        source,
        id,
        digest
    ]));
    let start = match args.get("cursor") {
        None => 0,
        Some(_) => {
            let cursor = string(args, "cursor")?;
            let (stamp, position) = cursor
                .split_once('.')
                .filter(|_| cursor.len() <= 100)
                .ok_or_else(|| invalid("Invalid original cursor."))?;
            if stamp != binding {
                return Err(Error::domain(
                    "CURSOR_STALE",
                    "Original or context changed. Discard the partial original and start again without a cursor.",
                ));
            }
            position
                .parse::<usize>()
                .ok()
                .filter(|i| *i > 0 && *i < serialized.len() && serialized.is_char_boundary(*i))
                .ok_or_else(|| invalid("Invalid original byte position."))?
        }
    };
    let page_at = |end| {
        json!({
            "source": source,
            "id": id,
            "sha256": digest,
            "startByte": start,
            "endByte": end,
            "totalBytes": serialized.len(),
            "fragment": &serialized[start..end],
            "nextCursor": (end < serialized.len()).then(|| format!("{binding}.{end}")),
        })
    };
    // Count the actual SDK result, including its second escaped text representation.
    let fits = |page: &Value| -> Result<bool> {
        Ok(serde_json::to_vec(&CallToolResult::structured(page.clone()))?.len() <= budget)
    };
    if serialized.len() - start <= budget {
        let last = page_at(serialized.len());
        if fits(&last)? {
            return Ok(last);
        }
    }
    // Only partial pages participate in this binary search. The last page has
    // different continuation overhead and was already measured above.
    let mut low = start;
    let mut high = (start + budget).min(serialized.len() - 1);
    while low < high {
        let mut mid = low + (high - low).div_ceil(2);
        while !serialized.is_char_boundary(mid) {
            mid -= 1;
        }
        if mid == low {
            break;
        }
        if fits(&page_at(mid))? {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    if low == start {
        return Err(Error::domain(
            "READ_RESULT_TOO_LARGE",
            "Original metadata exceeds the requested budget.",
        ));
    }
    Ok(page_at(low))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::garden::store::GardenStore;
    use uuid::Uuid;

    fn assemble(store: &GardenStore, auth: &Auth, mut args: Value) -> Value {
        let mut assembled = String::new();
        let mut pages = 0;
        loop {
            let page = store
                .v1_call(auth, "mother_read_original", args.clone())
                .unwrap();
            let reply = CallToolResult::structured(page.clone());
            assert!(serde_json::to_vec(&reply).unwrap().len() <= 4096);
            assert_eq!(page["startByte"], assembled.len());
            assembled.push_str(page["fragment"].as_str().unwrap());
            assert_eq!(page["endByte"], assembled.len());
            pages += 1;
            if page["nextCursor"].is_null() {
                assert_eq!(contracts::hash(&assembled), page["sha256"]);
                assert_eq!(assembled.len(), page["totalBytes"]);
                break;
            }
            args["cursor"] = page["nextCursor"].clone();
        }
        assert!(pages > 1);
        serde_json::from_str(&assembled).unwrap()
    }

    #[test]
    fn original_pages_preserve_long_unicode_and_check_current_access_and_revision() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store =
            GardenStore::initialize(dir.path().join("sample.redb"), &workspace, &generation)
                .unwrap();
        let body = "水🌿\"\\\n".repeat(1200);
        store.v1_initialize(&body).unwrap();
        let owner = store
            .v1_grant(
                "reader",
                &["read", "memory:write"].map(str::to_owned),
                &ClientAccess::default(),
                None,
            )
            .unwrap();
        let write = |title, content, references: Value| {
            let request = json!({
                "workspaceId": workspace,
                "generation": generation,
                "operationId": Uuid::new_v4().to_string(),
                "expectedRevision": 0,
                "record": {
                    "kind": "memory", "status": "active", "basis": "source",
                    "title": title, "content": content, "references": references,
                }
            });
            store
                .v1_call(&owner, "mother_write_record", request)
                .unwrap()["record"]
                .clone()
        };
        let hidden = write("Hidden", "Synthetic text", json!([]));
        let record = write("Story", &body, json!([hidden["id"]]));
        let id = record["id"].as_str().unwrap();
        let shared = store
            .v1_grant(
                "guest",
                &["read".into()],
                &ClientAccess {
                    mode: AccessMode::Shared,
                    subject: Some("guest".into()),
                    records: vec![id.to_owned()],
                },
                None,
            )
            .unwrap();
        let args = json!({"source":"record","id":id,"maxResponseBytes":4096});
        for auth in [&owner, &shared] {
            let expected = store
                .v1_call(auth, "mother_get_record", json!({"id":id}))
                .unwrap();
            assert_eq!(assemble(&store, auth, args.clone()), expected);
        }
        let persona_args = json!({"source":"persona","maxResponseBytes":4096});
        let expected = store
            .v1_call(&owner, "mother_get_context", json!({"limit":1}))
            .unwrap()["persona"]
            .clone();
        assert_eq!(assemble(&store, &owner, persona_args.clone()), expected);
        assert_eq!(
            store
                .v1_call(&shared, "mother_read_original", persona_args)
                .unwrap_err()
                .code(),
            "FORBIDDEN"
        );
        let cursor =
            store.v1_call(&owner, "mother_read_original", args).unwrap()["nextCursor"].clone();
        let next = json!({"source":"record","id":id,"cursor":cursor,"maxResponseBytes":4096});
        assert_eq!(
            store
                .v1_call(&shared, "mother_read_original", next.clone())
                .unwrap_err()
                .code(),
            "CURSOR_STALE"
        );
        let revision = json!({
            "workspaceId": workspace,
            "generation": generation,
            "operationId": Uuid::new_v4().to_string(),
            "id": id,
            "expectedRevision": 1,
            "record": {
                "kind": "memory", "status": "active", "basis": "source",
                "title": "Revised", "content": "Updated synthetic account",
            }
        });
        store
            .v1_call(&owner, "mother_write_record", revision)
            .unwrap();
        assert_eq!(
            store
                .v1_call(&owner, "mother_read_original", next)
                .unwrap_err()
                .code(),
            "CURSOR_STALE"
        );
    }
}
