# Validation — owner-supplied SOUL v0.3.1, 9 November 2026

The executable no longer embeds the maintainer's persona. New workspaces load an explicit `init --soul` path or SOUL.md beside the selected database; missing or invalid files do not fall back to a repository persona. Existing stored snapshots remain unchanged.

- `cargo test --locked`: 57 passed; 2 optional real-model tests excluded from this offline run.
- `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo build --release --locked`, and `cargo doc --no-deps --locked`: passed.
- Both actual-model release fixtures passed again before integration: semantic retrieval acceptance and MCP restart/cache reuse.
- Regression checks cover explicit selection, database-relative selection, unrelated working-directory files, missing/invalid/oversized/blank/BOM-only text, snapshot stability, and persona export/restore.
- Review's BOM-only validation finding was reproduced, fixed and verified. No new retrieval algorithm changes were made.

Installed the verified 0.3.1 binary in a new private version directory after backing up state/configuration. Its bytes do not contain the old personal persona phrase; the personal SOUL.md file remains unchanged. Reads through the previous and new installed binaries returned equal context after excluding capture timestamps. User PATH selects 0.3.1 for new terminals and the Codex launcher selects it on the next server start. The current live chat continues using the verified 0.3.0 process; no second restart is required to preserve existing context because this correction changes initialization only.

---

# Validation — local semantic context v0.3.0, 9 November 2026

Environment: Windows, Rust 1.93.0, Candle 0.9.2, tokenizers 0.22.2, pinned all-MiniLM-L6-v2 CPU model. Exact dependencies are locked in Cargo.lock.

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked`: 54 passed, 2 optional real-model tests ignored in this offline run.
- `cargo clippy --locked --all-targets --all-features -- -D warnings`: passed.
- `cargo build --release --locked`: passed.
- `cargo doc --no-deps --locked`: passed.
- Both opt-in release tests documented in [semantic search](semantic-search.md) passed using the actual verified local model, not injected vectors.

The real-model fixture retrieved all four expected records from paraphrased queries, including a clause after a long unrelated prefix with the relevant written-notice sentence in the returned contextual snippet. It also checked an irrelevant query, exact identifiers, source offsets, corrections/retractions, zero-token text, cache reuse, CLI access and revocation. The separate stdio fixture verified semantic MCP retrieval and persisted-cache reuse after restarting the process. In one local release run, preparing 28 passages took about 692 ms and cached queries took 12–13 ms; these small synthetic timings are not a general performance guarantee.

Offline tests additionally cover stale revisions and restore generations, concurrent cache profile changes, model unavailability/busy fallback, corrupt cache vectors/offsets, explicit capacity/partial coverage and existing writes/receipts/export behavior. Model download was explicit and verified against pinned hashes; no private context was sent to an embedding service. The README, artwork and SOUL.md remain unchanged.

These are functional acceptance fixtures, not a broad relevance benchmark. The initial model is English-focused; coverage and similarity limitations are documented. No third-party memory engine, automatic extraction or history import was added.

## Installed private instance

The verified 0.3.0 executable was installed in a versioned private directory alongside the running 0.2.0 binary. SQLite, client configuration, Codex configuration, prepared launcher and user PATH were backed up privately. Only the MOTHER command in Codex configuration changed; other settings and existing grants were preserved.

The installed CLI and a fresh installed MCP subprocess both retrieved the existing setup checkpoint semantically. The MCP process advertised all six tools and the optional search mode, reused the CLI-built cache, and read the unchanged current revision. Canonical exports before and after retrieval were equal after excluding the export timestamp; the client credential file was unchanged. These checks did not write synthetic records into the personal workspace.

A broad question about an assistant remembering work between sessions returned no hit at the unchanged 0.35 threshold. Two more specific questions about Codex setup retrieved the checkpoint. This is recorded as a recall limitation, not hidden by lowering the threshold. After the owner restarted the server, the current Codex chat advertised the new search modes and directly retrieved the setup checkpoint with `semanticUsed: true`. This verifies the live 0.3.0 chat connection separately from subprocess checks.

---

# Historical validation — Rust v0.2.0, 9 November 2026

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
