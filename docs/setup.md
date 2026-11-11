# Using your workspace

M.O.T.H.E.R is a standalone Rust executable with its own SQLite store. Bun and Node are not required at runtime. AnimaOS is not required. The checked-in files contain the program and an editable persona example; private records and client credentials stay in your user data directory.

## Terminal home

Run `mother` in a terminal or `mother tui` for the connection dashboard. It guides private ChatGPT setup, stores the runtime key in your OS credential store, and reports readiness. See [the terminal guide](terminal.md). Open `mother inbox` (or press **i** at home) to review proposed memories with Save, Edit or Dismiss; [Living Memory](living-memory.md) explains capture and cross-harness corrections. Existing scripting and MCP commands remain available.

## Build or upgrade

Use `mother doctor --config <private.json> --json` or press **t** at home to check the current executable through MCP without displaying private context. See [connection health](health.md) for report fields, bounds and the distinction from actual host verification.

With Rust 1.93 or newer, run these native Cargo shortcuts from the repository root:

```powershell
cargo dev
```

This builds the development executable and opens the TUI. For an optimized standalone executable:

```powershell
cargo release
```

These aliases are defined in `.cargo/config.toml`. You can also use the full Cargo command:

```powershell
cargo build --release --locked
.\target\release\mother.exe --help
```

The executable is `target/release/mother.exe` on Windows or `target/release/mother` elsewhere. The commands below assume it is available as `mother`; otherwise use its full path. You can copy the executable to a stable private folder before generating a harness connection.

Existing TypeScript workspaces use the same v1 SQLite schema and client JSON configs. Reuse your database and config; upgrading the executable does not change workspace identity, restore generation, grants or memories. Keep a private backup, then verify the existing connection and print a new native launcher:

```powershell
mother context --config "C:\path\to\codex.private.json"
mother connection --config "C:\path\to\codex.private.json" --format codex
```

Replace the previous Bun launch command in the chosen harness with that generated entry. Existing clients do not need new tokens. This is a runtime upgrade, so use the existing store directly rather than the restore command. See [validation notes](validation.md) for recorded compatibility checks.

## Initialize a new workspace

Only for a new instance, create your own UTF-8 persona file (you can customize [the neutral example](../examples/SOUL.md)), then select it explicitly:

```powershell
mother init --soul "C:\private\SOUL.md"
```

Alternatively, place your own `SOUL.md` beside the intended database and run `mother init`. With `--db`, the default file is beside that selected database. The current working directory is never searched automatically. An explicit `--soul` path may be absolute or relative to the directory where you run the command. Missing, unreadable, blank, invalid UTF-8 or oversized files fail before creating a database; the executable contains no fallback persona.

The command prints the database path and stable workspace identity. It refuses to overwrite an existing database. On Windows the default is `%LOCALAPPDATA%\MOTHER\context.sqlite`, outside OneDrive and the checkout. On other platforms it is `~/.mother/context.sqlite`. Set an absolute `MOTHER_HOME` before initialization to select another local private directory, or pass `--db` explicitly.

Use a local disk. Do not place the live SQLite database on a network share or sync its live files through a cloud folder. Use a portable export for transfer.

`SOUL.md` is owned by the workspace user. Initialization imports the chosen text (at most 32,000 UTF-16 units) and stores its SHA-256 digest. Runtime context reads and exports use this stored snapshot, so switching harnesses or restoring elsewhere does not depend on the original file path. Editing the source file afterward does not reload it automatically; use the explicit owner update command below. Existing workspaces retain their saved persona through upgrades, even if the original file is absent. The repository root `SOUL.md` is the maintainer's personal file, not a compiled default; it is used only if explicitly selected with `--soul`.

## Update an existing workspace's SOUL.md

Inspect the currently stored persona, then import your edited UTF-8 file using that persona's hash:

```powershell
mother soul show
mother soul update --file "C:\private\SOUL.md" --expected-hash "<hash from soul show>"
```

Both commands accept `--db "C:\private\context.sqlite"` for another workspace; use the same database for inspection and update. Read/review the current text before applying edits. There is no file watcher: saving the file alone does not alter the workspace. The file is read once, validated, and imported as the new stored snapshot.

The expected hash prevents overwriting a different current version. A stale hash returns `CONFLICT` without changing anything; reread and reconcile instead of automatically substituting the latest hash. Importing identical text with the current hash returns `changed: false`. If a response is lost, use `soul show` to compare the stored text/hash with your intended file; a retry with the previous hash will conflict after a successful update. Hashes compare current content, not revision history.

