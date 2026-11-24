# Attach original media from an MCP host

The bridge attaches a selected image, audio recording or video to an existing narrative memory. The local owner approves the file and memory in the live Garden. M.O.T.H.E.R. stores the original bytes and creates a graph evidence binding without revising the narrative memory.

## Provision an attachment credential

Follow [local service setup](garden-service.md) to give each harness a separate native credential. For a harness that can read and propose attachments, use an explicit grant such as:

```bash
mother garden grant-v1 media-companion --db "$db" --out "$native_config" \
  --access companion --subject media-companion --scopes read,media:attach
```

Run owner-only grant commands while the service is stopped. Reissue the selected harness's credential with its existing intended scopes and visibility plus `media:attach` if appropriate. Start the service with `--explore`, provision its service config with `garden service-config`, and reconnect the harness to the updated `mother mcp --config "$service_config"`. Existing grants receive no new scope. Shared grants are supported with record-level visibility. This scope allows selected proposals and their own receipts. Separate graph media scopes control disclosure of existing media.

Check `mother_get_capabilities.mediaAttachmentBridge` and the host's actual tool catalog. All five attachment tools should be present for the new credential. Follow every `tools/list` cursor when checking the protocol directly. The owner-only browser review is absent from both MCP and local IPC.

## Transfer and review

1. Read the target memory with `mother_get_record`. Keep its exact UUID and current revision.
2. Use a fresh operation UUID with `mother_media_attachment_prepare`. When the host supplies the original bytes, declare its basename, length and lowercase SHA-256. The optional source locator is an opaque label such as a file ID. Keep download URLs and tokens out of the locator.
3. Stream the selected original with `mother_media_attachment_upload`. Each call carries a base64 chunk, byte offset and the temporary transfer token from prepare. Read `receivedBytes` after disconnection. An exact prepare retry with the same UUID and declaration recovers the token while uploading. Exact chunk retries compare the overlapping bytes before continuing.
4. Call `mother_media_attachment_submit` to inspect the original and bind the inspected type, digest, filename, provenance and target revision. The resulting `pending_owner_review` means the bytes are staged locally.
5. Open the live Garden and choose **Attachment review**. Verify the target, revision, source, filename, bytes, digest and display policy. Click **Approve attachment** to publish and verify. The reader shows the active link under **Moments captured**. The Archive shows its provenance and digest.
6. Call `mother_media_attachment_status`. Report success only if the response is `approved`, `verification.verified` is true, and the canonical receipt, digest and active link match the operation's intended target. A retracted link returns `retracted` with its historical receipt.

When the source provides only a locator or preview, omit `file` at prepare. `SOURCE_BYTES_UNAVAILABLE` means the owner can select the original on **Attachment review**. The browser hashes and streams the selected file into the same private staging area. The owner then reviews the file details and explicitly approves. The source locator is retained as claimed provenance. It does not certify that an owner-selected file came from that locator.

Local hosts that can run Mother and access an explicitly selected file can stream it without putting base64 in model context:

```bash
mother garden media bridge --config "$service_config" propose \
  --file "$selected_file" --memory "$memory_id" --revision "$revision" --operation "$operation_id"
```

## Host capabilities

