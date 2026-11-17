# Local Garden owner service

One owner process holds the native Garden database open. Each harness connects through its own private service credential and an MCP stdio bridge. Bridges never open the database, start another owner or automatically retry a mutation. The service uses local Windows named pipes with remote clients rejected, or a Unix-domain socket. It opens no TCP listener.

## Build and private paths

Build the native executable:

```powershell
cargo build --release --locked
```

The executable is `target/release/mother.exe` on Windows or `target/release/mother` elsewhere. The commands below assume it is available as `mother`. Every build uses the native redb Garden; the SQLite runtime and converter are removed. Old SQLite configs fail explicitly without creating or migrating a store. See [historical cutover recovery](garden-migration.md).

Keep the database, endpoint descriptor and credentials outside the repository in an owner-private local directory. The endpoint directory must already exist. Unix requires mode `0700`; ancestors must prevent untrusted replacement. Windows verifies directory ownership, DACLs and the parent chain. Symlinks, reparse boundaries and hard-linked database aliases cannot establish a serving endpoint. Create a dedicated private directory rather than weakening these checks.

On Windows, an AppContainer or capability SID is not trusted because of its
prefix, length or association with a sandbox. Only the current process user,
SYSTEM and Administrators receive the existing trusted-principal treatment.
An unknown principal's ancestor allow ACE is rejected when it grants child
deletion, directory deletion, DACL/owner changes, or generic write/full access.
Read, traverse and synchronize rights on an ancestor do not trigger that check.
The endpoint directory itself stays owner-only, including its future contents;
an untrusted read grant there is rejected even when it would be harmless on an
ancestor. Ancestor inherit-only ACEs and Creator Owner inheritance keep their
existing semantics. Deny ACEs are not used to excuse a dangerous allow ACE, and
unsupported ACE forms fail closed.

Admission errors retain `MIGRATION_FENCED`. For a rejected allow ACE they identify
the private-directory/ancestor relationship, ancestor depth and access mask,
without echoing private paths, identity SIDs, credentials or security descriptors.
Missing, null or unreadable DACLs also fail closed; a null DACL never becomes an
SDK panic. These checks inspect permissions and do not repair or rewrite ACLs.
They are conservative snapshot checks, not a complete Windows effective-access
calculator or a guarantee against every filesystem race. If a parent really
permits untrusted replacement, Mother refuses admission; safely supporting such
a parent requires a separate anchored-access design. A child exclusion or an
unknown-SID whitelist is not an equivalent protection.

## Provision and start

For a new native workspace, supply your own persona and provision each harness before starting the owner. This example grants canonical reads and reviewed capture to one companion; graph authority remains independent.

```powershell
mother garden init --db "C:\private-data\mother\garden.redb"
mother garden compat-init --db "C:\private-data\mother\garden.redb" --soul "C:\private-data\mother\SOUL.md"
mother garden grant-v1 codex --db "C:\private-data\mother\garden.redb" --out "C:\private-data\mother\codex.native.json" --access companion --subject codex --scopes read,context:propose
mother garden service --db "C:\private-data\mother\garden.redb" --endpoint "C:\private-data\mother\service.json"
```

The service prints its public endpoint identity, then remains in the foreground. A second owner of the same database fails. Historical prepared cutover destinations remain fenced and cannot serve; the current executable has no source activation command. Inspect their existing journal rather than bypassing its fence. For an existing native workspace, reuse its database and owner-issued grants instead of running initialization again.

While the owner runs, use another terminal to create the bridge configuration:

```powershell
mother garden service-config --config "C:\private-data\mother\codex.native.json" --endpoint "C:\private-data\mother\service.json" --out "C:\private-data\mother\codex.service.json"
```

Configure that harness to launch:

```powershell
mother mcp --config "C:\private-data\mother\codex.service.json"
```

Repeat with a separate owner-issued grant and output file for each harness. `service-config` authenticates the running owner and writes a new file exclusively; it preserves the supplied grant and token. It neither creates additional authority nor edits harness settings. A previously activated cutover destination needs explicitly reissued native credentials first; reissue does not activate a prepared destination. Graph-only setup can omit v1 initialization and use the [graph provisioning commands](garden-proposals.md).

The service configuration has exactly `version: 3`, `backend: "garden_service"`, `endpointPath`, `workspaceId`, `generation`, `clientId` and `token`. Unknown fields and other formats fail closed. The endpoint descriptor contains public identity and address only. Keep both credential files private; debug formatting redacts their tokens.

## Authority and recovery

Before sending a bearer token, the client sends a fresh challenge. The owner proves possession of that client's current stored key using HMAC-SHA256, bound to the challenge, client, service, address and workspace generation. The client verifies the proof in constant time. A redirected endpoint or replayed proof receives no bearer token. Current credentials, workspace generation and the actual tool's scope are checked again for each domain call; a successful transport handshake grants no additional scope.

Native discovery uses the current credential's surface. A credential with any explicit graph scope receives graph tools and, when initialized, the v1 tools. A context-only credential receives the twelve v1/context tools, including capability discovery, with unchanged v1 schemas. `garden.available` and `availableTools` describe current graph authority; `definedTools` retains the graph definitions. Each call still checks its own current scope. Native search reuses the local search implementation. IPC exposes no grant, approval, import, restore, migration, launch or shutdown operation. Stop the service before separate owner CLI operations that need the database, then restart it afterward. Owner review remains explicit; pending proposals stay outside canonical recall.

Native `tools/list` discovery uses pages with at most 48 KiB of serialized result
data, including the next cursor. Clients must follow `nextCursor` to collect the
complete catalog; malformed or stale catalog cursors require restarting discovery.
The same pagination applies to direct native MCP. Context-only discovery fits
one page; graph catalogs remain paginated. Graph metadata includes explicit MCP
object roots while preserving canonical references and constraints. Native
context-only AnimaOS integration passes the actual executable fixture. The
separate additive AnimaOS worktree also admits all seven exact graph schemas
with separately bounded local-reference traversal, and exercises them through
its real tool executor and native bridge. Ordinary payload limits remain in
force, so use bounded batches and pages. This is synthetic local qualification,
as recorded in the [G9 assessment](benchmarks/garden-acceptance.md). Restart and
review the bridge after owner grant or catalog changes; discovery never grants
permission or revives an earlier approval.

Each local IPC JSON frame is limited to 4 MiB; descriptor and configuration reads are limited to 16 KiB. Server frame and acknowledgement phases have fixed 500 ms deadlines, and peer handling uses no unbounded worker pool. Client connection waiting is limited to 500 ms, owner-proof waiting to two seconds and domain-response waiting to thirty seconds. These are phase deadlines, not a promise that an admitted operation is cancelled when a client stops waiting.

Restart uses the same database and matching endpoint descriptor. The owner lease, retained database handle and an existing-owner probe prevent displacing a running owner. Descriptor publication is atomic and exclusive; unrelated files or another workspace generation are not overwritten. Clients reread the descriptor on subsequent calls and can reconnect to a replacement owner with the same workspace generation. Restart a bridge when owner configuration changes the advertised toolset.

A lost connection or deadline after submission can leave a write outcome uncertain. Reconnect and reconcile its original operation UUID through the appropriate receipt tool. An exact retry uses the same UUID and payload; a changed payload is rejected. The bridge never performs that retry automatically. Restore changes generation and requires new explicit credentials; editing an old configuration does not authorize the restored workspace.
