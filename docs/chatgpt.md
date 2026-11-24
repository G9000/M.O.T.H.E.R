# ChatGPT connection boundary

The current Mother executable serves native redb storage through local MCP stdio and an authenticated local owner service. It does not provide a managed ChatGPT tunnel, HTTP bridge, runtime-key store, sign-in startup or connection dashboard. The former `connect chatgpt` workflow was removed with the legacy app.

A ChatGPT integration requires a separately supported transport that can reach the local stdio bridge and preserve its authentication boundary. This repository's current owner CLI does not configure or qualify that remote transport. Do not expose the owner service endpoint or bearer credentials over a public listener. Follow [native setup](setup.md) and [service operation](garden-service.md) for the supported local boundary.

An existing ChatGPT connection is not upgraded or verified by rebuilding Mother. Old SQLite configurations fail explicitly. Any host or external transport configuration change needs its own reviewed setup; source updates never alter those settings. Tool responses returned to a remote model leave the local machine even though canonical records remain local.

Use a separate least-authority [credential](client-access.md) and verify the actual host with an authorized read and explicitly selected capture. Protocol discovery, owner approval and model recall are separate observations. Mother does not import ChatGPT history or automatically save conversations. [Living Memory](living-memory.md) describes the supported selected-proposal workflow.

For selected files, the ChatGPT adapter uses the same [Garden media proposals](media-bridge.md) as other MCP harnesses. Its file picker supplies a temporary download reference. The shared owner reviews the attachment before publication.
