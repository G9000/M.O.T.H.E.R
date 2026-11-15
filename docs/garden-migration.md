# Historical cutover recovery

The native redb Garden is Mother's sole runtime. The SQLite runtime, read-only converter and SQLite-source `garden migrate prepare` / `activate` commands were removed. There is no migration-enabled feature or alternate build. Old SQLite configs fail explicitly; updating the executable never creates, imports or activates a replacement store. Keep existing private files and settings intact until a separately reviewed recovery decision.

## Retained journal operations

Existing cutover journals, source/destination fences and append-only transitions remain authoritative. The owner can inspect a prior journal, reissue a captured client's exact restrictions with fresh native credentials, or request an eligible before-write rollback:

```powershell
mother garden migrate status --journal "C:\private-data\mother\cutover"
mother garden migrate reissue --journal "C:\private-data\mother\cutover" --client EXISTING_CLIENT_ID --out "C:\private-data\mother\client.native.json"
mother garden migrate rollback --journal "C:\private-data\mother\cutover" --operation-id "<original-cutover-operation-uuid>"
```

These commands operate on existing journals; they cannot prepare a new SQLite source or complete an interrupted source activation. Do not delete fences, forge journal transitions or use raw database opens to make a prepared destination serve. A blocked or uncertain historical cutover requires explicit inspection and reconciliation.

Reissue publishes a new private output exclusively and never edits harness settings, prints a token or starts a service. Its restrictions come from the historical journal; reissue does not widen them. For an already active native destination, use [the local service](garden-service.md) with that credential.

Stop the destination owner service before rollback. Eligible rollback verifies the unchanged canonical destination and archived source bytes, persists the rollback transition before restoring the source path and clears only its own source fence. Grant reissue and disposable derived caches do not count as canonical changes; records, revisions, receipts, proposals/reviews, pins, assignments, persona and stored blobs do. If canonical destination state changed, rollback returns `RECONCILIATION_REQUIRED` and preserves the fence. Missing or inconsistent state fails closed. Rollback never discards acknowledged destination writes.

Restoring an archived source file does not make SQLite usable by the current executable. Do not reopen it as an additional active endpoint or assume a current client can use old SQLite credentials. Runtime removal does not authorize live rollback or settings changes.

For native recovery, use [full backups](garden-backups.md). Small `garden import-v1` is a bounded compatibility import into an explicitly initialized matching-identity empty view; it is not SQLite conversion, endpoint activation or a generic restore command. Native restore changes generation and requires new credentials.

If a journal operation has an uncertain outcome, inspect `status` and reconcile the original operation UUID and exact request/output path. A checkpoint or elapsed time does not authorize replay. [G8 validation](garden-cutover-validation.md) preserves historical converter/activation measurements; those commands describe the old source, not this executable.
