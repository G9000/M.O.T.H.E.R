# Shared Memory Garden Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan task by task. Use subagent-driven-development only when delegation is authorized. Checkboxes track implementation, not planning completion.

**Goal:** Turn M.O.T.H.E.R into an extensible shared context garden while preserving existing memories, grants, receipts and v1 clients.

**Architecture:** Harnesses select knowledge; M.O.T.H.E.R preserves it and provides generic retrieval. Deliver the existing-v1 AnimaOS additive integration first, then a versioned temporal graph backed by a qualified pure-Rust engine and explicit migration. No automatic extraction model belongs in the store.

**Tech Stack:** Rust, existing MCP interface, v1 SQLite compatibility, redb as the proposed new-runtime engine, versioned JSON contracts, local IPC and rebuildable indexes. Exact new dependency versions are pinned after qualification.

**Design:** [Shared memory garden](../specs/2026-11-14-shared-memory-garden.md).

**Companion plan:** [AnimaOS additive provider](C:/Users/leoca/OneDrive/Desktop/anima/animaos-kit/docs/superpowers/plans/2026-11-14-mother-additive-provider.md).

## Scope and starting state

Planning is complete only when both plans have been reviewed against the design. Implementation has not started. This work does not build the separate visual garden mockup, import real chats, alter live client settings or publish a release.

Before implementation, re-read `AGENTS.md`, inspect both Git trees and isolate changes. At plan authoring, unrelated edits existed in `docs/connect.md`, `src/animaos.rs`, `src/animaos/tests.rs` and `src/tui/animaos.rs`. Coordinate those exact files before editing; do not reset them. Preserve README title, poem and artwork.

Each task follows: add the listed behavioral regression, run it to establish the missing behavior, implement the smallest unit, rerun focused tests, inspect the diff. Commit coherent task changes only within the implementation session's authorization. Do not commit unrelated work.

## Releases and dependencies

| Release | Tasks | Deliverable |
| --- | --- | --- |
| A | G1 and AnimaOS A1–A5, A7 | Additive sharing/recall using current v1 records and owner review |
| B prototype | G2–G5 | Qualified engine, versioned contracts, transactional graph and review |
| B complete | G6–G7 | Indexed retrieval, compatibility, full portability and temporal acceptance |
| C | G8–G9 | Explicit migration, pure-Rust runtime verification, scale and recovery evidence |

G2 and G3 may be researched independently; G4 depends on both. AnimaOS A6 follows G3–G7 and is excluded from Release A. No live adoption of the new engine precedes G8/G9; Release A can ship independently after its own checks. If the engine gate fails, finish A and keep v1 usable while revising the engine decision.

## G1: Pair AnimaOS in additive mode

**Modify:** `src/animaos.rs`, `src/animaos/tests.rs`, `src/tui/animaos.rs`, `src/tui/animaos/tests.rs`, `docs/animaos.md`, `docs/client-access.md`, `docs/connect.md`.

**Depends on:** AnimaOS A1's published binding schema and conversion endpoint.

- [ ] Add synthetic pairing tests for additive mode, optional persona source, wrong agent, stale revision and unchanged existing grants.
- [ ] Run `cargo test --locked animaos` and confirm the new behavior is missing before implementation.
- [ ] Add an owner choice labelled shared garden/additive; select exact agent and reviewed MCP connection. Save the new AnimaOS provider binding, not the legacy primary-memory binding.
- [ ] Reuse existing companion/full-read or shared/allowlisted grant policies without widening them. Default new grants to `read,context:propose`; persona selection requires persona-visible access. Full garden access and an explicit allowlist remain distinct owner choices.
- [ ] Show existing primary bindings and offer explicit conversion. The AnimaOS conversion is atomic; if outcome is unknown, read saved state rather than create a second connection/key.
- [ ] Document that selecting the provider adds shared context and that proposals need owner review. Explain how to disconnect the provider separately from revoking its key.
- [ ] Run focused pairing tests, then the complete M.O.T.H.E.R checks below before integration.

**Acceptance:** A newly paired synthetic agent uses the additive binding, retains local memory, and has no broader grant than reviewed. The old pairing path remains readable and explicitly labelled legacy primary behavior.

## G2: Qualify the pure-Rust storage engine

