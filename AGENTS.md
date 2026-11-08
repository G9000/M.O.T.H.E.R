# M.O.T.H.E.R contributor guide

This is the owner's personal workspace, independent of AnimaOS. Keep the README limited to its approved title, poem and artwork. Put technical instructions in docs/.

Use Bun and the locked dependencies. Run `bun test` and `bun run typecheck` for implementation changes. Tests use synthetic context and owned temporary directories, which they remove after closing all connections.

Keep personal records, tokens, private exports and runtime databases out of Git. The default data directory is outside this repository. Do not import real conversations, modify live harness settings, or start external actions as a test.

The domain contract is src/contracts.ts. All MCP mutations go through Store.write and its shared SQLite transaction. Preserve atomic revision checks and operation receipts. Never bypass grants or write records directly from a new adapter. The owner CLI's initialization, grants and restore operations are local administrative controls, not exposed MCP tools.

Preserve record IDs, source attribution, corrections, retractions and uncertain action outcomes. A restore must preserve logical identity, change generation, exclude credentials and never execute actions. Validate imports before creating a fresh destination.

SOUL.md is the initial persona seed. Existing workspaces use their stored persona snapshot. Harnesses retain their own model, permissions and instruction hierarchy. Do not claim full transcript synchronization or live host compatibility from protocol-only tests.
