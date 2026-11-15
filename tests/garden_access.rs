//! Synthetic checks for future graph authority helpers, not graph handlers.
use mother::{
    access::{AccessMode, ClientAccess},
    garden::access::GardenAccess,
};
use serde_json::json;

fn shared() -> ClientAccess {
    ClientAccess {
        mode: AccessMode::Shared,
        subject: Some("synthetic-agent".into()),
        records: vec![],
    }
}

#[test]
fn garden_policy_wire_shape_is_separate_and_strict() {
    assert_eq!(
        serde_json::to_value(GardenAccess::All {}).unwrap(),
        json!({"mode":"all"})
    );
    assert_eq!(
        serde_json::to_value(GardenAccess::default()).unwrap(),
        json!({"mode":"allowlist","objects":[]})
    );
    for policy in [
        json!({"mode":"all","objects":[]}),
        json!({"mode":"allowlist","records":[]}),
        json!({"mode":"allowlist","objects":[],"subject":"spoofed"}),
    ] {
        assert!(
            serde_json::from_value::<GardenAccess>(policy.clone()).is_err(),
            "policy must be rejected: {policy}"
        );
    }
    assert_eq!(
        serde_json::to_value(shared()).unwrap(),
        json!({"mode":"shared","subject":"synthetic-agent","records":[]})
    );
}
