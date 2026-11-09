# Connect AnimaOS from the terminal

Open M.O.T.H.E.R and press **n**, or run `mother connect animaos`.

1. Start AnimaOS on this computer. Keep the default `http://127.0.0.1:8080` address unless its local port differs.
2. Enter the **AnimaOS owner token**. This is AnimaOS's `ANIMA_LOCAL_ADMIN_TOKEN`, sent to its supported owner API. If AnimaOS also enables `ANIMAOS_RS_API_KEY`, enter that separate key in the optional field. These are AnimaOS credentials, not an OpenAI key. Both fields are masked and retained only for this setup session; paste them into the terminal, never into a chat.
3. Press Enter to load your agents. Choose one, press Enter to review, then Enter again to connect.

M.O.T.H.E.R creates a separate private grant with read and context:propose access. The selected AnimaOS agent can read shared context and submit Living Memory proposals. You approve those proposals in `mother inbox`. The setup uses the running M.O.T.H.E.R executable; use a stable installed build for ongoing use.

The adapter saves an exact local stdio connection through AnimaOS's owner API, enables it with its current version and checks tool discovery. If AnimaOS requires individual tool review or blocks tools under its agent policy, complete that review in AnimaOS's Connectors > MCP screen. M.O.T.H.E.R does not override those permissions.

When ready, ask the AnimaOS agent:

> Load my M.O.T.H.E.R context. When I ask you to keep something, submit a proposal for my review inbox.

Discovery verifies that AnimaOS reached the MCP server and listed the required tools. A real conversation read/proposal is a separate acceptance step. Setup never claims that it ran a model or saved a personal memory merely because discovery passed.

The address must be a numeric loopback HTTP address. Redirects and remote addresses are rejected, so owner credentials stay on this computer. No tunnel is needed for this local connection. AnimaOS remains an optional external harness; M.O.T.H.E.R's database, persona, inbox and MCP work independently.

If a connection request times out, the TUI reports uncertainty. Returning to Connect first retrieves existing server state and reconciles an exact matching entry. It does not blindly create another server or replace different launch settings. Credentials are not saved; reopening setup requires entering them again.

Use the AnimaOS MCP screen to stop or remove its connection. The ChatGPT disconnect command controls only its own tunnel runtime.
