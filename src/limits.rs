//! Bounded export capacity shared by mutation admission and restoration.
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
/// Largest supported credential-free portable JSON bundle.
pub const MAX_EXPORT_BYTES: usize = 64 * 1024 * 1024;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Usage {
    pub records: u64,
    pub operations: u64,
    pub payload_bytes: u64,
}
impl Usage {
    pub fn check(&self) -> Result<()> {
        let bound = self
            .payload_bytes
            .checked_mul(2)
            .and_then(|v| v.checked_add(self.operations.checked_mul(512)?))
            .and_then(|v| v.checked_add(self.records.checked_mul(128)?))
            .and_then(|v| v.checked_add(262144));
        if self.records > 10000
            || self.operations > 100000
            || bound.is_none_or(|n| n > MAX_EXPORT_BYTES as u64)
        {
            return Err(Error::domain(
                "RESOURCE_LIMIT",
                "Workspace reached its portable capacity. Export before archival or migration; this write was not committed.",
            ));
        }
        Ok(())
    }
}
