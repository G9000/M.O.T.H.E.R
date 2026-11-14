# Connect your apps

Run `mother` in a terminal. The home screen lists every agent app and whether it can read your memory:

```text
 ✓ Claude Code      connected
 ✓ Codex            connected
 ! Hermes           needs fixing: its access was revoked or is no longer valid
 ○ Claude Desktop   installed, not connected
 ○ AnimaOS          not connected
 ○ ChatGPT          unconfigured
```

Choose an app with ↑↓ and press **Enter**. Press **f** to repair everything marked `!`.

## Claude Code, Codex, Hermes, Claude Desktop

Enter does the whole job:

1. Gives the app its own access to your memory ("Knows me": it reads your context and persona, saves checkpoints and tasks, and *proposes* memories for your review inbox instead of writing them directly). An app that already has working access keeps it.
2. Adds M.O.T.H.E.R to the app's settings using the app's own command (`claude mcp add`, `codex mcp add`, `hermes mcp add`). Claude Desktop has no such command, so its settings file is edited directly. A backup of the old settings goes to `setup-backups/` in your M.O.T.H.E.R folder first.
3. Reads the settings back and checks the access works.

Restart the app afterwards. The same steps from a script or another terminal:

```powershell
mother apps                    # what's connected
mother connect claude-code     # or codex, hermes, claude-desktop
mother fix                     # repair broken entries (old program path, revoked access)
```

`fix` only touches apps whose entry can't work. It never removes an app you connected yourself.

## AnimaOS

Start AnimaOS, choose **AnimaOS** and press Enter.

- Your AnimaOS owner key is read from this computer (`ANIMA_LOCAL_ADMIN_TOKEN_FILE`, or `~/.animaos-swarm/data/control-plane.json.admin-token`). It stays in memory and is never shown. If it isn't found, paste it into the masked field.
- Pick the agent, then choose:
  - **Knows me** (recommended): the agent uses your M.O.T.H.E.R persona and memory.
  - **Only what I share**: the agent reads only the records you pick and keeps its own personality.
- If older M.O.T.H.E.R connections are in the way, the screen lists them. Press **f** to turn them off and connect cleanly. They are disabled, not deleted, so they stay visible in AnimaOS's Connectors > MCP screen. An identity link pointing at one of them is cleared so the new one can be set. Esc changes nothing.

## ChatGPT

ChatGPT can't run programs on your computer, so it reaches M.O.T.H.E.R through OpenAI's private tunnel. Two things only you can create on OpenAI's website:

1. A tunnel: platform.openai.com → Settings → Tunnels.
2. A runtime key with Tunnels Read + Use: platform.openai.com → API keys.

Then install [tunnel-client](https://github.com/openai/tunnel-client/releases), choose **ChatGPT**, fill in the tunnel ID and key, save, and wait for **tunnel ready**. In ChatGPT, add a custom MCP server with Connection: Tunnel and Authentication: none. Details: [chatgpt.md](chatgpt.md).

"Tunnel ready" means your side works. Ask ChatGPT to read your context once to confirm the whole path.

## Undo

Each app has separate access. `mother revoke <name>` (for example `mother revoke codex`) cuts one off without touching the others; the names are the files in `clients/`. Settings backups are in `setup-backups/`.
