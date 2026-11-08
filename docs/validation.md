# Validation — Rust v0.2.0, 9 November 2026

Environment: Windows, rustc/cargo 1.93.0, rmcp 3.9.0, rusqlite 0.41.1 with bundled SQLite. Exact dependency versions are in Cargo.lock.

## Native checks

- `cargo test --locked`: 28 tests passed, zero failures. The process-crash helper is included in the count.
- `cargo clippy --locked --all-targets --all-features -- -D warnings`: passed.
- `cargo build --release --locked`: passed; standalone Windows executable built.
- `cargo fmt --all -- --check`: passed.
- `cargo doc --no-deps --locked`: passed.

The native suite maps every original test and exercises database persistence, corrections, retractions, grants/revocation, bounded context, references, safe retries, two-process revision/duplicate-operation races, interrupted restore, clock rollback, portable capacity, exclusive file creation and owner CLI workflows. Extra coverage handles numeric normalization, Unicode search, UUID validation and malformed tool inputs.

## Cross-runtime evidence

Before removal, the TypeScript baseline at 6f6e0cc passed all 21 tests. Its original three MCP tests then passed against the Rust server through the official TypeScript SDK. No raw transcript or live personal context was used.

The synthetic v1 SQL fixture preserves the exact original grant hash, workspace identity, records and operation receipts. Rust opened it without migration and replayed all seven original requests, including a request whose record had subsequently been corrected. In a separate differential check, TypeScript replayed a Rust-created operation and both runtimes produced semantically equal exports. The fixture is retained for native regression testing.

## Existing private instance

A private SQLite recovery backup and client/launcher copies were made outside Git before installation. The tested Rust release executable was installed in the existing private M.O.T.H.E.R data folder. A real stdio client used the existing credential file to initialize MCP, list all six tools and retrieve context. A credential-free continuity export was also saved privately.

Logical hashes over every row of metadata, clients, records and operations, plus user_version, were identical before and after the read-only checks. The client JSON file's hash was unchanged. The prepared Codex connection entry now launches the Rust executable. No workspace or grant was recreated and no personal facts were imported.

## Limits

This proves local protocol and storage compatibility. It does not prove a conversation in a live Codex/Claude/AnimaOS host. No existing M.O.T.H.E.R entry was present in the inspected Codex configuration, and no live harness settings were changed. Remote HTTP/OAuth deployment, web ChatGPT integration, cross-machine synchronization and full transcript ingestion remain unsupported. MCP metadata/Unicode edge differences are documented in PORTING.md.