| Host | Connection | Source bytes | Qualification |
| --- | --- | --- | --- |
| ChatGPT connected app | Existing supported remote transport to native stdio | `mother_media_attachment_import` declares `openai/fileParams`, so ChatGPT can pass a file the owner attached or picked from the Library as `{download_url, file_id}`. The bridge downloads it from OpenAI file hosts and streams it. Without a passed file, an opaque Library ID yields the local owner handoff. | Native catalog and the synthetic import path are tested. Whether ChatGPT fills the file parameter for Library files through this tunnel must be confirmed with one real call; see below. |
| Codex | [Local MCP stdio](https://developers.openai.com/codex/mcp) with a dedicated config | A host-side reader can supply chunks or invoke the selected-file helper. Owner file handoff also works. | The native executable's stdio contract is covered by the synthetic protocol test. Verify the actual Codex catalog after reconnecting. |
| Claude Code | [Local MCP stdio](https://code.claude.com/docs/en/mcp#option-3-add-a-local-stdio-server) with a dedicated config | Same local selection and transfer options as Codex. | Separate credentials are covered by protocol tests. Verify the actual Claude Code catalog after reconnecting. |
| Generic MCP clients | Native MCP stdio or an authenticated adapter that preserves tool schemas and identity | The client reads authorized originals and submits bounded chunks. Clients without that access use the owner file handoff. | Standard MCP tool listing and calls are exercised against the executable. |

[OpenAI's plugin reference](https://developers.openai.com/plugins/reference#define-file-inputs) documents `openai/fileParams`: each declared field receives `{ download_url, file_id, mime_type?, file_name? }`. The owner service still accepts neither paths nor URLs and makes no outbound requests. Only the harness's own `mother mcp` bridge fetches, and only for `mother_media_attachment_import`.

### Host file import

1. Attach the original to the ChatGPT conversation, from the device or from the ChatGPT Library. Several files can be attached at once for a batch.
2. Ask ChatGPT to read the target memory and call `mother_media_attachment_import` with a fresh operation UUID, the memory ID and revision, and the attached file.
3. The bridge downloads once, computes size and SHA-256 from the actual bytes, prepares with the Library `file_id` as the source locator, streams 64 KiB chunks and submits. The result is `pending_owner_review`; nothing is attached yet.
4. Approve in the live Garden under **Attachment review**, then confirm with `mother_media_attachment_status`.

Download rules: HTTPS only; hosts under `oaiusercontent.com`, `chatgpt.com` or `openai.com`, matched by whole labels, plus exact storage hosts observed in live ChatGPT links (currently `oaisdmntprseasia.blob.core.windows.net`). Azure storage account names are first-come, so no `blob.core.windows.net` prefix or suffix is trusted; a new region host is reported by name and must be added explicitly; no credentials, ports or IP literals. Redirects are re-checked against the same rules, with at most three. The download is capped at 32 MiB and two minutes. Errors name the host but never echo the signed link. An exact retry of an operation that has already left `uploading` returns its state without downloading again; a retry while uploading resumes at `receivedBytes`. A bare string such as `chat_upload` or a missing file object returns `SOURCE_BYTES_UNAVAILABLE`, and the owner handoff still applies.

| Path | Status |
| --- | --- |
| ChatGPT file parameter, file attached in the conversation | Documented by OpenAI; verify with one live call after installing this build. |
| ChatGPT Library file attached to the conversation | Same mechanism; host behaviour for Library files is not documented and must be verified live. |
| ChatGPT Library ID only, no attached file | Unsupported for automatic transfer: owner handoff in Attachment review. |
| Codex, Claude Code, other local hosts | Selected-file helper or chunked upload; unchanged. |

## Limits and recovery

- Each operation expires one hour after preparation. Each file can reserve at most 32 MiB. Chunks are limited to 64 KiB. The Garden accepts up to 24 pending operations, four per client, with 256 MiB of declared staging capacity. Preparation is capped at eight operations per minute per client and chunk ingestion at 16 MiB per second Garden-wide.
- Staging lives under an owner-private, generation-specific `attachment-sessions-*` directory beside the database. The service reconciles expired sessions every minute. Direct MCP reconciles expired sessions during access or admission. These files stay outside canonical recall and backups. The directory holds up to 4,096 operation manifests, including terminal operations. With the owner stopped, archive individual completed session directories to a private location when needed. Archiving a session removes the agent's lookup by that transfer UUID. The canonical owner receipt stays in the Garden.
- Cancel through `mother_media_attachment_cancel` or reject from the owner review. An expired operation, a retracted or revised target, or a lost grant cannot publish a new link. An owner decision stays bound to the reviewed digest and revision and can be reconciled after a lost reply. If the browser still shows a recorded owner decision, use **Reconcile approval**. Check status by the original UUID before retrying a failed call. Prepare a new operation for a changed target or replacement file.
- Existing native backups include the canonical media and association. A restored generation uses new grants and sessions. To revoke, use the existing [owner revoke workflow](memory-attachments.md) with the `plan` object from the session's private `session.json` after stopping the owner. Preserve the original manifest in a private location and pass its plan and approval ID to the CLI. Revocation removes the active association and retains the original evidence and receipt. Reverting the software preserves these canonical records and leaves the older binary without this transfer session protocol.
- The Garden displays a sanitized derivative when supported. When embedded metadata cannot be stripped, the owner can withhold playback or explicitly approve display of the original in the private Garden. Consult the owner Archive or the existing media verifier if the original cannot be displayed.
