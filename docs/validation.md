# Validation — 9 November 2026

Environment: Windows, Bun 1.4.2, TypeScript 5.9.3, MCP TypeScript SDK 1.32.2, Zod 4.6.7. Dependency versions are locked in bun.lock.

## Passed

- `bun test`: 21 tests, 107 assertions, no failures.
- `bun run typecheck`: passed.
- Real stdio MCP clients in separate subprocesses: same workspace, attributed memory/task/checkpoint writes, correction, task completion and restart persistence.
- Two-process revision race: exactly one success and one structured CONFLICT. Both restarted clients retrieve the winning revision and matching receipt.
- Two-process duplicate-operation race: one record, identical receipts.
- Read-only denial, wrong-token denial and revocation enforced on an already connected client.
- Export/restore after the original database is removed: initialized persona, record IDs/revisions, embedded text artifact and unknown action outcome survive. Old grants fail, new generation rejects stale writes.
- Malformed exports, duplicate IDs, missing references, altered persona digest and existing destinations are rejected.
- Process exit during restore import: destination cannot open as a valid empty workspace.
- Clock rollback: corrected records remain readable/exportable.
- Record-capacity boundary: new writes fail before commit, current records remain exportable, guarded updates within capacity still work.
- CLI initialization, grants, connection output, context inspection, export, restore and revocation; invalid flags/names fail without unwanted initialization.
- Git ignore checks cover private JSON configs/exports, SQLite and sidecars, environment files, and private/export folders.
- README and original artwork unchanged.

An empty personal instance was initialized outside the checkout, with a private Codex client config. Its generated connection entry was parsed and used to start a real MCP client: six tools were discovered and context retrieval returned zero personal records. No personal history was imported. Live identifiers and credentials are not recorded here.

## Limits of this evidence

These are local protocol and CLI tests, not a conversation in an installed Codex, Claude, ChatGPT or AnimaOS host. No live host configuration was modified. Remote HTTP/OAuth deployment, web ChatGPT integration, cross-machine synchronization, full transcript ingestion and a personal pilot are not implemented or tested. External action execution is outside this server.

The first release has a bounded portable format and refuses writes at its admission limit. Retractions retain historical receipts. The setup guide documents capacity, incomplete-restore recovery, credential reprovisioning, local filesystem trust and unencrypted private exports.
