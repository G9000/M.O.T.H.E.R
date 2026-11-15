# Shared memory garden design

**Date:** 14 November 2026, Asia/Kuala_Lumpur.

**Status:** Architecture agreed in conversation; implementation details below are proposed for review. This document authorizes no deployment, live import, grant change or service restart.

## Purpose

M.O.T.H.E.R is a shared context layer and memory keeper. ChatGPT, Claude, Codex, AnimaOS and other harnesses select meaningful knowledge from their conversations and contribute it to one garden. Each harness keeps its own intelligence and native memory capabilities. Connecting the garden adds continuity; it does not replace the harness's learning or personality.

The Garden of Recollection is a conceptual inspiration: independent collectors contribute meaningful recollections to a shared place of preservation. This is not a promise of exhaustive capture or perfect recall. The earlier [visual garden mockup](2026-11-12-memory-garden.md) remains a separate, unbuilt UI concept.

## Decisions and boundaries

| Concern | Owner |
| --- | --- |
| Extracting facts, judging significance, summarizing, interpreting and consolidating | The contributing harness |
| Local working context, agent-specific memory and final prompt assembly | The consuming harness |
| Shared records, evidence, relationships, temporal validity and revision history | M.O.T.H.E.R |
| Transactions, conflict detection, idempotency, access policy and owner review | M.O.T.H.E.R |
| Generic indexed search, time queries and bounded graph traversal | M.O.T.H.E.R |
| Choosing retrieval signals and final relevance ranking | The consuming harness, using the garden's candidate queries |
| Persona adoption | Optional, configured separately from additive memory |

No extraction model runs inside the garden. Optional local embeddings are a derived search index, never a source of truth. A default search remains useful to ordinary MCP clients; advanced clients can request candidate lanes and combine scores themselves. Replacing the model or ranking profile must not rewrite canonical memories.

One machine is the deployment boundary. Multiple client processes and different permissions still exist. Do not add tenant billing, distributed replication, consensus, remote worker coordination or team administration. Do not remove current per-client grants. Keep personal context isolated from separate team workspaces.

## Current implementation and gaps

M.O.T.H.E.R paths below are repository-relative. AnimaOS paths refer to `C:/Users/leoca/OneDrive/Desktop/anima/animaos-kit`.

| Evidence | Current behavior | Required change |
| --- | --- | --- |
| `src/contracts.rs`, `contracts/schema-v1.sql` | Six fixed kinds; strict fields; plain record references | Versioned extensible graph contract while preserving v1 behavior |
| `src/store.rs`, `src/inbox.rs` | Atomic revision checks, operation receipts and reviewed capture | Reuse these invariants for graph batches and proposals |
| `src/history.rs` | History reconstructed from receipts | Indexed revision queries, including graph assertion history |
| `src/search.rs`, `src/embedding.rs` | Fixed embedding profile, snapshot scan, built-in hybrid ordering | Capability-based retrieval with bounded index work and explicit freshness |
| `src/limits.rs`, `src/portability.rs` | 10,000 records, 100,000 operations and 64 MiB portable capacity | Decouple lifetime capacity from one JSON export; streaming backups |
| AnimaOS [external_provider.rs](C:/Users/leoca/OneDrive/Desktop/anima/animaos-kit/packages/core-rust/crates/anima-memory/src/external_provider.rs) | Additive recall, turn and write hooks | Implement a M.O.T.H.E.R adapter; retain local memory |
| AnimaOS `hosts/rust-daemon/src/memory_provider.rs` | Only `none` registered; process-wide, lossy event queue | Agent-bound provider instances and reliable selected publication |
| AnimaOS `hosts/rust-daemon/src/workspace_context.rs` | Companion mode couples persona with primary memory | Explicit additive binding independent of persona |
| AnimaOS `hosts/rust-daemon/src/agent_runs.rs` | Companion skips local capture, turn mirroring and learning paths | Additive mode retains native paths without enabling indiscriminate external capture |

AnimaOS also has richer temporal and relationship types in `anima-memory`, plus a separate durable `MemoryPort`. These are reference implementations, not interchangeable guarantees. Do not move the whole AnimaOS crate into M.O.T.H.E.R or present its legacy in-memory manager as a durable transaction writer.

## Delivery sequence

1. **A: Additive integration on existing v1 storage.** Ship useful sharing and recall without requiring a database replacement or graph schema. Includes reliable publication, review outcomes, explicit mode conversion and independent persona selection.
2. **B: Extensible garden.** Introduce entity, assertion, evidence and temporal graph APIs, compatibility views, indexed retrieval and scalable backup.
3. **C: Pure Rust runtime and measured scale.** Complete the storage-engine qualification and explicit migration/cutover before claiming the new runtime is pure Rust or production-ready at larger scales.

