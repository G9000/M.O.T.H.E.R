# Memory Gardening v2

Memory Gardening gives you a short list of records worth revisiting. It works on an authorized slice of the native v1 context and proposes linked annotations through the Living Memory inbox.

## Run a pass

Use a direct native or owner-service configuration that can read v1 context. Create an empty owner-private directory outside source control for the pass state. Use the same directory, reviewing connection and scope on later runs.

```sh
mother garden tend --config /private/reviewer.json --state /private/gardening-state
```

The default result is a dry review written as JSON. It leaves canonical records, pins, links, and the review inbox unchanged. It reads up to 80 navigation entries and resolves their full authorized records. Use **--limit** to choose up to 100 navigation entries, or repeat **--source RECORD_UUID** to select up to 100 specific records. An existing scheduled job can run this same command without action flags. Keep report output private.

The pass returns up to 12 unhandled findings. **hasMoreFindings** indicates another available item in the same reviewed slice. After sending or dismissing items, rerun to get the next findings. The pass does not evaluate the rest of the workspace.

## Interpret and inspect

| Finding | Why it appears | What to check |
| --- | --- | --- |
| Overlapping accounts | Two records on the same normalized title have substantially overlapping wording. | Read the original wording, dates and authors side by side. Similarity does not establish a duplicate. |
| Possible contradiction | Two short, declarative statements for the same title match after removing one recognized negation. | Check timing, wording and evidence. Both accounts remain available. |
| Possible milestone gap | A record contains a line like **Milestone: Published drawings**. The selector sees no linked source or separate matching support in this reviewed slice. | Read the cited record. Add a supporting source if you have one, or leave the outcome open. |
| Stale follow-up | An open task has gone at least 30 UTC calendar days without an update. | Check whether the task is still relevant. Use **--stale-days** to choose a different threshold or **--as-of YYYY-MM-DD** to reproduce a pass. |

These local rules surface possibilities and can miss subtler cases. A gap means the reviewed material lacked separately identified support. It does not establish that an event happened, failed to happen, or lacks support elsewhere. Hidden sources, their IDs, titles, and raw content are not part of the report. The rule never follows an inaccessible milestone link.

The report names the configured reviewing agent. Each finding identifies its canonical evidence by **evidence[].id** and **evidence[].revision**, with the existing attributed actor and dates. To inspect a finding in the connected harness, call **mother_get_record** with each evidence ID. Compare the returned revision to the finding, then read the full original wording and sources side by side. If the revision differs or the record is unavailable, rerun the pass before acting. Authorized historical readers can use **mother_get_record_history** with the original ID and revision for context.

## Dismiss or submit

Copy the chosen finding ID from a report. Keep the same source and date options when acting on it.

```sh
mother garden tend --config /private/reviewer.json --state /private/gardening-state --dismiss FINDING_UUID
mother garden tend --config /private/reviewer.json --state /private/gardening-state --propose FINDING_UUID
```

Local dismissal records only a disposition. Submission requires a v1 review grant such as **context:propose** with **read**. It rechecks access and exact source revisions. It sends a single linked memory annotation that identifies the reviewer and labels the finding as an interpretation. The owner can approve, edit or dismiss it in the [Living Memory inbox](living-memory.md). Review and approval recheck all bound source revisions and current access. Approval adds the annotation and keeps the originals intact. Dismissal changes neither source. A stale or unavailable proposal can still be dismissed.

The finding ID is the stable admission operation ID for the same workspace generation, finding type and source revisions. After an uncertain submission result, rerun the same command. It checks the original receipt before trying an unchanged request. An admitted or dismissed item does not reappear for the same inputs. A source revision can create a new finding. State is bound to the workspace, restore generation and authenticated reviewing agent. It keeps disposition IDs and identity only, and refuses concurrent use or a mismatched identity.

Update the owner service along with the command before submitting through a service connection. The command requires an owner that advertises and enforces revision-bound evidence review. This slice uses the v1 inbox and needs no graph grant or new scheduler.
