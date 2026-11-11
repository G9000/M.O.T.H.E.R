# Understanding shared context

`mother_get_context` covers all canonical record categories by default: memory, decision, task, checkpoint, artifact and action. A memory is one category of shared context. A workspace can have no recallable memory-category records and still contain useful checkpoints or decisions.

## Workspace inventory and selected previews

The response separates full-workspace counts from the bounded list returned by this request:

| Field | Meaning |
| --- | --- |
| `inventory.totalRecords` | All canonical records, including retained historical records. |
| `inventory.byKind` | All canonical counts by category; all six categories are always present. |
| `inventory.recallableRecords` | Records available for ordinary recall, excluding retracted, superseded and cancelled statuses. |
| `inventory.recallableByKind` | Workspace counts by category for ordinary recall, independent of request filters and limit. |
| `inventory.excludedRecords` | Retained historical records omitted from ordinary recall. |
| `selection` | The applied query/kind filters, preview order, matching count, returned preview count and omitted matching count, with returned and omitted counts for each category. |
| `summary` | A brief description of workspace counts and preview coverage. |

`records` remains a bounded preview list, with the existing `omitted` and per-record `truncated` fields. To read a complete current record, use `mother_get_record` with its ID. Default recall does not emit retracted content. Inventory does not count unreviewed Living Memory proposals as accepted records.

For example, if a workspace has three checkpoints and no memories, a request filtered to `kind: memory` returns no previews, but its inventory still reports three recallable checkpoints. Neither that filter nor `limit: 1` makes the workspace inventory empty. Do not describe a filtered or truncated preview page as the whole workspace.

## Pinned core

Some things should never fall out of a first read: your name, what matters most to you, a decision you want kept. `mother_pin_record` with `{"id": ..., "pinned": true}` pins a memory or decision; `"pinned": false` unpins it. Every `mother_get_context` response then includes it under `pinned`, whatever the query, kind, time window or preview limit:

| Field | Meaning |
| --- | --- |
| `pinned` | Each pinned, still-active record with its current title, revision and content (up to 2,000 characters, with `truncated`), plus who pinned it and when. |
| `pinnedSummary` | `total` pins, how many are `shown`, how many are `inactive` because the record was retracted or superseded, and the `limit` of 12. |

A pin follows its record, so a corrected memory shows its newest text. A retracted pinned record drops out of `pinned` but still counts toward the limit until it is unpinned. Pinning needs the same write scope as editing that kind of record. Shared-access clients see only pins on records they were granted and cannot pin. Pins are carried in exports and restored with them.

Pins are kept in their own table rather than on the record, so an executable older than 0.5.0 ignores them and can still read the workspace. An export made by such an executable leaves pins out.

## Balanced previews

Checkpoints are saved every session, so a newest-first list soon holds nothing else. When a request has no `query` and no `kind`, `selection.order` is `balanced`: categories take turns for the preview limit, in the order checkpoint, task, decision, memory, action, artifact. Within a category the newest record goes first, except that tasks in doing/blocked and actions in pending/unknown go before settled ones. A category with fewer records than its share gives the remaining slots to the others. The returned list is still sorted newest first.

For example, with thirty checkpoints, two memories, one decision and the default limit of twenty, the previews are both memories, the decision and the seventeen newest checkpoints. `selection.returnedByKind` and `selection.omittedByKind` state that split, here thirteen omitted checkpoints. A workspace that fits within the limit is returned exactly as before.

With a `query`, `kind` or time window, `selection.order` is `recent` and previews are the newest matches, as before. Shared clients are balanced within their granted records only. `mother_search_context` is unchanged.

## Time windows

`updatedSince` and `updatedBefore` limit previews to records last updated in that window: at or after `updatedSince`, and strictly before `updatedBefore`. Either can be used alone. Each accepts RFC 3339 with an offset, such as `2026-11-11T09:30:00Z` or `2026-11-11T11:30:00+02:00`, or a plain date, `2026-11-11`, meaning midnight UTC. A time without an offset is rejected rather than guessed. Bounds are converted to UTC with whole milliseconds, rounding extra precision up, and echoed in `selection` in that form.

A window narrows `selection` and `records` only. Inventory and unsettled work stay workspace-wide, so "nothing this week" is never read as an empty workspace. `mother_search_context` accepts the same two fields and reports them under `search`.

The window is on the time a record was last saved, not on dates written inside it. A memory saved today about last year matches today.

## Earlier revisions

`mother_get_record_history` lists one record's revisions newest first: who saved each, when, its status, a 400-unit excerpt and which client-chosen fields changed from the revision before. Pass `revision` to read that version in full, or `limit` (1 to 100, default 20) to bound the list. Revisions are read from the receipts every write already keeps, so nothing new is stored and retracted or superseded records keep their history.

`currentGeneration: false` marks a revision committed before the workspace was last restored. `missingRevisions` lists any revision with no retained receipt; it is reported, never reconstructed. Shared-access clients cannot use this tool, because earlier revisions may contain text the owner did not grant them.

## Replacing a checkpoint

A checkpoint stays `active`, and so stays in recall, until something marks it `superseded`. When saving a checkpoint that replaces an earlier one, pass `supersedes: {"id": ..., "expectedRevision": ...}` using the ID and revision shown in that checkpoint's preview. Both changes commit together: the new checkpoint is saved and the earlier one becomes `superseded`, which moves it from recallable to retained history. If the earlier checkpoint changed, is already superseded, is missing or is not a checkpoint, nothing is saved.

Only one checkpoint can be named per save, and only by a checkpoint. Existing checkpoints are not retired automatically; an older pile can be cleared by superseding them one at a time or by revising their status with `mother_write_record`.

## Unsettled work

`unsettledWork` tracks tasks in doing/blocked states and actions in pending/unknown states. `unsettledWorkSummary` reports the workspace-wide total, returned count, omitted count and that scope. The existing `unsettledWorkTruncated` flag remains available.

Checkpoint next steps and open questions do not become task records automatically. An empty unsettled-work list means there are no task/action records in those states; it does not establish that every question in every checkpoint is settled. Read relevant full checkpoints to understand continuing work.

## Compatibility and upgrade

The inventory, summary, `selection.order`, `selection.returnedByKind` and `selection.omittedByKind` fields are additive. Balanced order changes which previews an unfiltered request returns only when the workspace has more recallable records than the limit. Existing context query inputs, preview limits, authorization, record categories, SQLite schema, workspace identity and receipts remain unchanged. CLI context inspection and MCP share the same response.

Running MCP processes need the updated executable and a reconnect to serve the new fields and tool description. Discovery or successful installation does not prove a particular model has used those fields correctly. Test it with an unfiltered context read and ask the agent to distinguish category counts from total context.
