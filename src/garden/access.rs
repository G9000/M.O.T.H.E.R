//! Explicit owner graph grants and typed disclosure checks for future handlers.
//!
//! These helpers do not establish graph object existence or implement graph storage.
//! Handlers must obtain references from trusted stored objects, check every returned
//! object, endpoint and evidence reference, and serialize within the same transaction.
use super::contracts::ExternalReference;
use crate::error::{Error, Result, invalid};
use serde::{Deserialize, Serialize};

/// Owner-declared graph object allowlist, independent of canonical record access.
/// An empty default grants no objects. Full access is a separate explicit owner
/// choice, permitted only for companion and legacy policies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum GardenAccess {
    All {},
    Allowlist { objects: Vec<ExternalReference> },
}
impl Default for GardenAccess {
    fn default() -> Self {
        Self::Allowlist { objects: vec![] }
    }
}
impl GardenAccess {
    pub(crate) fn validate(&self) -> Result<()> {
        let Self::Allowlist { objects } = self else {
            return Ok(());
        };
        if objects.len() > 10000 {
            return Err(invalid("Too many graph object IDs"));
        }
        for object in objects {
            object.validate().map_err(|error| invalid(error.message))?;
        }
        Ok(())
    }
    pub(crate) fn allows(&self, object: &ExternalReference) -> bool {
        match self {
            Self::All {} => true,
            Self::Allowlist { objects } => objects.iter().any(|entry| entry == object),
        }
    }
    pub(crate) fn require_visible(&self, objects: &[GardenObjectVisibility]) -> Result<()> {
        for object in objects {
            for reference in std::iter::once(&object.object)
                .chain(&object.endpoints)
                .chain(&object.evidence)
            {
                reference
                    .validate()
                    .map_err(|error| invalid(error.message))?;
                if !self.allows(reference) {
                    return Err(Error::domain("NOT_FOUND", "Graph object does not exist."));
                }
            }
        }
        Ok(())
    }
}

/// Independently scoped graph operation. Canonical scopes never imply these scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GardenAction {
    Read,
    Propose,
    Write,
}
impl GardenAction {
    pub fn scope(self) -> &'static str {
        match self {
            Self::Read => "garden:read",
            Self::Propose => "garden:propose",
            Self::Write => "garden:write",
        }
    }
}

/// Visibility footprint of a trusted graph object, including all linked IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GardenObjectVisibility {
    pub object: ExternalReference,
    pub endpoints: Vec<ExternalReference>,
    pub evidence: Vec<ExternalReference>,
}

/// Snapshot of a checked identity. It is not an enduring authorization capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GardenAuthority {
    subject: Option<String>,
}
impl GardenAuthority {
    /// External subject selected by the owner, never supplied by an MCP request.
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }
}

/// Identity and original scope from a trusted stored operation or proposal.
/// Future receipt handlers must populate this from storage, not request arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GardenOperationIdentity {
    pub workspace_id: String,
    pub generation: String,
    pub actor: String,
    pub subject: Option<String>,
    pub action: GardenAction,
}

/// Own proposal status and result IDs may be disclosed after approval. The payload
/// remains hidden unless all its objects and references are currently visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GardenReceiptVisibility {
    pub payload_visible: bool,
}
