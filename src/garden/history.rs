//! Private immutable revisions selected by store-owned transaction time.
use super::{
    contracts::{GardenDraft, GardenErrorCode, GardenRecord},
    store::{Result, failure, storage_error},
};
use chrono::{DateTime, Utc};
use redb::{ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

pub(crate) const HISTORY: TableDefinition<(&str, u64), &[u8]> =
    TableDefinition::new("garden_history");
pub(crate) const REVISIONS: TableDefinition<(&str, u64), u64> =
    TableDefinition::new("garden_revisions");
pub(crate) const TIMES: TableDefinition<(i64, u32), u64> =
    TableDefinition::new("garden_transaction_times");

pub(crate) fn initialize(tx: &WriteTransaction) -> Result<()> {
    tx.open_table(HISTORY).map_err(storage_error)?;
    tx.open_table(REVISIONS).map_err(storage_error)?;
    tx.open_table(TIMES).map_err(storage_error)?;
    Ok(())
}

pub(crate) fn instant(value: &str) -> Result<DateTime<Utc>> {
    if value.len() > 40 || !value.ends_with('Z') {
        return Err(failure(
            GardenErrorCode::InvalidInput,
            "Invalid UTC instant",
        ));
    }
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_| failure(GardenErrorCode::InvalidInput, "Invalid UTC instant"))
}

pub(crate) fn time_key(value: &str) -> Result<(i64, u32)> {
    let value = instant(value)?;
    Ok((value.timestamp(), value.timestamp_subsec_nanos()))
}

pub(crate) fn append(tx: &WriteTransaction, record: &GardenRecord) -> Result<()> {
    let metadata = &record.metadata;
    let bytes = serde_json::to_vec(record).map_err(storage_error)?;
    let mut history = tx.open_table(HISTORY).map_err(storage_error)?;
    let mut revisions = tx.open_table(REVISIONS).map_err(storage_error)?;
    let mut times = tx.open_table(TIMES).map_err(storage_error)?;
    let history_key = (metadata.id.as_str(), metadata.commit_sequence);
    let revision_key = (metadata.id.as_str(), metadata.revision);
    if history.get(history_key).map_err(storage_error)?.is_some()
        || revisions
            .get(revision_key)
            .map_err(storage_error)?
            .is_some()
    {
        return Err(failure(
            GardenErrorCode::Conflict,
            "Revision already exists",
        ));
    }
    let time = time_key(&metadata.transaction_time)?;
    if let Some(existing) = times.get(time).map_err(storage_error)?
        && existing.value() != metadata.commit_sequence
    {
        return Err(failure(
            GardenErrorCode::Conflict,
            "Transaction time already exists",
        ));
    }
    history
        .insert(history_key, bytes.as_slice())
        .map_err(storage_error)?;
    revisions
        .insert(revision_key, metadata.commit_sequence)
        .map_err(storage_error)?;
    times
        .insert(time, metadata.commit_sequence)
        .map_err(storage_error)?;
    Ok(())
}

pub(crate) fn get(
    tx: &ReadTransaction,
    id: &str,
    revision: Option<u64>,
    known_at: Option<&str>,
) -> Result<Option<GardenRecord>> {
    let bound = if let Some(known_at) = known_at {
        let times = tx.open_table(TIMES).map_err(storage_error)?;
        let mut range = times.range(..=time_key(known_at)?).map_err(storage_error)?;
        match range.next_back().transpose().map_err(storage_error)? {
            Some((_, sequence)) => sequence.value(),
            None => return Ok(None),
        }
    } else {
        u64::MAX
    };
    let history = tx.open_table(HISTORY).map_err(storage_error)?;
    let bytes = if let Some(revision) = revision {
        let revisions = tx.open_table(REVISIONS).map_err(storage_error)?;
        let Some(sequence) = revisions.get((id, revision)).map_err(storage_error)? else {
            return Ok(None);
        };
        if sequence.value() > bound {
            return Ok(None);
        }
        history.get((id, sequence.value())).map_err(storage_error)?
    } else {
        let mut range = history
            .range((id, 0)..=(id, bound))
            .map_err(storage_error)?;
        range
            .next_back()
            .transpose()
            .map_err(storage_error)?
            .map(|(_, value)| value)
    };
    bytes
        .map(|bytes| serde_json::from_slice(bytes.value()).map_err(storage_error))
        .transpose()
}

/// Resolve a known transaction instant without enumerating canonical records.
pub(crate) fn sequence_at(tx: &ReadTransaction, known_at: &str) -> Result<u64> {
    let table = tx.open_table(TIMES).map_err(storage_error)?;
    Ok(table
        .range(..=time_key(known_at)?)
        .map_err(storage_error)?
        .next_back()
        .transpose()
        .map_err(storage_error)?
        .map_or(0, |(_, sequence)| sequence.value()))
}

/// Read the final revision visible at a fixed committed sequence.
pub(crate) fn at_sequence(
    tx: &ReadTransaction,
    id: &str,
    sequence: u64,
) -> Result<Option<GardenRecord>> {
    let table = tx.open_table(HISTORY).map_err(storage_error)?;
    table
        .range((id, 0)..=(id, sequence))
        .map_err(storage_error)?
        .next_back()
        .transpose()
        .map_err(storage_error)?
        .map(|(_, bytes)| serde_json::from_slice(bytes.value()).map_err(storage_error))
        .transpose()
}

pub(crate) fn valid_at(record: &GardenRecord, at: &str) -> Result<bool> {
    let at = instant(at)?;
    let GardenDraft::Assertion { valid_time, .. } = &record.draft else {
        return Ok(false);
    };
    let from = valid_time.valid_from.as_deref().map(instant).transpose()?;
    let until = valid_time.valid_until.as_deref().map(instant).transpose()?;
    Ok(from.is_none_or(|from| from <= at) && until.is_none_or(|until| at < until))
}
