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
