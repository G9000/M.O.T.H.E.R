# The hearth: starting a harness on her work

The hearth is how M.O.T.H.E.R starts work in another harness. This is stage 1: you choose when to launch, and a launch only runs a command you declared in advance. Later stages may let her propose launches for your approval, then launch within limits you set; neither exists yet.

## Declare launchers

Create `hearth/launchers.json` inside your private data folder (`%LOCALAPPDATA%\MOTHER` on Windows, `~/.mother` elsewhere, or `MOTHER_HOME`):

```json
{
  "version": 1,
  "launchers": {
    "claude-code": {
      "harness": "claude-code",
      "command": "/Users/you/.local/bin/claude",
      "args": ["-p", "{prompt}"],
      "cwd": "/Users/you/projects/mother",
      "timeoutSeconds": 1800
    },
    "codex": {
      "harness": "codex",
      "command": "/usr/local/bin/codex",
      "args": ["exec", "{prompt}"],
      "timeoutSeconds": 1800
    }
  }
}
```

| Field | Meaning |
| --- | --- |
| `harness` | The grant name of the harness this starts. The task is handed to it, so it must already have a grant (see [harnesses](harnesses.md)). |
| `command` | Absolute path of the program. It runs directly, with no shell. |
| `args` | Arguments. `{prompt}` becomes the launch instructions and `{task}` the task ID, wherever they appear. |
| `cwd` | Optional absolute working folder. |
| `timeoutSeconds` | Optional, 1 to 86,400; default 1,800. |

Only you edit this file. M.O.T.H.E.R never adds or changes launchers, and nothing an MCP client sends can choose a command.

Each harness keeps its own permissions. A harness started headless can only do what its own settings allow; if it needs an approval you are not there to give, it will wait until its timeout. Set that harness's permission options in `args` deliberately, the same way you would when running it yourself.

```powershell
mother launch --list
```

shows each launcher and whether it is ready, with the reason if not.

## Launch

```powershell
mother launch <task-id> --launcher codex
```

1. Checks the task is open (todo, doing or blocked) and that no earlier launch of it is still pending or unknown.
2. Hands the task to the launcher's harness, exactly like `mother_assign_task`.
3. Records a `pending` action, by `@owner-launch`, referencing the task.
4. Runs the command with output going to a private log under `hearth/runs/`.
5. Records the outcome on the same action: `succeeded`, `failed`, or `unknown` if it was stopped at its timeout.

It prints the action ID, outcome, exit code and log path, and exits with an error unless the run succeeded.

The prompt the harness receives is fixed text naming only the task ID: read it with `mother_get_record`, work under your own permissions, update its status, save a checkpoint. Task titles and content are never pasted into the command line, so saved text cannot steer what runs; the harness reads the task through her MCP tools like any other record.

## When the outcome is unknown

A run stopped at its timeout may have done part of its work. M.O.T.H.E.R does not guess: the action stays `unknown`, appears in unsettled work, and a second launch of the same task is refused. Look at the log and the harness's own record of what it did, then either update the task yourself or run:

```powershell
mother launch <task-id> --launcher codex --retry-unknown
```

The same applies to a launch left `pending` because `mother launch` itself was interrupted. A `failed` launch is settled, so trying again needs no flag.

## Stopping

A launch runs in your terminal and ends when its command exits or its timeout is reached. Press Ctrl+C to stop it; the action then stays `pending` until you check it and retry or settle the task. Revoking a harness's grant stops that harness from reading or writing her context on its next call.
