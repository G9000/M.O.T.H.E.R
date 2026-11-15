# Graph proposals and owner approval (G5)

The native redb Garden supports selected graph proposal batches and explicit owner approval alongside its optional v1 context view. The SQLite runtime and converter are removed. This guide retains historical G5 qualification below; current query tools, v1 compatibility, full backups and local service are implemented. See [graph queries](garden-queries.md), [native compatibility](garden-compatibility.md) and [service operation](garden-service.md).

The graph inbox accepts selected entity/assertion/evidence batches for owner
review. Admission stores the original attributed proposal separately from
canonical records, history and indexes. Pending content is never established
memory. Extraction, selection and interpretation remain in each harness.

## Explicit graph setup

Graph and v1 views can share one native database with independent grants. For a new graph-only workspace use explicit private paths; stop the owner service before direct administration and restart it afterward:

```text
mother garden init --db <absolute-private-garden.redb>
mother garden grant <client> --db <garden.redb> --access companion --subject <agent> --out <new-private-config.json>
mother garden inbox --db <garden.redb>
mother garden mcp --config <private-config.json>
mother garden revoke <client> --db <garden.redb>
```

A grant defaults to `garden:read,garden:propose` and an empty graph allowlist.
`--objects <json-file>` supplies an array of typed `{kind,id}` references. Every
referenced endpoint and evidence needs its own visibility. `--all` explicitly
grants full graph visibility to a companion. Direct writes require an explicit
`garden:write` scope. Shared clients can read/propose with bounded visibility;
they cannot receive full visibility or direct-write authority.

Graph credentials use a strict version 2 configuration with the explicit
`backend: "garden_redb"` tag. Creation is exclusive and CLI output never includes
the token. Keep both database and configuration under an owner-private directory
outside the checkout. Existing v1 keys are not upgraded, imported or reconfigured.

Current graph discovery includes capability discovery and seven graph operations: get, apply, propose, get-proposal, get-receipt, query and changes. Available operations depend on the credential; definitions are not authority. The initialized v1 view is advertised separately according to current scopes. No MCP tool approves proposals, issues grants, restores backups or activates an endpoint. Graph setup alone installs no persona or model.

For multiple harnesses, one [local owner service](garden-service.md) holds the sole database handle and authenticated stdio bridges never open it. A direct `garden_redb` connection is a single-process option, incompatible with a running owner service on that file. Stop the service before owner review, then restart it afterward. Reconcile uncertain writes with the original UUID; never bypass the exclusive owner or cutover fences.

## Review and attribution

In a terminal, `garden inbox` opens native owner review; when piped, it lists JSON. An explicit reviewed JSON request can also use `garden review --db <garden.redb> --request <review.json> --graph`. The owner inbox shows each original and reviewed item, evidence, proposer and
expected revision. Owners may edit the bounded item array or add distinct item
keys, then inspect the complete batch before confirming approval. Original keys,
targets and expected revisions remain anchored to the submitted proposal;
approval does not silently rebase a conflict.

Approval uses the same canonical transaction writer as direct application. One
immediate-durability commit writes all canonical records, revisions, indexes,
stable item mappings, owner receipt and proposal closure. Current proposer
authority, grant identity, generation, visibility and revisions are checked
inside that transaction. Changed authority or conflicting state blocks approval.

Canonical metadata preserves existing contributors. Original new items attribute
the proposer; owner additions attribute the reserved owner actor. Reviewed writes
identify the owner actor and approver, with per-item owner-edited/owner-added
flags in the immutable receipt. Original proposal and reviewed result remain
distinct attributed records.

Dismissal closes only the proposal and produces no canonical mapping. Discarding
local edits does not erase shared evidence. Shared removal requires an explicit
revision-checked retract/supersede, which preserves history. The UI retains the
exact request and UUID after uncertain I/O; Enter reconciles that request while
navigation and editing are locked.