B's storage prototype begins early, but A must not wait for B or C. A uses only existing v1 record/proposal tools. Graph capability negotiation upgrades the adapter later.

Execution plans: [M.O.T.H.E.R](../plans/2026-11-14-shared-memory-garden.md) and [AnimaOS](C:/Users/leoca/OneDrive/Desktop/anima/animaos-kit/docs/superpowers/plans/2026-11-14-mother-additive-provider.md).

## Shared data model

Keep the advertised v1 schemas and SQLite file format intact. New graph tools use an explicit versioned envelope rather than adding unknown keys to v1 drafts.

Proposed v2 core:

- **Entity:** stable UUID, namespaced type, display name and aliases. People, projects, places and documents are data types rather than hardcoded product roles. Entity merging is an explicit attributed operation, never an automatic similarity side effect.
- **Assertion:** an independently versioned statement or edge: subject, predicate, object entity or typed value, validity interval, evidence references and author-supplied confidence/basis. Two conflicting assertions can coexist with distinct provenance.
- **Evidence:** selected source excerpt or artifact reference with source locator, contributor identity and observation time. Imported text is attributed data. A URL is not a stored copy of its bytes.
- **Revision:** store-issued commit sequence/time, actor, previous revision and lifecycle operation. Separate original contributor, later editor and owner approver.
- **Extension:** namespaced JSON payload with explicit schema version and bounded size/depth. Unknown extensions round-trip; patches affect only specified namespaces. Extension data cannot override identity, permissions, lifecycle or temporal fields.
- **Profile:** persona remains an optional owner-managed document. Merely reading garden records never installs it as a harness personality.

An assertion carries valid time (when its statement applies in the world), observed time (when its source observed it), and transaction time (when the garden recorded this version). Unknown dates remain unknown. Use half-open validity intervals and explicit UTC instants; preserve source timezone/precision when available. Revision history must answer both `validAt` and `knownAt` queries without reconstructing past knowledge from today's status.

Example: in April a harness learns that a move happened in March. A March-validity query may find that assertion today; a query of what the garden knew in March must not. A later correction closes the transaction-time version and preserves the original evidence.

Keep structured records small. Larger attachments use content-addressed blobs with size/hash metadata and explicit backup inclusion. Garbage collection respects retained revisions and evidence references. No automatic transcript upload, inferred edge creation or arbitrary filesystem attachment reads.

## Query and mutation contracts

Proposed new capabilities are `garden.entities.v2`, `garden.assertions.v2`, `garden.query.v2`, `garden.changes.v2` and `garden.proposals.v2`. Tool names are `mother_garden_get`, `mother_garden_query`, `mother_garden_apply`, `mother_garden_propose`, `mother_garden_get_proposal`, `mother_garden_get_receipt` and `mother_garden_changes`. Publish JSON schemas in a separate `contracts/garden-tools-v2.json`; retain `contracts/mcp-tools.json` v1 shapes. Advertise discovery through an additive capability tool.

Graph authorization uses new explicit scopes: `garden:read` for get/query/changes, `garden:propose` for reviewed batches and `garden:write` for direct batches. Existing `read`, `context:propose` and kind-specific write grants retain their v1 meanings and do not automatically authorize graph data. Owner setup can grant v1 and graph scopes together, but existing credentials are not widened by an upgrade. Shared-access clients may receive graph read/propose scopes only, with explicit graph-object allowlists; they never receive graph direct-write authority. Referenced endpoints/evidence require their own visibility. An approved new object does not enter an allowlist automatically. Companion/legacy full-workspace visibility still requires the new scopes before accessing graph records.

| Operation | Scope | Additional authority |
| --- | --- | --- |
| Get/query/changes, including graph history | `garden:read` | Every returned entity/assertion/evidence object visible under current policy |
| Apply entity/assertion/evidence batch | `garden:write` | Non-shared client; current revisions and authorized referenced targets |
| Propose entity/assertion/evidence batch | `garden:propose` | Current target visibility; no canonical mutation |
| Read own proposal/outcome | `garden:propose` | Authenticated proposer; details filtered by current visibility |
| Read own operation receipt | Scope of the original operation | Exact actor/workspace/generation/UUID; current payload visibility |
| Approve/dismiss, grant/change allowlist | Local owner only | Never an MCP permission |

