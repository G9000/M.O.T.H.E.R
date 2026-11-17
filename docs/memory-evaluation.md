# Memory evaluation

This is the baseline to run **before** changing Mother's retrieval. It evaluates
the existing native APIs in fresh synthetic Gardens; it does not improve retrieval
or import conversations. The Rust example needs no AnimaOS runtime, provider key,
network connection, downloaded dataset or installed model.

```powershell
cargo test --locked --test memory_eval
cargo run --locked --example memory_eval -- --output target/memory-eval-no-model.json
```

Choose a new output name for each run. Existing files are never overwritten.
Omit `--output` for JSON on stdout. Exit 0 means the measurement completed without
hard failures, **not** that recall quality passed. Add `--require-quality` to make
any failed quality case exit 1. Hard invariants or unexpected request errors exit 2.
Fixture initialization/output failure also exits 2 with an error, never a success
report. A run without a complete report is inconclusive.

## What is evaluated

The frozen [corpus](../evals/memory-v1/corpus.json) contains 19 synthetic selected
records, including corrections, a retraction and task chatter. A separate pending
proposal never becomes canonical. Four graph objects describe a fictional person,
two city assertions with half-open validity intervals, and their evidence; a later
correction tests knowledge time independently of valid time.

The [case/qrels file](../evals/memory-v1/cases.json) has 31 v1 cases and 10 graph
contract cases. V1 cases run literal, hybrid and semantic requests twice, in authored
order, with a fresh store/cache per mode: 196 observations, not 196 independent
questions. Every canonical v1 write advances a fixed clock, preventing random-ID
ties from changing ordinary literal rankings.

Coverage includes English, Chinese, Japanese and mixed-language questions,
paraphrases, several required memories, recent distractors, exact source readback,
correction/retraction, no-evidence questions, allowlist/revocation, denied writes,
pending proposals, bounded previews/pages, valid/known time and graph traversal.
Denied writes require byte-identical full backup files and an absent receipt.
Temporal cases explicitly forbid the wrong-time assertion. A one-unit traversal
must report incomplete coverage and `work_limit`, return at most one eligible edge
and never invent evidence beyond its authorization.

## Scoring rules

Only successfully scored, answerable retrieval cases enter ranking averages:

- Recall@5: unique relevant hits divided by all required positive qrels.
- Precision@5: unique relevant hits divided by five, including short lists.
- MRR@5: reciprocal rank of the first relevant hit, or zero.
- nDCG@5: gain `2^grade - 1`, discount `log2(rank + 1)`, grades 1–3.
- All-evidence coverage: every positive qrel must occur in the first five hits.

Duplicates consume positions but receive no repeated credit. The pet distractor
case also requires the top hit to be relevant. Missing/unknown facts have null
ranking metrics and a separate empty-result check; this does **not** demonstrate
an answering model's ability to abstain. Safety/contracts are hard gates independent
of relevance. A quality miss cannot erase a source, visibility or coverage failure.

Each mode/pass has its own overall, language, category and split aggregates, with
attempted/scored/error denominators and p50/p95 request latency. There is no pooled
cross-mode accuracy. Errors remain errors; they never become successful zero-hit
cases. Timing excludes fixture loading and full-read verification, and does not
provide a portable performance guarantee. The denied-write operation includes its
before/after backup snapshots and receipt verification in its timing; compare that
category separately from search latency.

## Honest no-model baseline

This slice deliberately selects absent assets, ignoring `MOTHER_MODEL_DIR`. Hybrid
and semantic requests must report literal fallback, `semanticUsed: false` and
`model_unavailable`. Their identical results are not three retrieval capabilities.
Learned retrieval is `not_run`: a separate follow-up must stabilize bounded index
preparation order before comparing real model candidates. No keyword translation
table, synthetic encoder, hidden prewarming or retry-until-success is used here.

AnimaOS answer/tool-use, capture quality and physical restart are separate `not_run`
lanes. These API checks do not establish correct model tool selection, additive
native-memory behavior, actual host recall, hallucination rate or explicit capture
decisions. Do not revive the previously declined live provider conversation.

