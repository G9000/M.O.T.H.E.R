//! Exact snapshot auditing with one canonical row and one receipt batch in memory.
use super::{
    CURRENT_VALID_END, CURRENT_VALID_START, QUERY_CURRENT, QUERY_HISTORY, SEQUENCES, StoreStats,
    TABLES, VALID_END, VALID_START, admitted_generations, corrupt, memberships, query_memberships,
    valid_bounds,
};
use crate::garden::{
    contracts::{
        BatchRequest, ExternalReference, GardenRecord, Lifecycle, Mutation, OperationReceipt,
        RecordRef,
    },
    history::{self, HISTORY, REVISIONS, TIMES},
    store::{CURRENT, META, RECEIPTS, REPLACEMENT_INCOMING, REPLACEMENTS, Result, storage_error},
};
use redb::{ReadOnlyTable, ReadTransaction, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeSet;

type QueryTable = ReadOnlyTable<(u8, &'static str, &'static str, u64), u8>;
type ValidTable = ReadOnlyTable<((i64, u32), &'static str, u64), u8>;
type RecordTable = ReadOnlyTable<&'static str, &'static [u8]>;

struct QueryAudit {
    query: QueryTable,
    start: ValidTable,
    end: ValidTable,
    postings: u64,
    bounds: u64,
}

impl QueryAudit {
    fn open(
        tx: &ReadTransaction,
        query: TableDefinition<(u8, &str, &str, u64), u8>,
        start: TableDefinition<((i64, u32), &str, u64), u8>,
        end: TableDefinition<((i64, u32), &str, u64), u8>,
    ) -> Result<Self> {
        Ok(Self {
            query: tx.open_table(query).map_err(storage_error)?,
            start: tx.open_table(start).map_err(storage_error)?,
            end: tx.open_table(end).map_err(storage_error)?,
            postings: 0,
            bounds: 0,
        })
    }

    fn record(&mut self, record: &GardenRecord, sequence: u64) -> Result<()> {
        let id = record.metadata.id.as_str();
        for (tag, term) in query_memberships(record)? {
            if self
                .query
                .get((tag, term.as_str(), id, sequence))
                .map_err(storage_error)?
                .is_none_or(|value| value.value() != 1)
            {
                return Err(corrupt());
            }
            self.postings += 1;
        }
        if let Some((from, until)) = valid_bounds(record)? {
            for (table, bound) in [(&self.start, from), (&self.end, until)] {
                if table
                    .get((bound, id, sequence))
                    .map_err(storage_error)?
                    .is_none_or(|value| value.value() != 1)
                {
                    return Err(corrupt());
                }
            }
            self.bounds += 1;
        }
        Ok(())
    }

    fn finish(self) -> Result<()> {
        if self.query.len().map_err(storage_error)? != self.postings
            || self.start.len().map_err(storage_error)? != self.bounds
            || self.end.len().map_err(storage_error)? != self.bounds
        {
            return Err(corrupt());
        }
        Ok(())
    }
}

fn current_record(table: &RecordTable, id: &str) -> Result<GardenRecord> {
    let bytes = table.get(id).map_err(storage_error)?.ok_or_else(corrupt)?;
    serde_json::from_slice(bytes.value()).map_err(storage_error)
}

fn check_latest(canonical: &RecordTable, record: &GardenRecord) -> Result<()> {
    if current_record(canonical, &record.metadata.id)? != *record {
        return Err(corrupt());
    }
    Ok(())
}

/// Verify exact canonical/index equality in one consistent read transaction.
pub(crate) fn audit_read(tx: &ReadTransaction) -> Result<StoreStats> {
    let generations = admitted_generations(tx)?;
    let metadata = tx.open_table(META).map_err(storage_error)?;
    let state = metadata
        .get("state")
        .map_err(storage_error)?
        .ok_or_else(corrupt)?;
    let state: Value = serde_json::from_slice(state.value()).map_err(storage_error)?;
    let sequence = state["sequence"].as_u64().ok_or_else(corrupt)?;
    if metadata
        .get("query_indexes_v1")
        .map_err(storage_error)?
        .is_none_or(|value| value.value() != [1_u8])
    {
        return Err(corrupt());
    }
    let canonical = tx.open_table(CURRENT).map_err(storage_error)?;
    let history = tx.open_table(HISTORY).map_err(storage_error)?;
    let revisions = tx.open_table(REVISIONS).map_err(storage_error)?;
    let sequences = tx.open_table(SEQUENCES).map_err(storage_error)?;
    let times = tx.open_table(TIMES).map_err(storage_error)?;
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    let incoming = tx.open_table(REPLACEMENT_INCOMING).map_err(storage_error)?;
    let legacy = TABLES
        .into_iter()
        .map(|definition| tx.open_table(definition).map_err(storage_error))
        .collect::<Result<Vec<_>>>()?;
    let mut legacy_counts = [0_u64; 8];
    let mut current_query =
        QueryAudit::open(tx, QUERY_CURRENT, CURRENT_VALID_START, CURRENT_VALID_END)?;
    let mut history_query = QueryAudit::open(tx, QUERY_HISTORY, VALID_START, VALID_END)?;
    let mut superseded = 0_u64;
    for entry in canonical.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let record: GardenRecord = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        record.validate()?;
        if key.value() != record.metadata.id {
            return Err(corrupt());
        }
        for (index, keys) in memberships(&record)?.into_iter().enumerate() {
            for key in keys {
                if legacy[index]
                    .get(key.as_str())
                    .map_err(storage_error)?
                    .is_none_or(|value| value.value() != 1)
                {
                    return Err(corrupt());
                }
                legacy_counts[index] += 1;
            }
        }
        current_query.record(&record, 0)?;
        if record.metadata.lifecycle == Lifecycle::Active {
            for reference in record.draft.references() {
                let RecordRef::Existing { kind, id } = reference else {
                    return Err(corrupt());
                };
                let endpoint = current_record(&canonical, id)?;
                if endpoint.draft.kind() != *kind
                    || endpoint.metadata.lifecycle != Lifecycle::Active
                {
                    return Err(corrupt());
                }
            }
        }
        let replacement = replacements.get(key.value()).map_err(storage_error)?;
        if (record.metadata.lifecycle == Lifecycle::Superseded) != replacement.is_some() {
            return Err(corrupt());
        }
        superseded += u64::from(replacement.is_some());
    }
    // Every legacy/query/time key includes its canonical ID (and historical
    // sequence). Per-record sets are unique and rows are disjoint across records.
    // Exact point membership plus equal cardinality therefore rejects every
    // extra row as well as every missing or wrong-valued row, without scratch sets.
    for (table, expected) in legacy.iter().zip(legacy_counts) {
        if table.len().map_err(storage_error)? != expected {
            return Err(corrupt());
        }
    }
    current_query.finish()?;
    let replacement_count = replacements.len().map_err(storage_error)?;
    if replacement_count != superseded
        || incoming.len().map_err(storage_error)? != replacement_count
    {
        return Err(corrupt());
    }
    for entry in replacements.iter().map_err(storage_error)? {
        let (source, target) = entry.map_err(storage_error)?;
        let source_record = current_record(&canonical, source.value())?;
        let target_record = current_record(&canonical, target.value())?;
        if source_record.metadata.lifecycle != Lifecycle::Superseded
            || source_record.draft.kind() != target_record.draft.kind()
            || target_record.metadata.lifecycle == Lifecycle::Retracted
            || incoming
                .get((target.value(), source.value()))
                .map_err(storage_error)?
                .is_none_or(|value| value.value() != 1)
        {
            return Err(corrupt());
        }
        let mut cursor = target.value().to_owned();
        let mut traversed = 1_u64;
        while let Some(next) = replacements.get(cursor.as_str()).map_err(storage_error)? {
            if traversed >= replacement_count {
                return Err(corrupt());
            }
            cursor = next.value().to_owned();
            traversed += 1;
        }
        if current_record(&canonical, &cursor)?.metadata.lifecycle != Lifecycle::Active {
            return Err(corrupt());
        }
    }
    let mut previous: Option<GardenRecord> = None;
    let mut history_count = 0_u64;
    let mut history_ids = 0_u64;
    for entry in history.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let (id, seq) = key.value();
        let record: GardenRecord = serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        record.validate()?;
        let meta = &record.metadata;
        if id != meta.id
            || seq != meta.commit_sequence
            || seq == 0
            || revisions
                .get((id, meta.revision))
                .map_err(storage_error)?
                .is_none_or(|value| value.value() != seq)
            || sequences
                .get((seq, id))
                .map_err(storage_error)?
                .is_none_or(|value| value.value() != meta.revision)
            || times
                .get(history::time_key(&meta.transaction_time)?)
                .map_err(storage_error)?
                .is_none_or(|value| value.value() != seq)
        {
            return Err(corrupt());
        }
        if let Some(old) = previous.as_ref().filter(|old| old.metadata.id == id) {
            if meta.revision != old.metadata.revision + 1
                || meta.previous_revision != Some(old.metadata.revision)
                || meta.contributor != old.metadata.contributor
            {
                return Err(corrupt());
            }
        } else {
            if let Some(old) = &previous {
                check_latest(&canonical, old)?;
            }
            if meta.revision != 1 || meta.previous_revision.is_some() {
                return Err(corrupt());
            }
            history_ids += 1;
        }
        history_query.record(&record, seq)?;
        previous = Some(record);
        history_count += 1;
    }
    if let Some(old) = previous {
        check_latest(&canonical, &old)?;
    }
    if history_ids != canonical.len().map_err(storage_error)?
        || revisions.len().map_err(storage_error)? != history_count
        || sequences.len().map_err(storage_error)? != history_count
        || times.len().map_err(storage_error)? != sequence
    {
        return Err(corrupt());
    }
    history_query.finish()?;
    let mut last_sequence = 0_u64;
    for entry in sequences.iter().map_err(storage_error)? {
        let (key, _) = entry.map_err(storage_error)?;
        let seq = key.value().0;
        if seq != last_sequence {
            if last_sequence.checked_add(1) != Some(seq) {
                return Err(corrupt());
            }
            last_sequence = seq;
        }
    }
    if last_sequence != sequence {
        return Err(corrupt());
    }
    let mut last_time = None;
    for (index, entry) in times.iter().map_err(storage_error)?.enumerate() {
        let (time, seq) = entry.map_err(storage_error)?;
        if seq.value() != index as u64 + 1 {
            return Err(corrupt());
        }
        last_time = Some(time.value());
    }
    match (state["last_time"].as_str(), last_time) {
        (None, None) => {}
        (Some(time), Some(last)) if history::time_key(time)? == last => {}
        _ => return Err(corrupt()),
    }
    audit_receipts(tx, &state, &generations, sequence, replacement_count)?;
    Ok(StoreStats {
        current_records: canonical.len().map_err(storage_error)?,
        history_records: history_count,
        receipts: sequence,
        sequence,
    })
}

