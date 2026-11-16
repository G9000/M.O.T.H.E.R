//! OS metadata checks for the owner-only migration and service trust boundary.
use crate::error::{Error, Result};
use std::{fs, path::Path};

fn denied() -> Error {
    Error::domain(
        "MIGRATION_FENCED",
        "Private endpoint ownership or file identity could not be established.",
    )
}

/// Reject existing file aliases which would evade path-bound cutover fences.
pub(crate) fn single_link(path: &Path) -> Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(denied()),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(denied());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(denied());
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x400 != 0 {
            return Err(denied());
        }
        let file = fs::File::open(path).map_err(|_| denied())?;
        if winapi_util::file::information(&file)
            .map_err(|_| denied())?
            .number_of_links()
            != 1
        {
            return Err(denied());
        }
    }
    Ok(())
}

/// Verify a private directory and that a broadly writable ancestor cannot replace it.
pub(crate) fn owner_directory(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path).map_err(|_| denied())?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(denied());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(denied());
        }
        for parent in path.ancestors().skip(1) {
            let m = fs::symlink_metadata(parent).map_err(|_| denied())?;
            let mode = m.permissions().mode();
            if m.file_type().is_symlink() || mode & 0o022 != 0 && mode & 0o1000 == 0 {
                return Err(denied());
            }
        }
    }
    #[cfg(windows)]
    {
        windows_directory(path)?;
    }
    Ok(())
}

#[cfg(windows)]
fn windows_directory(path: &Path) -> Result<()> {
    use std::os::windows::fs::MetadataExt;
    use windows_permissions::{
        LocalBox, Sid,
        constants::{AceFlags, AceType, SeObjectType, SecurityInformation},
    };
    let current = windows_permissions::utilities::current_process_sid().map_err(|_| denied())?;
    let owner = current.as_ref();
    let system: LocalBox<Sid> = "SY".parse().map_err(|_| denied())?;
    let admins: LocalBox<Sid> = "BA".parse().map_err(|_| denied())?;
    let creator: LocalBox<Sid> = "CO".parse().map_err(|_| denied())?;
    for (ordinal, directory) in path.ancestors().enumerate() {
        let attributes = fs::symlink_metadata(directory)
            .map_err(|_| denied())?
            .file_attributes();

        if attributes & 0x400 != 0 {
            return Err(denied());
        }
        let descriptor = windows_permissions::wrappers::GetNamedSecurityInfo(
            directory.as_os_str(),
            SeObjectType::SE_FILE_OBJECT,
            SecurityInformation::Owner | SecurityInformation::Dacl,
        )
        .map_err(|_| denied())?;
        if ordinal == 0
            && descriptor
                .owner()
                .is_none_or(|sid| sid != owner && sid != system.as_ref() && sid != admins.as_ref())
        {
            return Err(denied());
        }
        let acl = descriptor.dacl().ok_or_else(denied)?;
        for index in 0..acl.len() {
            let ace = acl.get_ace(index).ok_or_else(denied)?;
            if ordinal > 0 && ace.flags().contains(AceFlags::InheritOnly) {
                continue;
            }
            match ace.ace_type() {
                AceType::ACCESS_DENIED_ACE_TYPE => {}
                AceType::ACCESS_ALLOWED_ACE_TYPE => {
                    let sid = ace.sid().ok_or_else(denied)?;
                    if sid == creator.as_ref() && ace.flags().contains(AceFlags::InheritOnly) {
                        continue;
                    }
                    if sid == owner || sid == system.as_ref() || sid == admins.as_ref() {
                        continue;
                    }
                    // Packaged and sandboxed apps (Codex, Store apps) are granted their own app-container
                    // identity on profile folders such as AppData, and re-grant it while they run. Such a
                    // grant on an ancestor is tolerated; on the directory itself it still is not.
                    if ordinal > 0 && sandboxed_app(sid) {
                        continue;
                    }
                    // Reject child deletion, ancestor replacement, ownership/DACL changes and generic writes.
                    const REPLACE_BOUNDARY: u32 =
                        0x40 | 0x10000 | 0x40000 | 0x80000 | 0x10000000 | 0x40000000;
                    if ordinal == 0 || ace.mask().bits() & REPLACE_BOUNDARY != 0 {
                        return Err(denied());
                    }
                }
                _ => return Err(denied()),
            }
        }
    }
    Ok(())
}

/// One sandboxed app's own identity (S-1-15-2-… app container or S-1-15-3-… capability), not a user or group.
#[cfg(windows)]
fn sandboxed_app(sid: &windows_permissions::Sid) -> bool {
    const APP_PACKAGE_AUTHORITY: [u8; 6] = [0, 0, 0, 0, 0, 15];
    // (one package's own identity is a long hash; short forms such as S-1-15-2-1, "all application
    // packages", are groups that admit every app and stay refused)
    *sid.id_authority() == APP_PACKAGE_AUTHORITY
        && matches!(sid.sub_authority(0), Some(2 | 3))
        && sid.sub_authority_count() >= 8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_second_hard_link_is_never_a_separate_active_endpoint() {
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let source = dir.path().join("source");
        fs::write(&source, b"synthetic").unwrap();
        assert!(single_link(&source).is_ok());
        fs::hard_link(&source, dir.path().join("alias")).unwrap();
        assert_eq!(single_link(&source).unwrap_err().code(), "MIGRATION_FENCED");
    }
    #[test]
    fn owner_test_root_is_private() {
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        owner_directory(dir.path()).unwrap();
    }
    #[cfg(windows)]
    #[test]
    fn only_sandboxed_app_identities_are_tolerated() {
        use windows_permissions::{LocalBox, Sid};
        let is = |text: &str| sandboxed_app(&text.parse::<LocalBox<Sid>>().unwrap());
        assert!(is(
            "S-1-15-3-3557520199-3666692283-3112367039-3524159787-2791857073-3163583606-3692855932"
        ));
        assert!(is(
            "S-1-15-2-2968813833-811790644-2202111208-3784096404-1081847329-2708967783-1438471679"
        ));
        // (Windows' own "all application packages" group, and ordinary users and groups, are not)
        assert!(!is("S-1-15-2-1"));
        assert!(!is("S-1-15-3-1"));
        assert!(!is("S-1-5-32-545"));
        assert!(!is("S-1-1-0"));
    }
    #[cfg(windows)]
    #[test]
    fn an_ancestor_grant_to_a_sandboxed_app_changes_nothing_but_a_broad_one_fences() {
        let root =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let journal = root.path().join("journal");
        fs::create_dir(&journal).unwrap();
        let grant = |who: &str| {
            let out = std::process::Command::new("icacls")
                .arg(root.path())
                .args(["/grant", &format!("{who}:(F)")])
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stdout)
            );
        };
        // (this machine's own temp folder may already fail the check; a sandbox grant must not change that)
        let before = owner_directory(&journal).is_ok();
        grant("*S-1-15-3-1024-1-2-3-4-5-6-7");
        assert_eq!(owner_directory(&journal).is_ok(), before);
        grant("*S-1-5-32-545");
        assert_eq!(
            owner_directory(&journal).unwrap_err().code(),
            "MIGRATION_FENCED"
        );
    }
    #[cfg(unix)]
    #[test]
    fn broad_directory_permissions_do_not_admit_a_journal() {
        use std::os::unix::fs::PermissionsExt;
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o777)).unwrap();
        assert!(owner_directory(dir.path()).is_err());
    }
}
