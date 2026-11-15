//! Transactional current-record memberships and exact storage consistency auditing.
use super::{
    contracts::{
        AssertionObject, ExternalReference, GardenDraft, GardenErrorCode, GardenKind, GardenRecord,
        Lifecycle, RecordRef,
    },
    history::{self, HISTORY, REVISIONS, TIMES},
    store::{CURRENT, META, REPLACEMENT_INCOMING, REPLACEMENTS, Result, failure, storage_error},
};
use redb::{ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};
use serde_json::{Value, json};
use std::collections::BTreeSet;

const IDS: TableDefinition<&str, u8> = TableDefinition::new("garden_ids");
const OUTGOING: TableDefinition<&str, u8> = TableDefinition::new("garden_outgoing");
const INCOMING: TableDefinition<&str, u8> = TableDefinition::new("garden_incoming");
const PREDICATE: TableDefinition<&str, u8> = TableDefinition::new("garden_predicate");
const VALID_TIME: TableDefinition<&str, u8> = TableDefinition::new("garden_valid_time");
const EVIDENCE: TableDefinition<&str, u8> = TableDefinition::new("garden_evidence");
const SOURCES: TableDefinition<&str, u8> = TableDefinition::new("garden_sources");
const NAMESPACES: TableDefinition<&str, u8> = TableDefinition::new("garden_namespaces");
const TABLES: [TableDefinition<&str, u8>; 8] = [
    IDS, OUTGOING, INCOMING, PREDICATE, VALID_TIME, EVIDENCE, SOURCES, NAMESPACES,
];
const SEQUENCES: TableDefinition<(u64, &str), u64> =
    TableDefinition::new("garden_transaction_sequence");
pub(crate) type Memberships = [BTreeSet<String>; 8];

/// Verified cardinalities from one currently authorized read transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreStats {
    /// Number of current records, including inactive lifecycle states.
    pub current_records: u64,
    /// Number of immutable historical revisions.
    pub history_records: u64,
    /// Number of committed operation receipts.
    pub receipts: u64,
    /// Canonical committed transaction sequence.
    pub sequence: u64,
}

fn corrupt() -> super::contracts::GardenError {
    failure(
        GardenErrorCode::StoreUnavailable,
        "Garden storage consistency check failed",
    )
}

/// Historical generations are evidence only; active request admission still uses state.generation.
pub(crate) fn admitted_generations(tx: &ReadTransaction) -> Result<BTreeSet<String>> {
    let meta = tx.open_table(META).map_err(storage_error)?;
    let state: Value = serde_json::from_slice(
        meta.get("state")
            .map_err(storage_error)?
            .ok_or_else(corrupt)?
            .value(),
    )
    .map_err(storage_error)?;
    let current = state["generation"].as_str().ok_or_else(corrupt)?;
    uuid::Uuid::parse_str(current).map_err(|_| corrupt())?;
    let mut admitted = BTreeSet::from([current.to_owned()]);
    if let Some(bytes) = meta.get("retained_generations").map_err(storage_error)? {
        let retained: Vec<String> = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        for generation in retained {
            uuid::Uuid::parse_str(&generation).map_err(|_| corrupt())?;
            if !admitted.insert(generation) {
                return Err(corrupt());
            }
        }
    }
    Ok(admitted)
}

