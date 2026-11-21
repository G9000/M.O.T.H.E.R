# Garden full backups

Garden full backups are credential-free recovery bundles for the explicit redb backend. They preserve graph current records, immutable revisions, receipts, supersession mappings, reviewed-capture proposals and reviews, opaque extension payloads, and the enabled native v1 compatibility state. Native v1 state includes persona, records, retained receipts, inbox, pins and task assignments.

The SQLite runtime is removed. The bounded native v1 JSON export remains a compatibility format and refuses graph state that it cannot represent with `FULL_BACKUP_REQUIRED`. Full backup does not activate a new endpoint, migrate a live workspace or change any harness configuration.

## Owner commands

```powershell
mother garden blob --db "C:\private-data\garden.redb" --file "C:\private-data\selected-artifact.bin"
mother garden backup --db "C:\private-data\garden.redb" --out "C:\private-backups\garden-bundle"
mother garden verify --backup "C:\private-backups\garden-bundle"
mother garden restore --backup "C:\private-backups\garden-bundle" --db "C:\private-data\restored.redb"
```

Stop the owner service before direct database commands, then restart it afterward. Blob ingestion is needed only for content-addressed artifact references; URI references do not fetch external bytes. Backup output must be a new directory, restore output must be a nonexistent file, and their parent directories must exist. Keep bundles outside Git: they contain private context and are not encrypted. Restored credentials must be issued explicitly before a harness can read the restored workspace.

## Bundle format

`GardenStore::backup_to` creates a new private directory. One read transaction covers validation and every exported row, so concurrent commits do not mix different canonical snapshots. The manifest is written after the chunks. A failed or interrupted bundle is rejected when its files, manifest or checksums are incomplete.

The format consists of `manifest.json` and numbered JSONL chunk files. The manifest records format and required-extension versions, workspace identity and generation, per-section row totals, and each chunk's section, byte length, row count and SHA-256. Each row contains its canonical key and a hexadecimal encoding of the stored value bytes. This preserves unknown optional graph extension payloads without interpreting them.

The current format bounds each encoded row to 4 MiB, each ordinary chunk to 8 MiB and the restore manifest to 16 MiB. It has no 64 MiB total-data limit. Validation reads canonical rows individually and uses temporary on-disk indexes for receipt uniqueness and supersession provenance. Restoring and verifying need temporary disk space in addition to the bundle; verification builds and discards a staged database.

Derived query indexes are rebuilt from canonical revisions. Vector data, pagination leases and cursors, live grant tables, plaintext tokens and proposal credential snapshots are excluded. Historical proposer identity and subject attribution remain; that attribution grants no access in the restored workspace.

## Artifact bytes

Images have their own owner commands, provenance and verification; see [visual memory](visual-memory.md). Bundles include media assets, append-only acquisition records and derivative links, and restore validates them with the blobs they describe. Bundles written before media support omit those sections and still restore.

Blob ingestion is an owner operation. `GardenStore::put_blob_file` reads the file path explicitly supplied by the owner; MCP drafts cannot request a filesystem read. Bytes are hashed and stored in 1 MiB chunks inside the database transaction.

A backup includes ingested blob metadata and bytes. Every artifact source referenced by current or historical graph records, write receipts, pending proposals or reviews must have matching bytes, size and SHA-256. A missing historical or pending-only artifact causes backup or restore to fail. URI sources remain pointers: backup never fetches a URI or treats arbitrary opaque extension text as an artifact reference.

## Verification and restore

`GardenStore::verify_backup` checks a bundle without publishing a destination. `GardenStore::restore_backup` requires a fresh destination file and stages the database in an owned temporary directory beside it. Validation checks the manifest, exact file inventory, row counts and hashes, supported versions, canonical references, revision and receipt consistency, proposal provenance and blob content before publication. Paths containing symbolic links or Windows reparse points are rejected.

The restored workspace keeps its workspace ID and receives a new generation. Original receipt generations remain auditable; old operation requests cannot be replayed, and old pending proposals cannot be approved into the restored generation. Grant credentials are absent and must be explicitly reissued by the owner.

After validation and durable staging, restore publishes the database using a create-new hard link. The destination filesystem and process permissions must support creating hard links. An existing destination is never overwritten. The source database and bundle remain unchanged, and restore runs no external actions. Invalid bundles and failures before publication leave no destination; interrupted staging may leave an owned temporary directory for owner inspection. An error after publication can leave the complete destination in place. Inspect that destination before retrying; do not overwrite or delete it as an automatic recovery step.

Full backup recovery is distinct from endpoint cutover. It does not edit the original endpoint or harness settings. Any switch to a restored native generation requires separately reviewed credentials and host configuration; the removed SQLite converter/activation workflow is unavailable. Tests of synthetic backups establish local recovery behavior, not acceptance in a real harness.
