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

/// Give a directory this process has just exclusively created an owner-only DACL.
///
/// Windows has no creation mode, so a new directory inherits its parent's
/// entries; a harmless ancestor read grant would then fail `owner_directory`.
/// This mirrors Unix `0o700` and never runs on a directory that already existed.
#[cfg(windows)]
pub(crate) fn restrict_new_directory(path: &Path) -> Result<()> {
    use windows_permissions::{
        LocalBox, SecurityDescriptor,
        constants::{SeObjectType, SecurityInformation},
    };
    let current = windows_permissions::utilities::current_process_sid().map_err(|_| denied())?;
    let descriptor: LocalBox<SecurityDescriptor> =
        format!("D:P(A;OICI;FA;;;{current})(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)")
            .parse()
            .map_err(|_| denied())?;
    windows_permissions::wrappers::SetNamedSecurityInfo(
        path.as_os_str(),
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
        None,
        None,
        Some(descriptor.dacl().ok_or_else(denied)?),
        None,
    )
    .map_err(|_| denied())
}
/// Unix creation already applies `0o700`.
#[cfg(not(windows))]
pub(crate) fn restrict_new_directory(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(windows)]
fn windows_directory(path: &Path) -> Result<()> {
    use std::os::windows::fs::MetadataExt;
    use windows_permissions::{
        LocalBox, Sid,
        constants::{SeObjectType, SecurityInformation},
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
        windows_descriptor(
            &descriptor,
            &[owner, system.as_ref(), admins.as_ref()],
            creator.as_ref(),
            ordinal,
        )?;
    }
    Ok(())
}

#[cfg(windows)]
fn windows_descriptor(
    descriptor: &windows_permissions::SecurityDescriptor,
    trusted: &[&windows_permissions::Sid; 3],
    creator: &windows_permissions::Sid,
    ordinal: usize,
) -> Result<()> {
    use windows_permissions::constants::{AceFlags, AceType, SecurityInformation};
    if ordinal == 0 && descriptor.owner().is_none_or(|sid| !trusted.contains(&sid)) {
        return Err(windows_acl_denied(
            ordinal,
            "the owner is not a trusted principal",
            None,
        ));
    }
    // windows-permissions 0.2.4 panics on a present NULL DACL. A read-only
    // conversion either rejects it or emits NO_ACCESS_CONTROL, before we borrow
    // the ACL. Missing, malformed and unrestricted descriptors all fail closed.
    let text = windows_permissions::wrappers::ConvertSecurityDescriptorToStringSecurityDescriptor(
        descriptor,
        SecurityInformation::Dacl,
    )
    .map_err(|_| windows_acl_denied(ordinal, "the DACL is missing, null or unreadable", None))?;
    if text
        .to_str()
        .is_none_or(|value| !value.starts_with("D:") || value.contains("NO_ACCESS_CONTROL"))
    {
        return Err(windows_acl_denied(
            ordinal,
            "the DACL is missing, null or unreadable",
            None,
        ));
    }
    let acl = descriptor.dacl().ok_or_else(|| {
        windows_acl_denied(ordinal, "the DACL is missing, null or unreadable", None)
    })?;
    for index in 0..acl.len() {
        let ace = acl.get_ace(index).ok_or_else(denied)?;
        if ordinal > 0 && ace.flags().contains(AceFlags::InheritOnly) {
            continue;
        }
        match ace.ace_type() {
            AceType::ACCESS_DENIED_ACE_TYPE => {}
            AceType::ACCESS_ALLOWED_ACE_TYPE => {
                let sid = ace.sid().ok_or_else(denied)?;
                if sid == creator && ace.flags().contains(AceFlags::InheritOnly) {
                    continue;
                }
                if trusted.contains(&sid) {
                    continue;
                }
                // Reject child deletion, ancestor replacement, ownership/DACL changes and generic writes.
                const REPLACE_BOUNDARY: u32 =
                    0x40 | 0x10000 | 0x40000 | 0x80000 | 0x10000000 | 0x40000000;
                if ordinal == 0 || ace.mask().bits() & REPLACE_BOUNDARY != 0 {
                    let reason = if ordinal == 0 {
                        "an allow entry names an untrusted identity in an owner-only private directory"
                    } else {
                        "an untrusted allow entry grants unsafe write or directory-replacement/control rights"
                    };
                    return Err(windows_acl_denied(ordinal, reason, Some(ace.mask().bits())));
                }
            }
            _ => {
                return Err(windows_acl_denied(
                    ordinal,
                    "an unsupported access-control entry cannot be validated",
                    None,
                ));
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn windows_acl_denied(depth: usize, reason: &str, mask: Option<u32>) -> Error {
    let location = if depth == 0 {
        "private directory".to_owned()
    } else {
        format!("ancestor directory (depth {depth})")
    };
    let rights = mask
        .map(|value| format!(" (access mask {value:#010x})"))
        .unwrap_or_default();
    // Shared domain errors must never echo private paths, SIDs or descriptors.
    Error::domain(
        "MIGRATION_FENCED",
        format!(
            "The {location} failed its security check: {reason}{rights}. Inspect its ACL; no automatic ACL repair is performed."
        ),
    )
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

#[cfg(all(test, windows))]
#[path = "private_paths/windows_tests.rs"]
mod windows_tests;