#[derive(Deserialize)]
struct AuditOperation {
    payload_hash: String,
    request: BatchRequest,
    receipt: OperationReceipt,
    records: Vec<GardenRecord>,
}

// A fixed 8 KiB bitmap proves receipt-sequence uniqueness exactly. Additional
// 65,536-sequence windows rescan only receipt sequence projections, never retain
// the complete receipt/sequence set. Cost is O(R * ceil(sequence / 65,536)).
const RECEIPT_WINDOW: u64 = 65_536;
const RECEIPT_WORDS: usize = RECEIPT_WINDOW as usize / 64;

fn mark_receipt(bits: &mut [u64; RECEIPT_WORDS], offset: u64) -> Result<()> {
    let word = offset as usize / 64;
    let mask = 1_u64 << (offset % 64);
    if bits[word] & mask != 0 {
        return Err(corrupt());
    }
    bits[word] |= mask;
    Ok(())
}

struct ReceiptWindow {
    start: u64,
    end: u64,
    bits: [u64; RECEIPT_WORDS],
    count: u64,
}

impl ReceiptWindow {
    fn new(start: u64, sequence: u64) -> Self {
        Self {
            start,
            end: start.saturating_add(RECEIPT_WINDOW).min(sequence),
            bits: [0_u64; RECEIPT_WORDS],
            count: 0,
        }
    }

