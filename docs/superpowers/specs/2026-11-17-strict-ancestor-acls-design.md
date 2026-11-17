# Strict ancestor ACL checks

Owner approved the strict-checks approach on 17 November 2026: reject unsafe
ancestor grants with a clear error, allow harmless ancestor read/traverse rights,
and make no folder-permission changes. Mother stays independent and additive;
memory, grants, generation and selected sharing are outside this change.

The Windows ancestor exception in `src/private_paths.rs` trusts the shape of an
AppContainer/capability SID. That bypasses the access-mask check for unknown
principals, including grants permitting child deletion, directory deletion,
ownership changes or DACL changes. A long SID does not establish a trusted actor.
The existing OS ACL test can also pass with an already-rejected fixture, so it
does not prove that a safe path remains safe.

Restore the same exact principal trust boundary for every SID category: current
process user, SYSTEM and Administrators. Keep the existing owner-only leaf requirement,
Creator Owner inheritance handling, fail-closed unsupported ACE handling,
reparse rejection, single-link checks and conservative deny-ACE treatment.
Evaluate unknown ancestor allow ACEs against the replacement/control mask rather
than a SID prefix. Preserve `MIGRATION_FENCED`, distinguishing private-directory
access from ancestor replacement/control rejection without exposing paths,
credential text or full security descriptors through the shared error type.

Factor descriptor evaluation from the filesystem reader so tests can parse SDDL
and exercise the production evaluator entirely in memory. First extract the old
decision path unchanged, including its defect; demonstrate failing security
regressions, then remove the bypass. Test the reported SID, arbitrary capability
and package SIDs, short application groups and ordinary identities against each
dangerous right and benign read/traverse rights. Cover trusted identities,
inherit-only ACE placement, protected targets, null DACLs, unsupported ACE types
and exact privacy-safe diagnostics. Do not call ACL setters, `icacls /grant`,
`chmod`, or analogous permission mutations in these new Windows tests.

Review refinements: pass the actual current-process SID explicitly to the
evaluator; never infer trusted principals from a descriptor's owner. Test a
foreign-owned ancestor's dangerous self-grant. Preserve ancestor inherit-only
skipping before ACE dispatch, private-leaf Creator Owner handling, and rejection
of dangerous unknown allows even when deny ACEs precede or follow them. An
unsupported object-ACE fixture must include a GUID and assert its parsed ACE
type, because Windows normalizes GUID-free object ACEs to ordinary ACEs.

This is a source-and-test repair, not an installation. Existing installed owner,
stores and clients remain untouched. A genuinely unsafe parent will still be
rejected; supporting it safely would need a separate anchored-access design.
No claim is made that this snapshot ACL check eliminates every filesystem race.

Qualification uses owned synthetic temporary directories that inherit already
private permissions from an existing owner-private evidence directory. Only
process-local TEMP/TMP values are set; existing ACLs and live harness settings
are never changed. Run formatting, locked full tests, release build, strict
Clippy and the normal/build dependency audit. Preserve unrelated source and
archive failed as well as passing evidence before owned-scratch cleanup.
