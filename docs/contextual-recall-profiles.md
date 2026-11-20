# Contextual recall profiles

A recall profile selects which of your pinned memories and decisions can appear when a harness starts a particular session or task. It narrows the caller’s current grant for that context response. Your canonical pins and their attribution remain in place.

## Set up profiles

Create a JSON file in an owner-private directory outside source control. Use the workspace ID and exact pin IDs from an authorized context read. Choose names of up to 40 lowercase ASCII letters, digits or hyphens, starting with a letter. You can define up to 32 names and include up to 12 pin IDs per profile.

```json
{
  "version": 1,
  "workspaceId": "6ebfa60e-1257-4c47-ad85-ce872425517b",
  "profiles": [
    {
      "name": "work",
      "pins": [
        "55fb2671-dc2d-4774-ab34-4b35a71bc57b",
        "3d84946b-ac56-46bf-bba5-ea103d92eee4"
      ]
    },
    {
      "name": "personal",
      "pins": [
        "55fb2671-dc2d-4774-ab34-4b35a71bc57b",
        "93f6dfe1-d9ef-4adc-84d1-2f2834ee98bf"
      ]
    }
  ]
}
```

The sample IDs are fictional. An empty pins list defines a valid profile that includes no pins. IDs that are no longer pinned have no effect on profile selection. Profiles select current pins, so an ordinary unpinned record may still appear under the caller’s grant.

Add the policy to the command that owns the context read:

- For a direct native connection, append **--recall-profiles /private/recall-profiles.json** to **mother mcp --config /private/direct.json** in the intended harness’s launcher. Restart that launcher after policy edits.
- For the local owner service, add **--recall-profiles /private/recall-profiles.json** to **mother garden service --db /private/garden.redb --endpoint /private/service.json**. Restart the owner after policy edits and reconnect the harness normally. The owner enforces policy before sending the reply to the bridge. The bridge does not need the file or the flag.

Use your actual absolute paths. The file is configuration for the process that loads it, and stays outside workspace exports. Reuse the file only with the intended workspace identity. A malformed or unsafe file fails startup.

## Start a profiled session

Before enabling any startup context call in your harness, put the selected profile into that call. Add this to the harness’s system or session instructions where it will be seen before the first tool call. Replace work with the intended profile for the task.

> For this session, start with mother_get_context using {"profile":"work"}. Use that same profile on later context reads. If it reports RECALL_PROFILE_UNAVAILABLE, tell me and wait for my choice before making another context call.

If the host already makes an automatic unprofiled context call, change or disable that call before using this workflow. The profile binds each request. An omitted profile keeps ordinary context behavior, including all pins visible under the current grant. An unknown profile, profiles disabled on the connection, or a policy for another workspace returns **RECALL_PROFILE_UNAVAILABLE** with no context fallback. On success, check **selection.profile** for the selected name.

The policy does not expand access. An included pin must still be visible under the current grant. A pin omitted by profile or by grant stays out of the pin list, navigation previews and context counts. The profile does not edit text authored separately in the persona or other visible records. Search and full-record reads continue to follow the underlying grant, so use a narrower grant when a host must be unable to read specific records at all.

Pin attribution and truncation still work as described in [bounded MCP reads](bounded-mcp-reads.md). Use **mother_get_record** with an included pin’s ID for its complete current record, subject to the current grant. The 12 KiB navigation and 48 KiB wire limits still apply. If the required response fields alone cannot fit, the read returns **READ_RESULT_TOO_LARGE**.
