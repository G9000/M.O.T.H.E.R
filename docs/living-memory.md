# Living Memory

Keep what matters from a conversation, review it once, then continue from another harness. M.O.T.H.E.R stays an independent local context layer; it does not need AnimaOS or another reasoning model.

## Capture, review, continue

In a connected chat, ask:

> Keep what matters from this conversation. Search M.O.T.H.E.R for existing records first. Propose a small set of useful memories or corrections with their source and certainty for my review inbox. Do not save them directly as approved memories.

The active agent calls `mother_propose_memory` once per candidate. It can propose a memory, decision or task. It should resolve full existing records before proposing corrections, and keep tentative plans distinct from confirmed decisions. This is an explicit capture workflow; MCP does not automatically receive every conversation or monitor your chats.

Open the native inbox:

```powershell
mother inbox
```

Or run `mother` and press **i**. The inbox works without a configured ChatGPT tunnel. It uses the default workspace under `MOTHER_HOME` (or the usual private data directory).

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

Build the new executable with `cargo release`, then update your chosen stable install and restart/reload each harness's MCP process so it discovers `mother_propose_memory` and `mother_get_proposal`. Running processes retain the executable they started with. Existing client configs and the canonical workspace remain usable; do not initialize or restore over your current database for an upgrade. Building this feature alone does not upgrade installed clients or restart a tunnel.

A client with the matching existing `memory:write`, `decision:write` or `task:write` scope can propose that kind. A read-only client cannot propose. For a new client that may submit all three kinds for review but may not write canonical records directly, create a separate grant:

```powershell
mother grant reviewed-agent --scopes read,context:propose
```

Use that new private config in the intended harness. This command does not modify an existing grant. Existing write-capable grants still permit their direct-write tools for compatibility; the review-first workflow must use `mother_propose_memory`. No MCP tool can approve the inbox. Save rechecks the proposer's current grant and rejects revoked or insufficient access; Dismiss remains available to the local owner.

For private ChatGPT connections, a tunnel is still needed to reach the local MCP service. The inbox itself needs no OpenAI API key. Local harnesses use stdio MCP as before.

## Persistence and recovery

Proposals and their operation receipts are stored in additive inbox tables in the same SQLite database. Original v1 tables, workspace ID, restore generation, grants and canonical records remain unchanged by opening an existing workspace. Ordinary recall and semantic indexing only read approved canonical records.

Portable exports include the inbox extension once present. Restore validates it before creating a fresh destination, preserves proposal/audit IDs and excludes grants. Pending proposals retain their capture generation: after restore they are visibly stale and cannot be saved. They can be dismissed, or an agent can create a fresh proposal against the restored workspace and current target revision. Restore never approves or runs anything.

Legacy exports without an inbox remain supported. Use an inbox-aware M.O.T.H.E.R binary to export or restore workspaces containing these proposals; older binaries do not know how to preserve the extension.

## Verification boundary

Automated tests use synthetic workspaces and real MCP subprocesses, plus the same TUI review controller and terminal rendering used by the app. They cover approval, edits, dismissal, cross-client correction, retries, duplicates, stale revisions, revocation and restore. Actual ChatGPT/Hermes/Claude conversations must be checked after their installed executables are upgraded and tools reloaded; protocol tests alone do not establish that those apps have loaded the new tools.
