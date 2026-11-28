# Verify a local connection

Run `mother garden connection-check --config <host-service-config.json>` with the same service credential as the host. The report includes the check time, the stage reached, whether the owner authenticated, the result of a one-record probe and a next step. A failed probe exits nonzero. The report omits record contents, credentials and private paths.

`hostActivity` reports observations for that credential since the owner service started. Tool discovery sets `lastHostRequestAt`. A supported host-requested context, record, project or graph read updates `lastRead` with its time and outcome. `lastRecordReadAt` records the last time the bridge prepared a successful record-bearing reply. An empty query or a command-line probe does not set that timestamp. Delivery beyond the bridge and model use require evidence from the host. Give each host its own credential for distinct observations.

The former `mother doctor` command and terminal health screen were removed. Use the intended harness's MCP connection inspection with the native stdio bridge configured as described in [setup](setup.md) and [service operation](garden-service.md).

Check the executable path, its identity, the selected private service config, running owner service and expected workspace/generation. Discover all tool pages by following `nextCursor`, then perform an authorized context or graph read. Native discovery is permission-aware; a listed contract does not confer its scope. Never publish credentials, private record content or raw private config files as diagnostic output.

Successful initialization and discovery show that a local protocol session reached the executable. An actual authorized read establishes that session's read path. A real host/model using the result, a selected proposal, explicit owner approval and cross-host recall each need their own evidence. Missing, denied, unsupported and inconclusive outcomes must remain explicit.

Use the original operation UUID and exact request to reconcile uncertain writes through their receipt/proposal tools. Reconnecting never authorizes automatic replay. [Validation](validation.md) separates source, synthetic protocol, platform and actual-host coverage; no legacy diagnostic command is required by the native workflow.
