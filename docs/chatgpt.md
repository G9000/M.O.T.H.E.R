# ChatGPT connection boundary

The current Mother executable serves native redb storage through local MCP stdio and an authenticated local owner service. It does not provide a managed ChatGPT tunnel, HTTP bridge, runtime-key store, sign-in startup or connection dashboard. The former `connect chatgpt` workflow was removed with the legacy app.

A ChatGPT integration requires a separately supported transport that can reach the local stdio bridge and preserve its authentication boundary. This repository's current owner CLI does not configure or qualify that remote transport. Do not expose the owner service endpoint or bearer credentials over a public listener. Follow [native setup](setup.md) and [service operation](garden-service.md) for the supported local boundary.

An existing ChatGPT connection is not upgraded or verified by rebuilding Mother. Old SQLite configurations fail explicitly. Any host or external transport configuration change needs its own reviewed setup; source updates never alter those settings. Tool responses returned to a remote model leave the local machine even though canonical records remain local.

Use a separate least-authority [credential](client-access.md) and verify the actual host with an authorized read and explicitly selected capture. Protocol discovery, owner approval and model recall are separate observations. Mother does not import ChatGPT history or automatically save conversations. [Living Memory](living-memory.md) describes the supported selected-proposal workflow.

## Selected ChatGPT media

The local owner can enable two tools for one existing service credential. ChatGPT sends a selected file through its file parameter and a memory UUID. The bridge checks current memory access, downloads at most 2 MiB, verifies the media format, and stages a private proposal. The download uses the OS curl executable, HTTPS on OpenAI's `*.oaiusercontent.com` file domain, a 25 second deadline and no redirects. A file served from another domain needs the local backfill workflow. The signed URL is never saved. A Library ID alone does not contain the bytes.

For an existing Windows installation with the MOTHER native runtime and app tunnel launcher, run from Git Bash at the repository root:

```bash
powershell.exe -NoProfile -File ./adapters/enable-chatgpt-media.ps1
```

This builds and installs the executable, restarts the identified Garden owner with its existing arguments, enables the bridge only for the ChatGPT credential, and restarts its existing tunnel. The script checks the workspace generation and tunnel readiness. It retains the previous executable for recovery. Use `-CheckOnly` for a read-only installation check. For other installations, start the updated owner and run `mother garden media bridge --config "$service_config" enable`, then restart the stdio bridge. Check the actual ChatGPT tool list after deployment.

Ask ChatGPT to propose the selected attachment to the target memory. Keep the returned proposal UUID. In an owner terminal, review the filename, target memory and revision, digest, byte count and embedded metadata before typing `approve` or `reject`:

```bash
mother garden media bridge --config "$service_config" review --proposal "$proposal_id"
```

ChatGPT can call `mother_get_media_attachment` with that UUID to verify the stored bytes and active association after approval. Exact retries use the same proposal UUID. If review is interrupted, rerun the review command. The owner service stays online throughout. A stale memory revision requires a fresh proposal. Rejection discards the staged bytes.

The private transfer inbox is scoped to the workspace, generation and exact credential. It lives next to the service descriptor, outside Garden backups and canonical recall. It admits up to 24 pending files and 4,096 entries per credential. Completed entries retain review metadata. The owner can archive completed directories from the private inbox while the service is stopped. `disable` immediately blocks access for that credential and preserves pending submissions. Reissuing a credential or restoring a Garden requires explicit enablement again.
