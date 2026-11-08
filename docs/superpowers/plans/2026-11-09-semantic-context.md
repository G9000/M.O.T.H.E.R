# Semantic Context Implementation Plan

> For agentic workers: use superpowers:executing-plans to implement this plan task by task. Reviewers receive only the spec, plan and repository boundary.

**Goal:** Meaning-based retrieval of canonical M.O.T.H.E.R records through the existing MCP context boundary.
**Architecture:** Local Candle embedding model, rebuildable SQLite sidecar, bounded hybrid ranking with final canonical validation and explicit fallback. No agent execution or external memory service.
**Tech Stack:** Rust, Candle, tokenizers, bundled rusqlite, pinned HTTPS model downloads.

## Tasks

- [x] Add failing MCP/CLI tests for search mode and explicit unavailable-model fallback. Run cargo test --locked --test mcp and observe missing behavior.
- [x] Implement src/embedding.rs for pinned local model installation/status/loading, token windows, masked pooling/normalized vectors. Add dependencies in Cargo.toml and lock compatible versions. Model download is explicit and bounded; no network in inference.
- [x] Implement src/search.rs and focused cache helpers for authorized canonical snapshots, derived cache, bounded incremental indexing, ranking and fresh validation. Add unit/integration tests for revision/identity/cache/coverage failure modes and access checks.
- [x] Integrate existing six MCP tools in src/mcp.rs and add search/model/index commands in src/cli.rs. Update only the search schema in contracts/mcp-tools.json; keep v1 record and persistence contracts.
- [x] Run synthetic real-model tests, including no-overlap paraphrases, late long-record passage, exact identifier retention, changed/retracted records and cache reuse. Verify both CLI and MCP subprocesses; record observed limitations.
- [x] Run cargo fmt --all -- --check; cargo test --locked; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo build --release --locked; cargo doc --no-deps --locked. Have the implementation reviewed and resolve findings.
- [x] Update docs/setup.md, docs/design.md and docs/validation.md. Preserve README/artwork/SOUL. Back up private installation/config and install the verified version without overwriting a running executable. Verify live search/read compatibility and save a MOTHER checkpoint. Leave the branch reviewable; no merge/push unless requested.

## Delivery evidence

All implementation and offline/real-model checks passed; independent focused review found no remaining substantive issue. The installed 0.3.0 CLI and a fresh MCP process retrieved current private context without changing canonical state. The versioned installation, model and Codex launcher are ready. The owner restarted the Codex MCP connection; the live chat retrieved the setup checkpoint with semanticUsed true. A continuation checkpoint was saved through MCP.