Only persona metadata changes. Workspace identity/generation, memories, grants, existing record receipts and semantic record vectors are preserved. Connected MCP clients receive the new snapshot on their next context/search call, with no restart needed. Previously returned text in a harness's conversation is not erased or rewritten. An in-flight semantic search that observes the change returns fresh literal context rather than a stale persona.

This is owner administration using local filesystem access, like grant/export; MCP clients receive no new mutation scope or tool. The command creates no persona history or operation receipt. Use `mother export` beforehand if you want a portable backup; old exports retain their saved persona and new exports include the updated one. To undo, explicitly import the desired prior file with the current hash. The same nonblank UTF-8 and 32,000 UTF-16-unit limits apply as at initialization.

## Give each harness its own connection

Create a client with the capabilities you want it to have:

```powershell
mother grant codex --scopes read,memory:write,decision:write,task:write,checkpoint:write,artifact:write,action:write
mother grant second-harness --scopes read,memory:write,decision:write,task:write,checkpoint:write,artifact:write,action:write
```

Each command prints a private config path. The token is written to that file, never printed. Omitting `--scopes` creates a read-only client. Each grant requires a new client name; revocation is permanent for that name in this release. To reconnect after revocation, use a new name and config.

Use the exact config path printed by the grant command:

```powershell
mother connection --config "C:\path\to\codex.private.json" --format codex
mother connection --config "C:\path\to\second-harness.private.json"
```

The first prints a `[mcp_servers.mother]` TOML entry for Codex. Add that entry to your existing Codex configuration without replacing other settings. The second prints the executable and argument array for a client supporting local MCP stdio. Absolute paths are generated for your installation; no working-directory assumption is needed.

Codex's documented local MCP configuration uses a command and arguments in `config.toml`. See [the official connection guide](https://learn.chatgpt.com/docs/extend/mcp#connect-codex-to-an-mcp-server). This project's protocol tests are not proof that a particular installed host has loaded the connection; check that host's tool list after setup.

All clients must point to the same database path. Each launches its own lightweight server process, and SQLite coordinates their writes. When you switch machines, use export/restore and reconnect; this release does not synchronize multiple database copies over the network.

For Claude Code, Hermes, OpenClaw, Claude Desktop and similar clients, `mother connection --format claude-code|hermes|openclaw|mcp-servers` prints ready-to-paste setup, and `mother brief` prints a session-start briefing; see [bring her to every harness](harnesses.md). For another harness, enter the generated command and arguments in its supported MCP configuration and use [the session instructions](../adapters/session.md). A host without local MCP stdio needs an additional adapter. No claim is made that every harness supports these settings. For ChatGPT, use the [private tunnel guide](chatgpt.md): OpenAI's tunnel client connects the existing stdio server without adding a public HTTP endpoint to M.O.T.H.E.R.

## Local semantic retrieval

Install the optional local model with `mother model install`. Search uses hybrid retrieval by default and falls back explicitly to literal matching when the model or cache is unavailable. `mother search` and `mother index` expose the same retrieval layer through the CLI. See [local semantic search](semantic-search.md) for setup, bounds and offline behavior. The original six MCP tools and canonical v1 record store remain compatible; Living Memory adds two proposal tools.

## Check a connection

```powershell
mother context --config "C:\path\to\codex.private.json"
```

This verifies the grant and reads current context. It may print private content, so use your own terminal. It is an owner check, not a model conversation.

In each connected harness, ask:

> Use M.O.T.H.E.R to retrieve my current workspace identity, personality and unfinished work. Resolve the relevant full records before continuing. Save a checkpoint after meaningful progress.

For your first real handoff, use synthetic material: save a tea preference and an unfinished task in one harness, retrieve them in the other, correct the preference and advance the task, then return to the first. Verify identical record IDs and the latest revisions. No real account action is necessary.

## Tools and capabilities

