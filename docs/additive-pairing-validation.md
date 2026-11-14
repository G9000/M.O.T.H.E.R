# Additive AnimaOS pairing validation

The G1 pairing slice builds on the existing connection replacement, slot-limit,
old-program-path and error-reporting edits. Those four changed files were copied
byte-for-byte from the primary Mother checkout into `g9000/mother-pairing-additive`
before implementation. The primary checkout and its existing edits are retained.

New scoped pairings install AnimaOS's version-1 additive memory provider with
selected publication and local personality. Full garden access and a record
allowlist remain distinct grant choices. Workspace personality is a separate
opt-in available only with companion access. Grants stay exactly `read` and
`context:propose`; existing credentials and allowlists are preserved.

Setup detects additive-provider metadata before provisioning a key. An existing
matching primary binding is shown before a separate conversion choice; conversion
clears that binding and saves the provider in one revision-checked request.
Different bindings are preserved. Uncertain saves and tool reviews retain their
intent and reread the exact saved outcome rather than replaying the mutation.

Under Automatic policy, setup reviews the current six recall, selected-proposal
and outcome descriptors. Review policy keeps manual review in AnimaOS. Connection
repair clears only bindings referencing the explicitly selected old connections.

The new UI regression initially failed compilation on the absent persona and
conversion state. The additive pairing regression then reached the backend and
failed against the fixture's old metadata. Focused tests passed after implementation;
final check results are recorded below after review.

Validation uses synthetic local HTTP peers, private temporary stores and owned
test directories. Component rendering uses Ratatui's test backend. A real model
conversation, interactive terminal use and installed-host adoption are separate
acceptance steps. No live client settings, grants or personal records are changed.

Final Windows validation on 15 November 2026:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test --locked` | 297 passed, zero failures, two optional model tests ignored. |
| `cargo build --release --locked` | Passed. |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed. |
| `git diff --check` | Passed. |

The final suite includes partial non-reader review recovery: after the second
tool review applies but its acknowledgement is lost, setup verifies its exact
descriptor, discovery and next version before completing all six reviews. It
sends exactly six review requests, with no duplicated connection or review.
Other regressions cover unsupported daemons, wrong subjects, stale saves,
workspace-generation mismatches, owner personality selection, empty allowlists,
conversion without changing grant bytes, and relevant/unrelated provider repair.
Strict Clippy's initial unnecessary borrow and mutable binding warnings were
fixed before the final run. Both specification and code reviews found no remaining
actionable issues.

The build uses Mother's independent locked Cargo package. It has no new
dependency, schema or MCP tool change. Implementation is uncommitted in the
isolated worktree and has not been installed or applied to any running harness.
Task-owned temporary stores and logs are removed after these results are recorded.

## Integration before G8

On 15 November 2026, additive pairing was assembled with the verified G7 source,
the original pairing replacement safeguards and G2's retained qualification
artifacts. The integrated package passed 453 tests with zero failures and four
ignored tests, formatting, the release build and strict all-target/all-feature
Clippy. The ignored tests retain the two optional installed-model cases and two
private recovery children executed by their parent tests. Independent merge
review confirmed the existing repair logic and tests were retained. This is
source integration evidence, not a live connection, data migration or install.
