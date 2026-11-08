# Rust port — v0.2.0

M.O.T.H.E.R is now an independent Rust executable. The SQLite store, v1 JSON client configs and six synchronous MCP tools retain their existing contracts. No AnimaOS dependency or Bun/Node runtime is required. The TypeScript baseline remains in Git at commit `6f6e0cc`.

## API mapping

| TypeScript v0.1 | Rust v0.2 |
| --- | --- |
| contracts.ts / Zod record validation | contracts.rs validated inputs, record-kind enum and normalization |
| Store.initialize / open | Store::initialize / open; same SQLite user_version 1 |
| workspace / persona | Store::workspace / persona; preserved stored snapshots |
| grant / revoke / authorize | Store::grant / grant_to_file / revoke / authorize |
| context / get / receipt / write | Same Store methods with Result errors and SQLite transactions |
| exportBundle / restoreBundle | Store::export / restore; same credential-free v1 bundle |
| loadConfig / dataDirectory | config::load_config / data_directory; same private paths |
| createServer + StdioServerTransport | Official rmcp ServerHandler and stdio transport |
| bun src/main.ts --config FILE | mother mcp --config FILE |
| bun run mother COMMAND | mother COMMAND; same owner commands and flags |

MCP names remain mother_get_context, mother_search_context, mother_get_record, mother_get_receipt, mother_write_record and mother_checkpoint. The captured input schemas, descriptions and annotations remain in contracts/mcp-tools.json. Client attribution always comes from private configuration. SQLite is bundled into the Rust executable.

## Preserved behavior

- Runtime replacement opens the existing database directly. No reinitialization, import, grant replacement, ID remapping or generation change occurs.
- Schema/table/column names, camelCase record shapes, statuses, source references, version numbers and stable IDs remain v1.
- Legacy normalized request hashes continue to recognize earlier operation IDs. Record revision comparisons and receipts commit atomically.
- Missing references roll back the entire mutation. Retracted/superseded/cancelled records remain excluded from normal context. Explicit record reads can still retrieve them.
- UTF-16 string size limits, JavaScript whitespace trimming, default arrays and integral numeric spellings are preserved. Known request field names sort identically before SHA-256 hashing.
- Updating a record subtracts its actual stored payload byte count, so differences in JSON serialization cannot drift usage accounting across runtimes.
- Unknown external action outcomes remain unknown; MCP never executes actions. A checkpoint does not pause another harness.
- Fresh-only restore validates first, then publishes schema, identity, records and receipts in one transaction. Credentials are omitted and generation changes only on restore.

## Original test mapping

| Original test in TypeScript | Native equivalent |
| --- | --- |
| store: two connections / identity / attribution | tests/store.rs::two_connections_preserve_identity_and_attribution |
| store: correction rejects stale writer | correction_rejects_stale_write |
| store: same-operation retry / changed payload | retries_survive_restart_and_payload_changes_fail |
| store: read-only and revoked clients | read_only_and_revoked_grants_fail |
| store: token/workspace/generation/actor spoofing | token_workspace_generation_and_actor_cannot_be_spoofed |
| store: retractions | retractions_are_not_active_context |
| store: missing references / operation rollback | missing_reference_rolls_back_operation |
| store: bounded context / literal search | search_is_literal_and_omissions_are_explicit |
| store: kind / create-update revisions | record_kind_and_revision_contract_is_preserved |
| store: initialization preserves existing DB | initialization_never_overwrites |
| portability: identity/persona/records/no credentials | tests/portability.rs::export_restores_persona_identity_records_unknown_outcomes_without_credentials |
| portability: invalid schema/references/duplicates/digest | malformed_bundles_are_rejected_before_destination_creation |
| portability: existing destination / retraction | restore_is_fresh_only_and_retraction_survives |
| boundaries: clock rollback | tests/portability.rs::clock_rollback_never_makes_a_correction_unreadable |
| boundaries: admission capacity | admission_bound_preserves_exportability |
| boundaries: killed restore | store::tests::interrupted_restore_never_opens_as_empty_workspace |
| CLI: init/grant/context/connection/export/restore/revoke | tests/cli.rs::owner_can_initialize_grant_inspect_export_restore_and_revoke_without_leaking_credentials |
| CLI: invalid flags/names | invalid_cli_flags_and_unsafe_client_names_fail_before_mutation |
| MCP: handoff/correction/task/restart/revocation | tests/mcp.rs::real_stdio_round_trip_handoff_correction_task_progress_restart_and_connected_revocation |
| MCP: conflicting revisions across processes | two_processes_race_one_revision_exactly_one_commit_survives_and_both_see_it_after_restart |
| MCP: duplicate-operation race/read-only | duplicate_operation_race_creates_one_record_and_read_only_clients_cannot_mutate |

Additional coverage opens a captured TypeScript SQL database and replays all original receipts; tests safe integral JSON number spellings, JavaScript search whitespace, UUID versions, UTF-16 limits and malformed tool arguments.

## Deliberate differences

- The executable uses `mother mcp --config ...`; regenerate launcher entries with `mother connection`. Existing client JSON files and tokens remain valid. The generated entry points at the executable that produced it, so generate it after putting the binary in a stable location.
- The official Rust MCP SDK negotiates its supported protocol versions, omits the older SDK's `execution.taskSupport: forbidden` catalog extension, and explicitly emits `isError: false` for success. Task-lifecycle capabilities are not advertised; the supported contract is six synchronous tools. Catalog field ordering and error prose are not byte-for-byte identical; structured error codes are preserved.
- Rust strings cannot contain unpaired UTF-16 surrogates. An excerpt stops before a surrogate pair that would straddle the 400-unit boundary instead of emitting half an emoji. Lone-surrogate JSON text is rejected. Full valid-Unicode content remains intact.
- A new instance reads SOUL.md from the current directory, with an embedded seed fallback. Existing instances always read the stored snapshot. CLI help/error formatting comes from clap, and --help/--version are available.
- Backup files remain private and unencrypted; OS account/filesystem access is the trust boundary. Revocation cannot withdraw already disclosed context. Export capacity and retained-history limitations are unchanged.

## Verification

See docs/validation.md for current commands and results. The original 21 tests passed before removing TypeScript. Its three MCP subprocess tests also passed with the Rust server. A separate bidirectional check proved TypeScript can replay a Rust-created operation and both runtimes export the same semantic data. No personal records were used in those fixtures.
