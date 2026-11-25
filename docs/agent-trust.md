# Agent trust for media attachments

A credential's **grant** decides what an agent may do. A **trust policy** decides which of the agent's authorized media attachments still wait in owner review. Trust never adds a scope, and agents can't read or change another agent's policy, or change their own.

## Modes

| Mode | Routine attachments | Self-started backfill |
| --- | --- | --- |
| `manual_review` (default) | Wait in **Attachment review** | Wait in review |
| `trusted_companion` | Commit automatically | Wait in review |
| `autonomous_gardening` | Commit automatically | Commit automatically |

Every existing and new credential starts in `manual_review`. Nothing is upgraded until you change it.

These cases always go to owner review, whatever the mode:

- The agent didn't declare `association: "confirmed"`. An omitted or `uncertain` match stays a proposal.
- The media has embedded metadata Mother can't strip, so you still decide how it may be displayed.
- The agent declared `initiation: "autonomous"` outside `autonomous_gardening`.
- The agent has reached its daily automatic limit, by commit count or by bytes, over a rolling 24 hours.
- The credential no longer holds `read` and `media:attach`.

The agent's `confirmed` declaration is necessary but never sufficient. Your mode, the limits and the media checks decide.

## Set trust

Open the live Garden and choose **Agent trust** (the shield icon, also linked from Attachment review). For each credential, pick a mode and daily limits, then select **Save trust**. **Return to manual review** takes effect for the agent's next submission.

Trust can only be raised for a credential that already holds `read` and `media:attach`. Use [the media bridge grant steps](media-bridge.md) for that. Policies live in the Garden database and are guarded by a revision number. They are never included in backups, and a restored Garden starts every credential in manual review, the same as grants.

## What agents see and do

`mother_get_capabilities` reports `mediaAttachmentBridge.trust`: the mode, policy revision, `routineAttachments` (`automatic` or `owner_review`), `autonomousBackfill`, the limits and the conditions that always need review.

`mother_media_attachment_prepare` and `mother_media_attachment_import` accept optional `association` (`confirmed` or `uncertain`) and `initiation` (`owner_request` or `autonomous`) fields. At submit time the service re-reads the grant and policy. It then either:

- commits through the same canonical writer as owner approval, and returns `approved` with a verified receipt; or
- returns `pending_owner_review` with a `reviewReason`: `trust_policy_manual_review`, `association_review_required`, `embedded_metadata_display`, `autonomous_work_not_permitted`, `daily_commit_limit`, `daily_byte_limit` or `credential_not_eligible`.

There's no MCP or IPC tool to approve, retract, or change trust.

## Audit and retraction

Each committed link records how it was authorized. Owner approvals show `owner_manual_approval`. Automatic commits carry an `owner_granted_trust_policy` record with the client, mode, policy revision and decision time. That record is bound into the plan's content hash and the stored evidence binding, and applies only to plans committed by a policy. Earlier plans and receipts are unchanged.

**Agent trust** lists recent operations: agent, memory, file, status, authorization and digest. **Retract link** removes an active link. The original file, the evidence and the receipt stay in place, and any other memory that uses the same original keeps it.
