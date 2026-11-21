# Documentation changes

## 22 November 2026

- Added the [visual-memory harness skill](../skills/visual-memory/SKILL.md) and described it in the [visual memory guide](visual-memory.md), along with interpretations shown in media listings and the gallery.

- Documented staged ingest of large recordings, `--stage-above`, interrupted-stage reports, the backup guard and `garden media cleanup` in the [visual memory guide](visual-memory.md).

- Extended the [visual memory guide](visual-memory.md) to audio and video: accepted formats, streamed ingest, metadata reports, transcripts, gallery playback and the MCP audio and video rules. Marked M10 implemented in the spec.

- Documented `garden media export-all` and `garden media import` in the [visual memory guide](visual-memory.md), and marked M11 implemented in the spec.

- Added the [visual memory guide](visual-memory.md): image provenance, owner attachment, stripped derivatives, verification, the `--media` explorer gallery, reviewed attach-from-conversation, attributed interpretations and the separately scoped media MCP tools. Linked it from the backup guide and marked the [visual memory spec](superpowers/specs/2026-11-22-visual-memory-design.md) implemented, noting where the build differs from the plan. Checked with synthetic images only.

## 16 November 2026

- Added the [memory garden explorer guide](garden-explorer.md) for the new read-only `mother garden explore` command, and noted in the memory garden design note that a first version now exists. Checked with synthetic gardens only; no real workspace was opened. The guide now describes the walkable painted valley (Patch's Painted valley, MIT code and CC0 art) that replaced the first meadow page, its source in `web/garden/` and how the embedded page is rebuilt. A later pass the next day made the place the garden's own: one hill whose spiral path is the timeline of records, first-person only, with the borrowed valley, village, wanderer and physics removed.

## 15 November 2026

- Reconciled current setup, harness, context, retrieval, review, recovery and contributor guides with complete removal of the SQLite runtime, converter and Cargo feature. Native redb is the default and sole runtime; v1 MCP compatibility and native owner inbox review remain supported.
- Retired instructions for removed setup/app/tunnel/launch/model and general owner CLI surfaces. Historical cutover journals/fences retain status, exact grant reissue and eligible rollback; source preparation/activation are unavailable. No live data, installs or harness settings are changed by this documentation update.
- Preserved source-specific benchmark, plan and validation evidence. Older dual-runtime commands and counts below describe their historical source, not current support or acceptance.

- Qualified full graph-tool admission in the separate additive AnimaOS worktree: exact schemas, bounded offline reference traversal and typed integer argument conversion. Added actual native graph executor and daemon HTTP coverage. Preserved G9 evidence, the intermittent legacy restart failure and all remaining release holds.

- Qualified native context-only AnimaOS admission through credential-bound discovery, preserving v1 schemas, graph contracts and host budgets. Replaced whole-store audit heap copies with exact streaming checks; a retained 100k graph copy now audits within the unchanged memory guard. Original G9 failure evidence remains available.

- Reconciled context, design, Living Memory, retrieval and harness guides with the collector/keeper boundary, additive sharing, optional persona and separate proposal, approval, indexing and host-recall states.
- Documented native graph/v1 compatibility, the SQLite-free feature build, retained default legacy runtime/converter, single owner service and stop-service requirements for direct database administration. Clarified native inbox/review and legacy-only CLI surfaces.
- Linked explicit migration, fresh generation, source fences, exact credential reissue and rollback refusal after canonical destination writes. Preserved earlier milestone evidence and linked the current G9 qualification report without treating historical results as current acceptance.
- Kept Release A's separate AnimaOS implementation evidence distinct from Releases B/C's native runtime and scale qualification.
- Recorded failed scale and native AnimaOS schema-admission gates. Documented bounded native discovery pages and the native build's isolation from the optional model downloader/TLS dependencies.

## 14 November 2026

- Added the "Connect your apps" guide for the new home screen, `mother apps`, `mother connect <app>` and `mother fix`, and the AnimaOS owner-key lookup and old-connection repair. Linked it first from the README.

## 12 November 2026

- Documented the owner's tend screen and the `records`, `pins`, `pin`, `unpin`, `handoffs`, `assign` and `history` commands in the terminal guide, with pointers from the shared-context guide and design notes.
- Added the memory garden design reference: a standalone mockup with invented data under docs/mockups and a short spec. It is not built into the executable.

## 11 November 2026

- Added the hearth guide for owner-declared launchers and `mother launch`, recorded the decision to allow owner-triggered launches in the design notes and contributor guide, and linked it from the harness guide.
- Documented addressed handoffs between harnesses in the shared-context guide, harness guide, setup tool table, session instructions and design notes.
- Added the harness guide covering `mother brief` and setup for Claude Code, Codex, Hermes, OpenClaw, ChatGPT, AnimaOS and other MCP clients, and linked it from the README and setup guide.
- Documented pinned core memories in the shared-context guide, setup tool table, session instructions and design notes.
- Version 0.5.0. Documented balanced context previews, checkpoint supersession, record history and `updatedSince`/`updatedBefore` windows in the shared-context guide, setup tool table, session instructions, search guide and design notes. The setup tool table now also lists the two proposal tools. The AnimaOS guide states that `localhost` is accepted. Validation entries record each change and what was not exercised.
- Described the artwork for screen readers, named the runtime in one line and linked the semantic search, client access, connection health, design, validation and changes guides from the README. Title, poem and artwork are unchanged.

## 10 November 2026

- Refined the poem's opening to echo its sheltering imagery and linked the shared-context guide from the README.

- Added workspace-wide context inventory/category counts, explicit selection coverage and task/action unsettled-work totals so zero memory-category records is not mistaken for an empty workspace.

- Refreshed the centered poem with the first mother's candle and cradle imagery, preserving the artwork and closing lines.
- Added brief local memory, SOUL, review inbox and setup links to the README.
- Removed the obsolete porting document and repaired its references.

## 9 November 2026

- Added Living Memory: explicit MCP proposals, owner TUI review through `mother inbox`, durable approval/dismissal receipts, stale-correction checks and portable inbox recovery.

- Added native `cargo dev` and `cargo release` shortcuts; no Bun or Node tooling is needed.

- Added the native terminal dashboard, connection wizard, secure OS credential storage, typed status, disconnect and optional Windows sign-in startup guide.
- Replaced the private PowerShell setup flow with native connection commands and specified ChatGPT's No authentication choice for the private stdio tunnel.

- Added the private ChatGPT tunnel setup, credential handling, runtime lifecycle and cross-harness read/save acceptance guide; recorded the current unverified steps.
- Linked the guide from setup and the README, replacing the separate-deployment wording.
- Added the centered M.O.T.H.E.R expansion: Memory · Ontology · Temporal · Hearth · Experience · Remembrance, preserving the poem and artwork.
- Clarified that downloaded model assets live in the private data directory, not the source checkout.
