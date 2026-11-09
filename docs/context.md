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
| `selection` | The applied query/kind filters, matching count, returned preview count and omitted matching count. |
| `summary` | A brief description of workspace counts and preview coverage. |

`records` remains a bounded preview list, with the existing `omitted` and per-record `truncated` fields. To read a complete current record, use `mother_get_record` with its ID. Default recall does not emit retracted content. Inventory does not count unreviewed Living Memory proposals as accepted records.

For example, if a workspace has three checkpoints and no memories, a request filtered to `kind: memory` returns no previews, but its inventory still reports three recallable checkpoints. Neither that filter nor `limit: 1` makes the workspace inventory empty. Do not describe a filtered or truncated preview page as the whole workspace.

## Unsettled work

`unsettledWork` tracks tasks in doing/blocked states and actions in pending/unknown states. `unsettledWorkSummary` reports the workspace-wide total, returned count, omitted count and that scope. The existing `unsettledWorkTruncated` flag remains available.

Checkpoint next steps and open questions do not become task records automatically. An empty unsettled-work list means there are no task/action records in those states; it does not establish that every question in every checkpoint is settled. Read relevant full checkpoints to understand continuing work.

## Compatibility and upgrade

The new inventory and summary fields are additive. Existing context query inputs, preview limits, authorization, record categories, SQLite schema, workspace identity and receipts remain unchanged. CLI context inspection and MCP share the same response.

Running MCP processes need the updated executable and a reconnect to serve the new fields and tool description. Discovery or successful installation does not prove a particular model has used those fields correctly. Test it with an unfiltered context read and ask the agent to distinguish category counts from total context.
