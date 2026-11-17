# Strict ancestor ACL checks implementation plan

> Execute inline with test-driven development and verification skills. The owner
> approved this design; do not request the same scope approval again. Review is
> advisory. Do not commit, push, install or change existing permissions as part
> of qualification.

**Goal:** Remove SID-shape trust from Mother's Windows ancestor checks, retaining
strict private-directory admission and useful privacy-safe errors.

**Architecture:** The existing filesystem reader delegates to an in-memory
Windows descriptor evaluator. Both tests and production use that evaluator.
Only actual trusted principals bypass access-mask checks. No storage contract,
provider binding, graph authority, capture logic or publication logic changes.

**Tech stack:** Rust 2024/MSRV 1.93, locked windows-permissions 0.2.4, existing
native redb runtime. No new dependency or unsafe code.

- [x] Inspect current source, call sites, native error boundary and SDK descriptor
  parsing; preserve the clean base on `g9000/strict-ancestor-acls`.
- [x] Obtain owner approval for strict rejection and clear errors without ACL edits.
- [x] Review this design and plan for boundary mistakes; incorporate explicit
  current-user trust, inheritance/deny ordering and non-normalized object ACEs.
- [x] Establish configured Mother MCP project context using read-only calls.
- [x] Add in-memory SDDL regressions and mechanically extract the current policy
  unchanged. Run `cargo test --locked private_paths` and record the expected
  security assertion failures, not compile/setup failures.
- [x] Remove prefix trust and introduce bounded reason-specific messages while
  keeping the stable error code and private leaf policy. Run focused regressions.
- [x] Update `docs/garden-service.md` with exact behavior and limitations.
- [x] Run `cargo fmt --all -- --check`, `cargo test --locked`,
  `cargo build --release --locked`,
  `cargo clippy --locked --all-targets --all-features -- -D warnings`, and
  `cargo tree --locked --edges normal,build`; exclude SQLite dependencies.
- [x] Review the final diff, preserve installed/data/client state, save the
  required technical checkpoint, and clean only verified owned scratch.

Expected source files: `src/private_paths.rs` (policy extraction and repair),
`src/private_paths/windows_tests.rs` (pure Windows regressions), and
`docs/garden-service.md`. Existing Windows tests that mutate directory ACLs are
replaced by descriptor tests. Unrelated garden/explorer files stay unchanged.

Acceptance includes: unknown long package/capability identities never bypass
unsafe-right checks; harmless ancestor RX/S,X remain allowed; private leaf reads
by other identities remain rejected; unsupported/missing metadata fails closed;
code path and diagnostics are tested without editing any on-disk ACL.

The RED library run reproduced six enforcement/diagnostic failures and a genuine
NULL-DACL panic in the pinned SDK; the full initial attempt separately found four
missing explorer notice includes. The notices were recovered byte-for-byte from
the original `d120010` commit, with provenance retained privately. The repair uses
Windows' read-only DACL-to-SDDL conversion before borrowing the SDK's DACL view,
so missing/null/unreadable descriptors return a fenced error without unsafe code,
dependency changes or panic recovery. All 17 focused checks then passed.

Final Windows qualification passed: 239 full-suite tests, six intentionally
ignored cases, formatting, release build, strict Clippy and the SQLite-free
normal/build dependency audit. Installed executable and seven client hashes
are unchanged. Technical checkpoint `58acfccb-1d2d-464e-a2bb-9bbccd5e0483`
revision 1 was saved and read back through the existing authenticated local MCP
bridge; workspace generation is unchanged. The owned synthetic root was removed
after evidence archival. See [qualification](../../benchmarks/strict-ancestor-acls.md).
