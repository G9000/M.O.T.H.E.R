# Temporal garden v2 contracts

This describes the G3 contract boundary. The subsequent
[G4 storage library](garden-temporal-storage.md) implements the canonical graph
without enabling graph MCP dispatch or migration. The subsequent
[G5 graph-only prototype](garden-proposals.md) adds an explicit opt-in transport
and owner review. The default v1 connection still exposes v1 context and additive
`mother_get_capabilities` discovery. That v1 discovery explicitly
returns `garden.available: false`, empty available capabilities/tools, and
`status: "contracts_only"`. Defined future tools are documentation, not authority.

The v1 catalogue in `contracts/mcp-tools.json` stays unchanged. The separate
`contracts/capabilities-tool.json` is appended to live discovery. The future
catalogue is `contracts/garden-tools-v2.json`; each input/output schema resolves
its definitions locally, without fetching schemas over the network.

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

## Future operations

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
metadata. These are wire shapes: signed/opaque token issuance, expiry, epoch
invalidation and consistent paging still require runtime implementation.
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

Existing v1 `read`, `context:propose` and record-write scopes retain their meaning.
They grant no graph access. The v1 `SCOPES` constant is unchanged; new graph
scopes are explicit. Graph policies use separate private tables and anchors,
created only by the owner provisioning API. Opening an existing workspace does
not create graph grants, change generation or change SQLite's schema version.

`Store::grant_with_garden_access` is the explicit owner library API. It declares
either full graph access (companion/legacy only) or a typed graph-object allowlist.
The default allowlist is empty. Shared clients cannot receive full graph access
or direct-write authority. Their graph IDs do not inherit the v1 record allowlist.
Every returned object, referenced endpoint and evidence object needs its own
current visibility. Missing or malformed graph policies fail closed.

Graph IDs can be syntax-checked at provisioning time, but without graph storage
this is only an owner declaration, not an existence check. G3 added no graph CLI
or MCP grant/approval tool. G5 supplies a separate owner CLI and graph-only
connection; owner grants and approvals remain unavailable through MCP.
The current v1 MCP bridge still requires its normal
v1 read grant for startup; graph scopes are independent permissions, not a
replacement for that existing startup requirement.

The independent G4 graph store uses its own explicit `GardenStore::grant` API.
Its principals and canonical graph share one redb transaction authority. The
SQLite grant helpers described here remain contract scaffolding; they cannot
authorize a redb write or import a live credential into the new graph store.

The access helpers check current scopes, client/subject, workspace and generation
transactionally against synthetic stored grants. They return authorization
snapshots, not reusable capabilities. Future graph handlers must recheck inside
their canonical read/write transaction using references from trusted stored
objects. Requests cannot supply a different contributor or operation owner.

Own proposal outcomes can disclose status and canonical result IDs while hiding
currently unreadable payloads. Approval never adds those IDs to an allowlist.
Direct-operation receipts require full current visibility. Revoked or restored
identities cannot use these helpers to replay uncertain effects.

## Validation

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
