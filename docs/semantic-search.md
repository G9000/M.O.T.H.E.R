# Local semantic search

M.O.T.H.E.R retrieves selected context; harnesses choose what to remember, interpret results and perform final ranking. Canonical graph/v1 records and immutable receipts live in redb. Optional local vector caches are disposable redb data. Pending proposals never enter canonical recall or indexes. No AnimaOS runtime, remote model provider or automatic transcript ingestion is required.

## Supported surfaces and model availability

V1 MCP `mother_search_context` accepts `hybrid` (default), `semantic` and `literal` modes. `mother_get_context` remains the context/navigation operation. Graph queries expose separate lexical/vector component scores and coverage for the consuming harness to combine; they do not use v1 hybrid ranking.

The current executable has no `model install` or general owner `search` command. It can load already installed, verified pinned local Candle assets. Startup, search and graph preparation never download a model. There is no built-in installation workflow in this release; absent or invalid assets produce an explicit literal fallback instead of a hidden download or provider call.

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

## Background indexing

The running owner service keeps the v1 semantic index current on its own, so coverage no longer depends on clients repeating searches. Earlier, each search embedded at most eight records or 32 passages. One long record could use the whole allowance, and each client kept a separate cache, so coverage crept up by one or two records per search.

- The indexer runs as one low-priority thread in `mother garden service`. It embeds one record at a time, newest first, and releases the model between records so searches can use it. When work is pending it pauses 100 ms between records. When idle it checks every 10 seconds.
- Vectors live in one derived table shared by all clients, keyed by embedding profile and record ID. Each vector is valid only for the exact revision and text digest it was computed from. A vector is published only if the record is unchanged when the write commits. Visibility is still decided per caller by the authorized snapshot; the cache never selects records.
- Revised records are re-embedded. Vectors for retracted, superseded or deleted records, and for other embedding profiles, are pruned.
- Progress and per-record failures are stored in the database, so a restart resumes where indexing stopped. A failed record is retried when it changes, or after six hours.
- With no verified local model installed, the indexer stays idle and literal search covers every authorized record. Mother never downloads a model or sends memory text anywhere. Set `MOTHER_SEMANTIC_INDEX=off` for the service to disable background indexing.

`mother_get_capabilities` reports `semanticIndex` for the caller's authorized records: eligible, indexed, partially indexed, unembeddable, pending and failed counts, progress, model and profile, model state, whether background indexing is running, and the last run, success and error. Searches still embed up to eight missing records inline when the background indexer hasn't reached them yet.

With the service stopped, the owner can inspect or repair the derived index directly:

```powershell
mother garden semantic-index --db "$env:LOCALAPPDATA\MOTHER\garden.redb"
mother garden semantic-index --db "$env:LOCALAPPDATA\MOTHER\garden.redb" --build
mother garden semantic-index --db "$env:LOCALAPPDATA\MOTHER\garden.redb" --repair
```

`--build` indexes every pending record now with the installed model. `--repair` discards all derived vectors and indexing state, including the older per-client cache, so the service rebuilds them. Canonical records are never modified.

The shared index is bounded by the number of recallable records, at most 32 passages each. Canonical native storage has no legacy 10,000-record lifetime cap; that does not promise bounded v1 snapshot-search latency at every scale.

Before publishing derived data or returning candidates, search rechecks the current canonical snapshot and grant. A changed workspace yields fresh literal fallback; revocation fails closed. Missing/busy models, corrupt caches and unavailable infrastructure report their fallback honestly. Cache capacity does not evict/re-index indefinitely. Writes and full restore do not depend on embeddings; backups exclude rebuildable vectors.

## Verification

Ordinary tests use synthetic records and offline synthetic or missing-model fixtures. Real-model acceptance requires separately supplied verified local assets and an explicit opt-in test; no model was installed merely by building or running ordinary checks. Follow the current test inventory and [qualification report](validation.md) for runnable checks. Historical SQLite model installer, CLI and sidecar instructions are retired.

Storage/protocol tests, synthetic encoder tests, real-model retrieval quality and actual host use are separate evidence. Process interruption does not establish physical power-loss behavior, and a successful selected indexing batch does not establish complete semantic coverage.
