//! Private client configuration and bounded file access.
#[cfg(feature = "legacy-sqlite")]
use crate::{
    access::AccessMode,
    contracts::{hex_digest, integer, valid_client},
    store::Store,
};
use crate::{
    contracts::Auth,
    error::{Error, Result, invalid},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};
/// Private v1 legacy or v2 policy-aware credentials, with secret-redacting debug output.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClientConfig {
    pub version: u32,
    pub database_path: PathBuf,
    pub client_id: String,
    pub token: String,
}
impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("database_path", &self.database_path)
            .field("client_id", &self.client_id)
            .finish_non_exhaustive()
    }
}
impl ClientConfig {
    /// Authenticate current read access; v2 credentials also require an explicit policy.
    #[cfg(feature = "legacy-sqlite")]
    pub fn authorize(&self, store: &Store) -> Result<()> {
        if !matches!(self.version, 1 | 2) {
            return Err(invalid("Invalid client config version"));
        }
        let access = store.access_for(&self.auth())?;
        if self.version == 2 && access.mode == AccessMode::Legacy {
            return Err(Error::domain(
                "FORBIDDEN",
                "Policy-aware credentials require a current companion or shared policy.",
            ));
        }
        Ok(())
    }
    /// Bind calls to this configured client; identity never comes from a tool argument.
    pub fn auth(&self) -> Auth {
        Auth {
            client_id: self.client_id.clone(),
            token: self.token.clone(),
        }
    }
}
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
/// Load legacy credentials or authenticate policy-aware credentials without revealing secrets.
#[cfg(feature = "legacy-sqlite")]
pub fn load_config(path: &Path) -> Result<ClientConfig> {
    let mut value = read_json(path, 16384)?;
    value["version"] = serde_json::json!(integer(&value, "version", 1, 2)?);
    let config: ClientConfig = serde_json::from_value(value)?;
    if !matches!(config.version, 1 | 2)
        || !config.database_path.is_absolute()
        || !valid_client(&config.client_id)
        || !hex_digest(&config.token)
    {
        return Err(invalid("Invalid client config"));
    }
    if config.version == 2 {
        config.authorize(&Store::open(&config.database_path)?)?;
    }
    Ok(config)
}
/// Legacy credentials require a build with the SQLite compatibility feature.
#[cfg(not(feature = "legacy-sqlite"))]
pub fn load_config(_path: &Path) -> Result<ClientConfig> {
    Err(Error::domain(
        "LEGACY_UNAVAILABLE",
        "Legacy credentials require the legacy-sqlite feature; use native Garden credentials.",
    ))
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

#[cfg(all(test, not(feature = "legacy-sqlite")))]
mod tests {
    #[test]
    fn legacy_configuration_is_rejected_before_reading_or_opening_a_database() {
        let error =
            super::load_config(std::path::Path::new("synthetic-missing-config.json")).unwrap_err();
        assert_eq!(error.code(), "LEGACY_UNAVAILABLE");
    }
}
