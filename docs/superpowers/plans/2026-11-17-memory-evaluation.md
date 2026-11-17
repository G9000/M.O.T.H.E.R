# Memory evaluation implementation plan

> Execute this bounded slice in the current session. Preserve unrelated dirty UI
> files, installed runtime, all live memories, grants and folder permissions.

**Goal:** Reproducible synthetic recall baseline before retrieval improvements.

**Architecture:** A standalone Rust example plus reusable evaluation-only support,
frozen corpus/qrels and independent scorer tests. No production retrieval changes.

**Tech stack:** Existing locked Rust, serde_json, SHA-256, tempfile and native Garden.

- [x] Add scorer and dataset-validation tests; observe expected failing behavior.
- [x] Implement independently graded ranked retrieval and explicit error/empty cases.
- [x] Add versioned multilingual fixtures, distractors, lifecycle and access cases.
- [x] Execute unchanged native search and graph APIs in owned synthetic temporary stores.
- [x] Produce bounded JSON evidence with provenance, coverage and per-slice metrics.
- [x] Add misuse/scorer corruption controls and verify report output never overwrites.
- [x] Run baseline; publish honest local results and document unrun agent/model lanes.
- [x] Run required locked repository checks and review only the scoped diff.

## File and execution map

1. `examples/memory_eval/metrics.rs` owns formulas; `tests/memory_eval.rs`
   checks the hand-computed [noise,a] ranking, duplicate slots, empty qrels and
   a deliberately wrong retriever. Run `cargo test --locked --test memory_eval`:
   observe the zero-scoring stub fail, then implement and pass it.
2. `evals/memory-v1/corpus.json` contains only authored synthetic record revisions.
   `cases.json` contains version/id/k plus case id, scenario/split/category/language,
   public request fields and private grading fields relevant/forbidden/expectedError/
   expectedRevisions. `examples/memory_eval/dataset.rs` validates both and projects
   only request fields to the runner. Graph fixture keys are declared separately.
3. `examples/memory_eval/fixture.rs` creates a fresh temp Garden, explicit synthetic
   persona, distinct-time v1 revisions, owner/read-only/shared/revoked principals,
   and fixed-time entity/evidence/assertion graph revisions. Search uses actual
   NativeSearch; graph query/get uses existing public structs. Explicit selected
   aliases become runtime IDs without exposing gold relevance to the adapter.
4. `examples/memory_eval/runner.rs` executes two authored-order passes for each
   isolated literal/hybrid/semantic cache, then one graph pass. It resolves returned
   v1 IDs through get_record, checks source/revision and records omitted/coverage.
   Read-denial and write-denial gates check NOT_FOUND/FORBIDDEN and unchanged
   synthetic export plus absent receipt. Graph cases check validAt half-open bounds
   and knownAt prior revisions independently of natural-language planning.
5. `examples/memory_eval/mod.rs` builds provenance/aggregates/report and
   `examples/memory_eval.rs` offers stdout or exclusive `--output` and
   `--require-quality`. Learned-model mode is deferred pending deterministic index
   scheduling. Report lanes distinguish quality,
   contract, safety and unavailable agent answers. Never accept DB/config paths.
6. Extend tests to reject malformed qrels/split leakage and compare normalized two
   fresh-store outputs. Verify exclusive report creation. Run baseline with
   `cargo run --locked --example memory_eval -- --output target/memory-baseline.json`.
   Expected: complete JSON, honest semantic-language misses, zero safety violations;
   `--require-quality` must exit nonzero if those quality misses remain.
7. Document runnable commands and measured limitations in `docs/memory-evaluation.md`;
   retain a synthetic report under `evals/memory-v1/baseline-no-model.json` and link
   it from `docs/validation.md`. Run sequential fmt check, locked full tests, locked
   release build, Clippy with -D warnings and normal/build SQLite-free tree audit.

No installation, merge, push, provider call, reboot or folder ACL change is part of
this plan. No semantic or English query heuristics are added to the runtime.
