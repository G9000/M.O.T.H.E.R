//! Compact synthetic graph types. Integer IDs and logical known-time are experiment-only.
use serde::{Deserialize, Serialize};

/// An immutable synthetic endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
    pub namespace: u64,
    pub source: u64,
}

/// A synthetic edge with half-open, unsigned validity ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assertion {
    pub id: u64,
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
    pub valid_from: u64,
    pub valid_until: u64,
    pub source: u64,
    pub namespace: u64,
}

/// An edge write guarded by its current revision; zero creates an edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mutation {
    pub assertion: Assertion,
    pub expected_revision: u64,
}

/// One atomic synthetic operation, including its exact replay identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Batch {
    pub operation_id: u64,
    pub entities: Vec<Entity>,
    pub assertions: Vec<Mutation>,
}

/// A committed assertion version; sequence is experiment logical transaction time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredAssertion {
    pub assertion: Assertion,
    pub revision: u64,
    pub sequence: u64,
}

/// Durable operation result stored atomically with graph state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub operation_id: u64,
    pub sequence: u64,
    pub entity_count: u64,
    pub assertion_count: u64,
}

/// Barriers for deterministic synthetic interruption experiments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitStage {
    BeforeWrite,
    Staged,
    BeforeCommit,
    Committed,
}

/// Snapshot counts; current edges and append-only history are counted separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub entities: u64,
    pub assertions: u64,
    pub history: u64,
    pub receipts: u64,
    pub sequence: u64,
}

/// Hard limits for outgoing breadth-first expansion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Traversal {
    pub start: u64,
    pub max_hops: u64,
    pub max_results: u64,
    pub max_work: u64,
    pub valid_at: u64,
    pub known_at: u64,
    pub predicate: Option<u64>,
}

/// Bounded results with explicit truncation and examined-history work count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraversalResult {
    pub assertions: Vec<StoredAssertion>,
    pub visited: u64,
    pub work: u64,
    pub truncated: bool,
}

/// Generates an endpoint without retaining the complete dataset in memory.
pub fn synthetic_entity(id: u64) -> Entity {
    Entity {
        id,
        namespace: id % 8,
        source: id % 32,
    }
}

/// Generates one deterministic edge; IDs are one-based and edge offsets zero-based.
///
/// # Panics
/// Callers must supply a positive record count and entity ID; the experiment CLI
/// validates dataset sizes and multiplication overflow before generating records.
pub fn synthetic_assertion(
    entity_id: u64,
    edge_idx: u64,
    record_count: u64,
    edges_per_record: u64,
    seed: u64,
) -> Assertion {
    let mixed = entity_id
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(seed)
        .wrapping_add(edge_idx.wrapping_mul(1_442_695_040_888_963_407));
    Assertion {
        id: (entity_id - 1) * edges_per_record + edge_idx + 1,
        subject: entity_id,
        predicate: edge_idx % 8,
        object: mixed % record_count + 1,
        valid_from: (entity_id + edge_idx) % 1_000,
        valid_until: 10_000 + (entity_id + edge_idx) % 1_000,
        source: entity_id % 32,
        namespace: entity_id % 8,
    }
}
