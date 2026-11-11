# Bring her to every harness

M.O.T.H.E.R is one workspace that many harnesses share. Each harness needs two things: a connection to her MCP server, so it can read and save context, and a reason to read her at the start of a session, so she is present before you say anything. This guide covers both for each harness.

Give every harness its own grant, so revoking one never affects the others:

```powershell
mother grant claude-code --scopes read,memory:write,decision:write,task:write,checkpoint:write
```

Each command below takes the private config that `mother grant` printed. All of them print setup text for you to paste; none of them edits a harness's own settings. Paths in the output are resolved for this computer, and paths containing quotes, `$`, backticks or `%` are refused for the shell-command formats.

## Handing work between harnesses

Each grant name is also the harness's name for handoffs. A task handed to `codex` with `mother_assign_task` appears at the top of Codex's briefing and under `assignedToYou` in its context the next time it starts. See [handing work to another harness](context.md#handing-work-to-another-harness).

## Her session briefing

```powershell
mother brief --config "C:\path\to\claude-code.private.json"
```

prints a short plain-text briefing: an instruction to read `mother_get_context`, her persona, your pinned core, tasks other harnesses handed to this one, where the last session left off, any unsettled tasks or actions, and the harness names a task can be handed to. It stays under 9,000 characters, trimming the persona and long pins first and saying when it did. Saved records are labelled as data, not instructions. If the workspace cannot be read (for example after `mother revoke`), it prints a one-paragraph note saying so, with no private content, and still exits successfully so a harness session is never blocked by it.

Any harness that can run a command at session start, or that you build yourself, can put this text into its system prompt.

## Claude Code

```powershell
mother connection --config "<claude-code config>" --format claude-code
```

prints two steps:

1. A `claude mcp add --transport stdio --scope user mother -- …` command that registers her server for every project.
2. A `SessionStart` hook that runs `mother brief`. Merge it into the `hooks` section of `~/.claude/settings.json`. Claude Code adds a SessionStart hook's plain output to the model's context before the first prompt, so she is present even if the model never decides to call a tool.

Start a new session and check `/mcp`. The hook's output is capped by Claude Code at 10,000 characters, which the briefing stays under.

## Codex

```powershell
mother connection --config "<codex config>" --format codex
```

prints a `[mcp_servers.mother]` entry for `config.toml`. Codex also reads standing instructions from AGENTS.md files; add this line to your global one (usually `~/.codex/AGENTS.md`):

> At the start of each session, call mother_get_context and treat its pinned records as the owner's core context.

## Hermes Agent

```powershell
mother connection --config "<hermes config>" --format hermes
```

prints a `mother` entry for `mcp_servers` in `~/.hermes/config.yaml`. Run `/reload-mcp` in a session afterwards. Add the standing instruction above to Hermes's own persona or instructions so it reads her first.

## OpenClaw

```powershell
mother connection --config "<openclaw config>" --format openclaw
```

prints an `openclaw mcp set mother '{…}'` command for a POSIX shell. OpenClaw only writes MCP settings while its Gateway is stopped; start it again and check with `openclaw mcp doctor --probe`. On Windows, enter the printed JSON object through OpenClaw's own configuration screen instead.

## ChatGPT

ChatGPT reaches her through a private tunnel rather than a local process. Follow [Bring her to ChatGPT](chatgpt.md), and put the standing instruction above in the project or custom instructions.

## AnimaOS

`mother connect animaos` sets up the connection from the terminal. See [Connect AnimaOS](animaos.md).

## Claude Desktop and other MCP clients

```powershell
mother connection --config "<client config>" --format mcp-servers
```

prints a `{"mcpServers": {"mother": …}}` block, the shape used by Claude Desktop and many other clients. `--format json` prints just the command and arguments for anything else.

## Checking a harness

`mother doctor --config <that harness's config>` checks that the installed executable answers for that grant. It does not prove a harness has loaded the connection: open the harness and ask it what it knows about you from M.O.T.H.E.R.