**Create:** `experiments/garden-store/Cargo.toml`, `experiments/garden-store/src/main.rs`, `docs/benchmarks/2026-11-14-garden-engine.md`.

**Read:** `Cargo.toml`, `src/store.rs`, `src/portability.rs`, `src/search.rs`, `src/search/cache.rs`.

- [ ] Recheck official redb docs through Context7, pin a released version in the isolated experiment and record license/platform/MSRV/native dependency evidence.
- [ ] Implement a deterministic synthetic generator with entities, assertions, revisions, receipt IDs and time ranges. Keep it outside the real data directory.
- [ ] Define the needed indexes explicitly: ID, outgoing/incoming edge, predicate, valid time, transaction sequence, source, namespace and operation receipt.
- [ ] Measure the design's datasets and budgets, including atomic graph batches, read snapshots, writer contention and interrupted commits. Record durability settings, hardware and cold/warm conditions.
- [ ] Compare against the existing SQLite implementation where operations are equivalent; do not present unlike query semantics as a speed comparison.
- [ ] Record a go/no-go decision with failure evidence. The gate is behavioral correctness and acceptable measured performance, not a Rust-language label alone.

**Commands:** from `experiments/garden-store`, `cargo test --locked`, then `cargo run --release --locked -- --synthetic --records 100000 --edges-per-record 5 --seed 14`. Implement these exact CLI flags in the experiment. Capture outputs to the benchmark document, never real records.

**Acceptance:** A reproducible engine decision with no claim that the existing live store was upgraded. A failed gate blocks G4, not Release A.

## G3: Specify and validate v2 contracts

**Create:** `contracts/garden-tools-v2.json`, `src/garden/mod.rs`, `src/garden/contracts.rs`, `tests/garden_contracts.rs`, `tests/fixtures/garden-v2.json`.

**Modify:** `src/lib.rs`, `src/mcp.rs` for additive capability discovery only; retain v1 tool shapes.

- [ ] Write contract tests for entities, assertions, evidence, extension preservation, batch bounds and temporal interval validation.
- [ ] Define a versioned envelope with store-owned identity/revision fields and `extensions` keyed by namespace. Unknown namespaces round-trip; unsupported required schema versions fail explicitly.
- [ ] Define `object` as an exclusive entity-reference or typed-value variant. Reject contradictory representations, non-finite scores, overlarge JSON and invalid endpoints.
- [ ] Define get/query/apply/propose/proposal-status/receipt/changes request and response schemas from the design. Include per-item expected revisions, immutable operation IDs, coverage, cursors, watermarks and structured error codes.
- [ ] Extend owner grant validation with explicit `garden:read`, `garden:propose`, `garden:write` scopes and graph allowlists. Test old grants gain no new authority, shared clients cannot direct-write, hidden endpoints remain hidden and proposal outcome reads do not implicitly share newly approved content. Update `src/contracts.rs`, `src/access.rs` and `tests/access.rs` without changing existing v1 scope meanings.
- [ ] Define capabilities independently of persona content. Keep bounded bootstrap compatible with existing MCP envelopes.
- [ ] Add synthetic v1 fixtures that prove old drafts still reject unknown v2 fields rather than accepting a partial interpretation.
- [ ] Run `cargo test --locked --test garden_contracts` and `cargo test --locked --test mcp`.

**Acceptance:** A client can discover capabilities and preserve an unfamiliar extension without receiving authority to alter protected fields. Existing v1 schemas remain stable.

## G4: Implement transactional graph storage

**Create:** `src/garden/store.rs`, `src/garden/indexes.rs`, `src/garden/history.rs`, `tests/garden_store.rs`, `tests/garden_temporal.rs`.

**Modify:** `src/garden/mod.rs`, `Cargo.toml`, `Cargo.lock` only after G2 acceptance.

- [ ] Add tests for one atomic node/edge/evidence batch, response-loss retry, changed-payload UUID reuse and stale-revision conflict.
- [ ] Implement the qualified engine behind a garden-owned storage interface. Keep canonical records and their indexes in the same transaction; derived text/vector indexes are separate and rebuildable.
- [ ] Implement append-preserving assertion revisions with valid/observed/transaction time and exact `knownAt` reconstruction.
- [ ] Add explicit correction/supersession relationships. Reject missing endpoints, unauthorized targets and invalid cycles for lifecycle edges; ordinary graph relationships may be cyclic.
- [ ] Add grant-aware history and mutation checks before commit. No global multi-tenant schema is needed; existing workspace and client identities remain meaningful.
- [ ] Test two conflicting assertions, late-arriving evidence and a backdated correction. Confirm the old belief remains queryable at its transaction time.
- [ ] Run `cargo test --locked --test garden_store` and `cargo test --locked --test garden_temporal`.

