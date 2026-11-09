# Terminal connection experience

The owner approved a native connection manager and asked to improve the TUI. Build a small Rust terminal home around the existing independent context layer. Preserve the README style, six MCP tools, v1 store and all existing owner commands.

## Experience

`mother tui` opens an interactive dashboard; bare `mother` does so only on a real input/output terminal, otherwise keeps help output. Calm violet/cream palette, M.O.T.H.E.R title, concise navigation and explicit keyboard hints. Home shows local workspace availability and ChatGPT connection state. Connection setup collects an existing tunnel ID, the separate ChatGPT client config and installed official tunnel-client path; runtime key input is masked and stored only in the native OS credential store. It never creates cloud resources silently. It offers connect, refresh, disconnect and optional Windows sign-in startup. A guidance page explains Tunnel / No authentication only after readiness; it never presents local readiness as actual ChatGPT read/save acceptance.

CLI equivalents: `mother connect chatgpt`, `mother status`, `mother disconnect chatgpt`; interactive setup is shared with the dashboard and supports saved settings. Noninteractive calls never prompt. Optional Windows startup is explicitly toggled, off by default, and uses the same stable executable/configuration without embedding keys. Other platforms report unavailable startup integration.

## Boundaries

The connection adapter delegates process supervision to official `tunnel-client runtimes connect/status/stop`; never own a second competing supervisor. Non-secret connection settings are stored outside Git under MOTHER_HOME. Secret keys go to an OS vault and child process environment, never argv, stdout, settings, or crash diagnostics. Sanitize all external output; subprocesses have deadlines and bounded captured output. Status distinguishes unconfigured, missing dependencies/credentials, stopped, starting/unhealthy, ready and inspection failure. Repeated connect first reconciles existing status, and cannot report success without running+healthy+ready. A timeout is unknown, not permission to replay an effect. Read-only dashboard/status never creates grants, records or cloud resources. Runtime ownership must match the configured tunnel ID, generated profile and workspace-specific alias before accepting readiness, starting, or stopping. Missing/mismatched identity fails closed. An unrelated or legacy runtime is never implicitly replaced or stopped; timeout after a possible effect requires status reconciliation without automatic replay.

Existing CLI-generated private config is reused. Missing grant gets actionable owner setup guidance; initial version does not silently grant access. Existing PowerShell setup can coexist but is no longer required for native-managed setup. Windows path quoting must work with spaces and must reject command/parser control characters. Backend is kept out of mother library/store/MCP execution. UI restores the terminal on error/exit, handles resize/small screens and keeps secret text out of rendering/debug.

## Validation

Use synthetic temp settings and fake tunnel runner/vault for parser, secret, error, process, reconnection and startup boundary tests. Test TUI rendering/state with Ratatui TestBackend, masked input and non-TTY CLI behavior. Run all existing tests, fmt, release build and Clippy. No live cloud writes or real memory edits as tests. Verify installed local TUI separately if possible; do not claim actual ChatGPT acceptance until real calls pass.