/// Rebuild derived tables from immutable canonical rows in the staging transaction.
pub(crate) fn rebuild(tx: &WriteTransaction) -> Result<()> {
    for table in TABLES {
        tx.delete_table(table).map_err(storage_error)?;
    }
    tx.delete_table(SEQUENCES).map_err(storage_error)?;
    tx.delete_table(REVISIONS).map_err(storage_error)?;
    tx.delete_table(TIMES).map_err(storage_error)?;
    tx.delete_table(REPLACEMENT_INCOMING)
        .map_err(storage_error)?;
    for table in [QUERY_CURRENT, QUERY_HISTORY] {
        tx.delete_table(table).map_err(storage_error)?;
    }
    for table in [
        VALID_START,
        VALID_END,
        CURRENT_VALID_START,
        CURRENT_VALID_END,
    ] {
        tx.delete_table(table).map_err(storage_error)?;
    }
    initialize(tx)?;
    history::initialize(tx)?;
    let mut revisions = tx.open_table(REVISIONS).map_err(storage_error)?;
    let mut times = tx.open_table(TIMES).map_err(storage_error)?;
    let history = tx.open_table(HISTORY).map_err(storage_error)?;
    let mut previous: Option<GardenRecord> = None;
    for entry in history.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let record: GardenRecord = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        record.validate()?;
        let metadata = &record.metadata;
        if key.value() != (metadata.id.as_str(), metadata.commit_sequence) {
            return Err(corrupt());
        }
        if revisions
            .insert(
                (metadata.id.as_str(), metadata.revision),
                metadata.commit_sequence,
            )
            .map_err(storage_error)?
            .is_some()
        {
            return Err(corrupt());
        }
        if let Some(existing) = times
            .insert(
                history::time_key(&metadata.transaction_time)?,
                metadata.commit_sequence,
            )
            .map_err(storage_error)?
            && existing.value() != metadata.commit_sequence
        {
            return Err(corrupt());
        }
        update(
            tx,
            previous
                .as_ref()
                .filter(|old| old.metadata.id == metadata.id),
            &record,
        )?;
        previous = Some(record);
    }
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    let mut incoming = tx.open_table(REPLACEMENT_INCOMING).map_err(storage_error)?;
    for entry in replacements.iter().map_err(storage_error)? {
        let (from, to) = entry.map_err(storage_error)?;
        incoming
            .insert((to.value(), from.value()), 1)
            .map_err(storage_error)?;
    }
    Ok(())
}

pub(crate) fn initialize(tx: &WriteTransaction) -> Result<()> {
    for definition in TABLES {
        tx.open_table(definition).map_err(storage_error)?;
    }
    tx.open_table(SEQUENCES).map_err(storage_error)?;
    initialize_query_tables(tx)?;
    tx.open_table(META)
        .map_err(storage_error)?
        .insert("query_indexes_v1", &[1_u8][..])
        .map_err(storage_error)?;
    Ok(())
}

fn typed(reference: &RecordRef) -> Result<Value> {
    let RecordRef::Existing { kind, id } = reference else {
        return Err(corrupt());
    };
    Ok(json!({"kind":kind,"id":id}))
}

pub(crate) fn memberships(record: &GardenRecord) -> Result<Memberships> {
    let mut sets: Memberships = std::array::from_fn(|_| BTreeSet::new());
    let id = &record.metadata.id;
    sets[0].insert(json!([record.draft.kind(), id]).to_string());
    let extensions = match &record.draft {
        GardenDraft::Entity {
            entity_type,
            extensions,
            ..
        } => {
            sets[7].insert(json!([entity_type, id]).to_string());
            extensions
        }
        GardenDraft::Assertion {
            predicate,
            extensions,
            ..
        } => {
            sets[7].insert(json!([predicate, id]).to_string());
            extensions
        }
        GardenDraft::Evidence { extensions, .. } => extensions,
    };
    for namespace in extensions.keys() {
        sets[7].insert(json!([namespace, id]).to_string());
    }
    if record.metadata.lifecycle != Lifecycle::Active {
        return Ok(sets);
    }
    match &record.draft {
        GardenDraft::Assertion {
            subject,
            predicate,
            object,
            valid_time,
            evidence,
            ..
        } => {
            sets[1].insert(json!([typed(subject)?, id]).to_string());
            if let AssertionObject::EntityRef { entity_ref } = object {
                sets[2].insert(json!([typed(entity_ref)?, id]).to_string());
            }
            sets[3].insert(json!([predicate, id]).to_string());
            let from = valid_time
                .valid_from
                .as_deref()
                .map(history::time_key)
                .transpose()?;
            let until = valid_time
                .valid_until
                .as_deref()
                .map(history::time_key)
                .transpose()?;
            sets[4].insert(json!([from, until, id]).to_string());
            for evidence in evidence {
                sets[5].insert(json!([typed(evidence)?, id]).to_string());
            }
        }
        GardenDraft::Evidence { source, .. } => {
            sets[6].insert(json!([source, id]).to_string());
        }
        GardenDraft::Entity { .. } => {}
    }
    Ok(sets)
}

