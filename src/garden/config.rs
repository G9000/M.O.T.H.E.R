//! Explicit native credentials; graph and compatibility scopes remain independent.
use super::{
    contracts::{GardenError, GardenErrorCode},
    store::GardenStore,
};
use crate::{
    contracts::{Auth, hex_digest, valid_client},
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Owner-issued native credential, kept outside the checkout and never logged.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GardenClientConfig {
    pub version: u32,
    pub backend: String,
    pub database_path: PathBuf,
    pub client_id: String,
    pub token: String,
}
impl std::fmt::Debug for GardenClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GardenClientConfig")
            .field("database_path", &self.database_path)
            .field("client_id", &self.client_id)
            .finish_non_exhaustive()
    }
}
impl GardenClientConfig {
    /// Bind identity to the private configuration, never to tool arguments.
    pub fn auth(&self) -> Auth {
        Auth {
            client_id: self.client_id.clone(),
            token: self.token.clone(),
        }
    }
    /// Validate explicit format and current independent graph or v1 read authority.
    /// # Errors
    /// Rejects malformed configuration, revoked keys or absent supported scopes.
    pub fn authorize(&self, store: &GardenStore) -> Result<()> {
        self.validate()?;
        match store.authorize(&self.auth()) {
            Ok(()) => Ok(()),
            Err(error) if error.code == GardenErrorCode::Forbidden && store.v1_initialized()? => {
                store.v1_authorize(&self.auth(), "read")
            }
            Err(error) => Err(domain_error(error)),
        }
    }
    /// Validate configuration without opening or creating a database.
    /// # Errors
    /// Requires the native backend tag, absolute path and a well-formed credential.
    pub fn validate(&self) -> Result<()> {
        if self.version != 2
            || self.backend != "garden_redb"
            || !self.database_path.is_absolute()
            || !valid_client(&self.client_id)
            || !hex_digest(&self.token)
        {
            return Err(Error::domain(
                "INVALID_INPUT",
                "Explicit graph configuration is required.",
            ));
        }
        Ok(())
    }
}

/// Read a bounded explicit native configuration without converting existing credentials.
/// # Errors
/// Rejects unknown fields, other backends, oversized files and malformed identity.
pub fn load_garden_config(path: &Path) -> Result<GardenClientConfig> {
    let value = crate::config::read_json(path, 16_384)?;
    let config: GardenClientConfig = serde_json::from_value(value)?;
    config.validate()?;
    Ok(config)
}

/// Keep graph error codes stable across the existing owner CLI boundary.
pub fn domain_error(error: GardenError) -> Error {
    let code = match error.code {
        GardenErrorCode::InvalidInput => "INVALID_INPUT",
        GardenErrorCode::UnsupportedVersion => "UNSUPPORTED_VERSION",
        GardenErrorCode::UnsupportedExtension => "UNSUPPORTED_EXTENSION",
        GardenErrorCode::LimitExceeded => "LIMIT_EXCEEDED",
        GardenErrorCode::InvalidEndpoint => "INVALID_ENDPOINT",
        GardenErrorCode::Unauthorized => "UNAUTHORIZED",
        GardenErrorCode::Forbidden => "FORBIDDEN",
        GardenErrorCode::WorkspaceMismatch => "WORKSPACE_MISMATCH",
        GardenErrorCode::NotFound => "NOT_FOUND",
        GardenErrorCode::Conflict => "CONFLICT",
        GardenErrorCode::OperationReused => "OPERATION_REUSED",
        GardenErrorCode::GenerationMismatch => "GENERATION_MISMATCH",
        GardenErrorCode::RestartRequired => "RESTART_REQUIRED",
        GardenErrorCode::StoreUnavailable => "STORE_UNAVAILABLE",
    };
    Error::domain(code, error.message)
}
/// Select native storage only from the explicit private configuration backend tag.
/// # Errors
/// Rejects unknown backend tags before any database is opened.
pub fn uses_garden_backend(path: &Path) -> Result<bool> {
    let value = crate::config::read_json(path, 16_384)?;
    match value.get("backend") {
        None => Ok(false),
        Some(serde_json::Value::String(backend)) if backend == "garden_redb" => Ok(true),
        Some(_) => Err(Error::domain(
            "INVALID_INPUT",
            "Unknown explicit storage backend.",
        )),
    }
}
