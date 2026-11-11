# Balanced context previews

**Problem.** `mother_get_context` previews the newest recallable records of any category. Checkpoints are written every session; once they outnumber the preview limit, a harness's first read contains no memories or decisions.

**Decision.** With no `query` and no `kind`, categories take turns for the limit (checkpoint, task, decision, memory, action, artifact), newest first within each, unsettled tasks and actions before settled ones. Unused shares pass to the other categories. The returned list stays newest first. `selection` gains `order`, `returnedByKind` and `omittedByKind`.

**Unchanged.** Tool inputs, limits, the v1 schema, authorization, shared-client filtering, inventory, unsettled work, filtered requests (newest matches) and `mother_search_context`. A workspace within the limit returns exactly what it did before.

**Not included.** Pinning or ranking by importance, automatic checkpoint supersession, record history and time filters. Each is a separate change.

**Acceptance.** `tests/balanced_context.rs`.
