# The hearth, stage 1: owner-triggered launches

**Problem.** Handoffs let one harness leave work for another, but nothing starts the receiving harness; the owner has to open it by hand.

**Decision.** The owner declares launchers in a private `hearth/launchers.json` (absolute command, fixed arguments with `{prompt}`/`{task}`, optional working folder, timeout; no shell). `mother launch <task> --launcher <name>` hands the task to that launcher's harness, records a pending `action` by `@owner-launch`, runs the command with output to a private log, and records `succeeded`, `failed` or `unknown` (timeout). The prompt names the task by ID only. An unresolved launch blocks another launch of the same task until `--retry-unknown`.

**Unchanged.** MCP clients cannot launch or choose commands. Record, receipt and table shapes; each harness keeps its own grant and permissions.

**Not included.** Her own model or loop, proposals for approval, budgets and level-3 autonomy, detached runs, a TUI key, and editing launchers from M.O.T.H.E.R.

**Acceptance.** `tests/launch.rs`.
