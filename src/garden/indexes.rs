//! Transactional current-record memberships and exact storage consistency auditing.
use super::{
    contracts::{
        AssertionObject, BatchRequest, ExternalReference, GardenDraft, GardenErrorCode, GardenKind,
        GardenRecord, Lifecycle, Mutation, OperationReceipt, RecordRef,
    },
    history::{self, HISTORY, REVISIONS, TIMES},
    store::{
        CURRENT, META, RECEIPTS, REPLACEMENT_INCOMING, REPLACEMENTS, Result, failure, storage_error,
    },
};
use redb::{ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

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

/// Check exact canonical/history and index equality without exposing raw snapshots.
pub(crate) fn audit_read(tx: &ReadTransaction) -> Result<StoreStats> {
    let admitted_generations = admitted_generations(tx)?;
    let mut expected: Memberships = std::array::from_fn(|_| BTreeSet::new());
    let mut current = BTreeMap::new();
    let canonical = tx.open_table(CURRENT).map_err(storage_error)?;
    for entry in canonical.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let record: GardenRecord = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        record.validate()?;
        if key.value() != record.metadata.id {
            return Err(corrupt());
        }
        for (set, keys) in expected.iter_mut().zip(memberships(&record)?) {
            set.extend(keys);
        }
        current.insert(record.metadata.id.clone(), record);
    }
    for (definition, expected) in TABLES.into_iter().zip(expected) {
        let table = tx.open_table(definition).map_err(storage_error)?;
        let mut actual = BTreeSet::new();
        for entry in table.iter().map_err(storage_error)? {
            let (key, value) = entry.map_err(storage_error)?;
            if value.value() != 1 {
                return Err(corrupt());
            }
            actual.insert(key.value().to_owned());
        }
        if actual != expected {
            return Err(corrupt());
        }
    }
    // Active adjacency is meaningful only if its typed endpoints remain active.
    for record in current
        .values()
        .filter(|record| record.metadata.lifecycle == Lifecycle::Active)
    {
        for reference in record.draft.references() {
            let RecordRef::Existing { kind, id } = reference else {
                return Err(corrupt());
            };
            let endpoint = current.get(id).ok_or_else(corrupt)?;
            if endpoint.draft.kind() != *kind || endpoint.metadata.lifecycle != Lifecycle::Active {
                return Err(corrupt());
            }
        }
    }
    let replacements: BTreeMap<_, _> = tx
        .open_table(REPLACEMENTS)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
        .map(|entry| {
            let (source, target) = entry.map_err(storage_error)?;
            Ok((source.value().to_owned(), target.value().to_owned()))
        })
        .collect::<Result<_>>()?;
    let expected_reverse: BTreeSet<_> = replacements
        .iter()
        .map(|(source, target)| (target.clone(), source.clone()))
        .collect();
    let mut actual_reverse = BTreeSet::new();
    for entry in tx
        .open_table(REPLACEMENT_INCOMING)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (key, value) = entry.map_err(storage_error)?;
        let (target, source) = key.value();
        if value.value() != 1 {
            return Err(corrupt());
        }
        actual_reverse.insert((target.to_owned(), source.to_owned()));
    }
    if actual_reverse != expected_reverse {
        return Err(corrupt());
    }
    for record in current.values() {
        if (record.metadata.lifecycle == Lifecycle::Superseded)
            != replacements.contains_key(&record.metadata.id)
        {
            return Err(corrupt());
        }
    }
    for (source, target) in &replacements {
        let source_record = current.get(source).ok_or_else(corrupt)?;
        let target_record = current.get(target).ok_or_else(corrupt)?;
        if source_record.metadata.lifecycle != Lifecycle::Superseded
            || source_record.draft.kind() != target_record.draft.kind()
            || target_record.metadata.lifecycle == Lifecycle::Retracted
        {
            return Err(corrupt());
        }
        let mut visited = BTreeSet::new();
        let mut cursor = source;
        while let Some(next) = replacements.get(cursor) {
            if !visited.insert(cursor) {
                return Err(corrupt());
            }
            cursor = next;
        }
        if current
            .get(cursor)
            .is_none_or(|record| record.metadata.lifecycle != Lifecycle::Active)
        {
            return Err(corrupt());
        }
    }
    let mut history_records: BTreeMap<(String, u64), GardenRecord> = BTreeMap::new();
    let mut latest: BTreeMap<String, GardenRecord> = BTreeMap::new();
    let mut revisions = BTreeMap::new();
    let mut sequences = BTreeMap::new();
    let mut times = BTreeMap::new();
    let mut sequence_times = BTreeMap::new();
    for entry in tx
        .open_table(HISTORY)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (key, bytes) = entry.map_err(storage_error)?;
        let (id, sequence) = key.value();
        let record: GardenRecord = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        record.validate()?;
        let meta = &record.metadata;
        if id != meta.id || sequence != meta.commit_sequence || sequence == 0 {
            return Err(corrupt());
        }
        let time = history::time_key(&meta.transaction_time)?;
        if times
            .insert(time, sequence)
            .is_some_and(|existing| existing != sequence)
            || sequence_times
                .insert(sequence, time)
                .is_some_and(|existing| existing != time)
        {
            return Err(corrupt());
        }
        if let Some(previous) = latest.get(id) {
            if meta.revision != previous.metadata.revision + 1
                || meta.previous_revision != Some(previous.metadata.revision)
                || meta.contributor != previous.metadata.contributor
            {
                return Err(corrupt());
            }
        } else if meta.revision != 1 || meta.previous_revision.is_some() {
            return Err(corrupt());
        }
        revisions.insert((id.to_owned(), meta.revision), sequence);
        sequences.insert((sequence, id.to_owned()), meta.revision);
        latest.insert(id.to_owned(), record.clone());
        history_records.insert((id.to_owned(), sequence), record);
    }
    if latest != current {
        return Err(corrupt());
    }
    let actual_revisions: BTreeMap<_, _> = tx
        .open_table(REVISIONS)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
        .map(|entry| {
            let (key, value) = entry.map_err(storage_error)?;
            let (id, revision) = key.value();
            Ok(((id.to_owned(), revision), value.value()))
        })
        .collect::<Result<_>>()?;
    let actual_sequences: BTreeMap<_, _> = tx
        .open_table(SEQUENCES)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
        .map(|entry| {
            let (key, value) = entry.map_err(storage_error)?;
            let (sequence, id) = key.value();
            Ok(((sequence, id.to_owned()), value.value()))
        })
        .collect::<Result<_>>()?;
    let actual_times: BTreeMap<_, _> = tx
        .open_table(TIMES)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
        .map(|entry| {
            let (key, value) = entry.map_err(storage_error)?;
            Ok((key.value(), value.value()))
        })
        .collect::<Result<_>>()?;
    if revisions != actual_revisions || sequences != actual_sequences || times != actual_times {
        return Err(corrupt());
    }
    let metadata = tx.open_table(META).map_err(storage_error)?;
    let state = metadata
        .get("state")
        .map_err(storage_error)?
        .ok_or_else(corrupt)?;
    let state: Value = serde_json::from_slice(state.value()).map_err(storage_error)?;
    let sequence = state["sequence"].as_u64().ok_or_else(corrupt)?;
    if sequence_times.len() as u64 != sequence
        || sequence_times.keys().copied().ne(1..=sequence)
        || sequence_times
            .values()
            .zip(sequence_times.values().skip(1))
            .any(|(a, b)| a >= b)
    {
        return Err(corrupt());
    }
    match (state["last_time"].as_str(), sequence_times.last_key_value()) {
        (None, None) => {}
        (Some(time), Some((_, last))) if history::time_key(time)? == *last => {}
        _ => return Err(corrupt()),
    }
    let mut receipt_sequences = BTreeSet::new();
    let mut receipt_replacements = BTreeMap::new();
    for entry in tx
        .open_table(RECEIPTS)
        .map_err(storage_error)?
        .iter()
        .map_err(storage_error)?
    {
        let (key, bytes) = entry.map_err(storage_error)?;
        let operation: Value = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        let request: BatchRequest =
            serde_json::from_value(operation["request"].clone()).map_err(storage_error)?;
        request.validate()?;
        let receipt: OperationReceipt =
            serde_json::from_value(operation["receipt"].clone()).map_err(storage_error)?;
        receipt.validate()?;
        let records: Vec<GardenRecord> =
            serde_json::from_value(operation["records"].clone()).map_err(storage_error)?;
        let seq = receipt.commit_sequence.ok_or_else(corrupt)?;
        let committed_at = receipt.committed_at.as_deref().ok_or_else(corrupt)?;
        let canonical_request =
            serde_json::to_string(&serde_json::to_value(&request).map_err(storage_error)?)
                .map_err(storage_error)?;
        let expected_key = format!(
            "{}/{}/{}",
            receipt.operation_generation, receipt.actor, receipt.operation_id
        );
        if key.value() != expected_key
            || operation["payload_hash"].as_str()
                != Some(crate::contracts::hash(&canonical_request).as_str())
            || request.operation_id != receipt.operation_id
            || request.generation != receipt.operation_generation
            || request.workspace_id != state["workspace_id"]
            || !admitted_generations.contains(&request.generation)
        {
            return Err(corrupt());
        }
        if sequence_times.get(&seq) != Some(&history::time_key(committed_at)?)
            || !receipt_sequences.insert(seq)
            || receipt.results.len() != records.len()
        {
            return Err(corrupt());
        }
        let mut result_ids = BTreeSet::new();
        for result in &receipt.results {
            if !result_ids.insert(result.reference.id.clone()) {
                return Err(corrupt());
            }
            let history = history_records
                .get(&(result.reference.id.clone(), seq))
                .ok_or_else(corrupt)?;
            if result.revision != history.metadata.revision
                || result.reference.kind != history.draft.kind()
                || history.metadata.actor != receipt.actor
                || !records.contains(history)
            {
                return Err(corrupt());
            }
        }
        let historical_ids: BTreeSet<_> = sequences
            .keys()
            .filter(|(sequence, _)| *sequence == seq)
            .map(|(_, id)| id.clone())
            .collect();
        if result_ids != historical_ids {
            return Err(corrupt());
        }
        for item in request
            .items
            .iter()
            .filter(|item| item.mutation == Mutation::Supersede)
        {
            let source = item.id.as_ref().ok_or_else(corrupt)?;
            let target = match item.replacement.as_ref().ok_or_else(corrupt)? {
                RecordRef::Existing { kind, id } => ExternalReference {
                    kind: *kind,
                    id: id.clone(),
                },
                RecordRef::Item { kind, item_key } => receipt
                    .results
                    .iter()
                    .find(|result| result.item_key == *item_key && result.reference.kind == *kind)
                    .map(|result| result.reference.clone())
                    .ok_or_else(corrupt)?,
            };
            if source.kind != target.kind
                || receipt_replacements
                    .insert(source.id.clone(), target.id)
                    .is_some()
            {
                return Err(corrupt());
            }
        }
    }
    if receipt_sequences.iter().copied().ne(1..=sequence) {
        return Err(corrupt());
    }
    if receipt_replacements != replacements {
        return Err(corrupt());
    }
    audit_query(tx, &current, &history_records)?;
    Ok(StoreStats {
        current_records: current.len() as u64,
        history_records: history_records.len() as u64,
        receipts: receipt_sequences.len() as u64,
        sequence,
    })
}

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
type TimeMembership = ((i64, u32), String, u64);
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
fn audit_query(
    tx: &ReadTransaction,
    current: &BTreeMap<String, GardenRecord>,
    history: &BTreeMap<(String, u64), GardenRecord>,
) -> Result<()> {
    if tx
        .open_table(META)
        .map_err(storage_error)?
        .get("query_indexes_v1")
        .map_err(storage_error)?
        .is_none_or(|value| value.value() != [1_u8])
    {
        return Err(corrupt());
    }
    let mut expected_current = BTreeSet::new();
    let mut expected_history = BTreeSet::new();
    let mut starts: BTreeSet<TimeMembership> = BTreeSet::new();
    let mut ends: BTreeSet<TimeMembership> = BTreeSet::new();
    let mut current_starts = BTreeSet::new();
    let mut current_ends = BTreeSet::new();
    for record in current.values() {
        for (tag, term) in query_memberships(record)? {
            expected_current.insert((tag, term, record.metadata.id.clone(), 0));
        }
        if let Some((from, until)) = valid_bounds(record)? {
            current_starts.insert((from, record.metadata.id.clone(), 0));
            current_ends.insert((until, record.metadata.id.clone(), 0));
        }
    }
    for record in history.values() {
        for (tag, term) in query_memberships(record)? {
            expected_history.insert((
                tag,
                term,
                record.metadata.id.clone(),
                record.metadata.commit_sequence,
            ));
        }
        if let Some((from, until)) = valid_bounds(record)? {
            starts.insert((
                from,
                record.metadata.id.clone(),
                record.metadata.commit_sequence,
            ));
            ends.insert((
                until,
                record.metadata.id.clone(),
                record.metadata.commit_sequence,
            ));
        }
    }
    for (definition, expected) in [
        (QUERY_CURRENT, expected_current),
        (QUERY_HISTORY, expected_history),
    ] {
        let mut actual = BTreeSet::new();
        for entry in tx
            .open_table(definition)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (key, value) = entry.map_err(storage_error)?;
            let (tag, term, id, seq) = key.value();
            if value.value() != 1 {
                return Err(corrupt());
            }
            actual.insert((tag, term.to_owned(), id.to_owned(), seq));
        }
        if actual != expected {
            return Err(corrupt());
        }
    }
    for (definition, expected) in [
        (VALID_START, starts),
        (VALID_END, ends),
        (CURRENT_VALID_START, current_starts),
        (CURRENT_VALID_END, current_ends),
    ] {
        let mut actual = BTreeSet::new();
        for entry in tx
            .open_table(definition)
            .map_err(storage_error)?
            .iter()
            .map_err(storage_error)?
        {
            let (key, value) = entry.map_err(storage_error)?;
            let (at, id, seq) = key.value();
            if value.value() != 1 {
                return Err(corrupt());
            }
            actual.insert((at, id.to_owned(), seq));
        }
        if actual != expected {
            return Err(corrupt());
        }
    }
    Ok(())
}
