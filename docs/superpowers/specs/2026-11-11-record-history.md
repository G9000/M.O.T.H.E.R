# Record history

**Problem.** Every write keeps a receipt with the full record it produced, but no tool can read an earlier revision, so a correction erases what was there before from a harness's point of view.

**Decision.** A read-only MCP tool, `mother_get_record_history`, lists one record's revisions newest first (actor, time, status, excerpt, changed fields) or returns one revision in full. It requires the read scope and full-context access; shared-access clients are refused.

**Unchanged.** The v1 schema, record and receipt shapes, export format, write paths, and the AnimaOS required-tool list.

**Not included.** History across records, time-range queries, diffs of field contents, and an index on receipt record IDs.

**Acceptance.** `tests/history.rs` and the history stdio test in `tests/mcp.rs`.
