# Indexed graph queries and changes (G6)

The explicit graph-only connection now serves `mother_garden_query` and
`mother_garden_changes` alongside the five G5 operations and capability discovery.
Both require current `garden:read` authority. The default v1 connection and its
tool catalogue remain unchanged. No installed connection or live store is
reconfigured by this implementation.

## Query semantics

Queries select typed IDs, kinds, entity types, predicates, typed properties,
literal text, validity, knowledge time and directed traversal. Canonical records
and current plus historical index memberships commit together. ID, adjacency,
predicate, property and numeric time ranges drive candidate selection; selective
queries do not enumerate the whole canonical table.

`validAt` uses half-open validity. `knownAt` reconstructs the latest revision
known at that transaction time, including backdated corrections. Explicit
historical queries can return an old active belief while current permissions
still govern disclosure. Current queries and their continuation pages suppress
objects or referenced endpoints/evidence that are now inactive.

Typed property filters match an assertion whose predicate is `property.name`
and whose object is the same `TypedValue`. They also match an extension namespace
whose payload equals the serialized typed value. Signed numeric zero compares
consistently in indexed and allowlisted paths. Optional extension payloads remain
opaque to generic text/embedding extraction.

Traversal returns traversed assertions and newly reached neighbor entities.
`kinds` can select entities only. Start nodes do not become results merely by
being starts. Direction is incoming/outgoing/both; predicate and validity filters
constrain traversed edges. Other requested filters constrain returned candidates.
Ordinary graph cycles are allowed and deduplicated.

Every hit carries separate lexical/vector component scores and authorized
evidence references. These are provider candidates for the harness to interpret
and rank. Limits never silently imply complete coverage: inspect `coverage`,
its reason and the page cursor.
A query hit carries a mandatory full record. If one hit alone exceeds that
response envelope, the query returns `LIMIT_EXCEEDED`; use exact Get for a known
record ID. Change events have a separate metadata-only fallback described below.

## Stable pages and current safety

The first query fixes a canonical cutoff and bounded candidate set. Opaque random
cursors identify private lease state, bound to actor, credential/policy fingerprint,
workspace, generation, visibility epoch and normalized request digest. Reopening
the store preserves valid continuations. Keep the query unchanged while paging.

The five-minute deadline is fixed at issuance. Retrying a cursor preserves its
successor token and deadline. Expiry, clock rollback, altered queries, wrong
actors, changed grants/policies or generation produce `RESTART_REQUIRED`;
revoked credentials fail authorization. Current and historical footprints are
rechecked on every page. A canonical write interleaving with one serving request
causes a restart so a read cannot return a concurrently withdrawn payload.

Candidate arrays are stored once per snapshot; compact cursor positions share
them. The prototype bounds materialization to 1,024 candidates, 512 snapshot
blobs and 4,096 cursor positions, with a 262,144-byte bound per stored row/blob.
Expired cursor and snapshot cleanup is independently bounded. A broad query may
report truncated coverage before exhausting a large garden; refine its filters.

Allowlisted paths enumerate permitted IDs and validate full footprints before
public work budgets, expansion or scoring. Hidden endpoints, evidence, source
payloads and candidate counts are excluded. Existing global commit sequences
and watermark metadata remain visible, so changes in them can reveal workspace
write activity. Trusted raw history/authorization ceilings return generic
resource errors rather than raw skipped counts. These ceilings protect the local
process and can require a narrower request.

## Change tracking

A fresh changes request starts at sequence zero and returns committed revisions
ordered by `(sequence, record ID)`. Multiple items in the same transaction are
paged without gaps or duplicates. Pending proposals do not enter this stream;
approved graph revisions do. Retracted/superseded changes are tombstones with
no canonical record payload. Old active events do not supply content that has
since become inactive or unreadable.

Mid-snapshot pages preserve their cutoff. Once caught up, the returned non-null
checkpoint cursor can poll future commits until the same original deadline.
Adopt the returned cursor even after an empty page. Repeating an older live
checkpoint can return an already-seen ordered prefix plus later events;
deduplicate by canonical ID/revision/sequence. It cannot extend the deadline.
Page size also respects the serialized response budget, including large records.
When an otherwise authorized record cannot fit the feed's byte/depth envelope,
the event still carries its ID/revision/sequence/lifecycle with `record: null`.
Coverage reports `event_payload_omitted`, and the cursor advances. Use exact Get
to retrieve that record separately. Inspect lifecycle to distinguish this active
metadata-only event from a tombstone.

## Optional local vector candidates