`mother_garden_get_proposal` reads the caller's own submitted batch state and approval/dismissal receipt under its current propose scope; other clients require owner-authorized visibility. `mother_garden_get_receipt` resolves the caller's operation UUID within its workspace/generation and rechecks its current relevant scope. Both return only currently authorized payload details: an approval may expose the submitter's receipt/status and canonical result IDs without granting permission to read those new records. Revoked clients are denied. Receipts retain the original generation for audit; old-generation operations cannot be replayed into a restored workspace. These endpoints support response-loss recovery without listing unrelated proposal or operation content.

Each batch input has a unique client `itemKey`. The immutable approval receipt maps every itemKey to the resulting canonical ID/revision and records whether the owner edited that item; canonical payloads remain separately visibility-checked. Owner-added items get distinct keys and are marked as owner additions. A dismissal has no canonical mapping. The original proposal remains available as attributed history so a client can distinguish its suggestion from what the owner actually approved. A duplicate retry resolves the same receipt rather than deriving a new mapping from the latest record contents.

Queries support exact IDs, entity/predicate/property filters, valid/known time, text candidates, optional vector candidates and bounded traversal. Results include source IDs, revisions, component scores, coverage and index watermark. Apply access filters before candidate ranking and traversal expansion; hidden nodes, edges, counts and source snippets must not leak. Reference visibility is not inherited from a visible parent.

All list/search surfaces require limits and stable cursors. Cursors bind workspace, generation, query and visibility epoch; expired or invalid cursors return a restart-required response. A bounded snapshot lease prevents inconsistent pages; releasing the lease permits index cleanup. Change tokens are opaque and reveal only authorized changes. Revocation invalidates the client's cache epoch even if it no longer has permission to read a tombstone. Consumers clear cached shared context on epoch change or denied revalidation.

Graph writes atomically apply a bounded batch with expected revisions, operation UUID and receipt. Node, edge, history and change-log indexes commit together. Edge endpoints and evidence must exist and be authorized, or be created in the same batch. Deletes cannot leave silently valid dangling edges. Retract/supersede retain history. Explicit erasure is a distinct owner operation with documented backup implications, not the default interpretation of a local memory deletion.

Review-only clients propose a bounded graph batch. Approval atomically commits the exact reviewed batch and closes the proposal. Revocation, changed target revisions or restored generation block approval. No MCP client can approve its own proposal. v1 proposals continue unchanged.

## Storage and Rust decision

The target is a Rust memory service and pure-Rust database engine. Today `rusqlite` bundles SQLite, so the existing runtime does not satisfy that strict target. Do not describe a Rust wrapper around SQLite as a pure-Rust engine.

**Preferred candidate: redb**, qualified with synthetic workloads before adoption. Own the garden's data model, adjacency/time indexes and wire contract; use an established engine for transactions and crash recovery. A dedicated local owner process keeps the writer and derived search resources shared. MCP stdio bridges connect through authenticated local IPC (Windows named pipe with owner ACL; Unix socket with private permissions). Cloud harness access continues through the existing explicitly configured gateway/tunnel, not an exposed database.

The qualification report must pin a released engine version, inspect its dependency/native-code footprint, measure durability settings and test supported platforms. If the gate fails, keep the existing v1 release operational and revise the engine choice; do not silently relax the pure-Rust target. Do not implement a new database engine.

New storage contains v1-compatible canonical records as well as graph data in one transaction authority. Do not dual-write two independent canonical databases. Separate the SQLite compatibility importer from the new runtime build so legacy conversion dependencies do not invalidate the pure-Rust runtime claim. Existing v1 clients receive the same validated v1 records; graph-only objects are not squeezed into misleading v1 records.

## Additive AnimaOS behavior

Each agent selects at most one external provider. Keep the existing process-wide `none` default compatible, but bind M.O.T.H.E.R instances by agent ID, connection ID, workspace ID, generation and current grant. Never share one agent's authenticated handle with another.

An additive binding keeps native memory tools, local recall, task-result capture and learning active. Persona source is independently `local` or an explicitly reviewed workspace persona. The old `companion` binding retains its behavior until the owner performs a revision-checked conversion; no upgrade silently enables capture or changes identity. The UI defaults new garden connections to additive mode and explains existing primary-memory bindings.

The default publication policy is **selected candidates only**. `sync_turn` for the M.O.T.H.E.R adapter does not send transcripts. Ordinary `on_memory_write` notifications do not publish every local row. An explicit agent tool or the existing local review flow creates a selected share candidate with provenance. Proposal-only grants use the inbox; existing explicitly granted direct writes remain available. Model-generated confidence is labeled as such.

