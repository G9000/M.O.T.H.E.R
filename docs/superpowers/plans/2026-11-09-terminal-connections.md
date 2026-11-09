# Terminal connection implementation plan

> **For agentic workers:** Use superpowers:subagent-driven-development for bounded implementation tasks and review.

**Goal:** Native terminal setup and lifecycle for M.O.T.H.E.R's private ChatGPT connection.
**Architecture:** Binary-only connection adapter, shared CLI/wizard functions and Ratatui view; existing library/MCP unchanged.
**Tech Stack:** Rust, Ratatui/Crossterm, keyring, official tunnel-client.

- [x] Review design and API contract before implementation.
- [x] Add failing behavioral tests for non-TTY commands and new interface.
- [x] Implement src/connection.rs and src/connection/*: safe settings, vault, bounded subprocesses, typed status and optional Windows startup. Unit tests use synthetic runners/vault.
- [x] Implement src/tui.rs and src/tui/*: terminal lifecycle, dashboard, masked guided configuration, status/connection actions and TestBackend tests.
- [x] Integrate CLI commands and dependencies in src/main.rs, src/cli.rs, Cargo.toml/Cargo.lock; preserve old commands.
- [x] Update docs/setup.md, docs/chatgpt.md and validation evidence; README poem/art remain intact.
- [x] Review spec compliance and code quality; run fmt, cargo test --locked, cargo build --release --locked, cargo clippy --locked --all-targets --all-features -- -D warnings. Verify actual terminal where tools allow. Publish only when requested.
