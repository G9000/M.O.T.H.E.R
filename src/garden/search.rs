//! Optional rebuildable vector candidates; canonical records and access remain authoritative.
use super::*;
use crate::embedding::{EmbeddingProfile, LocalEmbedder};

const PROFILES: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_vector_profiles_v1");
const VECTORS: TableDefinition<(&str, &str, u64), &[u8]> =
    TableDefinition::new("garden_vectors_v1");
const MAX_PREPARE_RECORDS: usize = 100;
const MAX_PREPARE_BYTES: usize = 4 * 1024 * 1024;

/// Trusted runtime vectorizer. MCP arguments cannot select or construct one.
pub trait Vectorizer {
    fn profile(&self) -> EmbeddingProfile;
    fn vectors(&self, text: &str) -> std::result::Result<EmbeddingVectors, GardenError>;
}
/// Bounded passages from one canonical record, never an alternate source of record text.
#[derive(Debug, Clone)]
pub struct EmbeddingVectors {
    pub vectors: Vec<Vec<f32>>,
    pub partial: bool,
}
/// Coverage of only the explicitly selected preparation inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparationReport {
    pub prepared: usize,
    pub skipped: usize,
    pub model: Option<String>,
    pub profile: Option<String>,
    pub index_watermark: u64,
    pub reason: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    generation: String,
    reference: ExternalReference,
    revision: u64,
    digest: String,
    vectors: Vec<Vec<f32>>,
    partial: bool,
}
#[derive(Debug, Serialize, Deserialize)]
struct ProfileEntry {
    generation: String,
    profile: EmbeddingProfile,
}
#[derive(Debug, Clone)]
pub(crate) struct SearchCoverage {
    pub mode: SearchMode,
    pub model: Option<String>,
    pub profile: Option<String>,
    pub index_watermark: u64,
    pub reason: Option<String>,
    pub vector_available: bool,
}

