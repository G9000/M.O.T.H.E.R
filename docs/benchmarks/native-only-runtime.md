# Native-only runtime qualification

Mother now builds the native redb garden by default. The SQLite backend,
`legacy-sqlite` feature, runtime fallback and source converter are removed.
Existing native v1 context tools and graph tools share the same transactional
store. Native graph and context owner-review screens remain available; inbox
commands emit JSON when piped.

The old SQLite setup/app dashboard, managed tunnel, launcher, model installer
and general owner CLI are retired. Native setup, grants, service, review and
backup use the commands in [setup](../setup.md). Already verified local model
assets remain usable. Production has no HTTP download dependency; the dev-only
HTTP fixture uses `ureq` without TLS. Native terminal review uses Rust
`ratatui`/`crossterm` libraries.

Unsupported old credentials fail before opening a database. Regression tests
preserve the synthetic old file byte for byte. No live records, credentials,
installed executable, service or harness settings were changed. Existing native
identity, grants, revisions, immutable receipts and owner approval boundaries are
preserved. Historical cutover journals still support status, exact grant reissue
and eligible rollback; existing fences remain authoritative. Source preparation
and activation commands are removed.

## Verification

| Check | Result |
| --- | --- |
| Default `cargo test --locked` | 211 passed, zero failed, five ignored across 38 summaries. |
| Six retained native recovery cases | Exact graph-only/write-only/shared reissue, retry identity, revocation, unsafe output refusal, rollback integrity and newer-fence preservation pass. |
| TypeScript golden v1 compatibility | Original requests, receipt/hash replay, integral numeric revisions and JavaScript whitespace normalization pass on native storage. |
| Actual AnimaOS native HTTP workflows | Two passed, zero failed: context selected sharing/reconciliation and full graph discovery/review/grants/restart/revocation. |
| All seven graph operations | Actual AnimaOS ToolExecutionContext, stdio bridge and native owner pass against the new default release binary. |
| Required package checks | Format, strict all-target/all-feature Clippy and locked release build pass. |
| Documentation and platform compilation | Warning-free Rustdoc and native macOS all-target cross-compile pass. macOS execution was not run. |

The five default-suite ignores are the two opt-in native HTTP workflows and
three synthetic recovery process entry points. No installed-model or network
provider diagnostics were enabled. Tests use owned private temporary roots and
synthetic records. Counts are command aggregates. The native owner UI retains
controller/render tests; no new visual or physical terminal acceptance is claimed.

The [manifest](garden-acceptance-evidence/native-only-runtime-manifest.json)
records source, artifact, dependency and result hashes. The production
normal/build dependency tree excludes SQLite, the old keyring, HTTP client and
TLS/download chain. The lockfile also excludes SQLite; removal prunes unused
packages without upgrading remaining versions. AnimaOS source is unchanged from
the preceding full-graph qualification.

## Existing G9 assessment

**Release HOLD remains.** Original native write latency, lexical recall,
contention availability, 1m loading deadline and pending platform/physical/model
acceptance have not been remeasured. The AnimaOS default-parallel test failures
remain unresolved. The deleted SQLite HTTP workflow is retired, not fixed or
requalified. The two surviving native HTTP workflows pass on the current source.

The native graph writer, compatibility writer/search, backups, graph contracts,
service authentication, indexed queries, streaming audit and benchmark runners
retain their prior source hashes. Existing G9 measurements and failed synthetic
datasets remain distinct from this removal qualification. The previous
[full-graph assessment](garden-acceptance-evidence/full-graph-assessment.txt),
[full-graph manifest](garden-acceptance-evidence/full-graph-admission-manifest.json)
and earlier G9 evidence are preserved.

Reproduce from the Mother worktree with an isolated `CARGO_TARGET_DIR` and a
private temporary root:

```text
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo build --release --locked
cargo doc --locked --no-deps
cargo tree --locked --all-features --edges normal,build
cargo check --locked --all-targets --target x86_64-apple-darwin
cargo test --locked --test mcp additive_anima:: -- --ignored --test-threads=1
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. The HTTP opt-in requires
`ANIMA_MOTHER_TEST_DAEMON` to name the explicitly built additive AnimaOS daemon.
The separately enabled AnimaOS graph executor test uses its managed wrapper and
`ANIMA_WORKSPACE_CONTEXT_TEST_MOTHER` pointing at this default native-only
Mother release. No installation, migration, model call, commit or push is part
of qualification.
