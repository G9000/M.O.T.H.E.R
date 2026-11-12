# Independent personal workspace

M.O.T.H.E.R is one person's workspace and companion, independent of AnimaOS. The existing README, poem, and artwork stay unchanged. No user account system, hosted inference, web UI, or AnimaOS runtime dependency is needed.

## First usable release

A standalone Rust program uses SQLite as the single canonical store, outside the checkout. Local MCP clients launch a stdio adapter with an explicitly selected client identity. Multiple adapters use the same file; SQLite transactions coordinate writes. A CLI initializes the workspace, grants/revokes client capabilities, inspects context, and exports/restores private state.

Each owner supplies identity through their own SOUL.md at initialization. Selected memories, decisions, tasks, artifacts, action outcomes, and handoffs live in typed records. Every record has an ID, revision, timestamps, status, attribution, and source references. Context retrieval is bounded and records omissions; direct retrieval resolves full records. No automatic history scraping or full transcript synchronization.

Mutations require the current workspace ID and restore generation, a unique operation ID, and expected revision (zero for create). A transaction validates the grant, compares revision, commits the record and its receipt together. Retrying the same operation returns the same receipt; changing the payload fails. Grants are checked on every call. Deletion is retraction, not a claim of erasure from backups.

Each local client is provisioned with a random bearer token stored only in a private config file. The database stores its hash and scopes. Tool arguments cannot choose actor, scopes, workspace, or storage paths. These controls separate client capabilities; they are not a security boundary against a process running as the same OS user with direct filesystem access.

Export captures records and mutation receipts in a consistent read transaction. Restore accepts only a fresh destination, preserves public workspace/record IDs and revisions, changes restore generation, validates references, and excludes credentials/grants. Restored receipts stay available as evidence; old operations cannot be replayed under the new generation. No schedules or external actions execute.

## Acceptance

Two independent MCP client processes read the same workspace. Client A creates a fact, task and checkpoint; B retrieves them, corrects the fact and advances the task; A sees current values after restart. Stale writes, changed-payload retries, revoked/read-only clients, invalid references, invalid imports and stale restore generations fail. Export/restore preserves records and action outcomes without restoring grants.

The first transport is local stdio. Remote ChatGPT connections require a separately tested authenticated HTTP deployment and are not claimed here. Real Codex/Claude/AnimaOS sessions require their own host setup and are distinguished from protocol-client tests. This release does not stop another harness's running jobs: handoffs require paused or settled work.

## Concrete v1 contract

The discriminated record schema in src/contracts.rs is normative. Memory carries an asserted basis (owner_statement, source, model_inference); this is attributed to the writing client and never grants owner authority. Decision states distinguish proposed/confirmed/superseded/retracted. Tasks use todo/doing/blocked/done/cancelled. Actions use pending/succeeded/failed/unknown. Checkpoints contain nextStep, completed steps, open questions, and references. Artifacts embed bounded plain text or Markdown; arbitrary binary files and external source bytes are not imported or fetched.

References can point to any existing record in this single workspace, including retracted records as historical evidence. They do not imply execution authority. Read requires read; each mutation requires its kind's capability (memory:write, decision:write, task:write, checkpoint:write, artifact:write, action:write). Credential identity determines the actor; no tool can choose it.

Exports include the initialized authoritative persona content and SHA-256 digest, records (including text artifact bytes), and historic mutation receipts. External HTTPS sources remain references and are explicitly listed as omitted content. Restore works without the original checkout/data path; executable code is installed separately. Initialization loads the owner-selected SOUL.md or the file beside the database; no personal seed is compiled into the program. The stored snapshot is authoritative for existing workspaces and portable across harnesses.

Context returns at most 100 record previews, explicit omitted count, per-record truncation, and a separately bounded list of active tasks/uncertain actions. Full record retrieval is required before edits. Revision-race and duplicate-operation-race tests use two actual server processes; exactly one conflicting edit may commit. Revocation is tested against an already connected client.