pub(crate) fn record_text(record: &GardenRecord) -> String {
    let text = match &record.draft {
        GardenDraft::Entity {
            entity_type,
            display_name,
            aliases,
            ..
        } => format!("{entity_type}\n{display_name}\n{}", aliases.join("\n")),
        GardenDraft::Assertion {
            predicate, object, ..
        } => {
            let value = match object {
                AssertionObject::Value { value } => {
                    serde_json::to_string(value).unwrap_or_default()
                }
                AssertionObject::EntityRef { .. } => String::new(),
            };
            format!("{predicate}\n{value}")
        }
        GardenDraft::Evidence {
            source, excerpt, ..
        } => format!(
            "{}\n{}",
            serde_json::to_string(source).unwrap_or_default(),
            excerpt.as_deref().unwrap_or_default()
        ),
    };
    text.to_lowercase()
}
pub(crate) fn literal_terms(text: &str) -> BTreeSet<String> {
    let chars: Vec<_> = text.to_lowercase().chars().collect();
    (1..=3)
        .flat_map(|size| {
            chars
                .windows(size)
                .map(|window| window.iter().collect::<String>())
        })
        .collect()
}
fn descriptor(query: &VectorQuery) -> EmbeddingProfile {
    EmbeddingProfile {
        version: 1,
        model: query.model.clone(),
        profile: query.profile.clone(),
        dimensions: query.dimensions as usize,
    }
}
fn validate_profile(profile: &EmbeddingProfile) -> Result<()> {
    if profile.version != 1
        || profile.model.trim().is_empty()
        || profile.model.len() > 200
        || profile.profile.trim().is_empty()
        || profile.profile.len() > 200
        || profile.dimensions == 0
        || profile.dimensions > 4096
    {
        return Err(failure(
            GardenErrorCode::InvalidInput,
            "Invalid embedding profile",
        ));
    }
    Ok(())
}
fn profile_available(tx: &redb::ReadTransaction, profile: &EmbeddingProfile) -> Result<bool> {
    let table = match tx.open_table(PROFILES) {
        Ok(table) => table,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(false),
        Err(error) => return Err(storage_error(error)),
    };
    let key = profile.fingerprint();
    let Some(value) = table.get(key.as_str()).map_err(storage_error)? else {
        return Ok(false);
    };
    let stored: ProfileEntry = decode(value.value())?;
    Ok(stored.profile == *profile && stored.generation == state_read(tx)?.generation)
}
pub(super) fn coverage(
    tx: &redb::ReadTransaction,
    request: &QueryRequest,
    principal: &Principal,
) -> Result<SearchCoverage> {
    let mut result = SearchCoverage {
        mode: if request.text.is_some() {
            SearchMode::Literal
        } else {
            SearchMode::Exact
        },
        model: None,
        profile: None,
        index_watermark: 0,
        reason: None,
        vector_available: false,
    };
    if let Some(vector) = &request.vector {
        if matches!(principal.garden, GardenAccess::All {})
            && profile_available(tx, &descriptor(vector))?
        {
            result.mode = if request.text.is_some() {
                SearchMode::Hybrid
            } else {
                SearchMode::Vector
            };
            result.model = Some(vector.model.clone());
            result.profile = Some(vector.profile.clone());
            result.vector_available = true;
            result.reason = Some("vector_index_partial".into());
        } else {
            result.mode = SearchMode::LiteralFallback;
            result.reason = Some("authorized_vector_index_unavailable".into());
        }
    }
    Ok(result)
}
fn norm(values: &[f32]) -> f64 {
    values
        .iter()
        .map(|value| f64::from(*value).powi(2))
        .sum::<f64>()
        .sqrt()
}
fn validate_vectors(vectors: &EmbeddingVectors, dimensions: usize) -> Result<()> {
    if vectors.vectors.is_empty()
        || vectors.vectors.len() > 32
        || vectors.vectors.iter().any(|values| {
            values.len() != dimensions
                || values.iter().any(|v| !v.is_finite())
                || !norm(values).is_finite()
                || norm(values) <= f64::EPSILON
        })
    {
        return Err(failure(
            GardenErrorCode::InvalidInput,
            "Invalid derived embedding vectors",
        ));
    }
    Ok(())
}
fn cached(
    tx: &redb::ReadTransaction,
    record: &GardenRecord,
    vector: &VectorQuery,
) -> Result<Option<Entry>> {
    let profile = descriptor(vector);
    if !profile_available(tx, &profile)? {
        return Ok(None);
    }
    let table = match tx.open_table(VECTORS) {
        Ok(table) => table,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
        Err(error) => return Err(storage_error(error)),
    };
    let key = profile.fingerprint();
    let Some(bytes) = table
        .get((
            key.as_str(),
            record.metadata.id.as_str(),
            record.metadata.revision,
        ))
        .map_err(storage_error)?
    else {
        return Ok(None);
    };
    let entry: Entry = decode(bytes.value())?;
    if entry.digest != contracts::hash(&canonical_json(record)?)
        || entry.generation != state_read(tx)?.generation
        || entry.revision != record.metadata.revision
        || entry.reference.id != record.metadata.id
        || entry.reference.kind != record.draft.kind()
    {
        return Ok(None);
    }
    validate_vectors(
        &EmbeddingVectors {
            vectors: entry.vectors.clone(),
            partial: entry.partial,
        },
        profile.dimensions,
    )?;
    Ok(Some(entry))
}
pub(super) fn vector_available_for(
    tx: &redb::ReadTransaction,
    record: &GardenRecord,
    request: &QueryRequest,
) -> Result<bool> {
    match &request.vector {
        Some(vector) => Ok(cached(tx, record, vector)?.is_some()),
        None => Ok(false),
    }
}
pub(crate) fn score(
    tx: &redb::ReadTransaction,
    record: &GardenRecord,
    request: &QueryRequest,
) -> Result<Option<ComponentScores>> {
    let lexical = request
        .text
        .as_ref()
        .filter(|text| record_text(record).contains(&text.to_lowercase()))
        .map(|_| 1.0);
    let mut similarity: Option<f64> = None;
    if let Some(query) = &request.vector {
        let query_norm = query.values.iter().map(|v| v * v).sum::<f64>().sqrt();
        if query_norm.is_finite()
            && query_norm > f64::EPSILON
            && let Some(entry) = cached(tx, record, query)?
        {
            for values in entry.vectors {
                let dot = values
                    .iter()
                    .zip(&query.values)
                    .map(|(a, b)| f64::from(*a) * b)
                    .sum::<f64>();
                let value = (dot / (norm(&values) * query_norm)).clamp(-1.0, 1.0);
                if value > 0.0 {
                    similarity = Some(similarity.map_or(value, |old| old.max(value)));
                }
            }
        }
    }
    if request.text.is_none() && request.vector.is_none()
        || lexical.is_some()
        || similarity.is_some()
    {
        Ok(Some(ComponentScores {
            lexical,
            vector: similarity,
        }))
    } else {
        Ok(None)
    }
}
pub(crate) fn vector_candidates(
    tx: &redb::ReadTransaction,
    vector: &VectorQuery,
    limit: usize,
) -> Result<(Vec<ExternalReference>, bool)> {
    let profile = descriptor(vector);
    if !profile_available(tx, &profile)? {
        return Ok((vec![], false));
    }
    let table = tx.open_table(VECTORS).map_err(storage_error)?;
    let key = profile.fingerprint();
    let mut ids = BTreeSet::new();
    for (visited, row) in table
        .range((key.as_str(), "", 0)..=(key.as_str(), "\u{10ffff}", u64::MAX))
        .map_err(storage_error)?
        .enumerate()
    {
        if visited >= limit {
            return Ok((ids.into_iter().collect(), true));
        }
        let (_, bytes) = row.map_err(storage_error)?;
        let entry: Entry = decode(bytes.value())?;
        ids.insert(entry.reference);
        if ids.len() > limit {
            ids.pop_last();
            return Ok((ids.into_iter().collect(), true));
        }
    }
    Ok((ids.into_iter().collect(), false))
}

