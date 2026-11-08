//! Stable domain errors shared by the CLI and MCP boundary.
use thiserror::Error;
/// A context operation failure; transport layers must not expose internal paths or secrets.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{code}: {message}")]
    Domain { code: &'static str, message: String },
    #[error("Storage operation failed")]
    Sqlite(#[from] rusqlite::Error),
    #[error("File operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid JSON data")]
    Json(#[from] serde_json::Error),
}
/// Result of a workspace operation.
pub type Result<T> = std::result::Result<T, Error>;
impl Error {
    /// Construct a failure with a stable machine-readable code.
    pub fn domain(code: &'static str, message: impl Into<String>) -> Self {
        Self::Domain {
            code,
            message: message.into(),
        }
    }
    /// Return the stable protocol error category.
    pub fn code(&self) -> &str {
        match self {
            Self::Domain { code, .. } => code,
            Self::Json(_) => "INVALID_INPUT",
            _ => "STORE_UNAVAILABLE",
        }
    }
}
pub(crate) fn invalid(message: impl Into<String>) -> Error {
    Error::domain("INVALID_INPUT", message)
}
