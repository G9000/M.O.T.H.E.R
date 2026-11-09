# Test a local connection

Press **t** in the terminal home. Select an existing private client config with the arrows and press **Enter**. M.O.T.H.E.R launches the **currently running executable** with `mcp --config <selected file>`, performs MCP initialization, lists its tools and calls `mother_get_context` with that grant. No executable command is taken from the config. The background check has a ten-second deadline; Escape cancels, stops and reaps its owned child, then returns home. Page Up and Page Down scroll the report.

The list includes up to 128 private configs from the private data directory's `clients` folder, plus the saved ChatGPT config when present. It labels malformed configs without exposing their contents. Configs targeting another workspace are marked and cannot be tested from this workspace's dashboard; use the explicit command below if that workspace is intended.

For scripts:

```powershell
mother doctor --config "C:\private\clients\codex.private.json" --json
```

The report contains the executable path and SHA-256, reported server version, client identity, workspace ID, restore generation, and canonical/recallable counts for each category. Workspace and generation must match a read-only identity lookup in the selected config's database. Version alone does not distinguish builds that share a package version; use the executable hash and inventory support together. Context persona, record content, tokens, raw subprocess errors and stderr are never displayed. Context text is received only by the local verifier and discarded after projection to the allowed report fields.

`LOCAL MCP read verified` proves that this executable completed a local protocol read with the selected grant. It does **not** prove a running ChatGPT, Hermes, Codex or AnimaOS conversation has loaded this executable or exercised the connection. Test each actual host separately. The check does not change harness settings, grants, records or model files and does not download models.

Older servers with no inventory report `inventory_unsupported`, with an explicit verified-read flag when the context read succeeded. Missing or malformed counts are never shown as zero. Malformed messages, denied reads, mismatched workspace identity, output/message limits, cancellation and deadlines produce fixed redacted status codes. A fully verified report exits successfully; incomplete checks exit with a failure status while preserving the JSON report on stdout. The verifier limits the complete session to 2 MiB of stdout and 32 messages and joins its pipe reader after stopping its child.
