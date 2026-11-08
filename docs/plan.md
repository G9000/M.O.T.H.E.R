> Historical v0.1 TypeScript bootstrap plan, completed before the Rust port. Current implementation and validation are in rust-port-plan.md, setup.md and validation.md.

# Independent workspace implementation plan

**Goal:** Give the owner a usable private context store shared by local MCP harnesses without AnimaOS.
**Architecture:** Domain schemas and one transactional SQLite store, wrapped by MCP and CLI adapters. Local grants bind every operation to an actor and scopes. Private data defaults outside the Git checkout.
**Stack:** Bun, bun:sqlite, TypeScript, Zod, official MCP TypeScript SDK v1.

## 1. Persistence and contract
- [x] Add package/TypeScript configuration and ignore private state.
- [x] Write failing store tests for persistence, corrections, stale writes, operation receipts and authorization.
- [x] Implement src/contracts.ts, src/errors.ts, src/store.ts and src/schema.ts. Use synchronous immediate transactions and a versioned database schema.
- [x] Pass the store tests and typecheck.

## 2. Portable recovery
- [x] Add failing tests for consistent export, fresh-only restore, schema validation, stable references, changed generation, and omitted grants.
- [x] Implement src/portability.ts with validated credential-free bundles and no overwrite of existing state.
- [x] Pass restore tests, including unknown action outcomes and retractions.

## 3. Harness interface
- [x] Write failing tests for a real two-client MCP continuation over stdio.
- [x] Implement src/server.ts and src/main.ts with read/context/search/write/checkpoint tools; errors return structured codes and no false success.
- [x] Verify independent client attribution, denied writes, revocation, restart and replay through real protocol calls.

## 4. Owner setup
- [x] Add src/cli.ts and src/config.ts: init, grant, revoke, context, export, restore and help. Require explicit paths for exports and fresh restores; use exclusive file creation.
- [x] Add SOUL.md, AGENTS.md, adapter guidance and docs/setup.md. Keep README unchanged.
- [x] Verify CLI setup from an empty temporary directory, check private paths are ignored, and run all tests plus typecheck.
- [x] Review code and resolve substantive findings. Record actual evidence and remaining host/deployment limits in docs/validation.md.

Implementation stays on g9000/independent-workspace. Do not change the owner's live harness settings, import personal history, or publish a remote service automatically.

Review refinements: export/restore includes the initialized persona and embedded text artifacts, with external bytes explicitly omitted. Two concurrent MCP processes race both conflicting revisions and identical operation IDs; connected-client revocation is enforced on subsequent calls. All domain states/capabilities are defined in src/contracts.ts and docs/design.md.
