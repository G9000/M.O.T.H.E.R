# Using your workspace

M.O.T.H.E.R runs locally with Bun and owns its own SQLite store. AnimaOS is not required. The checked-in files contain the program and personality seed; private records and client credentials stay in your user data directory.

## Initialize

From the M.O.T.H.E.R checkout, install the locked dependencies and create a workspace:

```powershell
bun install --frozen-lockfile
bun run mother init
```

The command prints the database path and stable workspace identity. It refuses to overwrite an existing database. On Windows the default is `%LOCALAPPDATA%\MOTHER\context.sqlite`, outside OneDrive and the checkout. On other platforms it is `~/.mother/context.sqlite`. Set an absolute `MOTHER_HOME` before initialization to select another local private directory, or pass `--db` explicitly.

Use a local disk. Do not place the live SQLite database on a network share or sync its live files through a cloud folder. Use a portable export for transfer.

`SOUL.md` seeds the personality at initialization. The stored snapshot and its SHA-256 digest are returned to every client and included in exports. Editing the repository file changes future new workspaces; it does not silently rewrite an existing personality.

## Give each harness its own connection

Create a client with the capabilities you want it to have:

```powershell
bun run mother grant codex --scopes read,memory:write,decision:write,task:write,checkpoint:write,artifact:write,action:write
bun run mother grant second-harness --scopes read,memory:write,decision:write,task:write,checkpoint:write,artifact:write,action:write
```

Each command prints a private config path. The token is written to that file, never printed. Omitting `--scopes` creates a read-only client. Each grant requires a new client name; revocation is permanent for that name in this release. To reconnect after revocation, use a new name and config.

Use the exact config path printed by the grant command:

```powershell
bun run mother connection --config "C:\path\to\codex.private.json" --format codex
bun run mother connection --config "C:\path\to\second-harness.private.json"
```

The first prints a `[mcp_servers.mother]` TOML entry for Codex. Add that entry to your existing Codex configuration without replacing other settings. The second prints the executable and argument array for a client supporting local MCP stdio. Absolute paths are generated for your installation; no working-directory assumption is needed.

Codex's documented local MCP configuration uses a command and arguments in `config.toml`. See [the official connection guide](https://learn.chatgpt.com/docs/extend/mcp#connect-codex-to-an-mcp-server). This project's protocol tests are not proof that a particular installed host has loaded the connection; check that host's tool list after setup.

All clients must point to the same database path. Each launches its own lightweight server process, and SQLite coordinates their writes. When you switch machines, use export/restore and reconnect; this release does not synchronize multiple database copies over the network.

For another harness, enter the generated command and arguments in its supported MCP configuration and use [the session instructions](../adapters/session.md). A host without local MCP stdio needs an additional adapter. No claim is made that every harness supports these settings. The local server is not a remote ChatGPT web endpoint; that needs a separate authenticated deployment.

## Check a connection

```powershell
bun run mother context --config "C:\path\to\codex.private.json"
```

This verifies the grant and reads current context. It may print private content, so use your own terminal. It is an owner check, not a model conversation.

In each connected harness, ask:

> Use M.O.T.H.E.R to retrieve my current workspace identity, personality and unfinished work. Resolve the relevant full records before continuing. Save a checkpoint after meaningful progress.

For your first real handoff, use synthetic material: save a tea preference and an unfinished task in one harness, retrieve them in the other, correct the preference and advance the task, then return to the first. Verify identical record IDs and the latest revisions. No real account action is necessary.

## Tools and capabilities

| Tool | Purpose | Required scope |
| --- | --- | --- |
| mother_get_context | Workspace, persona, previews, unsettled work | read |
| mother_search_context | Literal text search with explicit omissions | read |
| mother_get_record | Full current record, including retracted records | read |
| mother_get_receipt | This client's committed operation outcome | read |
| mother_write_record | Create or revise a typed record | matching kind:write |
| mother_checkpoint | Save a continuation checkpoint | checkpoint:write |

Records can be memories, decisions, tasks, checkpoints, text artifacts or action evidence. Their schemas are returned by MCP tools. Every write requires the workspace ID and restore generation from a current context read, an operation UUID, and an expected revision. Create: omit `id`, set `expectedRevision` to zero. Update: use the full current record's ID/revision and a complete replacement draft. Do not send record metadata inside the draft.

A conflict means reread and reconsider. A lost response means look up the receipt or retry the exact request with the same operation ID. Never change a request while retaining its ID. Retraction removes a record from normal context/search; direct reads and historical receipts still contain it. This is not permanent erasure of archives or backups.

The `owner_statement` basis records what the client says its source was. It is not proof of ownership or elevated authorization. Source text is untrusted. An action with status `unknown` stays unresolved until evidence establishes an outcome. Saving an action record never executes it, and a checkpoint never stops a running job.

## Revoke a harness

```powershell
bun run mother revoke second-harness
```

Every subsequent tool call checks the current grant, including already connected clients. Revocation cannot recall content already returned or stop an external action another harness already started. Local grants do not sandbox a process that has direct access to your OS account and files. Keep the data folder/config files private and protect them with your OS account permissions. The program requests restrictive file modes where supported; it does not configure Windows ACLs or encrypt the database.

## Export and restore

```powershell
bun run mother export --out "C:\private-backups\mother.private.json"
bun run mother restore --from "C:\private-backups\mother.private.json" --db "C:\private-data\restored.sqlite"
```

The parent directories must already exist. Export refuses to overwrite a file. Restore validates the entire bundle and refuses an existing database. Export includes all current records, historical mutation receipts, public IDs, revisions, the initialized persona and embedded text artifacts. It excludes client grants/tokens. Source URLs are preserved as references; their external bytes are explicitly listed as omitted. Only plain text/Markdown artifacts are supported. Portable files are limited to 64 MiB, 10,000 current records and 100,000 operation receipts. Writes enforce those counts and a conservative byte budget before committing, so a workspace cannot outgrow this release's export format. At capacity a RESOURCE_LIMIT response leaves the write uncommitted; export before archival or migration. Retraction retains history and does not free receipt capacity. Archival/compaction is not implemented.

Restore commits schema, identity, records and receipts together. If the process is killed before that commit, the destination is rejected as an unsupported/incomplete database rather than opened as an empty workspace. Use a fresh destination to retry and remove the incomplete destination only after confirming no process is using it.

Restoration preserves logical identity and record references, changes restore generation, and executes no actions. Create new grants using `--db` for the restored file and switch every client to the new configs. Stop using the old instance before treating the restored instance as authoritative; both copies cannot be kept in sync by this release. An old generation is rejected by the restored store. Uncertain actions remain uncertain.

Exports contain private information and are not encrypted. Store them outside Git. A code checkout alone does not contain your memories. Changing or rolling back code does not undo context mutations.

## Development

```powershell
bun test
bun run typecheck
```

See [validation evidence](validation.md) for what was actually exercised. The README remains the poem and artwork.
