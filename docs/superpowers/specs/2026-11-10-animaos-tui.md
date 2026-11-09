# AnimaOS TUI connection setup

Owner approved adding AnimaOS connection setup directly to M.O.T.H.E.R's native TUI. MOTHER remains a standalone context layer. Optional adapter only talks to the running AnimaOS owner HTTP API. No dependency on AnimaOS crates or source checkout; no direct edits to its database or credentials.

Flow: home AnimaOS entry → loopback daemon address and masked owner admin token and optional masked API key → list/select agent → review exact local MOTHER process/access → Connect. TUI creates/reuses a separate read,context:propose private grant; posts a disabled stdio connection, enables it with current version, starts and discovers tools. Existing matching connection is reconciled first; mismatches or uncertain state are surfaced, never overwritten or blindly replayed. Tool-policy review remains owned by AnimaOS. Report discovery truthfully; conversation acceptance stays unverified until a real host call.

Credentials stay only in zeroizing session memory; never log remote response bodies or follow redirects. Loopback-only bounded HTTP requests; no shell launch or remote endpoints. Per-agent locking, scope/workspace validation, bounded errors and tests for duplicate/version/unknown effects. Background tasks keep terminal responsive. Confirmation is explicit before grant or server mutations. Preserve README art/style and all memory/MCP behavior.

Plan:
- [x] Implement optional binary adapter src/animaos.rs with synthetic HTTP/store tests first.
- [x] Implement src/tui/animaos.rs controller/view, home entry and connect animaos shortcut. Test input, secrets, confirmation and discovery status.
- [x] Document setup and owner authentication requirements honestly.
- [x] Independent review, fmt, full locked release tests, release build and strict Clippy. No install, real grant, live AnimaOS mutation, commit/merge/push as part of automated testing.

Validation: 138 tests passed, two existing optional embedding-model tests ignored; fmt, strict Clippy and release build passed. Final automatic-policy fixture strengthened to match actual needs_review state; all 17 adapter/setup tests passed again. Independent review approved. Native PTY smoke confirmed home entry, setup screen, masked synthetic credential input and clean cancellation with no private state created. No live owner connection or conversation acceptance, installation, commit, merge or push performed.
