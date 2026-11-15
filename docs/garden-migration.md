# Explicit Garden migration

Migration is an owner operation on synthetic or separately approved private data. Updating the existing SQLite runtime alone does not migrate a database or change its generation. A live migration and client setting changes require separate owner approval.

The `legacy-sqlite` build contains the read-only converter. The native build retains journal inspection, source/destination fence checks and rollback without linking SQLite. A dry run creates a new private destination and journal directory; it never changes the source rows, grants, settings or generation. The destination remains fenced until activation.

Preparation reads records, complete retained receipts, source references, inbox proposals and operations, pins, assignments and persona inside one SQLite read snapshot. The converter imports that bundle into a matching-identity staging store, validates a full Garden backup, then restores it into a fresh destination generation. Original receipt generations remain historical; pending proposals retain their old generation and require reconciliation. Unknown required extensions fail validation. Private grant restrictions live only in the owner journal: no old tokens or token hashes enter a portable bundle or log. Reissuance creates a new private credential for one existing client with its exact reviewed scopes and access restrictions.

Before activation, stop every source writer, including services, MCP hosts and older binaries, and close their SQLite connections. Let those writers finish their normal shutdown; migration never runs a checkpoint or rewrites the original SQLite database. A nonempty WAL or remaining SQLite sidecars blocks activation. The final source snapshot and restrictions must match preparation while a SQLite writer-admission lock is held. Changed state requires a new preparation and operation UUID.

Activation persists a source fence before releasing the final lock. Modern source opens and mutation admissions reject that fence, including connections opened before activation. Migration archives the original file beside its original location, makes the archive read-only, and fences the archive as well. It then records `activated`; destination serving requires that journal state. The destination must run under the single-owner local service. A stale client configuration fails instead of creating an empty database at the former source path. Owner-generated replacement configurations are explicit outputs; migration does not edit harness settings or start a service.

The journal is a private directory with monotonically numbered JSON transitions. Each transition is written to an exclusive temporary file, synced, atomically renamed to its unique final name and directory-synced where supported. A private file lock serializes owner commands. An operation UUID identifies the whole cutover; a retry with the same request resumes or returns its recorded state. Reusing that UUID with changed paths fails. Interruption after the source fence leaves the destination blocked until an explicit activation retry completes or an eligible rollback succeeds. Do not delete fences or journal transitions as recovery.

Rollback before destination canonical changes acquires the exclusive database owner and verifies the destination canonical fingerprint and archived source bytes. It then persists `rolling_back` to disable destination serving, restores the original source file and clears its own source fence. Stop the destination service before requesting rollback. Grant reissuance and disposable index/cache activity do not count as new canonical state. Records, revisions, receipts, graph inbox, v1 inbox, pins, assignments, persona and stored blobs do count. If any canonical state differs, rollback returns `RECONCILIATION_REQUIRED` and retains the source fence. After-write rollback requires a separately verified reverse export or reconciliation; this implementation does not discard acknowledged destination changes.

Use the following owner workflow with absolute paths inside an existing owner-private directory. `mother-migration` below denotes a separately retained executable built with `legacy-sqlite`; `mother` denotes the native executable. Both are built from the same source, and the default Cargo build enables `legacy-sqlite`. Keep the same operation UUID and journal for retries. Preparation creates a fenced destination and does not activate it.

```powershell
$privateRoot = 'C:\Users\OWNER\AppData\Local\MOTHER'
$operationId = [guid]::NewGuid().ToString()
$source = Join-Path $privateRoot 'context.sqlite'
$destination = Join-Path $privateRoot 'garden.redb'
$journal = Join-Path $privateRoot 'cutover'
$nativeConfig = Join-Path $privateRoot 'client.native.private.json'
$endpoint = Join-Path $privateRoot 'garden-service.private.json'
$serviceConfig = Join-Path $privateRoot 'client.service.private.json'

mother-migration garden migrate prepare --source $source --db $destination --journal $journal --operation-id $operationId
mother garden migrate status --journal $journal
# Review the report, obtain live-cutover approval, then stop and close every source writer.
mother-migration garden migrate activate --journal $journal --operation-id $operationId
mother garden migrate reissue --journal $journal --client EXISTING_CLIENT_ID --out $nativeConfig
# Start the owner service in its own terminal; it remains in the foreground.
mother garden service --db $destination --endpoint $endpoint
```

With that service running, create the bridge configuration from another terminal, then explicitly configure the intended harness to launch `mother mcp --config` with that service configuration. Configuration creation does not edit harness settings or prove host adoption.

```powershell
$privateRoot = 'C:\Users\OWNER\AppData\Local\MOTHER'
$nativeConfig = Join-Path $privateRoot 'client.native.private.json'
$endpoint = Join-Path $privateRoot 'garden-service.private.json'
$serviceConfig = Join-Path $privateRoot 'client.service.private.json'
mother garden service-config --config $nativeConfig --endpoint $endpoint --out $serviceConfig
mother mcp --config $serviceConfig
```

Before-write rollback uses `mother garden migrate rollback --journal $journal --operation-id $operationId` after stopping the destination service and all destination writers. If a command's outcome is uncertain, inspect `status` and retry only its identical journal, operation UUID and output path. Reissuance never prints a token; its private output directory must exclude broad read access. Preparation and activation return `LEGACY_UNAVAILABLE` in the native build.

These checks establish synthetic cutover behavior. They do not prove that a real host loaded new credentials, that old processes stopped, or that a live migration is authorized. Never open the archive with an older binary as another active endpoint.