**Acceptance:** Reopening the store reconstructs identical canonical graph/history; interrupted batches leave no partial relationships or missing receipt.

## G5: Add graph proposals and lifecycle rules

**Create:** `src/garden/inbox.rs`, `tests/garden_inbox.rs`.

**Modify:** `src/inbox.rs`, `src/tui/inbox.rs`, `src/tui/inbox/tests.rs`, `src/access.rs`, `src/mcp.rs` through backend abstractions rather than duplicating policy logic; `src/cli.rs`, `src/store.rs`, `src/animaos.rs` for explicit owner provisioning of graph scopes/allowlists.

- [ ] Test graph batch proposals remain absent from canonical queries and indexes.
- [ ] Implement owner approval of a bounded reviewed batch with one atomic receipt and proposal closure. Show each change, evidence, original proposer and expected revision.
- [ ] Test conflict, revoked proposer, changed generation, dismissal, owner edit and response-loss retry, including the graph-specific proposal-status and receipt read tools. Test approval receipt IDs are readable by the submitter without making unallowlisted canonical content readable.
- [ ] Define local-delete versus shared-retract behavior in the owner UI. No local event silently erases canonical evidence.
- [ ] Add explicit graph grant provisioning without upgrading existing keys. Test the design's operation/scope matrix, including an empty graph allowlist, separately authorized evidence and denied traversal expansion.
- [ ] Store stable itemKey-to-canonical-ID/revision mappings in approval receipts, including owner edits/additions. Test lost responses, dismissal and later canonical revisions cannot change the original operation outcome.
- [ ] Preserve original v1 inbox behavior and attribution. The v2 path must not make a new MCP approval tool available.
- [ ] Run `cargo test --locked --test garden_inbox`, `cargo test --locked --test inbox` and the inbox TUI tests.

**Acceptance:** Approval and closure cannot diverge after a crash; a pending graph is never recallable as established knowledge.

## G6: Build indexed queries and change tracking

**Create:** `src/garden/query.rs`, `src/garden/changes.rs`, `src/garden/search.rs`, `tests/garden_query.rs`, `tests/garden_changes.rs`.

**Modify:** `src/search.rs`, `src/search/cache.rs`, `src/embedding.rs` to expose versioned optional search capabilities without changing existing defaults unexpectedly.

- [ ] Test exact lookup, directed neighbors, predicate filters, bounded traversal and combined valid/known time.
- [ ] Implement index-backed candidates and stable paginated snapshot leases. Enforce request work/hop/edge budgets and report truncation.
- [ ] Add an opaque visibility-scoped change cursor and cache epoch. Test revocation while a page is being read, visibility shrink, restore, tombstones and expired cursors.
- [ ] Separate lexical/vector candidates from final harness ranking. Return model/profile and index watermark; never mix vectors from incompatible profiles.
- [ ] Make canonical changes visible through exact/filter paths even when derived indexing is behind. Revalidate candidate revision/status/access before return.
- [ ] Test hidden endpoints, sources and record counts cannot leak through graph expansion, embeddings or history. Missing model means explicit literal fallback.
- [ ] Run `cargo test --locked --test garden_query`, `cargo test --locked --test garden_changes`, `cargo test --locked --test access`, and existing semantic tests.

**Acceptance:** No whole-store scan is required for exact/adjacency/time indexed queries; stale derived results cannot resurrect retracted or unauthorized records.

## G7: Preserve v1 clients and scale backups

**Create:** `src/garden/compat.rs`, `src/garden/backup.rs`, `tests/garden_compat.rs`, `tests/garden_backup.rs`.

**Modify:** `src/store.rs`, `src/history.rs`, `src/portability.rs`, `src/limits.rs`, `src/mcp.rs`, `src/config.rs` to select a recognized backend explicitly.

