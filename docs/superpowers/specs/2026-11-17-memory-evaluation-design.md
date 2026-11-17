# Memory evaluation baseline

The owner approved establishing an evaluation baseline before changing retrieval.
This slice creates an offline, synthetic, Rust evaluation in Mother. It does not
change search, indexing, canonical memory logic, installed software or grants.

## Decision

Use two explicitly separate evidence lanes. The runnable first lane evaluates the
actual native v1 search/full-read and temporal graph APIs against frozen synthetic
corpora and independently declared relevance judgments. The second lane is
AnimaOS answer/tool-use evaluation, initially `not_run`: storage scores cannot
establish agent intelligence, abstention, additive memory or explicit publication.

A unit-test-only extension would miss ranked retrieval quality. Running a hosted
model benchmark immediately would mix retrieval, tool selection and answer quality
and require new provider authorization. The offline baseline is the smallest useful
first step. No paid calls, personal records, model downloads or installed settings
are needed. Existing unrelated garden UI edits remain untouched.

## Dataset and scoring

- Versioned, hand-authored synthetic corpus, separate query/qrels file and named
  development/holdout splits. Split by scenario; translations of a scenario must
  stay in one split. This small public holdout is a regression holdout, not secret
  or statistically representative evidence of generalization.
- Cover literal controls, paraphrases, English/Chinese/Japanese/mixed queries,
  distracting task chatter, several required memories, corrections/retractions,
  unknown facts, temporal validity and knowledge time, bounded traversal, source
  attribution, read allowlists and denied writes. Raw transcripts are not ingested.
- Report Recall@5, Precision@5 (fixed denominator), MRR@5, graded nDCG@5 and
  all-required-evidence coverage on answerable retrieval cases. Empty qrels yield
  null ranking metrics and a separate empty-result check, not perfect recall.
- Duplicate results never earn repeated credit. An errored case remains an error;
  aggregates disclose attempted/scored/error counts. Separate per-language,
  category and split aggregates prevent an English-heavy average hiding failures.
- Hard gates cover hidden/retracted content, denied writes, exact full-read
  attribution/revision, source consistency and bounds/coverage reporting. Retrieval
  emptiness is not claimed as model abstention. Graph contract checks are not
  natural-language graph planning scores.

## Execution and reproducibility

Create and delete only a fresh owned temporary native Garden. Never accept a live
DB, credential or dataset path. Advance the synthetic clock by a distinct second
for every write: a frozen equal timestamp would leave UUID-dependent rank ties.
Verify normalized results match across independent fresh stores. Map generated IDs
to synthetic aliases for reproducible
qrels; aliases/expected answers are never passed as extra retrieval hints.

Run literal/hybrid/semantic requests against the unchanged NativeSearch. By
default deliberately point to a nonexistent asset directory, measuring the honest
no-model fallback. Learned-model runs are explicitly `not_run` in this slice:
bounded index preparation uses random-ID order, so they need deterministic corpus
IDs/index scheduling in a separate evaluation lane. No synthetic encoder can be
labeled learned or multilingual semantic acceptance.
Report requested/effective mode, fallback reason, index coverage and model/profile.
Do not secretly warm/retry until success. Run modes literal, hybrid, semantic in
that order, each in a fresh store/cache; run the authored case order twice per
mode, labeled first_pass/repeat_pass. Report partial-index progression; neither
pass promises a cold model for every query. No cross-mode quality average is
reported. Latency numbers are observations, not portable performance guarantees.

One exhausted graph traversal is a bounds-only contract: either of two eligible
edges is valid under the one-unit work budget. Its allowed set, hit cap, incomplete
coverage and work-limit reason are checked; it has no ranking qrels. Reproducibility
checks normalize its permissible edge selection, timing/byte measurements and opaque snapshot/epoch
tokens, while retaining raw observations in the report.

Emit a machine-readable JSON report to stdout or a new, exclusively created report
file. Pin corpus/qrels and evaluator/selected-runtime source digests plus environment,
configuration and cutoffs. Include raw per-case observed IDs/revisions, search
metadata, bounds, latency, expected/actual outcomes and summary denominators.
Use positive integer relevance grades 1..3 and gain 2^grade-1, logarithmic rank
discount log2(rank+1). Duplicate hits consume rank slots but add no credit.
Each positive qrel is required evidence, not an alternative group. Macro-average
answerable cases within the same mode/pass and disclose denominators; slice by
split/category/language. Denied writes require unchanged canonical state and no
operation receipt in addition to the expected error.

Quality misses must remain visible in the baseline. Infrastructure failures and
safety violations fail the run; an opt-in quality gate additionally rejects misses.
Do not lower thresholds or change qrels to make the baseline green. Mark the source
manifest explicitly partial; include actual executable SHA-256 and debug/target
configuration so selected-module digests cannot imply complete source provenance.

## Research basis and limits

LongMemEval distinguishes extraction, multiple-session evidence, temporal reasoning,
knowledge updates and abstention: https://github.com/xiaowu0162/LongMemEval .
LoCoMo separates retrieval/QA and more complex conversational reasoning:
https://github.com/snap-research/locomo . BEIR reports ranked retrieval metrics:
https://github.com/beir-cellar/beir . This is an original Mother-specific synthetic
suite inspired by those task distinctions, not a run of their datasets, a comparable
leaderboard result or a substitute for future human/provider acceptance.

## Acceptance

Metric tests use manually computed rankings including partial/multiple evidence,
duplicates and empty qrels. Dataset validation rejects unknown/duplicate IDs,
overlapping relevance/forbidden sets and split leakage. An intentionally wrong
retriever must score worse; omitted/error cases cannot disappear into passing
averages. Run the baseline unchanged and retain its failures. Run repository fmt,
locked tests, release build, Clippy and SQLite-free normal/build dependency audit.
