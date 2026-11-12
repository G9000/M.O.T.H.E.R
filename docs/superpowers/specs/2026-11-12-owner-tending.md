# Owner tending in the terminal

**Problem.** Pins, handoffs and history could only be seen or changed through a harness. The owner had no direct view of the pinned core.

**Decision.** Owner-only commands (`records`, `pins`, `pin`, `unpin`, `handoffs`, `assign`, `history`) and a terminal screen (`mother tend`, or **p** at home) that lists records by kind, pins and unpins, and hands open tasks to harnesses. They reuse the harness rules and record `@owner` as the actor. None of it is reachable from MCP.

**Unchanged.** MCP tools, grants, schema and record shapes.

**Not included.** Editing or retracting records from the screen, search inside the screen, launching from the screen, and history inside the screen.

**Acceptance.** `tests/tend.rs`, `tests/tend_cli.rs` and `src/tui/tend/tests.rs`.