- [ ] Add cross-backend v1 contract tests using synthetic copies of `tests/fixtures/v1.json` and `v1.sql`.
- [ ] Store existing v1 records and receipts losslessly in the new engine; serve old tool semantics through a compatibility view. Graph-only objects remain on v2 APIs.
- [ ] Build streaming snapshot export with manifest, chunk checksums, versioned required extensions and referenced blobs. Stage and verify a restore before exposing it as usable.
- [ ] Preserve current small v1 export behavior for compatible stores. If graph data cannot be represented, fail with a clear requirement to use the new full backup, never silently drop it.
- [ ] Remove small lifetime caps only on the qualified new backend. Preserve bounded inputs, disk-capacity failure handling, receipt uniqueness and explicit retained-history policy.
- [ ] Test partial/corrupt backups, incompatible schemas, unknown required extensions, missing blob bytes and exact extension round-trips. Public portable backups exclude grants and tokens.
- [ ] Run `cargo test --locked --test garden_compat`, `cargo test --locked --test garden_backup` and `cargo test --locked --test portability`.

**Acceptance:** The same v1 semantic suite passes against both backends, and a full backup can reconstruct all graph/revision state without credentials.

## G8: Add one-machine service ownership and explicit migration

**Create:** `src/service/mod.rs`, `src/service/local.rs`, `src/service/protocol.rs`, `src/garden/migrate.rs`, `tests/garden_service.rs`, `tests/garden_migration.rs`, `docs/garden-migration.md`.

**Modify:** `src/main.rs`, `src/cli.rs`, `src/mcp.rs`, `src/config.rs`, `src/connection.rs`, `src/portability.rs`, plus feature/build layout needed to isolate legacy SQLite conversion.

- [x] Test two MCP clients using one local writer service; reject a second owner process, unauthorized IPC client and wrong workspace/generation.
- [x] Implement bounded authenticated local IPC and a stdio MCP bridge. Keep operation semantics identical through direct/service paths. Do not expose a new public listener.
- [x] Implement dry-run migration to a new destination, validation manifest and explicit cutover journal. Keep runtime upgrades distinct from restore-style generation changes.
- [x] Exercise the design's stop-writers/final-snapshot/rebind sequence with synthetic clients. Require owner-managed restricted-grant reissuance; no tokens in exports/logs.
- [x] Block old-generation proposals, preserve historical receipts and disable use of the archived source as an active endpoint. Detect stale configs rather than silently choosing an old file.
- [x] Document rollback before and after first destination write. After-write rollback must reconcile new acknowledged state; reject a rollback that would discard it.
- [x] Audit the new runtime dependency graph for native database/index code. Put SQLite import support in a separate migration build; document OS library calls separately from the pure-Rust engine claim.
- [ ] Run `cargo test --locked --test garden_service` and `cargo test --locked --test garden_migration` on Windows, Linux and macOS. Windows passed; Linux/macOS runtime qualification remains in G9.

**Acceptance:** No dual authoritative stores, no lost acknowledged mutations across supported cutover/recovery cases, and unchanged behavior for an ordinary v1 runtime upgrade.

## G9: Validate, document and release deliberately

**Modify:** `docs/context.md`, `docs/design.md`, `docs/living-memory.md`, `docs/semantic-search.md`, `docs/harnesses.md`, `docs/setup.md`, `docs/validation.md`, `docs/CHANGELOG.md`.

**Create:** `docs/benchmarks/garden-acceptance.md`.

- [ ] Document the collector/keeper boundary and optional persona. Distinguish connection, proposal, approval, indexed availability and actual host recall.
- [ ] Run the complete acceptance matrix in the design, including two synthetic harnesses and an additive AnimaOS agent.
- [ ] Re-run G2 workloads on the completed implementation, including 1m-record stress and high-degree nodes. Record unmet budgets without claiming readiness.
- [ ] Run all four required checks from the repository root:

```powershell
cargo fmt --all -- --check
cargo test --locked
cargo build --release --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
```

- [ ] Validate separate new-runtime and legacy-migration feature builds, including the pure-Rust dependency audit. Run platform CI equivalents without unilaterally changing repository automation.
- [ ] Review rollback instructions, compatibility results and failure recovery before proposing release. Building binaries does not authorize installing them or migrating live records.

**Done:** A and B/C have separate release notes; tested capability and measured limits are stated accurately. Owner accepts a live cutover separately, after reviewing the concrete migration report.