pub(crate) fn update(
    tx: &WriteTransaction,
    previous: Option<&GardenRecord>,
    current: &GardenRecord,
) -> Result<()> {
    update_query(tx, previous, current)?;
    let previous = previous.map(memberships).transpose()?;
    let next = memberships(current)?;
    for (index, definition) in TABLES.into_iter().enumerate() {
        let mut table = tx.open_table(definition).map_err(storage_error)?;
        if let Some(previous) = &previous {
            for key in &previous[index] {
                table.remove(key.as_str()).map_err(storage_error)?;
            }
        }
        for key in &next[index] {
            table.insert(key.as_str(), 1).map_err(storage_error)?;
        }
    }
    tx.open_table(SEQUENCES)
        .map_err(storage_error)?
        .insert(
            (
                current.metadata.commit_sequence,
                current.metadata.id.as_str(),
            ),
            current.metadata.revision,
        )
        .map_err(storage_error)?;
    Ok(())
}

/// Resolve only indexed active assertion dependencies, with an explicit work ceiling.
pub(crate) fn dependents(
    tx: &WriteTransaction,
    reference: &ExternalReference,
    limit: usize,
    mut authorize: impl FnMut(&ExternalReference) -> Result<()>,
) -> Result<Vec<String>> {
    let tables: &[TableDefinition<&str, u8>] = match reference.kind {
        GardenKind::Entity => &[OUTGOING, INCOMING],
        GardenKind::Evidence => &[EVIDENCE],
        GardenKind::Assertion => &[],
    };
    let reference = serde_json::to_value(reference).map_err(storage_error)?;
    let prefix = format!("[{reference},");
    let mut ids = BTreeSet::new();
    for definition in tables {
        let table = tx.open_table(*definition).map_err(storage_error)?;
        for entry in table.range(prefix.as_str()..).map_err(storage_error)? {
            let (key, _) = entry.map_err(storage_error)?;
            if !key.value().starts_with(&prefix) {
                break;
            }
            let key: Value = serde_json::from_str(key.value()).map_err(storage_error)?;
            let id = key[1].as_str().ok_or_else(corrupt)?;
            authorize(&ExternalReference {
                kind: GardenKind::Assertion,
                id: id.to_owned(),
            })?;
            ids.insert(id.to_owned());
            if ids.len() > limit {
                return Err(failure(
                    GardenErrorCode::LimitExceeded,
                    "Graph dependency work limit exceeded",
                ));
            }
        }
    }
    Ok(ids.into_iter().collect())
}

mod audit;
pub(crate) use audit::audit_read;

// Typed, versioned query indexes are derived from canonical revisions in the same
// transaction. Old G4 memberships remain unchanged for compatibility audits.
const QUERY_CURRENT: TableDefinition<(u8, &str, &str, u64), u8> =
    TableDefinition::new("garden_query_current_v1");
const QUERY_HISTORY: TableDefinition<(u8, &str, &str, u64), u8> =
    TableDefinition::new("garden_query_history_v1");
const VALID_START: TableDefinition<((i64, u32), &str, u64), u8> =
    TableDefinition::new("garden_valid_start_v1");
const VALID_END: TableDefinition<((i64, u32), &str, u64), u8> =
    TableDefinition::new("garden_valid_end_v1");
const CURRENT_VALID_START: TableDefinition<((i64, u32), &str, u64), u8> =
    TableDefinition::new("garden_current_valid_start_v1");
const CURRENT_VALID_END: TableDefinition<((i64, u32), &str, u64), u8> =
    TableDefinition::new("garden_current_valid_end_v1");
pub(crate) type QueryMemberships = BTreeSet<(u8, String)>;
pub(crate) type ValidBounds = ((i64, u32), (i64, u32));
#[derive(Debug, Clone)]
pub(crate) enum CandidateFilter {
    All,
    Kind(GardenKind),
    EntityType(String),
    Predicate(String),
    Property {
        namespace: String,
        value: super::contracts::TypedValue,
    },
    Adjacent {
        reference: ExternalReference,
        direction: super::contracts::Direction,
    },
    Literal(String),
    ValidAt(String),
}
#[derive(Debug)]
pub(crate) struct CandidatePage {
    pub ids: Vec<String>,
    pub truncated: bool,
}

