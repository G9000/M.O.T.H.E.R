# Native owner inbox

The native executable exposes `mother garden ...` and `mother mcp --config ...`. Bare `mother` prints help. The former setup dashboard, tending screens, app discovery, connection repair and tunnel manager were removed with the SQLite runtime. Native owner inbox review remains available.

Follow [setup](setup.md) and [the local service guide](garden-service.md) to create a native store, provision separate harness credentials and start the owner service. Stop the service before direct owner database administration, then restart it afterward.

`mother garden v1-inbox --db <garden.redb>` opens v1 proposal review and `mother garden inbox --db <garden.redb>` opens graph review when both input and output are terminals. Piped invocations list proposals as JSON. Read original attribution, sources, expected revisions and reviewed content before explicitly approving or dismissing a proposal. Editing a candidate is not approval. Conflicts or uncertain writes must retain their original request rather than silently rebase or replay.

A separately reviewed JSON request can use `mother garden review --db <garden.redb> --request <review.json>`, adding `--graph` for a graph batch. MCP cannot approve proposals. See [Living Memory](living-memory.md) and [graph proposals](garden-proposals.md).

Context reads, history, pins and handoffs remain available through authorized native v1 MCP tools. The former general owner record, pin, handoff, briefing, persona-update and launch commands have no current CLI replacements. [Historical validation](validation.md) preserves earlier terminal evidence without claiming those interfaces still exist.
