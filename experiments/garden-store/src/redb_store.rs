//! Redb adapter. Every canonical, index, history and receipt write shares one transaction.
use crate::{
    Assertion, Batch, CommitStage, Counts, Entity, Receipt, Result, StoredAssertion, Traversal,
    TraversalResult,
};
use redb::{
    Database, Durability, ReadTransaction, ReadableDatabase, ReadableTable, ReadableTableMetadata,
    TableDefinition,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;
use std::time::{Duration, Instant};

const ENTITIES: TableDefinition<u64, &[u8]> = TableDefinition::new("entities");
const ASSERTIONS: TableDefinition<u64, &[u8]> = TableDefinition::new("assertions");
const HISTORY: TableDefinition<(u64, u64), &[u8]> = TableDefinition::new("history");
const OUT_HISTORY: TableDefinition<(u64, u64, u64), ()> = TableDefinition::new("outgoing_history");
const OUTGOING: TableDefinition<(u64, u64), ()> = TableDefinition::new("outgoing");
const INCOMING: TableDefinition<(u64, u64), ()> = TableDefinition::new("incoming");
const PREDICATE: TableDefinition<(u64, u64), ()> = TableDefinition::new("predicate");
const VALID: TableDefinition<(u64, u64), ()> = TableDefinition::new("valid_from");
const SOURCE: TableDefinition<(u64, u64), ()> = TableDefinition::new("source");
const NAMESPACE: TableDefinition<(u64, u64), ()> = TableDefinition::new("namespace");
const TRANSACTIONS: TableDefinition<(u64, u64), ()> = TableDefinition::new("transaction_sequence");
const RECEIPTS: TableDefinition<u64, &[u8]> = TableDefinition::new("operation_receipts");
const META: TableDefinition<u64, u64> = TableDefinition::new("metadata");

/// A single-process database handle; redb serializes its concurrent writers.
#[derive(Debug)]
pub struct RedbStore {
    db: Database,
}

/// A stable MVCC read view, including historical versions and operation receipts.
#[derive(Debug)]
pub struct Snapshot {
    tx: ReadTransaction,
}

fn encode(words: &[u64]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

fn decode<const N: usize>(bytes: &[u8]) -> Result<[u64; N]> {
    if bytes.len() != N * 8 {
        return Err("invalid synthetic binary record length".into());
    }
    let mut words = [0; N];
    for (word, chunk) in words.iter_mut().zip(bytes.chunks_exact(8)) {
        *word = u64::from_le_bytes(chunk.try_into()?);
    }
    Ok(words)
}

fn assertion_words(a: Assertion) -> [u64; 8] {
    [
        a.id,
        a.subject,
        a.predicate,
        a.object,
        a.valid_from,
        a.valid_until,
        a.source,
        a.namespace,
    ]
}

fn encode_stored(value: StoredAssertion) -> Vec<u8> {
    let mut words = assertion_words(value.assertion).to_vec();
    words.extend([value.revision, value.sequence]);
    encode(&words)
}

fn decode_stored(bytes: &[u8]) -> Result<StoredAssertion> {
    let [
        id,
        subject,
        predicate,
        object,
        valid_from,
        valid_until,
        source,
        namespace,
        revision,
        sequence,
    ] = decode(bytes)?;
    Ok(StoredAssertion {
        assertion: Assertion {
            id,
            subject,
            predicate,
            object,
            valid_from,
            valid_until,
            source,
            namespace,
        },
        revision,
        sequence,
    })
}

fn payload(batch: &Batch) -> Vec<u8> {
    let mut words = Vec::with_capacity(3 + batch.entities.len() * 3 + batch.assertions.len() * 9);
    words.extend([
        batch.operation_id,
        batch.entities.len() as u64,
        batch.assertions.len() as u64,
    ]);
    for entity in &batch.entities {
        words.extend([entity.id, entity.namespace, entity.source]);
    }
    for mutation in &batch.assertions {
        words.extend(assertion_words(mutation.assertion));
        words.push(mutation.expected_revision);
    }
    encode(&words)
}

fn decode_receipt(bytes: &[u8]) -> Result<Receipt> {
    if bytes.len() < 32 {
        return Err("invalid synthetic receipt".into());
    }
    let [operation_id, sequence, entity_count, assertion_count] = decode(&bytes[..32])?;
    Ok(Receipt {
        operation_id,
        sequence,
        entity_count,
        assertion_count,
    })
}

type CurrentIndex = (TableDefinition<'static, (u64, u64), ()>, u64);

fn current_indexes(a: Assertion) -> [CurrentIndex; 6] {
    [
        (OUTGOING, a.subject),
        (INCOMING, a.object),
        (PREDICATE, a.predicate),
        (VALID, a.valid_from),
        (SOURCE, a.source),
        (NAMESPACE, a.namespace),
    ]
}

impl RedbStore {
    /// Opens an experiment file and creates only the synthetic tables.
    ///
    /// # Errors
    /// Returns I/O, corruption or engine-lock errors.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::initialize(Database::create(path)?)
    }

    /// Opens a synthetic instrumented backend for interrupting actual engine persistence calls.
    /// Ordinary qualification benchmarks use [`Self::open`] with the standard file backend.
    ///
    /// # Errors
    /// Returns backend I/O or engine errors.
    pub fn open_with_backend(backend: impl redb::StorageBackend) -> Result<Self> {
        Self::initialize(Database::builder().create_with_backend(backend)?)
    }

    fn initialize(db: Database) -> Result<Self> {
        let mut tx = db.begin_write()?;
        tx.set_durability(Durability::Immediate)?;
        tx.open_table(ENTITIES)?;
        tx.open_table(ASSERTIONS)?;
        tx.open_table(HISTORY)?;
        tx.open_table(OUT_HISTORY)?;
        tx.open_table(TRANSACTIONS)?;
        tx.open_table(RECEIPTS)?;
        tx.open_table(META)?;
        for (definition, _) in current_indexes(Assertion {
            id: 0,
            subject: 0,
            predicate: 0,
            object: 0,
            valid_from: 0,
            valid_until: 1,
            source: 0,
            namespace: 0,
        }) {
            tx.open_table(definition)?;
        }
        tx.commit()?;
        Ok(Self { db })
    }

    /// Captures a read view independent of subsequent commits.
    ///
    /// # Errors
    /// Returns engine read errors.
    pub fn snapshot(&self) -> Result<Snapshot> {
        Ok(Snapshot {
            tx: self.db.begin_read()?,
        })
    }

    /// Applies one durable synthetic batch.
    ///
    /// # Errors
    /// Rejects duplicate targets, missing endpoints, revision mismatches or changed replay payloads.
    /// Commit errors may have uncertain durability; inspect the receipt after reopening.
    pub fn apply(&self, batch: &Batch) -> Result<Receipt> {
        self.apply_with_hook(batch, |_| {})
    }

    /// Applies a batch and reports only time spent acquiring redb's writer transaction.
    ///
    /// # Errors
    /// Has the same failure and uncertain-commit behavior as [`Self::apply`].
    pub fn apply_timed(&self, batch: &Batch) -> Result<(Receipt, Duration)> {
        self.apply_inner(batch, |_| {})
    }

    /// Runs deterministic crash barriers for synthetic tests and benchmark workers.
    /// `Staged` runs after entities and the first edge, before the remainder and receipt.
    /// The hook is experiment instrumentation and runs while holding the writer transaction.
    ///
    /// # Errors
    /// Has the same failure and uncertain-commit behavior as [`Self::apply`].
    pub fn apply_with_hook(&self, batch: &Batch, hook: impl FnMut(CommitStage)) -> Result<Receipt> {
        Ok(self.apply_inner(batch, hook)?.0)
    }

    fn apply_inner(
        &self,
        batch: &Batch,
        mut hook: impl FnMut(CommitStage),
    ) -> Result<(Receipt, Duration)> {
        if batch.entities.len() > 10_000 || batch.assertions.len() > 10_000 {
            return Err("synthetic batch exceeds 10000 targets per kind".into());
        }
        let exact_payload = payload(batch);
        hook(CommitStage::BeforeWrite);
        let started = Instant::now();
        let mut tx = self.db.begin_write()?;
        let wait = started.elapsed();
        tx.set_durability(Durability::Immediate)?;
        {
            let receipts = tx.open_table(RECEIPTS)?;
            if let Some(existing) = receipts.get(batch.operation_id)? {
                if existing.value().get(32..) != Some(exact_payload.as_slice()) {
                    return Err("operation payload conflict".into());
                }
                return Ok((decode_receipt(existing.value())?, wait));
            }
        }
        let sequence = {
            let meta = tx.open_table(META)?;
            meta.get(0)?.map_or(Ok(1), |value| {
                value.value().checked_add(1).ok_or("sequence overflow")
            })?
        };
        let mut seen = BTreeSet::new();
        {
            let mut entities = tx.open_table(ENTITIES)?;
            for entity in &batch.entities {
                if entity.id == 0 || !seen.insert(entity.id) {
                    return Err("invalid or duplicate entity ID".into());
                }
                let bytes = encode(&[entity.id, entity.namespace, entity.source]);
                if let Some(existing) = entities.get(entity.id)?
                    && existing.value() != bytes
                {
                    return Err("immutable entity conflict".into());
                }
                entities.insert(entity.id, bytes.as_slice())?;
            }
        }
        seen.clear();
        for (offset, mutation) in batch.assertions.iter().enumerate() {
            let a = mutation.assertion;
            if a.id == 0 || a.valid_from >= a.valid_until || !seen.insert(a.id) {
                return Err("invalid or duplicate assertion ID/interval".into());
            }
            {
                let entities = tx.open_table(ENTITIES)?;
                if entities.get(a.subject)?.is_none() || entities.get(a.object)?.is_none() {
                    return Err("missing edge endpoint".into());
                }
            }
            let previous = {
                let assertions = tx.open_table(ASSERTIONS)?;
                assertions
                    .get(a.id)?
                    .map(|value| decode_stored(value.value()))
                    .transpose()?
            };
            if previous.map_or(0, |value| value.revision) != mutation.expected_revision {
                return Err("expected revision conflict".into());
            }
            if let Some(previous) = previous {
                for (definition, component) in current_indexes(previous.assertion) {
                    tx.open_table(definition)?.remove((component, a.id))?;
                }
            }
            let stored = StoredAssertion {
                assertion: a,
                revision: mutation
                    .expected_revision
                    .checked_add(1)
                    .ok_or("revision overflow")?,
                sequence,
            };
            let bytes = encode_stored(stored);
            tx.open_table(ASSERTIONS)?.insert(a.id, bytes.as_slice())?;
            tx.open_table(HISTORY)?
                .insert((a.id, sequence), bytes.as_slice())?;
            tx.open_table(OUT_HISTORY)?
                .insert((a.subject, a.id, sequence), ())?;
            tx.open_table(TRANSACTIONS)?.insert((sequence, a.id), ())?;
            for (definition, component) in current_indexes(a) {
                tx.open_table(definition)?.insert((component, a.id), ())?;
            }
            if offset == 0 {
                hook(CommitStage::Staged);
            }
        }
        if batch.assertions.is_empty() {
            hook(CommitStage::Staged);
        }
        let receipt = Receipt {
            operation_id: batch.operation_id,
            sequence,
            entity_count: batch.entities.len() as u64,
            assertion_count: batch.assertions.len() as u64,
        };
        let mut bytes = encode(&[
            receipt.operation_id,
            sequence,
            receipt.entity_count,
            receipt.assertion_count,
        ]);
        bytes.extend(exact_payload);
        tx.open_table(RECEIPTS)?
            .insert(batch.operation_id, bytes.as_slice())?;
        tx.open_table(META)?.insert(0, sequence)?;
        hook(CommitStage::BeforeCommit);
        tx.commit()?;
        hook(CommitStage::Committed);
        Ok((receipt, wait))
    }

    /// Reads current snapshot counts.
    ///
    /// # Errors
    /// Returns engine read errors.
    pub fn counts(&self) -> Result<Counts> {
        self.snapshot()?.counts()
    }

    /// Reads an operation result after an uncertain response.
    ///
    /// # Errors
    /// Returns engine read or corrupt-record errors.
    pub fn receipt(&self, operation_id: u64) -> Result<Option<Receipt>> {
        self.snapshot()?.receipt(operation_id)
    }

    /// Proves exact index membership and cardinality against canonical current and history rows.
    ///
    /// # Errors
    /// Returns an error for missing, extra or inconsistent index rows.
    pub fn verify_indexes(&self) -> Result<Counts> {
        self.snapshot()?.verify_indexes()
    }
}

impl Snapshot {
    /// Reads one endpoint by its synthetic integer ID.
    ///
    /// # Errors
    /// Returns engine or corrupt-record errors.
    pub fn entity(&self, id: u64) -> Result<Option<Entity>> {
        self.tx
            .open_table(ENTITIES)?
            .get(id)?
            .map(|value| {
                let [id, namespace, source] = decode(value.value())?;
                Ok(Entity {
                    id,
                    namespace,
                    source,
                })
            })
            .transpose()
    }

    /// Reads the current assertion version.
    ///
    /// # Errors
    /// Returns engine or corrupt-record errors.
    pub fn assertion(&self, id: u64) -> Result<Option<StoredAssertion>> {
        self.tx
            .open_table(ASSERTIONS)?
            .get(id)?
            .map(|value| decode_stored(value.value()))
            .transpose()
    }

    /// Resolves known-time first, then applies half-open valid-time filtering.
    /// Known-time is a logical commit sequence, not a production UTC instant.
    ///
    /// # Errors
    /// Returns engine or corrupt-record errors.
    pub fn assertion_at(
        &self,
        id: u64,
        valid_at: u64,
        known_at: u64,
    ) -> Result<Option<StoredAssertion>> {
        let history = self.tx.open_table(HISTORY)?;
        let mut range = history.range((id, 0)..=(id, known_at))?;
        let value = range
            .next_back()
            .transpose()?
            .map(|(_, value)| decode_stored(value.value()))
            .transpose()?;
        Ok(value.filter(|value| {
            value.assertion.valid_from <= valid_at && valid_at < value.assertion.valid_until
        }))
    }

    /// Reads a bounded current outgoing adjacency list without temporal filtering.
    ///
    /// # Errors
    /// Returns engine or corrupt-record errors.
    pub fn outgoing(&self, id: u64, limit: u64) -> Result<Vec<StoredAssertion>> {
        let index = self.tx.open_table(OUTGOING)?;
        let mut results = Vec::new();
        for item in index
            .range((id, 0)..=(id, u64::MAX))?
            .take(limit.min(10_000) as usize)
        {
            let (key, _) = item?;
            results.push(
                self.assertion(key.value().1)?
                    .ok_or("outgoing points to missing assertion")?,
            );
        }
        Ok(results)
    }

    /// Expands historical outgoing edges while respecting independent hard limits.
    /// Every examined history key consumes work, including obsolete and filtered versions.
    /// Truncation is conservative when a result or work boundary is reached exactly.
    ///
    /// # Errors
    /// Rejects limits beyond the experiment safety caps or returns engine errors.
    pub fn neighbors(&self, query: Traversal) -> Result<TraversalResult> {
        if query.max_hops > 16 || query.max_results > 10_000 || query.max_work > 100_000 {
            return Err("traversal exceeds experiment hard limits".into());
        }
        let mut result = TraversalResult {
            assertions: Vec::new(),
            visited: 0,
            work: 0,
            truncated: false,
        };
        let mut frontier = VecDeque::from([(query.start, 0)]);
        let mut visited = BTreeSet::new();
        let mut returned = BTreeSet::new();
        let index = self.tx.open_table(OUT_HISTORY)?;
        while let Some((node, depth)) = frontier.pop_front() {
            if !visited.insert(node) {
                continue;
            }
            result.visited += 1;
            if depth >= query.max_hops {
                continue;
            }
            let mut candidates = BTreeMap::new();
            for item in index.range((node, 0, 0)..=(node, u64::MAX, u64::MAX))? {
                if result.work >= query.max_work {
                    result.truncated = true;
                    return Ok(result);
                }
                let (key, _) = item?;
                let (_, id, sequence) = key.value();
                result.work += 1;
                if sequence <= query.known_at {
                    candidates.insert(id, ());
                }
            }
            for id in candidates.keys() {
                let Some(value) = self.assertion_at(*id, query.valid_at, query.known_at)? else {
                    continue;
                };
                let a = value.assertion;
                if a.subject != node
                    || query
                        .predicate
                        .is_some_and(|predicate| a.predicate != predicate)
                    || !returned.insert(a.id)
                {
                    continue;
                }
                if result.assertions.len() as u64 >= query.max_results {
                    result.truncated = true;
                    return Ok(result);
                }
                result.assertions.push(value);
                frontier.push_back((a.object, depth + 1));
            }
        }
        Ok(result)
    }

    /// Reads counts consistently within this held view.
    ///
    /// # Errors
    /// Returns engine read errors.
    pub fn counts(&self) -> Result<Counts> {
        Ok(Counts {
            entities: self.tx.open_table(ENTITIES)?.len()?,
            assertions: self.tx.open_table(ASSERTIONS)?.len()?,
            history: self.tx.open_table(HISTORY)?.len()?,
            receipts: self.tx.open_table(RECEIPTS)?.len()?,
            sequence: self
                .tx
                .open_table(META)?
                .get(0)?
                .map_or(0, |value| value.value()),
        })
    }

    /// Reads a durable operation receipt in this snapshot.
    ///
    /// # Errors
    /// Returns engine or corrupt-record errors.
    pub fn receipt(&self, operation_id: u64) -> Result<Option<Receipt>> {
        self.tx
            .open_table(RECEIPTS)?
            .get(operation_id)?
            .map(|value| decode_receipt(value.value()))
            .transpose()
    }

    /// Checks exact index membership with matching cardinalities, without full-set allocation.
    ///
    /// # Errors
    /// Returns an error for any stale, missing or extra indexed entry.
    pub fn verify_indexes(&self) -> Result<Counts> {
        let counts = self.counts()?;
        for item in self.tx.open_table(ENTITIES)?.iter()? {
            let (key, value) = item?;
            let [id, _, _] = decode::<3>(value.value())?;
            if key.value() != id || id == 0 {
                return Err("canonical entity ID mismatch".into());
            }
        }
        let assertions = self.tx.open_table(ASSERTIONS)?;
        for item in assertions.iter()? {
            let (key, value) = item?;
            let value = decode_stored(value.value())?;
            if key.value() != value.assertion.id
                || self.entity(value.assertion.subject)?.is_none()
                || self.entity(value.assertion.object)?.is_none()
            {
                return Err("canonical assertion ID/endpoint mismatch".into());
            }
            for (definition, component) in current_indexes(value.assertion) {
                let index = self.tx.open_table(definition)?;
                if index.len()? != counts.assertions
                    || index.get((component, value.assertion.id))?.is_none()
                {
                    return Err("current index set mismatch".into());
                }
            }
            let history = self.tx.open_table(HISTORY)?;
            let mut latest =
                history.range((value.assertion.id, 0)..=(value.assertion.id, u64::MAX))?;
            let latest = latest
                .next_back()
                .transpose()?
                .map(|(_, row)| decode_stored(row.value()))
                .transpose()?;
            if latest != Some(value) {
                return Err("current assertion differs from latest history".into());
            }
        }
        // Empty canonical tables must not hide orphan index entries.
        for (definition, _) in current_indexes(Assertion {
            id: 0,
            subject: 0,
            predicate: 0,
            object: 0,
            valid_from: 0,
            valid_until: 1,
            source: 0,
            namespace: 0,
        }) {
            if self.tx.open_table(definition)?.len()? != counts.assertions {
                return Err("current index cardinality mismatch".into());
            }
        }
        let outgoing = self.tx.open_table(OUT_HISTORY)?;
        let transactions = self.tx.open_table(TRANSACTIONS)?;
        if outgoing.len()? != counts.history || transactions.len()? != counts.history {
            return Err("historical index cardinality mismatch".into());
        }
        let mut last_id = None;
        let mut last_revision = 0;
        for item in self.tx.open_table(HISTORY)?.iter()? {
            let (key, value) = item?;
            let value = decode_stored(value.value())?;
            let a = value.assertion;
            if assertions.get(a.id)?.is_none()
                || self.entity(a.subject)?.is_none()
                || self.entity(a.object)?.is_none()
            {
                return Err("history references missing canonical object".into());
            }
            if last_id != Some(a.id) {
                last_revision = 0;
                last_id = Some(a.id);
            }
            if key.value() != (a.id, value.sequence)
                || value.sequence > counts.sequence
                || value.revision != last_revision + 1
                || outgoing.get((a.subject, a.id, value.sequence))?.is_none()
                || transactions.get((value.sequence, a.id))?.is_none()
            {
                return Err("historical index set/revision mismatch".into());
            }
            last_revision = value.revision;
        }
        if counts.receipts != counts.sequence {
            return Err("receipt/sequence count mismatch".into());
        }
        let mut sequences = BTreeSet::new();
        for item in self.tx.open_table(RECEIPTS)?.iter()? {
            let (key, value) = item?;
            let receipt = decode_receipt(value.value())?;
            if key.value() != receipt.operation_id
                || receipt.sequence == 0
                || receipt.sequence > counts.sequence
                || !sequences.insert(receipt.sequence)
            {
                return Err("operation receipt sequence mismatch".into());
            }
        }
        Ok(counts)
    }
}
