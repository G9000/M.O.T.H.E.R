# Living Memory

Keep what matters from a conversation, review it once, then continue from another harness. M.O.T.H.E.R stays an independent local context layer; it does not need AnimaOS or another reasoning model.

## Capture, review, continue

In a connected chat, ask:

> Keep what matters from this conversation. Search M.O.T.H.E.R for existing records first. Propose a small set of useful memories or corrections with their source and certainty for my review inbox. Do not save them directly as approved memories.

The active agent calls `mother_propose_memory` once per candidate. It can propose a memory, decision or task. It should resolve full existing records before proposing corrections, and keep tentative plans distinct from confirmed decisions. This is an explicit capture workflow; MCP does not automatically receive every conversation or monitor your chats.

The harness chooses and interprets the candidate; M.O.T.H.E.R keeps its provenance and review outcome. Selection, proposal admission, canonical approval, semantic indexing and actual recall in another host are separate steps. Approval makes a record eligible for authorized retrieval; it does not guarantee vectors or prompt inclusion. Each harness retains its own native memory, and sharing does not require adopting the workspace persona.

Claude Code also supports [opt-in capture and resumable backfill](claude-code-capture.md) from one owner-selected local transcript directory. It submits to the same inbox.

For a private, dated letter written from the witnessing agent's perspective, use the [Witness Journal skill](witness-journal.md). Drafts enter the inbox only when you ask to submit them.

Use [Memory Gardening](memory-gardening.md) to find records worth revisiting and submit linked annotations for the same owner review.

Stop the native owner service before opening its database with an owner command. Run `mother garden v1-inbox --db <garden.redb>` for v1 proposals or `mother garden inbox --db <garden.redb>` for graph batches. When input and output are terminals these commands open native owner review; piped invocations list proposals as JSON. A separately reviewed request can use `mother garden review --db ... --request ...` (add `--graph` for graph proposals). Restart the service after review. IPC exposes no approval operation. The former setup dashboard and generic `mother inbox` command are removed. See [native review](garden-compatibility.md) and [service operation](garden-service.md).

For interactive v1 review:

1. Select a proposal with the arrows and press **Enter**.
2. Read its source, proposing harness, attribution, before snapshot and proposed changes. Use arrows or Page Up / Page Down to scroll the full record, including source links and references.
3. Press **s** for **Save**, **e** for **Edit**, or **d** for **Dismiss**. Save and Dismiss each require a final Enter confirmation. Escape returns without applying the action.

In Edit, Tab or Shift+Tab selects the title, content, status or memory basis. Arrow keys move the text cursor; Left/Right changes a status or basis choice. Enter inserts a new line in content. Ctrl+U clears the selected text field. Ctrl+S returns the edited candidate to review; it does not publish it. Escape discards the current editor changes. Sources and references are displayed in review and retained unchanged by the editor; a different source or target requires a fresh proposal.

Save creates or updates the canonical record. Other connected agents can then retrieve it through the existing context/search tools. Dismiss closes the proposal without changing active context. Both outcomes retain an audit record. The approved record's actor is `@owner-review`; the proposal preserves the original harness, source, basis and candidate, plus the approval receipt and final record. Owner edits do not silently replace the original proposal.

## Corrections and conflicts

A correction keeps the original record ID and captures its current revision. If another harness changes it before you Save, M.O.T.H.E.R rejects the stale correction. Refresh shows the current record alongside the captured version without silently rebasing your proposal or replacing your edits. Ask the agent to read the latest record and propose a reconciled correction; dismiss the stale item when ready.

Identical pending proposals from the same harness are combined only when their complete draft, source, attribution and target revision match. Every submission keeps its own operation receipt. Different status, certainty, sources or proposers remain distinct. Approval checks exact canonical duplicates again within the transaction and blocks duplicate creation. Similar wording is a review responsibility, not an automatic merge.

An uncertain storage result keeps the exact operation ID and request in the confirmation screen. Enter explicitly retries that same operation, returning its existing receipt if it already committed. The TUI does not automatically repeat a write. After restarting, refresh the inbox: atomic approval means a saved item is already closed and its canonical record is available.

## Connect and upgrade

Build with `cargo build --release --locked`. All builds use native redb storage; the SQLite runtime and converter are removed. A build does not install the executable, restart the owner or change host settings. Existing native databases retain their identity, generation and credentials; old SQLite configs fail explicitly. Reconnect each intended harness to discover its authorized native tools. Follow [service setup](garden-service.md) and [historical cutover recovery](garden-migration.md); neither a build nor a reconnect activates a prepared destination.

A client with the matching existing `memory:write`, `decision:write` or `task:write` scope can propose that kind. A read-only client cannot propose. For a new client that may submit all three kinds for review but may not write canonical records directly, create a separate grant:

```powershell
mother garden grant-v1 reviewed-agent --db "C:\private-data\mother\garden.redb" --out "C:\private-data\mother\reviewed-agent.native.json" --access companion --subject reviewed-agent --scopes read,context:propose
```

Stop the owner service before provisioning. Restart it afterward, bind the new direct credential with `garden service-config`, then use that private service config in the intended harness. This command does not modify an existing grant. Existing write-capable grants still permit their direct-write tools for compatibility; the review-first workflow must use `mother_propose_memory`. No MCP tool can approve the inbox. Save rechecks the proposer's current grant and rejects revoked or insufficient access; Dismiss remains available to the local owner.

Native owner review needs no OpenAI API key. Local harnesses use the authenticated stdio bridge. Mother no longer manages ChatGPT tunnels; any remote transport requires separate supported setup and acceptance.

## Persistence and recovery

Native v1 and graph proposals use separate inbox tables inside the canonical redb store. Merely opening an existing native workspace preserves its identity, generation, grants and records. Pending proposals stay outside ordinary recall and semantic indexing; those read committed canonical records, including records saved through compatible direct-write grants.

Portable exports include the inbox extension once present. Restore validates it before creating a fresh destination, preserves proposal/audit IDs and excludes grants. Pending proposals retain their capture generation: after restore they are visibly stale and cannot be saved. They can be dismissed, or an agent can create a fresh proposal against the restored workspace and current target revision. Restore never approves or runs anything.

The native v1 compatibility importer accepts validated historical portable bundles, including those without an inbox extension, subject to its matching-identity empty-view requirement. It does not open a SQLite database or supply a generic legacy restore command. Use a full native backup when small v1 export cannot represent all state.

For native workspaces, use a [full backup](garden-backups.md) to preserve graph and v1 proposals, outcomes and canonical state. Small v1 export refuses with `FULL_BACKUP_REQUIRED` when it cannot represent the whole workspace. Restored pending proposals retain their old generation and cannot be approved as new requests. Historical cutover journals and source fences remain enforced; exact grant reissue is explicit and new SQLite conversion/activation is unavailable. Rollback after new canonical writes requires reconciliation rather than discarding approved knowledge.

## Verification boundary

Automated tests use synthetic workspaces and real MCP subprocesses, plus the same TUI review controller and terminal rendering used by the native owner inbox. They cover approval, edits, dismissal, cross-client correction, retries, duplicates, stale revisions, revocation and restore. Actual ChatGPT/Hermes/Claude conversations must be checked after their installed executables are upgraded and tools reloaded; protocol tests alone do not establish that those apps have loaded the new tools.
