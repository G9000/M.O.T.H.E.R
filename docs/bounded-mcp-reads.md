# Bounded MCP reads

The native direct MCP server and local-owner stdio bridge apply the same output limits. Opt-in [contextual recall profiles](contextual-recall-profiles.md) select eligible pins before context counts, previews and output limits are applied.

- Context, search and history-list navigation JSON must fit 12 KiB in compact UTF-8 serialization, including escaping. Record previews first become compact metadata from the end of the page, then trailing previews are omitted, but at least three matching records are kept when they exist. The response updates returned/omitted counts, re-points `pagination.nextCursor` at the last record returned, and supplies `responseBudget` with the byte limit, omitted and compacted counts. Search coverage metadata remains intact. See [context retrieval](context-retrieval.md).
- Unfiltered balanced context removes previews from the most represented category first, keeping the surviving previews in their existing display order. Recent checkpoints cannot consume every preview slot simply because older personal memories sort later.
- Required identity, pin metadata, assigned tasks and unsettled-work metadata are preserved whole. Pinned content excerpts are shortened before the record floor gives way: every pin remains listed with its ID and attribution, affected entries have `truncated: true`, and `responseBudget.pinnedContentTruncated` plus `pinnedContentNote` explain the compaction. This is a literal prefix, not a generated summary. The persona stays whole unless not even one compact record would fit beside it; it is then replaced by an explicit `omitted` marker with its size, hash and how to read it alone, and `responseBudget.personaOmitted` is true. Callers can set `includePersona` and `includePinned` to false on repeated calls. If required metadata alone exceeds the budget, the read returns a small explicit `READ_RESULT_TOO_LARGE` error. Output limits never rewrite stored data.
- Known read replies must fit 48 KiB in the actual serialized MCP result, accounting for both structured content and its text representation. Full record and individual revision reads are complete or return `READ_RESULT_TOO_LARGE`; they are never silently shortened.
- The 48 KiB result budget leaves framing headroom beneath a 64 KiB host event limit. Hosts may impose tighter budgets. Request smaller navigation selections and inspect omission flags; use targeted searches and full record reads before claiming recall.
- Mutation and receipt outputs keep their existing contracts. A transport failure still requires receipt reconciliation before any uncertain mutation retry.

These limits belong to the transport adapter. Each harness owns its query planning, ranking, intelligence, native memory and selected sharing.

## Request a smaller reply

When the connected server advertises `maxResponseBytes` on `mother_get_context` or `mother_search_context`, a client can choose a limit from 4096 to 12288 bytes. This ceiling covers the serialized MCP tool result, including both the text and structured representations. Leave additional room for the host's own message envelope.

For example, this `mother_get_context` request looks for a project's current decision.

```json
{"query":"Orchard","kind":"decision","status":"confirmed","limit":20,"maxResponseBytes":4096}
```

Keep the same query or recall profile on subsequent calls. While `pagination.hasMore` is true, pass `pagination.nextCursor` back as the tool's `cursor` argument. Context uses recent order by default for these calls, and also accepts created order. Search preserves its ranking. If the server returns `CURSOR_STALE`, restart from the first page. In particular, a large group of records saved at the same timestamp has a compact cursor that detects changes to that group.

The returned previews identify their records and authors. Check `responseBudget`, the pagination counts and individual truncation flags before interpreting a page as complete. A smaller response can shorten excerpts and omit optional sections, including persona text and pinned content. The original Garden records remain available subject to the current grant.

### Read a complete original in smaller pieces

Use `mother_get_record` when the full record fits. If a client needs a hard byte ceiling for the original as well, discover `mother_read_original` and request a record by ID. To read the persona, use `"source":"persona"` and omit `id`. Shared clients are limited to their granted records.

```json
{"source":"record","id":"<id returned in a preview>","maxResponseBytes":4096}
```

Append each returned `fragment` in order and pass `nextCursor` as `cursor` with the same source until the cursor is null. Parse the concatenated string as JSON once, after the final fragment. The digest identifies the serialized source across pages. If the source changes, discard the partial result and read it again from the beginning.

## Pinned-context regression qualification, 17 November 2026

Synthetic presentation tests cover three and twelve pins with escaped multilingual text, stable metadata and attribution, deterministic UTF-8 prefixes, unchanged small responses, filtered reads with no previews, and explicit failure when required non-content fields cannot fit. A real stdio subprocess test creates and pins twelve synthetic memories, checks the context and wire budgets after three and twelve pins, and reads every complete canonical record back. Twelve is a pin-count limit, not a guarantee that an arbitrary persona plus all required metadata fits the response budget.

The initial working-tree suite passed 250 tests with six intentional ignores, including concurrent evaluation work. Isolated publication qualification of this fix alone passed 243 tests with six intentional ignores on Windows using an owner-private synthetic temporary root. Formatting, release build, strict Clippy and the normal/build SQLite dependency exclusion audit passed. Shared stale Cargo artifacts were avoided with a separate target directory. The synthetic qualification did not change existing private-folder ACLs, installed executables, running services, live pins or canonical memories.

The owner subsequently authorized installing the verified build and restarting the existing ChatGPT bridge. Actual app context and full pinned-record reads succeeded after reconnection, with unchanged workspace identity, persona, inventory and pins. Existing credential and launcher hashes were preserved, and the database owner stayed running. Multiple personal memories were not repinned as a live test.
