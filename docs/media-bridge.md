# Shared Garden media proposals

Enable the bridge once per Garden generation. All existing and future harnesses with a valid credential and read access to the target narrative memory can propose a selected file. Each proposal records the authenticated client. Pending files and status belong to that credential, and the owner reviews the exact bytes before the Garden publishes a link. Both native stdio transports use this policy.

## Enable the Garden

On an existing Windows native-service installation, run at the repository root from Git Bash:

```bash
powershell.exe -NoProfile -File ./adapters/enable-garden-media.ps1
```

The activator builds and installs the release, restarts the identified owner with its existing arguments, enables the Garden policy, and checks matching service credentials. If the existing ChatGPT tunnel was running, it restarts that tunnel. Use `-CheckOnly` to inspect the installation. A host still running an older stdio executable needs to reconnect to pick up the new binary.

For other installations, run the updated executable with an existing service or direct credential. This sets one Garden policy regardless of which credential you use.

```bash
mother garden media bridge --config "$config" enable
mother garden media bridge --config "$config" policy
```

## Attach a selected file

Any MCP harness can submit actual file bytes with `mother_propose_media_attachment`. Supply the memory UUID, a stable proposal UUID, base64 content, and optionally the file name, media type and source file ID. Hosts with local file access can avoid putting binary data into model context by using their existing credential and an explicitly selected file:

```bash
mother garden media bridge --config "$config" propose --file "$file" --memory "$memory_id" --proposal "$proposal_id"
```

The optional `mother_propose_media_attachment_from_file` adapter accepts a host-provided file reference. ChatGPT supplies that reference through its file picker. The adapter checks the target before downloading, uses the OS curl client, limits HTTPS to OpenAI's `*.oaiusercontent.com` file hosts, refuses redirects and imposes a 25 second deadline. Other harnesses can use the portable byte tool with their own file provider. Each transfer accepts up to 2 MiB and verifies the media type. A filename or an opaque Library ID alone cannot supply the original bytes.

Use the proposing harness's credential in an owner terminal to review the target, revision, provenance, digest and embedded metadata. Type `approve`, `reject` or `skip`.

```bash
mother garden media bridge --config "$config" review --proposal "$proposal_id"
```

The service stays online during review. The harness can call `mother_get_media_attachment` to verify the canonical bytes and active link following approval. An interrupted decision can be reconciled by rerunning review. Exact submission retries keep the same proposal UUID. A stale revision needs a new proposal.

The owner-private transfer directory is adjacent to the native database and specific to its workspace and generation. Each credential can hold 24 pending files and 4,096 entries. Completed entries retain review metadata. The owner can archive completed directories while the service is stopped. Pending files and review decisions stay outside canonical recall and backups. Disabling the Garden bridge blocks submissions and status for all harnesses. A restored generation needs fresh grants and enablement.
