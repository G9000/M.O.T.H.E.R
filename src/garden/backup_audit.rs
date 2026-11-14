//! Bounded-memory canonical and derived-index validation for full snapshots.
use super::{
    CURRENT, RECEIPTS, REPLACEMENT_INCOMING, REPLACEMENTS, StoredOperation, decode, indexes,
    state_read, storage_error,
};
use crate::garden::{
    contracts::{ExternalReference, GardenErrorCode, GardenRecord, Lifecycle, Mutation, RecordRef},
    history::{self, HISTORY, REVISIONS, TIMES},
    indexes::StoreStats,
};
use redb::{Database, ReadTransaction, ReadableTable, ReadableTableMetadata, TableDefinition};

type Result<T> = super::Result<T>;
const MEMBERSHIPS: [TableDefinition<&str, u8>; 8] = [
    TableDefinition::new("garden_ids"),
    TableDefinition::new("garden_outgoing"),
    TableDefinition::new("garden_incoming"),
    TableDefinition::new("garden_predicate"),
    TableDefinition::new("garden_valid_time"),
    TableDefinition::new("garden_evidence"),
    TableDefinition::new("garden_sources"),
    TableDefinition::new("garden_namespaces"),
];
const SEQUENCES: TableDefinition<(u64, &str), u64> =
    TableDefinition::new("garden_transaction_sequence");
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
// These indexes contain only validation keys. Canonical payloads never enter the
// scratch database, whose owned temporary directory is discarded after auditing.
const SEEN_RECEIPTS: TableDefinition<u64, u8> = TableDefinition::new("receipt_sequences");
const SEEN_REPLACEMENTS: TableDefinition<&str, &str> = TableDefinition::new("receipt_replacements");

fn corrupt() -> super::GardenError {
    super::failure(
        GardenErrorCode::StoreUnavailable,
        "Full snapshot consistency check failed",
    )
}
fn require(condition: bool) -> Result<()> {
    if condition { Ok(()) } else { Err(corrupt()) }
}
fn current(tx: &ReadTransaction, id: &str) -> Result<GardenRecord> {
    decode(
        tx.open_table(CURRENT)
            .map_err(storage_error)?
            .get(id)
            .map_err(storage_error)?
            .ok_or_else(corrupt)?
            .value(),
    )
}
fn historical(tx: &ReadTransaction, id: &str, sequence: u64) -> Result<GardenRecord> {
    decode(
        tx.open_table(HISTORY)
            .map_err(storage_error)?
            .get((id, sequence))
            .map_err(storage_error)?
            .ok_or_else(corrupt)?
            .value(),
    )
}
fn query_keys(tx: &ReadTransaction, record: &GardenRecord, history: bool) -> Result<(u64, u64)> {
    let definition = if history {
        QUERY_HISTORY
    } else {
        QUERY_CURRENT
    };
    let sequence = if history {
        record.metadata.commit_sequence
    } else {
        0
    };
    let table = tx.open_table(definition).map_err(storage_error)?;
    let terms = indexes::query_memberships(record)?;
    for (tag, term) in &terms {
        require(
            table
                .get((*tag, term.as_str(), record.metadata.id.as_str(), sequence))
                .map_err(storage_error)?
                .is_some_and(|v| v.value() == 1),
        )?;
    }
    let mut times = 0;
    if let Some((from, until)) = indexes::valid_bounds(record)? {
        let (start, end) = if history {
            (VALID_START, VALID_END)
        } else {
            (CURRENT_VALID_START, CURRENT_VALID_END)
        };
        for (definition, bound) in [(start, from), (end, until)] {
            require(
                tx.open_table(definition)
                    .map_err(storage_error)?
                    .get((bound, record.metadata.id.as_str(), sequence))
                    .map_err(storage_error)?
                    .is_some_and(|v| v.value() == 1),
            )?;
        }
        times = 1;
    }
    Ok((terms.len() as u64, times))
}
fn count_query(tx: &ReadTransaction, historical: bool, terms: u64, times: u64) -> Result<()> {
    require(
        tx.open_table(if historical {
            QUERY_HISTORY
        } else {
            QUERY_CURRENT
        })
        .map_err(storage_error)?
        .len()
        .map_err(storage_error)?
            == terms,
    )?;
    let definitions = if historical {
        [VALID_START, VALID_END]
    } else {
        [CURRENT_VALID_START, CURRENT_VALID_END]
    };
    for definition in definitions {
        require(
            tx.open_table(definition)
                .map_err(storage_error)?
                .len()
                .map_err(storage_error)?
                == times,
        )?;
    }
    Ok(())
}

