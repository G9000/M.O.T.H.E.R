# Witness Journal / Letters from Mother

Ask a witnessing agent for a private, dated letter about an event or period you choose. The agent uses sources you authorize. You can read the letter, ask for changes, or send it to the Living Memory inbox for your approval.

## Install and invoke

Copy the entire [witness-journal skill folder](../skills/witness-journal) into the skills directory your harness uses. Keep its `SKILL.md` and `references` folder together.

| Harness | Personal skill directory | Project skill directory | Invoke |
| --- | --- | --- | --- |
| Codex CLI or IDE extension | ~/.agents/skills/ | .agents/skills/ | Choose witness-journal from `/skills` or type `$witness-journal`. |
| Claude Code | ~/.claude/skills/ | .claude/skills/ | Type `/witness-journal`. |

The resulting folder should be named witness-journal, with `SKILL.md` inside it. If the skill is missing from the selector, check the folder and open a new session. A host that supports the Agent Skills format and the same M.O.T.H.E.R v1 tools can also use the folder. Installation paths and invocation vary by host. [Codex skill discovery](https://learn.chatgpt.com/docs/build-skills) · [Claude Code skill discovery](https://code.claude.com/docs/en/skills#choose-where-skills-load)

Use a [connected M.O.T.H.E.R harness](garden-service.md) with authorized v1 read tools for record-backed drafts. Sending a draft for review additionally needs the `context:propose` scope or a compatible memory grant. A [review-only grant](living-memory.md#connect-and-upgrade) is sufficient.

## Draft a letter

Invoke the skill and name the period or event and the evidence you want it to use. For example:

> Draft a private Witness Journal letter about the fictional lantern workshop on August 14, 2026. Use the authorized workshop note I selected and identify any gaps. Keep the draft here.

The letter names its writer, dates the entry, states the period, and cites the sources for substantive observations. It attributes your words to you and keeps any interpretation in the agent's own voice. The [worked fictional example](../skills/witness-journal/references/fictional-example.md) shows a first draft and an approved correction.

For changes, describe what you want revised. To send a specific draft to your inbox, say “Submit this draft for my review.” The agent sends a proposal and returns its ID. Approve, edit or dismiss it through the [Living Memory owner inbox](living-memory.md#capture-review-continue). An approved correction uses the original record ID and retains the original writing in revision history. An authorized reader can retrieve that earlier revision.

The ordinary workflow stays text-only. Request an illustration separately for the specific entry and approve the exact material and destination before anything leaves the workspace.
