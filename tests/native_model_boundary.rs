//! The native runtime has no network installer command. Its only download
//! dependency is confined to the bounded host-file import adapter.
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
}

/// `ureq` may only be used by `src/service/host_file.rs`, never by a model or general downloader.
#[test]
fn the_download_client_is_confined_to_the_host_file_adapter() {
    fn rust_files(dir: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                rust_files(&path, found);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    let users: Vec<_> = files
        .iter()
        .filter(|path| fs::read_to_string(path).unwrap().contains("ureq"))
        .map(|path| {
            path.strip_prefix(&src)
                .unwrap()
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/")
        })
        .collect();
    assert_eq!(users, ["service/host_file.rs"]);
    let manifest = include_str!("../Cargo.toml");
    let runtime = manifest.split("[dev-dependencies]").next().unwrap();
    let declared: Vec<_> = runtime
        .lines()
        .filter(|line| line.trim_start().starts_with("ureq ="))
        .collect();
    assert_eq!(declared.len(), 1);
    assert!(declared[0].contains("default-features = false"));
}
