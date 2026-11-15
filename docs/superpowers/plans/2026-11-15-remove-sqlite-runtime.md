# Remove the obsolete SQLite runtime

> **For agentic workers:** Use subagent-driven-development for independent library, test and review tasks. Root owns Cargo, entry points, docs and integration. Preserve existing G9 and AnimaOS changes.

**Goal:** Make the native temporal garden the single Mother runtime, with no SQLite feature, backend, fallback or converter dependency.

**Design:** The owner's explicit removal request replaces the earlier dual-runtime policy. Keep v1 MCP contracts through the native compatibility view, graph storage, native service, receipts, grants, backups and native cutover fences. Remove the SQLite store and old CLI/app dashboard/onboarding implementation and SQLite-source conversion commands. Preserve independent native graph and context owner-review screens with their pure Rust terminal dependencies; explicit inbox commands open them in a terminal and emit JSON when piped. Existing records, credentials, live settings and historical benchmark evidence remain untouched. Unsupported old configurations fail explicitly; they never create or migrate a store implicitly. Native init/grant/service/review/backup commands remain available.

**Alternatives:** Merely changing the default leaves an obsolete backend and duplicate tests; keeping a feature preserves SQLite dependencies. Remove the implementation and feature completely. Historical v1 data schemas and evidence can remain as archival artifacts, with current docs stating their status.

- [x] Freeze the previous full-graph qualification manifest/source inventory; add a failing native-only default/dependency/runtime-boundary regression.
- [x] Remove obsolete library branches/modules and conversion-only source while preserving native context APIs, graph authority and cutover fences.
- [x] Make native CLI/MCP the sole entry points; remove Cargo feature/dependencies and update the locked graph offline.
- [x] Retire SQLite-only tests and retain/adapt native G9 compatibility, service, graph, backup and real AnimaOS fixtures.
- [x] Update current contributor/setup/runtime docs while preserving historical measurements and manifests.
- [x] Run package format, tests, release, strict all-target/all-feature Clippy, docs, native dependency audit and actual native AnimaOS interoperability; independently review removals and retained invariants.

No database deletion, live migration, installation, settings update, commit or push is included.

**Qualified:** default package 211 passed/zero failed/five ignored; six native recovery cases passed; both actual native AnimaOS HTTP fixtures and all seven graph executor calls passed. Format, strict Clippy, release, warning-free Rustdoc and macOS all-target cross-compile passed. Dependency pruning removed 105 locked packages with zero additions/upgrades. Twenty-seven native G9 source files and all AnimaOS source hashes match the frozen full-graph manifest. Independent review found no remaining removal blockers; six restored native journal cases close its identified rollback/reissuance coverage gap.

Current report: [native-only qualification](../../benchmarks/native-only-runtime.md). Original G9 performance/stress/platform holds remain release gates.
