# Temporal garden v2 contracts

The native redb Garden is the sole runtime. Graph operations are implemented and have independent current scopes/object policies. Permission-aware discovery reports defined contracts separately from operations authorized for the credential; a v1 context-only credential gains no graph authority. Graph-only and initialized v1 views can share one canonical database. See [native compatibility](garden-compatibility.md), [graph setup](garden-proposals.md) and [local service operation](garden-service.md).

The v1 catalogue in `contracts/mcp-tools.json` remains unchanged. `contracts/capabilities-tool.json` supplies additive capability discovery and `contracts/garden-tools-v2.json` defines the graph catalogue. Schemas resolve their definitions locally without fetching network content. The G3 validation section below preserves historical contracts-only milestone evidence; its inactive graph-discovery result does not describe the current native runtime.

## Model and evolution

Every graph request uses version 2, a workspace ID and restore generation.
Entities, assertions and evidence are separate kinds. Client drafts cannot set
canonical identity, revision, contributor, actor, approver, transaction time or
visibility. Update/lifecycle batch items identify their existing target and
expected revision separately from the draft. Creates have expected revision 0.
Each batch has an immutable operation UUID and unique client item keys; intra-batch
references identify matching creates with the required endpoint kind.

Entities have a namespaced type, display name and aliases. Assertions have an
entity subject, namespaced predicate and an exclusive entity reference or typed
value. Evidence carries a selected excerpt or source/artifact locator. A source
locator is attributed data; validating it does not fetch a URL or read a file.
Hashes and sizes for artifacts do not establish that the blob is available.

Unknown optional extensions are preserved as namespaced entries with a
`schemaVersion` and bounded JSON `payload`. Unsupported required extension
schemas are rejected explicitly. Extension payloads do not become protected core
fields. These contracts do not execute extension content.

For updates, `extensions` names only the namespaces being changed. G4 storage
merges those entries with the stored map and retains unmentioned namespaces;
an empty map must not erase unfamiliar extensions. This contract defines no
implicit namespace deletion. Actual merge/persistence behavior belongs to G4.

Known timestamps are explicit UTC instants. Validity is half-open, and an end
must be later than a known start. Unknown dates remain null with unknown
precision; source timezone and precision are metadata. Valid time, observed
time and store-issued transaction time remain separate. Revision metadata
preserves contributor, later actor and optional owner approver.

## Graph operations

| Tool | Contract purpose | Explicit scope |
| --- | --- | --- |
| `mother_garden_get` | One typed graph object/revision | `garden:read` |
| `mother_garden_query` | Bounded filters, candidate search and traversal | `garden:read` |
| `mother_garden_apply` | Atomic revision-checked graph batch | `garden:write`, non-shared client |
| `mother_garden_propose` | Reviewed batch without canonical mutation | `garden:propose` |
| `mother_garden_get_proposal` | Own proposal and reviewed outcome | `garden:propose` |
| `mother_garden_get_receipt` | Own immutable operation outcome | Original operation scope |
| `mother_garden_changes` | Authorized changes with opaque cursor | `garden:read` |

Responses carry coverage, canonical/index watermarks and visibility state where
applicable. Query/change pages carry opaque cursors and bounded snapshot-lease
metadata. The native runtime implements signed/opaque cursors, expiry, epoch invalidation and consistent paging; see [query semantics](garden-queries.md).
Client-provided cursor contents never grant authority. The structured
restart-required error tells a client when it must restart a traversal/page.

An approval receipt maps client item keys to canonical IDs/revisions and marks
owner edits/additions. A dismissal has no canonical mapping. Original proposals
and approved results stay distinguishable; current records must not be used to
reconstruct an older approval receipt.

Successful direct Apply returns `applied`; proposal admission returns `pending`.
Proposal status reads may report pending, approved, dismissed or blocked, while
operation-receipt reads retain the original operation's status. If an original
proposal is included, an approved receipt must map all its item keys. Every
visible payload must match the receipt's ID, kind, revision and commit sequence.
`payloadOmitted: true` permits partial or no payloads; false means every result's
payload is returned. This flag describes completeness, not permission. The
authority boundary must decide which payloads may be included.

## Graph authority boundary

Existing v1 `read`, `context:propose` and record-write scopes retain their meaning and grant no graph access. Graph policies and principals live in private redb tables under the same transaction authority as canonical graph/v1 data. Merely opening a native workspace neither creates grants nor changes its generation.

