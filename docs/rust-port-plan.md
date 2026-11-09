# Rust port plan

Preserve the personal workspace and its v1 storage/wire contract while replacing the Bun runtime with a standalone Rust program. No AnimaOS dependencies. README, artwork, SOUL seed, live records, grants, identities and generation remain intact.

- [x] Characterize all 21 original tests; capture synthetic v1 database/export/operation and tool-schema fixtures before replacing TypeScript.
- [x] Build an independent Cargo package (rusqlite, official rmcp SDK). Port validation, SQLite transactions, canonical request hashes, grants, context, receipts, export/restore and capacity accounting.
- [x] Port CLI and the six stdio MCP tools. Same client JSON configs and flags, with `mother mcp --config ...` as the native launcher. Preserve machine-readable outputs and structured tool errors.
- [x] Run original tests against the original implementation and differential/protocol checks against Rust. Add native test equivalents, including two-process races, revoked live clients, restore interruption, clock rollback and v1 replay compatibility.
- [x] Run cargo fmt/check/build/test/clippy. Update AGENTS/setup/validation with API/test mapping and documented deliberate differences. Remove production TypeScript and Bun dependencies after parity checks.
- [x] Verify the existing private instance read-only, retain an external credential-free recovery export, install the tested executable privately and regenerate only the prepared connection file. Do not change live harness settings or database generation. Prepare the verified Rust port for a local signed commit.

Compatibility details: keep user_version 1, SQL tables/columns, camelCase JSON and statuses, UTF-16 size limits, default arrays, millisecond UTC timestamps, and SHA-256 over the normalized sorted-key request. Usage deltas use actual stored JSON byte lengths to avoid drift across serialization languages. Restore changes generation; ordinary runtime replacement must not.
