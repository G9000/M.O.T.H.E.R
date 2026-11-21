# Living Memory Graph

Browse how recorded ideas connect and inspect the evidence that the current connection is allowed to read. The relationship viewer uses native Garden graph queries. The [3D garden explorer](garden-explorer.md) remains a separate view of native v1 records.

## Open a neighborhood

Use an existing native connection with Garden read access. The configuration can point to the running local owner service or directly to its database. While the owner service is running, use the service configuration.

```sh
mother garden relationships --config /private/reader.json --entity ENTITY_UUID
```

To start from a v1 memory, pair its record ID with the authorized Garden entity that represents it. Copy the memory ID from its record and the entity ID from its approved graph operation receipt or a current authorized graph read. Record the association in an owner-private mapping file outside source control. The example contains fictional IDs.

```json
{
  "version": 1,
  "workspaceId": "a62ebe25-cac4-4e0c-9612-86a05be639e6",
  "generation": "7f8e2a2f-d645-4717-82e8-9fca33928b1a",
  "mappings": [
    {
      "memoryId": "c1cc7732-fdd8-48f5-9139-b24b71925222",
      "entityId": "d19882f2-032e-4444-b21a-3c7c0ce4200a"
    }
  ]
}
```

```sh
mother garden relationships --config /private/reader.json --memory MEMORY_UUID --mapping /private/graph-map.json
```

The file is the owner's explicit association. The command checks workspace and restore generation, requires unique memory IDs, and checks both records against current access. It does not match by title, guess a shared UUID, create an entity or extend the grant. If the memory has no representation yet, propose an entity through the [graph inbox](garden-proposals.md) and map the approved ID. Verify the association against the intended workspace again after a restore.

The interactive list uses arrow keys. Up and Down select a link, Page Up and Page Down scroll its attribution and evidence, and Left and Right follow the selected link to its source or destination. Backspace returns to the previous idea. Press R to refresh and Q to quit. Piped output is JSON from the same bounded view.

The neighborhood traverses up to two hops and 24 authorized edges in both directions while displaying each stored direction. Cycles terminate without repeating assertions. The panel includes the assertion ID and revision, original contributor, current revision actor, owner approver when present, and up to three available evidence objects with source and excerpt. It shows other authorized entity relationships too. Labels and excerpts are bounded display copies. Use the authorized graph Get operation with the shown ID and revision for the full record.

A missing mapping or an unreadable entity yields a message without exposing the missing record. An empty result is scoped to the granted neighborhood. Partial coverage means the query hit a work, edge, page or response bound. The view omits hidden endpoints and relationships with missing or inaccessible evidence. It does not list or count what was hidden. A context-only connection explains graph unavailability. The terminal checks that access and the graph snapshot are still current before drawing cached details, and clears the view if they change.

## Suggest a relationship for review

Use the same authorized connection with Garden proposal access. Choose an operation UUID once and keep it with the command for receipt lookup.

```sh
mother garden suggest-link --config /private/proposer.json \
  --from EARLIER_ENTITY_UUID --to LATER_ENTITY_UUID \
  --relationship evolved_into --evidence EVIDENCE_UUID \
  --operation-id OPERATION_UUID
```

Each suggestion creates one directed assertion proposal. Repeat **--evidence** to attach up to eight currently authorized graph evidence IDs. Omit evidence when the suggestion has no supporting evidence, and the review panel will show that absence. The proposer comes from the authenticated connection.

| Relationship | Read from subject to object |
| --- | --- |
| related_to | The subject has a meaningful connection to the object. The claim can be symmetric; the viewer preserves the stored direction. |
| inspired_by | The subject took inspiration from the object. Point from the later idea to its earlier source. |
| contradicts | The subject conflicts with a claim or position represented by the object. Keep each claim and its evidence independently reviewable. |
| evolved_into | The earlier subject developed into the later object. |
| supersedes | The newer subject took the place of the older object in the documented history. This records a relationship and leaves both endpoint lifecycles intact. |

The command reports the original admission and proposal ID. A replay can return the admission's original pending status even when the current outcome has changed. Use **mother_garden_get_proposal** with the displayed operation ID and operation generation for the current outcome.

The owner can open **mother garden inbox --db /private/garden.redb** to review and approve or dismiss the suggestion. Stop the local owner service before opening the database directly for review and restart it afterward. Only approval makes the assertion traversable. Dismissal leaves the graph unchanged. Superseding or retracting a canonical object requires a separate revision-checked lifecycle operation. This command never submits that operation.

## Correct an accepted link

Read the current link and confirm its ID and revision. Run **suggest-link** with all desired relationship fields and a fresh operation UUID, adding **--correct ASSERTION_UUID --expected-revision NUMBER**. Omit evidence arguments to retain existing evidence. Supply an evidence list to replace the current list. Approval creates a new revision on the same assertion ID. The original contributor and prior revision, including its evidence, remain in history and continue to require current authorization to read. Review retains the attribution of the correcting proposal.

If admission is uncertain, look up **mother_garden_get_receipt** with the original operation ID and generation before any retry. Use the identical request if a retry is needed. If the correction's target has already changed, use the original receipt or current proposal outcome to reconcile. A stale correction is never silently rebased.
