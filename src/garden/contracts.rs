//! Strict v2 wire models and bounded, storage-independent validation.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_JSON_BYTES: usize = 262_144;
pub const MAX_JSON_DEPTH: usize = 24;
pub const MAX_BATCH_ITEMS: usize = 64;
pub const MAX_REFS: usize = 64;
pub const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
pub type Extensions = BTreeMap<String, Extension>;
pub type RequiredExtensions = BTreeMap<String, u32>;
type Result<T> = std::result::Result<T, GardenError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GardenErrorCode {
    InvalidInput,
    UnsupportedVersion,
    UnsupportedExtension,
    LimitExceeded,
    InvalidEndpoint,
    Unauthorized,
    Forbidden,
    WorkspaceMismatch,
    NotFound,
    Conflict,
    OperationReused,
    GenerationMismatch,
    RestartRequired,
    StoreUnavailable,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("{code:?}: {message}")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GardenError {
    pub code: GardenErrorCode,
    pub message: String,
    pub restart_required: bool,
}
fn error(code: GardenErrorCode, message: &str) -> GardenError {
    GardenError {
        code,
        message: message.into(),
        restart_required: code == GardenErrorCode::RestartRequired,
    }
}
fn invalid(message: &str) -> GardenError {
    error(GardenErrorCode::InvalidInput, message)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GardenTool {
    Get,
    Query,
    Apply,
    Propose,
    GetProposal,
    GetReceipt,
    Changes,
}
impl GardenTool {
    pub fn name(self) -> &'static str {
        match self {
            Self::Get => "mother_garden_get",
            Self::Query => "mother_garden_query",
            Self::Apply => "mother_garden_apply",
            Self::Propose => "mother_garden_propose",
            Self::GetProposal => "mother_garden_get_proposal",
            Self::GetReceipt => "mother_garden_get_receipt",
            Self::Changes => "mother_garden_changes",
        }
    }
    pub fn parse(name: &str) -> Result<Self> {
        [
            Self::Get,
            Self::Query,
            Self::Apply,
            Self::Propose,
            Self::GetProposal,
            Self::GetReceipt,
            Self::Changes,
        ]
        .into_iter()
        .find(|tool| tool.name() == name)
        .ok_or_else(|| invalid("Unknown Garden tool"))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GardenKind {
    Entity,
    Assertion,
    Evidence,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExternalReference {
    pub kind: GardenKind,
    pub id: String,
}
impl ExternalReference {
    pub fn validate(&self) -> Result<()> {
        uuid(&self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "refType", rename_all = "camelCase", deny_unknown_fields)]
pub enum RecordRef {
    Existing {
        kind: GardenKind,
        id: String,
    },
    Item {
        kind: GardenKind,
        #[serde(rename = "itemKey")]
        item_key: String,
    },
}
impl RecordRef {
    pub fn kind(&self) -> GardenKind {
        match self {
            Self::Existing { kind, .. } | Self::Item { kind, .. } => *kind,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Extension {
    pub schema_version: u32,
    pub payload: Value,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimePrecision {
    Unknown,
    Year,
    Month,
    Day,
    Minute,
    Second,
    Instant,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObservedTime {
    pub at: Option<String>,
    pub source_timezone: Option<String>,
    pub precision: TimePrecision,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidTime {
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub source_timezone: Option<String>,
    pub precision: TimePrecision,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceBasis {
    OwnerStatement,
    Source,
    ModelInference,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Confidence {
    pub score: f64,
    pub basis: ConfidenceBasis,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum TypedValue {
    String { value: String },
    Number { value: f64, unit: Option<String> },
    Boolean { value: bool },
    Instant { value: String },
    Json { value: Value },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum AssertionObject {
    EntityRef {
        #[serde(rename = "entityRef")]
        entity_ref: RecordRef,
    },
    Value {
        value: TypedValue,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum SourceLocator {
    Uri {
        uri: String,
    },
    Artifact {
        sha256: String,
        #[serde(rename = "sizeBytes")]
        size_bytes: u64,
        #[serde(rename = "mediaType")]
        media_type: String,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum GardenDraft {
    Entity {
        #[serde(rename = "entityType")]
        entity_type: String,
        #[serde(rename = "displayName")]
        display_name: String,
        aliases: Vec<String>,
        extensions: Extensions,
    },
    Assertion {
        subject: RecordRef,
        predicate: String,
        object: AssertionObject,
        #[serde(rename = "validTime")]
        valid_time: ValidTime,
        #[serde(rename = "observedTime")]
        observed_time: ObservedTime,
        evidence: Vec<RecordRef>,
        confidence: Option<Confidence>,
        extensions: Extensions,
    },
    Evidence {
        source: SourceLocator,
        excerpt: Option<String>,
        #[serde(rename = "observedTime")]
        observed_time: ObservedTime,
        extensions: Extensions,
    },
}
impl GardenDraft {
    pub fn kind(&self) -> GardenKind {
        match self {
            Self::Entity { .. } => GardenKind::Entity,
            Self::Assertion { .. } => GardenKind::Assertion,
            Self::Evidence { .. } => GardenKind::Evidence,
        }
    }
    pub fn references(&self) -> Vec<&RecordRef> {
        match self {
            Self::Assertion {
                subject,
                object,
                evidence,
                ..
            } => {
                let mut refs = vec![subject];
                if let AssertionObject::EntityRef { entity_ref } = object {
                    refs.push(entity_ref);
                }
                refs.extend(evidence);
                refs
            }
            _ => vec![],
        }
    }
    pub fn validate(&self) -> Result<()> {
        let extensions = match self {
            Self::Entity {
                entity_type,
                display_name,
                aliases,
                extensions,
            } => {
                namespace(entity_type)?;
                text(display_name, 512)?;
                count(aliases.len(), 32)?;
                for alias in aliases {
                    text(alias, 512)?;
                }
                extensions
            }
            Self::Evidence {
                source,
                excerpt,
                observed_time,
                extensions,
            } => {
                source.validate()?;
                if let Some(excerpt) = excerpt {
                    text(excerpt, 16_384)?;
                }
                observed_time.validate()?;
                extensions
            }
            Self::Assertion {
                subject,
                predicate,
                object,
                valid_time,
                observed_time,
                evidence,
                confidence,
                extensions,
            } => {
                namespace(predicate)?;
                expect_kind(subject, GardenKind::Entity)?;
                if let AssertionObject::EntityRef { entity_ref } = object {
                    expect_kind(entity_ref, GardenKind::Entity)?;
                } else if let AssertionObject::Value { value } = object {
                    value.validate()?;
                }
                valid_time.validate()?;
                observed_time.validate()?;
                count(evidence.len(), MAX_REFS)?;
                for reference in evidence {
                    expect_kind(reference, GardenKind::Evidence)?;
                }
                if let Some(confidence) = confidence {
                    score(confidence.score)?;
                }
                extensions
            }
        };
        count(extensions.len(), 32)?;
        for (name, extension) in extensions {
            namespace(name)?;
            if extension.schema_version == 0 {
                return Err(invalid("Extension schemaVersion must be positive"));
            }
            bounded_json(&extension.payload)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutation {
    Create,
    Update,
    Retract,
    Supersede,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchItem {
    pub item_key: String,
    pub expected_revision: u64,
    pub mutation: Mutation,
    #[serde(
        default,
        deserialize_with = "present_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ExternalReference>,
    #[serde(
        default,
        deserialize_with = "present_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub draft: Option<GardenDraft>,
    #[serde(
        default,
        deserialize_with = "present_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub replacement: Option<RecordRef>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchRequest {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub operation_id: String,
    pub required_extensions: RequiredExtensions,
    pub items: Vec<BatchItem>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetRequest {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub required_extensions: RequiredExtensions,
    pub reference: ExternalReference,
    pub revision: Option<u64>,
    pub known_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Traversal {
    pub start: Vec<ExternalReference>,
    pub direction: Direction,
    pub max_hops: u32,
    pub max_edges: u32,
    pub work_budget: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Incoming,
    Outgoing,
    Both,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueryRequest {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub required_extensions: RequiredExtensions,
    pub ids: Vec<ExternalReference>,
    pub kinds: Vec<GardenKind>,
    pub entity_type: Option<String>,
    pub predicate: Option<String>,
    pub property: Option<PropertyFilter>,
    pub valid_at: Option<String>,
    pub known_at: Option<String>,
    pub text: Option<String>,
    pub vector: Option<VectorQuery>,
    pub traversal: Option<Traversal>,
    pub limit: u32,
    pub cursor: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PropertyFilter {
    pub name: String,
    pub value: TypedValue,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorQuery {
    pub profile: String,
    pub model: String,
    pub dimensions: u32,
    pub values: Vec<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OutcomeRequest {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub required_extensions: RequiredExtensions,
    pub operation_id: String,
    pub operation_generation: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangesRequest {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub required_extensions: RequiredExtensions,
    pub limit: u32,
    pub cursor: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GardenRequest {
    Get(GetRequest),
    Query(Box<QueryRequest>),
    Batch(BatchRequest),
    Outcome(OutcomeRequest),
    Changes(ChangesRequest),
}
impl GardenRequest {
    /// Parse and validate a strict request. This does not authorize or execute it.
    pub fn parse(tool: GardenTool, value: &Value) -> Result<Self> {
        bounded_json(value)?;
        check_version(value)?;
        let request = match tool {
            GardenTool::Get => Self::Get(decode(value)?),
            GardenTool::Query => Self::Query(Box::new(decode(value)?)),
            GardenTool::Apply | GardenTool::Propose => Self::Batch(decode(value)?),
            GardenTool::GetProposal | GardenTool::GetReceipt => Self::Outcome(decode(value)?),
            GardenTool::Changes => Self::Changes(decode(value)?),
        };
        request.validate()?;
        Ok(request)
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Batch(q) => q.validate(),
            Self::Get(q) => {
                envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.required_extensions,
                )?;
                q.reference.validate()?;
                if let Some(rev) = q.revision {
                    revision(rev, 1)?;
                }
                optional_time(&q.known_at)
            }
            Self::Query(q) => {
                envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.required_extensions,
                )?;
                page(q.limit, &q.cursor)?;
                count(q.ids.len(), MAX_REFS)?;
                for reference in &q.ids {
                    reference.validate()?;
                }
                count(q.kinds.len(), 3)?;
                if let Some(kind) = &q.entity_type {
                    namespace(kind)?;
                }
                if let Some(predicate) = &q.predicate {
                    namespace(predicate)?;
                }
                if let Some(filter) = &q.property {
                    namespace(&filter.name)?;
                    filter.value.validate()?;
                }
                optional_time(&q.valid_at)?;
                optional_time(&q.known_at)?;
                if let Some(t) = &q.text {
                    text(t, 2000)?;
                }
                if let Some(vector) = &q.vector {
                    text(&vector.model, 200)?;
                    text(&vector.profile, 200)?;
                    if vector.dimensions == 0
                        || vector.dimensions > 4096
                        || vector.values.len() != vector.dimensions as usize
                        || vector.values.iter().any(|v| !v.is_finite())
                    {
                        return Err(invalid("Invalid vector profile/dimensions/values"));
                    }
                }
                if let Some(t) = &q.traversal {
                    if t.start.is_empty()
                        || t.max_hops == 0
                        || t.max_hops > 8
                        || t.max_edges == 0
                        || t.max_edges > 1000
                        || t.work_budget == 0
                        || t.work_budget > 10_000
                    {
                        return Err(invalid("Invalid traversal budget"));
                    }
                    count(t.start.len(), MAX_REFS)?;
                    for reference in &t.start {
                        reference.validate()?;
                    }
                }
                Ok(())
            }
            Self::Outcome(q) => {
                envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.required_extensions,
                )?;
                uuid(&q.operation_id)?;
                uuid(&q.operation_generation)
            }
            Self::Changes(q) => {
                envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.required_extensions,
                )?;
                page(q.limit, &q.cursor)
            }
        }
    }
}
impl BatchRequest {
    pub fn validate(&self) -> Result<()> {
        envelope(
            self.version,
            &self.workspace_id,
            &self.generation,
            &self.required_extensions,
        )?;
        uuid(&self.operation_id)?;
        if self.items.is_empty() {
            return Err(invalid("Empty batch"));
        }
        count(self.items.len(), MAX_BATCH_ITEMS)?;
        let mut keys = BTreeMap::new();
        let mut targets = BTreeSet::new();
        for item in &self.items {
            item_key(&item.item_key)?;
            revision(item.expected_revision, 0)?;
            let kind = match item.mutation {
                Mutation::Create => {
                    if item.id.is_some()
                        || item.expected_revision != 0
                        || item.replacement.is_some()
                    {
                        return Err(invalid(
                            "Creates cannot set identity, revision or replacement",
                        ));
                    }
                    item.draft
                        .as_ref()
                        .ok_or_else(|| invalid("Create requires draft"))?
                        .kind()
                }
                Mutation::Update | Mutation::Retract | Mutation::Supersede => {
                    let reference = item
                        .id
                        .as_ref()
                        .ok_or_else(|| invalid("Existing target required"))?;
                    reference.validate()?;
                    if item.expected_revision == 0 || !targets.insert(reference.clone()) {
                        return Err(invalid("Invalid revision or duplicate target"));
                    }
                    if item.mutation == Mutation::Update {
                        if item
                            .draft
                            .as_ref()
                            .is_none_or(|draft| draft.kind() != reference.kind)
                            || item.replacement.is_some()
                        {
                            return Err(invalid("Update kind must match target"));
                        }
                    } else if item.draft.is_some()
                        || (item.mutation == Mutation::Supersede) != item.replacement.is_some()
                    {
                        return Err(invalid("Lifecycle mutation has invalid draft/replacement"));
                    }
                    reference.kind
                }
            };
            if let Some(draft) = &item.draft {
                draft.validate()?;
            }
            if keys
                .insert(item.item_key.as_str(), (kind, item.mutation))
                .is_some()
            {
                return Err(invalid("Duplicate itemKey"));
            }
        }
        for item in &self.items {
            let mut refs = item
                .draft
                .as_ref()
                .map_or_else(Vec::new, GardenDraft::references);
            if let Some(replacement) = &item.replacement {
                refs.push(replacement);
                if item
                    .id
                    .as_ref()
                    .is_some_and(|id| id.kind != replacement.kind())
                {
                    return Err(error(
                        GardenErrorCode::InvalidEndpoint,
                        "Replacement kind mismatch",
                    ));
                }
            }
            for reference in refs {
                match reference {
                    RecordRef::Existing { id, .. } => uuid(id)?,
                    RecordRef::Item { kind, item_key } => {
                        let Some((actual, mutation)) = keys.get(item_key.as_str()) else {
                            return Err(error(
                                GardenErrorCode::InvalidEndpoint,
                                "Unknown batch endpoint",
                            ));
                        };
                        if actual != kind || *mutation != Mutation::Create {
                            return Err(error(
                                GardenErrorCode::InvalidEndpoint,
                                "Batch endpoints must reference matching creates",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
    /// Existing references need current existence/kind/visibility checks by the authority boundary.
    pub fn external_references(&self) -> Vec<ExternalReference> {
        let mut refs = BTreeSet::new();
        for item in &self.items {
            if let Some(id) = &item.id {
                refs.insert(id.clone());
            }
            let mut linked = item
                .draft
                .as_ref()
                .map_or_else(Vec::new, GardenDraft::references);
            if let Some(replacement) = &item.replacement {
                linked.push(replacement);
            }
            for reference in linked {
                if let RecordRef::Existing { kind, id } = reference {
                    refs.insert(ExternalReference {
                        kind: *kind,
                        id: id.clone(),
                    });
                }
            }
        }
        refs.into_iter().collect()
    }
}

fn decode<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|_| invalid("Invalid or unknown contract field"))
}
fn check_version(value: &Value) -> Result<()> {
    if value.get("version").and_then(Value::as_u64) != Some(2) {
        return Err(error(
            GardenErrorCode::UnsupportedVersion,
            "Expected Garden contract version 2",
        ));
    }
    Ok(())
}
fn envelope(
    version: u32,
    workspace: &str,
    generation: &str,
    required: &RequiredExtensions,
) -> Result<()> {
    if version != 2 {
        return Err(error(
            GardenErrorCode::UnsupportedVersion,
            "Expected Garden contract version 2",
        ));
    }
    uuid(workspace)?;
    uuid(generation)?;
    if !required.is_empty() {
        return Err(error(
            GardenErrorCode::UnsupportedExtension,
            "No required extension schema is implemented by this contract-only boundary",
        ));
    }
    Ok(())
}
pub fn bounded_json(value: &Value) -> Result<()> {
    fn depth(value: &Value, level: usize) -> bool {
        level <= MAX_JSON_DEPTH
            && match value {
                Value::Object(map) => map.values().all(|v| depth(v, level + 1)),
                Value::Array(values) => values.iter().all(|v| depth(v, level + 1)),
                _ => true,
            }
    }
    if !depth(value, 0)
        || serde_json::to_vec(value)
            .map_err(|_| invalid("Invalid JSON"))?
            .len()
            > MAX_JSON_BYTES
    {
        return Err(error(
            GardenErrorCode::LimitExceeded,
            "JSON byte/depth bound exceeded",
        ));
    }
    Ok(())
}
fn count(actual: usize, max: usize) -> Result<()> {
    if actual > max {
        Err(error(
            GardenErrorCode::LimitExceeded,
            "Collection bound exceeded",
        ))
    } else {
        Ok(())
    }
}
fn text(value: &str, max: usize) -> Result<()> {
    if value.trim().is_empty() || value.chars().count() > max {
        Err(invalid("Invalid text length"))
    } else {
        Ok(())
    }
}
fn namespace(value: &str) -> Result<()> {
    text(value, 200)?;
    let Some((prefix, name)) = value.split_once(':') else {
        return Err(invalid("Namespaced identifier required"));
    };
    if prefix.is_empty()
        || name.is_empty()
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._:/-".contains(&c))
    {
        return Err(invalid("Invalid namespaced identifier"));
    }
    Ok(())
}
fn uuid(value: &str) -> Result<()> {
    crate::contracts::id(value).map_err(|_| invalid("Invalid UUID"))
}
fn revision(value: u64, min: u64) -> Result<()> {
    if !(min..=MAX_SAFE_INTEGER).contains(&value) {
        Err(invalid("Invalid revision/sequence"))
    } else {
        Ok(())
    }
}
fn item_key(value: &str) -> Result<()> {
    text(value, 80)?;
    if !value
        .bytes()
        .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
    {
        return Err(invalid("Invalid itemKey"));
    }
    Ok(())
}
fn instant(value: &str) -> Result<DateTime<Utc>> {
    if value.len() > 40 || !value.ends_with('Z') {
        return Err(invalid("Expected explicit UTC instant"));
    }
    DateTime::parse_from_rfc3339(value)
        .map(|t| t.with_timezone(&Utc))
        .map_err(|_| invalid("Invalid UTC instant"))
}
fn optional_time(value: &Option<String>) -> Result<()> {
    if let Some(value) = value {
        instant(value)?;
    }
    Ok(())
}
fn timezone(value: &Option<String>) -> Result<()> {
    if let Some(zone) = value {
        text(zone, 100)?;
        if zone.chars().any(char::is_control) {
            return Err(invalid("Invalid source timezone metadata"));
        }
    }
    Ok(())
}
fn score(value: f64) -> Result<()> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        Err(invalid("Score must be finite and between zero and one"))
    } else {
        Ok(())
    }
}
fn expect_kind(reference: &RecordRef, expected: GardenKind) -> Result<()> {
    if reference.kind() != expected {
        return Err(error(
            GardenErrorCode::InvalidEndpoint,
            "Incorrect endpoint kind",
        ));
    }
    match reference {
        RecordRef::Existing { id, .. } => uuid(id),
        RecordRef::Item { item_key: key, .. } => item_key(key),
    }
}
fn page(limit: u32, cursor: &Option<String>) -> Result<()> {
    if !(1..=100).contains(&limit) {
        return Err(invalid("Invalid page limit"));
    }
    if let Some(cursor) = cursor {
        text(cursor, 4096)?;
    }
    Ok(())
}
impl ObservedTime {
    pub fn validate(&self) -> Result<()> {
        optional_time(&self.at)?;
        timezone(&self.source_timezone)?;
        if self.at.is_some() == (self.precision == TimePrecision::Unknown) {
            return Err(invalid(
                "Unknown observation requires null instant and unknown precision",
            ));
        }
        Ok(())
    }
}
impl ValidTime {
    pub fn validate(&self) -> Result<()> {
        optional_time(&self.valid_from)?;
        optional_time(&self.valid_until)?;
        timezone(&self.source_timezone)?;
        if let (Some(start), Some(end)) = (&self.valid_from, &self.valid_until)
            && instant(start)? >= instant(end)?
        {
            return Err(invalid("Validity must be a nonempty half-open interval"));
        }
        if (self.valid_from.is_none() && self.valid_until.is_none())
            != (self.precision == TimePrecision::Unknown)
        {
            return Err(invalid("Unknown validity requires unknown precision"));
        }
        Ok(())
    }
}
impl TypedValue {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::String { value } => text(value, 16_384),
            Self::Number { value, unit } => {
                if !value.is_finite() {
                    return Err(invalid("Number must be finite"));
                }
                if let Some(unit) = unit {
                    text(unit, 100)?;
                }
                Ok(())
            }
            Self::Boolean { .. } => Ok(()),
            Self::Instant { value } => instant(value).map(|_| ()),
            Self::Json { value } => bounded_json(value),
        }
    }
}
impl SourceLocator {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Uri { uri } => {
                text(uri, 2048)?;
                let url = url::Url::parse(uri).map_err(|_| invalid("Invalid source URI"))?;
                if !matches!(url.scheme(), "https" | "http" | "urn")
                    || !url.username().is_empty()
                    || url.password().is_some()
                {
                    return Err(invalid("Unsupported source URI or embedded credentials"));
                }
                Ok(())
            }
            Self::Artifact {
                sha256,
                size_bytes,
                media_type,
            } => {
                if !crate::contracts::hex_digest(sha256)
                    || *size_bytes == 0
                    || *size_bytes > MAX_SAFE_INTEGER
                {
                    return Err(invalid("Invalid artifact hash or size"));
                }
                text(media_type, 200)
            }
        }
    }
}

/// Store-owned metadata, never accepted inside a client draft.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevisionMetadata {
    pub id: String,
    pub revision: u64,
    pub previous_revision: Option<u64>,
    pub commit_sequence: u64,
    pub transaction_time: String,
    pub actor: String,
    pub contributor: String,
    pub approver: Option<String>,
    pub lifecycle: Lifecycle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lifecycle {
    Active,
    Retracted,
    Superseded,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GardenRecord {
    pub metadata: RevisionMetadata,
    pub draft: GardenDraft,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Coverage {
    pub complete: bool,
    pub truncated: bool,
    pub reason: Option<String>,
    pub canonical_watermark: u64,
    pub index_watermark: u64,
    pub search_mode: SearchMode,
    pub model: Option<String>,
    pub profile: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    Exact,
    Literal,
    Vector,
    Hybrid,
    LiteralFallback,
}
/// Opaque cursor and bounded snapshot lease metadata, checked against private runtime state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PageState {
    pub next_cursor: Option<String>,
    pub visibility_epoch: String,
    pub snapshot_id: String,
    pub snapshot_expires_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentScores {
    pub lexical: Option<f64>,
    pub vector: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueryHit {
    pub record: GardenRecord,
    pub sources: Vec<ExternalReference>,
    pub scores: ComponentScores,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetResponse {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub record: GardenRecord,
    pub coverage: Coverage,
    pub visibility_epoch: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueryResponse {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub hits: Vec<QueryHit>,
    pub coverage: Coverage,
    pub page: PageState,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeStatus {
    Pending,
    Approved,
    Dismissed,
    Applied,
    Blocked,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemResult {
    pub item_key: String,
    pub reference: ExternalReference,
    pub revision: u64,
    pub owner_edited: bool,
    pub owner_added: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperationReceipt {
    pub operation_id: String,
    pub operation_generation: String,
    pub actor: String,
    pub status: OutcomeStatus,
    pub proposal_id: Option<String>,
    pub committed_at: Option<String>,
    pub commit_sequence: Option<u64>,
    pub results: Vec<ItemResult>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OutcomeResponse {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub receipt: OperationReceipt,
    pub original_proposal: Option<BatchRequest>,
    pub visible_records: Vec<GardenRecord>,
    /// True when some or all original/canonical payload details were omitted; IDs confer no visibility.
    pub payload_omitted: bool,
    pub visibility_epoch: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalResponse {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub proposal_id: String,
    pub operation_id: String,
    pub status: OutcomeStatus,
    pub visibility_epoch: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Change {
    pub reference: ExternalReference,
    pub revision: u64,
    pub sequence: u64,
    pub lifecycle: Lifecycle,
    pub record: Option<GardenRecord>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangesResponse {
    pub version: u32,
    pub workspace_id: String,
    pub generation: String,
    pub changes: Vec<Change>,
    pub coverage: Coverage,
    pub page: PageState,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GardenResponse {
    Get(Box<GetResponse>),
    Query(QueryResponse),
    Outcome(OutcomeResponse),
    Proposal(ProposalResponse),
    Changes(ChangesResponse),
    Error(GardenError),
}
impl GardenResponse {
    /// Validate a synthetic or backend-supplied response before exposing its payload.
    pub fn parse(tool: GardenTool, value: &Value) -> Result<Self> {
        bounded_json(value)?;
        if value.get("code").is_some() {
            let failure: GardenError = decode(value)?;
            if failure.restart_required != (failure.code == GardenErrorCode::RestartRequired) {
                return Err(invalid("Invalid restart-required error"));
            }
            let response = Self::Error(failure);
            response.validate()?;
            return Ok(response);
        }
        check_version(value)?;
        let response = match tool {
            GardenTool::Get => Self::Get(decode(value)?),
            GardenTool::Query => Self::Query(decode(value)?),
            GardenTool::Apply | GardenTool::GetReceipt | GardenTool::GetProposal => {
                Self::Outcome(decode(value)?)
            }
            GardenTool::Propose => Self::Proposal(decode(value)?),
            GardenTool::Changes => Self::Changes(decode(value)?),
        };
        response.validate()?;
        if let Self::Outcome(outcome) = &response {
            match tool {
                GardenTool::Apply if outcome.receipt.status != OutcomeStatus::Applied => {
                    return Err(invalid("Apply requires an applied receipt"));
                }
                GardenTool::GetProposal if outcome.receipt.status == OutcomeStatus::Applied => {
                    return Err(invalid(
                        "Proposal reads cannot return a direct-apply receipt",
                    ));
                }
                _ => {}
            }
        }
        Ok(response)
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Get(q) => {
                response_envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.visibility_epoch,
                )?;
                q.record.validate()?;
                q.coverage.validate()?;
                if q.record.metadata.commit_sequence > q.coverage.canonical_watermark {
                    return Err(invalid("Record exceeds canonical watermark"));
                }
                Ok(())
            }
            Self::Query(q) => {
                response_envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.page.visibility_epoch,
                )?;
                q.page.validate()?;
                q.coverage.validate()?;
                count(q.hits.len(), 100)?;
                for hit in &q.hits {
                    hit.record.validate()?;
                    if hit.record.metadata.commit_sequence > q.coverage.canonical_watermark {
                        return Err(invalid("Record exceeds canonical watermark"));
                    }
                    count(hit.sources.len(), MAX_REFS)?;
                    for reference in &hit.sources {
                        reference.validate()?;
                        if reference.kind != GardenKind::Evidence {
                            return Err(invalid("Query sources must be evidence"));
                        }
                    }
                    for value in [hit.scores.lexical, hit.scores.vector]
                        .into_iter()
                        .flatten()
                    {
                        if !value.is_finite() {
                            return Err(invalid("Component score must be finite"));
                        }
                    }
                }
                Ok(())
            }
            Self::Outcome(q) => {
                response_envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.visibility_epoch,
                )?;
                q.receipt.validate()?;
                if let Some(proposal) = &q.original_proposal {
                    proposal.validate()?;
                    if proposal.operation_id != q.receipt.operation_id
                        || proposal.generation != q.receipt.operation_generation
                        || proposal.workspace_id != q.workspace_id
                    {
                        return Err(invalid("Original proposal must match receipt identity"));
                    }
                }
                if let Some(proposal) = &q.original_proposal
                    && q.receipt.status == OutcomeStatus::Approved
                {
                    let original: BTreeSet<_> = proposal
                        .items
                        .iter()
                        .map(|item| item.item_key.as_str())
                        .collect();
                    let mapped: BTreeSet<_> = q
                        .receipt
                        .results
                        .iter()
                        .filter(|result| !result.owner_added)
                        .map(|result| result.item_key.as_str())
                        .collect();
                    if original != mapped
                        || q.receipt.results.iter().any(|result| {
                            result.owner_added == original.contains(result.item_key.as_str())
                        })
                    {
                        return Err(invalid(
                            "Receipt must map every original item and mark owner additions",
                        ));
                    }
                }
                count(q.visible_records.len(), MAX_BATCH_ITEMS)?;
                let mut returned = BTreeSet::new();
                for record in &q.visible_records {
                    record.validate()?;
                    let reference = ExternalReference {
                        kind: record.draft.kind(),
                        id: record.metadata.id.clone(),
                    };
                    let mapping = q
                        .receipt
                        .results
                        .iter()
                        .find(|result| result.reference == reference)
                        .ok_or_else(|| invalid("Outcome payload has no receipt mapping"))?;
                    if record.metadata.revision != mapping.revision
                        || Some(record.metadata.commit_sequence) != q.receipt.commit_sequence
                        || !returned.insert(reference)
                    {
                        return Err(invalid(
                            "Outcome payload must match the original receipt revision and sequence",
                        ));
                    }
                }
                if !q.payload_omitted
                    && q.receipt
                        .results
                        .iter()
                        .any(|result| !returned.contains(&result.reference))
                {
                    return Err(invalid(
                        "Complete outcome payload must include every receipt result",
                    ));
                }
                Ok(())
            }
            Self::Proposal(q) => {
                response_envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.visibility_epoch,
                )?;
                uuid(&q.proposal_id)?;
                uuid(&q.operation_id)?;
                if q.status != OutcomeStatus::Pending {
                    return Err(invalid("Proposal admission response must be pending"));
                }
                Ok(())
            }
            Self::Changes(q) => {
                response_envelope(
                    q.version,
                    &q.workspace_id,
                    &q.generation,
                    &q.page.visibility_epoch,
                )?;
                q.page.validate()?;
                q.coverage.validate()?;
                count(q.changes.len(), 100)?;
                let mut sequence = None;
                for change in &q.changes {
                    change.reference.validate()?;
                    revision(change.revision, 1)?;
                    revision(change.sequence, 1)?;
                    if sequence.is_some_and(|before| change.sequence < before)
                        || change.sequence > q.coverage.canonical_watermark
                    {
                        return Err(invalid("Invalid change sequence order/watermark"));
                    }
                    sequence = Some(change.sequence);
                    if let Some(record) = &change.record {
                        record.validate()?;
                        if record.metadata.id != change.reference.id
                            || record.draft.kind() != change.reference.kind
                            || record.metadata.revision != change.revision
                            || record.metadata.lifecycle != change.lifecycle
                            || record.metadata.commit_sequence != change.sequence
                        {
                            return Err(invalid("Change payload does not match reference"));
                        }
                    }
                }
                Ok(())
            }
            Self::Error(e) => {
                if e.restart_required != (e.code == GardenErrorCode::RestartRequired) {
                    return Err(invalid("Invalid restart-required error"));
                }
                text(&e.message, 2000)
            }
        }
    }
}
fn response_envelope(version: u32, workspace: &str, generation: &str, epoch: &str) -> Result<()> {
    envelope(version, workspace, generation, &BTreeMap::new())?;
    text(epoch, 200)
}
impl GardenRecord {
    pub fn validate(&self) -> Result<()> {
        let m = &self.metadata;
        uuid(&m.id)?;
        revision(m.revision, 1)?;
        revision(m.commit_sequence, 1)?;
        if m.previous_revision
            != if m.revision == 1 {
                None
            } else {
                Some(m.revision - 1)
            }
        {
            return Err(invalid("Invalid previous revision"));
        }
        instant(&m.transaction_time)?;
        text(&m.actor, 80)?;
        text(&m.contributor, 80)?;
        if let Some(approver) = &m.approver {
            text(approver, 80)?;
        }
        self.draft.validate()?;
        if self
            .draft
            .references()
            .iter()
            .any(|r| matches!(r, RecordRef::Item { .. }))
        {
            return Err(invalid(
                "Stored records cannot contain unresolved item references",
            ));
        }
        Ok(())
    }
}
impl Coverage {
    pub fn validate(&self) -> Result<()> {
        revision(self.canonical_watermark, 0)?;
        revision(self.index_watermark, 0)?;
        if self.index_watermark > self.canonical_watermark || self.complete && self.truncated {
            return Err(invalid("Invalid coverage/watermarks"));
        }
        if let Some(reason) = &self.reason {
            text(reason, 1000)?;
        }
        if !self.complete && self.reason.is_none() {
            return Err(invalid("Incomplete coverage requires reason"));
        }
        if matches!(self.search_mode, SearchMode::Vector | SearchMode::Hybrid)
            && (self.model.is_none() || self.profile.is_none())
        {
            return Err(invalid("Vector results require model/profile"));
        }
        if let Some(model) = &self.model {
            text(model, 200)?;
        }
        if let Some(profile) = &self.profile {
            text(profile, 200)?;
        }
        Ok(())
    }
}
impl PageState {
    pub fn validate(&self) -> Result<()> {
        page(1, &self.next_cursor)?;
        text(&self.visibility_epoch, 200)?;
        text(&self.snapshot_id, 200)?;
        instant(&self.snapshot_expires_at)?;
        Ok(())
    }
}
impl OperationReceipt {
    pub fn validate(&self) -> Result<()> {
        uuid(&self.operation_id)?;
        uuid(&self.operation_generation)?;
        text(&self.actor, 80)?;
        if let Some(id) = &self.proposal_id {
            uuid(id)?;
        }
        optional_time(&self.committed_at)?;
        if let Some(sequence) = self.commit_sequence {
            revision(sequence, 1)?;
        }
        let committed = matches!(
            self.status,
            OutcomeStatus::Approved | OutcomeStatus::Applied
        );
        if committed
            && (self.committed_at.is_none()
                || self.commit_sequence.is_none()
                || self.results.is_empty())
            || !committed
                && (self.committed_at.is_some()
                    || self.commit_sequence.is_some()
                    || !self.results.is_empty())
        {
            return Err(invalid("Invalid receipt commit state"));
        }
        if matches!(
            self.status,
            OutcomeStatus::Pending
                | OutcomeStatus::Approved
                | OutcomeStatus::Dismissed
                | OutcomeStatus::Blocked
        ) && self.proposal_id.is_none()
        {
            return Err(invalid("Proposal outcome requires proposal ID"));
        }
        count(self.results.len(), MAX_BATCH_ITEMS)?;
        let mut keys = BTreeSet::new();
        for result in &self.results {
            item_key(&result.item_key)?;
            result.reference.validate()?;
            revision(result.revision, 1)?;
            if !keys.insert(&result.item_key) {
                return Err(invalid("Duplicate receipt itemKey"));
            }
        }
        Ok(())
    }
}

// Missing optional mutation fields are omitted. Explicit null must not hide a protected field.
fn present_value<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
