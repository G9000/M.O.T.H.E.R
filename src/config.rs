//! Private client configuration and bounded file access.
use crate::error::{Error, Result, invalid};
use serde_json::Value;
use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};
/// Verify a credential output resides inside an owner-private directory.
///
/// # Errors
/// Rejects relative paths, aliases and directories another local principal can replace.
pub fn check_private_output(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        return Err(invalid("Private output must be absolute"));
    }
    crate::private_paths::owner_directory(
        path.parent()
            .ok_or_else(|| invalid("Private output needs a parent"))?,
    )?;
    crate::private_paths::single_link(path)
}
/// Read at most the specified number of bytes and parse a UTF-8 JSON file.
pub fn read_json(path: &Path, max: usize) -> Result<Value> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take(max as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > max {
        return Err(invalid("File exceeds the supported size limit"));
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| invalid("Expected UTF-8 JSON"))?;
    Ok(serde_json::from_str(text.trim_start_matches('\u{feff}'))?)
}
/// Select the existing private user data directory, independently of AnimaOS.
pub fn data_directory() -> Result<PathBuf> {
    if let Some(value) = std::env::var_os("MOTHER_HOME") {
        let path = PathBuf::from(value);
        if !path.is_absolute() {
            return Err(invalid("MOTHER_HOME must be absolute"));
        }
        return Ok(path);
    }
    if cfg!(windows)
        && let Some(value) = std::env::var_os("LOCALAPPDATA")
    {
        return Ok(PathBuf::from(value).join("MOTHER"));
    }
    std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
        .map(|p| PathBuf::from(p).join(".mother"))
        .ok_or_else(|| Error::domain("CONFIGURATION", "Cannot locate the private data directory"))
}
/// Exclusively create an owner-private file without replacing an existing path.
pub fn create_private_file(path: &Path) -> Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    Ok(options.open(path)?)
}
