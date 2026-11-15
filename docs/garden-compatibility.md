# Native v1 compatibility

G7 adds an explicitly enabled v1 view to the redb Garden. The eleven existing MCP tool schemas stay unchanged: context, search, full records, history, writes, checkpoints, receipts, proposed memories, proposal status, pins and task assignment. Native v1 records and graph records use the same database transaction authority, with separate typed tables and independent grants. Graph-only objects do not appear as invented v1 records.

The native redb Garden is the sole runtime. A strict `"backend":"garden_redb"` configuration selects a direct single-process connection; `"backend":"garden_service"` selects the authenticated local owner service. Both are supported through `mother mcp --config`. Old SQLite configs and unknown backend tags fail explicitly before opening a store. There is no autodetection, implicit conversion, shadow database or dual write.

The default Cargo build is native, with no SQLite feature or converter build. The executable exposes `garden` owner commands and MCP. The former general `context`, `search`, `index`, `brief`, `connection`, persona-update and app setup commands are removed; their v1 MCP tools remain available where authorized. Native inbox review remains interactive in terminals and emits JSON when piped. See [service operation](garden-service.md), [historical cutover recovery](garden-migration.md) and [validation boundaries](validation.md).

## Explicit owner setup

These examples create a separate workspace; they do not install a harness connection or migrate the owner's existing workspace. Parent directories must already exist. The owner supplies the persona file.

```powershell
mother garden init --db "C:\private-data\garden.redb"
mother garden compat-init --db "C:\private-data\garden.redb" --soul "C:\private-data\SOUL.md"
mother garden grant-v1 codex --db "C:\private-data\garden.redb" --out "C:\private-data\codex.private.json" --access companion --subject codex --scopes read,context:propose
mother garden service --db "C:\private-data\garden.redb" --endpoint "C:\private-data\service.json"
```

The service remains in the foreground and holds the sole database handle. In another terminal, authenticate that owner and create the harness bridge config:

```powershell
mother garden service-config --config "C:\private-data\codex.private.json" --endpoint "C:\private-data\service.json" --out "C:\private-data\codex.service.json"
mother mcp --config "C:\private-data\codex.service.json"
```

Repeat with a distinct credential for each harness. Direct `garden_redb` MCP configurations remain an explicit single-process interface; they cannot share the database with a running owner service. Use the service topology above for multiple harnesses.

`grant-v1` defaults to `read`. Kind write scopes and `context:propose` remain distinct. Shared clients require their own identity and explicit v1 record allowlist; they receive no owner persona, hidden counts, historical payloads or implicit access to a newly approved record. Independent graph scopes require an explicit graph policy through `--objects` or `--all`; full graph visibility requires an explicitly selected companion grant.

Without v1 initialization, a graph-authorized connection retains its eight tools, including capability discovery. Enabling v1 adds the eleven legacy schemas. A context-only native credential receives twelve v1/context tools without unrelated graph descriptors. Graph capability availability reflects the current grant; defined graph contracts remain discoverable as definitions. Advertising a tool does not confer its scope. Revocation and current grants are checked on each call; restart/review discovery after grant changes.

Stop the service before separate owner commands that open its database, including grant/revoke, review, indexing, export and backup; restart it afterward. Review proposed memories interactively using `mother garden v1-inbox --db ...` and graph batches using `mother garden inbox --db ...`. When piped, these commands list proposals as JSON. A separately reviewed JSON request can use `mother garden review --db ... --request ...` (add `--graph` for a graph batch). Preserve the original operation UUID when reconciling uncertain review. MCP/IPC cannot approve proposals, issue grants, ingest files or restore backups. Pending proposals remain outside canonical context and search.

## Optional local search

Native v1 hybrid and semantic search use the existing pinned local Candle model and shared v1 ranking and passage metadata. Search loads only explicitly installed, verified local assets; it does not download models or call a remote provider. Literal search does not load or call an encoder. Missing or busy models, failed embeddings and unavailable caches return a truthful literal fallback.

The rebuildable vector cache uses redb, independently of canonical records. Its keys bind workspace generation, client, visibility policy, model profile, record revision and content digest. Before publishing derived vectors or returning semantic candidates, search rechecks the current grant, policy and canonical snapshot. Revocation fails closed; a changed snapshot returns fresh literal results with `workspace_changed`. The cache is bounded to 128 MiB and 32,000 passages, and it is excluded from full backups. Search metadata reports coverage and capacity rather than promising complete learned retrieval.

## Preservation and portability

Native writes normalize requests consistently and preserve public IDs, sources, correction and retraction revisions, immutable receipts and uncertain action statuses. Retrying the same operation reconciles its receipt; a changed payload or conflicting operation family is rejected. A checkpoint replacement retires its predecessor in the same transaction. Legacy portable fixtures retain their cross-runtime request hashes.

