# Native additive admission and bounded audit fixes

The [next slice](2026-11-15-full-graph-anima-admission.md) separately qualifies
full graph schemas and native tool execution in AnimaOS. This checklist retains
the preceding context-only and audit qualification results.

> **For agentic workers:** Use superpowers:subagent-driven-development for the independent audit implementation and review. Root owns integration, all Cargo commands and qualification evidence. Preserve existing uncommitted G9 work.

**Goal:** Admit AnimaOS's native context-only provider and complete exact audit of the retained 100k synthetic graph within the existing 12 GiB process-memory guard.

**Architecture:** Native discovery selects its context or graph surface from authenticated current graph authority. Context-only credentials receive unchanged v1 schemas plus capability discovery; graph credentials retain full graph schemas and pagination. Canonical contracts, per-call authorization and AnimaOS safety budgets remain unchanged. Audit a consistent read transaction with streaming rows, exact point/range checks and cardinality proofs instead of whole-store heap copies.

**Tech stack:** Existing Rust/redb/MCP package, Windows isolated G9 Cargo target and the owned Ubuntu qualification copy.

## Scope and alternatives

Raising host schema budgets would broaden unrelated connectors. Reducing graph descriptors to permissive object shells would discard useful contracts. Choose credential-bound surfaces for additive context admission; full graph descriptors remain available to graph clients. Add explicit MCP object roots while preserving their existing constraints. This does not promise admission of full graph schemas into AnimaOS's narrower generic descriptor budget.

For audit, full in-memory reconstruction caused the measured 12 GiB failure. External scratch sorting would add temporary storage/recovery machinery. Choose streamed expected-entry lookups plus exact table lengths where record ID/revision makes memberships disjoint, ordered history checks, per-sequence receipt ranges and fixed-size receipt uniqueness windows. Preserve every previous corruption invariant.

## Tasks

- [x] Preserve the original G9 assessment as text and retain every original failed-run JSON/manifest. Reproduce context discovery RED and retain the measured same-source 100k memory failure as the audit baseline.
- [x] Implement credential-bound native surfaces and truthful capabilities, with direct/service regressions and complete graph-catalog/object-root constraint checks. Recheck v1 eligibility at startup fallback; keep current authorization and restart/review after grant changes.
- [x] Both actual Mother/AnimaOS executable fixtures pass: legacy and native context-only selected sharing, owner edit, correction and restart reconciliation. No installed host/model or full graph-schema admission claim.
- [x] Stream exact auditing with point membership/cardinality proofs, ordered history and per-sequence receipt ranges. New corruption matrix and receipt-window boundary/reset/duplicate tests pass; independent invariant review found no lost checks.
- [x] Audit a disposable copy of the retained 100k corpus: exact 608,321 current/608,448 history/9,700 receipts, 97.2497 s audit, 1,997,213,696 bytes observed peak RSS; no resource guard fired. Only the copy receives a fresh synthetic read credential; original database SHA-256 remains identical. This is a separate successful audit, not a retroactive full benchmark pass.
- [x] Final Windows default/native: 495/164 passed; Linux: 504/170 passed. Both strict Clippy configurations, release builds, warning-free Rustdoc and native dependency exclusions pass; Windows format and native macOS compile check pass. Linux rustfmt remains unavailable. 148 source files match across platforms.
- [x] Independent audit and admission reviews approve the scoped fixes. Source hashes, original manifest preservation, synthetic JSON and docs links verify cleanly. Current report retains original failures and states the other write/lexical/contention/1m/macOS gates remain open.

No commit/push, Mother installation, live migration, live harness setting change or model call is included. Source stays in the existing G9 worktree; unrelated main licensing and AnimaOS implementation edits remain intact.
