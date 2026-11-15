# G8 Migration, Local Service and Native Build

> **For agentic workers:** Use superpowers:subagent-driven-development for the independent build, service and migration tasks. Root owns integration, Cargo validation and final review.

**Goal:** Provide an explicit, recoverable one-machine cutover and a native executable whose dependency graph excludes SQLite.

**Architecture:** One owner service holds the native database and receives bounded authenticated local IPC. Stdio MCP bridges retain configured client identity and the existing domain operations. A separate legacy-enabled converter prepares and validates a new native store, fences the source before activation and records every cutover phase. Native-only builds retain journal inspection, native service, MCP and storage operations while excluding legacy conversion.

**Tech stack:** Rust, redb 4.6.0, interprocess 2.4.8 local sockets/named pipes, existing MCP SDK and optional `legacy-sqlite` feature.

## Preconditions and scope

- [x] Commit and integrate the existing G1-G7 source, pairing safeguards, approved design and G2 qualification evidence.
- [x] Validate the assembled source: 453 tests and required format/build/Clippy checks.
- [x] Push and verify origin/main at `30f991b8d91fb56accaf44125a269b25436a4e30` before G8 implementation.
- [x] Create isolated `g9000/mother-garden-cutover` from that pushed commit.

This task implements the workflow and exercises synthetic stores only. It does not migrate personal data, install the new executable, change live grants, launch a real harness or activate an existing endpoint. Ordinary runtime upgrades retain their current generation. Actual host adoption and the G9 scale/release matrix remain separate.

## Build boundary

- [x] Make `rusqlite` optional behind default `legacy-sqlite` for existing releases and tests.
- [x] Separate SQL implementations from native/shared validators, policy types, ranking and contract helpers.
- [x] Build the native executable with `--no-default-features`; reject legacy configs explicitly.
- [x] Check normal/build dependency trees contain no `rusqlite` or `libsqlite3-sys` for that executable.
- [x] Retain default-feature regression coverage and add native-only coverage.

## Single-owner service

- [x] Add strict versioned request/response frames, bounded bytes and deadlines, fixed local endpoint naming and authenticated domain dispatch.
- [x] Hold one database owner and reject duplicate owners; avoid any TCP/public listener.
- [x] Recheck current grants, workspace and generation for every request and preserve operation UUID receipt semantics.
- [x] Add an explicit service config and stdio bridge; no IPC approval, grant or launch operations.
- [x] Exercise two clients, reconnect/restart, revoked keys, wrong identity, malformed/slow/oversized peers and real process ownership.

## Migration and cutover

- [x] Read a consistent source snapshot without rewriting canonical source data; preserve every supported v1 section and reject unsupported required state.
- [x] Prepare a fresh-generation native destination and compare complete state, source pointers, persona and retained receipts.
- [x] Keep owner-only grant restrictions outside public bundles and reissue only the reviewed restrictions with new credentials.
- [x] Use a durable UUID-bound journal, final source fingerprint check and persistent source/destination fences.
- [x] Fence already-open source writers before native activation. Repeated/uncertain actions reconcile the existing phase.
- [x] Archive source without destroying it; detect stale configs and inactive destinations.
- [x] Permit rollback before destination canonical changes; after changes require reconciliation and preserve acknowledged state.
- [x] Test interruption/recovery across migration phases and old-generation proposal/receipt behavior.

## Acceptance

- [x] Default four required checks and Rustdoc pass.
- [x] Native-only build, tests, strict Clippy, Rustdoc and dependency audit pass.
- [x] Real local IPC and synthetic migration/cutover tests pass, with no live context or settings used.
- [x] Independent requirements and code review findings resolved.
- [x] Record exact platform coverage, results, remaining operational limits and recovery instructions.

Validated on Windows with Rust/Cargo 1.93.0: 484 default tests and 152 native tests passed; both release builds, both strict Clippy checks, Rustdoc and the native dependency audit passed. See [qualification evidence and limits](../../garden-cutover-validation.md). G8 remains uncommitted on its isolated branch; Linux/macOS runtime and real host adoption remain G9 work. Existing CI automation is unchanged.
