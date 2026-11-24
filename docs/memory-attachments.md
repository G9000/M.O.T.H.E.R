# Attach an exported file to a memory

Use the owner CLI with the same native database as the Garden. Export the original file to a private local folder first. Stop the owner service before opening its database directly, then restart it with Explorer enabled after importing. Existing clients reconnect to the same workspace and generation.

For one attachment, run this in Git Bash. Set the paths and memory UUID to your selection. The file argument uses a relative path with forward slashes.

```bash
mother garden media backfill plan --db "$db" --root "$export_dir" \
  --file clip.mp4 --memory "$memory_id" --interactive
```

Read the memory title, ID and revision alongside the filename, SHA-256 and metadata flags. Enter `y` to attach. The receipt identifies the graph evidence and binding assertion. An optional `--source-file-id` records the exported file's opaque source ID. A Library ID alone cannot supply bytes.

The owner Garden prefers a stripped derivative. Some formats, including MP4, retain their original embedded tags. If you want a particular original playable in your private Garden, include `--allow-original-in-owner-garden` when planning that attachment and review the resulting manifest. This approves display of that asset, including any embedded location or device metadata, within the owner session. The MCP original-bytes policy remains independently enforced.

## Save a review and verify it

Keep each manifest outside the repository. Run the plan command once per file and candidate memory. Saving a plan is a dry run. Leave a manifest unapplied to defer or skip it.

```bash
mother garden media backfill plan --db "$db" --root "$export_dir" \
  --file clip.mp4 --memory "$memory_id" > "$export_dir/review.json"
# After reading the manifest, supply its operationId as the approval.
mother garden media backfill apply --db "$db" --root "$export_dir" \
  --manifest "$export_dir/review.json" --approve "$operation_id"
mother garden media backfill verify --db "$db" --root "$export_dir" \
  --manifest "$export_dir/review.json"
```

Each file may be up to 256 MiB. Use an owner-controlled local import root. Planning reads the selected bytes and detects the native media format. Applying checks the byte hash, workspace, restore generation and expected memory revision. Repeat the exact manifest after an uncertain result to recover its immutable receipt. Verification also checks the current source bytes and resolves the active binding. A changed memory revision requires a fresh review for a new attachment.

To withdraw the association, run `mother garden media backfill revoke --db "$db" --manifest "$export_dir/review.json" --approve "$operation_id"`. Retraction keeps the bytes and both receipts. After a full restore, existing links can be browsed. New mutations require current-generation approval. Native media-only exports carry assets and acquisitions. Use a full Garden backup to preserve the narratives, bindings and receipts together.

## Read it

Open the memory in any of the three Garden Reader lenses and choose the file under **Moments captured**. The Archive includes the original digest, approval time, source and evidence ID. The Reading Path's **Has media** filter includes records with displayable approved assets. The owner gallery's total byte and listing limits also apply to the reader.

## Storage and authority

Each approval is one redb transaction. It stores the original bytes and acquisition, creates artifact evidence, creates a graph entity representing this association, and records the target memory UUID in a graph assertion. The receipt binds these three graph IDs. Linking another memory can reuse the same content-addressed bytes. The write leaves the v1 narrative and revision history intact.

The private manifest and its hash identify exactly what the owner reviewed. Only the owner CLI publishes or retracts these links. The reader resolves the evidence against the immutable direct-owner receipt, the current binding revision and the existing active memory. Retracted bindings and client-created lookalikes cannot authorize inline attachments. An authenticated local Garden session serves the selected bytes. No new MCP write authority is required for import.