Recall queries the garden under current permissions alongside native recall, then combines bounded, attributed results. Avoid duplicate prompt entries using canonical origin IDs first, normalized text second. Do not automatically import every garden hit as a permanent local memory. A local item linked to a garden revision is a projection/reference, not independent evidence. On conflict, show source/date and fetch the current shared record; do not choose truth merely by newest timestamp.

The publication outbox is durable, agent-bound and separate from the existing lossy enrichment queue. Store the selected candidate and its outbox admission atomically in the source transaction, or make the outbox the durable source of an explicit share action before claiming it was queued. Local memory saving remains available if sharing is unavailable; report the two outcomes separately.

States: `queued`, `sending`, `unknown`, `submitted_for_review`, `saved`, `conflict`, `blocked`, `dismissed`, `cancelled`. A timeout becomes `unknown`; reconcile receipt or retry the identical payload/UUID. Never create a new UUID for an uncertain attempt. Stop on revocation, generation change or changed binding. Proposal submission is not canonical saving; reconciliation observes approval/dismissal and maps approved record IDs/revisions. Local deletion cancels an unsent candidate or proposes an explicit shared correction when requested; it never automatically erases shared history.

Store mapping keys `(workspace, generation, agent, local_id, publication_id)` plus source revision/digest, proposal/operation IDs, garden record ID and last observed garden revision. This prevents feedback loops when recalled content is later evaluated locally. Publication uses current grants and tool approval policy even when initiated by background work; a queued action is not permission to bypass a later Deny.

## Migration and rollback

Release A requires no store migration. Updating software preserves all existing agents, chats, grants and v1 databases. Converting a particular Companion binding is an explicit owner setting operation, with a preview of local-memory and persona behavior. Reversing it does not erase newly created local memories or garden records.

For the new engine, build a separate destination from a consistent private snapshot. Verify every canonical ID, full record, revision/receipt, source, proposal state, pin and persona hash; preserve extension tables that are supported, reject unknown required extensions. The old SQLite database is never rewritten or deleted by migration.

Cutover requires stopping writers, taking a final verified snapshot, validating the destination, rebinding authorized clients and starting one authoritative service. Treat destination activation as an explicit restore-style generation change. Ordinary portable exports exclude grants; a local cutover preserves grant restrictions through owner-controlled reissuance, never by exposing tokens in a bundle. Pending proposals remain blocked against their old generation and require fresh reconciliation. Retain old operation receipts with original generations for audit.

Before any new write, rollback can restore the old endpoint/configuration. After new writes, rollback requires a verified reverse export/reconciliation; never restart an old snapshot and silently lose acknowledged writes. Keep pre-cutover data read-only and labelled as an archive so old clients cannot become a second writer authority. Live cutover is a separate owner-authorized operation, never a test.

## Acceptance and performance gates

- A synthetic memory selected in harness A becomes retrievable in harness B after approval, with the same canonical ID and provenance. Unselected conversation noise never enters the garden.
- AnimaOS additive mode still recalls and updates local memory while the garden is online, offline or revoked. No other agent gains access.
- Response-loss, restart and duplicate-delivery tests produce one committed operation or one review proposal. Conflicts are visible and pending-review items never appear in canonical recall.
- Temporal queries distinguish valid time from knowledge time. Contradictory assertions retain separate evidence. Unknown extension payloads survive unrelated edits, export and restore.
- Revocation, corrections and retractions invalidate derived caches before those items are injected again; pending indexes cannot resurrect inactive records.
- Kill/restart tests exercise transaction boundaries, outbox admission, batch approval, migration and backup restore. Use synthetic records and owned temporary directories only.
- Benchmark 10k/100k records with 5x relationships; exercise a 1m-record stress dataset separately. Report p50/p95/p99, cold/warm reads, durable write latency, RAM, disk, traversal fanout and concurrent readers/writers. Provisional warm p95 budgets on a recorded reference SSD machine: exact lookup 10 ms, bounded temporal/neighborhood query 50 ms, 32-assertion durable batch 100 ms, lexical candidates 100 ms. Embedding latency is measured separately. These are proposed acceptance targets, not measured results.
- Never solve a missed latency target by disabling durability or access checks. Bound traversal hops, returned edges and work budget; return explicit truncation.

## References

- [Hermes external providers](https://github.com/NousResearch/hermes-agent/blob/main/website/docs/user-guide/features/memory-providers.md): additive integration pattern; it does not require copying Hermes's provider-side extraction.
- [redb](https://docs.rs/redb/latest/redb/): candidate embedded engine; pin and revalidate a specific release during qualification.
- [Garden of Recollection](https://honkai-star-rail.fandom.com/wiki/Garden_of_Recollection): conceptual inspiration, not a technical or governance specification.
