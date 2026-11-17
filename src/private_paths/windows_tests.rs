//! Exercise the production Windows evaluator with in-memory SDDL; no ACL setters.
use super::windows_descriptor;
use crate::error::Result;
use windows_permissions::{LocalBox, SecurityDescriptor, Sid, constants::AceType};

const CURRENT: &str = "S-1-5-21-111-222-333-1001";
const FOREIGN: &str = "S-1-5-21-444-555-666-1002";
const REPORTED: &str =
    "S-1-15-3-3557520199-3666692283-3112367039-3524159787-2791857073-3163583606-3692855932";
const CAPABILITY: &str = "S-1-15-3-1024-11-22-33-44-55-66-77-88";
const PACKAGE: &str = "S-1-15-2-11-22-33-44-55-66-77";
const UNKNOWN: [&str; 7] = [
    REPORTED,
    CAPABILITY,
    PACKAGE,
    "S-1-15-2-1",
    "S-1-15-3-1",
    "BU",
    "WD",
];
const DANGEROUS: [u32; 7] = [
    0x001f01ff, 0x10000000, 0x40000000, 0x40, 0x10000, 0x40000, 0x80000,
];

fn descriptor(owner: &str, aces: &str) -> LocalBox<SecurityDescriptor> {
    format!("O:{owner}G:BAD:P(A;;FA;;;{CURRENT})(A;;FA;;;SY)(A;;FA;;;BA){aces}")
        .parse()
        .unwrap()
}

fn evaluate(sd: &SecurityDescriptor, depth: usize) -> Result<()> {
    let current: LocalBox<Sid> = CURRENT.parse().unwrap();
    let system: LocalBox<Sid> = "SY".parse().unwrap();
    let admins: LocalBox<Sid> = "BA".parse().unwrap();
    let creator: LocalBox<Sid> = "CO".parse().unwrap();
    windows_descriptor(
        sd,
        &[current.as_ref(), system.as_ref(), admins.as_ref()],
        creator.as_ref(),
        depth,
    )
}

#[test]
fn reported_capability_full_control_does_not_bypass_ancestor_policy() {
    let sd = descriptor(CURRENT, &format!("(A;;FA;;;{REPORTED})"));
    assert_eq!(evaluate(&sd, 1).unwrap_err().code(), "MIGRATION_FENCED");
}

#[test]
fn package_full_control_does_not_bypass_ancestor_policy() {
    let sd = descriptor(CURRENT, &format!("(A;;FA;;;{PACKAGE})"));
    assert_eq!(evaluate(&sd, 1).unwrap_err().code(), "MIGRATION_FENCED");
}

#[test]
fn every_unknown_sid_category_obeys_each_replacement_and_control_right() {
    for sid in UNKNOWN {
        for mask in DANGEROUS {
            for flags in ["", "ID", "OICI"] {
                let sd = descriptor(CURRENT, &format!("(A;{flags};0x{mask:08x};;;{sid})"));
                for depth in [1, 7] {
                    assert!(
                        evaluate(&sd, depth).is_err(),
                        "unknown {sid}, mask {mask:#x}, flags {flags}, depth {depth}"
                    );
                }
            }
        }
    }
}

#[test]
fn ordinary_users_and_broad_application_groups_remain_rejected() {
    for sid in [FOREIGN, "BU", "WD", "S-1-15-2-1", "S-1-15-3-1"] {
        let sd = descriptor(CURRENT, &format!("(A;;FA;;;{sid})"));
        assert_eq!(evaluate(&sd, 1).unwrap_err().code(), "MIGRATION_FENCED");
    }
}

#[test]
fn benign_ancestor_read_traverse_and_synchronize_do_not_fence() {
    for sid in UNKNOWN {
        for rights in ["FRFX", "0x00100020", "0x001200a9", "GRGX"] {
            let sd = descriptor(CURRENT, &format!("(A;OICIID;{rights};;;{sid})"));
            assert!(evaluate(&sd, 1).is_ok(), "benign {sid}, rights {rights}");
        }
    }
}

#[test]
fn private_leaf_remains_owner_only_even_for_benign_other_reads() {
    for sid in UNKNOWN {
        let sd = descriptor(CURRENT, &format!("(A;;FRFX;;;{sid})"));
        assert_eq!(evaluate(&sd, 0).unwrap_err().code(), "MIGRATION_FENCED");
    }
}

#[test]
fn only_explicit_current_user_system_and_administrators_are_trusted() {
    for sid in [CURRENT, "SY", "BA"] {
        let sd = descriptor(sid, &format!("(A;;GA;;;{sid})"));
        assert!(evaluate(&sd, 0).is_ok());
        assert!(evaluate(&sd, 1).is_ok());
    }
}