fn initialize_query_tables(tx: &WriteTransaction) -> Result<()> {
    tx.open_table(QUERY_CURRENT).map_err(storage_error)?;
    tx.open_table(QUERY_HISTORY).map_err(storage_error)?;
    tx.open_table(VALID_START).map_err(storage_error)?;
    tx.open_table(VALID_END).map_err(storage_error)?;
    tx.open_table(CURRENT_VALID_START).map_err(storage_error)?;
    tx.open_table(CURRENT_VALID_END).map_err(storage_error)?;
    Ok(())
}
fn reference_term(reference: &ExternalReference) -> Result<String> {
    serde_json::to_string(reference).map_err(storage_error)
}
fn property_term(namespace: &str, value: &Value) -> String {
    // f64 equality treats signed zero as equal; derived keys must do the same.
    fn normalize(value: &Value) -> Value {
        match value {
            Value::Number(number) if number.is_f64() && number.as_f64() == Some(0.0) => json!(0.0),
            Value::Array(values) => Value::Array(values.iter().map(normalize).collect()),
            Value::Object(values) => Value::Object(
                values
                    .iter()
                    .map(|(key, value)| (key.clone(), normalize(value)))
                    .collect(),
            ),
            value => value.clone(),
        }
    }
    json!([namespace, normalize(value)]).to_string()
}
pub(crate) fn query_memberships(record: &GardenRecord) -> Result<QueryMemberships> {
    let mut terms = BTreeSet::from([
        (0, String::new()),
        (
            1,
            serde_json::to_string(&record.draft.kind()).map_err(storage_error)?,
        ),
    ]);
    let extensions = match &record.draft {
        GardenDraft::Entity {
            entity_type,
            extensions,
            ..
        } => {
            terms.insert((2, entity_type.clone()));
            extensions
        }
        GardenDraft::Assertion {
            subject,
            predicate,
            object,
            evidence,
            extensions,
            ..
        } => {
            terms.insert((3, predicate.clone()));
            let RecordRef::Existing { kind, id } = subject else {
                return Err(corrupt());
            };
            terms.insert((
                4,
                reference_term(&ExternalReference {
                    kind: *kind,
                    id: id.clone(),
                })?,
            ));
            match object {
                AssertionObject::EntityRef { entity_ref } => {
                    let RecordRef::Existing { kind, id } = entity_ref else {
                        return Err(corrupt());
                    };
                    terms.insert((
                        5,
                        reference_term(&ExternalReference {
                            kind: *kind,
                            id: id.clone(),
                        })?,
                    ));
                }
                AssertionObject::Value { value } => {
                    terms.insert((
                        6,
                        property_term(
                            predicate,
                            &serde_json::to_value(value).map_err(storage_error)?,
                        ),
                    ));
                }
            }
            for reference in evidence {
                let RecordRef::Existing { kind, id } = reference else {
                    return Err(corrupt());
                };
                terms.insert((
                    7,
                    reference_term(&ExternalReference {
                        kind: *kind,
                        id: id.clone(),
                    })?,
                ));
            }
            extensions
        }
        GardenDraft::Evidence { extensions, .. } => extensions,
    };
    for (namespace, extension) in extensions {
        terms.insert((8, namespace.clone()));
        terms.insert((6, property_term(namespace, &extension.payload)));
    }
    for term in super::store::search::literal_terms(&super::store::search::record_text(record)) {
        terms.insert((9, term));
    }
    Ok(terms)
}
pub(crate) fn valid_bounds(record: &GardenRecord) -> Result<Option<ValidBounds>> {
    let GardenDraft::Assertion { valid_time, .. } = &record.draft else {
        return Ok(None);
    };
    let from = valid_time
        .valid_from
        .as_deref()
        .map(history::time_key)
        .transpose()?
        .unwrap_or((i64::MIN, 0));
    let until = valid_time
        .valid_until
        .as_deref()
        .map(history::time_key)
        .transpose()?
        .unwrap_or((i64::MAX, u32::MAX));
    Ok(Some((from, until)))
}
fn update_query(
    tx: &WriteTransaction,
    previous: Option<&GardenRecord>,
    current: &GardenRecord,
) -> Result<()> {
    if let Some(previous) = previous
        && let Some((from, until)) = valid_bounds(previous)?
    {
        tx.open_table(CURRENT_VALID_START)
            .map_err(storage_error)?
            .remove((from, previous.metadata.id.as_str(), 0))
            .map_err(storage_error)?;
        tx.open_table(CURRENT_VALID_END)
            .map_err(storage_error)?
            .remove((until, previous.metadata.id.as_str(), 0))
            .map_err(storage_error)?;
    }
    insert_current_time(tx, current)?;
    {
        let mut table = tx.open_table(QUERY_CURRENT).map_err(storage_error)?;
        if let Some(previous) = previous {
            for (tag, term) in query_memberships(previous)? {
                table
                    .remove((tag, term.as_str(), previous.metadata.id.as_str(), 0))
                    .map_err(storage_error)?;
            }
        }
        for (tag, term) in query_memberships(current)? {
            table
                .insert((tag, term.as_str(), current.metadata.id.as_str(), 0), 1)
                .map_err(storage_error)?;
        }
    }
    append_query_history(tx, current)
}
fn insert_current_time(tx: &WriteTransaction, record: &GardenRecord) -> Result<()> {
    if let Some((from, until)) = valid_bounds(record)? {
        tx.open_table(CURRENT_VALID_START)
            .map_err(storage_error)?
            .insert((from, record.metadata.id.as_str(), 0), 1)
            .map_err(storage_error)?;
        tx.open_table(CURRENT_VALID_END)
            .map_err(storage_error)?
            .insert((until, record.metadata.id.as_str(), 0), 1)
            .map_err(storage_error)?;
    }
    Ok(())
}
fn append_query_history(tx: &WriteTransaction, current: &GardenRecord) -> Result<()> {
    let mut table = tx.open_table(QUERY_HISTORY).map_err(storage_error)?;
    for (tag, term) in query_memberships(current)? {
        table
            .insert(
                (
                    tag,
                    term.as_str(),
                    current.metadata.id.as_str(),
                    current.metadata.commit_sequence,
                ),
                1,
            )
            .map_err(storage_error)?;
    }
    if let Some((from, until)) = valid_bounds(current)? {
        tx.open_table(VALID_START)
            .map_err(storage_error)?
            .insert(
                (
                    from,
                    current.metadata.id.as_str(),
                    current.metadata.commit_sequence,
                ),
                1,
            )
            .map_err(storage_error)?;
        tx.open_table(VALID_END)
            .map_err(storage_error)?
            .insert(
                (
                    until,
                    current.metadata.id.as_str(),
                    current.metadata.commit_sequence,
                ),
                1,
            )
            .map_err(storage_error)?;
    }
    Ok(())
}
/// Only the first open of a pre-query Garden file enumerates canonical history.
/// This additive upgrade preserves state, generation, principals and receipts.
pub(crate) fn upgrade(tx: &WriteTransaction) -> Result<()> {
    if tx
        .open_table(META)
        .map_err(storage_error)?
        .get("query_indexes_v1")
        .map_err(storage_error)?
        .is_some()
    {
        return Ok(());
    }
    initialize_query_tables(tx)?;
    {
        let records = tx.open_table(CURRENT).map_err(storage_error)?;
        let mut index = tx.open_table(QUERY_CURRENT).map_err(storage_error)?;
        for entry in records.iter().map_err(storage_error)? {
            let (_, bytes) = entry.map_err(storage_error)?;
            let record: GardenRecord =
                serde_json::from_slice(bytes.value()).map_err(storage_error)?;
            insert_current_time(tx, &record)?;
            for (tag, term) in query_memberships(&record)? {
                index
                    .insert((tag, term.as_str(), record.metadata.id.as_str(), 0), 1)
                    .map_err(storage_error)?;
            }
        }
    }
    for entry in tx
        .open_table(HISTORY)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (_, bytes) = entry.map_err(storage_error)?;
        let record: GardenRecord = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        append_query_history(tx, &record)?;
    }
    tx.open_table(META)
        .map_err(storage_error)?
        .insert("query_indexes_v1", &[1_u8][..])
        .map_err(storage_error)?;
    Ok(())
}

