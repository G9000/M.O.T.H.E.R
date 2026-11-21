# Life Timeline and Memory Time Travel

Use **mother garden timeline** to browse dates and inspect what the Garden knew on a past day. You can use an existing native Garden credential or a credential for the local owner service. The connection needs current **garden:read** access. A context-only v1 connection does not have graph history access. The timeline reads native entities, assertions and evidence. Records from the v1 explorer appear here only if represented in native Garden data.

## Browse dates

```text
mother garden timeline list --config <private-config> --from 2025-02-28 --through 2025-03-01 --kind assertion --predicate example:milestone
```

- **--from** and **--through** are inclusive calendar dates in UTC. Assertions match when their validity interval overlaps the selected days. An open start or end extends in that direction. The stored validity end is exclusive. For example, an assertion ending at midnight on March 1 is outside a range that starts on March 1.
- Evidence uses its stored source-observation timestamp. Entities and records without a known event date appear as unknown and remain discoverable even with a date filter. Add **--dated-only** to exclude them.
- Month and year precision are labeled approximate. Filtering uses the stored bounds for assertions and the stored observation timestamp for evidence. It does not invent dates inside a month or year. The view preserves any recorded source timezone and shows when each revision was committed separately.
- Within each page, entries with a known start sort by the actual event time. Records without a start appear after them. Page boundaries follow the stable native candidate order, so successive pages are not globally sorted by event date. When a page has a next cursor, repeat the same command with **--cursor**. Date filtering happens within the page, so a page can show no matches and still have a next cursor. Use **--kind** or **--predicate** to narrow the native scan. **--limit** sets the number of candidates per page from 1 to 24.

The command prints a readable list in a terminal and JSON when piped. Add **--json** to request JSON explicitly.

## Inspect an account and its sources

```text
mother garden timeline list --config <private-config> --known-on 2025-03-05 --predicate example:milestone
mother garden timeline inspect --config <private-config> --kind assertion --id <claim-id> --revision 1
mother garden timeline inspect --config <private-config> --kind evidence --id <source-id> --revision <source-revision>
```

**--known-on** uses the end of the selected UTC day as the transaction cutoff. For a fictional opening entered March 3 as happening March 1, the March 5 list shows that first account and its March 3 evidence. If the opening is corrected on March 10 to February 28, the March 11 list shows the correction and its March 10 evidence. The earlier account remains available with its original revision and contributor, subject to current access.

The list and comparison show up to three evidence previews per claim, with each source locator, contributor, revision and record date. **inspect** shows the full authorized record and its evidence links. Choose an exact claim revision with **--revision**, or use **--known-on** to reconstruct the record at that date. Inspection resolves evidence at the claim's recorded time for an exact historical revision and at the selected cutoff for a known-on request. The command displays source locators without opening or fetching them.

## Compare two cutoffs

```text
mother garden timeline compare --config <private-config> --earlier 2025-03-05 --later 2025-03-11 --predicate example:milestone
```

Compare shows supporting revisions for additions, corrections, supersessions and retractions. Date filters match a change whose earlier or later event overlaps the range. A row is marked added only if its authorized first revision was recorded between the cutoffs. A record that was already present but whose old revision is outside the current grant cannot appear as newly added. Earlier and later accounts must both be authorized when drawing a comparison about a preexisting record.

Comparison considers the first bounded page of up to 16 authorized candidates at each cutoff. Partial coverage means the report is an excerpt of possible changes. Narrow the kind or predicate to focus it. The date and page limits apply to the retained, authorized view. The results do not inventory inaccessible records or establish that missing history never existed.

Both periods and every supporting record use the current credential and graph policy. If a grant changes, the native cursor cannot be reused to recover its former view. Changes during a multi-read operation can require restarting it. These reads do not restore the workspace, modify canonical records, or send data to any remote service. The local query engine manages its normal paging leases.
