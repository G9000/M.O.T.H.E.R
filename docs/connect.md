# Connect a harness

Provision each harness explicitly using the [native setup guide](setup.md), then bind its private credential to the running [local owner service](garden-service.md). Configure that harness to launch the stable Mother executable with arguments `mcp`, `--config`, and its absolute service-config path.

Mother no longer scans installed apps, edits their settings, repairs connections, manages tunnels or installs sign-in startup. The former connection dashboard and `connect`, `apps`, `fix`, `status` and disconnect commands were removed. Existing host settings are never rewritten automatically; old SQLite configurations fail explicitly.

Use a separate grant per harness and the least required [access policy](client-access.md). A successful tool listing is protocol evidence; verify context reads and selected proposal outcomes in the actual host separately. See [harness setup](harnesses.md), [AnimaOS](animaos.md), [ChatGPT](chatgpt.md) and [connection verification](health.md).