/// Validate a snapshot with at most one record and one bounded write batch in RAM.
/// Receipt uniqueness and supersession provenance use temporary on-disk indexes.
pub(crate) fn audit_read(tx: &ReadTransaction) -> Result<StoreStats> {
    require(
        tx.open_table(super::META)
            .map_err(storage_error)?
            .get("query_indexes_v1")
            .map_err(storage_error)?
            .is_some_and(|v| v.value() == [1]),
    )?;
    let state = state_read(tx)?;
    let generations = indexes::admitted_generations(tx)?;
    let canonical = tx.open_table(CURRENT).map_err(storage_error)?;
    let mut member_counts = [0u64; 8];
    let (mut current_terms, mut current_times) = (0, 0);
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    let incoming = tx.open_table(REPLACEMENT_INCOMING).map_err(storage_error)?;
    for entry in canonical.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let record: GardenRecord = decode(bytes.value())?;
        record.validate()?;
        require(key.value() == record.metadata.id)?;
        for (index, keys) in indexes::memberships(&record)?.into_iter().enumerate() {
            let table = tx.open_table(MEMBERSHIPS[index]).map_err(storage_error)?;
            for key in &keys {
                require(
                    table
                        .get(key.as_str())
                        .map_err(storage_error)?
                        .is_some_and(|v| v.value() == 1),
                )?;
            }
            member_counts[index] += keys.len() as u64;
        }
        let (terms, times) = query_keys(tx, &record, false)?;
        current_terms += terms;
        current_times += times;
        if record.metadata.lifecycle == Lifecycle::Active {
            for reference in record.draft.references() {
                let RecordRef::Existing { kind, id } = reference else {
                    return Err(corrupt());
                };
                let endpoint = current(tx, id)?;
                require(
                    endpoint.draft.kind() == *kind
                        && endpoint.metadata.lifecycle == Lifecycle::Active,
                )?;
            }
        }
        require(
            (record.metadata.lifecycle == Lifecycle::Superseded)
                == replacements
                    .get(key.value())
                    .map_err(storage_error)?
                    .is_some(),
        )?;
    }
    for (definition, count) in MEMBERSHIPS.into_iter().zip(member_counts) {
        require(
            tx.open_table(definition)
                .map_err(storage_error)?
                .len()
                .map_err(storage_error)?
                == count,
        )?;
    }
    count_query(tx, false, current_terms, current_times)?;
    require(incoming.len().map_err(storage_error)? == replacements.len().map_err(storage_error)?)?;
    for entry in replacements.iter().map_err(storage_error)? {
        let (source, target) = entry.map_err(storage_error)?;
        let source_record = current(tx, source.value())?;
        let target_record = current(tx, target.value())?;
        require(
            source_record.metadata.lifecycle == Lifecycle::Superseded
                && source_record.draft.kind() == target_record.draft.kind()
                && target_record.metadata.lifecycle != Lifecycle::Retracted,
        )?;
        require(
            incoming
                .get((target.value(), source.value()))
                .map_err(storage_error)?
                .is_some_and(|v| v.value() == 1),
        )?;
        let mut cursor = source.value().to_owned();
        let mut steps = 0u64;
        loop {
            let next = replacements
                .get(cursor.as_str())
                .map_err(storage_error)?
                .map(|v| v.value().to_owned());
            let Some(next) = next else {
                break;
            };
            cursor = next;
            steps += 1;
            require(steps <= canonical.len().map_err(storage_error)?)?;
        }
        require(current(tx, &cursor)?.metadata.lifecycle == Lifecycle::Active)?;
    }
    let history = tx.open_table(HISTORY).map_err(storage_error)?;
    let revisions = tx.open_table(REVISIONS).map_err(storage_error)?;
    let sequences = tx.open_table(SEQUENCES).map_err(storage_error)?;
    let times = tx.open_table(TIMES).map_err(storage_error)?;
    let mut previous: Option<GardenRecord> = None;
    let (mut heads, mut history_terms, mut history_times) = (0u64, 0u64, 0u64);
    for entry in history.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let (id, sequence) = key.value();
        let record: GardenRecord = decode(bytes.value())?;
        record.validate()?;
        let meta = &record.metadata;
        require(
            id == meta.id
                && sequence == meta.commit_sequence
                && sequence > 0
                && sequence <= state.sequence,
        )?;
        let time = history::time_key(&meta.transaction_time)?;
        require(
            times
                .get(time)
                .map_err(storage_error)?
                .is_some_and(|v| v.value() == sequence),
        )?;
        if let Some(old) = previous.as_ref().filter(|old| old.metadata.id == id) {
            require(
                meta.revision == old.metadata.revision + 1
                    && meta.previous_revision == Some(old.metadata.revision)
                    && meta.contributor == old.metadata.contributor,
            )?;
        } else {
            if let Some(old) = &previous {
                require(current(tx, &old.metadata.id)? == *old)?;
                heads += 1;
            }
            require(meta.revision == 1 && meta.previous_revision.is_none())?;
        }
        require(
            revisions
                .get((id, meta.revision))
                .map_err(storage_error)?
                .is_some_and(|v| v.value() == sequence),
        )?;
        require(
            sequences
                .get((sequence, id))
                .map_err(storage_error)?
                .is_some_and(|v| v.value() == meta.revision),
        )?;
        let (terms, bounds) = query_keys(tx, &record, true)?;
        history_terms += terms;
        history_times += bounds;
        previous = Some(record);
    }
    if let Some(old) = previous {
        require(current(tx, &old.metadata.id)? == old)?;
        heads += 1;
    }
    require(heads == canonical.len().map_err(storage_error)?)?;
    require(
        history.len().map_err(storage_error)? == revisions.len().map_err(storage_error)?
            && history.len().map_err(storage_error)? == sequences.len().map_err(storage_error)?,
    )?;
    count_query(tx, true, history_terms, history_times)?;
    require(times.len().map_err(storage_error)? == state.sequence)?;
    let mut last_time = None;
    for (offset, entry) in times.iter().map_err(storage_error)?.enumerate() {
        let (time, sequence) = entry.map_err(storage_error)?;
        require(sequence.value() == offset as u64 + 1)?;
        last_time = Some(time.value());
    }
    require(
        state
            .last_time
            .as_deref()
            .map(history::time_key)
            .transpose()?
            == last_time,
    )?;
    let directory = tempfile::tempdir().map_err(storage_error)?;
    let scratch =
        Database::create(directory.path().join("validation.redb")).map_err(storage_error)?;
    let scratch_tx = scratch.begin_write().map_err(storage_error)?;
    let mut seen = scratch_tx
        .open_table(SEEN_RECEIPTS)
        .map_err(storage_error)?;
    let mut replacement_provenance = scratch_tx
        .open_table(SEEN_REPLACEMENTS)
        .map_err(storage_error)?;
    let receipts = tx.open_table(RECEIPTS).map_err(storage_error)?;
    for entry in receipts.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let operation: StoredOperation = decode(bytes.value())?;
        let request = &operation.request;
        let receipt = &operation.receipt;
        request.validate()?;
        receipt.validate()?;
        let seq = receipt.commit_sequence.ok_or_else(corrupt)?;
        let stamp = receipt.committed_at.as_deref().ok_or_else(corrupt)?;
        let canonical_request =
            serde_json::to_string(&serde_json::to_value(request).map_err(storage_error)?)
                .map_err(storage_error)?;
        require(
            key.value()
                == super::operation_key(
                    &receipt.operation_generation,
                    &receipt.actor,
                    &receipt.operation_id,
                )
                && operation.payload_hash == crate::contracts::hash(&canonical_request)
                && request.operation_id == receipt.operation_id
                && request.generation == receipt.operation_generation
                && request.workspace_id == state.workspace_id
                && generations.contains(&request.generation),
        )?;
        require(
            times
                .get(history::time_key(stamp)?)
                .map_err(storage_error)?
                .is_some_and(|v| v.value() == seq),
        )?;
        require(seen.insert(seq, 1).map_err(storage_error)?.is_none())?;
        require(receipt.results.len() == operation.records.len())?;
        let mut ids = std::collections::BTreeSet::new();
        for result in &receipt.results {
            require(ids.insert(result.reference.id.as_str()))?;
            let record = historical(tx, &result.reference.id, seq)?;
            require(
                result.revision == record.metadata.revision
                    && result.reference.kind == record.draft.kind()
                    && record.metadata.actor == receipt.actor
                    && operation.records.contains(&record),
            )?;
        }
        let mut count = 0usize;
        for row in sequences
            .range((seq, "")..=(seq, "\u{10ffff}"))
            .map_err(storage_error)?
        {
            let (key, _) = row.map_err(storage_error)?;
            require(ids.contains(key.value().1))?;
            count += 1;
        }
        require(count == ids.len())?;
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
                    .find(|r| r.item_key == *item_key && r.reference.kind == *kind)
                    .map(|r| r.reference.clone())
                    .ok_or_else(corrupt)?,
            };
            require(
                source.kind == target.kind
                    && replacement_provenance
                        .insert(source.id.as_str(), target.id.as_str())
                        .map_err(storage_error)?
                        .is_none(),
            )?;
            require(
                replacements
                    .get(source.id.as_str())
                    .map_err(storage_error)?
                    .is_some_and(|v| v.value() == target.id),
            )?;
        }
    }
    require(
        seen.len().map_err(storage_error)? == state.sequence
            && replacement_provenance.len().map_err(storage_error)?
                == replacements.len().map_err(storage_error)?,
    )?;
    Ok(StoreStats {
        current_records: canonical.len().map_err(storage_error)?,
        history_records: history.len().map_err(storage_error)?,
        receipts: receipts.len().map_err(storage_error)?,
        sequence: state.sequence,
    })
}
