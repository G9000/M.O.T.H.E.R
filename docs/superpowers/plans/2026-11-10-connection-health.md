# Connection health execution plan

Approved goal: deploy shared-context counts, make connection verification visible in the native terminal and test the memory review/correction loop.

- [x] Inventory live clients/processes and back up installed executables/configuration metadata without secrets in output.
- [x] Implement src/health.rs, TUI selection/report screen and optional doctor CLI; bounded MCP protocol read with redacted metadata and guaranteed child cleanup. Tests first for successful counts, old server, invalid/revoked config, malformed response, timeouts, output bounds and zero side effects.
- [x] Root deploy context inventory safely, reconnect AnimaOS through versioned owner API and verify returned catalog and local context response.
- [x] Independently review health code and pass full Rust/doc checks. Install health build only after verification.
- [x] Exercise authorized synthetic capture → owner review → cross-client read → correction → reread. Use real host conversations where available and report missing acceptance explicitly. Keep operation UUIDs stable across uncertain writes, and retract synthetic canonical records after results are verified.
- [x] Save checked evidence and handoff state; retain any unresolved live acceptance steps.

Acceptance boundary: actual ChatGPT read/correction confirmed. AnimaOS discovery ended before a tool read in both approved prompts; Claude/Hermes model conversations remain unverified. The synthetic capture/correction loop completed through local MCP, native owner inbox and cross-client reads. Owner authorized committing the feature, merging to main and pushing to GitHub after validation.
