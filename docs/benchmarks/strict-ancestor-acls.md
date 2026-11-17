# Strict Windows ancestor ACL qualification — 17 November 2026

Source repair on `g9000/strict-ancestor-acls`, based on
`be28b6026cb1bfa4f4dafdf089ea77f31a5f24a5`.
The owner approved restoring strict admission and clear errors without changing
folder permissions. This report covers source qualification, not installation.

## Change and regression evidence

The former AppContainer/capability SID-shape exemption bypassed ancestor
replacement/control checks. Every unknown principal now faces the same mask;
current-process user, SYSTEM and Administrators retain their existing trust.
Harmless ancestor read/traverse access remains accepted, and other-party access
to the private directory remains rejected. No storage, graph, grant, provider,
memory extraction or selected-sharing contract changes.

Tests parse Windows security descriptors in memory and call the production
evaluator. They do not set filesystem ACLs. The extracted old policy produced
10 passing and seven failing checks: six enforcement/diagnostic failures plus a
NULL-DACL panic in the pinned `windows-permissions` SDK. A read-only DACL-to-SDDL
preflight now fences missing/null/unreadable DACLs before the SDK borrows them.
The repaired focused suite passed all 17 checks.

The full command initially could not compile because four existing explorer
notice includes were absent. `LICENSE.txt`, `LICENCE-NOTES.md`,
`THIRD-PARTY-NOTICES.md` and `CREDITS.md` were recovered byte-for-byte from their
original `d120010` Git blobs. No explorer application or artwork changed.

## Native qualification

Windows checks use an owned synthetic root inheriting an already-private parent,
process-local TEMP/TMP and a synthetic MOTHER_HOME. No ACL setter, live-store
administration, model download, remote model exchange or Discord test is part
of this repair.

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed |
| `cargo test --locked private_paths -- --nocapture` | 17 passed |
| `cargo test --locked` | 239 passed, zero failed, six intentionally ignored |
| `cargo build --release --locked` | Passed |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed |
| `cargo tree --locked --edges normal,build` | Passed; no `rusqlite` or `libsqlite3-sys` |

The ignored cases are three parent-invoked process helpers, one demonstration
export and two opt-in external AnimaOS daemon cases. No Linux/macOS run, live
host adoption or safe operation below a genuinely writable ancestor is claimed.
Final code/security review found no blocking P1/P2 issues.

## Preservation and operational limit

The installed Mother binary and all seven private client files have unchanged
SHA-256 hashes. Existing personal records are not edited or read directly as
database files. The only context write was the contributor-required technical
checkpoint `58acfccb-1d2d-464e-a2bb-9bbccd5e0483` revision 1, confirmed by readback
through the existing authenticated local MCP connection; workspace generation
is unchanged. The owned synthetic root was removed after evidence archival.
No service restart, configuration change, permission change or binary installation
is authorized by this qualification.

An unsafe ancestor still produces `MIGRATION_FENCED`. Rejected allow entries
identify the private-directory/ancestor relationship, depth and access mask,
without exposing paths, SIDs or descriptors. This is a conservative snapshot
check, not a complete effective-access calculator or a guarantee against all
filesystem races. Safely supporting a writable parent needs a separate
anchored-access design; a SID whitelist or child exclusion is insufficient.

Windows semantics were checked against the pinned SDK and Microsoft's
[directory rights](https://learn.microsoft.com/en-us/windows/win32/wmisdk/file-and-directory-access-rights-constants),
[ACE syntax](https://learn.microsoft.com/en-us/windows/win32/secauthz/ace-strings),
[descriptor syntax](https://learn.microsoft.com/en-us/windows/win32/secauthz/security-descriptor-string-format)
and [read-only descriptor conversion](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsecuritydescriptortostringsecuritydescriptorw)
documentation.