#[test]
fn descriptor_owner_never_whitelists_a_foreign_ancestor_principal() {
    let sd = descriptor(FOREIGN, &format!("(A;;FA;;;{FOREIGN})"));
    assert!(evaluate(&sd, 0).is_err());
    assert!(evaluate(&sd, 1).is_err());
}

#[test]
fn deny_entries_do_not_cancel_rejection_of_unknown_dangerous_allows() {
    for aces in [
        format!("(D;;FA;;;{REPORTED})(A;;FA;;;{REPORTED})"),
        format!("(A;;FA;;;{REPORTED})(D;;FA;;;{REPORTED})"),
    ] {
        let sd = descriptor(CURRENT, &aces);
        assert!(evaluate(&sd, 1).is_err());
    }
    let sd = descriptor(CURRENT, &format!("(D;;FA;;;{REPORTED})"));
    assert!(evaluate(&sd, 1).is_ok());
}

#[test]
fn inherit_only_ancestor_aces_do_not_apply_to_that_ancestor() {
    let sd = descriptor(CURRENT, &format!("(A;OICIIO;GA;;;{CAPABILITY})"));
    assert!(evaluate(&sd, 1).is_ok());
    assert!(
        evaluate(&sd, 0).is_err(),
        "a private leaf must protect its future contents too"
    );
}

#[test]
fn creator_owner_inherit_only_is_preserved_but_effective_creator_is_not_trusted() {
    let sd = descriptor(CURRENT, "(A;OICIIO;GA;;;CO)");
    assert!(evaluate(&sd, 0).is_ok());
    let sd = descriptor(CURRENT, "(A;;GA;;;CO)");
    assert!(evaluate(&sd, 0).is_err());
    assert!(evaluate(&sd, 1).is_err());
}

#[test]
fn unsupported_object_ace_fails_closed_without_normalization_hiding_it() {
    let sd = descriptor(
        CURRENT,
        &format!("(OA;;FA;11111111-1111-1111-1111-111111111111;;{CAPABILITY})"),
    );
    assert_eq!(
        sd.dacl().unwrap().get_ace(3).unwrap().ace_type(),
        AceType::ACCESS_ALLOWED_OBJECT_ACE_TYPE
    );
    assert!(evaluate(&sd, 0).is_err());
    assert!(evaluate(&sd, 1).is_err());
    let sd = descriptor(
        CURRENT,
        &format!("(OA;IO;FA;11111111-1111-1111-1111-111111111111;;{CAPABILITY})"),
    );
    assert_eq!(
        sd.dacl().unwrap().get_ace(3).unwrap().ace_type(),
        AceType::ACCESS_ALLOWED_OBJECT_ACE_TYPE
    );
    assert!(
        evaluate(&sd, 1).is_ok(),
        "ancestor inherit-only skipping precedes ACE dispatch"
    );
    assert!(evaluate(&sd, 0).is_err());
}

#[test]
fn missing_or_null_dacl_and_untrusted_private_owner_fail_closed() {
    for text in [
        format!("O:{CURRENT}"),
        format!("O:{CURRENT}D:NO_ACCESS_CONTROL"),
        "D:(A;;FA;;;SY)".into(),
    ] {
        let sd: LocalBox<SecurityDescriptor> = text.parse().unwrap();
        assert!(evaluate(&sd, 0).is_err());
    }
    assert!(evaluate(&descriptor(FOREIGN, ""), 0).is_err());
}

#[test]
fn rejection_explains_ancestor_depth_and_rights_without_exposing_identity_or_paths() {
    let sd = descriptor(CURRENT, &format!("(A;;0x00040000;;;{REPORTED})"));
    let error = evaluate(&sd, 4).unwrap_err();
    let text = error.to_string();
    assert_eq!(error.code(), "MIGRATION_FENCED");
    assert!(
        text.contains("ancestor") && text.contains("depth 4") && text.contains("0x00040000"),
        "{text}"
    );
    assert!(
        !text.contains(REPORTED)
            && !text.contains(CURRENT)
            && !text.contains("D:P")
            && !text.contains("C:\\")
    );
}

#[test]
fn private_directory_rejection_is_distinct_from_ancestor_replacement() {
    let sd = descriptor(CURRENT, &format!("(A;;FRFX;;;{FOREIGN})"));
    let error = evaluate(&sd, 0).unwrap_err();
    let text = error.to_string();
    assert!(
        text.contains("private directory") && text.contains("untrusted"),
        "{text}"
    );
    assert!(!text.contains(FOREIGN) && !text.contains(CURRENT));
}
