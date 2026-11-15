# G8 cutover qualification

The G1–G7 foundation and preserved additive pairing safeguards were merged and
pushed to `origin/main` at `30f991b8d91fb56accaf44125a269b25436a4e30` before G8
implementation. G8 is being qualified separately on `g9000/mother-garden-cutover`.

## Reproduce the builds

The default feature retains the existing SQLite runtime and explicit converter.
The native executable uses the same package with `--no-default-features`:

```powershell
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo build --release --locked
cargo test --locked --no-default-features
cargo clippy --locked --no-default-features --all-targets -- -D warnings
cargo build --release --locked --no-default-features
cargo tree --locked --no-default-features --edges normal,build --prefix none
```

Builds share `target/release/mother` (`mother.exe` on Windows), so retain the
converter and native executable separately before building the other variant.
The native command tree contains `garden` and `mcp`; the legacy connection
dashboard, launch setup and SQLite administration remain in the default build.
Native owner review uses `garden review` with an explicit reviewed JSON request;
approval is never an IPC or MCP operation.

The native normal/build tree must contain neither `rusqlite` nor
`libsqlite3-sys`. The check also excludes `keyring`, `ratatui` and `crossterm`.
This qualifies the pure-Rust storage engine and SQLite-free runtime dependency
boundary. Named pipes, sockets, file ownership, ACL inspection and durable file
operations still use operating-system APIs through Rust wrappers.

## Private synthetic paths

Service and migration tests require a real owner-private temporary root. They
reject a permissive ancestor that could replace their endpoint or journal. A
shared or sandbox-modified Windows temp directory may fail this requirement.
Create a dedicated synthetic directory without changing existing data ACLs:

```powershell
$testRoot = Join-Path $env:LOCALAPPDATA ('Mother-tests-' + [guid]::NewGuid())
New-Item -ItemType Directory -Path $testRoot -ErrorAction Stop | Out-Null
$owner = [System.Security.Principal.WindowsIdentity]::GetCurrent().Name
icacls $testRoot /inheritance:r /grant:r "${owner}:(OI)(CI)F" 'SYSTEM:(OI)(CI)F'
if ($LASTEXITCODE -ne 0) { throw 'Private test root could not be secured' }
$env:TEMP = $testRoot
$env:TMP = $testRoot
cargo test --locked
```

On Unix use a private `0700` temporary root. Fixtures canonicalize system temp
aliases before constructing migration paths, including macOS's `/var` alias.
Explicit endpoint/file symlinks, reparse points and hard-linked database aliases
remain rejected. Never point these tests at an owner workspace or reuse an
existing journal.

## Covered recovery boundaries

Synthetic tests exercise complete v1 conversion, exact graph-only and write-only
grant reissue, changed-source refusal, already-open source mutation fencing,
fresh generations, historical receipts and blocked old proposals. Fault hooks
and real subprocess exits cover preparation and durable authority barriers.
Rollback checks all canonical state, including blobs, pins and assignments; a
missing or changed destination requires reconciliation. Retrying an older
rollback cannot clear a later cutover's source fence.

Real local processes exercise two clients and two MCP stdio bridges, immutable
receipt retries across owner restart, duplicate-owner refusal, current grant
revocation and bounded malformed peers. Authentication tests cover a redirected
owner and replayed proofs without transferring the bearer token. The Windows
pipe implementation handles partial/empty nonblocking reads and bounded writes
without spawning unbounded flush workers. Credential outputs reject broad read
permissions before publishing an intent or a token.

See [migration and rollback](garden-migration.md) and
[local service operation](garden-service.md) for the owner workflow.

## Platform scope

Qualification completed on 15 November 2026:

| Check | Result |
| --- | --- |
| Default `cargo test --locked` | 484 passed, 6 ignored |
| Native `cargo test --locked --no-default-features` | 152 passed, 3 ignored |
| Format and Git whitespace checks | Passed |
| Strict all-target Clippy, default/all-features and native | Passed |
| Locked release build, default and native | Passed |
| Rustdoc with `-D warnings`, default and native | Passed |
| Native normal/build dependency audit | 598 tree lines; no SQLite or legacy UI/keyring crates |
| Independent implementation and requirements review | No unresolved blockers |

Four default ignored entries and all three native ignored entries are synthetic
subprocess helpers invoked by their recovery parent tests. The other two default
ignored tests require the optional verified local embedding model; that model
was not installed or downloaded for this qualification.

The native Windows release artifact's SHA-256 was
`0c0e7c5ca0c7b56d401b0c84579b8a4a96ad85ab09a1d63deb1b884ee55ea1b1`;
its actual `--help` starts successfully and exposes the native command tree.

Local qualification uses Windows x86-64, Rust/Cargo 1.93.0 and synthetic data.
Linux/macOS runtime checks, physical power-loss testing, completed-engine scale
benchmarks and real harness adoption remain G9 acceptance work. The repository's
existing CI automation is unchanged. No live store, installed binary, grant or
harness configuration is migrated by these checks.
