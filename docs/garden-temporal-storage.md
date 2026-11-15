# Transactional temporal graph storage (G4)

Mother uses the independent Rust `garden::store::GardenStore` and pinned redb storage as its sole runtime. Graph state and the explicitly enabled v1 compatibility view share canonical transaction authority. The SQLite runtime and converter are removed; old configs fail explicitly. See [native compatibility](garden-compatibility.md), [graph setup](garden-proposals.md), [full backups](garden-backups.md) and [the local owner service](garden-service.md).

This guide retains the G4 storage foundation and its historical qualification below. Statements in that qualification about future slices and inactive graph transport describe that milestone only; current graph operations and native v1 tools are implemented with independent scopes.

## Authority and durability

An owner explicitly initializes a new private graph file with a workspace ID and
generation, then provisions graph credentials. Initialization refuses an existing
destination; opening does not create a missing file. These credentials are
independent of v1 context authority; no SQLite grants are loaded by this runtime. Credentials and permissions are not imported
automatically from another harness.

The caller chooses an explicit private data path outside the checkout. File
creation reuses Mother's exclusive private-file helper; Windows permissions are
inherited from the parent directory, so that directory must already be private.

Graph principals, scopes, subject and object policies live in redb alongside
canonical data. Grant changes, revocation and graph writes use the same exclusive
writer. Each read opens a fresh snapshot and checks its current credential and
policy. The library does not return reusable authorization snapshots. Shared
clients receive bounded read access; writes require an explicit graph write scope.

One immediate-durability transaction commits the complete batch: current records,
immutable revisions, temporal and graph indexes, lifecycle relations, change log,
operation receipt and metadata. A stale revision, hidden or absent endpoint, or
invalid lifecycle transition aborts the entire batch. Retiring an endpoint or
evidence cannot leave active assertions dangling. Supersession chains cannot
cycle; ordinary graph relations can.

An operation UUID is scoped to its generation and authenticated client. An exact
retry returns the original mappings and revision payloads, including after reopen
and later updates. Reusing that UUID with a different payload fails. Replays and
receipt lookups recheck current authority and reference visibility before exposing
historical outcomes. A storage failure during commit may have an uncertain
outcome: reconcile using the original operation UUID.

## Temporal meaning

Every revision separates validity in the world, source observation time and
store-issued transaction time. Unknown source times remain unknown. Transaction
times advance monotonically at nanosecond precision even when the runtime clock
repeats or moves backward, including after reopen. A commit sequence orders
canonical changes.

Historical reads use transaction time (`knownAt`) and never substitute a revision
written later. Validity uses half-open intervals: the start is included and the
end excluded. Late observations and backdated corrections append revisions
without erasing what the garden previously believed. Conflicting assertions can
coexist. Extraction, interpretation and selection remain harness responsibilities.

IDs, revision links, sequence, transaction time and actor attribution are
store-issued. Updates preserve the original contributor and merge extension
namespaces, retaining unmentioned opaque values. The merged result is validated
against the same structural bounds as a new record.

Before commit, every revision must fit the mandatory single-record response,
including the largest future sequence watermarks. A batch may omit its optional
record payloads when the combined response would exceed the wire bound; its
immutable canonical mappings remain available.

## Boundaries

The G4 foundation supplies atomic graph batches, lifecycle transitions and authorized temporal reads. Later native slices add proposal review, indexed queries, traversal/pagination, change feeds, optional local vectors, v1 compatibility and complete backups on the same redb authority. The runtime performs no automatic memory extraction or transcript ingestion.

Routine operations use point lookups and indexed retirement dependencies. Their referenced-object closure has a 10,000-object work ceiling and fails with `LimitExceeded` when exceeded. The owner-authorized consistency audit deliberately examines the whole store. Derived indexes never replace current canonical authority.

One local owner service holds the database handle for multiple authenticated stdio bridges. Direct owner administration requires stopping that service. Existing cutover fences and journals remain enforced; new SQLite conversion and source activation are unavailable.

The graph engine is pure Rust. Native inbox rendering uses Rust terminal libraries and retained operating-system support. Historical prototype storage measurements do not establish current production latency or cross-platform physical power-loss guarantees; see [validation](validation.md).

## Historical G4 validation

Synthetic tests cover atomic rollback, immutable retries, current revocation,
endpoint visibility, lifecycle rules, extension retention, clock rollback,
backdated corrections, conflicting assertions and reopen. The recovery test
terminates an owned child running the actual store before and during commit and
after durable commit. Reopened records, revisions, indexes and receipts must be
complete or absent; retrying the original operation reconciles both outcomes.
These are local process-termination tests, not hardware power-cut or live-harness
acceptance.

Executed on Windows on 15 November 2026 with synthetic data and owner-private
temporary directories:

| Check | Result |
| --- | --- |
| Five focused graph test targets | 21 passed; one private child helper ignored by the ordinary runner and exercised through its parent. |
| Process recovery | Ten interruption cases passed: five commit barriers each for initial graph creation and supersession with assertion rewiring. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test --locked` | 332 passed, zero failed, three ignored. |
| `cargo build --release --locked` | Passed. |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed. |
| `cargo doc --locked --no-deps` | Passed. |
| `git diff --check` | Passed. |

The other two ignored tests require an explicitly installed real embedding
model. No model accounts, external providers, installed clients or live stores
were used. Local tests do not replace Linux/macOS CI or hardware durability
qualification.

Tests first established the absent storage APIs. Further failing regressions
demonstrated retired assertions incorrectly matching current validity, hidden
endpoint/dependency error disclosure, and near-limit records committing without
fitting their mandatory read response. All were corrected and passed on the
final source. Independent review found no remaining G4 blockers.

## G4 completion

- [x] Atomic entity/assertion/evidence batches, original-response retry, UUID reuse and stale-revision checks.
- [x] Qualified engine behind garden-owned storage with canonical indexes in the same transaction.
- [x] Append-preserving valid/observed/transaction times and exact `knownAt` reconstruction.
- [x] Explicit supersession relationships, endpoint authorization and lifecycle-cycle rejection.
- [x] Current grant-aware history, mutations and receipt visibility before commit.
- [x] Conflicting assertions, late evidence and backdated corrections preserve old beliefs.
- [x] Reopen, exact index/history/receipt audit and interrupted-batch acceptance.

Changes remain uncommitted in `g9000/mother-garden-storage`, based on the verified
G3 source. Existing pairing changes in the primary checkout remain untouched.
The subsequent [G5 slice](garden-proposals.md) supplies graph proposals, local
owner approval and an explicit prototype transport. Full query support, v1
compatibility and live migration remain later gates.
