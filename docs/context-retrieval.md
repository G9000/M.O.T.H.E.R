# Context retrieval, pagination and review

`mother_get_context` and `mother_search_context` work the same way for every harness: ChatGPT, Codex, Claude Desktop, Hermes, AnimaOS and future clients. Nothing here is specific to one host. Every page re-applies the caller's current grant, so filters and cursors never widen what a credential can read.

## Filters

Both tools accept these optional fields. All filtering happens in the service, before the response budget is applied.

| Field | Meaning |
| --- | --- |
| `kind` | One record kind: `memory`, `decision`, `task`, `checkpoint`, `artifact` or `action`. |
| `actor` | Only records attributed to this client ID, for example `chatgpt`. |
| `status` | One recallable status, such as `active`. Retracted, superseded and cancelled versions stay in `mother_get_record_history`. |
| `basis` | Memories only: `owner_statement`, `source` or `model_inference`. This keeps the owner's own words, verified sources and assistant interpretations apart. |
| `updatedSince`, `updatedBefore` | A window on the record's last update. |
| `limit` | Page size, 1–100 (default 20). |
| `previewMaxChars` | Excerpt length per record, 0–2000 (default 400). `0` returns compact metadata only. |
| `includePersona`, `includePinned` | Both default to `true`. Set them to `false` on repeated calls so the whole budget goes to records. `contextSections` says what was included. |
| `cursor` | The `pagination.nextCursor` from the previous page of the same query. |

`mother_get_context` also takes `order`:

- `balanced` is the default when no filter is given. It's a one-page overview where categories take turns. It has no cursor.
- `recent` is the default when any filter is given: newest `updatedAt` first.
- `created` is oldest `createdAt` first. A revision never moves a record, so this order suits complete reviews.

Example:

```json
{"kind": "memory", "actor": "chatgpt", "limit": 25, "includePersona": false, "includePinned": false, "previewMaxChars": 300}
```

## Pagination

Every `recent` or `created` page includes `pagination`:

- `matchingRecords`: total matches for the filters, across all pages.
- `returnedRecords`, `remainingAfterPage` and `hasMore`.
- `nextCursor`: keep calling with it until `hasMore` is `false`. Keeping the last cursor also lets a harness pick up records added later.
- `changed`: records revised or added behind the cursor since the previous page.
- `consistency`: a one-line statement of these rules.

How concurrent changes behave:

- A cursor is a record position, not an offset. Removing or retracting a record doesn't shift later pages.
- In `created` order, revisions don't move records and new records arrive at the end.
- In `recent` order, a revised record moves ahead of the cursor. It's reported once in `pagination.changed`, so nothing is skipped silently.
- If more than 10 records change between two pages, the next page returns `CURSOR_STALE`. Start again without a cursor.
- A cursor is bound to its filters, client and workspace generation. Reusing it with other filters, from another credential or after a restore returns `INVALID_INPUT`.

Search pages are offsets into one ranking. When matching records, the grant or semantic index coverage changes, the cursor returns `CURSOR_STALE`. For a stable full enumeration, use `mother_get_context` with `order: "created"`.

## Response budget

Context, search and history lists still fit 12 KiB of compact JSON, but no section can starve the others any more. The budget is spent in this order:

1. Required metadata always stays: identity, inventory, counts, assignments and unsettled work.
2. Record excerpts become compact from the end of the page first. Compact records keep `id`, `kind`, `title`, `status`, `revision`, `actor`, `createdAt` and `updatedAt`, with `compact: true` and an empty `excerpt`.
3. Trailing records are then dropped, but at least three matching records are kept when they exist. Pinned excerpts are shortened before that floor gives way.
4. The persona text gives way only when not even one compact record would fit beside it. It's replaced by `{"omitted": true, "reason": "response_budget", "bytes", "hash", "retrieve"}`, and `contextSections.persona` is `omitted_budget`. To read it alone, call `mother_get_context` with `{"limit":1,"includePinned":false,"previewMaxChars":0}`.

`responseBudget` reports `omittedItems`, `compactedRecords`, `recordFloor`, `personaOmitted` and `pinnedContentTruncated`. When records are dropped to fit, `pagination.nextCursor` is moved back to the last record actually returned, so the next page continues without gaps.

## Reviewing a harness's memories

Use this to garden a harness's memories without changing anything:

1. Enumerate: call `mother_get_context` with `{"actor": "<client>", "kind": "memory", "order": "created", "limit": 100, "includePersona": false, "includePinned": false, "previewMaxChars": 0}` and follow `nextCursor` until `hasMore` is `false`.
2. Fetch each record you need with `mother_get_record`. History comes from `mother_get_record_history`, where the grant allows it.
3. Compare duplicates, stale versions and contradictions within each `basis`. Keep owner statements, sources and model inferences distinct.
4. Write recommendations as proposals, for example with `mother_propose_memory` or `mother garden tend`, for the owner to review. Reading and enumerating never modify canonical memories, and nothing is pruned automatically. Stories, snapshots, media associations and Witness Journal reflections stay intact.

## Harness compatibility

Existing calls keep their meaning. `{}` still returns the balanced overview with the persona and pins, and every older field is unchanged. All new fields are additive: `pagination`, `contextSections`, `createdAt`, `basis` and `compact`. A harness that ignores them still works, but it should read `pagination.hasMore` before claiming it has seen every record.
