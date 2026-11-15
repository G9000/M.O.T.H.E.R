# Local Garden owner service

One owner process holds the native Garden database open. Each harness connects through its own private service credential and an MCP stdio bridge. Bridges never open the database, start another owner or automatically retry a mutation. The service uses local Windows named pipes with remote clients rejected, or a Unix-domain socket. It opens no TCP listener.

## Build and private paths

Build the executable without the legacy SQLite backend:

```powershell
cargo build --release --locked --no-default-features
```

The executable is `target/release/mother.exe` on Windows or `target/release/mother` elsewhere. The commands below assume it is available as `mother`. The default build retains `legacy-sqlite` for existing SQLite workspaces and explicit conversion; changing a build feature does not migrate a database. See [migration](garden-migration.md).

Keep the database, endpoint descriptor and credentials outside the repository in an owner-private local directory. The endpoint directory must already exist. Unix requires mode `0700`; ancestors must prevent untrusted replacement. Windows verifies directory ownership, DACLs and the parent chain. Symlinks, reparse boundaries and hard-linked database aliases cannot establish a serving endpoint. Create a dedicated private directory rather than weakening these checks.

## Provision and start

For a new native workspace, supply your own persona and provision each harness before starting the owner. This example grants canonical reads and reviewed capture to one companion; graph authority remains independent.

```powershell
mother garden init --db "C:\private-data\mother\garden.redb"
mother garden compat-init --db "C:\private-data\mother\garden.redb" --soul "C:\private-data\mother\SOUL.md"
mother garden grant-v1 codex --db "C:\private-data\mother\garden.redb" --out "C:\private-data\mother\codex.native.json" --access companion --subject codex --scopes read,context:propose
mother garden service --db "C:\private-data\mother\garden.redb" --endpoint "C:\private-data\mother\service.json"
```

The service prints its public endpoint identity, then remains in the foreground. A second owner of the same database fails. Existing prepared migration destinations require explicit activation before serving. For an existing native workspace, reuse its database and owner-issued grants instead of running initialization again.

While the owner runs, use another terminal to create the bridge configuration:

```powershell
mother garden service-config --config "C:\private-data\mother\codex.native.json" --endpoint "C:\private-data\mother\service.json" --out "C:\private-data\mother\codex.service.json"
```

Configure that harness to launch:

```powershell
mother mcp --config "C:\private-data\mother\codex.service.json"
```

Repeat with a separate owner-issued grant and output file for each harness. `service-config` authenticates the running owner and writes a new file exclusively; it preserves the supplied grant and token. It neither creates additional authority nor edits harness settings. A migrated workspace needs explicitly reissued native credentials first. Graph-only setup can omit v1 initialization and use the [graph provisioning commands](garden-proposals.md).

The service configuration has exactly `version: 3`, `backend: "garden_service"`, `endpointPath`, `workspaceId`, `generation`, `clientId` and `token`. Unknown fields and other formats fail closed. The endpoint descriptor contains public identity and address only. Keep both credential files private; debug formatting redacts their tokens.

## Authority and recovery

Before sending a bearer token, the client sends a fresh challenge. The owner proves possession of that client's current stored key using HMAC-SHA256, bound to the challenge, client, service, address and workspace generation. The client verifies the proof in constant time. A redirected endpoint or replayed proof receives no bearer token. Current credentials, workspace generation and the actual tool's scope are checked again for each domain call; a successful transport handshake grants no additional scope.

The bridge advertises the native graph tools and, when initialized, the unchanged v1 tools. Native search reuses the local search implementation. IPC exposes no grant, approval, import, restore, migration, launch or shutdown operation. Stop the service before separate owner CLI operations that need the database, then restart it afterward. Owner review remains explicit; pending proposals stay outside canonical recall.

Each JSON frame is limited to 4 MiB; descriptor and configuration reads are limited to 16 KiB. Server frame and acknowledgement phases have fixed 500 ms deadlines, and peer handling uses no unbounded worker pool. Client connection waiting is limited to 500 ms, owner-proof waiting to two seconds and domain-response waiting to thirty seconds. These are phase deadlines, not a promise that an admitted operation is cancelled when a client stops waiting.

Restart uses the same database and matching endpoint descriptor. The owner lease, retained database handle and an existing-owner probe prevent displacing a running owner. Descriptor publication is atomic and exclusive; unrelated files or another workspace generation are not overwritten. Clients reread the descriptor on subsequent calls and can reconnect to a replacement owner with the same workspace generation. Restart a bridge when owner configuration changes the advertised toolset.

A lost connection or deadline after submission can leave a write outcome uncertain. Reconnect and reconcile its original operation UUID through the appropriate receipt tool. An exact retry uses the same UUID and payload; a changed payload is rejected. The bridge never performs that retry automatically. Restore changes generation and requires new explicit credentials; editing an old configuration does not authorize the restored workspace.
