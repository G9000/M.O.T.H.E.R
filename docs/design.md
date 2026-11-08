# Independent personal workspace

M.O.T.H.E.R is one person's workspace and companion, independent of AnimaOS. The existing README, poem, and artwork stay unchanged. No user account system, hosted inference, web UI, or AnimaOS runtime dependency is needed.

## First usable release

A Bun/TypeScript program uses SQLite as the single canonical store, outside the checkout. Local MCP clients launch a stdio adapter with an explicitly selected client identity. Multiple adapters use the same file; SQLite transactions coordinate writes. A CLI initializes the workspace, grants/revokes client capabilities, inspects context, and exports/restores private state.

Identity lives in SOUL.md. Selected memories, decisions, tasks, artifacts, action outcomes, and handoffs live in typed records. Every record has an ID, revision, timestamps, status, attribution, and source references. Context retrieval is bounded and records omissions; direct retrieval resolves full records. No automatic history scraping or full transcript synchronization.

Mutations require the current workspace ID and restore generation, a unique operation ID, and expected revision (zero for create). A transaction validates the grant, compares revision, commits the record and its receipt together. Retrying the same operation returns the same receipt; changing the payload fails. Grants are checked on every call. Deletion is retraction, not a claim of erasure from backups.

Each local client is provisioned with a random bearer token stored only in a private config file. The database stores its hash and scopes. Tool arguments cannot choose actor, scopes, workspace, or storage paths. These controls separate client capabilities; they are not a security boundary against a process running as the same OS user with direct filesystem access.

Export captures records and mutation receipts in a consistent read transaction. Restore accepts only a fresh destination, preserves public workspace/record IDs and revisions, changes restore generation, validates references, and excludes credentials/grants. Restored receipts stay available as evidence; old operations cannot be replayed under the new generation. No schedules or external actions execute.

## Acceptance

Two independent MCP client processes read the same workspace. Client A creates a fact, task and checkpoint; B retrieves them, corrects the fact and advances the task; A sees current values after restart. Stale writes, changed-payload retries, revoked/read-only clients, invalid references, invalid imports and stale restore generations fail. Export/restore preserves records and action outcomes without restoring grants.

The first transport is local stdio. Remote ChatGPT connections require a separately tested authenticated HTTP deployment and are not claimed here. Real Codex/Claude/AnimaOS sessions require their own host setup and are distinguished from protocol-client tests. This release does not stop another harness's running jobs: handoffs require paused or settled work.

## Concrete v1 contract

The discriminated record schema in src/contracts.ts is normative. Memory carries an asserted basis (owner_statement, source, model_inference); this is attributed to the writing client and never grants owner authority. Decision states distinguish proposed/confirmed/superseded/retracted. Tasks use todo/doing/blocked/done/cancelled. Actions use pending/succeeded/failed/unknown. Checkpoints contain nextStep, completed steps, open questions, and references. Artifacts embed bounded plain text or Markdown; arbitrary binary files and external source bytes are not imported or fetched.

References can point to any existing record in this single workspace, including retracted records as historical evidence. They do not imply execution authority. Read requires read; each mutation requires its kind's capability (memory:write, decision:write, task:write, checkpoint:write, artifact:write, action:write). Credential identity determines the actor; no tool can choose it.

Exports include the initialized authoritative persona content and SHA-256 digest, records (including text artifact bytes), and historic mutation receipts. External HTTPS sources remain references and are explicitly listed as omitted content. Restore works without the original checkout/data path; executable code is installed separately. SOUL.md is the seed for a new workspace, not a silently changing identity for existing instances.

Context returns at most 100 record previews, explicit omitted count, per-record truncation, and a separately bounded list of active tasks/uncertain actions. Full record retrieval is required before edits. Revision-race and duplicate-operation-race tests use two actual server processes; exactly one conflicting edit may commit. Revocation is tested against an already connected client.

Recovery hardening: restore publishes schema and imported state in one transaction. Interrupted incomplete files fail to open. Mutations enforce portable capacity before committing, using transactional record/receipt counts and a conservative compact-export byte bound. Update timestamps never move behind the previous record timestamp when the system clock moves backward.
