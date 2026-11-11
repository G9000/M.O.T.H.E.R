# Checkpoint supersession

**Problem.** Every session saves a checkpoint and nothing retires the previous one, so active checkpoints accumulate in recall.

**Decision.** A checkpoint write may carry `supersedes: {id, expectedRevision}`. In the same transaction the store marks that earlier active checkpoint `superseded`, as an attributed revision with its own receipt. Any failed check rejects the whole write.

**Unchanged.** The v1 schema, record and receipt shapes, scopes, idempotent retries and export format. A request without `supersedes` hashes and behaves exactly as before.

**Not included.** Superseding several checkpoints in one write, automatic supersession of a client's previous checkpoint, and retroactive cleanup of existing active checkpoints.

**Acceptance.** `tests/supersede.rs`.