| Tool | Purpose | Required scope |
| --- | --- | --- |
| mother_get_context | Workspace, persona, previews, unsettled work | read |
| mother_search_context | Local hybrid, semantic or literal search with explicit coverage | read |
| mother_get_record | Full current record, including retracted records | read |
| mother_get_record_history | Earlier revisions of one record, or one earlier revision in full | read (not shared-access clients) |
| mother_get_receipt | This client's committed operation outcome | read |
| mother_write_record | Create or revise a typed record | matching kind:write |
| mother_checkpoint | Save a continuation checkpoint | checkpoint:write |
| mother_pin_record | Pin or unpin a memory or decision in the always-included core | matching kind:write |
| mother_propose_memory | Submit a memory, decision or task for owner review | context:propose or matching kind:write |
| mother_get_proposal | Read a submitted proposal and its review outcome | read |

Records can be memories, decisions, tasks, checkpoints, text artifacts or action evidence. Their schemas are returned by MCP tools. Every write requires the workspace ID and restore generation from a current context read, an operation UUID, and an expected revision. Create: omit `id`, set `expectedRevision` to zero. Update: use the full current record's ID/revision and a complete replacement draft. Do not send record metadata inside the draft.

A checkpoint write may include `supersedes` with the ID and current revision of the earlier active checkpoint it replaces. That checkpoint becomes `superseded` in the same transaction, with its own attributed revision and receipt, and leaves ordinary context and search; a stale revision, a missing record or a non-checkpoint target rejects the whole write.

A conflict means reread and reconsider. A lost response means look up the receipt or retry the exact request with the same operation ID. Never change a request while retaining its ID. Retraction removes a record from normal context/search; direct reads and historical receipts still contain it. This is not permanent erasure of archives or backups.

The `owner_statement` basis records what the client says its source was. It is not proof of ownership or elevated authorization. Source text is untrusted. An action with status `unknown` stays unresolved until evidence establishes an outcome. Saving an action record never executes it, and a checkpoint never stops a running job.

## Revoke a harness

```powershell
mother revoke second-harness
```

Every subsequent tool call checks the current grant, including already connected clients. Revocation cannot recall content already returned or stop an external action another harness already started. Local grants do not sandbox a process that has direct access to your OS account and files. Keep the data folder/config files private and protect them with your OS account permissions. The program requests restrictive file modes where supported; it does not configure Windows ACLs or encrypt the database.

## Export and restore

```powershell
mother export --out "C:\private-backups\mother.private.json"
mother restore --from "C:\private-backups\mother.private.json" --db "C:\private-data\restored.sqlite"
```

The parent directories must already exist. Export refuses to overwrite a file. Restore validates the entire bundle and refuses an existing database. Export includes all current records, historical mutation receipts, public IDs, revisions, the initialized persona and embedded text artifacts. It excludes client grants/tokens. Source URLs are preserved as references; their external bytes are explicitly listed as omitted. Only plain text/Markdown artifacts are supported. Portable files are limited to 64 MiB, 10,000 current records and 100,000 operation receipts. Writes enforce those counts and a conservative byte budget before committing, so a workspace cannot outgrow this release's export format. At capacity a RESOURCE_LIMIT response leaves the write uncommitted; export before archival or migration. Retraction retains history and does not free receipt capacity. Archival/compaction is not implemented.

Restore commits schema, identity, records and receipts together. If the process is killed before that commit, the destination is rejected as an unsupported/incomplete database rather than opened as an empty workspace. Use a fresh destination to retry and remove the incomplete destination only after confirming no process is using it.

Restoration preserves logical identity and record references, changes restore generation, and executes no actions. Create new grants using `--db` for the restored file and switch every client to the new configs. Stop using the old instance before treating the restored instance as authoritative; both copies cannot be kept in sync by this release. An old generation is rejected by the restored store. Uncertain actions remain uncertain.

Exports contain private information and are not encrypted. Store them outside Git. A code checkout alone does not contain your memories. Changing or rolling back code does not undo context mutations.

## Development

```powershell
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
```

See [validation evidence](validation.md) for what was actually exercised. The README remains the poem and artwork.

## Connect AnimaOS

Use `mother connect animaos`, or press **n** in the TUI. Select your AnimaOS agent and confirm the connection after owner authentication. See [AnimaOS setup](animaos.md). No manual command/path copying is needed after opening the wizard.

## Read shared context

Context includes checkpoints, decisions and other record categories as well as memories. Workspace inventory counts are separate from request filters and bounded previews. See [understanding shared context](context.md) for category counts, retained history and unsettled-work scope.

## Optional agent context policies

New credentials can select companion identity or an explicit context-only record allowlist. Existing grants retain legacy access. See [client context policies](client-access.md) for owner grant flags, scoped counts, proposal visibility and restore behavior.