## Reproduction and dataset discipline

Reports include raw synthetic result aliases/revisions, requests, gold judgments,
source/coverage checks, timing, fallback metadata, corpus/qrels/evaluator digests,
selected compiled runtime module digests, OS/architecture/debug configuration and
the actual executable SHA-256. The source manifest is explicitly partial; it is not
a claim that a few hashes identify the whole source/build environment.

Two fresh-store runs must produce equivalent normalized results. Timing/byte
measurements, opaque
snapshot/visibility tokens and the permitted edge choice in the bounds-only
traversal are normalized only for that test. The report retains raw observations.
That bounds-only case has no ranking qrels and cannot inflate retrieval scores.

Scenario-level dev/holdout splits keep translations of one scenario together. This
small, public, manually authored holdout is a regression aid, not an independent
secret benchmark or evidence of generalization. Review new relevance judgments
before testing changes; do not tune on held-out scenarios or edit gold to erase a
failure. Add owner-reported failure categories using fictional examples, not private
transcripts. Version corpus/qrels changes and rerun both baseline and candidate
under the same executable configuration and budgets.

The task distinctions draw on [LongMemEval](https://github.com/xiaowu0162/LongMemEval)
and [LoCoMo](https://github.com/snap-research/locomo), with ranked retrieval metrics
used by [BEIR](https://github.com/beir-cellar/beir). This suite is original synthetic
test data, not a run of those benchmarks and not comparable to their leaderboards.

See the [design](superpowers/specs/2026-11-17-memory-evaluation-design.md) and
[implementation plan](superpowers/plans/2026-11-17-memory-evaluation.md).

## Recorded baseline, 17 November 2026

The [machine-readable no-model baseline](../evals/memory-v1/baseline-no-model.json)
records 196 observations, zero unexpected errors and no failed hard gates. Quality
fails: 14 of 31 v1 cases fail in each mode/pass, for 84 failed observations.
The six repeated no-model mode/pass results are equivalent, not independent wins.

| First-pass literal slice | Answerable scored cases | Recall@5 | MRR@5 | nDCG@5 |
| --- | ---: | ---: | ---: | ---: |
| All | 23 | 0.4348 | 0.3913 | 0.4027 |
| English | 12 | 0.6667 | 0.5833 | 0.6052 |
| Chinese | 4 | 0.2500 | 0.2500 | 0.2500 |
| Japanese | 4 | 0.2500 | 0.2500 | 0.2500 |
| Mixed | 3 | 0 | 0 | 0 |

All ten structured graph contract cases pass, including forbidden wrong-time
results, known-time correction, hidden references and the exhausted work budget.
These small synthetic results establish gaps in the no-model path; they do not
estimate the owner's live recall rate or multilingual model performance.

Qualification used a frozen copy of `94a71098e9c31654816f1c7a70576a65c1731cc5`
plus only the twelve scoped evaluation/code/fixture/design/guide files. Rust and
fixture hashes were compared with the delivered files. Windows x86_64 checks:
fmt, locked tests (**246 passed, six intentional ignores**), locked release build,
strict all-target/all-feature Clippy and normal/build SQLite exclusion all pass.
Seven tests verify the evaluator itself. The quality-gate invocation exits 1 as
expected; normal report collection exits 0. Linux/macOS and live model/host runs
were not performed for this slice.

The initial default sandbox full-suite attempt failed existing owner-private
temporary-directory/cutover admission checks. Running in a fresh owner-private
synthetic root resolved that environment failure without ACL edits. One repeated
evaluation test exposed permissible budget-edge response-size variability; its
normalization now excludes byte measurements while retaining and enforcing all
actual byte bounds. Concurrent MCP pinned-context work later introduced a separate
failing test in the shared checkout; it was preserved and excluded through the
frozen source copy. The passing qualification is for that isolated slice, not a
claim that the concurrently changing shared checkout is fully green.

No production retrieval implementation, dependencies, live garden, installed
binary, grant, harness configuration or folder permission was changed by this
evaluation. Technical continuation checkpoints are the only shared context writes.
