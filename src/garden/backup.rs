//! Streaming, credential-free Garden bundles and atomic fresh-file restoration.
use super::{
    CURRENT, GardenStore, META, RECEIPTS, REPLACEMENTS, Result, compat, decode, encode, failure,
    history, inbox, indexes, media, state_read, state_write, storage_error, validate_identity,
};
use crate::{
    contracts,
    garden::contracts::{GardenError, GardenErrorCode, MAX_SAFE_INTEGER},
};
use redb::{
    Durability, ReadTransaction, ReadableDatabase, ReadableTable, TableDefinition, TableHandle,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};
use uuid::Uuid;

const FORMAT: u32 = 1;
const MAX_ROW: usize = 4 * 1024 * 1024;
const CHUNK_TARGET: usize = 8 * 1024 * 1024;
const BLOB_CHUNK: usize = 1024 * 1024;
pub(super) const BLOBS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_blob_metadata");
pub(super) const BLOB_BYTES: TableDefinition<(&str, u64), &[u8]> =
    TableDefinition::new("garden_blob_chunks");
const INBOX: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_inbox");
const REVIEWS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_review_operations");
const FAMILIES: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_operation_families");
const INBOX_OPS: TableDefinition<&str, &str> = TableDefinition::new("garden_inbox_operations");

/// Stored content address; bytes stay inside the same atomic Garden database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlobInfo {
    pub sha256: String,
    pub size_bytes: u64,
    pub chunks: u64,
}
/// Durable restore barriers exposed only to trusted owner instrumentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupRestoreStage {
    Validated,
    Staged,
    BeforeCommit,
    Committed,
    BeforePublish,
    Published,
}
/// Verified bundle counts, independent of credentials or derived indexes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BackupReport {
    pub workspace_id: String,
    pub generation: String,
    pub rows: u64,
    pub chunks: u64,
    pub bytes: u64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Manifest {
    format: u32,
    required_extensions: BTreeMap<String, u32>,
    workspace_id: String,
    generation: String,
    sections: BTreeMap<String, u64>,
    chunks: Vec<Chunk>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Chunk {
    file: String,
    section: String,
    size_bytes: u64,
    sha256: String,
    rows: u64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    key: Value,
    value: String,
}
fn bad(message: &str) -> GardenError {
    failure(GardenErrorCode::InvalidInput, message)
}
fn hex(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(ALPHABET[(b >> 4) as usize] as char);
        out.push(ALPHABET[(b & 15) as usize] as char);
    }
    out
}
fn unhex(text: &str) -> Result<Vec<u8>> {
    if !text.len().is_multiple_of(2)
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(bad("Invalid portable byte encoding"));
    }
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| {
            let digit = |b: u8| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
            Ok((digit(p[0]) << 4) | digit(p[1]))
        })
        .collect()
}