Recovery hardening: restore publishes schema and imported state in one transaction. Interrupted incomplete files fail to open. Mutations enforce portable capacity before committing, using transactional record/receipt counts and a conservative compact-export byte bound. Update timestamps never move behind the previous record timestamp when the system clock moves backward.

Unfiltered context previews are balanced across record categories, so frequent checkpoints cannot displace older memories and decisions within the preview limit; per-category returned and omitted counts make the split explicit. Filtered requests return their newest matches. See [shared context](context.md).

A checkpoint write can name the earlier active checkpoint it replaces. The store retires that record inside the same IMMEDIATE transaction as an ordinary revision by the same actor, with its own server-generated operation ID and receipt, after checking its kind, status and expected revision. The request fingerprint covers the named target, so a retry returns the original receipt without retiring anything twice and a changed target is rejected as a reused operation. Stored record and receipt shapes are unchanged.

Record history is a read-only projection of retained mutation receipts, each of which already holds the complete record it produced. It needs the read scope and full-context access, and adds no table, column or write path.

Context and search accept an optional half-open window on each record's last update time. Bounds are normalized to the stored UTC millisecond form, so they compare as text against the existing `updated_at` column and index; no schema change is needed.

Pinned core records live in an additive `pins` table keyed by record ID, created on first use and ignored by older executables. Every context read returns the visible, still-active pinned records in full up to a per-record bound, outside the preview limit; the workspace holds at most twelve pins. Pinning requires the record kind's write scope and is idempotent. Exports carry pins as an optional extension validated against the bundle's records.

Addressed handoffs name a receiving harness by its client ID in an additive `assignments` table keyed by task ID. Assigning requires `task:write`, an open task and a current receiving grant that can read it. Context returns the caller's open assigned tasks outside the preview limit, plus the active client names for non-shared clients. M.O.T.H.E.R still runs nothing: the receiving harness acts under its own permissions and reports through ordinary task revisions.

## Owner-triggered launches (decision, 11 November 2026)

The first release ran nothing: no agent loop, no tool execution. That still holds for the MCP server and every client. As a deliberate change, the owner CLI can now start a harness through `mother launch`, using only launchers the owner declared in a private file: an absolute command, fixed arguments with `{prompt}` and `{task}` placeholders, an optional working folder and a timeout, run without a shell. The prompt names the task by ID only. Each launch hands the task to the launcher's harness and is recorded as an `action` by `@owner-launch`, pending before the command starts and settled after it ends; a timeout is recorded as `unknown`, and an unresolved launch blocks another launch of the same task until the owner retries explicitly. The launched harness acts under its own grant and permissions.

M.O.T.H.E.R choosing launches herself (proposals for approval, then launches within owner-set budgets) is a later stage and requires its own decision. See [the hearth](hearth.md).

The owner has the same pin, handoff, history and record-listing operations without a grant, through the CLI and the terminal's tend screen. They share the harness code paths and rules, record `@owner` as the actor, and are not reachable from MCP.

## Local retrieval layer (v0.3.0)

The existing search tool and owner CLI share local Candle inference and a disposable SQLite vector sidecar. Canonical SQLite records remain authoritative. Cache keys bind workspace/generation, model/profile, record revision and text digest; search rechecks grants and current canonical state before returning. Hybrid retrieval preserves exact identifiers and adds meaning-based candidates. Missing or busy infrastructure produces explicit literal fallback.

This extends the context layer only: no agent loop, tool execution, transcript ingestion, automatic memory extraction or third-party memory service. Each harness still decides what to retrieve and save. See [semantic search](semantic-search.md) for model installation, coverage limits and acceptance fixtures.

## Owner persona updates (v0.3.2)

The owner CLI can inspect the stored persona and explicitly import a revised SOUL.md using the current content hash. Store::update_persona validates the draft and performs its hash comparison and metadata replacement inside one IMMEDIATE SQLite transaction. It does not alter workspace identity, grants, record revisions or receipts, and is not exposed as an MCP write tool. Context and export operations read the current stored snapshot; search already includes it in final freshness validation. Persona history and automatic file watching are outside this scope.
