# Context retrieval, pagination and review

`mother_get_context` and `mother_search_context` work the same way for every harness: ChatGPT, Codex, Claude Desktop, Hermes, AnimaOS and future clients. Nothing here is specific to one host. Every page re-applies the caller's current grant, so filters and cursors never widen what a credential can read.

## Resuming a project

Ask a connected harness “Where did we leave off on Nibs?” or “What changed on Nibs since my last visit?” Both use `mother_project_resume` with `{"project":"Nibs"}`. The service matches the project name as consecutive words in visible titles or text, ignoring case and punctuation. It returns the most recently updated confirmed decision, active checkpoint and open task. Each preview includes its record ID and revision for opening the original with `mother_get_record`. An empty slot means no matching record in the current grant.

The same reply includes up to three unseen or revised records, newest first. On a later visit the comparison starts at the acknowledged revision. For an unseen revised record it uses the immediately preceding revision if retained. For example, a task moving from blocked to done carries both statuses. Text corrections include a window around the first changed character. Shared readers receive current wording only. A previously seen record that no longer mentions the project is flagged.

After showing a page, call `mother_mark_project_seen` with its `project`, `workspaceId`, `generation` and a `seen` array of the displayed ID/revision pairs. If `remaining` is positive, read again. Repeating an acknowledgement or acknowledging an older revision cannot move the reading position backward. Positions are private to the project and credential, survive a service restart and are excluded from backups. Every harness with the relevant read grant can retrieve the shared project records.

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
- `consistency`: `keyset_positions` for context pages (`ranking_snapshot` for search), naming the rules below.

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

## Project names, keepsakes and event dates

The owner can register a former project name, set a record aside as a keepsake, and supply the date an event happened. Run these commands with the owner service stopped. Replace the example arguments with the native database path and record ID.

```text
mother garden project-alias --db <database> --project <current-name> --alias <former-name>
mother garden keepsake --db <database> --record <record-id>
mother garden moment-date --db <database> --record <record-id> --date 2026-01-17
```

Use `--remove` on the same alias pair, `--release` on a keepsake, or `--clear` in place of the date to undo the setting. A full Garden backup carries these settings across restore.

Project briefs recognize all registered names and consult reading positions saved under each name. Searching for a whole registered name also finds records mentioning another name in that project. Co-occurrence alone never creates an alias. Each preview’s `recallReason` identifies the relevant selection rule, matching project name or semantic match.

Keepsakes stay available in the owner Garden, curated walks and authorized direct reads by ID. Ordinary context, pins, assignments, project briefs and search exclude them. Call `mother_find_moment` with a natural-language `clue` to find up to three accessible accounts by matching distinctive words. Set `includeKeepsakes` only for an explicit request. Each hit includes the matched words, record ID, revision, saved time and any owner-supplied event date. Follow linked record IDs with `mother_get_record`. Approved photos have digest handles when the client can see the graph relationship and has media metadata access. Image bytes require the usual additional permission. The response reports partial media coverage when it reaches the scan limit.

## Harness compatibility

Existing calls keep their meaning. `{}` still returns the balanced overview with the persona and pins, and every older field is unchanged. All new fields are additive: `pagination`, `contextSections`, `createdAt`, `basis` and `compact`. A harness that ignores them still works, but it should read `pagination.hasMore` before claiming it has seen every record.
