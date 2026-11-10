# Connect AnimaOS from the terminal

Open M.O.T.H.E.R and press **n**, or run `mother connect animaos`.

1. Start AnimaOS on this computer. Keep the default `http://127.0.0.1:8080` address unless its local port differs.
2. Enter the **AnimaOS owner token**. This is AnimaOS's `ANIMA_LOCAL_ADMIN_TOKEN`, sent to its supported owner API. If AnimaOS also enables `ANIMAOS_RS_API_KEY`, enter that separate key in the optional field. These are AnimaOS credentials, not an OpenAI key. Both fields are masked and retained only for this setup session; paste them into the terminal, never into a chat.
3. Press Enter to load your agents. Choose one, then choose its access role. Shared context is the default and its empty record allowlist shares no records. Setup shows the latest 200 current local records with titles, numbers and UUIDs. Enter comma-separated numbers from that list or record UUIDs to share specific records; PgDn/PgUp scroll the list. Companion uses M.O.T.H.E.R's persona and primary durable memory for this selected agent. Legacy context tools preserve the previous full-context/persona-readable connection without activating companion identity or primary-memory isolation.
4. Press Enter to review the exact agent, workspace, launch path, role and allowlist, then Enter again to connect.

M.O.T.H.E.R creates a separate private grant for each agent and role with exactly read and context:propose access. Companion and shared grants pin the selected agent as their subject. Shared grants expose only the explicit record-ID allowlist and never expose the persona; an empty list exposes no records. Companion can read canonical context and the owner persona. Both can submit Living Memory proposals for approval in `mother inbox`, rather than write canonical memory directly. The setup uses the running M.O.T.H.E.R executable; use a stable installed build for ongoing use.

The adapter saves an exact local stdio connection through AnimaOS's owner API, enables it with its current version and checks tool discovery. If AnimaOS requires individual tool review or blocks tools under its agent policy, complete that review in AnimaOS's Connectors > MCP screen. M.O.T.H.E.R does not override those permissions.

For companion/shared setup under Automatic tool policy, confirming setup authorizes a snapshot review of only the exact discovered `mother_get_context` reader. The adapter submits its current connection version, discovery ID and descriptor digest to the owner tool-review API, without creating extra execution grants. It reuses an already matching reviewed reader. Under Review policy, complete manual tool review and grants in AnimaOS, then return to Connect. Only after readiness and reader review does setup save the optional per-agent workspace-context binding, pinned to the local workspace ID, restore generation, subject, connection and `mother_get_context` tool. Discovery alone does not install companion identity. Companion applies the current persona to each turn and uses M.O.T.H.E.R as primary durable memory. If it is unavailable or validation fails, the same chat keeps its history and uses the agent's complete configured local personality, with durable memory explicitly unavailable. No offline memory mirroring or automatic replay occurs. Shared mode keeps the agent's local personality.

Existing legacy grants and connections are preserved. New roles have separate client identities; setup never widens or rotates a grant. If an older M.O.T.H.E.R connection is enabled, explicitly disable it in AnimaOS before setting up the new role. If the agent already has another workspace-context binding, explicitly disable or replace that binding in AnimaOS settings. A changed allowlist on an existing role is rejected; review the existing grant and revoke/replace it explicitly through owner setup rather than expecting Connect to change its authority silently.

When ready, ask the AnimaOS agent:

> Load my M.O.T.H.E.R context. When I ask you to keep something, submit a proposal for my review inbox.

Discovery verifies that AnimaOS reached the MCP server and listed the required tools. A real conversation read/proposal is a separate acceptance step. Setup never claims that it ran a model or saved a personal memory merely because discovery passed.

The address must be a numeric loopback HTTP address. Redirects and remote addresses are rejected, so owner credentials stay on this computer. No tunnel is needed for this local connection. AnimaOS remains an optional external harness; M.O.T.H.E.R's database, persona, inbox and MCP work independently.

If a connection request times out, the TUI reports uncertainty. Returning to Connect first retrieves existing server state and reconciles an exact matching entry. It does not blindly create another server or replace different launch settings. Credentials are not saved; reopening setup requires entering them again.

Before a binding save, setup records the exact intended binding locally. If its response is lost, setup reads the owner API and compares the exact binding; it never blindly repeats the PUT. An unresolved outcome stays blocked for owner inspection. A successful setup is protocol evidence only; a real online/offline/recovery conversation remains a separate host acceptance step.

Reader reviews also save their exact intent before sending. After a lost response, returning to Connect reads current connection metadata and requires the same connection, expected next version, discovery ID, current descriptor and reviewed descriptor. A changed or missing result stays uncertain; setup never replays the review automatically.

Use the AnimaOS MCP screen to stop or remove its connection. The ChatGPT disconnect command controls only its own tunnel runtime.