Owner provisioning uses `GardenStore::grant` for graph-only credentials or the explicit native v1 grant APIs with a separately selected graph policy. The CLI exposes `garden grant` and `garden grant-v1`; grant and approval operations are never MCP tools. Full graph access requires explicit companion selection; an omitted allowlist shares no objects. Shared clients cannot receive full graph or direct-write authority, and graph IDs do not inherit the v1 record allowlist.

Every returned object and referenced endpoint/evidence needs current visibility. Missing or malformed policies fail closed. Current native dispatch supports context-only, graph-only and combined grants according to the initialized view and explicit scopes; graph-only startup does not require a v1 read grant. [Permission-aware discovery](garden-compatibility.md) does not itself authorize a call.

Canonical reads, mutations and review recheck client, scope, subject, workspace, generation and stored references inside the transaction. Earlier authorization observations are not reusable capabilities. Requests cannot choose a different contributor or operation owner. Old SQLite grants/configurations are unsupported; there is no converter or implicit import.

Own proposal outcomes can disclose status and canonical result IDs while hiding
currently unreadable payloads. Approval never adds those IDs to an allowlist.
Direct-operation receipts require full current visibility. Revoked or restored
identities cannot use these helpers to replay uncertain effects.

## Historical G3 validation

The contract tests use an offline JSON Schema validator only as a development
dependency, plus typed semantic validation. JSON schemas describe structural
bounds; cross-field relationships, revision semantics, endpoint kinds and temporal
ordering require the typed validators. External object existence, permissions,
lifecycle-cycle checks and actual transaction idempotency require later storage.
This distinction is part of the contract, not a runtime acceptance claim.

The synthetic fixture in `tests/fixtures/garden-v2.json` includes a future optional
extension and intra-batch references. Tests cover malformed/protected fields,
required-version rejection, unknown extension round trips, temporal bounds,
exclusive values, batch/JSON limits, v1 strictness and separate graph permissions.
Executed on Windows, 15 November 2026, with synthetic records and owner-private
temporary directories. No installed client settings, model accounts or real
graph grants were changed.

| Check | Result |
| --- | --- |
| Graph contract tests | 18 passed, including offline validation of all seven standalone request/response schema pairs. |
| Graph access tests | Eight passed. |
| MCP suite | 18 passed; one existing optional real-model test ignored. |
| `cargo test --locked` | 311 passed, zero failures; two optional model tests ignored. |
| `cargo build --release --locked` | Passed. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed after style/layout and test-only clone fixes. |
| `git diff --check` | Passed. |

The full suite and release build passed on the final production source. A final
test-only change replaced three clones with borrowed slices; the eight access
tests, formatting and strict Clippy were repeated successfully afterward.
Specification and code review found no remaining blocking issues.

Tests first established the absent graph APIs and unknown discovery tool.
Additional regressions exposed and fixed Serde's tagged unit-variant extra-field
acceptance, explicit-null versus omitted-field mismatch, incomplete approval
mappings, tool-status ambiguity and mismatched immutable receipt payloads.
The live MCP tests prove discovery is bounded, persona-independent and rechecks
revocation; they do not claim actual graph retrieval or mutation.

The JSON Schema validator is a development-only dependency with default features
disabled and offline resolution enforced. Existing dependency versions and v1
tool/schema files were preserved. The private graph policy tables are additive
owner-provisioning state, not a new canonical graph database.

Changes remain uncommitted in the isolated `g9000/mother-garden-contracts`
worktree. Pairing and engine-qualification worktrees remain separate. Temporary
stores and task logs are removed after recording these results. The subsequent
[G4 library implementation](garden-temporal-storage.md) supplies canonical storage,
transaction-integrated authorization and extension merging. Default v1 connections
keep graph tools inactive; G5 adds an explicit graph-only prototype connection.

The current structural limits are 262,144 serialized JSON bytes, JSON nesting
depth 24, 64 batch items, 64 linked references and 32 extensions per draft.
Entity names and aliases are bounded to 512 Unicode scalar values and 32 aliases.
Known UTC instant strings are bounded to 40 characters. Batch identity/draft/
replacement fields that do not apply must be omitted rather than supplied as
null. Nullable temporal fields deliberately use null for unknown values.

Existing object references are returned separately by the batch validator for
future existence, kind and authorization checks. A structurally valid UUID is
not proof that its endpoint exists or is visible. Intra-batch references must
point to a create of the declared kind; graph lifecycle cycle detection remains
a storage responsibility.
