# Bring M.O.T.H.E.R to ChatGPT

Connect the same local workspace you use in Codex to ChatGPT through OpenAI's Secure MCP Tunnel. M.O.T.H.E.R remains an independent Rust context layer: its SQLite store and persona stay on your machine, while context returned by MCP tools is shared with ChatGPT. This does not import your chat history or make ChatGPT save every conversation automatically.

The tunnel client makes an outbound HTTPS connection. M.O.T.H.E.R continues to serve stdio; no public port, AnimaOS service or new HTTP server is needed. Your computer and the tunnel client must stay running for discovery and tool calls.

## Prepare the workspace and tunnel

1. Follow [workspace setup](setup.md) to install `mother` and initialize your own persona. For an existing workspace, reuse its database and back it up; do not run initialization again.
2. Create a separate ChatGPT grant. This example allows reads, memories and checkpoints; it does not allow writes to tasks, decisions, artifacts or action records:

   ```powershell
   mother grant chatgpt --scopes read,memory:write,checkpoint:write
   ```

   Reuse an existing suitable ChatGPT config instead of granting the same name twice. If using another database, pass the same `--db` path to owner commands. Keep the printed config path private; it contains the M.O.T.H.E.R credential.
3. Run `mother context --config "C:/private/chatgpt.private.json"` with that actual config path to verify local access. The result can contain private context; inspect it in your own terminal.
4. Download the supported client from [OpenAI's tunnel-client releases](https://github.com/openai/tunnel-client/releases), verify its published checksum, and make `tunnel-client` available in your terminal. The commands below were checked against v0.0.20; consult `tunnel-client help quickstart` for your installed version.
5. In [Platform tunnels](https://platform.openai.com/settings/organization/tunnels), create or reuse a tunnel associated with both your Platform organization and the ChatGPT workspace where you will use it. Keep its tunnel ID for the next step. Creation requires **Tunnels Read + Manage**; running the client and selecting the tunnel require **Tunnels Read + Use**. ChatGPT workspace policy must also allow custom MCP servers.
6. Create a dedicated runtime key in [Platform API keys](https://platform.openai.com/settings/organization/api-keys) with the required tunnel access. Use a runtime key, not an admin key. The OpenAI key authenticates the tunnel; the separate local M.O.T.H.E.R config controls access to records. Never paste either credential into a chat or commit it.

## Start the private connection

This PowerShell example prompts for the runtime key without echoing it, passes it through the process environment, and clears that environment value afterward. It does not save the key to disk. Replace the tunnel ID and both example paths first; use the executable and config paths from your installation, not a repository-relative path.

Use forward slashes inside the MCP command on Windows: the tunnel command parser treats backslashes as escapes. Quote each path inside that command if it contains spaces.

```powershell
$tunnelId = 'tunnel_REPLACE_WITH_YOUR_ID'
$mcpCommand = '"C:/path/to/mother.exe" mcp --config "C:/private/chatgpt.private.json"'
$profileDir = Join-Path $env:LOCALAPPDATA 'MOTHER/chatgpt-tunnel/profiles'
$secureKey = Read-Host 'OpenAI runtime API key' -AsSecureString
$keyPointer = [IntPtr]::Zero
$previousKey = $env:CONTROL_PLANE_API_KEY
try {
    $keyPointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secureKey)
    $env:CONTROL_PLANE_API_KEY = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($keyPointer)
    $env:HEALTH_LISTEN_ADDR = '127.0.0.1:0'
    $env:MCP_STDIO_SEND_INITIALIZED_NOTIFICATION = 'true'

    # First setup only. Reuse an existing matching profile; do not overwrite it.
    tunnel-client init --sample sample_mcp_stdio_local --profile mother-chatgpt --profile-dir $profileDir --tunnel-id $tunnelId --mcp-command $mcpCommand --control-plane-api-key-ref env:CONTROL_PLANE_API_KEY --health-listen-addr 127.0.0.1:0
    if ($LASTEXITCODE -ne 0) { throw 'Profile setup failed; inspect the existing configuration.' }

    tunnel-client doctor --profile mother-chatgpt --profile-dir $profileDir --explain
    if ($LASTEXITCODE -ne 0) { throw 'Resolve the doctor findings before connecting.' }

    tunnel-client runtimes connect --alias mother-chatgpt --tunnel-id $tunnelId --profile mother-chatgpt --profile-dir $profileDir --mcp-command $mcpCommand --runtime-api-key env:CONTROL_PLANE_API_KEY --json
    if ($LASTEXITCODE -ne 0) { throw 'Tunnel startup failed.' }
    tunnel-client runtimes status mother-chatgpt --json
} finally {
    $env:CONTROL_PLANE_API_KEY = $previousKey
    if ($keyPointer -ne [IntPtr]::Zero) {
        [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($keyPointer)
    }
    $secureKey.Dispose()
}
```

The health/admin listener uses a local ephemeral port, avoiding conflicts with other apps. The initialized notification supports M.O.T.H.E.R's MCP handshake. The managed runtime inherits the key in memory and continues after this setup terminal closes; clearing the parent variable does not stop it. Do not enable raw HTTP or payload logging for a personal workspace.

Check that status reports `process_running`, `healthy` and `ready`, and inspect the local client status for active polling. Starting a process alone is not proof that ChatGPT can use it. On a later restart, reuse the same profile, alias, tunnel and M.O.T.H.E.R config; supply the runtime key again and skip `init`. Inspect `runtimes status` before reconnecting an already-running alias. This recipe does not install automatic startup after reboot.

## Add it in ChatGPT

While the runtime is healthy:

1. Open [ChatGPT Plugins](https://chatgpt.com/plugins), select the plus button, then **Add custom MCP server**.
2. Name it **M.O.T.H.E.R**, choose **Tunnel** under **Connection**, and select your tunnel or paste its ID.
3. Configure the authentication choice appropriate to this stdio connection. M.O.T.H.E.R uses its local config credential; it does not implement a separate OAuth service. Do not paste the local credential into a URL or description.
4. Review the access warning, then create the private plugin. Review the six discovered tools against the [capability table](setup.md#tools-and-capabilities); tool visibility does not grant extra write scopes.
5. Install it for your use. Start a chat, type `@`, and select M.O.T.H.E.R.

Use the [session guidance](../adapters/session.md) for intentional retrieval and saving. Each harness retains its own permissions and instruction hierarchy. A connected plugin does not replace ChatGPT's built-in memory or guarantee automatic use in every chat.

## Verify reads and saves across harnesses

Use clearly synthetic content for the first test, and authorize any requested write confirmation:

> Use M.O.T.H.E.R to read my workspace identity and stored persona. Search for a synthetic memory titled "ChatGPT tunnel acceptance". If absent, save a memory saying "Synthetic connection test: the marker is lavender-731", clearly attributed to this test request. Return the record ID, revision and operation receipt. Read the full record back by ID. Do not store this as a real preference or repeat a write whose outcome is uncertain.

Then ask Codex, using the same workspace, to retrieve that exact record ID. Confirm the workspace ID/generation, record content and revision agree. Ask ChatGPT to read the current record, correct the marker to `lavender-732`, and retrieve it from Codex again. Each new mutation needs a new operation UUID and the current expected revision. For an uncertain response, inspect the receipt or retry the identical request with the same UUID; receipts belong to the client that performed the write.

Finally, have ChatGPT save a checkpoint describing only the observed result, and retract the synthetic memory after reading its current revision. Retraction removes it from normal recall but retains its history. Check that the record and checkpoint can still be retrieved after a tunnel restart. This is acceptance of actual ChatGPT read/save behavior; a local CLI read or tool discovery alone does not establish it. See [record and retry rules](setup.md#tools-and-capabilities).

## Disconnect or troubleshoot

- Stop the local connection with `tunnel-client runtimes stop mother-chatgpt`. Inspect status afterward. Stop this runtime only; Codex's separate local MCP connection can continue.
- Revoke record access with `mother revoke chatgpt` (add `--db` for a non-default workspace). Existing MCP processes check revocation on subsequent calls. Reconnecting a revoked client requires a new name and config.
- A missing tunnel in ChatGPT usually needs its ChatGPT workspace association and the operator's **Read + Use** access checked. Platform organization association alone is insufficient.
- For authentication or readiness errors, inspect `doctor` and runtime status, the runtime-key permissions, absolute MCP paths and initialized-notification setting. Do not recreate the database or grants to resolve a tunnel error.
- If a write is denied, compare the requested record kind with the ChatGPT grant. The example grant intentionally supports only memory and checkpoint writes.

Disconnecting or revoking access cannot remove context already returned to ChatGPT. Keep the database, local client configs, runtime keys, logs and private backups outside Git.

## Verification boundary

On 9 November 2026, the maintainer's private setup reached tunnel creation, a separate scoped ChatGPT grant, verified local CLI reads, checksum-verified tunnel-client v0.0.20 installation, and profile/launcher preparation. Runtime-key entry, a healthy tunnel, ChatGPT plugin discovery and live ChatGPT read/save acceptance were still pending. No end-to-end ChatGPT success is claimed by these docs.

References: [Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels), [official onboarding](https://github.com/openai/tunnel-client/blob/master/docs/onboarding.md), and [connect and test a ChatGPT plugin](https://developers.openai.com/plugins/deploy/connect-chatgpt).