fn byte_tables() -> Vec<TableDefinition<'static, &'static str, &'static [u8]>> {
    let mut tables = vec![META, CURRENT, RECEIPTS, INBOX, REVIEWS, FAMILIES, BLOBS];
    tables.extend(compat::backup_tables().iter().copied());
    tables.extend(MEDIA_TABLES);
    tables
}
/// Media sections added after format 1 shipped; bundles written earlier omit them.
const MEDIA_TABLES: [TableDefinition<'static, &'static str, &'static [u8]>; 3] =
    [media::ASSETS, media::ACQUISITIONS, media::DERIVATIVES];
fn optional_section(name: &str) -> bool {
    MEDIA_TABLES.iter().any(|t| t.name() == name)
}
fn sections() -> BTreeSet<String> {
    byte_tables()
        .iter()
        .map(|t| t.name().to_owned())
        .chain(
            [
                "garden_history",
                "garden_replacements",
                "garden_inbox_operations",
                "garden_blob_chunks",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .collect()
}
fn known_tables() -> BTreeSet<String> {
    sections()
        .into_iter()
        .chain(
            [
                "garden_principals",
                // Trust policies follow grants: never exported, never restored.
                "garden_trust_policies",
                "garden_replacement_incoming",
                "garden_revisions",
                "garden_transaction_times",
                "garden_ids",
                "garden_outgoing",
                "garden_incoming",
                "garden_predicate",
                "garden_valid_time",
                "garden_evidence",
                "garden_sources",
                "garden_namespaces",
                "garden_transaction_sequence",
                "garden_query_current_v1",
                "garden_query_history_v1",
                "garden_valid_start_v1",
                "garden_valid_end_v1",
                "garden_current_valid_start_v1",
                "garden_current_valid_end_v1",
                "garden_page_leases",
                "garden_page_expiry",
                "garden_page_tokens",
                "garden_page_snapshot_data",
                "garden_page_snapshot_expiry",
                "garden_vector_profiles_v1",
                "garden_vectors_v1",
                "garden_v1_admissions",
                "garden_v1_pending_dedup",
                "garden_v1_vectors_v1",
                "garden_media_stages",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .collect()
}
pub(super) fn safe_path(path: &Path) -> Result<()> {
    for parent in path.ancestors() {
        match fs::symlink_metadata(parent) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    return Err(bad("Links are not allowed in backup paths"));
                }
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    if meta.file_attributes() & 0x400 != 0 {
                        return Err(bad("Reparse points are not allowed in backup paths"));
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(storage_error(e)),
        }
    }
    Ok(())
}
#[cfg(unix)]
fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)
        .and_then(|f| f.sync_all())
        .map_err(storage_error)
}
#[cfg(not(unix))]
fn sync_dir(_path: &Path) -> Result<()> {
    Ok(())
}
pub(super) fn create_bundle_dir(path: &Path) -> Result<()> {
    #[cfg(unix)]
    let mut builder = fs::DirBuilder::new();
    #[cfg(not(unix))]
    let builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(path).map_err(storage_error)
}
struct Export<'a> {
    dir: &'a Path,
    manifest: Manifest,
    file: Option<File>,
    current: Option<Chunk>,
    hash: Sha256,
    target: usize,
}
impl Export<'_> {
    fn finish_chunk(&mut self) -> Result<()> {
        if let Some(file) = self.file.take() {
            file.sync_all().map_err(storage_error)?;
            let mut chunk = self
                .current
                .take()
                .ok_or_else(|| bad("Missing chunk state"))?;
            chunk.sha256 = hex(&self.hash.clone().finalize());
            self.manifest.chunks.push(chunk);
            self.hash = Sha256::new();
        }
        Ok(())
    }
    fn row(&mut self, section: &str, key: Value, bytes: &[u8]) -> Result<()> {
        let row = Row {
            key,
            value: hex(bytes),
        };
        let mut line = serde_json::to_vec(&row).map_err(storage_error)?;
        line.push(b'\n');
        if line.len() > MAX_ROW {
            return Err(bad("Canonical row exceeds portable 4 MiB row bound"));
        }
        if self.current.as_ref().is_some_and(|c| {
            c.section != section || c.size_bytes + line.len() as u64 > self.target as u64
        }) {
            self.finish_chunk()?;
        }
        if self.file.is_none() {
            let filename = format!("chunk-{:08}.jsonl", self.manifest.chunks.len());
            self.file = Some(
                crate::config::create_private_file(&self.dir.join(&filename))
                    .map_err(storage_error)?,
            );
            self.current = Some(Chunk {
                file: filename,
                section: section.into(),
                size_bytes: 0,
                sha256: String::new(),
                rows: 0,
            });
        }
        self.file
            .as_mut()
            .ok_or_else(|| bad("Missing chunk writer"))?
            .write_all(&line)
            .map_err(storage_error)?;
        self.hash.update(&line);
        let c = self
            .current
            .as_mut()
            .ok_or_else(|| bad("Missing chunk metadata"))?;
        c.size_bytes += line.len() as u64;
        c.rows += 1;
        *self
            .manifest
            .sections
            .get_mut(section)
            .ok_or_else(|| bad("Unknown section"))? += 1;
        Ok(())
    }
}
impl GardenStore {
    /// Stream owner-supplied bytes into one content-addressed database transaction.
    /// # Errors
    /// Rejects empty input, read failures, conflicting content addresses or storage failure.
    pub fn put_blob(&self, reader: impl Read) -> Result<BlobInfo> {
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let info = put_blob_tx(&tx, reader)?;
        tx.commit().map_err(storage_error)?;
        Ok(info)
    }
}
/// Stage content-addressed chunks inside a caller-owned write transaction.
pub(super) fn put_blob_tx(tx: &redb::WriteTransaction, mut reader: impl Read) -> Result<BlobInfo> {
    {
        let temporary = Uuid::new_v4().to_string();
        let mut hash = Sha256::new();
        let mut size = 0u64;
        let mut count = 0u64;
        let mut buffer = vec![0; BLOB_CHUNK];
        {
            let mut table = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
            loop {
                let mut n = 0;
                while n < buffer.len() {
                    match reader.read(&mut buffer[n..]) {
                        Ok(0) => break,
                        Ok(read) => n += read,
                        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(e) => return Err(storage_error(e)),
                    }
                }
                if n == 0 {
                    break;
                }
                size = size
                    .checked_add(n as u64)
                    .filter(|s| *s <= MAX_SAFE_INTEGER)
                    .ok_or_else(|| bad("Blob size exceeds safe integer bound"))?;
                hash.update(&buffer[..n]);
                table
                    .insert((temporary.as_str(), count), &buffer[..n])
                    .map_err(storage_error)?;
                count += 1;
            }
        }
        if size == 0 {
            return Err(bad("Empty blob is not an artifact"));
        }
        let info = BlobInfo {
            sha256: hex(&hash.finalize()),
            size_bytes: size,
            chunks: count,
        };
        {
            let mut table = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
            for index in 0..count {
                let bytes = table
                    .get((temporary.as_str(), index))
                    .map_err(storage_error)?
                    .ok_or_else(|| bad("Missing staged blob"))?
                    .value()
                    .to_owned();
                table
                    .insert((info.sha256.as_str(), index), bytes.as_slice())
                    .map_err(storage_error)?;
                table
                    .remove((temporary.as_str(), index))
                    .map_err(storage_error)?;
            }
        }
        tx.open_table(BLOBS)
            .map_err(storage_error)?
            .insert(info.sha256.as_str(), encode(&info)?.as_slice())
            .map_err(storage_error)?;
        Ok(info)
    }
}
impl GardenStore {
    /// Read a file only from the trusted owner API, never from graph/MCP inputs.
    /// # Errors
    /// Propagates file access and blob transaction errors.
    pub fn put_blob_file(&self, path: &Path) -> Result<BlobInfo> {
        self.put_blob(File::open(path).map_err(storage_error)?)
    }
    /// Export one consistent snapshot into a new directory, writing its manifest last.
    /// # Errors
    /// Rejects existing destinations, unknown tables, missing blobs and I/O failure.
    pub fn backup_to(&self, out: &Path) -> Result<BackupReport> {
        self.backup_with_chunk_size(out, CHUNK_TARGET)
    }
    /// Export with a trusted test chunk budget; row bounds remain enforced.
    #[doc(hidden)]
    pub fn backup_with_chunk_size(&self, out: &Path, target: usize) -> Result<BackupReport> {
        if target == 0 || target > CHUNK_TARGET {
            return Err(bad("Invalid backup chunk target"));
        }
        safe_path(out)?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        super::backup_audit::audit_read(&tx)?;
        inbox::audit_read(&tx)?;
        compat::validate_restored(&tx).map_err(storage_error)?;
        let known = known_tables();
        for table in tx.list_tables().map_err(storage_error)? {
            if !known.contains(table.name()) {
                return Err(bad("Unknown canonical table; backup refused"));
            }
        }
        if tx
            .list_multimap_tables()
            .map_err(storage_error)?
            .next()
            .is_some()
        {
            return Err(bad("Unknown canonical multimap table"));
        }
        media::validate_stages(&tx)?;
        validate_blobs(&tx)?;
        media::validate(&tx)?;
        create_bundle_dir(out)?;
        let owner = Uuid::new_v4().to_string();
        let marker = out.join(".mother-backup-owner");
        let mut owner_file = crate::config::create_private_file(&marker).map_err(storage_error)?;
        owner_file
            .write_all(owner.as_bytes())
            .map_err(storage_error)?;
        owner_file.sync_all().map_err(storage_error)?;
        drop(owner_file);
        let result = (|| {
            let mut export = Export {
                dir: out,
                manifest: Manifest {
                    format: FORMAT,
                    required_extensions: BTreeMap::from([("mother:full-garden".into(), 1)]),
                    workspace_id: state.workspace_id,
                    generation: state.generation,
                    sections: sections().into_iter().map(|s| (s, 0)).collect(),
                    chunks: Vec::new(),
                },
                file: None,
                current: None,
                hash: Sha256::new(),
                target,
            };
            let present: BTreeSet<_> = tx
                .list_tables()
                .map_err(storage_error)?
                .map(|t| t.name().to_owned())
                .collect();
            for table in byte_tables() {
                if !present.contains(table.name()) {
                    continue;
                }
                for row in tx
                    .open_table(table)
                    .map_err(storage_error)?
                    .iter()
                    .map_err(storage_error)?
                {
                    let (key, bytes) = row.map_err(storage_error)?;
                    if table.name() == META.name()
                        && !matches!(key.value(), "state" | "retained_generations")
                    {
                        if !matches!(
                            key.value(),
                            "inbox_registry_initialized"
                                | "query_indexes_v1"
                                | "pager_initialized"
                                | "pager_clock"
                        ) {
                            return Err(bad("Unknown canonical metadata; backup refused"));
                        }
                        continue;
                    }
                    let portable = if table.name() == INBOX.name() {
                        inbox::portable_proposal(bytes.value())?
                    } else {
                        bytes.value().to_owned()
                    };
                    export.row(table.name(), json!(key.value()), &portable)?;
                }
            }
            for row in tx
                .open_table(history::HISTORY)
                .map_err(storage_error)?
                .iter()
                .map_err(storage_error)?
            {
                let (key, bytes) = row.map_err(storage_error)?;
                let (id, seq) = key.value();
                export.row("garden_history", json!([id, seq]), bytes.value())?;
            }
            for definition in [REPLACEMENTS, INBOX_OPS] {
                for row in tx
                    .open_table(definition)
                    .map_err(storage_error)?
                    .iter()
                    .map_err(storage_error)?
                {
                    let (key, value) = row.map_err(storage_error)?;
                    export.row(
                        definition.name(),
                        json!(key.value()),
                        value.value().as_bytes(),
                    )?;
                }
            }
            if present.contains(BLOB_BYTES.name()) {
                for row in tx
                    .open_table(BLOB_BYTES)
                    .map_err(storage_error)?
                    .iter()
                    .map_err(storage_error)?
                {
                    let (key, bytes) = row.map_err(storage_error)?;
                    let (id, index) = key.value();
                    export.row(BLOB_BYTES.name(), json!([id, index]), bytes.value())?;
                }
            }
            export.finish_chunk()?;
            let bytes = serde_json::to_vec_pretty(&export.manifest).map_err(storage_error)?;
            if bytes.len() > 16 * 1024 * 1024 {
                return Err(bad("Backup manifest exceeds metadata bound"));
            }
            let mut file = crate::config::create_private_file(&out.join("manifest.json"))
                .map_err(storage_error)?;
            file.write_all(&bytes).map_err(storage_error)?;
            file.sync_all().map_err(storage_error)?;
            fs::remove_file(&marker).map_err(storage_error)?;
            sync_dir(out)?;
            Ok(report(&export.manifest))
        })();
        if result.is_err()
            && safe_path(out).is_ok()
            && fs::read_to_string(&marker).is_ok_and(|text| text == owner)
        {
            let _ = fs::remove_dir_all(out);
        }
        result
    }
    /// Validate hashes, schemas and full canonical relationships without publishing a store.
    /// # Errors
    /// Rejects incomplete, corrupt, credential-bearing or incompatible bundles.
    pub fn verify_backup(bundle: &Path) -> Result<BackupReport> {
        let temporary = tempfile::tempdir().map_err(storage_error)?;
        let (_, report) =
            restore_staging(bundle, &temporary.path().join("verified.redb"), &mut |_| {})?;
        Ok(report)
    }
    /// Restore a verified full snapshot to a fresh generation and nonexistent file.
    /// # Errors
    /// Existing destinations are never overwritten; invalid bundles never publish a target.
    pub fn restore_backup(bundle: &Path, destination: &Path) -> Result<Self> {
        Self::restore_backup_with_hook(bundle, destination, |_| {})
    }
    /// Restore with trusted qualification hooks at publication and transaction barriers.
    #[doc(hidden)]
    pub fn restore_backup_with_hook(
        bundle: &Path,
        destination: &Path,
        mut hook: impl FnMut(BackupRestoreStage),
    ) -> Result<Self> {
        safe_path(destination)?;
        if destination.exists() {
            return Err(bad("Restore destination already exists"));
        }
        let parent = destination
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let temporary = tempfile::Builder::new()
            .prefix(".mother-restore-")
            .tempdir_in(parent)
            .map_err(storage_error)?;
        let stage = temporary.path().join("staging.redb");
        let (store, _) = restore_staging(bundle, &stage, &mut hook)?;
        drop(store);
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&stage)
            .and_then(|f| f.sync_all())
            .map_err(storage_error)?;
        hook(BackupRestoreStage::BeforePublish);
        safe_path(destination)?;
        fs::hard_link(&stage, destination).map_err(storage_error)?;
        sync_dir(parent)?;
        hook(BackupRestoreStage::Published);
        Self::open(destination)
    }
}
fn report(manifest: &Manifest) -> BackupReport {
    BackupReport {
        workspace_id: manifest.workspace_id.clone(),
        generation: manifest.generation.clone(),
        rows: manifest.sections.values().sum(),
        chunks: manifest.chunks.len() as u64,
        bytes: manifest.chunks.iter().map(|c| c.size_bytes).sum(),
    }
}
fn read_manifest(bundle: &Path) -> Result<Manifest> {
    safe_path(bundle)?;
    if !fs::symlink_metadata(bundle)
        .map_err(storage_error)?
        .is_dir()
    {
        return Err(bad("Backup must be a directory"));
    }
    let path = bundle.join("manifest.json");
    safe_path(&path)?;
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(storage_error)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(storage_error)?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err(bad("Backup manifest exceeds metadata bound"));
    }
    let manifest: Manifest =
        serde_json::from_slice(&bytes).map_err(|_| bad("Invalid backup manifest"))?;
    validate_identity(&manifest.workspace_id, &manifest.generation)?;
    if manifest.format != FORMAT
        || manifest.required_extensions != BTreeMap::from([("mother:full-garden".into(), 1)])
        || !manifest.sections.keys().all(|s| sections().contains(s))
        || !sections()
            .iter()
            .all(|s| optional_section(s) || manifest.sections.contains_key(s))
    {
        return Err(bad(
            "Unsupported backup format, required extension or canonical section",
        ));
    }
    let mut files = BTreeSet::from(["manifest.json".to_owned()]);
    let mut totals: BTreeMap<_, u64> = manifest.sections.keys().map(|s| (s.clone(), 0)).collect();
    for (index, chunk) in manifest.chunks.iter().enumerate() {
        if chunk.file != format!("chunk-{index:08}.jsonl")
            || !files.insert(chunk.file.clone())
            || chunk.size_bytes == 0
            || chunk.size_bytes > CHUNK_TARGET as u64
            || chunk.rows == 0
            || !contracts::hex_digest(&chunk.sha256)
        {
            return Err(bad("Invalid chunk manifest"));
        }
        let total = totals
            .get_mut(&chunk.section)
            .ok_or_else(|| bad("Unknown canonical section"))?;
        *total = total
            .checked_add(chunk.rows)
            .ok_or_else(|| bad("Chunk count overflow"))?;
    }
    if totals != manifest.sections {
        return Err(bad("Manifest section counts do not match chunks"));
    }
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(bundle).map_err(storage_error)? {
        let entry = entry.map_err(storage_error)?;
        safe_path(&entry.path())?;
        if !entry.file_type().map_err(storage_error)?.is_file() {
            return Err(bad("Backup entries must be regular files"));
        }
        actual.insert(
            entry
                .file_name()
                .into_string()
                .map_err(|_| bad("Invalid backup file name"))?,
        );
    }
    if actual != files {
        return Err(bad("Backup has missing or unlisted files"));
    }
    Ok(manifest)
}
fn insert_row(tx: &redb::WriteTransaction, section: &str, row: Row) -> Result<()> {
    let bytes = unhex(&row.value)?;
    if section == INBOX.name() && !decode::<Value>(&bytes)?["principal"].is_null() {
        return Err(bad("Portable proposal contains a credential snapshot"));
    }
    if let Some(definition) = byte_tables().into_iter().find(|t| t.name() == section) {
        let key = row
            .key
            .as_str()
            .ok_or_else(|| bad("Invalid canonical string key"))?;
        if section == META.name() && !matches!(key, "state" | "retained_generations") {
            return Err(bad("Unknown canonical metadata"));
        }
        let mut table = tx.open_table(definition).map_err(storage_error)?;
        if table.get(key).map_err(storage_error)?.is_some() {
            return Err(bad("Duplicate canonical key"));
        }
        table.insert(key, bytes.as_slice()).map_err(storage_error)?;
        return Ok(());
    }
    match section {
        "garden_history" | "garden_blob_chunks" => {
            let pair = row
                .key
                .as_array()
                .filter(|p| p.len() == 2)
                .ok_or_else(|| bad("Invalid canonical pair key"))?;
            let id = pair[0]
                .as_str()
                .ok_or_else(|| bad("Invalid canonical pair ID"))?;
            let sequence = pair[1]
                .as_u64()
                .ok_or_else(|| bad("Invalid canonical pair sequence"))?;
            let definition = if section == "garden_history" {
                history::HISTORY
            } else {
                BLOB_BYTES
            };
            let mut table = tx.open_table(definition).map_err(storage_error)?;
            if table.get((id, sequence)).map_err(storage_error)?.is_some() {
                return Err(bad("Duplicate canonical key"));
            }
            table
                .insert((id, sequence), bytes.as_slice())
                .map_err(storage_error)?;
        }
        "garden_replacements" | "garden_inbox_operations" => {
            let key = row
                .key
                .as_str()
                .ok_or_else(|| bad("Invalid canonical string key"))?;
            let value =
                std::str::from_utf8(&bytes).map_err(|_| bad("Invalid canonical string value"))?;
            let definition = if section == "garden_replacements" {
                REPLACEMENTS
            } else {
                INBOX_OPS
            };
            let mut table = tx.open_table(definition).map_err(storage_error)?;
            if table.get(key).map_err(storage_error)?.is_some() {
                return Err(bad("Duplicate canonical key"));
            }
            table.insert(key, value).map_err(storage_error)?;
        }
        _ => return Err(bad("Unknown canonical section")),
    }
    Ok(())
}
fn restore_staging(
    bundle: &Path,
    stage: &Path,
    hook: &mut impl FnMut(BackupRestoreStage),
) -> Result<(GardenStore, BackupReport)> {
    let manifest = read_manifest(bundle)?;
    hook(BackupRestoreStage::Validated);
    let store = GardenStore::initialize(stage, &manifest.workspace_id, &manifest.generation)?;
    let mut tx = store.db.begin_write().map_err(storage_error)?;
    tx.set_durability(Durability::Immediate)
        .map_err(storage_error)?;
    tx.open_table(META)
        .map_err(storage_error)?
        .remove("state")
        .map_err(storage_error)?;
    for definition in byte_tables() {
        tx.open_table(definition).map_err(storage_error)?;
    }
    tx.open_table(BLOB_BYTES).map_err(storage_error)?;
    for chunk in &manifest.chunks {
        let path = bundle.join(&chunk.file);
        safe_path(&path)?;
        let file = File::open(path).map_err(storage_error)?;
        if file.metadata().map_err(storage_error)?.len() != chunk.size_bytes {
            return Err(bad("Chunk size mismatch"));
        }
        let mut reader = BufReader::new(file);
        let mut hash = Sha256::new();
        let mut size = 0u64;
        let mut rows = 0u64;
        loop {
            let mut line = Vec::new();
            let read = reader
                .by_ref()
                .take(MAX_ROW as u64 + 1)
                .read_until(b'\n', &mut line)
                .map_err(storage_error)?;
            if read == 0 {
                break;
            }
            if read > MAX_ROW || line.last() != Some(&b'\n') {
                return Err(bad("Invalid or oversized chunk row"));
            }
            hash.update(&line);
            size = size
                .checked_add(read as u64)
                .ok_or_else(|| bad("Chunk size overflow"))?;
            if size > chunk.size_bytes {
                return Err(bad("Chunk exceeds declared size"));
            }
            let row: Row = serde_json::from_slice(&line).map_err(|_| bad("Invalid chunk row"))?;
            insert_row(&tx, &chunk.section, row)?;
            rows += 1;
        }
        if size != chunk.size_bytes || rows != chunk.rows || hex(&hash.finalize()) != chunk.sha256 {
            return Err(bad("Chunk checksum or row count mismatch"));
        }
    }
    let mut state = state_write(&tx)?;
    if state.workspace_id != manifest.workspace_id || state.generation != manifest.generation {
        return Err(bad("Manifest and canonical identity mismatch"));
    }
    let mut retained: Vec<String> = {
        let meta = tx.open_table(META).map_err(storage_error)?;
        meta.get("retained_generations")
            .map_err(storage_error)?
            .map(|b| decode(b.value()))
            .transpose()?
            .unwrap_or_default()
    };
    let unique: BTreeSet<_> = retained.iter().cloned().collect();
    if unique.len() != retained.len()
        || retained
            .iter()
            .any(|g| contracts::id(g).is_err() || g == &state.generation)
    {
        return Err(bad("Invalid retained generation registry"));
    }
    retained.push(state.generation.clone());
    state.generation = Uuid::new_v4().to_string();
    state.epoch = 1;
    {
        let mut meta = tx.open_table(META).map_err(storage_error)?;
        meta.insert("state", encode(&state)?.as_slice())
            .map_err(storage_error)?;
        meta.insert("retained_generations", encode(&retained)?.as_slice())
            .map_err(storage_error)?;
    }
    indexes::rebuild(&tx)?;
    compat::rebuild(&tx).map_err(storage_error)?;
    hook(BackupRestoreStage::Staged);
    hook(BackupRestoreStage::BeforeCommit);
    tx.commit().map_err(storage_error)?;
    {
        let read = store.db.begin_read().map_err(storage_error)?;
        super::backup_audit::audit_read(&read)?;
        inbox::audit_read(&read)?;
        compat::validate_restored(&read).map_err(storage_error)?;
        validate_blobs(&read)?;
        media::validate(&read)?;
    }
    hook(BackupRestoreStage::Committed);
    Ok((store, report(&manifest)))
}
fn artifact_draft(draft: &Value, tx: &ReadTransaction) -> Result<()> {
    if draft["kind"] != "evidence" || draft["source"]["type"] != "artifact" {
        return Ok(());
    }
    let source = &draft["source"];
    let hash = source["sha256"]
        .as_str()
        .ok_or_else(|| bad("Invalid artifact reference"))?;
    let size = source["sizeBytes"]
        .as_u64()
        .ok_or_else(|| bad("Invalid artifact reference size"))?;
    let table = tx
        .open_table(BLOBS)
        .map_err(|_| bad("Referenced artifact bytes are missing; ingest blobs before backup"))?;
    let info: BlobInfo = decode(
        table
            .get(hash)
            .map_err(storage_error)?
            .ok_or_else(|| {
                bad("Referenced artifact bytes are missing; ingest blobs before backup")
            })?
            .value(),
    )?;
    if info.sha256 != hash || info.size_bytes != size {
        return Err(bad("Referenced artifact size or digest mismatch"));
    }
    Ok(())
}
fn artifact_items(items: &Value, tx: &ReadTransaction) -> Result<()> {
    if let Some(items) = items.as_array() {
        for item in items {
            artifact_draft(&item["draft"], tx)?;
        }
    }
    Ok(())
}
fn artifact_refs(value: &Value, tx: &ReadTransaction) -> Result<()> {
    artifact_draft(&value["draft"], tx)?;
    if let Some(records) = value["records"].as_array() {
        for record in records {
            artifact_draft(&record["draft"], tx)?;
        }
    }
    artifact_items(&value["reviewed_items"], tx)?;
    artifact_items(&value["original"]["items"], tx)?;
    artifact_items(&value["request"]["items"], tx)?;
    artifact_items(&value["request"]["reviewedItems"], tx)?;
    Ok(())
}
fn validate_blobs(tx: &ReadTransaction) -> Result<()> {
    let present: BTreeSet<_> = tx
        .list_tables()
        .map_err(storage_error)?
        .map(|t| t.name().to_owned())
        .collect();
    let mut expected_chunks = 0u64;
    if present.contains(BLOBS.name()) {
        let meta = tx.open_table(BLOBS).map_err(storage_error)?;
        let chunks = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
        for entry in meta.iter().map_err(storage_error)? {
            let (key, bytes) = entry.map_err(storage_error)?;
            let info: BlobInfo = decode(bytes.value())?;
            if info.sha256 != key.value()
                || !contracts::hex_digest(&info.sha256)
                || info.size_bytes == 0
                || info.chunks != info.size_bytes.div_ceil(BLOB_CHUNK as u64)
            {
                return Err(bad("Invalid blob metadata"));
            }
            let mut hash = Sha256::new();
            let mut size = 0u64;
            for index in 0..info.chunks {
                let bytes = chunks
                    .get((key.value(), index))
                    .map_err(storage_error)?
                    .ok_or_else(|| bad("Missing blob chunk"))?;
                let expected = if index + 1 == info.chunks {
                    info.size_bytes - size
                } else {
                    BLOB_CHUNK as u64
                };
                if bytes.value().len() as u64 != expected {
                    return Err(bad("Invalid blob chunk size"));
                }
                hash.update(bytes.value());
                size += bytes.value().len() as u64;
            }
            if size != info.size_bytes || hex(&hash.finalize()) != info.sha256 {
                return Err(bad("Blob content checksum mismatch"));
            }
            expected_chunks = expected_chunks
                .checked_add(info.chunks)
                .ok_or_else(|| bad("Blob count overflow"))?;
        }
    }
    if present.contains(BLOB_BYTES.name()) {
        use redb::ReadableTableMetadata;
        if tx
            .open_table(BLOB_BYTES)
            .map_err(storage_error)?
            .len()
            .map_err(storage_error)?
            != expected_chunks
        {
            return Err(bad("Unreferenced or missing blob chunks"));
        }
    }
    for definition in [CURRENT, RECEIPTS, INBOX, REVIEWS] {
        for entry in tx
            .open_table(definition)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (_, bytes) = entry.map_err(storage_error)?;
            artifact_refs(&decode::<Value>(bytes.value())?, tx)?;
        }
    }
    for entry in tx
        .open_table(history::HISTORY)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (_, bytes) = entry.map_err(storage_error)?;
        artifact_refs(&decode::<Value>(bytes.value())?, tx)?;
    }
    Ok(())
}
