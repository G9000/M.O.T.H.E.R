# Local semantic context retrieval

Approved scope: add semantic search over existing M.O.T.H.E.R records, keeping this entirely in the independent context layer. SQLite remains authoritative. No history import, automatic extraction/consolidation, model-provider execution, agent runtime, third-party memory service, network synchronization, or AnimaOS dependency.

## Choice

Use Candle CPU inference with the English sentence-transformers/all-MiniLM-L6-v2 model (384 dimensions, Apache-2.0). Its fixed weights/tokenizer/config are downloaded only by an explicit owner CLI installation command, pinned to a revision and SHA-256 digests. Search never downloads or sends text remotely. Compared with FastEmbed/ONNX, this avoids a separate native runtime DLL; compared with a hosted memory engine, it avoids another authoritative store and service. Model quality/language coverage are limited and will be measured with synthetic paraphrase fixtures, not assumed.

## Boundaries and data

- Existing v1 database, identities, grants, records, receipts and export format stay unchanged.
- Derived vectors live in a separate private SQLite cache beside the selected database. Cache keys include workspace ID, generation, model/profile, record ID, revision and text digest. It can be removed and rebuilt; it never owns memory.
- Token windows cover title + newline + content with overlap; record window count and per-search inference work are bounded. Partial coverage is explicit. Match snippets identify offsets in that exact joined text.
- Search authorizes before reading/embedding and reauthorizes against a fresh canonical snapshot before returning. Changed content/revisions, restored identity, retractions, supersession/cancellation and revoked grants must not leak stale results. A changed snapshot yields fresh literal fallback rather than stale semantic output.
- Cache storage and indexing work are capped. Corrupt, missing, busy or unavailable model/cache produces a stated literal fallback, never invented semantic participation. Writes remain independent of retrieval availability.

## Interfaces

Keep all six MCP tools. Extend mother_search_context with optional mode: hybrid (default), semantic, literal. Existing query/kind/limit constraints remain. mother_get_context keeps its original literal/navigation behavior. Responses retain existing preview/context fields and add search metadata: requested/effective mode, reason, model, semantic participation, eligible/indexed/unindexed/partial counts. Semantic-only fallback is explicit, not silent. Exact substring hits remain first in hybrid results; semantic-only hits follow by cosine relevance with a documented conservative floor. Full records remain necessary before edits.

Add CLI model install/status, search --config --query [--mode], and index --config with bounded batch controls. MCP loads the embedding model lazily and reuses it behind a try-lock; normal reads/writes do not wait on embedding inference. Hybrid searches index a small bounded set of missing current records; the CLI can prepare more. No hidden model download at server startup or during search.

## Verification and delivery

Tests must fail first for new behavior. Preserve all existing tests and v1 fixtures. Test fallback metadata; literal special-character and Unicode behavior; exact-match preservation; kind/limit validation; stale/corrupt cache; correction/retraction/revocation; identity changes; capacity and partial coverage; long text; restart/cache reuse. A real downloaded model must retrieve paraphrases with no literal overlap, including a relevant clause after a long unrelated prefix. Real-model checks use synthetic records and are separate from normal offline tests. Validate CLI and MCP paths, fmt, full tests, clippy, release and docs.

Install the tested native binary alongside the running version and update the prepared/configured launcher only after verification, preserving settings and private state. No force overwrite of a running executable. Report if the current Codex connection needs a restart. No push or merge is implied by this feature request.

## Concrete bounds and validation clarifications

At most 10,000 canonical records (existing store limit) are scanned. Each record has at most 32 overlapping 192-token windows, overlap 32; extra windows set partial=true. Each hybrid search indexes at most 8 missing records and 32 passages; a record is only started when its whole bounded plan fits the remaining passage budget. Cache rows including partial plans are treated as completed for that revision, so the next query advances to the next missing record in stable ID order. CLI batches allow 1..100 records and at most 1024 passages. The cache admits at most 32,000 passages and has a 128 MiB SQLite page ceiling; at capacity it stops indexing explicitly instead of evicting useful entries repeatedly. Stale/deleted/retracted rows are pruned on indexing. Every cache key is bound to the content hash and model/window profile.

Authorization/identity errors from canonical storage never become a successful fallback. After inference acquire a fresh authorized snapshot; if its workspace/record fingerprint differs, regenerate literal results and all response fields from that snapshot and report workspace_changed. Literal matching retains SQLite lower/instr title-space-content semantics. Real-model acceptance includes distractors, irrelevant queries, late supported passages, correct snippet offsets, exact identifiers and restart reuse; fixtures are not a general quality benchmark.
