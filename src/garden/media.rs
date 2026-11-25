//! Owner-controlled visual evidence: append-only provenance, owner attachment,
//! verification and separately scoped metadata and byte reads.
//!
//! Original bytes live in the content-addressed blob tables of the same database
//! and are never rewritten. Interpretations are ordinary attributed assertions.
#[path = "media_backfill.rs"]
mod backfill;
use super::{
    CURRENT, GardenStore, Principal, Result,
    backup::{BLOB_BYTES, BLOBS, BlobInfo, put_blob_tx},
    context_read, context_write, decode, encode, envelope, failure, inbox, indexes, operation_key,
    persist_batch, principal_read, state_read, state_write, storage_error, visible,
};
use crate::{
    access::AccessMode,
    contracts::{self, Auth},
    garden::{access::GardenAction, contracts::*},
};
pub use backfill::{AttachmentAuthorization, AttachmentPlan};
use redb::{Durability, ReadableDatabase, ReadableTable, TableDefinition, TableHandle};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{Read, Write},
    path::Path,
};
use uuid::Uuid;

pub(crate) const ASSETS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_media_assets");
pub(crate) const ACQUISITIONS: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_media_acquisitions");
pub(crate) const DERIVATIVES: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_media_derivatives");
/// Transient record of an unpublished large ingest; never exported or restored.
pub(crate) const STAGES: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_media_stages");
/// Recordings above this size are staged unless the owner chooses otherwise.
pub const DEFAULT_STAGE_ABOVE: u64 = 64 * 1024 * 1024;
/// Chunks (1 MiB each) written per staging transaction.
const STAGE_BATCH_CHUNKS: u64 = 16;
const CHUNK: u64 = 1024 * 1024;

/// Source URI marking a proposed attachment whose original the owner has not supplied.
pub const PENDING_URI: &str = "urn:mother:media:pending";
pub const PENDING_EXTENSION: &str = "mother.media:pending";
pub const USE_EXTENSION: &str = "mother.media:use";
pub const INTERPRETATION_EXTENSION: &str = "mother.media:interpretation";
pub const INTERPRETATION_PREDICATES: [&str; 4] = [
    "mother.media:caption",
    "mother.media:description",
    "mother.media:ocr_text",
    "mother.media:transcript",
];
/// Default per-asset ingest cap; owners may lower or raise it explicitly.
pub const DEFAULT_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// Largest image returned through MCP; base64 must fit the local service frame.
pub const MAX_REPLY_BYTES: u64 = 2 * 1024 * 1024;
pub const LIST_TOOL: &str = "mother_garden_media_list";
pub const GET_TOOL: &str = "mother_garden_media_get";
const MAX_LIST: usize = 50;
const SCAN_BUDGET: usize = 500;
const MAX_ACQUISITIONS_SHOWN: usize = 20;
const MAX_REPORT: usize = 1000;
const EXPORT_FORMAT: &str = "mother-media-export";
const MAX_EXPORT_MANIFEST: usize = 16 * 1024 * 1024;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportManifest {
    format: String,
    version: u32,
    assets: Vec<ExportedAsset>,
    #[serde(default)]
    missing: Vec<Value>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportedAsset {
    asset: MediaAsset,
    file: Option<String>,
    acquisitions: Vec<Acquisition>,
    derivatives: Vec<String>,
}
fn file_name(asset: &MediaAsset) -> Result<String> {
    let extension = match asset.media_type.as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/heic" => "heic",
        "image/heif" => "heif",
        "image/avif" => "avif",
        "video/mp4" => "mp4",
        "video/quicktime" => "mov",
        "video/webm" => "webm",
        "video/x-matroska" => "mkv",
        "video/x-msvideo" => "avi",
        "audio/mp4" => "m4a",
        "audio/mpeg" => "mp3",
        "audio/wav" => "wav",
        "audio/ogg" => "ogg",
        "audio/flac" => "flac",
        _ => return Err(invalid("Unsupported stored media type")),
    };
    Ok(format!("{}.{extension}", asset.sha256))
}

fn invalid(message: &str) -> GardenError {
    failure(GardenErrorCode::InvalidInput, message)
}
fn forbidden(message: &str) -> GardenError {
    failure(GardenErrorCode::Forbidden, message)
}

/// How the owner obtained the bytes. AI-generated images are labelled as such.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaOrigin {
    Captured,
    Uploaded,
    Imported,
    Screenshot,
    AiGenerated,
}
impl MediaOrigin {
    /// Parse the owner CLI spelling.
    /// # Errors
    /// Rejects unknown origins.
    pub fn parse(value: &str) -> Result<Self> {
        serde_json::from_value(json!(value)).map_err(|_| invalid("Unknown media origin"))
    }
}

/// Facts derived from stored bytes, one row per content address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MediaAsset {
    pub sha256: String,
    pub size_bytes: u64,
    pub media_type: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub embedded_metadata: Vec<String>,
    pub first_ingested_at: String,
}

/// One append-only owner ingest event. The same bytes may have many.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Acquisition {
    pub id: String,
    pub sha256: String,
    pub acquired_at: String,
    pub original_filename: Option<String>,
    pub event_time: Option<ObservedTime>,
    pub source: Option<String>,
    pub origin: MediaOrigin,
    pub derived_from: Option<String>,
    pub actor: String,
}

