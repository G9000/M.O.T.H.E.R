# Owner-supplied workspace persona

The owner corrected the design: M.O.T.H.E.R must be usable by other people with their own SOUL.md, not an embedded copy of the maintainer's personality.

Use `mother init --soul <path>` to select UTF-8 text, or default to SOUL.md beside the selected database. Do not search the working directory or compile a fallback. Keep the existing immutable-at-runtime SQLite persona snapshot and v1 export format so every harness sees the same persona and existing instances remain unchanged. A neutral editable example is available separately; it is never silently selected.

An alternative would reload a live file on every context call. That would introduce path dependence across machines and require new update/concurrency/export semantics, outside this initialization correction. Existing workspace persona editing remains a separate feature.

Validate missing/invalid/blank/oversized files, explicit selection, database-relative default selection, immunity to an unrelated cwd persona, snapshot stability and export/restore. Preserve README, personal SOUL.md and canonical private state. Run the independent repository's required checks. Version the installed executable rather than replacing a running server.

## Completion

Implemented and reviewed. 57 offline tests, required format/lint/build/docs checks and the real-model MCP restart fixture passed. Versioned 0.3.1 installed; existing personal context compared equal. Live 0.3.0 semantic connection had already been verified after the owner restart. The initialization correction will be present when the server next starts; it does not change existing workspace snapshots.