`mother garden import-v1 --db ... --file ...` imports a validated small portable bundle only into an empty compatibility view with the same explicit workspace ID and generation. This is a low-level owner import boundary, not a live migration or endpoint activation command. `mother garden export-v1 --db ... --out ...` produces the existing small portable format only when it represents all canonical state. Graph proposals, reviews, objects, blobs or native state beyond the legacy limits produce `FULL_BACKUP_REQUIRED`.

New native storage does not impose the SQLite release's 10,000-record, 100,000-receipt or 64 MiB lifetime caps. Individual requests, references, previews, pins and serialized rows stay bounded; history and receipts are retained. Use [full backups](garden-backups.md) for the complete graph and v1 state. Restore preserves identity, changes generation, excludes grants and blocks old requests and pending approvals.

When an imported record has more than 1,000 missing revision snapshots, native history listing returns `RESOURCE_LIMIT` instead of enumerating an unbounded gap. Request a specific retained revision directly. Dense histories and their immutable receipts are retained without a lifetime count cap.

Each harness continues to decide what to remember and how to interpret retrieved context. This compatibility layer preserves storage and access behavior; it performs no automatic extraction or transcript ingestion.

The SQLite converter and source activation commands are removed. Existing journal fences still block unactivated destinations. Historical status, exact grant reissue and eligible rollback remain available; after canonical destination writes rollback requires reconciliation rather than discarding acknowledged changes. [The recovery guide](garden-migration.md) explains those retained boundaries. Low-level v1 import is not endpoint activation.

## G7 qualification, 2026-11-15

This is historical milestone evidence. G8 was subsequently integrated and pushed at `df6c6eb07337ab28a153f5c9decf27d63b34d8d1`; current release/platform/scale gates are reported separately in the [G9 assessment](benchmarks/garden-acceptance.md). Statements below about uncommitted work and remaining G8 work describe the G7 checkpoint only.

Validated in the isolated `g9000/mother-garden-backups` worktree on Windows with synthetic context, private temporary directories and no changes to installed harness settings or live databases.

| Check | Result |
| --- | --- |
| v1 compatibility | All eleven tools exercised through a shared SQLite/native scenario runner; original TypeScript request/receipt fixtures replay unchanged. |
| Authority and owner review | Current scope/revocation, conflicting corrections, edited references, shared payload filtering, cross-family UUIDs, checkpoint rollback and native owner UI checked. |
| Native semantic search | Six synthetic-encoder tests cover candidates beyond literal matches, cache reuse, literal/no-encoder behavior, model busy/missing, corrupt-cache fallback, revocation, revision changes, profile identity and narrowed visibility. |
| Actual stdio | Five tests cover explicit backend selection, unchanged legacy schemas, v1-only and graph-only authority, unavailable local model fallback and rejected unknown backends. |
| Full backup | Current/history/receipts/inbox/pins/assignments/persona/source pointers and opaque graph extensions survive restore; grants and proposal credential snapshots do not. |
| Corruption | Rechecksummed semantic/receipt/blob corruption, missing blobs/chunks, duplicate canonical rows, required versions, path escape and unexpected files fail before publication. Unix symlink coverage is present but was not run on Windows. |
| Capacity | A 65 MiB ingested blob produces a chunked bundle beyond the old total export limit. Native writes and full restore retain 10,001 current records with bounded context previews. |
| Process interruption | Six backup-restore barriers leave the destination absent or complete. Existing twenty graph transaction/review interruption scenarios also pass. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test --locked` | 439 passed, zero failed, four ignored. |
| `cargo build --release --locked` | Passed. |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed. |
| `cargo doc --locked --no-deps` | Passed. |
| `git diff --check` | Passed. |

The ignored tests are two explicitly installed real-model tests and two private child helpers which their recovery parents execute directly. Learned retrieval was qualified with synthetic encoders; no real model was installed or run. Atomic hard-link publication required the Windows tests to run outside the filesystem sandbox against the owned temporary test root. Independent review found no remaining blocking G7 findings.

These checks establish synthetic local storage, protocol and interruption recovery. They do not establish Linux/macOS CI, hardware power-loss durability, production latency, actual model quality, real harness loading or live migration acceptance. G7 adds no new Cargo dependency or lockfile change relative to the verified G6 source. Changes remain uncommitted; existing pairing edits in the primary checkout are preserved.

- [x] Native v1 contracts and semantic compatibility on one redb transaction authority.
- [x] Explicit backend selection with independent current grants and owner review.
- [x] Bounded full graph/v1/blob backup validation and fresh-generation restore.
- [x] Small v1 export compatibility with explicit full-backup refusal when necessary.
- [x] Capacity, corruption, generation and process-interruption acceptance checks.
- [x] Required package checks and independent review.
- [ ] G8 live migration, endpoint cutover, real harness acceptance and a build excluding the legacy SQLite backend.
