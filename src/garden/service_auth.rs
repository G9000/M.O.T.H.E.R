//! Current-principal owner proofs for authenticating a local transport before credential transfer.
use super::{GardenStore, PRINCIPALS, Principal, Result, decode, failure, storage_error};
use crate::garden::contracts::GardenErrorCode;
use hmac::{Hmac, Mac};
use redb::ReadableDatabase;
use sha2::Sha256;

/// Protocol-specific domain separation prevents reusing proofs in other keyed contexts.
const PROOF_LABEL: &[u8] = b"MOTHER local IPC owner proof v1\0";

pub(crate) fn proof_mac(token_hash: &str, context: &[u8]) -> Result<[u8; 32]> {
    if !crate::contracts::hex_digest(token_hash) || context.len() > 4096 {
        return Err(failure(
            GardenErrorCode::InvalidInput,
            "Invalid local service proof context",
        ));
    }
    let mut mac = Hmac::<Sha256>::new_from_slice(token_hash.as_bytes()).map_err(storage_error)?;
    mac.update(PROOF_LABEL);
    mac.update(context);
    Ok(mac.finalize().into_bytes().into())
}

impl GardenStore {
    /// Prove possession of a current principal's stored key without exposing that key.
    ///
    /// This trusted owner API authenticates transport ownership only. Ordinary credentials,
    /// workspace identity and tool scopes must still be checked inside each domain operation.
    /// # Errors
    /// Rejects unknown or revoked principals and oversized challenge contexts.
    pub fn local_service_proof(&self, client_id: &str, context: &[u8]) -> Result<[u8; 32]> {
        if !crate::contracts::valid_client(client_id) || context.len() > 4096 {
            return Err(failure(
                GardenErrorCode::InvalidInput,
                "Invalid local service proof context",
            ));
        }
        let tx = self.db.begin_read().map_err(storage_error)?;
        let table = tx.open_table(PRINCIPALS).map_err(storage_error)?;
        let principal = table
            .get(client_id)
            .map_err(storage_error)?
            .ok_or_else(|| {
                failure(
                    GardenErrorCode::Unauthorized,
                    "Invalid local service credential",
                )
            })?;
        let principal: Principal = decode(principal.value())?;
        proof_mac(&principal.token_hash, context)
    }
}
