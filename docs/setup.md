# Using your workspace

M.O.T.H.E.R is a standalone Rust executable with one native redb Garden runtime. Bun, Node and AnimaOS are not required. Graph storage and the explicitly enabled v1 context view share one transactional authority. Private records, persona snapshots, backups and client credentials belong in an owner-private local directory outside this checkout and cloud-synced folders.

## Build

```powershell
cargo build --release --locked
.\target\release\mother.exe --help
```

Use Rust 1.93 or newer. The executable is `target/release/mother.exe` on Windows or `target/release/mother` elsewhere. The examples assume it is available as `mother`; otherwise use its absolute path. All builds are native: there is no SQLite feature or alternate converter build. Bare `mother` prints help. The command tree is `garden` plus `mcp --config`.

Building does not install a binary, update a running service, change harness settings or migrate data. Existing native stores retain identity, generation, grants and records when opened by an updated executable. Old SQLite configs are explicitly unsupported; they never create or convert a database automatically. Preserve old private data and consult [historical cutover recovery](garden-migration.md).

## Initialize a new native workspace

Create an owner-private directory on a local disk and write your own nonblank UTF-8 `SOUL.md`; [the neutral example](../examples/SOUL.md) is a starting point. Keep endpoint and credential paths private as described in [service operation](garden-service.md). Initialization is only for a new store:

```powershell
mother garden init --db "C:\private-data\mother\garden.redb"
mother garden compat-init --db "C:\private-data\mother\garden.redb" --soul "C:\private-data\mother\SOUL.md"
mother garden grant-v1 codex --db "C:\private-data\mother\garden.redb" --out "C:\private-data\mother\codex.native.json" --access companion --subject codex --scopes read,context:propose
mother garden service --db "C:\private-data\mother\garden.redb" --endpoint "C:\private-data\mother\service.json"
```

`init` refuses an existing destination. `compat-init` explicitly enables the v1 view and saves the owner's persona snapshot; it does not infer a persona from the working directory. The persona is bounded to 32,000 UTF-16 units. Existing native stores retain that snapshot even if the source file is absent or edited. The removed `soul show/update` commands have no replacement in the current owner CLI.

The service holds the sole database handle and remains in the foreground. In a second terminal, authenticate that service and create the harness bridge config:

```powershell
mother garden service-config --config "C:\private-data\mother\codex.native.json" --endpoint "C:\private-data\mother\service.json" --out "C:\private-data\mother\codex.service.json"
```

Configure the intended harness to launch the stable executable with arguments `mcp`, `--config`, and the absolute service-config path. Repeat provisioning with a distinct credential for each harness. Mother does not edit harness settings or adopt a persona on the harness's behalf. See [harness setup](harnesses.md), [access policies](client-access.md) and [native compatibility](garden-compatibility.md).

Graph-only workspaces may omit `compat-init` and use `garden grant` with explicit graph policies; [graph setup](garden-proposals.md) describes independent graph authority. The local service supports multiple authenticated harness bridges. A direct `garden_redb` config is an explicit single-process option and cannot share the file with a running owner service.

## Review, search and recovery

Stop the owner service before separate commands that open the database, including grants, owner review, selected indexing, export and backup. Restart it afterward.

```powershell
mother garden v1-inbox --db "C:\private-data\mother\garden.redb"
mother garden inbox --db "C:\private-data\mother\garden.redb"
mother garden review --db "C:\private-data\mother\garden.redb" --request "C:\private-data\mother\review.json"
```

The inbox commands open native owner review when both input and output are terminals; piped invocations emit JSON listings. Review can also use a separate explicit JSON request, with `--graph` for a graph batch. Preserve its operation UUID and exact payload when reconciling uncertainty. MCP and IPC cannot approve proposals. [Living Memory](living-memory.md) explains capture, attribution and conflicts.

Context, search, full records, history, pins, assignments, writes and checkpoints remain MCP tools on the native v1 view. The former general owner `context`, `search`, `index`, `brief`, `connection`, `doctor`, `connect`, `apps`, `launch`, `model` and setup dashboard commands are removed. Local semantic retrieval can use already verified assets; the runtime never downloads them. See [search](semantic-search.md).

Use [full backups](garden-backups.md) for complete graph/v1/blob recovery. Restore requires a fresh destination, preserves workspace identity, changes generation and excludes grants. Small v1 export/import remains a bounded compatibility interface and refuses state requiring full backup. Historical cutover journals and fences remain enforced, but new SQLite conversion/activation is unavailable. No update or recovery operation implicitly changes live settings or replays uncertain effects.

[Validation](validation.md) separates current native qualification requirements from archived measurements and host acceptance. No protocol test proves a running host adopted a connection or that a model used it correctly.
