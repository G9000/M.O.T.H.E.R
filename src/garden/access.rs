//! Explicit owner graph grants and typed disclosure checks for future handlers.
//!
//! These helpers do not establish graph object existence or implement graph storage.
//! Handlers must obtain references from trusted stored objects, check every returned
//! object, endpoint and evidence reference, and serialize within the same transaction.
use super::contracts::ExternalReference;
use crate::{
    access::ClientAccess,
    contracts::Auth,
    error::{Error, Result, invalid},
    store::{self, Store},
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

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

fn table_exists(conn: &Connection, table: &str) -> Result<bool> {
    Ok(conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?)",
        [table],
        |r| r.get(0),
    )?)
}

pub(crate) fn load(conn: &Connection, client: &str) -> Result<GardenAccess> {
    let forbidden = || Error::domain("FORBIDDEN", "An explicit current graph policy is required.");
    if !table_exists(conn, "client_garden_policy_requirements")?
        || !table_exists(conn, "client_garden_access")?
    {
        return Err(forbidden());
    }
    let required: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM client_garden_policy_requirements WHERE client_id=?)",
        [client],
        |r| r.get(0),
    )?;
    if !required {
        return Err(forbidden());
    }
    let raw: Option<String> = conn
        .query_row(
            "SELECT policy FROM client_garden_access WHERE client_id=?",
            [client],
            |r| r.get(0),
        )
        .optional()?;
    let policy: GardenAccess = serde_json::from_str(&raw.ok_or_else(forbidden)?)?;
    policy.validate()?;
    if matches!(policy, GardenAccess::All {})
        && crate::access::load(conn, client)?.mode == crate::access::AccessMode::Shared
    {
        return Err(forbidden());
    }
    Ok(policy)
}

fn save(conn: &Connection, client: &str, policy: &GardenAccess) -> Result<()> {
    policy.validate()?;
    // Additive owner provisioning only. Opening a v1 store does not mutate its
    // schema version, generation, grants, or graph policy.
    conn.execute_batch("CREATE TABLE IF NOT EXISTS client_garden_policy_requirements(client_id TEXT PRIMARY KEY); CREATE TABLE IF NOT EXISTS client_garden_access(client_id TEXT PRIMARY KEY,policy TEXT NOT NULL)")?;
    conn.execute(
        "INSERT INTO client_garden_policy_requirements VALUES (?)",
        [client],
    )?;
    conn.execute(
        "INSERT INTO client_garden_access VALUES (?,?)",
        params![client, serde_json::to_string(policy)?],
    )?;
    Ok(())
}

fn current_workspace(conn: &Connection, workspace_id: &str, generation: &str) -> Result<()> {
    crate::contracts::id(workspace_id)?;
    crate::contracts::id(generation)?;
    let current = store::workspace(conn)?;
    if current["id"] != workspace_id {
        return Err(Error::domain(
            "WORKSPACE_MISMATCH",
            "Fetch the current workspace.",
        ));
    }
    if current["generation"] != generation {
        return Err(Error::domain(
            "GENERATION_MISMATCH",
            "Workspace was restored; reconnect and reconcile current objects.",
        ));
    }
    Ok(())
}

impl Store {
    /// Provision explicit graph scopes and an independent graph allowlist.
    /// This owner-only API validates ID syntax without claiming graph objects exist.
    pub fn grant_with_garden_access(
        &mut self,
        client: &str,
        scopes: &[String],
        access: &ClientAccess,
        garden: &GardenAccess,
    ) -> Result<Value> {
        if !scopes
            .iter()
            .any(|scope| crate::contracts::GARDEN_SCOPES.contains(&scope.as_str()))
        {
            return Err(invalid("A graph policy requires an explicit graph scope"));
        }
        garden.validate()?;
        if matches!(garden, GardenAccess::All {})
            && access.mode == crate::access::AccessMode::Shared
        {
            return Err(invalid(
                "Shared clients cannot receive full graph visibility",
            ));
        }
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut grant = store::grant(&tx, client, scopes)?;
        crate::access::save(&tx, client, scopes, access)?;
        save(&tx, client, garden)?;
        grant["access"] = access.metadata();
        grant["gardenAccess"] = json!(garden);
        tx.commit()?;
        Ok(grant)
    }

    /// Check current graph scope, generation, identity and a typed visibility
    /// footprint. Future handlers must repeat this inside their own transaction.
    pub fn authorize_garden(
        &mut self,
        auth: &Auth,
        action: GardenAction,
        workspace_id: &str,
        generation: &str,
        objects: &[GardenObjectVisibility],
    ) -> Result<GardenAuthority> {
        let tx = self.conn.transaction()?;
        store::authorize(&tx, auth, action.scope())?;
        current_workspace(&tx, workspace_id, generation)?;
        load(&tx, &auth.client_id)?.require_visible(objects)?;
        let authority = GardenAuthority {
            subject: crate::access::load(&tx, &auth.client_id)?.subject,
        };
        tx.commit()?;
        Ok(authority)
    }

    /// Determine receipt disclosure using the current original-operation scope
    /// and identity. Approval never appends result IDs to either allowlist.
    /// A successful own-proposal result permits status and result IDs only when
    /// `payload_visible` is false. Other receipts require complete visibility.
    pub fn garden_receipt_visibility(
        &mut self,
        auth: &Auth,
        origin: &GardenOperationIdentity,
        objects: &[GardenObjectVisibility],
    ) -> Result<GardenReceiptVisibility> {
        let tx = self.conn.transaction()?;
        store::authorize(&tx, auth, origin.action.scope())?;
        current_workspace(&tx, &origin.workspace_id, &origin.generation)?;
        let identity = crate::access::load(&tx, &auth.client_id)?;
        if origin.actor != auth.client_id || origin.subject != identity.subject {
            return Err(Error::domain(
                "NOT_FOUND",
                "Graph operation does not exist.",
            ));
        }
        let payload_visible = match load(&tx, &auth.client_id)?.require_visible(objects) {
            Ok(()) => true,
            Err(error) if error.code() == "NOT_FOUND" && origin.action == GardenAction::Propose => {
                false
            }
            Err(error) => return Err(error),
        };
        tx.commit()?;
        Ok(GardenReceiptVisibility { payload_visible })
    }
}
