use mother::{
    access::ClientAccess,
    garden::{access::GardenAccess, contracts::*, store::GardenStore},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};
use uuid::Uuid;

struct Fixture {
    _directory: tempfile::TempDir,
    store: GardenStore,
    auth: mother::contracts::Auth,
    workspace: String,
    generation: String,
    bundle: PathBuf,
    destination: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let directory =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let workspace = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(
            directory.path().join("source.redb"),
            &workspace,
            &generation,
        )
        .unwrap();
        let auth = store
            .grant(
                "synthetic",
                &[
                    "garden:read".into(),
                    "garden:write".into(),
                    "garden:propose".into(),
                ],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        let bundle = directory.path().join("bundle");
        let destination = directory.path().join("restored.redb");
        Self {
            _directory: directory,
            store,
            auth,
            workspace,
            generation,
            bundle,
            destination,
        }
    }
    fn entity(&self, name: &str) -> BatchRequest {
        serde_json::from_value(json!({"version":2,"workspaceId":self.workspace,"generation":self.generation,
            "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"entity","mutation":"create","expectedRevision":0,
                "draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}}]})).unwrap()
    }
    fn snapshot(&self) {
        self.store.backup_to(&self.bundle).unwrap();
    }
    fn rejected(&self) {
        assert!(
            GardenStore::verify_backup(&self.bundle).is_err(),
            "verification accepted corrupt bundle"
        );
        assert!(
            GardenStore::restore_backup(&self.bundle, &self.destination).is_err(),
            "restore accepted corrupt bundle"
        );
        assert!(
            !self.destination.exists(),
            "failed restore published a destination"
        );
        self.store.audit(&self.auth).unwrap();
    }
}
fn manifest(bundle: &Path) -> Value {
    serde_json::from_slice(&fs::read(bundle.join("manifest.json")).unwrap()).unwrap()
}
fn save_manifest(bundle: &Path, value: &Value) {
    fs::write(
        bundle.join("manifest.json"),
        serde_json::to_vec(value).unwrap(),
    )
    .unwrap();
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}

// Refresh all transport hashes/counts after tampering, so these tests exercise
// canonical validation rather than merely detecting a damaged JSONL checksum.
fn edit_rows(bundle: &Path, section: &str, edit: impl FnOnce(&mut Vec<Value>)) {
    let mut value = manifest(bundle);
    let chunk = value["chunks"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|c| c["section"] == section)
        .expect("section has a chunk");
    let path = bundle.join(chunk["file"].as_str().unwrap());
    let mut rows = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect::<Vec<Value>>();
    let before = rows.len();
    edit(&mut rows);
    let mut encoded = Vec::new();
    for row in &rows {
        serde_json::to_writer(&mut encoded, row).unwrap();
        encoded.push(b'\n');
    }
    fs::write(path, &encoded).unwrap();
    chunk["rows"] = json!(rows.len());
    chunk["sizeBytes"] = json!(encoded.len());
    chunk["sha256"] = json!(format!("{:x}", Sha256::digest(&encoded)));
    let old = value["sections"][section].as_u64().unwrap();
    value["sections"][section] = json!(old - before as u64 + rows.len() as u64);
    save_manifest(bundle, &value);
}
fn edit_payload(row: &mut Value, edit: impl FnOnce(&mut Value)) {
    let mut payload: Value =
        serde_json::from_slice(&unhex(row["value"].as_str().unwrap())).unwrap();
    edit(&mut payload);
    row["value"] = json!(hex(&serde_json::to_vec(&payload).unwrap()));
}

#[test]
fn checksummed_historical_receipt_forgery_is_rejected_before_publication() {
    let f = Fixture::new();
    let original = f.store.apply(&f.auth, &f.entity("original")).unwrap();
    let mut correction = f.entity("corrected");
    correction.items[0].mutation = Mutation::Update;
    correction.items[0].id = Some(original.receipt.results[0].reference.clone());
    correction.items[0].expected_revision = 1;
    f.store.apply(&f.auth, &correction).unwrap();
    f.snapshot();
    edit_rows(&f.bundle, "garden_receipts", |rows| {
        let row = rows
            .iter_mut()
            .find(|row| {
                let payload: Value =
                    serde_json::from_slice(&unhex(row["value"].as_str().unwrap())).unwrap();
                payload["receipt"]["commitSequence"] == 1
            })
            .unwrap();
        edit_payload(row, |payload| {
            payload["records"][0]["draft"]["displayName"] = json!("forged earlier evidence")
        });
    });
    f.rejected();
}

#[test]
fn checksummed_duplicate_canonical_row_is_rejected_before_publication() {
    let f = Fixture::new();
    f.store.apply(&f.auth, &f.entity("person")).unwrap();
    f.snapshot();
    edit_rows(&f.bundle, "garden_current", |rows| {
        rows.push(rows[0].clone())
    });
    f.rejected();
}

#[test]
fn required_extension_and_schema_version_are_fail_closed() {
    for (field, replacement) in [
        (
            "requiredExtensions",
            json!({"mother:full-garden":1,"unknown:required":1}),
        ),
        ("format", json!(2)),
    ] {
        let f = Fixture::new();
        f.snapshot();
        let mut value = manifest(&f.bundle);
        value[field] = replacement;
        save_manifest(&f.bundle, &value);
        f.rejected();
    }
}

#[test]
fn checksummed_blob_content_corruption_is_rejected_before_publication() {
    let f = Fixture::new();
    f.store
        .put_blob(Cursor::new(b"synthetic artifact bytes"))
        .unwrap();
    f.snapshot();
    edit_rows(&f.bundle, "garden_blob_chunks", |rows| {
        let mut bytes = unhex(rows[0]["value"].as_str().unwrap());
        bytes[0] ^= 1;
        rows[0]["value"] = json!(hex(&bytes));
    });
    f.rejected();
}

#[test]
fn manifest_cannot_hide_blob_bytes_required_only_by_a_pending_proposal() {
    let f = Fixture::new();
    let blob = f
        .store
        .put_blob(Cursor::new(b"pending artifact bytes"))
        .unwrap();
    let evidence:BatchRequest=serde_json::from_value(json!({"version":2,"workspaceId":f.workspace,"generation":f.generation,
        "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[{"itemKey":"evidence","mutation":"create","expectedRevision":0,
            "draft":{"kind":"evidence","source":{"type":"artifact","sha256":blob.sha256,"sizeBytes":blob.size_bytes,"mediaType":"text/plain"},
                "excerpt":null,"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}}]})).unwrap();
    f.store.propose(&f.auth, &evidence).unwrap();
    f.snapshot();
    let mut value = manifest(&f.bundle);
    value["chunks"].as_array_mut().unwrap().retain(|chunk| {
        if matches!(
            chunk["section"].as_str(),
            Some("garden_blob_metadata" | "garden_blob_chunks")
        ) {
            fs::remove_file(f.bundle.join(chunk["file"].as_str().unwrap())).unwrap();
            false
        } else {
            true
        }
    });
    // Renumber remaining chunk files to retain an otherwise valid manifest.
    for (index, chunk) in value["chunks"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        let name = format!("chunk-{index:08}.jsonl");
        let old = chunk["file"].as_str().unwrap();
        if name != old {
            fs::rename(f.bundle.join(old), f.bundle.join(&name)).unwrap();
            chunk["file"] = json!(name);
        }
    }
    value["sections"]["garden_blob_metadata"] = json!(0);
    value["sections"]["garden_blob_chunks"] = json!(0);
    save_manifest(&f.bundle, &value);
    f.rejected();
}

#[test]
fn missing_chunk_and_invalid_checksum_never_publish_a_destination() {
    for missing in [false, true] {
        let f = Fixture::new();
        f.snapshot();
        let mut value = manifest(&f.bundle);
        let file = f.bundle.join(value["chunks"][0]["file"].as_str().unwrap());
        if missing {
            fs::remove_file(file).unwrap();
        } else {
            value["chunks"][0]["sha256"] = json!("0".repeat(64));
            save_manifest(&f.bundle, &value);
        }
        f.rejected();
    }
}

#[test]
fn manifest_chunk_paths_cannot_escape_the_bundle() {
    for name in ["../outside.jsonl", "C:\\outside.jsonl", "/outside.jsonl"] {
        let f = Fixture::new();
        f.snapshot();
        let mut value = manifest(&f.bundle);
        value["chunks"][0]["file"] = json!(name);
        save_manifest(&f.bundle, &value);
        f.rejected();
    }
}

#[cfg(unix)]
#[test]
fn backup_paths_reject_parent_aliases_while_canonical_paths_remain_usable() {
    let f = Fixture::new();
    let alias = f._directory.path().join("alias");
    std::os::unix::fs::symlink(f._directory.path(), &alias).unwrap();
    let aliased_bundle = alias.join("bundle");
    assert!(f.store.backup_to(&aliased_bundle).is_err());
    assert!(
        !f.bundle.exists(),
        "rejected alias must not publish a bundle"
    );

    f.snapshot();
    GardenStore::verify_backup(&f.bundle).unwrap();
    assert!(GardenStore::verify_backup(&aliased_bundle).is_err());
    assert!(GardenStore::restore_backup(&f.bundle, &alias.join("restored.redb")).is_err());
    assert!(
        !f.destination.exists(),
        "rejected alias must not publish a database"
    );
    let restored = GardenStore::restore_backup(&f.bundle, &f.destination).unwrap();
    drop(restored);
}

#[cfg(unix)]
#[test]
fn symlinked_chunk_is_rejected_even_when_bytes_and_manifest_match() {
    let f = Fixture::new();
    f.snapshot();
    let value = manifest(&f.bundle);
    let chunk = f.bundle.join(value["chunks"][0]["file"].as_str().unwrap());
    let outside = f._directory.path().join("outside.jsonl");
    fs::rename(&chunk, &outside).unwrap();
    std::os::unix::fs::symlink(&outside, &chunk).unwrap();
    f.rejected();
}
