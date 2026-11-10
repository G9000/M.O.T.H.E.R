# Client context policies

M.O.T.H.E.R remains an independent workspace. Owners can bind a new private client credential to an external agent identity without depending on any particular host. Existing grants stay `legacy`; opening an older database does not change its schema version, generation, scopes, or persona access.

Create a companion credential with full canonical context and the stored persona:

```powershell
mother grant companion-a --access companion --subject agent-a --scopes read,context:propose
```

Create a context-only credential with an explicit allowlist of canonical record IDs:

```powershell
mother grant reader-b --access shared --subject agent-b --records "<record-id>,<record-id>" --scopes read,context:propose
```

Omit `--records` to share no canonical records. Each ID must already exist in this workspace. Subjects are owner-selected identifiers, never tool arguments. A companion policy does not add write scopes: choose scopes explicitly. Shared policies accept only `read` and optional `context:propose`, and cannot write canonical records. Use a separate grant for each agent. To change access in this release, revoke the old grant and provision a new named grant; do not edit the live database.

`mother_get_context` and search add `access: {version: 1, mode: "legacy" | "companion" | "shared", subject: string | null}`. Shared responses omit `persona`. Inventory, omitted/eligible counts, previews, unsettled work and literal/semantic candidates cover only allowed records; their coverage is labeled `granted_records`. Full records omit references to hidden IDs. Filters cannot expand the owner's policy. Search rechecks current policy and revocation before returning; restricted derived embeddings use separate caches so they cannot prune the workspace-wide index or expose its coverage.

Shared proposal readers can retrieve only their own proposals and only while referenced/target canonical records remain visible. Duplicate and similar-record metadata is filtered. Owner approval does not implicitly share a newly created canonical record. A receipt or proposal containing a canonical record that has since become hidden returns `NOT_FOUND` rather than disclosing historical payloads. This can prevent content reconciliation by that client; the owner can inspect the local inbox and export. An unavailable receipt does not authorize replaying a mutation.

New companion/shared private configuration files use `version: 2`, with the same flat credential fields. Legacy files remain `version: 1`. Older executables that only accept version 1 reject these scoped files before serving context. The current loader accepts both versions and authenticates version 2 against an existing companion/shared policy; a missing policy cannot silently become legacy access. MCP rechecks this requirement on every request. Use a policy-aware executable with scoped files; changing a scoped file to version 1 removes the older-binary launch guard. This guard does not constrain trusted local owners who can edit credential files or access tokens and the database directly.

The policy lives in an additive local table, alongside private grants. A separate scoped-grant requirement marker is committed with each new scoped grant. Every transactional policy read checks that marker: removing policy content or replacing it with legacy content fails closed, including withdrawal between predispatch authentication and record retrieval. Portable exports and restores omit grants, policies and requirement markers. Restores preserve canonical data and persona, create a fresh generation and require new owner-created credentials. No new MCP tool or caller-supplied agent filter is introduced.
