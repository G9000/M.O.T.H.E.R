//! Streaming rollback stamp: canonical bytes only, never grants or disposable indexes.
use super::{GardenStore, META, PRINCIPALS, Principal, Result, decode, failure, storage_error};
use redb::{ReadableDatabase, ReadableTable, TableDefinition, TableHandle};
use sha2::{Digest, Sha256};

fn part(hash: &mut Sha256, bytes: &[u8]) {
    hash.update((bytes.len() as u64).to_be_bytes());
    hash.update(bytes);
}

impl GardenStore {
    /// Open an ordinary native endpoint and recheck its fence while retaining database ownership.
    /// # Errors
    /// Rejects inactive cutover destinations, aliases and unavailable stores.
    pub fn open_active(path: impl AsRef<std::path::Path>) -> crate::error::Result<Self> {
        let path = path.as_ref();
        crate::garden::migrate::check_destination(path)?;
        let store = Self::open(path).map_err(crate::garden::config::domain_error)?;
        crate::garden::migrate::check_destination(path)?;
        Ok(store)
    }
    pub(crate) fn migration_grant_exists(&self, client: &str) -> Result<bool> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        Ok(tx
            .open_table(PRINCIPALS)
            .map_err(storage_error)?
            .get(client)
            .map_err(storage_error)?
            .is_some())
    }
    pub(crate) fn verify_migration_grant(
        &self,
        auth: &crate::contracts::Auth,
        scopes: &[String],
        access: &crate::access::ClientAccess,
        garden: Option<&crate::garden::access::GardenAccess>,
    ) -> Result<()> {
        use subtle::ConstantTimeEq;
        let tx = self.db.begin_read().map_err(storage_error)?;
        let table = tx.open_table(PRINCIPALS).map_err(storage_error)?;
        let principal: Principal = decode(
            table
                .get(auth.client_id.as_str())
                .map_err(storage_error)?
                .ok_or_else(|| {
                    failure(
                        crate::garden::contracts::GardenErrorCode::Unauthorized,
                        "Missing reissued principal",
                    )
                })?
                .value(),
        )?;
        if !bool::from(
            principal
                .token_hash
                .as_bytes()
                .ct_eq(crate::contracts::hash(&auth.token).as_bytes()),
        ) || principal.scopes != scopes
            || &principal.access != access
            || principal.garden != garden.cloned().unwrap_or_default()
        {
            return Err(failure(
                crate::garden::contracts::GardenErrorCode::Forbidden,
                "Reissued credential restrictions do not match the migration manifest",
            ));
        }
        Ok(())
    }
    pub(crate) fn migration_fingerprint(&self) -> Result<String> {
        let tx = self.db.begin_read().map_err(storage_error)?;
        let present: std::collections::BTreeSet<_> = tx
            .list_tables()
            .map_err(storage_error)?
            .map(|table| table.name().to_owned())
            .collect();
        let mut hash = Sha256::new();
        for name in [
            "garden_metadata",
            "garden_current",
            "garden_receipts",
            "garden_inbox",
            "garden_review_operations",
            "garden_operation_families",
            "garden_blob_metadata",
            "garden_v1_meta",
            "garden_v1_records",
            "garden_v1_operations",
            "garden_v1_proposals",
            "garden_v1_inbox_operations",
            "garden_v1_pins",
            "garden_v1_assignments",
        ] {
            if !present.contains(name) {
                continue;
            }
            let definition: TableDefinition<&str, &[u8]> = TableDefinition::new(name);
            part(&mut hash, name.as_bytes());
            for row in tx
                .open_table(definition)
                .map_err(storage_error)?
                .iter()
                .map_err(storage_error)?
            {
                let (key, value) = row.map_err(storage_error)?;
                if name == META.name() {
                    if !matches!(key.value(), "state" | "retained_generations") {
                        continue;
                    }
                    if key.value() == "state" {
                        let mut state: serde_json::Value =
                            serde_json::from_slice(value.value()).map_err(storage_error)?;
                        state
                            .as_object_mut()
                            .ok_or_else(|| storage_error("invalid metadata"))?
                            .remove("epoch");
                        part(&mut hash, key.value().as_bytes());
                        part(
                            &mut hash,
                            &serde_json::to_vec(&state).map_err(storage_error)?,
                        );
                        continue;
                    }
                }
                part(&mut hash, key.value().as_bytes());
                part(&mut hash, value.value());
            }
        }
        for name in ["garden_history", "garden_blob_chunks"] {
            if !present.contains(name) {
                continue;
            }
            let definition: TableDefinition<(&str, u64), &[u8]> = TableDefinition::new(name);
            part(&mut hash, name.as_bytes());
            for row in tx
                .open_table(definition)
                .map_err(storage_error)?
                .iter()
                .map_err(storage_error)?
            {
                let (key, value) = row.map_err(storage_error)?;
                let (id, sequence) = key.value();
                part(&mut hash, id.as_bytes());
                part(&mut hash, &sequence.to_be_bytes());
                part(&mut hash, value.value());
            }
        }
        for name in ["garden_replacements", "garden_inbox_operations"] {
            if !present.contains(name) {
                continue;
            }
            let definition: TableDefinition<&str, &str> = TableDefinition::new(name);
            part(&mut hash, name.as_bytes());
            for row in tx
                .open_table(definition)
                .map_err(storage_error)?
                .iter()
                .map_err(storage_error)?
            {
                let (key, value) = row.map_err(storage_error)?;
                part(&mut hash, key.value().as_bytes());
                part(&mut hash, value.value().as_bytes());
            }
        }
        Ok(format!("{:x}", hash.finalize()))
    }
}
