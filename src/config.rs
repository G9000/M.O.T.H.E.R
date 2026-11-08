//! Private client configuration and bounded file access.
use crate::{
    contracts::{Auth, hex_digest, integer, valid_client},
    error::{Error, Result, invalid},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};
/// Existing v1 client credentials, with secret-redacting debug output.
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
    /// Bind calls to this configured client; identity never comes from a tool argument.
    pub fn auth(&self) -> Auth {
        Auth {
            client_id: self.client_id.clone(),
            token: self.token.clone(),
        }
    }
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
/// Load a v1 client config without printing its bearer token.
pub fn load_config(path: &Path) -> Result<ClientConfig> {
    let mut value = read_json(path, 16384)?;
    value["version"] = serde_json::json!(integer(&value, "version", 1, 1)?);
    let config: ClientConfig = serde_json::from_value(value)?;
    if config.version != 1
        || !config.database_path.is_absolute()
        || !valid_client(&config.client_id)
        || !hex_digest(&config.token)
    {
        return Err(invalid("Invalid client config"));
    }
    Ok(config)
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
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    Ok(options.open(path)?)
}
