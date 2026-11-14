# Synthetic garden storage qualification

This independent Cargo package qualifies redb for a future Mother garden. It is
not linked into Mother's runtime and does not open its personal data directory.
Every public benchmark invocation requires `--synthetic`; databases use owned
temporary directories and are removed on completion. No database-path flag is
accepted. Internal subprocess modes require a marked temporary directory.

```powershell
cargo test --locked
cargo run --release --locked -- --synthetic --records 100000 --edges-per-record 5 --seed 14
```

Supported dataset sizes are 10,000, 100,000 and 1,000,000 entities. Five edges per
entity is the qualification workload. Endpoints load first, then relationships
in chunks of 256 entities, each transaction using `Durability::Immediate`.
Small-write samples are separate 32-assertion transactions, not amortized bulk
load numbers. JSON output includes sample counts and nearest-rank p50/p95/p99,
bounded traversal fanout, writer waits and database size.

The warm and reopened-process reads use the same initial graph and sample IDs.
Reopening clears redb's application cache; it does not flush Windows or drive
caches. A 4-reader/2-writer workload reports transaction contention separately.
Peak RAM must be measured externally; it is not inferred from database size.

```powershell
cargo test --locked --all-features
cargo run --release --locked --features sqlite-baseline -- --synthetic --records 10000 --edges-per-record 5 --seed 14 --sqlite
```

`sqlite-baseline` enables an optional SQLite semantic reference. It cross-checks
sampled graph results, but uses different JSON encoding, indexes and validation
costs. Its timings must not be used for an engine speed ranking. SQLite and its
bundled C library are absent from the default feature dependency graph. For the
unchanged v1 runtime, run the separate root example instead:

```powershell
cargo run --release --locked --example garden_v1_baseline -- --synthetic --records 10000
```

Run that command from Mother's root, not this package. It includes the existing
authentication, validation, receipts and v1 capacity limit; it is contextual
baseline evidence rather than a graph performance comparison.

Storage tests check atomic batches, idempotent receipts, stale revisions,
rollback, half-open validity, historical corrections, snapshot isolation,
serialized writers and independent traversal limits. Recovery tests kill actual
children before a write, mid-mutation, before commit, after commit, and inside
the file backend sync callback invoked by commit. They verify all indexes and
exact retry outcomes. This tests process interruption, not physical power loss.

The integer IDs, compact records and logical transaction-time ticks are
experiment types, not v2 contracts. No authorization, owner inbox, migration,
lexical index, embedding model or backup implementation is qualified here.
See [the qualification report](../../docs/benchmarks/2026-11-14-garden-engine.md)
for measurements, platform evidence and the adoption decision.