/// Index rows are candidates only; callers must recheck the selected historical
/// revision and current graph visibility before disclosing anything.
pub(crate) fn candidates(
    tx: &ReadTransaction,
    filter: &CandidateFilter,
    sequence: u64,
    after: Option<&str>,
    limit: usize,
) -> Result<CandidatePage> {
    use super::contracts::Direction;
    let terms = match filter {
        CandidateFilter::All => vec![(0, String::new())],
        CandidateFilter::Kind(kind) => {
            vec![(1, serde_json::to_string(kind).map_err(storage_error)?)]
        }
        CandidateFilter::EntityType(value) => vec![(2, value.clone())],
        CandidateFilter::Predicate(value) => vec![(3, value.clone())],
        CandidateFilter::Property { namespace, value } => vec![(
            6,
            property_term(
                namespace,
                &serde_json::to_value(value).map_err(storage_error)?,
            ),
        )],
        CandidateFilter::Adjacent {
            reference,
            direction,
        } => {
            let term = reference_term(reference)?;
            match direction {
                Direction::Incoming => vec![(5, term)],
                Direction::Outgoing => vec![(4, term)],
                Direction::Both => vec![(4, term.clone()), (5, term)],
            }
        }
        CandidateFilter::Literal(text) => {
            let normalized = text.to_lowercase();
            let chars: Vec<_> = normalized.chars().collect();
            let gram: String = chars.iter().take(3).collect();
            vec![(9, gram)]
        }
        CandidateFilter::ValidAt(at) => return time_candidates(tx, at, sequence, after, limit),
    };
    let table = tx.open_table(QUERY_HISTORY).map_err(storage_error)?;
    let mut ids = BTreeSet::new();
    let mut work = 0_usize;
    let mut truncated = false;
    let scan_limit = limit.max(1).saturating_mul(8);
    for (tag, term) in terms {
        for entry in table
            .range(
                (tag, term.as_str(), after.unwrap_or(""), 0)
                    ..=(tag, term.as_str(), "\u{10ffff}", u64::MAX),
            )
            .map_err(storage_error)?
        {
            let (key, _) = entry.map_err(storage_error)?;
            let (_, _, id, seq) = key.value();
            if after.is_some_and(|after| id <= after) {
                continue;
            }
            work += 1;
            if seq <= sequence {
                ids.insert(id.to_owned());
            }
            if ids.len() >= limit || work >= scan_limit {
                truncated = true;
                break;
            }
        }
        if truncated {
            break;
        }
    }
    Ok(CandidatePage {
        ids: ids.into_iter().collect(),
        truncated,
    })
}
fn time_candidates(
    tx: &ReadTransaction,
    at: &str,
    sequence: u64,
    after: Option<&str>,
    limit: usize,
) -> Result<CandidatePage> {
    let at = history::time_key(at)?;
    let table = tx.open_table(VALID_START).map_err(storage_error)?;
    let mut ids = BTreeSet::new();
    let mut truncated = false;
    for (work, entry) in table
        .range(..=(at, "\u{10ffff}", u64::MAX))
        .map_err(storage_error)?
        .enumerate()
    {
        let (key, _) = entry.map_err(storage_error)?;
        let (_, id, seq) = key.value();
        if seq <= sequence && after.is_none_or(|after| id > after) {
            ids.insert(id.to_owned());
        }
        if ids.len() >= limit || work + 1 >= limit.max(1).saturating_mul(8) {
            truncated = true;
            break;
        }
    }
    Ok(CandidatePage {
        ids: ids.into_iter().collect(),
        truncated,
    })
}
/// Ordered immutable event identifiers; the sequence+ID pair is indivisible.
pub(crate) fn events(
    tx: &ReadTransaction,
    after_sequence: u64,
    after_id: &str,
    cutoff: u64,
    limit: usize,
) -> Result<Vec<(u64, String, u64)>> {
    let table = tx.open_table(SEQUENCES).map_err(storage_error)?;
    let mut rows = Vec::new();
    for entry in table
        .range((after_sequence, after_id)..=(cutoff, "\u{10ffff}"))
        .map_err(storage_error)?
    {
        let (key, revision) = entry.map_err(storage_error)?;
        let (sequence, id) = key.value();
        if sequence == after_sequence && id <= after_id {
            continue;
        }
        rows.push((sequence, id.to_owned(), revision.value()));
        if rows.len() >= limit {
            break;
        }
    }
    Ok(rows)
}
