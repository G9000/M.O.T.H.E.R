# Existing workspace persona updates

Owner request: allow updating SOUL.md for an existing workspace while keeping M.O.T.H.E.R an independent context layer.

Provide owner CLI `mother soul show [--db]` returning workspace identity and the current persona/content hash, and `mother soul update --file <path> --expected-hash <sha256> [--db]`. Read the source once with existing bounded UTF-8/nonblank validation. Validate at the Store boundary too. In an IMMEDIATE SQLite transaction, compare the current hash, replace only persona metadata and return previous/new hashes plus changed status. Same content/current hash is a no-op; a stale hash is a conflict even for a retry. After uncertain responses, show current state and reconcile; do not promise operation receipts or update history.

Retain schema v1, workspace identity/generation, grants, records, existing mutation receipts and exports. Already-connected MCP clients read the new snapshot on their next call; source files are not watched. The update is owner administration, not a new MCP mutation capability. Search's final snapshot check already includes persona and must reject stale in-flight results via current literal fallback. Export/restore carries the updated snapshot.

Alternative automatic file reload would introduce path dependence and uncontrolled concurrent updates. Explicit import is the chosen scope; OS file access remains the owner authority boundary. Existing exports are the rollback mechanism; no automatic backups/history subsystem is added.

Implement with regression-first CLI tests, store concurrency/invalid-data/preservation tests, an already-connected two-client MCP test and a search freshness test. Run required repository checks and install a new versioned binary without changing the real owner's persona, since no new personal content was supplied. Do not merge/push this follow-up without a new request.

## Outcome

Implemented, reviewed and verified: 62 offline tests, both actual-model fixtures, format/lint/release/docs checks passed. Installed 0.3.2 and verified an existing 0.3.0 MCP process observes synthetic updates without restart. The real owner persona was unchanged.