    fn reset(&mut self, start: u64, sequence: u64) {
        self.start = start;
        self.end = start.saturating_add(RECEIPT_WINDOW).min(sequence);
        self.bits.fill(0);
        self.count = 0;
    }

    fn observe(&mut self, sequence: u64) -> Result<()> {
        if sequence > self.start && sequence <= self.end {
            mark_receipt(&mut self.bits, sequence - self.start - 1)?;
            self.count += 1;
        }
        Ok(())
    }

    fn finish(&self) -> Result<()> {
        if self.count != self.end - self.start {
            return Err(corrupt());
        }
        Ok(())
    }
}

fn audit_receipts(
    tx: &ReadTransaction,
    state: &Value,
    generations: &BTreeSet<String>,
    sequence: u64,
    replacement_count: u64,
) -> Result<()> {
    let receipts = tx.open_table(RECEIPTS).map_err(storage_error)?;
    let history = tx.open_table(HISTORY).map_err(storage_error)?;
    let sequences = tx.open_table(SEQUENCES).map_err(storage_error)?;
    let times = tx.open_table(TIMES).map_err(storage_error)?;
    let replacements = tx.open_table(REPLACEMENTS).map_err(storage_error)?;
    let canonical = tx.open_table(CURRENT).map_err(storage_error)?;
    if receipts.len().map_err(storage_error)? != sequence {
        return Err(corrupt());
    }
    let mut window = ReceiptWindow::new(0, sequence);
    let mut supersedes = 0_u64;
    for entry in receipts.iter().map_err(storage_error)? {
        let (key, bytes) = entry.map_err(storage_error)?;
        let operation: AuditOperation =
            serde_json::from_slice(bytes.value()).map_err(storage_error)?;
        let AuditOperation {
            payload_hash,
            request,
            receipt,
            records,
        } = operation;
        request.validate()?;
        receipt.validate()?;
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
            || payload_hash != crate::contracts::hash(&canonical_request)
            || request.operation_id != receipt.operation_id
            || request.generation != receipt.operation_generation
            || request.workspace_id != state["workspace_id"]
            || !generations.contains(&request.generation)
            || seq == 0
            || seq > sequence
            || times
                .get(history::time_key(committed_at)?)
                .map_err(storage_error)?
                .is_none_or(|value| value.value() != seq)
            || receipt.results.len() != records.len()
        {
            return Err(corrupt());
        }
        window.observe(seq)?;
        let mut result_ids = BTreeSet::new();
        for result in &receipt.results {
            if !result_ids.insert(result.reference.id.as_str()) {
                return Err(corrupt());
            }
            let bytes = history
                .get((result.reference.id.as_str(), seq))
                .map_err(storage_error)?
                .ok_or_else(corrupt)?;
            let historical: GardenRecord =
                serde_json::from_slice(bytes.value()).map_err(storage_error)?;
            if result.revision != historical.metadata.revision
                || result.reference.kind != historical.draft.kind()
                || historical.metadata.actor != receipt.actor
                || !records.contains(&historical)
            {
                return Err(corrupt());
            }
        }
        let mut historical_count = 0_usize;
        for entry in sequences
            .range((seq, "")..=(seq, "\u{10ffff}"))
            .map_err(storage_error)?
        {
            let (key, _) = entry.map_err(storage_error)?;
            if !result_ids.contains(key.value().1) {
                return Err(corrupt());
            }
            historical_count += 1;
        }
        if historical_count != result_ids.len() {
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
            let source_record = current_record(&canonical, &source.id)?;
            if source.kind != target.kind
                || source_record.draft.kind() != source.kind
                || source_record.metadata.lifecycle != Lifecycle::Superseded
                || source_record.metadata.commit_sequence != seq
                || replacements
                    .get(source.id.as_str())
                    .map_err(storage_error)?
                    .is_none_or(|value| value.value() != target.id)
            {
                return Err(corrupt());
            }
            // A terminal source has one canonical commit sequence. Unique receipt
            // sequences and request targets make these forward matches disjoint.
            supersedes += 1;
        }
    }
    if supersedes != replacement_count {
        return Err(corrupt());
    }
    window.finish()?;
    #[derive(Deserialize)]
    struct SequenceProjection {
        receipt: ReceiptProjection,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ReceiptProjection {
        commit_sequence: u64,
    }
    while window.end < sequence {
        window.reset(window.end, sequence);
        for entry in receipts.iter().map_err(storage_error)? {
            let (_, bytes) = entry.map_err(storage_error)?;
            let projection: SequenceProjection =
                serde_json::from_slice(bytes.value()).map_err(storage_error)?;
            window.observe(projection.receipt.commit_sequence)?;
        }
        window.finish()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{RECEIPT_WINDOW, ReceiptWindow};

    #[test]
    fn receipt_windows_cover_boundaries_and_reset_between_passes() {
        let mut window = ReceiptWindow::new(0, RECEIPT_WINDOW + 2);
        window.observe(0).unwrap();
        window.observe(RECEIPT_WINDOW + 1).unwrap();
        for sequence in 1..=RECEIPT_WINDOW {
            window.observe(sequence).unwrap();
        }
        window.finish().unwrap();
        window.reset(RECEIPT_WINDOW, RECEIPT_WINDOW + 2);
        window.observe(RECEIPT_WINDOW).unwrap();
        window.observe(RECEIPT_WINDOW + 1).unwrap();
        assert!(window.finish().is_err(), "last sequence must be present");
        window.observe(RECEIPT_WINDOW + 2).unwrap();
        window.observe(RECEIPT_WINDOW + 3).unwrap();
        window.finish().unwrap();
        assert!(
            window.observe(RECEIPT_WINDOW + 1).is_err(),
            "second-window duplicate must fail"
        );
    }

    #[test]
    fn receipt_windows_reject_missing_and_cardinality_neutral_duplicate_sequences() {
        let mut window = ReceiptWindow::new(0, 4);
        for sequence in [1, 2, 4] {
            window.observe(sequence).unwrap();
        }
        assert!(window.finish().is_err(), "sequence 3 is missing");
        assert!(
            window.observe(4).is_err(),
            "a duplicate cannot replace missing sequence 3"
        );
        window.observe(3).unwrap();
        window.finish().unwrap();
        window.reset(0, 0);
        window.observe(1).unwrap();
        window.finish().unwrap();
    }
}