/// Owner-supplied provenance for an ingest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcquisitionInput {
    pub original_filename: Option<String>,
    pub event_time: Option<ObservedTime>,
    pub source: Option<String>,
    pub origin: MediaOrigin,
    pub declared_media_type: Option<String>,
}
impl AcquisitionInput {
    pub fn new(origin: MediaOrigin) -> Self {
        Self {
            original_filename: None,
            event_time: None,
            source: None,
            origin,
            declared_media_type: None,
        }
    }
    fn validate(&self) -> Result<()> {
        if let Some(name) = &self.original_filename
            && (name.is_empty()
                || name.len() > 255
                || name.contains(['/', '\\', '\0'])
                || name == "."
                || name == "..")
        {
            return Err(invalid(
                "Original filename must be a bare name of 1 to 255 bytes",
            ));
        }
        if self
            .source
            .as_ref()
            .is_some_and(|s| s.trim().is_empty() || s.len() > 1000 || s.contains('\0'))
        {
            return Err(invalid("Source description must be 1 to 1000 bytes"));
        }
        if let Some(time) = &self.event_time {
            time.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IngestOptions {
    pub max_bytes: u64,
    pub strip_metadata: bool,
    /// Recordings larger than this are staged across bounded transactions.
    pub stage_above: u64,
}
impl Default for IngestOptions {
    fn default() -> Self {
        Self {
            max_bytes: DEFAULT_MAX_BYTES,
            strip_metadata: false,
            stage_above: DEFAULT_STAGE_ABOVE,
        }
    }
}

/// A large recording written in bounded transactions, awaiting its atomic publish.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Stage {
    sha256: String,
    size_bytes: u64,
    chunks: u64,
    written: u64,
    started_at: String,
}
/// Work done before the publishing transaction opens.
#[derive(Debug, Clone)]
pub(super) enum Prepared {
    /// Small files and images: ingest entirely inside the publishing transaction.
    Inline,
    /// A large recording: chunks already written (or already stored, `stage: None`).
    Staged {
        stage: Option<String>,
        sha256: String,
        size_bytes: u64,
        found: crate::media::Inspection,
        /// Chunks reused from an interrupted stage of the same bytes.
        resumed: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivativeReport {
    pub sha256: String,
    pub size_bytes: u64,
    pub acquisition_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestReport {
    pub sha256: String,
    pub size_bytes: u64,
    pub media_type: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub embedded_metadata: Vec<String>,
    pub acquisition_id: String,
    pub reused: bool,
    pub derivative: Option<DerivativeReport>,
    /// Stripping was requested but this format is never rewritten; the original is kept.
    pub derivative_unsupported: bool,
    /// Chunks reused from an interrupted staged ingest of the same bytes.
    pub resumed_chunks: u64,
}

/// Bytes the owner attaches: a fresh file, or an asset already in the Garden.
#[derive(Debug, Clone)]
pub enum AttachSource<'a> {
    File {
        path: &'a Path,
        input: AcquisitionInput,
        options: IngestOptions,
    },
    Existing {
        sha256: String,
    },
}
/// Owner completion of a harness-proposed attachment with the original file.
#[derive(Debug, Clone)]
pub struct AttachmentReview<'a> {
    pub path: &'a Path,
    pub input: AcquisitionInput,
    pub options: IngestOptions,
    /// Record the proposer's description as an attributed interpretation.
    pub accept_description: bool,
}
#[derive(Debug, Clone)]
pub struct AttachRequest<'a> {
    pub source: AttachSource<'a>,
    pub target: String,
    pub expected_revision: u64,
    pub note: Option<String>,
}

/// Validated payload of a harness-proposed attachment awaiting the owner's original.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PendingAttachment {
    pub target: ExternalReference,
    pub expected_revision: u64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub event_time: Option<ObservedTime>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Producer {
    kind: ProducerKind,
    name: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ProducerKind {
    Model,
    Person,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Interpretation {
    producer: Producer,
    method: String,
    #[serde(default)]
    produced_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UsePayload {
    #[serde(default)]
    acquisition_id: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

fn bounded(text: &str, max: usize, message: &str) -> Result<()> {
    if text.trim().is_empty() || text.len() > max || text.contains('\0') {
        return Err(invalid(message));
    }
    Ok(())
}
fn extension_payload<T: for<'a> Deserialize<'a>>(
    extensions: &Extensions,
    name: &str,
    message: &str,
) -> Result<Option<T>> {
    extensions
        .get(name)
        .map(|extension| {
            if extension.schema_version != 1 {
                return Err(failure(GardenErrorCode::UnsupportedExtension, message));
            }
            serde_json::from_value(extension.payload.clone()).map_err(|_| invalid(message))
        })
        .transpose()
}
fn pending_of(draft: &GardenDraft) -> Result<Option<PendingAttachment>> {
    let GardenDraft::Evidence {
        source, extensions, ..
    } = draft
    else {
        return Ok(None);
    };
    let marked = matches!(source, SourceLocator::Uri { uri } if uri == PENDING_URI);
    let payload: Option<PendingAttachment> =
        extension_payload(extensions, PENDING_EXTENSION, "Invalid pending attachment")?;
    match (marked, payload) {
        (false, None) => Ok(None),
        (true, Some(payload)) => {
            if payload.target.kind != GardenKind::Assertion
                || payload.expected_revision == 0
                || payload.expected_revision > MAX_SAFE_INTEGER
            {
                return Err(invalid(
                    "Pending attachments target an existing assertion revision",
                ));
            }
            payload.target.validate()?;
            if let Some(description) = &payload.description {
                bounded(
                    description,
                    4000,
                    "Pending description must be 1 to 4000 bytes",
                )?;
            }
            if let Some(time) = &payload.event_time {
                time.validate()?;
            }
            Ok(Some(payload))
        }
        _ => Err(invalid(
            "Pending attachments need both the pending source URI and extension",
        )),
    }
}
/// Pending attachment carried by one proposal item, if any.
pub(super) fn pending_item(item: &BatchItem) -> Result<Option<PendingAttachment>> {
    item.draft.as_ref().map_or(Ok(None), pending_of)
}
fn check_interpretation_shape(draft: &GardenDraft) -> Result<()> {
    let GardenDraft::Assertion {
        predicate,
        object,
        evidence,
        confidence,
        extensions,
        ..
    } = draft
    else {
        return Ok(());
    };
    if !predicate.starts_with("mother.media:") {
        if extensions.contains_key(INTERPRETATION_EXTENSION) {
            return Err(invalid(
                "Interpretation attribution needs a media predicate",
            ));
        }
        return Ok(());
    }
    if !INTERPRETATION_PREDICATES.contains(&predicate.as_str()) {
        return Err(invalid("Unknown media interpretation predicate"));
    }
    if !matches!(
        object,
        AssertionObject::Value {
            value: TypedValue::String { .. }
        }
    ) {
        return Err(invalid("Media interpretations hold a string value"));
    }
    if evidence.is_empty() {
        return Err(invalid("Media interpretations cite the image evidence"));
    }
    let attribution: Interpretation = extension_payload(
        extensions,
        INTERPRETATION_EXTENSION,
        "Invalid interpretation attribution",
    )?
    .ok_or_else(|| invalid("Media interpretations require producer attribution"))?;
    bounded(
        &attribution.producer.name,
        200,
        "Producer name must be 1 to 200 bytes",
    )?;
    bounded(
        &attribution.method,
        200,
        "Interpretation method must be 1 to 200 bytes",
    )?;
    if let Some(at) = &attribution.produced_at {
        chrono::DateTime::parse_from_rfc3339(at)
            .map_err(|_| invalid("producedAt must be an RFC 3339 instant"))?;
    }
    if attribution.producer.kind == ProducerKind::Model
        && confidence
            .as_ref()
            .is_some_and(|c| c.basis != ConfidenceBasis::ModelInference)
    {
        return Err(invalid(
            "Model interpretations use model_inference confidence",
        ));
    }
    Ok(())
}

fn asset_read(
    table: &impl ReadableTable<&'static str, &'static [u8]>,
    sha: &str,
) -> Result<Option<MediaAsset>> {
    table
        .get(sha)
        .map_err(storage_error)?
        .map(|v| decode(v.value()))
        .transpose()
}
fn prefixed(
    table: &impl ReadableTable<&'static str, &'static [u8]>,
    prefix: &str,
) -> Result<Vec<(String, Vec<u8>)>> {
    let end = format!("{prefix}\u{10ffff}");
    let mut out = Vec::new();
    for entry in table.range(prefix..end.as_str()).map_err(storage_error)? {
        let (key, value) = entry.map_err(storage_error)?;
        out.push((key.value().to_owned(), value.value().to_owned()));
    }
    Ok(out)
}
fn acquisitions(
    table: &impl ReadableTable<&'static str, &'static [u8]>,
    sha: &str,
) -> Result<Vec<Acquisition>> {
    prefixed(table, &format!("{sha}/"))?
        .into_iter()
        .map(|(_, v)| decode(&v))
        .collect()
}
fn derivatives(
    table: &impl ReadableTable<&'static str, &'static [u8]>,
    sha: &str,
) -> Result<Vec<String>> {
    Ok(prefixed(table, &format!("{sha}/"))?
        .into_iter()
        .filter_map(|(k, _)| k.split_once('/').map(|(_, d)| d.to_owned()))
        .collect())
}
/// Active evidence IDs citing exactly this stored asset, via the canonical source index.
fn uses(tx: &redb::ReadTransaction, asset: &MediaAsset) -> Result<Vec<String>> {
    let source = SourceLocator::Artifact {
        sha256: asset.sha256.clone(),
        size_bytes: asset.size_bytes,
        media_type: asset.media_type.clone(),
    };
    let full = json!([source, ""]).to_string();
    let prefix = &full[..full.len() - 3];
    let table = tx.open_table(indexes::SOURCES).map_err(storage_error)?;
    let end = format!("{prefix}\u{10ffff}");
    let mut ids = Vec::new();
    for entry in table.range(prefix..end.as_str()).map_err(storage_error)? {
        let (key, _) = entry.map_err(storage_error)?;
        let parsed: Value = serde_json::from_str(key.value()).map_err(storage_error)?;
        if parsed[0] == json!(source)
            && let Some(id) = parsed[1].as_str()
        {
            ids.push(id.to_owned());
        }
    }
    Ok(ids)
}
const MAX_INTERPRETATIONS_SHOWN: usize = 20;
/// Active interpretation assertions citing the given evidence. With a principal,
/// only interpretations that principal can currently see are returned.
fn interpretations(
    tx: &redb::ReadTransaction,
    principal: Option<&Principal>,
    evidence_ids: &[String],
) -> Result<Vec<Value>> {
    let index = tx.open_table(indexes::EVIDENCE).map_err(storage_error)?;
    let current = tx.open_table(CURRENT).map_err(storage_error)?;
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for evidence in evidence_ids {
        let full = json!([{"kind":"evidence","id":evidence}, ""]).to_string();
        let prefix = &full[..full.len() - 3];
        let end = format!("{prefix}\u{10ffff}");
        for entry in index.range(prefix..end.as_str()).map_err(storage_error)? {
            let (key, _) = entry.map_err(storage_error)?;
            let parsed: Value = serde_json::from_str(key.value()).map_err(storage_error)?;
            let Some(id) = parsed[1].as_str() else {
                continue;
            };
            if !seen.insert(id.to_owned()) {
                continue;
            }
            let Some(bytes) = current.get(id).map_err(storage_error)? else {
                continue;
            };
            let record: GardenRecord = decode(bytes.value())?;
            let GardenDraft::Assertion {
                predicate,
                object:
                    AssertionObject::Value {
                        value: TypedValue::String { value },
                    },
                extensions,
                ..
            } = &record.draft
            else {
                continue;
            };
            if !INTERPRETATION_PREDICATES.contains(&predicate.as_str())
                || record.metadata.lifecycle != Lifecycle::Active
            {
                continue;
            }
            if let Some(principal) = principal {
                let Ok((context, replacements)) = context_read(tx, principal, vec![id.to_owned()])
                else {
                    continue;
                };
                if visible(principal, &record, &context, &replacements).is_err() {
                    continue;
                }
            }
            let attribution = extensions
                .get(INTERPRETATION_EXTENSION)
                .map_or(Value::Null, |e| e.payload.clone());
            out.push(json!({"assertionId":id,"revision":record.metadata.revision,
                "predicate":predicate,"text":value,"attribution":attribution,
                "contributor":record.metadata.contributor,"approver":record.metadata.approver}));
            if out.len() == MAX_INTERPRETATIONS_SHOWN {
                return Ok(out);
            }
        }
    }
    Ok(out)
}
fn read_bytes(tx: &redb::ReadTransaction, sha: &str) -> Result<Vec<u8>> {
    let meta = tx.open_table(BLOBS).map_err(storage_error)?;
    let info: BlobInfo = decode(
        meta.get(sha)
            .map_err(storage_error)?
            .ok_or_else(|| failure(GardenErrorCode::NotFound, "Media bytes are missing"))?
            .value(),
    )?;
    let chunks = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
    let mut bytes = Vec::with_capacity(usize::try_from(info.size_bytes).unwrap_or(0));
    for index in 0..info.chunks {
        bytes.extend_from_slice(
            chunks
                .get((sha, index))
                .map_err(storage_error)?
                .ok_or_else(|| storage_error("missing media chunk"))?
                .value(),
        );
    }
    if bytes.len() as u64 != info.size_bytes || hex(&Sha256::digest(&bytes)) != sha {
        return Err(failure(
            GardenErrorCode::StoreUnavailable,
            "Stored media failed digest verification",
        ));
    }
    Ok(bytes)
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn read_owner_file(path: &Path, max: u64) -> Result<Vec<u8>> {
    if max == 0 || max > MAX_SAFE_INTEGER {
        return Err(invalid("Invalid media size cap"));
    }
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|_| invalid("Media file could not be opened"))?
        .take(max + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid("Media file could not be read"))?;
    if bytes.is_empty() {
        return Err(invalid("Empty media file"));
    }
    if bytes.len() as u64 > max {
        return Err(failure(
            GardenErrorCode::LimitExceeded,
            "Media file exceeds the ingest size cap",
        ));
    }
    Ok(bytes)
}
fn store_asset(
    tx: &redb::WriteTransaction,
    info: &BlobInfo,
    found: &crate::media::Inspection,
    now: &str,
) -> Result<bool> {
    let mut table = tx.open_table(ASSETS).map_err(storage_error)?;
    if let Some(existing) = asset_read(&table, &info.sha256)? {
        if existing.size_bytes != info.size_bytes || existing.media_type != found.media_type {
            return Err(storage_error("media asset metadata conflict"));
        }
        return Ok(true);
    }
    let asset = MediaAsset {
        sha256: info.sha256.clone(),
        size_bytes: info.size_bytes,
        media_type: found.media_type.into(),
        width: found.width,
        height: found.height,
        embedded_metadata: found.embedded.iter().map(|s| (*s).to_owned()).collect(),
        first_ingested_at: now.into(),
    };
    table
        .insert(info.sha256.as_str(), encode(&asset)?.as_slice())
        .map_err(storage_error)?;
    Ok(false)
}
fn append_acquisition(
    tx: &redb::WriteTransaction,
    sha: &str,
    input: &AcquisitionInput,
    derived_from: Option<&str>,
    now: &str,
) -> Result<String> {
    let acquisition = Acquisition {
        id: Uuid::new_v4().to_string(),
        sha256: sha.into(),
        acquired_at: now.into(),
        original_filename: input.original_filename.clone(),
        event_time: input.event_time.clone(),
        source: input.source.clone(),
        origin: input.origin,
        derived_from: derived_from.map(str::to_owned),
        actor: inbox::OWNER.into(),
    };
    let key = format!("{sha}/{}", acquisition.id);
    let mut table = tx.open_table(ACQUISITIONS).map_err(storage_error)?;
    if table.get(key.as_str()).map_err(storage_error)?.is_some() {
        return Err(storage_error("acquisition identity collision"));
    }
    table
        .insert(key.as_str(), encode(&acquisition)?.as_slice())
        .map_err(storage_error)?;
    Ok(acquisition.id)
}

/// Ingest owner bytes and provenance inside a caller-owned write transaction.
pub(super) fn ingest_tx(
    tx: &redb::WriteTransaction,
    bytes: &[u8],
    input: &AcquisitionInput,
    options: IngestOptions,
    now: &str,
) -> Result<IngestReport> {
    input.validate()?;
    if bytes.is_empty() || bytes.len() as u64 > options.max_bytes {
        return Err(failure(
            GardenErrorCode::LimitExceeded,
            "Media bytes are empty or exceed the ingest size cap",
        ));
    }
    let found = crate::media::inspect(bytes).map_err(invalid)?;
    if input
        .declared_media_type
        .as_deref()
        .is_some_and(|declared| declared != found.media_type)
    {
        return Err(invalid(
            "Declared media type does not match the image bytes",
        ));
    }
    let info = put_blob_tx(tx, bytes)?;
    let reused = store_asset(tx, &info, &found, now)?;
    let acquisition_id = append_acquisition(tx, &info.sha256, input, None, now)?;
    let mut derivative = None;
    let unsupported = options.strip_metadata && !found.embedded.is_empty() && !found.strippable;
    if options.strip_metadata && !found.embedded.is_empty() && found.strippable {
        let clean = crate::media::strip(bytes).map_err(invalid)?;
        let checked = crate::media::inspect(&clean).map_err(invalid)?;
        if !checked.embedded.is_empty() || checked.media_type != found.media_type {
            return Err(storage_error("derivative still carries metadata"));
        }
        let clean_info = put_blob_tx(tx, clean.as_slice())?;
        store_asset(tx, &clean_info, &checked, now)?;
        let id = append_acquisition(tx, &clean_info.sha256, input, Some(&info.sha256), now)?;
        tx.open_table(DERIVATIVES)
            .map_err(storage_error)?
            .insert(
                format!("{}/{}", info.sha256, clean_info.sha256).as_str(),
                [].as_slice(),
            )
            .map_err(storage_error)?;
        derivative = Some(DerivativeReport {
            sha256: clean_info.sha256,
            size_bytes: clean_info.size_bytes,
            acquisition_id: id,
        });
    }
    Ok(IngestReport {
        sha256: info.sha256,
        size_bytes: info.size_bytes,
        media_type: found.media_type.into(),
        width: found.width,
        height: found.height,
        embedded_metadata: found.embedded.iter().map(|s| (*s).to_owned()).collect(),
        acquisition_id,
        reused,
        derivative,
        derivative_unsupported: unsupported,
        resumed_chunks: 0,
    })
}
/// Read and ingest an owner-selected file inside a caller-owned transaction.
/// Images are read whole for inspection; recordings are streamed into the blob
/// store and inspected from both ends, so memory stays bounded.
pub(super) fn ingest_file_tx(
    tx: &redb::WriteTransaction,
    path: &Path,
    input: &AcquisitionInput,
    options: IngestOptions,
    now: &str,
    prepared: &Prepared,
) -> Result<IngestReport> {
    use std::io::{Seek, SeekFrom};
    if let Prepared::Staged {
        stage,
        sha256,
        size_bytes,
        found,
        resumed,
    } = prepared
    {
        return publish_staged_tx(
            tx,
            stage.as_deref(),
            sha256,
            *size_bytes,
            found,
            input,
            options,
            now,
            *resumed,
        );
    }
    if options.max_bytes == 0 || options.max_bytes > MAX_SAFE_INTEGER {
        return Err(invalid("Invalid media size cap"));
    }
    let unreadable = |_| invalid("Media file could not be read");
    let mut file = File::open(path).map_err(|_| invalid("Media file could not be opened"))?;
    let length = file.metadata().map_err(unreadable)?.len();
    if length == 0 {
        return Err(invalid("Empty media file"));
    }
    if length > options.max_bytes {
        return Err(failure(
            GardenErrorCode::LimitExceeded,
            "Media file exceeds the ingest size cap",
        ));
    }
    let scan = crate::media::SCAN_BYTES as u64;
    let mut head = Vec::new();
    (&mut file)
        .take(scan)
        .read_to_end(&mut head)
        .map_err(unreadable)?;
    if !crate::media::sniff(&head).is_some_and(crate::media::is_recording) {
        drop(file);
        let bytes = read_owner_file(path, options.max_bytes)?;
        return ingest_tx(tx, &bytes, input, options, now);
    }
    input.validate()?;
    let mut tail = Vec::new();
    file.seek(SeekFrom::Start(length.saturating_sub(scan)))
        .map_err(unreadable)?;
    (&mut file)
        .take(scan)
        .read_to_end(&mut tail)
        .map_err(unreadable)?;
    let found = crate::media::inspect_recording(&head, &tail).map_err(invalid)?;
    if input
        .declared_media_type
        .as_deref()
        .is_some_and(|declared| declared != found.media_type)
    {
        return Err(invalid(
            "Declared media type does not match the recording bytes",
        ));
    }
    file.seek(SeekFrom::Start(0)).map_err(unreadable)?;
    let info = put_blob_tx(tx, (&mut file).take(options.max_bytes + 1))?;
    if info.size_bytes != length {
        return Err(invalid("Media file changed while it was being read"));
    }
    let reused = store_asset(tx, &info, &found, now)?;
    let acquisition_id = append_acquisition(tx, &info.sha256, input, None, now)?;
    Ok(IngestReport {
        sha256: info.sha256,
        size_bytes: info.size_bytes,
        media_type: found.media_type.into(),
        width: None,
        height: None,
        embedded_metadata: found.embedded.iter().map(|s| (*s).to_owned()).collect(),
        acquisition_id,
        reused,
        derivative: None,
        derivative_unsupported: options.strip_metadata && !found.embedded.is_empty(),
        resumed_chunks: 0,
    })
}
/// Publish a staged (or already stored) recording in the caller's transaction.
#[allow(clippy::too_many_arguments)]
fn publish_staged_tx(
    tx: &redb::WriteTransaction,
    stage: Option<&str>,
    sha256: &str,
    size_bytes: u64,
    found: &crate::media::Inspection,
    input: &AcquisitionInput,
    options: IngestOptions,
    now: &str,
    resumed: u64,
) -> Result<IngestReport> {
    input.validate()?;
    if input
        .declared_media_type
        .as_deref()
        .is_some_and(|declared| declared != found.media_type)
    {
        return Err(invalid(
            "Declared media type does not match the recording bytes",
        ));
    }
    let info = BlobInfo {
        sha256: sha256.to_owned(),
        size_bytes,
        chunks: size_bytes.div_ceil(CHUNK),
    };
    let mut blobs = tx.open_table(BLOBS).map_err(storage_error)?;
    if let Some(id) = stage {
        let mut stages = tx.open_table(STAGES).map_err(storage_error)?;
        let staged: Stage = decode(
            stages
                .get(id)
                .map_err(storage_error)?
                .ok_or_else(|| storage_error("staged ingest disappeared"))?
                .value(),
        )?;
        if staged.sha256 != sha256
            || staged.written != staged.chunks
            || staged.chunks != info.chunks
        {
            return Err(storage_error("staged ingest is incomplete"));
        }
        stages.remove(id).map_err(storage_error)?;
        if blobs.get(sha256).map_err(storage_error)?.is_none() {
            blobs
                .insert(sha256, encode(&info)?.as_slice())
                .map_err(storage_error)?;
        }
    } else if blobs.get(sha256).map_err(storage_error)?.is_none() {
        return Err(storage_error("reused recording bytes disappeared"));
    }
    drop(blobs);
    let reused = store_asset(tx, &info, found, now)?;
    let acquisition_id = append_acquisition(tx, sha256, input, None, now)?;
    Ok(IngestReport {
        sha256: sha256.to_owned(),
        size_bytes,
        media_type: found.media_type.into(),
        width: None,
        height: None,
        embedded_metadata: found.embedded.iter().map(|s| (*s).to_owned()).collect(),
        acquisition_id,
        reused,
        derivative: None,
        derivative_unsupported: options.strip_metadata && !found.embedded.is_empty(),
        resumed_chunks: resumed,
    })
}
/// Evidence draft citing a stored asset, with per-use context.
pub(super) fn evidence_draft(
    tx: &redb::WriteTransaction,
    sha: &str,
    acquisition_id: Option<&str>,
    note: Option<&str>,
    event_time: Option<&ObservedTime>,
) -> Result<GardenDraft> {
    let table = tx.open_table(ASSETS).map_err(storage_error)?;
    let asset = asset_read(&table, sha)?
        .ok_or_else(|| failure(GardenErrorCode::NotFound, "Media asset does not exist"))?;
    if let Some(note) = note {
        bounded(note, 2000, "Use note must be 1 to 2000 bytes")?;
    }
    let payload = UsePayload {
        acquisition_id: acquisition_id.map(str::to_owned),
        note: note.map(str::to_owned),
    };
    Ok(GardenDraft::Evidence {
        source: SourceLocator::Artifact {
            sha256: asset.sha256,
            size_bytes: asset.size_bytes,
            media_type: asset.media_type,
        },
        excerpt: None,
        observed_time: event_time.cloned().unwrap_or(ObservedTime {
            at: None,
            source_timezone: None,
            precision: TimePrecision::Unknown,
        }),
        extensions: BTreeMap::from([(
            USE_EXTENSION.to_owned(),
            Extension {
                schema_version: 1,
                payload: serde_json::to_value(payload).map_err(storage_error)?,
            },
        )]),
    })
}
/// Update item linking new evidence to an existing assertion at its expected revision.
pub(super) fn link_item(
    tx: &redb::WriteTransaction,
    item_key: &str,
    target: &str,
    expected_revision: u64,
    evidence_key: &str,
) -> Result<BatchItem> {
    let record: GardenRecord = decode(
        tx.open_table(CURRENT)
            .map_err(storage_error)?
            .get(target)
            .map_err(storage_error)?
            .ok_or_else(|| failure(GardenErrorCode::NotFound, "Graph object does not exist"))?
            .value(),
    )?;
    let mut draft = record.draft;
    let GardenDraft::Assertion { evidence, .. } = &mut draft else {
        return Err(invalid("Images attach to an assertion"));
    };
    if evidence.len() >= MAX_REFS {
        return Err(failure(
            GardenErrorCode::LimitExceeded,
            "Assertion already cites the maximum evidence",
        ));
    }
    evidence.push(RecordRef::Item {
        kind: GardenKind::Evidence,
        item_key: evidence_key.into(),
    });
    Ok(BatchItem {
        item_key: item_key.into(),
        expected_revision,
        mutation: Mutation::Update,
        id: Some(ExternalReference {
            kind: GardenKind::Assertion,
            id: target.into(),
        }),
        draft: Some(draft),
        replacement: None,
    })
}
/// Owner-added interpretation recording a proposer's description of an attached image.
pub(super) fn description_item(
    tx: &redb::WriteTransaction,
    item_key: &str,
    target: &str,
    evidence_key: &str,
    description: &str,
    proposer: &str,
) -> Result<BatchItem> {
    let record: GardenRecord = decode(
        tx.open_table(CURRENT)
            .map_err(storage_error)?
            .get(target)
            .map_err(storage_error)?
            .ok_or_else(|| failure(GardenErrorCode::NotFound, "Graph object does not exist"))?
            .value(),
    )?;
    let GardenDraft::Assertion { subject, .. } = record.draft else {
        return Err(invalid("Images attach to an assertion"));
    };
    let unknown = TimePrecision::Unknown;
    Ok(BatchItem {
        item_key: item_key.into(),
        expected_revision: 0,
        mutation: Mutation::Create,
        id: None,
        draft: Some(GardenDraft::Assertion {
            subject,
            predicate: "mother.media:description".into(),
            object: AssertionObject::Value {
                value: TypedValue::String {
                    value: description.into(),
                },
            },
            valid_time: ValidTime {
                valid_from: None,
                valid_until: None,
                source_timezone: None,
                precision: unknown,
            },
            observed_time: ObservedTime {
                at: None,
                source_timezone: None,
                precision: unknown,
            },
            evidence: vec![RecordRef::Item {
                kind: GardenKind::Evidence,
                item_key: evidence_key.into(),
            }],
            confidence: None,
            extensions: BTreeMap::from([(
                INTERPRETATION_EXTENSION.to_owned(),
                Extension {
                    schema_version: 1,
                    payload: json!({"producer":{"kind":"model","name":proposer},
                        "method":"conversation_description"}),
                },
            )]),
        }),
        replacement: None,
    })
}

/// Reject canonical drafts that would misrepresent media; called by the graph writer.
pub(super) fn check_canonical(
    tx: &redb::WriteTransaction,
    record: &GardenRecord,
    current: &BTreeMap<String, GardenRecord>,
) -> Result<()> {
    if pending_of(&record.draft)?.is_some() {
        return Err(invalid(
            "Pending attachments need the owner's original file before approval",
        ));
    }
    match &record.draft {
        GardenDraft::Evidence {
            source:
                SourceLocator::Artifact {
                    sha256,
                    size_bytes,
                    media_type,
                },
            extensions,
            ..
        } => {
            let table = tx.open_table(ASSETS).map_err(storage_error)?;
            if let Some(asset) = asset_read(&table, sha256)?
                && (asset.size_bytes != *size_bytes || &asset.media_type != media_type)
            {
                return Err(invalid(
                    "Artifact locator disagrees with the stored media size or type",
                ));
            }
            let _: Option<UsePayload> =
                extension_payload(extensions, USE_EXTENSION, "Invalid media use context")?;
        }
        GardenDraft::Assertion {
            evidence,
            predicate,
            ..
        } if predicate.starts_with("mother.media:") => {
            check_interpretation_shape(&record.draft)?;
            for reference in evidence {
                let RecordRef::Existing { id, .. } = reference else {
                    return Err(invalid("Unresolved interpretation evidence"));
                };
                let cited = current.get(id).ok_or_else(|| {
                    failure(GardenErrorCode::NotFound, "Graph object does not exist")
                })?;
                if !matches!(
                    cited.draft,
                    GardenDraft::Evidence {
                        source: SourceLocator::Artifact { .. },
                        ..
                    }
                ) {
                    return Err(invalid("Media interpretations cite image evidence only"));
                }
            }
        }
        _ => check_interpretation_shape(&record.draft)?,
    }
    Ok(())
}
/// Validate media-specific proposal content at admission, before owner review.
pub(super) fn check_proposal(
    tx: &redb::WriteTransaction,
    principal: &Principal,
    request: &BatchRequest,
) -> Result<()> {
    let mut pending = 0;
    for item in &request.items {
        let Some(draft) = &item.draft else { continue };
        check_interpretation_shape(draft)?;
        if let Some(attachment) = pending_of(draft)? {
            pending += 1;
            if pending > 1 || item.mutation != Mutation::Create {
                return Err(invalid(
                    "A proposal carries at most one new pending attachment",
                ));
            }
            if matches!(
                draft,
                GardenDraft::Evidence {
                    excerpt: Some(_),
                    ..
                }
            ) {
                return Err(invalid(
                    "Put the proposed description in the pending payload",
                ));
            }
            let (current, replacements) =
                context_write(tx, principal, vec![attachment.target.id.clone()])?;
            let target = current
                .get(&attachment.target.id)
                .filter(|r| r.draft.kind() == GardenKind::Assertion)
                .ok_or_else(|| failure(GardenErrorCode::NotFound, "Graph object does not exist"))?;
            visible(principal, target, &current, &replacements)?;
        }
    }
    Ok(())
}

const TABLES: [TableDefinition<'static, &'static str, &'static [u8]>; 4] =
    [ASSETS, ACQUISITIONS, DERIVATIVES, STAGES];
/// Backups refuse while an interrupted large-file ingest is staged.
pub(super) fn validate_stages(tx: &redb::ReadTransaction) -> Result<()> {
    let present = tx
        .list_tables()
        .map_err(storage_error)?
        .any(|t| t.name() == STAGES.name());
    if present
        && tx
            .open_table(STAGES)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
            .next()
            .is_some()
    {
        return Err(invalid(
            "An interrupted large-file ingest is staged; run garden media cleanup first",
        ));
    }
    Ok(())
}
/// Create empty media tables; opening an older store adds them without other changes.
pub(super) fn initialize(tx: &redb::WriteTransaction) -> Result<()> {
    for table in TABLES {
        tx.open_table(table).map_err(storage_error)?;
    }
    Ok(())
}
pub(super) fn needs_initialize(db: &redb::Database) -> Result<bool> {
    let tx = db.begin_read().map_err(storage_error)?;
    let present: BTreeSet<String> = tx
        .list_tables()
        .map_err(storage_error)?
        .map(|t| t.name().to_owned())
        .collect();
    Ok(TABLES.iter().any(|t| !present.contains(t.name())))
}
/// Report media tools in capability discovery; only granted ones become available.
pub fn add_capabilities(capabilities: &mut Value, granted: &[&str]) {
    let garden = &mut capabilities["garden"];
    if let Some(defined) = garden["definedTools"].as_array_mut() {
        defined.extend([json!(LIST_TOOL), json!(GET_TOOL)]);
    }
    if let Some(defined) = garden["definedCapabilities"].as_array_mut() {
        defined.push(json!("garden.media.v1"));
    }
    if garden["available"] == json!(true) && !granted.is_empty() {
        if let Some(available) = garden["availableTools"].as_array_mut() {
            available.extend(granted.iter().map(|t| json!(t)));
        }
        if let Some(listed) = garden["capabilities"].as_array_mut() {
            listed.push(json!("garden.media.v1"));
        }
    }
}
fn has_scope(principal: &Principal, scope: &str) -> bool {
    principal.scopes.iter().any(|s| s == scope)
}
/// Media scopes form a chain on top of graph read; shared clients never get bytes.
pub(crate) fn validate_scopes(
    scopes: &[String],
    shared: bool,
) -> std::result::Result<(), &'static str> {
    let has = |s: &str| scopes.iter().any(|x| x == s);
    if has("media:meta") && !has("garden:read") {
        return Err("media:meta requires garden:read");
    }
    if has("media:bytes") && !has("media:meta") {
        return Err("media:bytes requires media:meta");
    }
    if has("media:originals") && !has("media:bytes") {
        return Err("media:originals requires media:bytes");
    }
    if shared && (has("media:bytes") || has("media:originals")) {
        return Err("Shared clients cannot receive media bytes");
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ListRequest {
    version: u32,
    workspace_id: String,
    generation: String,
    #[serde(default)]
    sha256: Option<String>,
    #[serde(default)]
    after: Option<String>,
    #[serde(default)]
    limit: Option<u32>,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Variant {
    #[default]
    Auto,
    Original,
    Derivative,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GetRequest {
    version: u32,
    workspace_id: String,
    generation: String,
    sha256: String,
    #[serde(default)]
    variant: Variant,
}
fn parse<T: for<'a> Deserialize<'a>>(args: &Value) -> Result<T> {
    bounded_json(args)?;
    serde_json::from_value(args.clone()).map_err(|_| invalid("Invalid media request"))
}
fn digest(value: &str) -> Result<()> {
    if contracts::hex_digest(value) {
        Ok(())
    } else {
        Err(invalid("Invalid media digest"))
    }
}
fn visible_uses(
    tx: &redb::ReadTransaction,
    principal: &Principal,
    asset: &MediaAsset,
) -> Result<Vec<Value>> {
    let mut shown = Vec::new();
    for id in uses(tx, asset)? {
        let Ok((current, replacements)) = context_read(tx, principal, vec![id.clone()]) else {
            continue;
        };
        if let Some(record) = current.get(&id)
            && visible(principal, record, &current, &replacements).is_ok()
        {
            shown.push(json!({"evidenceId":id,"revision":record.metadata.revision}));
        }
    }
    Ok(shown)
}
fn authorize_media(
    tx: &redb::ReadTransaction,
    auth: &Auth,
    workspace: &str,
    generation: &str,
    scope: &str,
) -> Result<(super::State, Principal)> {
    let state = state_read(tx)?;
    envelope(&state, workspace, generation)?;
    let principal = principal_read(tx, auth, GardenAction::Read)?;
    if !has_scope(&principal, scope) {
        return Err(forbidden("Current media scope required"));
    }
    Ok((state, principal))
}

impl GardenStore {
    fn now(&self) -> String {
        self.clock
            .now()
            .to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
    }
    fn begin(&self) -> Result<redb::WriteTransaction> {
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        Ok(tx)
    }
    /// Owner-only ingest of one explicitly selected image file with provenance.
    /// # Errors
    /// Rejects unsupported, oversized, malformed or conflicting media; nothing is
    /// committed on failure.
    pub fn media_add(
        &self,
        path: &Path,
        input: &AcquisitionInput,
        options: IngestOptions,
    ) -> Result<IngestReport> {
        self.with_prepared(path, options, |prepared| {
            let tx = self.begin()?;
            let report = ingest_file_tx(&tx, path, input, options, &self.now(), prepared)?;
            tx.commit().map_err(storage_error)?;
            Ok(report)
        })
    }
    /// Stage a large recording before `publish` runs; discard the stage if it fails.
    pub(super) fn with_prepared<T>(
        &self,
        path: &Path,
        options: IngestOptions,
        publish: impl FnOnce(&Prepared) -> Result<T>,
    ) -> Result<T> {
        let prepared = self.prepare_file(path, options)?;
        let result = publish(&prepared);
        if result.is_err()
            && let Prepared::Staged {
                stage: Some(id), ..
            } = &prepared
        {
            let _ = self.discard_stage(id);
        }
        result
    }
    /// Hash a large recording, then write its chunks under that address in bounded
    /// transactions. Nothing references the chunks until the publishing transaction.
    fn prepare_file(&self, path: &Path, options: IngestOptions) -> Result<Prepared> {
        self.prepare_limited(path, options, None)
    }
    /// Find an unpublished stage of exactly these bytes, most advanced first.
    fn find_stage(&self, sha256: &str, size: u64) -> Result<Option<(String, Stage)>> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        let table = tx.open_table(STAGES).map_err(storage_error)?;
        let mut best: Option<(String, Stage)> = None;
        for entry in table.iter().map_err(storage_error)? {
            let (key, value) = entry.map_err(storage_error)?;
            let staged: Stage = decode(value.value())?;
            if staged.sha256 == sha256
                && staged.size_bytes == size
                && best
                    .as_ref()
                    .is_none_or(|(_, b)| staged.written > b.written)
            {
                best = Some((key.value().to_owned(), staged));
            }
        }
        Ok(best)
    }
    /// Stage with an optional batch limit; a limit leaves an unfinished stage behind.
    fn prepare_limited(
        &self,
        path: &Path,
        options: IngestOptions,
        batch_limit: Option<u64>,
    ) -> Result<Prepared> {
        use std::io::{Seek, SeekFrom};
        let unreadable = |_| invalid("Media file could not be read");
        let Ok(mut file) = File::open(path) else {
            return Ok(Prepared::Inline);
        };
        let length = file.metadata().map_err(unreadable)?.len();
        if length == 0 || length > options.max_bytes || length <= options.stage_above {
            return Ok(Prepared::Inline);
        }
        let scan = crate::media::SCAN_BYTES as u64;
        let mut head = Vec::new();
        (&mut file)
            .take(scan)
            .read_to_end(&mut head)
            .map_err(unreadable)?;
        if !crate::media::sniff(&head).is_some_and(crate::media::is_recording) {
            return Ok(Prepared::Inline);
        }
        let mut tail = Vec::new();
        file.seek(SeekFrom::Start(length.saturating_sub(scan)))
            .map_err(unreadable)?;
        (&mut file)
            .take(scan)
            .read_to_end(&mut tail)
            .map_err(unreadable)?;
        let found = crate::media::inspect_recording(&head, &tail).map_err(invalid)?;
        file.seek(SeekFrom::Start(0)).map_err(unreadable)?;
        let mut hash = Sha256::new();
        let mut buffer = vec![0; CHUNK as usize];
        let mut total = 0u64;
        loop {
            let read = file.read(&mut buffer).map_err(unreadable)?;
            if read == 0 {
                break;
            }
            hash.update(&buffer[..read]);
            total += read as u64;
        }
        if total != length {
            return Err(invalid("Media file changed while it was being read"));
        }
        let sha256 = hex(&hash.finalize());
        let already = match self
            .db
            .begin_read()
            .map_err(storage_error)?
            .open_table(BLOBS)
        {
            Ok(table) => table.get(sha256.as_str()).map_err(storage_error)?.is_some(),
            // Blob tables are created by the first ingest.
            Err(redb::TableError::TableDoesNotExist(_)) => false,
            Err(error) => return Err(storage_error(error)),
        };
        if already {
            return Ok(Prepared::Staged {
                stage: None,
                sha256,
                size_bytes: length,
                found,
                resumed: 0,
            });
        }
        let fill = |file: &mut File, buffer: &mut [u8]| -> Result<usize> {
            let mut filled = 0;
            while filled < buffer.len() {
                match file.read(&mut buffer[filled..]).map_err(unreadable)? {
                    0 => break,
                    n => filled += n,
                }
            }
            Ok(filled)
        };
        file.seek(SeekFrom::Start(0)).map_err(unreadable)?;
        let mut check = Sha256::new();
        let mut resumed = 0;
        let mut previous = self.find_stage(&sha256, length)?;
        if let Some((old, staged)) = &previous {
            // Reuse only chunks that match the file byte for byte.
            let tx = self.db.begin_read().map_err(storage_error)?;
            let chunks = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
            let mut matches = staged.written <= staged.chunks;
            for index in 0..staged.written.min(staged.chunks) {
                let filled = fill(&mut file, &mut buffer)?;
                let stored = chunks
                    .get((sha256.as_str(), index))
                    .map_err(storage_error)?;
                if stored.is_none_or(|bytes| bytes.value() != &buffer[..filled]) {
                    matches = false;
                    break;
                }
                check.update(&buffer[..filled]);
            }
            drop(chunks);
            drop(tx);
            if matches {
                resumed = staged.written;
            } else {
                self.discard_stage(old)?;
                previous = None;
                file.seek(SeekFrom::Start(0)).map_err(unreadable)?;
                check = Sha256::new();
            }
        }
        let (id, mut stage) = match previous {
            Some(found_stage) => found_stage,
            None => {
                let id = Uuid::new_v4().to_string();
                let stage = Stage {
                    sha256: sha256.clone(),
                    size_bytes: length,
                    chunks: length.div_ceil(CHUNK),
                    written: 0,
                    started_at: self.now(),
                };
                let tx = self.begin()?;
                tx.open_table(STAGES)
                    .map_err(storage_error)?
                    .insert(id.as_str(), encode(&stage)?.as_slice())
                    .map_err(storage_error)?;
                tx.commit().map_err(storage_error)?;
                (id, stage)
            }
        };
        let mut batches = 0u64;
        let written = (|| -> Result<bool> {
            while stage.written < stage.chunks {
                if batch_limit.is_some_and(|limit| batches == limit) {
                    return Ok(false);
                }
                batches += 1;
                let tx = self.begin()?;
                {
                    let mut chunks = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
                    let end = (stage.written + STAGE_BATCH_CHUNKS).min(stage.chunks);
                    while stage.written < end {
                        let filled = fill(&mut file, &mut buffer)?;
                        let expected = if stage.written + 1 == stage.chunks {
                            length - stage.written * CHUNK
                        } else {
                            CHUNK
                        };
                        if filled as u64 != expected {
                            return Err(invalid("Media file changed while it was being read"));
                        }
                        check.update(&buffer[..filled]);
                        chunks
                            .insert((sha256.as_str(), stage.written), &buffer[..filled])
                            .map_err(storage_error)?;
                        stage.written += 1;
                    }
                }
                tx.open_table(STAGES)
                    .map_err(storage_error)?
                    .insert(id.as_str(), encode(&stage)?.as_slice())
                    .map_err(storage_error)?;
                tx.commit().map_err(storage_error)?;
            }
            if hex(&check.clone().finalize()) != sha256 {
                return Err(invalid("Media file changed while it was being read"));
            }
            Ok(true)
        })();
        match written {
            Err(error) => {
                let _ = self.discard_stage(&id);
                Err(error)
            }
            Ok(_) => Ok(Prepared::Staged {
                stage: Some(id),
                sha256,
                size_bytes: length,
                found,
                resumed,
            }),
        }
    }
    /// Remove one unpublished stage. Chunks of a published blob are never touched.
    fn discard_stage(&self, id: &str) -> Result<bool> {
        let tx = self.begin()?;
        let staged: Option<Stage> = tx
            .open_table(STAGES)
            .map_err(storage_error)?
            .remove(id)
            .map_err(storage_error)?
            .map(|v| decode(v.value()))
            .transpose()?;
        let Some(staged) = staged else {
            return Ok(false);
        };
        let published = tx
            .open_table(BLOBS)
            .map_err(storage_error)?
            .get(staged.sha256.as_str())
            .map_err(storage_error)?
            .is_some();
        if !published {
            let mut chunks = tx.open_table(BLOB_BYTES).map_err(storage_error)?;
            for index in 0..staged.chunks {
                chunks
                    .remove((staged.sha256.as_str(), index))
                    .map_err(storage_error)?;
            }
        }
        tx.commit().map_err(storage_error)?;
        Ok(true)
    }
    /// Owner-only: discard every interrupted large-file stage.
    /// # Errors
    /// Returns storage errors; published blobs are never removed.
    pub fn media_cleanup(&self) -> Result<Value> {
        let ids: Vec<String> = {
            let tx = self.db.begin_read().map_err(storage_error)?;
            let table = tx.open_table(STAGES).map_err(storage_error)?;
            table
                .iter()
                .map_err(storage_error)?
                .map(|entry| {
                    entry
                        .map(|(k, _)| k.value().to_owned())
                        .map_err(storage_error)
                })
                .collect::<Result<_>>()?
        };
        let mut removed = 0;
        for id in &ids {
            if self.discard_stage(id)? {
                removed += 1;
            }
        }
        Ok(json!({"stagesRemoved":removed}))
    }
    /// Trusted test hook: stage a recording without publishing it, as if interrupted.
    #[doc(hidden)]
    pub fn media_stage_only(&self, path: &Path, options: IngestOptions) -> Result<Option<String>> {
        Ok(match self.prepare_file(path, options)? {
            Prepared::Staged { stage, .. } => stage,
            Prepared::Inline => None,
        })
    }
    /// Trusted test hook: write at most `batches` staging transactions, then stop.
    #[doc(hidden)]
    pub fn media_stage_partial(
        &self,
        path: &Path,
        options: IngestOptions,
        batches: u64,
    ) -> Result<Option<String>> {
        Ok(match self.prepare_limited(path, options, Some(batches))? {
            Prepared::Staged { stage, .. } => stage,
            Prepared::Inline => None,
        })
    }
    /// Apply a trusted owner batch in the caller's transaction, under the canonical writer.
    pub(super) fn owner_apply_tx(
        &self,
        tx: &redb::WriteTransaction,
        items: Vec<BatchItem>,
    ) -> Result<OutcomeResponse> {
        self.owner_apply_with_id(tx, items, Uuid::new_v4().to_string())
    }

    fn owner_apply_with_id(
        &self,
        tx: &redb::WriteTransaction,
        items: Vec<BatchItem>,
        operation_id: String,
    ) -> Result<OutcomeResponse> {
        let state = state_write(tx)?;
        let request = BatchRequest {
            version: 2,
            workspace_id: state.workspace_id.clone(),
            generation: state.generation.clone(),
            operation_id,
            required_extensions: BTreeMap::new(),
            items,
        };
        GardenRequest::parse(
            GardenTool::Apply,
            &serde_json::to_value(&request).map_err(storage_error)?,
        )?;
        let owner = inbox::owner_principal();
        inbox::check_operation(
            tx,
            &request.generation,
            &request.operation_id,
            inbox::OWNER,
            "apply",
        )?;
        let prepared = self.prepare_batch(tx, &request, &owner, state, None, inbox::OWNER)?;
        inbox::claim_operation(
            tx,
            &request.generation,
            &request.operation_id,
            inbox::OWNER,
            "apply",
        )?;
        let key = operation_key(&request.generation, inbox::OWNER, &request.operation_id);
        persist_batch(tx, &key, &prepared)?;
        Ok(prepared.response)
    }
    /// Owner-only: ingest (or reuse) an image and link it to an assertion as evidence
    /// in one revision-checked transaction. The assertion gains a new revision.
    /// # Errors
    /// A stale revision, missing target or invalid media commits nothing.
    pub fn media_attach(&self, request: &AttachRequest<'_>) -> Result<Value> {
        match &request.source {
            AttachSource::File { path, options, .. } => {
                self.with_prepared(path, *options, |prepared| {
                    self.attach_with(request, &Some(prepared.clone()))
                })
            }
            AttachSource::Existing { .. } => self.attach_with(request, &None),
        }
    }
    fn attach_with(
        &self,
        request: &AttachRequest<'_>,
        prepared: &Option<Prepared>,
    ) -> Result<Value> {
        let tx = self.begin()?;
        let now = self.now();
        let (sha, acquisition, event_time, report) = match &request.source {
            AttachSource::File {
                path,
                input,
                options,
            } => {
                let report = ingest_file_tx(
                    &tx,
                    path,
                    input,
                    *options,
                    &now,
                    prepared.as_ref().unwrap_or(&Prepared::Inline),
                )?;
                (
                    report.sha256.clone(),
                    Some(report.acquisition_id.clone()),
                    input.event_time.clone(),
                    Some(report),
                )
            }
            AttachSource::Existing { sha256 } => {
                digest(sha256)?;
                (sha256.clone(), None, None, None)
            }
        };
        let evidence = BatchItem {
            item_key: "image".into(),
            expected_revision: 0,
            mutation: Mutation::Create,
            id: None,
            draft: Some(evidence_draft(
                &tx,
                &sha,
                acquisition.as_deref(),
                request.note.as_deref(),
                event_time.as_ref(),
            )?),
            replacement: None,
        };
        let link = link_item(
            &tx,
            "target",
            &request.target,
            request.expected_revision,
            "image",
        )?;
        let outcome = self.owner_apply_tx(&tx, vec![evidence, link])?;
        tx.commit().map_err(storage_error)?;
        Ok(json!({"ingest":report,"outcome":outcome}))
    }
    /// Graph media tools the credential may currently call.
    /// # Errors
    /// Returns storage errors; missing graph authority yields no tools.
    pub fn media_tools(&self, auth: &Auth) -> Result<Vec<&'static str>> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        let Ok(principal) = principal_read(&tx, auth, GardenAction::Read) else {
            return Ok(vec![]);
        };
        let mut tools = vec![];
        if has_scope(&principal, "media:meta") {
            tools.push(LIST_TOOL);
        }
        if has_scope(&principal, "media:bytes") && principal.access.mode != AccessMode::Shared {
            tools.push(GET_TOOL);
        }
        Ok(tools)
    }
    /// Bounded metadata listing of assets cited by evidence visible to the caller.
    /// Assets cited only by hidden evidence are omitted entirely.
    /// # Errors
    /// Requires `garden:read` and `media:meta` for the current workspace generation.
    pub fn media_list(&self, auth: &Auth, args: &Value) -> Result<Value> {
        let request: ListRequest = parse(args)?;
        if request.version != 2 {
            return Err(failure(
                GardenErrorCode::UnsupportedVersion,
                "Unsupported media version",
            ));
        }
        let limit = request.limit.unwrap_or(20) as usize;
        if limit == 0 || limit > MAX_LIST {
            return Err(failure(
                GardenErrorCode::LimitExceeded,
                "Media limit must be 1 to 50",
            ));
        }
        for value in request.sha256.iter().chain(&request.after) {
            digest(value)?;
        }
        let tx = self.db.begin_read().map_err(storage_error)?;
        let (state, principal) = authorize_media(
            &tx,
            auth,
            &request.workspace_id,
            &request.generation,
            "media:meta",
        )?;
        let assets = tx.open_table(ASSETS).map_err(storage_error)?;
        let acquisition_table = tx.open_table(ACQUISITIONS).map_err(storage_error)?;
        let derivative_table = tx.open_table(DERIVATIVES).map_err(storage_error)?;
        let candidates: Vec<MediaAsset> = if let Some(sha) = &request.sha256 {
            asset_read(&assets, sha)?.into_iter().collect()
        } else {
            let start = request.after.clone().unwrap_or_default();
            let mut rows = Vec::new();
            for entry in assets.range(start.as_str()..).map_err(storage_error)? {
                let (key, value) = entry.map_err(storage_error)?;
                if request.after.as_deref() == Some(key.value()) {
                    continue;
                }
                rows.push(decode(value.value())?);
                if rows.len() > SCAN_BUDGET {
                    break;
                }
            }
            rows
        };
        let mut shown = Vec::new();
        let mut last = None;
        let mut more = false;
        for (index, asset) in candidates.iter().enumerate() {
            if index == SCAN_BUDGET || shown.len() == limit {
                more = true;
                break;
            }
            last = Some(asset.sha256.clone());
            let used = visible_uses(&tx, &principal, asset)?;
            if used.is_empty() {
                continue;
            }
            let all = acquisitions(&acquisition_table, &asset.sha256)?;
            let cited: Vec<String> = used
                .iter()
                .filter_map(|u| u["evidenceId"].as_str().map(str::to_owned))
                .collect();
            let readings = interpretations(&tx, Some(&principal), &cited)?;
            let derived: Vec<Value> = derivatives(&derivative_table, &asset.sha256)?
                .into_iter()
                .filter_map(|sha| asset_read(&assets, &sha).ok().flatten())
                .map(|d| json!({"sha256":d.sha256,"sizeBytes":d.size_bytes,"mediaType":d.media_type}))
                .collect();
            shown.push(json!({"sha256":asset.sha256,"sizeBytes":asset.size_bytes,
                "mediaType":asset.media_type,"width":asset.width,"height":asset.height,
                "embeddedMetadata":asset.embedded_metadata,"derivatives":derived,
                "acquisitions":all.iter().take(MAX_ACQUISITIONS_SHOWN).collect::<Vec<_>>(),
                "acquisitionsOmitted":all.len().saturating_sub(MAX_ACQUISITIONS_SHOWN),
                "uses":used,"interpretations":readings}));
        }
        Ok(
            json!({"version":2,"workspaceId":state.workspace_id,"generation":state.generation,
            "assets":shown,"nextCursor":if more {last} else {None},
            "coverage":{"complete":!more,"scanned":candidates.len().min(SCAN_BUDGET)}}),
        )
    }
    /// Return one bounded image for an explicitly authorized client, derivative first.
    /// # Errors
    /// Requires `media:bytes`; originals need `media:originals`; oversize images are refused.
    pub fn media_get(&self, auth: &Auth, args: &Value) -> Result<Value> {
        let request: GetRequest = parse(args)?;
        if request.version != 2 {
            return Err(failure(
                GardenErrorCode::UnsupportedVersion,
                "Unsupported media version",
            ));
        }
        digest(&request.sha256)?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let (state, principal) = authorize_media(
            &tx,
            auth,
            &request.workspace_id,
            &request.generation,
            "media:bytes",
        )?;
        if principal.access.mode == AccessMode::Shared {
            return Err(forbidden("Shared clients cannot receive media bytes"));
        }
        let assets = tx.open_table(ASSETS).map_err(storage_error)?;
        let hidden = || failure(GardenErrorCode::NotFound, "Media asset does not exist");
        let original = asset_read(&assets, &request.sha256)?.ok_or_else(hidden)?;
        if visible_uses(&tx, &principal, &original)?.is_empty() {
            return Err(hidden());
        }
        let derived = derivatives(
            &tx.open_table(DERIVATIVES).map_err(storage_error)?,
            &original.sha256,
        )?
        .into_iter()
        .find_map(|sha| asset_read(&assets, &sha).ok().flatten());
        let originals = has_scope(&principal, "media:originals");
        let (variant, chosen) = match (request.variant, derived) {
            (Variant::Original, _) if !originals => {
                return Err(forbidden("Original media requires media:originals"));
            }
            (Variant::Original, _) => ("original", original.clone()),
            (Variant::Derivative | Variant::Auto, Some(d)) => ("derivative", d),
            (Variant::Derivative, None) => {
                return Err(failure(
                    GardenErrorCode::NotFound,
                    "No stripped derivative exists",
                ));
            }
            (Variant::Auto, None) if original.embedded_metadata.is_empty() || originals => {
                ("original", original.clone())
            }
            (Variant::Auto, None) => {
                return Err(forbidden(
                    "Original carries embedded metadata; the owner must create a stripped derivative or grant media:originals",
                ));
            }
        };
        if chosen.media_type.starts_with("video/") {
            return Err(forbidden(
                "Video is not returned over MCP; the owner views it locally",
            ));
        }
        if chosen.size_bytes > MAX_REPLY_BYTES {
            return Err(failure(
                GardenErrorCode::LimitExceeded,
                "Media exceeds the 2 MiB MCP reply bound; view it locally as the owner",
            ));
        }
        let bytes = read_bytes(&tx, &chosen.sha256)?;
        Ok(
            json!({"version":2,"workspaceId":state.workspace_id,"generation":state.generation,
            "sha256":chosen.sha256,"originalSha256":original.sha256,"variant":variant,
            "mediaType":chosen.media_type,"sizeBytes":chosen.size_bytes,"width":chosen.width,
            "height":chosen.height,"data":crate::media::base64(&bytes)}),
        )
    }
    /// Owner listing of all assets, or those cited by one assertion's evidence.
    /// # Errors
    /// Returns storage errors or a missing record.
    pub fn media_owner_list(&self, record: Option<&str>) -> Result<Value> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        let assets = tx.open_table(ASSETS).map_err(storage_error)?;
        let acquisition_table = tx.open_table(ACQUISITIONS).map_err(storage_error)?;
        let derivative_table = tx.open_table(DERIVATIVES).map_err(storage_error)?;
        let mut selected: Vec<(MediaAsset, Option<String>)> = Vec::new();
        if let Some(id) = record {
            let current = tx.open_table(CURRENT).map_err(storage_error)?;
            let found: GardenRecord = decode(
                current
                    .get(id)
                    .map_err(storage_error)?
                    .ok_or_else(|| {
                        failure(GardenErrorCode::NotFound, "Graph object does not exist")
                    })?
                    .value(),
            )?;
            let GardenDraft::Assertion { evidence, .. } = found.draft else {
                return Err(invalid("Attachments belong to assertions"));
            };
            for reference in evidence {
                let RecordRef::Existing { id, .. } = reference else {
                    continue;
                };
                let Some(bytes) = current.get(id.as_str()).map_err(storage_error)? else {
                    continue;
                };
                let cited: GardenRecord = decode(bytes.value())?;
                if let GardenDraft::Evidence {
                    source: SourceLocator::Artifact { sha256, .. },
                    ..
                } = cited.draft
                    && let Some(asset) = asset_read(&assets, &sha256)?
                {
                    selected.push((asset, Some(id)));
                }
            }
        } else {
            for entry in assets.iter().map_err(storage_error)? {
                let (_, value) = entry.map_err(storage_error)?;
                selected.push((decode(value.value())?, None));
                if selected.len() >= MAX_REPORT {
                    break;
                }
            }
        }
        let mut out = Vec::new();
        for (asset, evidence) in selected {
            let cited = uses(&tx, &asset)?;
            out.push(json!({"asset":asset,"evidenceId":evidence,
                "interpretations":interpretations(&tx, None, &cited)?,
                "memoryLinks":backfill::memory_links(&tx,&cited)?,
                "uses":cited,
                "derivatives":derivatives(&derivative_table, &asset.sha256)?,
                "acquisitions":acquisitions(&acquisition_table, &asset.sha256)?}));
        }
        Ok(json!({"assets":out}))
    }
    /// Owner-only: read stored bytes after verifying their digest.
    /// # Errors
    /// Missing or corrupt bytes are reported, never repaired.
    pub fn media_owner_bytes(&self, sha: &str) -> Result<Vec<u8>> {
        digest(sha)?;
        read_bytes(&self.db.begin_read().map_err(storage_error)?, sha)
    }
    /// Owner-only: write stored bytes to a new private file so the owner can open it.
    /// # Errors
    /// Never overwrites; verifies the stored digest first.
    pub fn media_export(&self, sha: &str, out: &Path) -> Result<Value> {
        digest(sha)?;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let bytes = read_bytes(&tx, sha)?;
        let mut file = crate::config::create_private_file(out)
            .map_err(|_| invalid("Export destination must be a new file"))?;
        file.write_all(&bytes).map_err(storage_error)?;
        file.sync_all().map_err(storage_error)?;
        Ok(json!({"sha256":sha,"sizeBytes":bytes.len(),"path":out}))
    }
    /// Owner-only: write every stored image as `<sha256>.<ext>` plus a manifest into a
    /// new private directory. Unreadable bytes and dangling references are listed as
    /// missing; nothing is fabricated.
    /// # Errors
    /// Refuses existing or linked destinations; a failed export removes only its own directory.
    pub fn media_export_all(&self, out: &Path) -> Result<Value> {
        super::backup::safe_path(out)?;
        if out.exists() {
            return Err(invalid("Export destination already exists"));
        }
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let report = verify(&tx)?;
        super::backup::create_bundle_dir(out)?;
        let result = (|| -> Result<Value> {
            let acquisition_table = tx.open_table(ACQUISITIONS).map_err(storage_error)?;
            let derivative_table = tx.open_table(DERIVATIVES).map_err(storage_error)?;
            let mut assets = Vec::new();
            let mut missing: Vec<Value> = Vec::new();
            let mut bytes_written = 0u64;
            for entry in tx
                .open_table(ASSETS)
                .map_err(storage_error)?
                .iter()
                .map_err(storage_error)?
            {
                let (_, value) = entry.map_err(storage_error)?;
                let asset: MediaAsset = decode(value.value())?;
                let file = match read_bytes(&tx, &asset.sha256) {
                    Ok(bytes) => {
                        let name = file_name(&asset)?;
                        let mut handle = crate::config::create_private_file(&out.join(&name))
                            .map_err(storage_error)?;
                        handle.write_all(&bytes).map_err(storage_error)?;
                        handle.sync_all().map_err(storage_error)?;
                        bytes_written += bytes.len() as u64;
                        Some(name)
                    }
                    Err(error) => {
                        missing.push(json!({"kind":"unreadable_bytes","sha256":asset.sha256,
                            "reason":error.message}));
                        None
                    }
                };
                assets.push(json!({"asset":asset,"file":file,
                    "acquisitions":acquisitions(&acquisition_table, &asset.sha256)?,
                    "derivatives":derivatives(&derivative_table, &asset.sha256)?,
                    "evidenceIds":uses(&tx, &asset)?}));
            }
            for problem in report["problems"].as_array().into_iter().flatten() {
                if problem["kind"] == "missing_reference" {
                    missing.push(problem.clone());
                }
            }
            let manifest = json!({"format":EXPORT_FORMAT,"version":1,
                "workspaceId":state.workspace_id,"generation":state.generation,
                "exportedAt":self.now(),"assets":assets,"missing":missing});
            let encoded = serde_json::to_vec_pretty(&manifest).map_err(storage_error)?;
            if encoded.len() > MAX_EXPORT_MANIFEST {
                return Err(failure(
                    GardenErrorCode::LimitExceeded,
                    "Media manifest exceeds its 16 MiB bound",
                ));
            }
            let mut handle = crate::config::create_private_file(&out.join("manifest.json"))
                .map_err(storage_error)?;
            handle.write_all(&encoded).map_err(storage_error)?;
            handle.sync_all().map_err(storage_error)?;
            Ok(
                json!({"path":out,"assets":manifest["assets"].as_array().map_or(0, Vec::len),
                "files":manifest["assets"].as_array().map_or(0, |a| a.iter().filter(|x| !x["file"].is_null()).count()),
                "bytes":bytes_written,"missing":manifest["missing"]}),
            )
        })();
        if result.is_err() {
            let _ = std::fs::remove_dir_all(out);
        }
        result
    }
    /// Owner-only: verify and add a media export's images, acquisitions and derivative
    /// links in one transaction. Graph evidence is not imported; repeating is a no-op.
    /// # Errors
    /// Any digest, size, type, name or provenance mismatch commits nothing.
    pub fn media_import(&self, dir: &Path) -> Result<Value> {
        super::backup::safe_path(dir)?;
        let manifest_path = dir.join("manifest.json");
        let mut bytes = Vec::new();
        File::open(&manifest_path)
            .map_err(|_| invalid("Media export manifest is missing"))?
            .take(MAX_EXPORT_MANIFEST as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(storage_error)?;
        if bytes.len() > MAX_EXPORT_MANIFEST {
            return Err(failure(
                GardenErrorCode::LimitExceeded,
                "Media manifest exceeds its 16 MiB bound",
            ));
        }
        let manifest: ExportManifest =
            serde_json::from_slice(&bytes).map_err(|_| invalid("Invalid media export manifest"))?;
        if manifest.format != EXPORT_FORMAT || manifest.version != 1 {
            return Err(failure(
                GardenErrorCode::UnsupportedVersion,
                "Unsupported media export format",
            ));
        }
        let exported: BTreeSet<&str> = manifest
            .assets
            .iter()
            .filter(|a| a.file.is_some())
            .map(|a| a.asset.sha256.as_str())
            .collect();
        let tx = self.begin()?;
        let now = self.now();
        let (mut imported, mut reused, mut added, mut skipped, mut linked) =
            (0u64, 0u64, 0u64, 0u64, 0u64);
        for entry in &manifest.assets {
            let Some(file) = &entry.file else { continue };
            digest(&entry.asset.sha256)?;
            if *file != file_name(&entry.asset)? {
                return Err(invalid("Exported file name does not match its asset"));
            }
            let path = dir.join(file);
            super::backup::safe_path(&path)?;
            let content = read_owner_file(&path, entry.asset.size_bytes)?;
            if content.len() as u64 != entry.asset.size_bytes
                || hex(&Sha256::digest(&content)) != entry.asset.sha256
            {
                return Err(invalid(
                    "Exported file does not match its recorded digest or size",
                ));
            }
            let found = crate::media::inspect(&content).map_err(invalid)?;
            if found.media_type != entry.asset.media_type {
                return Err(invalid(
                    "Exported file does not match its recorded media type",
                ));
            }
            let info = put_blob_tx(&tx, content.as_slice())?;
            if store_asset(&tx, &info, &found, &now)? {
                reused += 1;
            } else {
                imported += 1;
            }
        }
        let mut table = tx.open_table(ACQUISITIONS).map_err(storage_error)?;
        let assets = tx.open_table(ASSETS).map_err(storage_error)?;
        for entry in manifest.assets.iter().filter(|a| a.file.is_some()) {
            for acquisition in &entry.acquisitions {
                if acquisition.sha256 != entry.asset.sha256
                    || contracts::id(&acquisition.id).is_err()
                {
                    return Err(invalid("Acquisition does not belong to its exported asset"));
                }
                AcquisitionInput {
                    original_filename: acquisition.original_filename.clone(),
                    event_time: acquisition.event_time.clone(),
                    source: acquisition.source.clone(),
                    origin: acquisition.origin,
                    declared_media_type: None,
                }
                .validate()?;
                chrono::DateTime::parse_from_rfc3339(&acquisition.acquired_at)
                    .map_err(|_| invalid("Invalid acquisition time"))?;
                if let Some(source) = &acquisition.derived_from
                    && asset_read(&assets, source)?.is_none()
                    && !exported.contains(source.as_str())
                {
                    return Err(invalid("Derivative provenance names an absent original"));
                }
                let key = format!("{}/{}", acquisition.sha256, acquisition.id);
                if let Some(existing) = table.get(key.as_str()).map_err(storage_error)? {
                    if decode::<Acquisition>(existing.value())? != *acquisition {
                        return Err(invalid("Acquisition ID collides with different provenance"));
                    }
                    skipped += 1;
                    continue;
                }
                table
                    .insert(key.as_str(), encode(acquisition)?.as_slice())
                    .map_err(storage_error)?;
                added += 1;
            }
        }
        drop(table);
        let mut links = tx.open_table(DERIVATIVES).map_err(storage_error)?;
        for entry in manifest.assets.iter().filter(|a| a.file.is_some()) {
            for derived in &entry.derivatives {
                digest(derived)?;
                if asset_read(&assets, derived)?.is_none() {
                    continue;
                }
                let key = format!("{}/{derived}", entry.asset.sha256);
                if links.get(key.as_str()).map_err(storage_error)?.is_none() {
                    links
                        .insert(key.as_str(), [].as_slice())
                        .map_err(storage_error)?;
                    linked += 1;
                }
            }
        }
        drop(links);
        drop(assets);
        tx.commit().map_err(storage_error)?;
        Ok(
            json!({"assetsImported":imported,"assetsReused":reused,"acquisitionsAdded":added,
            "acquisitionsAlreadyPresent":skipped,"derivativesLinked":linked,
            "missingInExport":manifest.missing.len()}),
        )
    }
    /// Read-only integrity report over media bytes, provenance and references.
    /// # Errors
    /// Returns storage errors only; problems are reported, never repaired.
    pub fn media_verify(&self) -> Result<Value> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        verify(&tx)
    }
}

/// Backup and restore gate: every asset, acquisition and derivative link is consistent.
pub(super) fn validate(tx: &redb::ReadTransaction) -> Result<()> {
    if verify(tx)?["ok"] == json!(true) {
        Ok(())
    } else {
        Err(invalid(
            "Media provenance, derivatives or bytes are inconsistent; run garden media verify",
        ))
    }
}
fn verify(tx: &redb::ReadTransaction) -> Result<Value> {
    let present: BTreeSet<_> = tx
        .list_tables()
        .map_err(storage_error)?
        .map(|t| t.name().to_owned())
        .collect();
    let mut problems: Vec<Value> = Vec::new();
    let mut note = |problem: Value| {
        if problems.len() < MAX_REPORT {
            problems.push(problem);
        }
    };
    let blob_meta = |sha: &str| -> Result<Option<BlobInfo>> {
        if !present.contains(BLOBS.name()) {
            return Ok(None);
        }
        tx.open_table(BLOBS)
            .map_err(storage_error)?
            .get(sha)
            .map_err(storage_error)?
            .map(|v| decode(v.value()))
            .transpose()
    };
    let mut checked = 0u64;
    let mut asset_set = BTreeSet::new();
    if present.contains(ASSETS.name()) {
        for entry in tx
            .open_table(ASSETS)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (key, value) = entry.map_err(storage_error)?;
            let asset: MediaAsset = decode(value.value())?;
            checked += 1;
            asset_set.insert(asset.sha256.clone());
            if asset.sha256 != key.value() {
                note(json!({"kind":"asset_key_mismatch","sha256":key.value()}));
                continue;
            }
            match blob_meta(&asset.sha256)? {
                None => note(json!({"kind":"missing_bytes","sha256":asset.sha256})),
                Some(info) if info.size_bytes != asset.size_bytes => {
                    note(json!({"kind":"size_mismatch","sha256":asset.sha256}));
                }
                Some(_) => match read_bytes(tx, &asset.sha256) {
                    Ok(bytes) => {
                        if crate::media::sniff(&bytes) != Some(asset.media_type.as_str()) {
                            note(json!({"kind":"type_mismatch","sha256":asset.sha256}));
                        }
                    }
                    Err(_) => note(json!({"kind":"corrupt_bytes","sha256":asset.sha256})),
                },
            }
        }
    }
    if present.contains(ACQUISITIONS.name()) {
        for entry in tx
            .open_table(ACQUISITIONS)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (key, value) = entry.map_err(storage_error)?;
            let acquisition: Acquisition = decode(value.value())?;
            if key.value() != format!("{}/{}", acquisition.sha256, acquisition.id)
                || !asset_set.contains(&acquisition.sha256)
                || acquisition
                    .derived_from
                    .as_ref()
                    .is_some_and(|d| !asset_set.contains(d))
            {
                note(
                    json!({"kind":"dangling_acquisition","acquisitionId":acquisition.id,"sha256":acquisition.sha256}),
                );
            }
        }
    }
    if present.contains(DERIVATIVES.name()) {
        for entry in tx
            .open_table(DERIVATIVES)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (key, _) = entry.map_err(storage_error)?;
            let pair = key.value().split_once('/');
            if pair.is_none_or(|(a, b)| !asset_set.contains(a) || !asset_set.contains(b)) {
                note(json!({"kind":"dangling_derivative","link":key.value()}));
            }
        }
    }
    if present.contains(STAGES.name()) {
        for entry in tx
            .open_table(STAGES)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (key, value) = entry.map_err(storage_error)?;
            let staged: Stage = decode(value.value())?;
            note(
                json!({"kind":"interrupted_stage","stageId":key.value(),"sha256":staged.sha256,
                "written":staged.written,"chunks":staged.chunks}),
            );
        }
    }
    let mut references = 0u64;
    for entry in tx
        .open_table(CURRENT)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (_, value) = entry.map_err(storage_error)?;
        let record: GardenRecord = decode(value.value())?;
        if let GardenDraft::Evidence {
            source: SourceLocator::Artifact {
                sha256, size_bytes, ..
            },
            ..
        } = &record.draft
        {
            references += 1;
            match blob_meta(sha256)? {
                None => note(
                    json!({"kind":"missing_reference","evidenceId":record.metadata.id,"sha256":sha256}),
                ),
                Some(info) if info.size_bytes != *size_bytes => {
                    note(
                        json!({"kind":"reference_size_mismatch","evidenceId":record.metadata.id,"sha256":sha256}),
                    );
                }
                Some(_) => {}
            }
        }
    }
    let truncated = problems.len() >= MAX_REPORT;
    Ok(
        json!({"assetsChecked":checked,"artifactReferences":references,
        "ok":problems.is_empty(),"problems":problems,"truncated":truncated}),
    )
}
