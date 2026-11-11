# Local semantic search

M.O.T.H.E.R retrieves context; it does not run an agent, execute tools on behalf of a harness, or invent memories. Canonical records, corrections, grants, receipts and exports remain in the existing SQLite v1 store. Retrieval uses a disposable vector cache beside that database. No AnimaOS runtime or third-party memory service is required.

## Setup

The Rust executable includes CPU inference using Candle 0.9.2. Explicitly install the pinned English sentence-transformers/all-MiniLM-L6-v2 model:

```powershell
mother model install
mother model status
```

Installation downloads approximately 91 MB of public weights, tokenizer and configuration from Hugging Face at a fixed revision, checks exact sizes and SHA-256 hashes, and publishes the verified directory. No record/query text or credentials are sent. There is no paid model API call. Search and MCP startup never download anything; after installation inference is local. The model uses the Apache-2.0 license; see its [model card](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2).

By default assets live under the private MOTHER data directory in its `models` subfolder, under `all-MiniLM-L6-v2`. `MOTHER_MODEL_DIR` can select another absolute model directory. A missing/corrupt model or invalid model-directory setting leaves ordinary memory reads/writes and literal search usable. Explicit model commands report failures. Invalid existing installations are not overwritten; select another directory or move the invalid directory aside before installing again.

## Search and indexing

```powershell
mother search --config "C:\private\codex.private.json" --query "What was the next step for deployment?"
mother search --config "C:\private\codex.private.json" --query "ZX-731" --mode literal
mother search --config "C:\private\codex.private.json" --query "How do we recover lost documents?" --mode semantic
mother index --config "C:\private\codex.private.json" --max-records 32
```

`mother_search_context` uses the same implementation. Its optional `mode` is `hybrid` (default), `semantic`, or `literal`. Search adds no separate tool. `updatedSince` and `updatedBefore` restrict candidates to records last updated in that window, as described in [shared context](context.md#time-windows). `mother_get_context` remains the original literal/navigation operation.

Hybrid puts exact substring matches first, then semantic matches ordered by cosine similarity. Semantic mode uses meaning-based matches. Literal mode preserves the existing SQLite matching behavior, including literal percent/underscore and SQLite's Unicode case behavior. A 0.35 cosine floor filters weak semantic candidates; it is a relevance heuristic, not a truth/confidence probability or a calibrated quality guarantee. The initial model targets English; other languages and specialized terms may retrieve less reliably. Exact identifiers remain searchable through hybrid/literal mode.

Results retain workspace, persona, preview and unsettled-work fields. Additional `search` metadata states the requested and effective modes, whether semantics participated, fallback/partial reason, model/profile, and coverage counts. `omitted` counts matching candidates beyond the result limit; `unindexedRecords` and `partialRecords` separately report semantic coverage gaps. `unembeddableRecords` counts records whose text yields no model tokens; they are marked for that revision so they cannot block later records. Re-running indexing cannot fix such text or records beyond the supported window cap.

A semantic match includes a passage preview and byte offsets into the exact string `title + "\n" + content`, plus a truncation flag. Short matches include adjacent sentence context when it fits within 400 UTF-16 units. `startByte`/`endByte` delimit this contextual span; `embeddingStartByte`/`embeddingEndByte` retain the exact scored span. Longer previews are truncated explicitly. This is navigation to a current record, not a citation proving the record's contents. Fetch the full record before editing or acting.

## Bounds and freshness

Each request scans at most 10,000 active canonical records, the existing store limit. Overlapping 192-token windows (32-token overlap), short sentences (up to 96 tokens), and adjacent sentence pairs (up to 192 tokens) share a cap of 32 distinct token sequences per record. Duplicate raw windows do not consume that cap; uncovered raw text is reported as partial. Supplemental sentence passages use only remaining slots. Hybrid/semantic search prepares at most 8 missing records and 32 new passages; indexing CLI batches accept 1–100 records, with a ceiling of 1,024 new passages. Repeat bounded indexing until `complete` is true, or inspect explicit partial/non-indexable/capacity counts when no further progress is possible. Caches retain completed and explicitly partial outcomes, so subsequent batches advance instead of repeatedly embedding the same record.

Cache admission is capped at 32,000 passages and SQLite pages at 128 MiB. A full cache stops additional indexing with an explicit reason; it does not evict and re-index records indefinitely. Stale/inactive entries are pruned during indexing. The cache contains derived vectors and offsets, is private, and is excluded from exports. It can be rebuilt after closing clients and moving the derived `<database>.search.sqlite` file aside; never move or delete the canonical database for this purpose.

Every cache entry is bound to workspace identity, restore generation, pinned model revision, window profile, record ID, revision and text digest. Search checks access before processing and again against a fresh canonical snapshot before returning. A concurrent correction/retraction causes fresh literal fallback; revocation remains an error. Cached text never overrides the canonical store. Missing, busy or invalid retrieval infrastructure produces a stated fallback, including for requested semantic mode; `semanticUsed: false` must not be described as semantic retrieval.

The service reuses one model per MCP process. A concurrent search falls back instead of waiting for that model. Memory writes do not depend on the cache/model and retain their existing atomic revision and receipt behavior. Restore needs no embedding cache; it rebuilds from canonical records.

## Acceptance

Normal tests run offline with isolated synthetic stores and missing-model fixtures. The opt-in real-model suite requires the explicitly installed model:

```powershell
$env:MOTHER_TEST_MODEL_DIR = "$env:LOCALAPPDATA\MOTHER\models\all-MiniLM-L6-v2"
cargo test --release --locked --test semantic_real -- --ignored --nocapture
cargo test --release --locked --test mcp real_model_semantic -- --ignored --nocapture
```

These checks exercise no-overlap paraphrases, distractors, an irrelevant query, a clause after a long unrelated prefix, passage offsets, exact identifiers, corrections/retractions, zero-token text, cache reuse and MCP/CLI reads. They are functional acceptance fixtures, not a broad retrieval benchmark. Selective history import and automatic memory extraction/consolidation are not part of this change.
