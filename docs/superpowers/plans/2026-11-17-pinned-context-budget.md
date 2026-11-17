# Pinned context budget fix

Goal: keep context readable when the supported pinned set exceeds the 12 KiB presentation budget, without changing canonical memories, pins, persona or permissions.

Design: preserve complete pin metadata and all pin entries. Remove ordinary navigation previews using the existing policy first. If necessary, reduce the largest pinned content excerpts until the serialized context fits. Mark affected entries `truncated: true` and report budget compaction with instructions to use `mother_get_record`. Never summarize or rewrite canonical text. Preserve explicit size errors when required non-pin context or metadata cannot fit. Full-record reads remain unchanged.

Alternatives: increasing transport limits risks host rejection; a fixed small excerpt wastes available space and still fails for multilingual/escaped text. Adaptive byte-aware presentation addresses the actual response size.

- [x] Reproduce three-pin and twelve-pin failures with synthetic UTF-8/escaped content; check identity, attribution, complete retrieval and transport size.
- [x] Implement deterministic pinned excerpt compaction in `src/mcp/read_output.rs` and update advertised descriptions/docs.
- [x] Run focused regressions, formatting, locked tests, release build, Clippy and normal/build dependency audit.
- [x] Review diff and record validation. Do not install, restart services or alter live pins as a test.

Validation: Windows, 250 tests passed and six intentionally ignored. Formatting, release build and strict all-target/all-feature Clippy passed; normal/build tree contains neither SQLite crate. Used isolated Cargo artifacts after shared artifacts listed stale tests. The default temporary directory failed existing private-path ACL checks, so the full successful suite used a newly created owner-private synthetic temporary root. Existing folders and ACLs were untouched. Read-only independent review found no actionable issues. No installation or live-host acceptance performed.

Publication qualification: an isolated checkout containing only this fix passed 243 tests with six intentional ignores, formatting, release build and strict Clippy. The earlier 250-test working-tree run included concurrent evaluation work. Subsequent separately authorized installation and ChatGPT bridge restart preserved existing credentials and canonical state; actual app context and full pinned-record readback succeeded. No live repinning test was performed.
