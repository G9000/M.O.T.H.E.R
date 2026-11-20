# Opt-in Claude Code capture

This adapter uses Claude Code’s local JSONL transcripts. It reads one owner-selected project directory and sends selected candidates to the same [Living Memory inbox](living-memory.md) used by manual capture. Claude Code can invoke it at SessionEnd. You can use the same source for a date-window backfill.

## Setup and pause

Build M.O.T.H.E.R and configure a [local owner service](garden-service.md) with the v1 compatibility view. Give the adapter a dedicated v1 client identity with only the read and context:propose scopes. Use its private service client configuration for both capture and backfill. The private state directory must already exist. Keep it, the capture policy and the service configuration outside source control in owner-private directories. On Unix these directories normally have mode 0700. On Windows they need the equivalent ACL.

Write a JSON policy with absolute paths. Missing or false enabled keeps capture off. Set it to true only for the source you intend to authorize.

```json
{
  "enabled": false,
  "transcript_dir": "/absolute/private/claude-project",
  "state_dir": "/absolute/private/capture-state",
  "service_config": "/absolute/private/review-client.service.json"
}
```

The transcript directory is one Claude Code project directory containing UUID-named session JSONL files. A hook that points elsewhere is rejected. The adapter never discovers other projects, reads nested agent logs, or reaches out to Anthropic. Use a different policy for each deliberately selected project and use the same service client and state directory whenever reprocessing its source items.

For an explicit end-of-session invocation:

```sh
mother capture --policy /absolute/private/capture.json session-end --session SESSION_UUID
```

To run automatically, add a SessionEnd command to your Claude Code settings. A project-specific, untracked local settings file is suitable when the project and transcript directory have the same scope. The command reads the documented SessionEnd JSON on standard input. Adjust the executable and policy paths, using your shell’s path quoting as needed.

```json
{
  "hooks": {
    "SessionEnd": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "/absolute/bin/mother capture --policy /absolute/private/capture.json session-end",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

To pause, set enabled to false. Capture checks policy at startup and immediately before each send. A service request already sent can still complete. Claude Code writes its transcript asynchronously, so the hook can see an incomplete or lagging file. Backfill can pick up missed turns after they are written. [Claude Code hook reference](https://code.claude.com/docs/en/hooks#sessionend)

## What can be selected

Capture uses a deliberately narrow local selector. Short, standalone first-person statements such as “I’ve decided to use the blue cover.” or “We finished the pilot today.” can be captured without a separate request. The selector derives a short title and attributed summary. Quoted or conditional proposals, tool results, generated metadata, sidechain turns, and unrecognized phrasing are skipped. It reads each eligible statement on its own, so an implicit reversal in a later turn may require a correction at review. Milestones reflect the owner’s report. An explicit assistant line in the form “Observation [Pilot pace]: The pilot seemed paced for beginners.” is labeled as an unverified agent observation.

Use “Decision correction [Exact current title]: Use the yellow cover instead.” for a new revision proposal. Capture searches first and reads a unique current confirmed match in full. It carries its record ID and expected revision forward with its existing source references. If the title is ambiguous or the revision changes, capture skips the correction. A fresh correction turn can be considered against the current record. The owner reviews both new items and corrections through the existing inbox.

A standalone turn beginning “Do not remember:” excludes that passage while allowing unrelated owner candidates from the session. Capture suppresses assistant observations in those sessions and checks for repeated wording in other candidates. “Do not remember this session.” or an ambiguous exclusion skips the whole session. These controls are applied to available source text before submission. Use the session-wide exclusion or keep sensitive sessions outside the opted-in source. The local filter rejects common credential markers and token shapes, but cannot identify an arbitrary secret embedded in ordinary prose or undo a proposal already sent before a later exclusion. Logs and coverage show counts and dates. They contain no turn text.

## Preview and resume a backfill

Choose an inclusive start and exclusive end date in UTC. Preview works with the policy enabled and uses no service credentials or inbox writes.

```sh
mother capture --policy /absolute/private/capture.json backfill --from 2026-09-01 --before 2026-09-07
```

The report counts recognizable session files, unavailable or incomplete files, session-wide exclusions, skipped turns and selected items. It reports which requested UTC dates had readable, in-scope turns. Dates without readable turns can indicate no activity, expired or unavailable data, or a gap in the selected source. The adapter cannot inventory files that Claude Code did not retain. Preview reruns against current files and does not reserve a snapshot.

Add **--submit** to run. Resubmit the same window with the same policy, stable client identity and state directory after interruption. Each session/turn has a stable operation UUID shared by both capture paths. The adapter checks the original receipt before submitting. If a response is lost and the receipt remains unconfirmed, the run stops until it can reconcile the original operation. The private journal preserves the exact selected draft and expected revision for unchanged retry. Do not discard it during a run or use it with a different service identity or restore generation. The journal contains selected drafts and provenance, so protect it like the inbox.

A received proposal is counted as previously submitted on rerun, including when the owner has dismissed it. Only the owner can approve; proposals remain outside canonical search until approval. See [inbox review commands](living-memory.md).

The repository tests exercise the CLI, the SessionEnd input, and the authenticated local service with synthetic Claude Code JSONL files. They do not load the hook into an installed Claude Code client or read any real conversation.
