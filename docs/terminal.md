# Your terminal home

Run `mother` in a terminal, or `mother tui`, to open M.O.T.H.E.R's dashboard. Piped invocations of bare `mother` still print help; all existing CLI and MCP commands remain available. M.O.T.H.E.R remains an independent context layer with your existing local database and owner-supplied persona.

## Keys

| Key | Action |
| --- | --- |
| n | Connect an AnimaOS agent through local MCP |
| t | Test this executable's local MCP read using an existing client config |
| p | Open the pinned core and handoffs (also `mother tend`) |
| i | Open the Memory Inbox for Save, Edit or Dismiss |
| s | Set up or inspect the ChatGPT connection |
| c | Connect using saved settings and credential |
| d | Disconnect this workspace's managed runtime |
| r | Refresh observed status |
| a | Review an explicit Windows sign-in startup change |
| h or ? | Connection help and ChatGPT verification steps |
| q or Esc | Leave the dashboard; an established connection keeps running |

In setup, use Tab/Shift+Tab or the arrow keys to move between fields. Ctrl+U clears a field. Enter reviews the settings before saving; Escape returns or cancels. Key input is masked, never included in the review screen, and stored in the native OS credential store. Blank key input preserves the existing credential for that connection.

## Tend the pinned core and handoffs

Press **p** at home, or run `mother tend`. This is your own view of what every harness reads first, and it needs no harness or grant.

| Key | Action |
| --- | --- |
| Tab / Shift+Tab | Switch between Pinned, Memories, Decisions and Tasks |
| Up / Down | Select a record; its text, who last saved it and its ID show on the right |
| p | Pin or unpin the selected memory or decision |
| a | Hand the selected open task to a harness, or to nobody |
| r | Refresh |
| Esc | Return home |

At most twelve records can be pinned. Pins and handoffs you make here are recorded as `@owner`. A harness finds a task handed to it the next time it reads context; nothing runs by itself. The list shows the 500 most recently saved records.

The same controls work as plain commands, which print JSON and may show private text:

```powershell
mother records --kind memory --query tea
mother pins
mother pin <record-id>
mother unpin <record-id>
mother handoffs
mother assign <task-id> --to codex
mother assign <task-id> --clear
mother history <record-id>
mother history <record-id> --revision 1
```

`mother records` is how you find a record's ID. Each command accepts `--db` for a workspace outside the default location.

## Connect ChatGPT once

Install OpenAI's official tunnel-client and create a private tunnel associated with your Platform organization and target ChatGPT workspace. Create a separate M.O.T.H.E.R ChatGPT client grant and keep its private config. The [ChatGPT guide](chatgpt.md) covers these prerequisites.

Run `mother connect chatgpt --setup`. Supply the tunnel ID, optional organization ID, absolute client-config path, installed tunnel-client path, stable mother executable path and OpenAI runtime API key. Review and save, then press c. No cloud account, tunnel, grant or credential is created silently. The native connection manager replaces the need for a private PowerShell launcher.

Use a stable installed mother executable for the connection. A running managed tunnel survives the dashboard closing, but your computer must remain on. Startup at Windows sign-in is off by default and requires an explicit action. macOS/Linux startup integration is not provided; connection commands remain available. The native credential store must be available; there is no plaintext or in-memory fallback masquerading as saved credentials.

Non-secret connection settings live under the private MOTHER data directory. Each configured identity gets its own runtime alias and profile. M.O.T.H.E.R checks ownership before connecting or stopping; it does not take over an unrelated tunnel or the earlier PowerShell-managed alias. An unknown result requires inspection before another effect. Disconnecting stops transport but keeps your settings and credential for reuse.

## Scriptable commands

Press **t** for a bounded, read-only [connection health check](health.md), or run `mother doctor --config <private.json> --json`. Its local MCP report includes executable identity and category counts; actual host conversations require separate verification.

```powershell
mother connect chatgpt
mother status
mother status --json
mother disconnect chatgpt
mother startup status
mother startup enable
mother startup disable
```

Without a terminal, connect uses only previously saved settings and credentials. It never prompts for a secret. Missing setup or a connection that is not ready returns a failure. `mother status --json` reports the observed state and readiness signals without returning keys or raw subprocess logs.

Local readiness means the intended runtime is running, healthy and ready. It does not prove that a ChatGPT conversation can read and save memory. Only after readiness, add M.O.T.H.E.R in ChatGPT with **Connection: Tunnel** and **Authentication: No authentication** for this stdio server. The tunnel still authenticates with OpenAI and M.O.T.H.E.R still validates its separate local grant. Then complete the [cross-harness acceptance test](chatgpt.md#verify-reads-and-saves-across-harnesses).

The Memory Inbox reviews proposed context with explicit Save, Edit and Dismiss actions. Run `mother inbox` to open it directly; see [Living Memory](living-memory.md). The dashboard does not change SOUL.md, import chats or execute saved actions. Use the existing [owner commands](setup.md) for workspace administration.

If a previously configured executable or client-config file disappears, M.O.T.H.E.R preserves the old configuration until its runtime can be reconciled. Restore the expected file or inspect the old alias with the official tunnel client before changing paths; setup does not guess that an unreachable runtime has stopped.

## AnimaOS

Press **n** at home or run `mother connect animaos` for masked owner authentication, agent selection and local MCP setup. See [the AnimaOS guide](animaos.md). The adapter is optional; M.O.T.H.E.R remains independent.
