//! Synthetic storage qualification only; this crate is not a garden runtime or wire contract.
mod model;
mod redb_store;

pub use model::{
    Assertion, Batch, CommitStage, Counts, Entity, Mutation, Receipt, StoredAssertion, Traversal,
    TraversalResult, synthetic_assertion, synthetic_entity,
};
pub use redb_store::{RedbStore, Snapshot};

/// Experiment errors, including engine failures and invalid synthetic batches.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// A fallible experiment operation.
pub type Result<T> = std::result::Result<T, Error>;
