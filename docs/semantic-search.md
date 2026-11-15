# Local semantic search

M.O.T.H.E.R retrieves selected context; harnesses choose what to remember, interpret results and perform final ranking. Canonical graph/v1 records and immutable receipts live in redb. Optional local vector caches are disposable redb data. Pending proposals never enter canonical recall or indexes. No AnimaOS runtime, remote model provider or automatic transcript ingestion is required.

## Supported surfaces and model availability

V1 MCP `mother_search_context` accepts `hybrid` (default), `semantic` and `literal` modes. `mother_get_context` remains the context/navigation operation. Graph queries expose separate lexical/vector component scores and coverage for the consuming harness to combine; they do not use v1 hybrid ranking.

The current executable has no `model install/status`, general owner `search` or v1 `index` commands. It can load already installed, verified pinned local Candle assets. Startup, search and graph preparation never download a model. There is no built-in installation workflow in this release; absent or invalid assets produce an explicit literal fallback instead of a hidden download or provider call.

`MOTHER_MODEL_DIR` selects an absolute model directory. Existing verified all-MiniLM-L6-v2 assets can remain usable; the loader checks the pinned files/profile rather than trusting arbitrary model content. Model absence, corruption or an invalid path setting leaves ordinary context reads/writes and literal search available. The English model's relevance and multilingual quality require their own evaluation.

Selected graph preparation remains an explicit owner command:

```powershell
mother garden index --config "C:\private-data\mother\graph.native.json" --objects "C:\private-data\mother\selected-objects.json"
```

The objects file is a bounded JSON array of typed `{kind,id}` references. Stop the owner service first: this command opens the database directly and requires a direct `garden_redb` credential, not a service config. It checks the selected objects' current graph authority and never downloads assets. Restart the service afterward. See [graph query semantics](garden-queries.md), [native compatibility](garden-compatibility.md#optional-local-search) and [service operation](garden-service.md).

## Results and freshness

Hybrid search puts exact literal matches first, followed by semantic candidates ordered by cosine similarity. A 0.35 cosine floor is a relevance heuristic, not a truth probability or quality guarantee. Literal mode never loads or invokes an encoder. Exact identifiers remain available through hybrid/literal search.

Responses report requested/effective mode, whether semantics participated, fallback/partial reason, model/profile and coverage. `semanticUsed: false` must not be described as semantic retrieval. Omitted candidates, unindexed records, partial coverage and zero-token records are distinct outcomes. A prepared vector is not evidence of actual host recall.

Semantic passage previews include byte offsets into the exact `title + "\n" + content` string and explicit truncation. Contextual preview offsets and exact scored embedding offsets remain separate. Fetch the current full record before editing or acting; a vector score or excerpt is navigation, not proof of the claim.

Native v1 search prepares at most eight missing records and 32 new passages per request. Its derived cache is bounded to 32,000 passages and 128 MiB. Canonical native storage has no legacy 10,000-record lifetime cap; that does not promise bounded v1 snapshot-search latency at every scale. Cache keys bind workspace/generation, client visibility, pinned model/profile, record revision and content digest.

Before publishing derived data or returning candidates, search rechecks the current canonical snapshot and grant. A changed workspace yields fresh literal fallback; revocation fails closed. Missing/busy models, corrupt caches and unavailable infrastructure report their fallback honestly. Cache capacity does not evict/re-index indefinitely. Writes and full restore do not depend on embeddings; backups exclude rebuildable vectors.

## Verification

Ordinary tests use synthetic records and offline synthetic or missing-model fixtures. Real-model acceptance requires separately supplied verified local assets and an explicit opt-in test; no model was installed merely by building or running ordinary checks. Follow the current test inventory and [qualification report](validation.md) for runnable checks. Historical SQLite model installer, CLI and sidecar instructions are retired.

Storage/protocol tests, synthetic encoder tests, real-model retrieval quality and actual host use are separate evidence. Process interruption does not establish physical power-loss behavior, and a successful selected indexing batch does not establish complete semantic coverage.
