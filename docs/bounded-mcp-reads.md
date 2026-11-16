# Bounded MCP reads

The native direct MCP server and local-owner stdio bridge apply the same output limits. Storage, receipts, grants and advertised tool schemas are unchanged.

- Context, search and history-list navigation JSON must fit 12 KiB in compact UTF-8 serialization, including escaping. Whole previews are omitted until it fits. The response updates returned/omitted counts and supplies `responseBudget` with the byte limit and number omitted for that reason. Search coverage metadata remains intact.
- Unfiltered balanced context removes previews from the most represented category first, keeping the surviving previews in their existing display order. Recent checkpoints cannot consume every preview slot simply because older personal memories sort later.
- Required identity, persona, pinned records, assigned tasks and unsettled-work metadata are preserved whole. If required context alone exceeds the budget, the read returns a small explicit `READ_RESULT_TOO_LARGE` error. Output limits never rewrite stored data.
- Known read replies must fit 48 KiB in the actual serialized MCP result, accounting for both structured content and its text representation. Full record and individual revision reads are complete or return `READ_RESULT_TOO_LARGE`; they are never silently shortened.
- The 48 KiB result budget leaves framing headroom beneath a 64 KiB host event limit. Hosts may impose tighter budgets. Request smaller navigation selections and inspect omission flags; use targeted searches and full record reads before claiming recall.
- Mutation and receipt outputs keep their existing contracts. A transport failure still requires receipt reconciliation before any uncertain mutation retry.

These limits belong to the transport adapter. Each harness owns its query planning, ranking, intelligence, native memory and selected sharing.
