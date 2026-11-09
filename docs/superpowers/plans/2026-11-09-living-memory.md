# Living Memory Implementation Plan

> Use superpowers:subagent-driven-development with bounded implementation ownership and independent review. The owner already approved implementation; do not add another permission gate for routine choices.

**Goal:** Explicit MCP capture into a durable TUI review inbox, isolated from active context until approval.
**Architecture:** Additive SQLite inbox extension, canonical transactional writer reuse, local owner review, existing Rust terminal.
**Tech Stack:** Rust, rusqlite, rmcp, Ratatui, existing locked dependencies.

- [x] Storage and protocol: tests first in tests/inbox.rs and tests/mcp.rs; implement src/inbox.rs, src/store.rs transaction helper, src/contracts.rs scope, src/portability.rs extension, src/mcp.rs and contracts/mcp-tools.json tools. Confirm isolation, revision conflicts, atomic approval, attribution, grants, idempotence, duplication, bounds and recovery. Keep tests synthetic. Run cargo test --locked --test inbox and --test mcp. Inspect failures before implementation.
- [x] TUI: tests first in src/tui/inbox.rs or src/tui/tests.rs; implement review controller, scrollable source/before/after, in-terminal editing, save/dismiss confirmation, stable retry requests and refresh. Wire home and mother inbox shortcut via src/tui.rs, src/tui/view.rs and src/cli.rs. Use synthetic temp DB and TestBackend. Run cargo test --locked --bin mother.
- [x] Documentation: docs/living-memory.md, docs/terminal.md and docs/setup.md describe capture prompt, grants, upgrade/reload, review, conflicts and recovery; preserve README style and existing artwork/poem. Update version only if required for separate install; no install in this task.
- [x] Review and validation: separate spec and quality reviews; run cargo fmt --all -- --check, cargo test --locked, cargo build --release --locked, cargo clippy --locked --all-targets --all-features -- -D warnings. Record synthetic cross-harness workflow evidence and known live-host limitations; leave branch ready for user-requested commit/merge.

Use existing absolute target C:/Users/leoca/OneDrive/Desktop/M.O.T.H.E.R/target for sequential Cargo checks to reuse dependencies. Do not terminate existing terminals. If a running debug binary locks output, use release tests and record the constraint. No edits to AnimaOS.

Validation completed: 119 tests passed, 2 optional embedding-model tests ignored; fmt, release build and strict Clippy passed. Native PTY smoke queued a synthetic MCP proposal, opened/reviewed/edited/saved it in the TUI, then exported and verified one approved owner-attributed record and the closed proposal. Independent specification and final quality reviews approved. Existing debug executable was locked by a running app; full tests ran with --release against the same locked dependencies. No install, live harness restart, commit, merge or push performed.