Literal substring postings index normalized core text, including short queries.
Vector candidates use a separate rebuildable redb cache bound to generation,
canonical revision/digest and the complete versioned model/profile/dimension
fingerprint. Source changes, retractions and permission changes are checked
before cached candidates can become output.

The default optional descriptor identifies the existing pinned MiniLM encoder
and immutable upstream revision, pooling/window policy and dimensions. Descriptor
inspection loads no model. Owner preparation uses only already-installed,
hash-verified local assets; it does not download or call a provider:

```text
mother garden index --config <graph-private.json> --objects <selected-typed-ids.json>
```

The source file contains an array of typed `{kind,id}` references. Preparation
requires authorized canonical sources, at most 100 references and 4 MiB of
selected source records. Authority, generation and revisions are checked again
after computation. Missing assets return `local_model_unavailable` without
changing canonical records. The trusted Rust SDK also exposes
`prepare_search_with` for an owner-installed vectorizer adapter; MCP arguments
cannot install such an adapter.

Vector requests provide a query vector, model identity, profile and dimensions.
Only matching cached vectors participate; lexical/vector scores remain separate.
Unavailable matching vectors produce explicit literal fallback when text is
provided, and an empty incomplete vector result otherwise. Restricted coverage
does not change merely because a hidden record was indexed. Derived coverage
remains conservative (`indexWatermark: 0`, `vector_index_partial`); exact/filter
and literal paths use synchronous canonical indexes. Stale vector revisions are
retained within this prototype and may consume a bounded candidate scan; cache
retention and large-scale performance remain later qualification work.

## Upgrade and acceptance

Existing G5 graph files receive additive current/historical indexes and lease
tables on their first open by this runtime, preserving identity, generation,
grants and canonical history. Later opens skip backfill. Audit checks exact
current/historical memberships and numeric temporal indexes. This is separate
from the later v1 compatibility, full backup and live migration work.

Executed on Windows on 15 November 2026 with synthetic records and owner-private
temporary stores:

| Check | Result |
| --- | --- |
| Indexed query tests | 13 passed, including exact/temporal/property filters, directed neighbors, cyclic traversal budgets, current evidence visibility, cursor identity/expiry/rollback, reopen and 514 one-result pages. |
| Change-feed tests | 10 passed, including checkpoints, tombstones, hidden evidence, grant changes, duplicate allowlists and size/depth payload fallback. |
| Index upgrade/audit | Two passed; G5 identity, grants, history and records survive additive backfill, and extraneous historical memberships fail audit. |
| Optional search | Five passed with synthetic vectorizers: profile fingerprints, compatibility, current authority, invalid vectors, hidden-index coverage and stale revision suppression. |
| Graph stdio | Both tests passed: G5 owner-review compatibility and G6 snapshot/change-checkpoint operations across actual process boundaries. |
| Owner CLI | Four passed, including explicit selected preparation with missing local assets and no canonical/model-directory changes. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test --locked` | 396 passed, zero failed, three ignored. |
| `cargo build --release --locked` | Passed. |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed. |
| `cargo doc --locked --no-deps` | Passed. |
| `git diff --check` | Passed. |

The ignored tests are two optional real embedding-model tests and the recovery
child helper that its parent executes explicitly. Existing v1 access, semantic,
MCP, inbox and G4/G5 storage tests pass; the full suite also retains twenty
process-interruption recovery cases. Graph learned-vector integration was checked
with synthetic adapters, without installing or running a real model. No provider,
real conversation, live runtime database or installed harness setting was used.

Failing regressions established missing APIs/tools, signed-zero index mismatch,
missing neighbor results, duplicated allowlist events, response limits, and the
512-token paging capacity problem before their fixes. Independent review found
no remaining blocking G6 findings. These checks establish local synthetic
protocol, storage and recovery behavior, not Linux/macOS CI, real host adoption,
hardware power-loss acceptance or production workload latency.

## G6 completion

- [x] Indexed exact, directed neighbor, predicate/property and combined valid/known-time queries.
- [x] Bounded traversal and stable snapshot paging with explicit coverage.
- [x] Opaque change cursors, current visibility epochs, tombstones and fixed expiry.
- [x] Separate lexical/vector signals with model/profile identity and truthful derived coverage.
- [x] Canonical exact/filter/literal availability independent of vector preparation.
- [x] Current and historical endpoint/evidence checks before disclosure, including read-time authority changes.
- [x] Full access/semantic compatibility, transport, index upgrade and required package checks.

Changes remain uncommitted in `g9000/mother-garden-queries`, carrying the verified
G5 source forward. Existing pairing edits in the primary checkout remain
untouched. G7 v1 compatibility/full backups and later live adoption remain
separate planned slices. This slice adds no new dependency or lockfile changes.
