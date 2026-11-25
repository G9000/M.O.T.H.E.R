//! Owner-granted trust policies. Authorization (a credential's scopes) decides what an
//! agent may do; a trust policy only decides which authorized media attachments still
//! need owner review. Policies are written only from owner surfaces (the live Garden
//! session or owner CLI), never through MCP or local IPC, and never add scopes.
use super::{GardenStore, PRINCIPALS};
use crate::error::{Error, Result, invalid};
use redb::{ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub(crate) const TRUST_POLICIES: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_trust_policies");

/// Upper bounds the owner can choose; defaults are deliberately small.
pub const MAX_COMMITS_PER_DAY: u32 = 500;
pub const MAX_BYTES_PER_DAY: u64 = 1024 * 1024 * 1024;
const DEFAULT_COMMITS_PER_DAY: u32 = 20;
const DEFAULT_BYTES_PER_DAY: u64 = 256 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustMode {
    /// Every canonical attachment waits for owner approval (the default).
    ManualReview,
    /// Routine attachments the owner asked for commit automatically within limits.
    TrustedCompanion,
    /// As trusted companion, and the agent may also run self-initiated backfill.
    AutonomousGardening,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustPolicy {
    pub version: u32,
    pub mode: TrustMode,
    /// Increases on every owner change; 0 means the implicit default.
    pub revision: u64,
    pub max_auto_commits_per_day: u32,
    pub max_auto_bytes_per_day: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Default for TrustPolicy {
    fn default() -> Self {
        Self {
            version: 1,
            mode: TrustMode::ManualReview,
            revision: 0,
            max_auto_commits_per_day: DEFAULT_COMMITS_PER_DAY,
            max_auto_bytes_per_day: DEFAULT_BYTES_PER_DAY,
            updated_at: None,
        }
    }
}

impl TrustPolicy {
    pub fn automatic(&self) -> bool {
        self.mode != TrustMode::ManualReview
    }
    /// The effective policy as an agent sees it in capability discovery.
    pub fn effective(&self, eligible: bool) -> Value {
        let automatic = eligible && self.automatic();
        json!({
            "mode": self.mode,
            "revision": self.revision,
            "routineAttachments": if automatic {"automatic"} else {"owner_review"},
            "autonomousBackfill": automatic && self.mode == TrustMode::AutonomousGardening,
            "limits": {"maxAutoCommitsPerDay": self.max_auto_commits_per_day,
                "maxAutoBytesPerDay": self.max_auto_bytes_per_day},
            "alwaysOwnerReview": ["association other than \"confirmed\"",
                "unstrippable embedded metadata", "self-initiated work outside autonomous_gardening",
                "daily limit reached", "credential without read and media:attach"],
            "agentsCannotChangeTrust": true,
        })
    }
}

fn scopes(principal: &Value) -> Vec<String> {
    principal["scopes"]
        .as_array()
        .map(|s| {
            s.iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn eligible(principal: &Value) -> bool {
    let scopes = scopes(principal);
    ["read", "media:attach"]
        .iter()
        .all(|needed| scopes.iter().any(|s| s == needed))
}

fn unavailable(error: impl std::fmt::Display) -> Error {
    Error::domain(
        "STORE_UNAVAILABLE",
        format!("Trust policy storage failed: {error}"),
    )
}

impl GardenStore {
    /// The owner's trust policy for one credential; absent means manual review.
    pub fn trust_policy(&self, client: &str) -> Result<TrustPolicy> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        let table = match tx.open_table(TRUST_POLICIES) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(TrustPolicy::default()),
            Err(error) => return Err(unavailable(error)),
        };
        match table.get(client).map_err(unavailable)? {
            Some(row) => Ok(serde_json::from_slice(row.value())?),
            None => Ok(TrustPolicy::default()),
        }
    }

    /// Whether a credential currently holds the authorization trust can relax review for.
    pub fn trust_eligible(&self, client: &str) -> Result<bool> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        let table = tx.open_table(PRINCIPALS).map_err(unavailable)?;
        let Some(row) = table.get(client).map_err(unavailable)? else {
            return Ok(false);
        };
        Ok(eligible(&serde_json::from_slice(row.value())?))
    }

    /// Owner-only: every credential with its authorization summary and trust policy.
    /// Token hashes are never read into the result.
    pub fn trust_overview(&self) -> Result<Value> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        let principals = tx.open_table(PRINCIPALS).map_err(unavailable)?;
        let mut agents = Vec::new();
        for row in principals.iter().map_err(unavailable)? {
            let (client, value) = row.map_err(unavailable)?;
            let client = client.value().to_owned();
            let principal: Value = serde_json::from_slice(value.value())?;
            let policy = self.trust_policy(&client)?;
            let ok = eligible(&principal);
            agents.push(json!({
                "clientId": client,
                "scopes": scopes(&principal),
                "visibility": principal["access"]["mode"],
                "graphAccess": principal["garden"]["mode"],
                "eligibleForTrust": ok,
                "policy": policy,
                "effective": policy.effective(ok),
            }));
        }
        Ok(
            json!({"agents": agents, "limits": {"maxAutoCommitsPerDay": MAX_COMMITS_PER_DAY,
            "maxAutoBytesPerDay": MAX_BYTES_PER_DAY}}),
        )
    }

    /// Owner-only: replace one credential's trust policy, guarded by its current revision.
    /// Raising trust requires the credential to already hold read and media:attach; it
    /// never grants scopes. Returning to manual review is always allowed.
    pub fn set_trust_policy(
        &self,
        client: &str,
        mode: TrustMode,
        max_auto_commits_per_day: u32,
        max_auto_bytes_per_day: u64,
        expected_revision: u64,
    ) -> Result<TrustPolicy> {
        if !(1..=MAX_COMMITS_PER_DAY).contains(&max_auto_commits_per_day)
            || !(1..=MAX_BYTES_PER_DAY).contains(&max_auto_bytes_per_day)
        {
            return Err(invalid("Trust limits are outside the supported range"));
        }
        let tx = self.db.begin_write().map_err(unavailable)?;
        let policy = {
            let principals = tx.open_table(PRINCIPALS).map_err(unavailable)?;
            let principal: Value = match principals.get(client).map_err(unavailable)? {
                Some(row) => serde_json::from_slice(row.value())?,
                None => return Err(Error::domain("NOT_FOUND", "Unknown credential")),
            };
            if mode != TrustMode::ManualReview && !eligible(&principal) {
                return Err(Error::domain(
                    "FORBIDDEN",
                    "Grant read and media:attach to this credential before trusting it.",
                ));
            }
            let mut table = tx.open_table(TRUST_POLICIES).map_err(unavailable)?;
            let current: TrustPolicy = match table.get(client).map_err(unavailable)? {
                Some(row) => serde_json::from_slice(row.value())?,
                None => TrustPolicy::default(),
            };
            if current.revision != expected_revision {
                return Err(Error::domain(
                    "CONFLICT",
                    "The trust policy changed. Refresh and review it again.",
                ));
            }
            let policy = TrustPolicy {
                version: 1,
                mode,
                revision: current.revision + 1,
                max_auto_commits_per_day,
                max_auto_bytes_per_day,
                updated_at: Some(self.clock.now().to_rfc3339()),
            };
            table
                .insert(client, serde_json::to_vec(&policy)?.as_slice())
                .map_err(unavailable)?;
            policy
        };
        tx.commit().map_err(unavailable)?;
        Ok(policy)
    }
}