The owner list reads at most 101 rows to show a 100-row page and detect a following
page. `n`/`p` move between pages and `r` returns to the first page to discover new
proposals. Its omitted count describes other proposals outside the current page.
This is an owner inbox cursor, not a G6 stable query snapshot lease.

## Reconciliation and permissions

The admission operation receipt remains `pending` even after owner review.
`mother_garden_get_proposal` reports the current pending/approved/dismissed/blocked
outcome. Both lookups use the original admission `operationId` plus its generation;
the owner inbox separately uses `proposalId`. Retrying the identical proposal
does not create another proposal or derive a new receipt from current records.

The original authenticated proposer can obtain approved item-to-ID/revision
mappings without gaining canonical visibility. Payloads still require current
permission for the object, its endpoints and evidence; unallowlisted payloads
are omitted. Current proposal scope is required. Other clients cannot inspect
the proposal by guessing its UUID, and revoked clients cannot reconcile through
the connection. Owner review uses its own immutable operation UUID and exact
request for response-loss recovery.

## Historical G5 acceptance

Executed on Windows on 15 November 2026 with synthetic records and owner-private
temporary stores:

| Check | Result |
| --- | --- |
| Graph inbox | 14 tests passed: isolation, exact retries, review edits/additions, revocation/replacement, generation, conflicts, historical receipts, hidden evidence, pagination and audit tampering. |
| Owner review UI | 13 tests passed, including real-store lost-response reconciliation and TestBackend viewport/pagination checks. |
| Owner CLI and graph config | Three CLI tests and one strict-config test passed. |
| Graph stdio MCP | Passed submission, owner approval while connected, immutable admission receipt, IDs-only approval outcome, denied payload/direct-write access, absent approval tool and current revocation. |
| Process interruption | Twenty cases passed across initial creation, supersession, approval and dismissal, each interrupted at five transaction/sync barriers. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test --locked` | 364 passed, zero failed, three ignored. |
| `cargo build --release --locked` | Passed. |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed. |
| `cargo doc --locked --no-deps` | Passed. |
| `git diff --check` | Passed. |

The ignored tests are two optional real embedding-model tests and the private
recovery child helper, which its parent executes explicitly. Existing v1 inbox,
MCP and temporal storage tests pass in the full suite. G5 adds no dependency or
lockfile changes beyond the copied G4 baseline.

Failing regressions established missing APIs and CLI behavior, incorrect MCP
catalog loading, inaccessible owner pages, missing proposal/review audit links,
and offscreen list selection/controls before their fixes. The final audit checks
canonical history/indexes and the proposal/admission/review/operation mappings
in one authorized snapshot. Independent specification and code review found no
remaining blocking G5 findings.

This is synthetic protocol, controller, rendered-buffer and process-recovery
acceptance. It does not establish a real harness loading the connection, manual
terminal acceptance, Linux/macOS CI, hardware power-loss durability or production
scale. No live v1 store was migrated and no installed harness was configured.

## G5 completion

- [x] Pending graph batches remain outside canonical records, history and indexes.
- [x] Owner approval commits the reviewed batch, canonical receipt and proposal closure atomically.
- [x] Conflicts, revoked/replaced grants, generation changes, dismissal, edits and response-loss retry are explicit.
- [x] Receipt mappings preserve item keys and original revisions without granting unallowlisted payload access.
- [x] Owner UI separates local discard/dismissal from explicit shared retraction.
- [x] Explicit graph provisioning preserves existing v1 keys and scope meanings.
- [x] Original v1 inbox attribution and behavior remain compatible; no MCP approval tool exists.
- [x] Focused graph/v1 inbox, UI, transport, recovery and required package checks pass.

Changes remain uncommitted in the isolated `g9000/mother-garden-inbox` worktree,
carrying the verified G4 implementation forward. Existing pairing edits in the
primary checkout are untouched. The subsequent G6 milestone supplies indexed
queries/change tracking. G7 v1 compatibility/backups and later live adoption
remain separate planned slices.
