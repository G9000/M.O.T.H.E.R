# Garden storage engine qualification

Measured 15 November 2026 on Windows. G2 experiment only; no live store upgrade.
This report accompanies the 14 November garden plan. Results are from synthetic
data and the isolated `g9000/mother-garden-engine` worktree.

## Candidate and boundary

The candidate is **redb 4.6.0**, exactly pinned with an independent Cargo.lock in
`experiments/garden-store`. It was released on 13 November 2026, is dual licensed
MIT/Apache-2.0, and declares Rust 1.90 as its minimum. The experiment declares the
same floor and was built with Rust/Cargo 1.93.0, x86_64-pc-windows-msvc.
The minimum-version toolchain itself was not run.

Sources checked: [released package](https://docs.rs/crate/redb/4.6.0),
[release manifest](https://github.com/cberner/redb/blob/v4.6.0/Cargo.toml),
[upstream platform CI](https://github.com/cberner/redb/blob/v4.6.0/.github/workflows/ci.yml),
and the downloaded package's manifest/build script. Context7 was used for current
transaction, snapshot and durability documentation; actual compiled signatures
come from the pinned release. Redb documents Immediate commits and snapshots in
its [transaction API](https://docs.rs/redb/4.6.0/redb/struct.WriteTransaction.html).

The default dependency tree contains redb, serde/serde_json and tempfile, with
Rust transitive dependencies and platform bindings. No SQLite, `*-sys` C database
library, C/C++ database build, LMDB or RocksDB is linked into the default
experiment. On Linux/macOS redb uses `libc` Rust bindings for OS file locks;
Windows uses OS APIs. This is a pure-Rust database engine claim, not a claim to
replace the operating system. The optional `sqlite-baseline` feature deliberately
adds bundled SQLite through rusqlite; that feature is excluded from this claim.
Mother's existing runtime still uses SQLite and was not converted.

The library compile-checks on installed Linux and macOS targets as well as
Windows. Runtime tests and measurements here are Windows-only. Upstream CI
includes Linux, macOS, Windows and Linux ARM; upstream configuration is evidence
of intended coverage, not a locally executed platform result.

## Method

Machine: AMD Ryzen 9 9900X, 12 cores/24 logical processors, 68,304,498,688 bytes
physical RAM; Windows 11 Pro build 26300. Temporary files reside on C:, NTFS,
WD_BLACK SN850X 2000GB NVMe SSD. About 101.6 GB was free before this work.
This is the owner's working machine; background applications, OS scheduling and
storage caches were not isolated. No OS/drive cache flush or physical power-cut
test was performed.

The generator streams seed-14 entities with five relationships per entity.
Endpoints load before edges. It uses synthetic integer IDs, numeric namespaces,
source IDs, logical transaction sequences and half-open valid-time ticks.
It does not implement the proposed v2 contract, observation-time provenance,
unknown extensions, authorization, owner review, text or vector retrieval.

Canonical entities, current assertions, append-only assertion history, adjacency,
predicate, valid-start, source, namespace, transaction and receipt tables commit
atomically. Receipt bodies retain the exact original compact request bytes for
collision-free replay comparison; current and historical assertion payloads
are both retained. All writes explicitly use `Durability::Immediate`.

| Index | Physical experiment key |
| --- | --- |
| Entity/assertion ID | `id` in separate canonical tables |
| Current outgoing/incoming | `(subject, assertionId)` / `(object, assertionId)` |
| Historical outgoing | `(subject, assertionId, transactionSequence)` |
| Predicate | `(predicate, assertionId)` |
| Valid-time candidate | `(validFrom, assertionId)`; upper bound checked on payload |
| Transaction sequence | `(sequence, assertionId)` |
| Source / namespace | `(source, assertionId)` / `(namespace, assertionId)` |
| Revision history | `(assertionId, sequence)` with revision in payload |
| Operation receipt | `operationId`, including exact request and result |

Every final dataset audits exact canonical/index membership and cardinality,
current-versus-latest-history equality, endpoint existence, historical revision
sequence and receipt sequence uniqueness. Index presence is not proof that every
possible query planner has been implemented: measured traversal uses historical
outgoing adjacency and filters its resolved versions.

Bulk chunks cover 256 entities and then their relationships. Bulk load time is
separate from small durable writes. Warm read samples follow 32 lookup/16 query
warmups; measured samples are 256 lookups and 128 neighborhood queries. Each
lookup includes opening a snapshot. Queries allow two hops, 32 returned edges
and 128 examined history keys; `validAt=5000` includes all initial edge intervals.
Historical correction/half-open exclusions are exercised in tests rather than
this broad-validity throughput distribution. Results explicitly report work,
visited nodes, returned edges and truncation.

The reopen child reads the same initial graph and sample IDs after closing the
first handle, before benchmark mutations. These are **application-cache-reopened
reads, not cold-disk reads**. Four write warmups precede 64 independent durable
32-assertion batches. A separate four-reader/two-writer start-barrier workload
collects 512 reader-query and 64 writer-batch samples, including time waiting for
the single writer transaction. Scheduling does not guarantee continuous overlap.
Percentiles use nearest-rank order statistics; p99 of 64 writes is the maximum.

Windows working-set monitoring samples the benchmark processes every 100 ms.
The OS peak field records each observed process's peak since startup. A short
reopen child can complete between samples; these figures are observed process
working sets, not a complete system-RAM or allocator-residency measurement.

## Measurements and decision

**Conditional GO for redb as the G4 storage prototype after G3 contracts. NO-GO
for replacing Mother's canonical runtime or promising a universal 100 ms write
latency.** Correctness tests and the 10k/100k bounded-workload latency gates pass
on this Windows machine. The separately measured 1m stress run completes, but
two contending writers exceed the provisional 100 ms p95 target. Keep durable
commits and use a bounded writer admission/scheduling design; qualify that design
before making a user-facing tail-latency promise.

Times below are ms. Each cell is **p50 / p95 / p99**; raw sample summaries,
counts, work, fanout and memory observations are retained in
[`garden-engine-evidence/`](garden-engine-evidence/).

| Dataset (entities / initial relationships) | Warm exact lookup | Warm bounded temporal neighborhood | Durable 32-assertion batch |
| --- | --- | --- | --- |
| 10k / 50k | 0.0010 / 0.0014 / 0.0016 | 0.0355 / 0.0388 / 0.0758 | 7.1633 / 8.7036 / 11.1529 |
| 100k / 500k | 0.0017 / 0.0023 / 0.0025 | 0.0463 / 0.0586 / 0.0611 | 11.4679 / 14.8014 / 15.4218 |
| 1m / 5m (stress) | 0.0053 / 0.0092 / 0.0115 | 0.1072 / 0.1467 / 0.1873 | 16.4961 / 21.7718 / 26.5426 |

Provisional warm p95 budgets are 10 ms lookup, 50 ms bounded query and 100 ms
durable batch. All uncontended rows pass. All initial queries examined 30 edge
versions and returned 30 edges (maximum 31 visited nodes); none were truncated.
Limits and truncation are separately tested with restrictive bounds. Sparse
five-edge neighborhoods and current knowledge do not establish high-degree or
history-heavy query performance. Predicate/source/namespace/valid-start index
scans are maintained and audited, but not independently benchmarked.

| Dataset | Bulk durable load (s) | Final DB (MiB) | Observed parent peak working set (MiB) |
| --- | ---: | ---: | ---: |
| 10k | 0.907 | 34.004 | 42.664 |
| 100k | 22.364 | 272.004 | 330.164 |
| 1m | 336.751 | 2523.504 | 1843.250 |

Final graph counts include 4,224 additional edges from write warmups and measured
small/concurrent batches. At 1m, the exact audit passed with 1,000,000 entities,
5,004,224 current assertions, 5,004,224 history rows and 7,946 receipts/sequences.
The five-million-edge initial workload has one version per edge; sustained
correction-history growth is not a stress measurement here. End-to-end 1m elapsed
time, including audits/reopen/measurement/cleanup, was 392.350 s. A transient
database file of 4,294,971,392 bytes was observed during the run, before close
reduced it to 2,646,085,632 bytes. The transient observation is a lower bound on
peak disk use, not a sampled peak series.

| Dataset | Reopened exact lookup | Reopened bounded neighborhood |
| --- | --- | --- |
| 10k | 0.0035 / 0.0116 / 0.0295 | 0.0623 / 0.1187 / 0.1348 |
| 100k | 0.0071 / 0.0227 / 0.0503 | 0.1012 / 0.1754 / 0.2021 |
| 1m | 0.0084 / 0.0287 / 0.0628 | 0.1470 / 0.2312 / 0.2573 |

The reopened-process open times were 0.945, 1.087 and 8.497 ms respectively.
These values do not establish cold-disk service latency.

| Dataset | Contended writer batch | Writer acquisition wait p95 / p99 | Concurrent reader query p95 / p99 |
| --- | --- | --- | --- |
| 10k | 7.6929 / 43.4459 / 67.2669 | 36.9398 / 58.3343 | 0.3913 / 5.5380 |
| 100k | 11.7020 / 37.7755 / 308.6222 | 23.8902 / 297.1301 | 1.2461 / 10.5364 |
| 1m | 17.3864 / 140.2036 / 259.0167 | 118.8498 / 244.2971 | 2.2656 / 13.5798 |

The 1m contended p95 failure and 100k p99 wait are retained as failure evidence.
No durability or validation setting was relaxed to improve those results.
One sample run per dataset is evidence for this prototype decision, not a
confidence interval or a production service-level guarantee.

### Existing v1 API baseline

This uses the unchanged Mother Store implementation, including authenticated
record writes/reads, validation, durable receipts and its 10,000-record ceiling.
It creates a disposable workspace and never loads an embedding model. The
literal candidate path is Mother's existing Search implementation. Times are ms.

| Operation | Samples | p50 | p95 | p99 |
| --- | ---: | ---: | ---: | ---: |
| Individual authenticated record write | 10,000 | 0.3222 | 0.4940 | 0.8897 |
| Reopened-handle lookup | 512 | 0.0213 | 0.0327 | 0.0647 |
| Warm lookup | 512 | 0.0202 | 0.0262 | 0.0642 |
| Literal substring candidates | 128 | 183.1963 | 206.6198 | 239.3677 |

Load took 3.677 s. The database and WAL together used 15,847,424 bytes. These
authenticated v1 operations are not equivalent to graph batches/traversals, so
no speed ratio versus redb is claimed. The existing literal path misses the
proposed 100 ms p95 lexical budget here; a new lexical candidate index needs its
own G6 qualification. This experiment makes no redb lexical performance claim.

The optional SQLite semantic-reference run at 10k completed successfully and
checked 256 generated assertion payloads and 128 complete bounded-query results
against redb. Its raw output is retained as `engine-sqlite-reference.json`.
Because its encoding, indexes and validation work differ, no speed ranking is
derived from that output. It is not a replacement for the v1 API measurements.

## Correctness and validation

The first atomic-store test failed against the unimplemented apply boundary,
then passed with the transactional implementation. Review found and corrected
endpoint loading order, index audit completeness, child cleanup/deadlines and
the initial warm/reopen dataset drift. Compilation caught checked SQLite
integer conversion needs; strict Clippy caught type-complexity and conditional
style issues. Final checks below passed after those fixes.

Five storage tests cover atomic/idempotent reopen, changed-request receipt
conflicts, stale revisions, late invalid-endpoint rollback, exact index updates,
half-open valid time, old known-time reconstruction, stable snapshots, writer
blocking and independent traversal limits. Two CLI tests cover synthetic-only
input and actual child termination at five transaction barriers:

1. Before starting the write transaction.
2. After a first assertion and indexes have been staged, with the rest absent.
3. After all rows/indexes/receipt are staged, before commit.
4. After Immediate commit returns, simulating an unobserved acknowledgement.
5. Inside commit's first armed file-backend `sync_data` callback, before the
   delegated flush. This proves commit was executing, not only imminent.

After each kill, reopen audits all canonical/index/history/receipt state, then
retries the identical operation and proves one result. The in-flight commit
allows either whole committed or whole absent state, never a partial batch.
These are 32-entity/32-assertion recovery fixtures, not scale-sized crash tests.
The last barrier does not interrupt a kernel flush mid-syscall, simulate torn
writes or prove drive behavior under physical power loss.

| Check | Executed outcome |
| --- | --- |
| Experiment `cargo fmt --all -- --check` | Passed. |
| Experiment `cargo test --locked` | Seven tests passed, zero failures. |
| Experiment `cargo test --locked --all-features` | Seven tests passed, zero failures. |
| Experiment `cargo build --release --locked` | Passed; default pure-Rust build restored after the optional reference run. |
| Experiment `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed. |
| Linux/macOS `cargo check --locked --all-targets --no-default-features --target ...` | Both passed for x86_64-unknown-linux-gnu and x86_64-apple-darwin. Compile checks, not runtime tests. |
| Mother root formatting, `cargo test --locked`, release build and strict all-target/all-feature Clippy | Passed with the new baseline example; 283 tests passed, two optional model tests ignored. |
| Synthetic 10k / 100k / 1m release benchmarks | All completed; canonical/index audits passed. |
| Optional SQLite semantic cross-check | Passed at 10k. |
| `git diff --check` | Passed. |

Raw synthetic measurement JSON, memory observations, dependency trees and
build hashes are retained under `garden-engine-evidence/`; temporary databases
and task logs are removed after recording results. No root dependency, v1 schema,
live grant, client config or personal record was changed. The experiment and
baseline example are uncommitted in the isolated worktree.

## Conditions on adoption

Proceed with the graph contract work (G3) and a redb-backed prototype (G4).
Keep the public model, permissions and migration independent from this synthetic
integer-key layout. The measured latency headroom makes redb a reasonable engine
foundation; implementing a new database engine is not justified by this evidence.

Production replacement remains a no-go until the later gates establish graph
authorization/review, indexed lexical retrieval, compatibility and backups,
explicit migration, and real runtime acceptance. Also qualify writer admission
under the observed contention tails, high-degree/history-heavy workloads and
the intended Windows/Linux/macOS runtime matrix. No new canonical runtime
dependency should be added on the strength of this report alone. The v1 release
continues to work independently of that future adoption.

## Reproduction

From `experiments/garden-store`:

```powershell
cargo test --locked
cargo run --release --locked -- --synthetic --records 100000 --edges-per-record 5 --seed 14
```

Use `--records 10000` or `--records 1000000` for the other runs. The reported
final scale runs invoke that same optimized executable directly to sample its
process memory. The exact plan command above was also run successfully.

The optional SQLite semantic reference uses the same generator and checks
sampled payloads/traversals against redb. It uses JSON and different index and
validation work, so its times are **not a database-engine speed comparison**.
Run with `--features sqlite-baseline` and append `--sqlite`. The root
`garden_v1_baseline` example instead calls Mother's unchanged authenticated v1
Store API and literal Search. Neither comparison silently removes validation
from Mother or flattens graph operations into v1 records.
