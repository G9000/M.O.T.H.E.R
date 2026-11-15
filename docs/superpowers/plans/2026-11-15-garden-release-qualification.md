# G9 acceptance and release qualification

> **For agentic workers:** Use superpowers:subagent-driven-development for the independent benchmark, workflow acceptance and documentation tasks. Root owns integration, all Cargo executions, platform evidence and final review.

**Goal:** Produce a reproducible, evidence-backed release assessment of the completed Garden runtime, preserving any failed or unavailable gates.

**Architecture:** Exercise the public canonical APIs and real local service/MCP processes with synthetic data. Keep extraction and final ranking in harnesses, and independently qualify local AnimaOS memory alongside additive sharing. Report storage, IPC, host, platform and scale outcomes separately; do not infer production readiness from unit tests.

**Tech stack:** Standalone Rust package, redb 4.6.0, existing MCP/local IPC, Windows and existing Ubuntu WSL toolchains, unchanged upstream CI.

**Current assessment: HOLD.** Checkmarks below record completed execution or
documentation, not passing release qualification. The [acceptance report](../../benchmarks/garden-acceptance.md)
retains failed durable-write latency, lexical recall, contention availability
and deadline-failed 1m stress. The [admission/audit follow-up](2026-11-15-garden-admission-audit-fixes.md)
now qualifies native context-only AnimaOS integration and a separate exact audit
of the retained 100k corpus within the existing memory guard. Original failures
remain historical evidence. The [full graph follow-up](2026-11-15-full-graph-anima-admission.md)
separately qualifies graph schemas and actual native tool execution in the
additive AnimaOS worktree. macOS runtime remains unqualified.

The checklists below preserve the original G9 execution results. Current source
and platform results are in the follow-up report/manifest, separate from the
original build and measurement artifacts.

## Prerequisite

- [x] Revalidate G8: default/native tests, format, strict Clippy and both release builds.
- [x] Commit and fast-forward G8 onto main, verify the merged tree, run integration tests and push origin/main at df6c6eb07337ab28a153f5c9decf27d63b34d8d1.
- [x] Create g9000/mother-garden-release from that pushed source. Preserve other worktrees and unrelated AnimaOS UI edits.

## Completed-engine benchmark

Files: examples/garden_acceptance.rs and docs/benchmarks/garden-acceptance.md, plus small synthetic measurement summaries under docs/benchmarks/garden-acceptance-evidence/.

- [x] Add a deterministic synthetic-only runner using the actual GardenStore public contracts, current authorization, Immediate durability and indexes, with no direct-table seed bypass.
- [x] Execute 10k/100k entities with five relationships each and bounded high-degree/history cases. Both 10k final audits completed; 100k timed workloads completed but the 12 GiB RSS guard killed its final audit (exit 137), so it remains unqualified.
- [x] Finish the separate 1m stress invocation and record its failed result: cooperative one-hour deadline, exit 1, 802,752 acknowledged entities/zero assertions/12,544 receipts, no later workloads or audit. Peak RSS 1,255,124,992 bytes, database 15,167,623,168 bytes. No capacity pass is inferred.
- [x] Report p50/p95/p99 exact, temporal/neighborhood, lexical and durable 32-assertion operations, reopened application-cache reads, reader/writer contention, RAM/disk and truncation. Label 100k checkpoint data pre-audit; mark cold-disk and service IPC costs unavailable rather than infer them.
- [x] Preserve failures and measured limits. Accepted budgets, durability and work limits remain unchanged; report the interrupted audit and retained progress rather than turn partial execution into a pass.

## Workflow acceptance

Files: tests/garden_acceptance.rs and the relevant existing AnimaOS provider test location, only if needed after inspection.

- [x] Exercise two synthetic MCP harnesses: selected proposal, owner approval, identical canonical ID/provenance in the other harness, unselected noise absent, uncertain retries and restart.
- [x] Cover temporal contradiction/evidence, extension preservation, current access/retraction/index freshness and full backup/restore with changed generation through the focused and package suites.
- [x] Locate the actual separate-worktree additive AnimaOS implementation and exercise native memory, provider online/offline/revoked behavior and agent isolation. Managed full suite: 5,950 passed/66 ignored; focused provider suite: 75 passed/two ignored, including the hard-kill publication regression. Explicit real Mother receipt interoperability: one passed. Legacy two-executable sharing passed twice in fresh profiles; no AnimaOS main integration or model/installed-host adoption is inferred.
- [x] Run the native owner-service AnimaOS fixture and retain its failed result. Fix and validate oversized discovery frames with bounded pages; the final actual run still fails first-descriptor schema admission (`SchemaRejected`, HTTP 503). Native interoperability remains unqualified and requires a focused follow-up.

## Platform and build qualification

- [x] Finish final isolated Windows checks after dependency isolation and pagination: 490 default passed/eight ignored, 159 native passed/three ignored, zero failures; format, both release builds, strict Clippy, warning-free Rustdoc and native macOS compile check pass.
- [x] Use an owned private Linux copy and Ubuntu WSL for runtime qualification. Final pagination-source logs show 499 default passed/eight ignored and 165 native passed/three ignored, zero failures; both builds, strict Clippy and warning-free Rustdoc pass. Linux rustfmt is unavailable; Windows checks identical-source formatting. No live records/configs or daemon changes.
- [x] Inspect unchanged CI run 43688011366 for the pushed G8 SHA: all jobs failed overall; Windows/Linux tests passed but Clippy failed, and macOS backup fixtures failed. Preserve those failures separately from G9 fixes.
- [x] Isolate the model downloader in default `legacy-sqlite`; native installation returns `MODEL_UNAVAILABLE` before I/O with a regression test. The 564-line native normal/build audit excludes SQLite, downloader/TLS/C crypto and legacy UI/keyring dependencies. Native all-target macOS x86-64 compilation passes after isolation; this is not macOS runtime acceptance.
- [x] Complete source/build hash aggregation and independent source review. The manifest verifies 145 identical source files across platforms and retains final binaries separately from the measured Linux runner. Measurements predate installer isolation and MCP paging; no uncaptured historical Windows binary hash is invented. Exact commands/hardware are recorded; CI automation is unchanged.

## Documentation and release assessment

Files: docs/context.md, docs/design.md, docs/living-memory.md, docs/semantic-search.md, docs/harnesses.md, docs/setup.md, docs/validation.md, docs/CHANGELOG.md and the acceptance report.

- [x] Reconcile current collector/keeper boundaries, optional persona, compatibility, service operation and explicit migration/rollback instructions, preserving historical milestone evidence.
- [x] Keep Release A additive integration notes separate from B/C native graph/runtime notes. Clearly separate proposed, admitted, approved, indexed and actually recalled knowledge.
- [x] Publish the current HOLD assessment with failed/unavailable/pending gates and exact reproduction commands; no scale, platform or interoperability pass is invented.
- [x] Complete independent compatibility/recovery, pagination and release-report review; verify all 145 source hashes and finalize exact stress evidence. Release assessment remains HOLD; execution checkmarks do not imply passing gates.

No release publication, version tag, installation, live migration, model download or live harness configuration change is authorized by qualification. Source fixes necessary to pass a demonstrated acceptance failure are in scope. Final evidence and pending release actions remain reviewable in this isolated branch.