struct LocalVectorizer(LocalEmbedder);
impl Vectorizer for LocalVectorizer {
    fn profile(&self) -> EmbeddingProfile {
        crate::search::embedding_profile()
    }
    fn vectors(&self, text: &str) -> Result<EmbeddingVectors> {
        let plan = self.0.passages(text).map_err(storage_error)?;
        let vectors = plan
            .passages
            .iter()
            .map(|passage| self.0.embed(passage).map_err(storage_error))
            .collect::<Result<Vec<_>>>()?;
        Ok(EmbeddingVectors {
            vectors,
            partial: plan.partial,
        })
    }
}
impl GardenStore {
    fn search_sources(
        &self,
        auth: &Auth,
        references: &[ExternalReference],
    ) -> Result<(State, Vec<GardenRecord>)> {
        if references.is_empty() || references.len() > MAX_PREPARE_RECORDS {
            return Err(failure(
                GardenErrorCode::LimitExceeded,
                "Select one to 100 graph sources",
            ));
        }
        let tx = self.db.begin_read().map_err(storage_error)?;
        let state = state_read(&tx)?;
        let principal = principal_read(&tx, auth, GardenAction::Read)?;
        let mut records = Vec::new();
        let mut bytes = 0usize;
        for reference in references {
            reference.validate()?;
            require_allowed(&principal, reference)?;
            let (current, replacements) =
                context_read(&tx, &principal, vec![reference.id.clone()])?;
            let record = current
                .get(&reference.id)
                .filter(|r| {
                    r.draft.kind() == reference.kind && r.metadata.lifecycle == Lifecycle::Active
                })
                .ok_or_else(not_found)?;
            visible(&principal, record, &current, &replacements)?;
            bytes = bytes.saturating_add(encode(record)?.len());
            if bytes > MAX_PREPARE_BYTES {
                return Err(failure(
                    GardenErrorCode::LimitExceeded,
                    "Selected embedding sources exceed byte budget",
                ));
            }
            records.push(record.clone());
        }
        Ok((state, records))
    }
    /// Prepare only selected authorized sources with already-installed verified local assets.
    /// # Errors
    /// Rejects unauthorized sources; missing local assets return an explicit unavailable report.
    pub fn prepare_search(
        &self,
        auth: &Auth,
        references: &[ExternalReference],
    ) -> Result<PreparationReport> {
        self.search_sources(auth, references)?;
        let model = crate::embedding::model_directory().and_then(|dir| LocalEmbedder::load(&dir));
        match model {
            Ok(model) => self.prepare_search_with(auth, references, &LocalVectorizer(model)),
            Err(_) => Ok(PreparationReport {
                prepared: 0,
                skipped: references.len(),
                model: None,
                profile: None,
                index_watermark: 0,
                reason: Some("local_model_unavailable".into()),
            }),
        }
    }
    /// Install a trusted runtime vectorizer, never one supplied by an MCP request.
    /// # Errors
    /// Rechecks authority and canonical revisions after computation; rejects invalid vectors.
    pub fn prepare_search_with(
        &self,
        auth: &Auth,
        references: &[ExternalReference],
        vectorizer: &dyn Vectorizer,
    ) -> Result<PreparationReport> {
        if references.is_empty() || references.len() > MAX_PREPARE_RECORDS {
            return Err(failure(
                GardenErrorCode::LimitExceeded,
                "Select one to 100 graph sources",
            ));
        }
        let profile = vectorizer.profile();
        validate_profile(&profile)?;
        let (source_state, records) = self.search_sources(auth, references)?;
        let computed = records
            .iter()
            .map(|record| {
                let vectors = vectorizer.vectors(&record_text(record))?;
                validate_vectors(&vectors, profile.dimensions)?;
                Ok((record, vectors))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut tx = self.db.begin_write().map_err(storage_error)?;
        tx.set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let principal = principal_write(&tx, auth, GardenAction::Read)?;
        let current_state = state_write(&tx)?;
        envelope(
            &current_state,
            &source_state.workspace_id,
            &source_state.generation,
        )?;
        let key = profile.fingerprint();
        let mut prepared = 0;
        tx.open_table(PROFILES).map_err(storage_error)?;
        tx.open_table(VECTORS).map_err(storage_error)?;
        for (record, vectors) in computed {
            let (current, replacements) =
                context_write(&tx, &principal, vec![record.metadata.id.clone()])?;
            let now = current.get(&record.metadata.id).ok_or_else(not_found)?;
            visible(&principal, now, &current, &replacements)?;
            if now != record || now.metadata.lifecycle != Lifecycle::Active {
                continue;
            }
            let entry = Entry {
                generation: current_state.generation.clone(),
                reference: ExternalReference {
                    kind: record.draft.kind(),
                    id: record.metadata.id.clone(),
                },
                revision: record.metadata.revision,
                digest: contracts::hash(&canonical_json(record)?),
                vectors: vectors.vectors,
                partial: vectors.partial,
            };
            tx.open_table(VECTORS)
                .map_err(storage_error)?
                .insert(
                    (
                        key.as_str(),
                        record.metadata.id.as_str(),
                        record.metadata.revision,
                    ),
                    encode(&entry)?.as_slice(),
                )
                .map_err(storage_error)?;
            prepared += 1;
        }
        if prepared > 0 {
            tx.open_table(PROFILES)
                .map_err(storage_error)?
                .insert(
                    key.as_str(),
                    encode(&ProfileEntry {
                        generation: current_state.generation,
                        profile: profile.clone(),
                    })?
                    .as_slice(),
                )
                .map_err(storage_error)?;
        }
        tx.commit().map_err(storage_error)?;
        Ok(PreparationReport {
            prepared,
            skipped: references.len() - prepared,
            model: Some(profile.model),
            profile: Some(profile.profile),
            index_watermark: 0,
            reason: Some("vector_index_partial".into()),
        })
    }
}
