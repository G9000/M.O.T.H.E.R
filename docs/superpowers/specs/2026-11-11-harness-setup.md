# Harness setup and session briefing

**Problem.** Her context reaches any harness that connects, but she is only present if the model decides to call `mother_get_context`, and each harness needs hand-assembled MCP configuration.

**Decision.** `mother brief` prints a bounded plain-text briefing for session-start hooks and custom system prompts. `mother connection --format` prints paste-ready setup for Claude Code (MCP registration plus a SessionStart hook running the briefing), Hermes, OpenClaw and generic `mcpServers` clients.

**Unchanged.** The store, MCP tools, grants and existing `json`/`codex` output. Nothing edits a harness's settings.

**Not included.** Writing harness config files, startup hooks for harnesses without a documented hook mechanism, and verification inside each real harness.

**Acceptance.** `tests/harness_setup.rs`.
