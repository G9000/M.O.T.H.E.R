# Client context policies

M.O.T.H.E.R is an independent native redb workspace. Each external harness or agent receives its own owner-provisioned credential. Its subject and scopes come from private configuration and stored grants, never tool arguments. Sharing knowledge does not replace the harness's native memory, learning or configured personality; persona adoption remains a separate host choice.

Stop the owner service before issuing grants, then restart it afterward. Create a companion credential with full canonical v1 context and the stored persona:

```powershell
mother garden grant-v1 companion-a --db "C:\private-data\mother\garden.redb" --out "C:\private-data\mother\companion-a.native.json" --access companion --subject agent-a --scopes read,context:propose
```

Create a context-only credential with an explicit allowlist of existing canonical record IDs:

```powershell
mother garden grant-v1 reader-b --db "C:\private-data\mother\garden.redb" --out "C:\private-data\mother\reader-b.native.json" --access shared --subject agent-b --records "<record-id>,<record-id>" --scopes read,context:propose
```

Omitting `--records` shares no canonical records. Companion access does not add write scopes. Shared clients can receive `read`, `context:propose` and `media:attach` for explicitly scoped read and reviewed capture. The latter is detailed in the [media bridge guide](media-bridge.md). Use separate names and private output files. Follow [service setup](garden-service.md) to bind each direct native credential to the running owner and explicitly configure its harness.

The v1 context/search `access` metadata identifies the stored policy. Shared responses omit persona; inventory, omitted counts, previews, unsettled work and search candidates cover only allowed records and label their coverage `granted_records`. Full records omit references to hidden IDs. Filters cannot expand access. Current policy and revocation are rechecked before returning data; restricted vector caches cannot reveal full-workspace coverage.

Shared proposal readers can retrieve only their own proposals while referenced/target records remain visible. Duplicate/similarity metadata is filtered. Approval does not implicitly share a newly created record. A receipt or proposal whose record became hidden returns `NOT_FOUND` rather than disclosing historical payloads. That prevents client content reconciliation; it never authorizes replaying an uncertain write. The owner can inspect native inbox state separately.

Direct native credentials use strict version-2 configs with `backend: "garden_redb"`; service configs use version 3 and `backend: "garden_service"`. Both are explicit native formats. Old SQLite configs and unknown tags fail closed, with no implicit policy conversion or database creation. Changing a credential file does not grant access to a restored generation. Portable bundles exclude grants, policies and credentials; restore requires fresh owner provisioning.

Graph scopes `garden:read`, `garden:propose` and `garden:write` and typed object policies are independent of v1 context access. A graph-only credential uses `garden grant`; a v1 grant needs explicit graph scopes and an explicit `--objects` policy or companion `--all` to access graph content. An empty graph allowlist exposes no objects. See [graph provisioning](garden-proposals.md), [contracts](garden-v2-contracts.md) and [native compatibility](garden-compatibility.md).

`garden revoke` revokes graph credentials; it is not a v1-revocation command. The retired general `mother revoke` and automatic app setup/replacement workflows have no v1 CLI replacements in this slice. Do not claim graph revocation also removed v1 authority, or edit the live database to change a policy. Current domain calls still enforce the stored grant and generation.
