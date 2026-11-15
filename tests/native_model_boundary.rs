//! The native runtime has no network installer command or download dependency.
use std::{fs, process::Command};
#[test]
fn native_install_is_unavailable_without_touching_the_destination() {
    let root = tempfile::tempdir().unwrap();
    let parent = root.path().join("private-model-parent");
    let original = b"synthetic existing owner data";
    fs::write(&parent, original).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mother"))
        .args(["model", "install"])
        .env("MOTHER_HOME", root.path())
        .env("MOTHER_MODEL_DIR", parent.join("model"))
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(fs::read(&parent).unwrap(), original);
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    assert!(
        !include_str!("../Cargo.toml")
            .split("[dev-dependencies]")
            .next()
            .unwrap()
            .lines()
            .any(|line| line.trim_start().starts_with("ureq ="))
    );
}
