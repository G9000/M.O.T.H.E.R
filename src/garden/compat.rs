//! Native v1 compatibility records, credentials and receipts in the Garden transaction.
#[path = "compat_search.rs"]
pub(crate) mod learned;
#[path = "compat_view.rs"]
pub(crate) mod view;
use super::{
    CURRENT, GardenStore, PRINCIPALS, Principal, bump_epoch, inbox, operation_key, state_write,
};
use crate::{
    access::{AccessMode, ClientAccess},
    contracts::{self, Auth, Kind, Write, integer, string},
    error::{Error, Result, invalid},
    garden::access::GardenAccess,
};
use chrono::SecondsFormat;
use rand::RngCore;
use redb::{
    Durability, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition,
    TableHandle,
};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
use subtle::ConstantTimeEq;
use uuid::Uuid;

pub(crate) const V1_META: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_meta");
const RECORDS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_records");
const OPERATIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_operations");
const PROPOSALS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_proposals");
const INBOX_OPERATIONS: TableDefinition<&str, &[u8]> =
    TableDefinition::new("garden_v1_inbox_operations");
const PINS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_pins");
const ASSIGNMENTS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_assignments");
// Rebuildable lookup: proposal IDs carry no admission UUID in the v1 payload.
const ADMISSIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_admissions");
const PENDING: TableDefinition<&str, &[u8]> = TableDefinition::new("garden_v1_pending_dedup");
const TABLES: [TableDefinition<&str, &[u8]>; 7] = [
    V1_META,
    RECORDS,
    OPERATIONS,
    PROPOSALS,
    INBOX_OPERATIONS,
    PINS,
    ASSIGNMENTS,
];

pub(crate) fn backup_tables() -> &'static [TableDefinition<'static, &'static str, &'static [u8]>] {
    &TABLES
}
fn unavailable(_: impl std::fmt::Display) -> Error {
    Error::domain(
        "STORE_UNAVAILABLE",
        "Native storage unavailable; reconcile the original operation UUID if a write was attempted.",
    )
}
fn graph(error: super::GardenError) -> Error {
    crate::garden::config::domain_error(error)
}
fn not_found() -> Error {
    Error::domain("NOT_FOUND", "Record or operation does not exist.")
}
fn reused() -> Error {
    Error::domain(
        "OPERATION_REUSED",
        "An operation ID cannot be reused for another request or operation family.",
    )
}

trait View {
    fn keys(&self, table: TableDefinition<&str, &[u8]>) -> Result<Vec<String>>;
    fn get(&self, table: TableDefinition<&str, &[u8]>, key: &str) -> Result<Option<Value>>;
    fn values(&self, table: TableDefinition<&str, &[u8]>) -> Result<Vec<Value>>;
}
impl View for redb::ReadTransaction {
    fn keys(&self, table: TableDefinition<&str, &[u8]>) -> Result<Vec<String>> {
        self.open_table(table)
            .map_err(unavailable)?
            .iter()
            .map_err(unavailable)?
            .map(|entry| {
                let (k, _) = entry.map_err(unavailable)?;
                Ok(k.value().to_owned())
            })
            .collect()
    }
    fn get(&self, table: TableDefinition<&str, &[u8]>, key: &str) -> Result<Option<Value>> {
        Ok(self
            .open_table(table)
            .map_err(unavailable)?
            .get(key)
            .map_err(unavailable)?
            .map(|v| serde_json::from_slice(v.value()))
            .transpose()?)
    }
    fn values(&self, table: TableDefinition<&str, &[u8]>) -> Result<Vec<Value>> {
        self.open_table(table)
            .map_err(unavailable)?
            .iter()
            .map_err(unavailable)?
            .map(|entry| {
                let (_, v) = entry.map_err(unavailable)?;
                Ok(serde_json::from_slice(v.value())?)
            })
            .collect()
    }
}
impl View for redb::WriteTransaction {
    fn keys(&self, table: TableDefinition<&str, &[u8]>) -> Result<Vec<String>> {
        self.open_table(table)
            .map_err(unavailable)?
            .iter()
            .map_err(unavailable)?
            .map(|entry| {
                let (k, _) = entry.map_err(unavailable)?;
                Ok(k.value().to_owned())
            })
            .collect()
    }
    fn get(&self, table: TableDefinition<&str, &[u8]>, key: &str) -> Result<Option<Value>> {
        Ok(self
            .open_table(table)
            .map_err(unavailable)?
            .get(key)
            .map_err(unavailable)?
            .map(|v| serde_json::from_slice(v.value()))
            .transpose()?)
    }
    fn values(&self, table: TableDefinition<&str, &[u8]>) -> Result<Vec<Value>> {
        self.open_table(table)
            .map_err(unavailable)?
            .iter()
            .map_err(unavailable)?
            .map(|entry| {
                let (_, v) = entry.map_err(unavailable)?;
                Ok(serde_json::from_slice(v.value())?)
            })
            .collect()
    }
}
fn put(
    tx: &redb::WriteTransaction,
    table: TableDefinition<&str, &[u8]>,
    key: &str,
    value: &Value,
) -> Result<()> {
    tx.open_table(table)
        .map_err(unavailable)?
        .insert(key, serde_json::to_vec(value)?.as_slice())
        .map_err(unavailable)?;
    Ok(())
}
fn remove(
    tx: &redb::WriteTransaction,
    table: TableDefinition<&str, &[u8]>,
    key: &str,
) -> Result<()> {
    tx.open_table(table)
        .map_err(unavailable)?
        .remove(key)
        .map_err(unavailable)?;
    Ok(())
}
fn initialize_tables(tx: &redb::WriteTransaction) -> Result<()> {
    for table in TABLES {
        tx.open_table(table).map_err(unavailable)?;
    }
    tx.open_table(ADMISSIONS).map_err(unavailable)?;
    tx.open_table(PENDING).map_err(unavailable)?;
    Ok(())
}
fn metadata(tx: &impl View, key: &str) -> Result<Value> {
    tx.get(V1_META, key)?.ok_or_else(|| {
        Error::domain(
            "COMPAT_NOT_INITIALIZED",
            "The owner has not enabled the native v1 compatibility view.",
        )
    })
}
fn record(tx: &impl View, id: &str) -> Result<Value> {
    tx.get(RECORDS, id)?.ok_or_else(not_found)
}
fn principal(tx: &impl View, auth: &Auth, scope: &str) -> Result<Principal> {
    let value = tx.get(PRINCIPALS, &auth.client_id)?.ok_or_else(|| {
        Error::domain(
            "UNAUTHORIZED",
            "Client is unknown, revoked, or has an invalid token.",
        )
    })?;
    let p: Principal = serde_json::from_value(value)?;
    if !bool::from(
        p.token_hash
            .as_bytes()
            .ct_eq(contracts::hash(&auth.token).as_bytes()),
    ) {
        return Err(Error::domain(
            "UNAUTHORIZED",
            "Client is unknown, revoked, or has an invalid token.",
        ));
    }
    p.access.validate()?;
    p.garden.validate()?;
    if !p.scopes.iter().any(|s| s == scope) {
        return Err(Error::domain(
            "FORBIDDEN",
            format!("This client needs {scope}."),
        ));
    }
    if p.access.mode == AccessMode::Shared
        && !matches!(
            scope,
            "read" | "context:propose" | "garden:read" | "garden:propose"
        )
    {
        return Err(Error::domain(
            "FORBIDDEN",
            "Shared clients cannot write canonical records.",
        ));
    }
    Ok(p)
}
fn identity(tx: &impl View, v: &Value) -> Result<Value> {
    let w = metadata(tx, "workspace")?;
    if v["workspaceId"] != w["id"] {
        return Err(Error::domain(
            "WORKSPACE_MISMATCH",
            "Fetch the current workspace.",
        ));
    }
    if v["generation"] != w["generation"] {
        return Err(Error::domain(
            "STALE_GENERATION",
            "Workspace was restored; reconnect and reconcile current records.",
        ));
    }
    Ok(w)
}
fn required_id<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    contracts::strict(v, &[key])?;
    let id = string(v, key)?;
    contracts::id(id)?;
    Ok(id)
}
fn claim(tx: &redb::WriteTransaction, actor: &str, v: &Value, family: &str) -> Result<()> {
    inbox::claim_operation(
        tx,
        string(v, "generation")?,
        string(v, "operationId")?,
        actor,
        family,
    )
    .map_err(graph)
}
fn check_family(tx: &redb::WriteTransaction, actor: &str, v: &Value, family: &str) -> Result<()> {
    inbox::check_operation(
        tx,
        string(v, "generation")?,
        string(v, "operationId")?,
        actor,
        family,
    )
    .map_err(graph)
}
fn key(v: &Value, actor: &str) -> Result<String> {
    Ok(operation_key(
        string(v, "generation")?,
        actor,
        string(v, "operationId")?,
    ))
}
fn prior(
    tx: &redb::WriteTransaction,
    actor: &str,
    v: &Value,
    family: &str,
) -> Result<Option<Value>> {
    check_family(tx, actor, v, family)?;
    if let Some(op) = tx.get(INBOX_OPERATIONS, &key(v, actor)?)? {
        if op["family"] != family.trim_start_matches("v1_")
            || op["requestHash"] != contracts::hash(&contracts::canonical(v))
        {
            return Err(reused());
        }
        return Ok(Some(op["receipt"].clone()));
    }
    if tx.get(OPERATIONS, &key(v, actor)?)?.is_some() {
        return Err(reused());
    }
    Ok(None)
}
fn references(tx: &impl View, draft: &Value) -> Result<()> {
    for id in draft["references"]
        .as_array()
        .ok_or_else(|| invalid("Invalid references"))?
    {
        if tx
            .get(
                RECORDS,
                id.as_str().ok_or_else(|| invalid("Invalid reference"))?,
            )?
            .is_none()
        {
            return Err(Error::domain(
                "REFERENCE_NOT_FOUND",
                "Referenced record does not exist in this workspace.",
            ));
        }
    }
    Ok(())
}

impl GardenStore {
    /// Whether the owner explicitly enabled this native v1 compatibility view.
    pub fn v1_initialized(&self) -> Result<bool> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        match tx.open_table(V1_META) {
            Ok(table) => Ok(table.get("workspace").map_err(unavailable)?.is_some()),
            Err(redb::TableError::TableDoesNotExist(_)) => Ok(false),
            Err(e) => Err(unavailable(e)),
        }
    }
    /// Enable v1 with an explicit owner persona, using the existing Garden identity.
    pub fn v1_initialize(&self, soul: &str) -> Result<Value> {
        let persona = json!({"content":soul,"hash":contracts::hash(soul)});
        contracts::validate_persona(&persona)?;
        let mut tx = self.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        initialize_tables(&tx)?;
        if tx.get(V1_META, "workspace")?.is_some() {
            return Err(Error::domain(
                "CONFLICT",
                "Native v1 compatibility is already initialized.",
            ));
        }
        let state = state_write(&tx).map_err(graph)?;
        let w = json!({"id":state.workspace_id,"generation":state.generation,"schemaVersion":1,"createdAt":self.clock.now().to_rfc3339_opts(SecondsFormat::Millis,true)});
        put(&tx, V1_META, "workspace", &w)?;
        put(&tx, V1_META, "persona", &persona)?;
        tx.commit().map_err(unavailable)?;
        Ok(w)
    }
    /// Validate current native v1 scope and visibility policy without widening graph grants.
    pub fn v1_authorize(&self, auth: &Auth, scope: &str) -> Result<()> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        metadata(&tx, "workspace")?;
        principal(&tx, auth, scope)?;
        Ok(())
    }
    /// Read a policy under the currently authenticated native v1 credential.
    pub fn v1_access_for(&self, auth: &Auth) -> Result<ClientAccess> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        metadata(&tx, "workspace")?;
        Ok(principal(&tx, auth, "read")?.access)
    }
    /// Discover native identity without scanning canonical records or returning owner persona.
    pub fn v1_identity(&self, auth: &Auth) -> Result<Value> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        let w = metadata(&tx, "workspace")?;
        principal(&tx, auth, "read")?;
        Ok(json!({"workspaceId":w["id"],"generation":w["generation"]}))
    }
    /// Explicitly provision native v1 authority and optional independent graph authority.
    pub fn v1_grant(
        &self,
        client: &str,
        scopes: &[String],
        access: &ClientAccess,
        garden: Option<&GardenAccess>,
    ) -> Result<Auth> {
        let (auth, p) = prepare_grant(client, scopes, access, garden)?;
        let mut tx = self.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        metadata(&tx, "workspace")?;
        put(&tx, PRINCIPALS, client, &serde_json::to_value(p)?)?;
        bump_epoch(&tx).map_err(graph)?;
        tx.commit().map_err(unavailable)?;
        Ok(auth)
    }
    /// Write an explicit private native credential and its grant with one transaction.
    #[allow(clippy::too_many_arguments)]
    pub fn v1_grant_to_file_with_access(
        &self,
        client: &str,
        scopes: &[String],
        path: &Path,
        database_path: &Path,
        access: &ClientAccess,
        garden: Option<&GardenAccess>,
    ) -> Result<()> {
        use std::io::Write as _;
        if !database_path.is_absolute() {
            return Err(invalid("Database path must be absolute"));
        }
        let (auth, p) = prepare_grant(client, scopes, access, garden)?;
        let config = json!({"version":2,"backend":"garden_redb","databasePath":database_path,"clientId":auth.client_id,"token":auth.token});
        let mut tx = self.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        metadata(&tx, "workspace")?;
        let mut file = crate::config::create_private_file(path)?;
        let staged = (|| -> Result<()> {
            put(&tx, PRINCIPALS, client, &serde_json::to_value(p)?)?;
            bump_epoch(&tx).map_err(graph)?;
            serde_json::to_writer_pretty(&mut file, &config)?;
            file.write_all(b"\n")?;
            file.sync_all()?;
            Ok(())
        })();
        drop(file);
        if let Err(e) = staged {
            drop(tx);
            let _ = std::fs::remove_file(path);
            return Err(e);
        }
        tx.commit().map_err(unavailable)
    }
    /// Losslessly import a validated small v1 portable bundle into an empty compatibility view.
    pub fn import_v1_bundle(&self, raw: Value) -> Result<()> {
        let bundle = crate::portability::validate(raw)?;
        let mut tx = self.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        initialize_tables(&tx)?;
        if tx.get(V1_META, "workspace")?.is_some() {
            return Err(Error::domain(
                "CONFLICT",
                "Compatibility view already exists.",
            ));
        }
        let state = state_write(&tx).map_err(graph)?;
        if bundle["workspace"]["id"] != state.workspace_id
            || bundle["workspace"]["generation"] != state.generation
        {
            return Err(Error::domain(
                "WORKSPACE_MISMATCH",
                "Import requires the same explicit workspace and generation.",
            ));
        }
        for name in ["workspace", "persona"] {
            put(&tx, V1_META, name, &bundle[name])?;
        }
        for (name, table, id_field) in [
            ("records", RECORDS, "id"),
            ("pins", PINS, "recordId"),
            ("assignments", ASSIGNMENTS, "recordId"),
        ] {
            if let Some(rows) = bundle[name].as_array() {
                for row in rows {
                    put(&tx, table, string(row, id_field)?, row)?;
                }
                if name != "records" {
                    put(&tx, V1_META, name, &json!(true))?;
                }
            }
        }
        for op in bundle["operations"]
            .as_array()
            .ok_or_else(|| invalid("Invalid operations"))?
        {
            put(&tx, OPERATIONS, &key(op, string(op, "actor")?)?, op)?;
        }
        if let Some(extension) = bundle.get("inbox") {
            put(&tx, V1_META, "inbox", &json!(true))?;
            for p in extension["proposals"]
                .as_array()
                .ok_or_else(|| invalid("Invalid proposals"))?
            {
                put(&tx, PROPOSALS, string(p, "id")?, p)?;
            }
            for op in extension["operations"]
                .as_array()
                .ok_or_else(|| invalid("Invalid inbox operations"))?
            {
                put(&tx, INBOX_OPERATIONS, &key(op, string(op, "actor")?)?, op)?;
            }
        }
        // The explicit validated import admits retained historical generations; ordinary writes never do.
        let mut generations: BTreeSet<String> = tx
            .get(super::META, "retained_generations")?
            .map(serde_json::from_value)
            .transpose()?
            .unwrap_or_default();
        for row in bundle["operations"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(
                bundle["inbox"]["operations"]
                    .as_array()
                    .into_iter()
                    .flatten(),
            )
            .chain(
                bundle["inbox"]["proposals"]
                    .as_array()
                    .into_iter()
                    .flatten(),
            )
        {
            let generation = string(row, "generation")?;
            if generation != state.generation {
                generations.insert(generation.to_owned());
            }
        }
        if !generations.is_empty() {
            put(
                &tx,
                super::META,
                "retained_generations",
                &json!(generations),
            )?;
        }
        rebuild(&tx).map_err(graph)?;
        tx.commit().map_err(unavailable)
    }
    /// Dispatch v1 tools against native canonical tables without an owner profile policy.
    pub fn v1_call(&self, auth: &Auth, tool: &str, args: Value) -> Result<Value> {
        if tool == "mother_get_context" {
            return self.v1_context(
                auth,
                args,
                &crate::recall_profiles::RecallProfiles::default(),
            );
        }
        if tool == "mother_search_context" {
            static SEARCH: std::sync::OnceLock<learned::NativeSearch> = std::sync::OnceLock::new();
            return SEARCH
                .get_or_init(learned::NativeSearch::from_environment)
                .search(self, auth, args);
        }
        if matches!(
            tool,
            "mother_write_record"
                | "mother_checkpoint"
                | "mother_propose_memory"
                | "mother_pin_record"
                | "mother_assign_task"
        ) {
            let mut tx = self.db.begin_write().map_err(unavailable)?;
            tx.set_durability(Durability::Immediate)
                .map_err(unavailable)?;
            metadata(&tx, "workspace")?;
            let result = match tool {
                "mother_write_record" | "mother_checkpoint" => {
                    if tool == "mother_checkpoint" && args["record"]["kind"] != "checkpoint" {
                        return Err(invalid("mother_checkpoint requires a checkpoint record."));
                    }
                    let input = Write::parse(args)?;
                    principal(&tx, auth, &input.kind.scope())?;
                    write_transaction(&tx, &auth.client_id, &input, self.clock.as_ref(), false)?
                }
                "mother_propose_memory" => propose(&tx, auth, args, self.clock.as_ref())?,
                "mother_pin_record" => pin(&tx, auth, args, self.clock.as_ref())?,
                _ => assign(&tx, auth, args, self.clock.as_ref())?,
            };
            tx.commit().map_err(unavailable)?;
            return Ok(result);
        }
        let tx = self.db.begin_read().map_err(unavailable)?;
        metadata(&tx, "workspace")?;
        let p = principal(&tx, auth, "read")?;
        let mut result = match tool {
            "mother_get_record" => {
                let id = required_id(&args, "id")?;
                p.access.require(id)?;
                record(&tx, id)?
            }
            "mother_get_record_history" => {
                if p.access.mode == AccessMode::Shared {
                    return Err(Error::domain(
                        "FORBIDDEN",
                        "Shared clients can read only the current version of granted records.",
                    ));
                }
                history(&tx, args)?
            }
            "mother_get_receipt" => {
                let id = required_id(&args, "operationId")?;
                let w = metadata(&tx, "workspace")?;
                let k = operation_key(string(&w, "generation")?, &auth.client_id, id);
                let op = tx
                    .get(OPERATIONS, &k)?
                    .or(tx.get(INBOX_OPERATIONS, &k)?)
                    .ok_or_else(not_found)?;
                let mut r = op["receipt"].clone();
                p.access.receipt(&auth.client_id, &mut r)?;
                r
            }
            "mother_get_proposal" => {
                let id = required_id(&args, "id")?;
                let mut proposal = tx.get(PROPOSALS, id)?.ok_or_else(not_found)?;
                p.access.proposal(&auth.client_id, &mut proposal)?;
                proposal = full_proposal(&tx, proposal, &p.access)?;
                p.access.proposal(&auth.client_id, &mut proposal)?;
                proposal
            }
            _ => return Err(invalid("Unknown v1 tool")),
        };
        p.access.scrub(&mut result);
        Ok(result)
    }
    /// Read a v1 context snapshot after applying the current grant and optional owner profile.
    pub fn v1_context(
        &self,
        auth: &Auth,
        args: Value,
        profiles: &crate::recall_profiles::RecallProfiles,
    ) -> Result<Value> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        let workspace = metadata(&tx, "workspace")?;
        let p = principal(&tx, auth, "read")?;
        let mut result = context(
            &tx,
            &auth.client_id,
            &p.access,
            args,
            workspace,
            profiles,
            self.clock.as_ref(),
        )?;
        p.access.scrub(&mut result);
        Ok(result)
    }
    /// Credential-free small portable export; incompatible graph state requires a full backup.
    pub fn v1_export(&self) -> Result<Value> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        if graph_state_exists(&tx)? {
            return Err(Error::domain(
                "FULL_BACKUP_REQUIRED",
                "Graph state cannot be represented by a v1 export; use a full backup.",
            ));
        }
        preflight_small_export(&tx)?;
        let bundle = bundle(
            &tx,
            self.clock
                .now()
                .to_rfc3339_opts(SecondsFormat::Millis, true),
        )?;
        if serde_json::to_vec(&bundle)?.len() > 64 * 1024 * 1024 {
            return Err(Error::domain(
                "FULL_BACKUP_REQUIRED",
                "Native state exceeds the small v1 portable export; use a full backup.",
            ));
        }
        crate::portability::validate(bundle).map_err(|e| {
            if e.code() == "RESOURCE_LIMIT" || e.code() == "INVALID_INPUT" {
                Error::domain(
                    "FULL_BACKUP_REQUIRED",
                    "Native state exceeds small v1 portable limits; use a full backup.",
                )
            } else {
                e
            }
        })
    }
}

fn prepare_grant(
    client: &str,
    scopes: &[String],
    access: &ClientAccess,
    garden: Option<&GardenAccess>,
) -> Result<(Auth, Principal)> {
    if !contracts::valid_client(client)
        || scopes.is_empty()
        || scopes.iter().any(|s| !contracts::valid_scope(s))
        || !scopes
            .iter()
            .any(|s| contracts::SCOPES.contains(&s.as_str()))
    {
        return Err(invalid("Explicit v1 scopes and valid client required"));
    }
    access.validate()?;
    if scopes
        .iter()
        .any(|s| contracts::GARDEN_SCOPES.contains(&s.as_str()))
        && garden.is_none()
    {
        return Err(invalid(
            "Graph scopes require explicit independent graph policy",
        ));
    }
    let garden = garden.cloned().unwrap_or_default();
    garden.validate()?;
    if access.mode == AccessMode::Shared
        && (matches!(garden, GardenAccess::All {})
            || scopes.iter().any(|s| {
                !matches!(
                    s.as_str(),
                    "read" | "context:propose" | "garden:read" | "garden:propose"
                )
            }))
    {
        return Err(Error::domain(
            "FORBIDDEN",
            "Shared grants cannot receive canonical writes or full graph visibility.",
        ));
    }
    let mut bytes = [0; 32];
    rand::rng().fill_bytes(&mut bytes);
    let token = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    Ok((
        Auth {
            client_id: client.into(),
            token: token.clone(),
        },
        Principal {
            token_hash: contracts::hash(&token),
            scopes: scopes.to_vec(),
            access: access.clone(),
            garden,
        },
    ))
}

fn write_transaction(
    tx: &redb::WriteTransaction,
    actor: &str,
    input: &Write,
    clock: &dyn super::Clock,
    review: bool,
) -> Result<Value> {
    let v = &input.value;
    identity(tx, v)?;
    let k = key(v, actor)?;
    check_family(tx, actor, v, if review { "v1_review" } else { "v1_write" })?;
    let fingerprint = contracts::hash(&contracts::canonical(v));
    if let Some(op) = tx.get(OPERATIONS, &k)? {
        if op["requestHash"] != fingerprint {
            return Err(reused());
        }
        return Ok(op["receipt"].clone());
    }
    if tx.get(INBOX_OPERATIONS, &k)?.is_some() {
        return Err(reused());
    }
    let previous = v
        .get("id")
        .map(|_| record(tx, string(v, "id")?))
        .transpose()?;
    if let Some(p) = &previous {
        if p["revision"] != v["expectedRevision"] {
            return Err(Error::domain(
                "CONFLICT",
                "Record changed; retrieve it before proposing a new edit.",
            ));
        }
        if p["kind"] != v["record"]["kind"] {
            return Err(Error::domain("KIND_MISMATCH", "Record kind cannot change."));
        }
    }
    references(tx, &v["record"])?;
    let mut time = clock.now();
    if let Some(p) = &previous {
        time = time.max(contracts::timestamp(string(p, "updatedAt")?)?);
    }
    let stamp = time.to_rfc3339_opts(SecondsFormat::Millis, true);
    let mut r = v["record"].clone();
    r["id"] = previous
        .as_ref()
        .map_or_else(|| json!(Uuid::new_v4().to_string()), |p| p["id"].clone());
    r["revision"] = json!(
        previous
            .as_ref()
            .map_or(Ok(0), |p| integer(p, "revision", 1, 9007199254740990))?
            + 1
    );
    r["actor"] = json!(actor);
    r["createdAt"] = previous
        .as_ref()
        .map_or_else(|| json!(stamp), |p| p["createdAt"].clone());
    r["updatedAt"] = json!(stamp);
    let receipt = json!({"operationId":v["operationId"],"generation":v["generation"],"committedAt":stamp,"record":r});
    put(tx, RECORDS, string(&r, "id")?, &r)?;
    put(
        tx,
        OPERATIONS,
        &k,
        &json!({"actor":actor,"generation":v["generation"],"operationId":v["operationId"],"requestHash":fingerprint,"receipt":receipt}),
    )?;
    claim(tx, actor, v, if review { "v1_review" } else { "v1_write" })?;
    if let Some(target) = v.get("supersedes") {
        let id = string(target, "id")?;
        let current = record(tx, id)?;
        if current["kind"] != "checkpoint" {
            return Err(Error::domain(
                "KIND_MISMATCH",
                "Only checkpoints can supersede checkpoints.",
            ));
        }
        if current["revision"] != target["expectedRevision"] || current["status"] != "active" {
            return Err(Error::domain(
                "CONFLICT",
                "Checkpoint to supersede changed or is inactive.",
            ));
        }
        let mut draft = crate::inbox::candidate(&current)?;
        draft["status"] = json!("superseded");
        let retire = Write::parse(
            json!({"workspaceId":v["workspaceId"],"generation":v["generation"],"operationId":Uuid::new_v4().to_string(),"id":id,"expectedRevision":target["expectedRevision"],"record":draft}),
        )?;
        write_transaction(tx, actor, &retire, clock, false)?;
    }
    Ok(receipt)
}

fn bundle(tx: &impl View, stamp: String) -> Result<Value> {
    let records = tx.values(RECORDS)?;
    let operations = tx.values(OPERATIONS)?;
    let mut result = json!({"schemaVersion":1,"exportedAt":stamp,"workspace":metadata(tx,"workspace")?,"persona":metadata(tx,"persona")?,"records":records,"operations":operations,"omittedExternalSources":[]});
    if tx.get(V1_META, "inbox")?.is_some() {
        result["inbox"] = json!({"version":1,"proposals":tx.values(PROPOSALS)?,"operations":tx.values(INBOX_OPERATIONS)?});
    }
    for (name, table) in [("pins", PINS), ("assignments", ASSIGNMENTS)] {
        if tx.get(V1_META, name)?.is_some() {
            result[name] = json!(tx.values(table)?);
        }
    }
    // Sources are pointers only; no external content or credential is captured.
    let mut sources = BTreeSet::new();
    let mut scan = |r: &Value| {
        if let Some(values) = r["sources"].as_array() {
            for source in values {
                if let Some(uri) = source["uri"].as_str() {
                    sources.insert(uri.to_owned());
                }
            }
        }
    };
    for r in result["records"].as_array().into_iter().flatten() {
        scan(r);
    }
    for o in result["operations"].as_array().into_iter().flatten() {
        scan(&o["receipt"]["record"]);
    }
    for p in result["inbox"]["proposals"]
        .as_array()
        .into_iter()
        .flatten()
    {
        scan(&p["record"]);
        scan(&p["before"]);
    }
    result["omittedExternalSources"] = json!(sources);
    Ok(result)
}

fn graph_state_exists(tx: &redb::ReadTransaction) -> Result<bool> {
    for table in [
        CURRENT,
        super::RECEIPTS,
        TableDefinition::new("garden_inbox"),
        TableDefinition::new("garden_review_operations"),
        TableDefinition::new("garden_blob_metadata"),
    ] {
        match tx.open_table(table) {
            Ok(t) => {
                if !t.is_empty().map_err(unavailable)? {
                    return Ok(true);
                }
            }
            Err(redb::TableError::TableDoesNotExist(_)) => {}
            Err(e) => return Err(unavailable(e)),
        }
    }
    Ok(!tx
        .open_table(super::super::history::HISTORY)
        .map_err(unavailable)?
        .is_empty()
        .map_err(unavailable)?)
}

fn preflight_small_export(tx: &redb::ReadTransaction) -> Result<()> {
    metadata(tx, "workspace")?;
    let count = |table| -> Result<u64> {
        tx.open_table(table)
            .map_err(unavailable)?
            .len()
            .map_err(unavailable)
    };
    let mut usage = crate::limits::Usage {
        records: count(RECORDS)? + count(PROPOSALS)?,
        operations: count(OPERATIONS)? + count(INBOX_OPERATIONS)?,
        payload_bytes: 0,
    };
    let full = || {
        Error::domain(
            "FULL_BACKUP_REQUIRED",
            "Native state exceeds small v1 portable limits; use a full backup.",
        )
    };
    usage.check().map_err(|_| full())?;
    for table in [RECORDS, OPERATIONS, PROPOSALS, INBOX_OPERATIONS] {
        for entry in tx
            .open_table(table)
            .map_err(unavailable)?
            .iter()
            .map_err(unavailable)?
        {
            let (_, bytes) = entry.map_err(unavailable)?;
            let row: Value = serde_json::from_slice(bytes.value())?;
            let size = if table.name() == OPERATIONS.name() {
                row["receipt"].to_string().len()
            } else if table.name() == INBOX_OPERATIONS.name() {
                row["request"].to_string().len() + row["receipt"].to_string().len()
            } else {
                row.to_string().len()
            };
            usage.payload_bytes = usage
                .payload_bytes
                .checked_add(size as u64)
                .ok_or_else(full)?;
            usage.check().map_err(|_| full())?;
        }
    }
    Ok(())
}

pub(crate) fn rebuild(tx: &redb::WriteTransaction) -> super::Result<()> {
    initialize_tables(tx).map_err(super::storage_error)?;
    if tx
        .get(V1_META, "workspace")
        .map_err(super::storage_error)?
        .is_none()
    {
        return Ok(());
    }
    let state = state_write(tx)?;
    let mut w = metadata(tx, "workspace").map_err(super::storage_error)?;
    if w["id"] != state.workspace_id {
        return Err(super::storage_error("Native workspace mismatch"));
    }
    w["generation"] = json!(state.generation);
    put(tx, V1_META, "workspace", &w).map_err(super::storage_error)?;
    tx.open_table(ADMISSIONS)
        .map_err(super::storage_error)?
        .retain(|_, _| false)
        .map_err(super::storage_error)?;
    tx.open_table(PENDING)
        .map_err(super::storage_error)?
        .retain(|_, _| false)
        .map_err(super::storage_error)?;
    for entry in tx
        .open_table(PROPOSALS)
        .map_err(super::storage_error)?
        .iter()
        .map_err(super::storage_error)?
    {
        let (_, bytes) = entry.map_err(super::storage_error)?;
        let p: Value = serde_json::from_slice(bytes.value()).map_err(super::storage_error)?;
        if p["status"] == "pending" {
            let hash = crate::inbox::dedup(&p);
            if tx
                .get(PENDING, &hash)
                .map_err(super::storage_error)?
                .is_some()
            {
                return Err(super::storage_error("Duplicate pending native proposal"));
            }
            put(tx, PENDING, &hash, &p["id"]).map_err(super::storage_error)?;
        }
    }
    for entry in tx
        .open_table(INBOX_OPERATIONS)
        .map_err(super::storage_error)?
        .iter()
        .map_err(super::storage_error)?
    {
        let (k, bytes) = entry.map_err(super::storage_error)?;
        let row: Value = serde_json::from_slice(bytes.value()).map_err(super::storage_error)?;
        if row["family"] == "propose" {
            let id = string(&row["receipt"]["proposal"], "id").map_err(super::storage_error)?;
            if tx
                .get(ADMISSIONS, id)
                .map_err(super::storage_error)?
                .is_none()
            {
                put(tx, ADMISSIONS, id, &json!(k.value())).map_err(super::storage_error)?;
            }
        }
    }
    for table in [OPERATIONS, INBOX_OPERATIONS] {
        for entry in tx
            .open_table(table)
            .map_err(super::storage_error)?
            .iter()
            .map_err(super::storage_error)?
        {
            let (k, bytes) = entry.map_err(super::storage_error)?;
            let row: Value = serde_json::from_slice(bytes.value()).map_err(super::storage_error)?;
            let family = if table.name() == INBOX_OPERATIONS.name() {
                format!(
                    "v1_{}",
                    string(&row, "family").map_err(super::storage_error)?
                )
            } else if let Some(inbox) = tx
                .get(INBOX_OPERATIONS, k.value())
                .map_err(super::storage_error)?
            {
                if inbox["family"] != "review" || inbox["receipt"]["receipt"] != row["receipt"] {
                    return Err(super::storage_error("Native operation collision"));
                }
                "v1_review".into()
            } else {
                "v1_write".into()
            };
            inbox::claim_operation(
                tx,
                string(&row, "generation").map_err(super::storage_error)?,
                string(&row, "operationId").map_err(super::storage_error)?,
                string(&row, "actor").map_err(super::storage_error)?,
                &family,
            )?;
        }
    }
    Ok(())
}
pub(crate) fn registry_visit(
    tx: &redb::ReadTransaction,
    mut visit: impl FnMut(String, String, String) -> super::Result<()>,
) -> super::Result<()> {
    match tx.open_table(V1_META) {
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(()),
        Err(e) => return Err(super::storage_error(e)),
        Ok(t) => {
            if t.get("workspace").map_err(super::storage_error)?.is_none() {
                return Ok(());
            }
        }
    }
    let canonical = tx.open_table(OPERATIONS).map_err(super::storage_error)?;
    let extension = tx
        .open_table(INBOX_OPERATIONS)
        .map_err(super::storage_error)?;
    for entry in canonical.iter().map_err(super::storage_error)? {
        let (k, bytes) = entry.map_err(super::storage_error)?;
        let row: Value = serde_json::from_slice(bytes.value()).map_err(super::storage_error)?;
        if let Some(inbox) = extension.get(k.value()).map_err(super::storage_error)? {
            let inbox: Value =
                serde_json::from_slice(inbox.value()).map_err(super::storage_error)?;
            if inbox["family"] != "review"
                || inbox["receipt"]["receipt"] != row["receipt"]
                || inbox["actor"] != row["actor"]
            {
                return Err(super::storage_error("Native operation family collision"));
            }
        } else {
            visit(
                k.value().to_owned(),
                string(&row, "actor")
                    .map_err(super::storage_error)?
                    .to_owned(),
                "v1_write".into(),
            )?;
        }
    }
    for entry in extension.iter().map_err(super::storage_error)? {
        let (k, bytes) = entry.map_err(super::storage_error)?;
        let row: Value = serde_json::from_slice(bytes.value()).map_err(super::storage_error)?;
        visit(
            k.value().to_owned(),
            string(&row, "actor")
                .map_err(super::storage_error)?
                .to_owned(),
            format!(
                "v1_{}",
                string(&row, "family").map_err(super::storage_error)?
            ),
        )?;
    }
    Ok(())
}
pub(crate) fn validate_restored(tx: &redb::ReadTransaction) -> super::Result<()> {
    audit_native(tx).map_err(super::storage_error)
}

fn visit_rows(
    tx: &redb::ReadTransaction,
    table: TableDefinition<&str, &[u8]>,
    mut visit: impl FnMut(&str, Value) -> Result<()>,
) -> Result<()> {
    let table = tx.open_table(table).map_err(unavailable)?;
    for entry in table.iter().map_err(unavailable)? {
        let (k, bytes) = entry.map_err(unavailable)?;
        if bytes.value().len() > 4 * 1024 * 1024 {
            return Err(invalid("Native row exceeds portable bound"));
        }
        visit(k.value(), serde_json::from_slice(bytes.value())?)?;
    }
    Ok(())
}
fn audit_record(tx: &redb::ReadTransaction, r: &Value) -> Result<()> {
    if contracts::parse_record(r.clone())? != *r {
        return Err(invalid("Native record is not normalized"));
    }
    references(tx, r)
}
fn scoped_records(tx: &redb::ReadTransaction, values: &[&Value]) -> Result<Vec<Value>> {
    let mut ids = BTreeSet::new();
    for value in values {
        for r in [
            *value,
            &value["record"],
            &value["before"],
            &value["receipt"]["record"],
            &value["receipt"]["receipt"]["record"],
            &value["receipt"]["proposal"]["record"],
            &value["receipt"]["proposal"]["before"],
        ] {
            if r["kind"].is_string()
                && let Some(id) = r["id"].as_str()
            {
                ids.insert(id.to_owned());
            }
            if let Some(refs) = r["references"].as_array() {
                if refs.len() > 30 {
                    return Err(invalid("Too many native references"));
                }
                for id in refs {
                    ids.insert(
                        id.as_str()
                            .ok_or_else(|| invalid("Invalid reference"))?
                            .to_owned(),
                    );
                }
            }
        }
        if let Some(id) = value["targetId"].as_str() {
            ids.insert(id.to_owned());
        }
        if let Some(id) = value["receipt"]["proposal"]["duplicateRecordId"].as_str() {
            ids.insert(id.to_owned());
        }
        if let Some(similar) = value["receipt"]["proposal"]["similarRecordIds"].as_array() {
            if similar.len() > 5 {
                return Err(invalid("Too many similarity warnings"));
            }
            for id in similar {
                ids.insert(
                    id.as_str()
                        .ok_or_else(|| invalid("Invalid similarity warning"))?
                        .to_owned(),
                );
            }
        }
    }
    ids.into_iter().map(|id| record(tx, &id)).collect()
}
fn audit_operation(tx: &redb::ReadTransaction, k: &str, op: &Value) -> Result<()> {
    contracts::strict(
        op,
        &[
            "actor",
            "generation",
            "operationId",
            "requestHash",
            "receipt",
        ],
    )?;
    let actor = string(op, "actor")?;
    if actor.is_empty() || actor.encode_utf16().count() > 80 {
        return Err(invalid("Invalid operation actor"));
    }
    for field in ["generation", "operationId"] {
        contracts::id(string(op, field)?)?;
    }
    if k != key(op, actor)? || !contracts::hex_digest(string(op, "requestHash")?) {
        return Err(invalid("Invalid native operation key or hash"));
    }
    let r = &op["receipt"];
    contracts::strict(r, &["operationId", "generation", "committedAt", "record"])?;
    contracts::timestamp(string(r, "committedAt")?)?;
    let produced = &r["record"];
    audit_record(tx, produced)?;
    let latest = record(tx, string(produced, "id")?)?;
    if op["actor"] != produced["actor"]
        || op["generation"] != r["generation"]
        || op["operationId"] != r["operationId"]
        || r["committedAt"] != produced["updatedAt"]
        || integer(produced, "revision", 1, 9007199254740991)?
            > integer(&latest, "revision", 1, 9007199254740991)?
        || (produced["revision"] == latest["revision"] && *produced != latest)
    {
        return Err(invalid("Invalid native operation receipt"));
    }
    Ok(())
}
fn audit_inbox_operation(tx: &redb::ReadTransaction, w: &Value, op: &Value) -> Result<()> {
    let id = if op["family"] == "propose" {
        string(&op["receipt"]["proposal"], "id")?
    } else {
        string(&op["request"], "proposalId")?
    };
    let p = tx
        .get(PROPOSALS, id)?
        .ok_or_else(|| invalid("Native proposal missing"))?;
    let admission = if op["family"] == "propose" {
        op.clone()
    } else {
        let k = tx
            .get(ADMISSIONS, id)?
            .ok_or_else(|| invalid("Native admission missing"))?;
        tx.get(
            INBOX_OPERATIONS,
            k.as_str().ok_or_else(|| invalid("Invalid admission key"))?,
        )?
        .ok_or_else(|| invalid("Native admission missing"))?
    };
    let review = if p["status"] == "pending" {
        None
    } else {
        let k = operation_key(
            string(&p["receipt"], "generation")?,
            inbox::OWNER,
            string(&p["receipt"], "operationId")?,
        );
        Some(
            tx.get(INBOX_OPERATIONS, &k)?
                .ok_or_else(|| invalid("Native review missing"))?,
        )
    };
    if op["family"] == "review" && review.as_ref() != Some(op) {
        return Err(invalid("Native review attribution disagrees"));
    }
    let mut canonical = Vec::new();
    if let Some(review) = &review
        && let Some(c) = tx.get(OPERATIONS, &key(review, inbox::OWNER)?)?
    {
        canonical.push(c);
    }
    let empty = Value::Null;
    let records = scoped_records(tx, &[&p, &admission, review.as_ref().unwrap_or(&empty)])?;
    let mut operations = vec![admission];
    if let Some(review) = review {
        operations.push(review);
    }
    crate::inbox::validate_with_limits(
        &json!({"version":1,"proposals":[p],"operations":operations}),
        w,
        &records,
        &canonical,
        false,
    )
}
fn audit_native(tx: &redb::ReadTransaction) -> Result<()> {
    let mut has_workspace = false;
    match tx.open_table(V1_META) {
        Err(redb::TableError::TableDoesNotExist(_)) => {}
        Err(e) => return Err(unavailable(e)),
        Ok(meta) => {
            has_workspace = meta.get("workspace").map_err(unavailable)?.is_some();
            for entry in meta.iter().map_err(unavailable)? {
                let (k, _) = entry.map_err(unavailable)?;
                if !["workspace", "persona", "inbox", "pins", "assignments"].contains(&k.value()) {
                    return Err(invalid("Unknown native metadata"));
                }
            }
        }
    }
    if !has_workspace {
        for table in TABLES {
            match tx.open_table(table) {
                Ok(t) => {
                    if !t.is_empty().map_err(unavailable)? {
                        return Err(invalid("Orphan native data without compatibility identity"));
                    }
                }
                Err(redb::TableError::TableDoesNotExist(_)) => {}
                Err(e) => return Err(unavailable(e)),
            }
        }
        return Ok(());
    }
    let w = metadata(tx, "workspace")?;
    contracts::validate_workspace(&w)?;
    contracts::validate_persona(&metadata(tx, "persona")?)?;
    let state = super::state_read(tx).map_err(graph)?;
    let generations = super::super::indexes::admitted_generations(tx).map_err(graph)?;
    if w["id"] != state.workspace_id || w["generation"] != state.generation {
        return Err(invalid("Native and graph workspace identity disagree"));
    }
    for (name, table) in [
        ("inbox", PROPOSALS),
        ("inbox", INBOX_OPERATIONS),
        ("pins", PINS),
        ("assignments", ASSIGNMENTS),
    ] {
        if tx.get(V1_META, name)?.is_none()
            && !tx
                .open_table(table)
                .map_err(unavailable)?
                .is_empty()
                .map_err(unavailable)?
        {
            return Err(invalid("Native optional extension omitted by metadata"));
        }
    }
    visit_rows(tx, RECORDS, |k, r| {
        if string(&r, "id")? != k {
            return Err(invalid("Native record key disagrees"));
        }
        audit_record(tx, &r)
    })?;
    visit_rows(tx, OPERATIONS, |k, op| {
        if !generations.contains(string(&op, "generation")?) {
            return Err(invalid("Undeclared native operation generation"));
        }
        audit_operation(tx, k, &op)
    })?;
    visit_rows(tx, PROPOSALS, |k, p| {
        if string(&p, "id")? != k || !generations.contains(string(&p, "generation")?) {
            return Err(invalid("Native proposal key or generation disagrees"));
        }
        let admission = tx
            .get(ADMISSIONS, k)?
            .ok_or_else(|| invalid("Native proposal admission missing"))?;
        let op = tx
            .get(
                INBOX_OPERATIONS,
                admission
                    .as_str()
                    .ok_or_else(|| invalid("Invalid admission index"))?,
            )?
            .ok_or_else(|| invalid("Native admission operation missing"))?;
        if p["status"] == "pending" && tx.get(PENDING, &crate::inbox::dedup(&p))? != Some(json!(k))
        {
            return Err(invalid("Native pending dedup index disagrees"));
        }
        audit_inbox_operation(tx, &w, &op)
    })?;
    visit_rows(tx, INBOX_OPERATIONS, |k, op| {
        if key(&op, string(&op, "actor")?)? != k
            || !generations.contains(string(&op, "generation")?)
        {
            return Err(invalid(
                "Native inbox operation key or generation disagrees",
            ));
        }
        audit_inbox_operation(tx, &w, &op)
    })?;
    if tx
        .open_table(PINS)
        .map_err(unavailable)?
        .len()
        .map_err(unavailable)?
        > crate::pins::MAX_PINS as u64
    {
        return Err(invalid("Too many native pins"));
    }
    visit_rows(tx, PINS, |k, pin| {
        if string(&pin, "recordId")? != k {
            return Err(invalid("Native pin key disagrees"));
        }
        crate::pins::validate(&json!([pin]), &[record(tx, k)?])
    })?;
    visit_rows(tx, ASSIGNMENTS, |k, a| {
        if string(&a, "recordId")? != k {
            return Err(invalid("Native assignment key disagrees"));
        }
        crate::handoffs::validate_with_limits(&json!([a]), &[record(tx, k)?], false)
    })?;
    Ok(())
}

fn recallable(r: &Value) -> bool {
    !matches!(
        r["status"].as_str(),
        Some("retracted" | "superseded" | "cancelled")
    )
}
fn unsettled(r: &Value) -> bool {
    (r["kind"] == "task" && matches!(r["status"].as_str(), Some("doing" | "blocked")))
        || (r["kind"] == "action" && matches!(r["status"].as_str(), Some("pending" | "unknown")))
}
fn recent(a: &Value, b: &Value) -> std::cmp::Ordering {
    b["updatedAt"]
        .as_str()
        .cmp(&a["updatedAt"].as_str())
        .then(a["id"].as_str().cmp(&b["id"].as_str()))
}
fn excerpt(content: &str, max: usize) -> String {
    let mut used = 0;
    content
        .chars()
        .take_while(|c| {
            used += c.len_utf16();
            used <= max
        })
        .collect()
}
fn preview(r: &Value) -> Value {
    let content = r["content"].as_str().unwrap_or_default();
    json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"actor":r["actor"],"updatedAt":r["updatedAt"],"excerpt":excerpt(content,400),"truncated":content.encode_utf16().count()>400})
}
fn counts() -> Value {
    json!({"memory":0,"decision":0,"task":0,"checkpoint":0,"artifact":0,"action":0})
}
fn increment(counts: &mut Value, kind: &str, amount: i64) {
    counts[kind] = json!(counts[kind].as_i64().unwrap_or_default() + amount);
}
fn visible_records(tx: &impl View, access: &ClientAccess) -> Result<Vec<Value>> {
    Ok(tx
        .values(RECORDS)?
        .into_iter()
        .filter(|r| r["id"].as_str().is_some_and(|id| access.allows(id)))
        .collect())
}
fn context(
    tx: &impl View,
    client: &str,
    access: &ClientAccess,
    query: Value,
    workspace: Value,
    profiles: &crate::recall_profiles::RecallProfiles,
    clock: &dyn super::Clock,
) -> Result<Value> {
    contracts::strict(
        &query,
        &[
            "query",
            "kind",
            "limit",
            "updatedSince",
            "updatedBefore",
            "profile",
        ],
    )?;
    let profile = query
        .get("profile")
        .map(|_| string(&query, "profile"))
        .transpose()?;
    let eligible = profile
        .map(|name| profiles.pins(name, string(&workspace, "id")?))
        .transpose()?;
    let mut pins = tx
        .values(PINS)?
        .into_iter()
        .filter(|p| p["recordId"].as_str().is_some_and(|id| access.allows(id)))
        .collect::<Vec<_>>();
    let mut excluded_pins = std::collections::HashSet::new();
    if let Some(eligible) = eligible {
        pins.retain(|pin| {
            let id = pin["recordId"].as_str().unwrap_or_default();
            if eligible.contains(id) {
                return true;
            }
            excluded_pins.insert(id.to_owned());
            false
        });
    }
    let visible = |id: &str| access.allows(id) && !excluded_pins.contains(id);
    let (since, before) = contracts::time_window(&query)?;
    let q = query
        .get("query")
        .map(|_| string(&query, "query"))
        .transpose()?;
    if q.is_some_and(|q| q.encode_utf16().count() > 200) {
        return Err(invalid("Query too long"));
    }
    let kind = query
        .get("kind")
        .map(|_| string(&query, "kind"))
        .transpose()?;
    if let Some(k) = kind {
        Kind::parse(k)?;
    }
    let limit = if query.get("limit").is_some() {
        integer(&query, "limit", 1, 100)? as usize
    } else {
        20
    };
    let all: Vec<_> = tx
        .values(RECORDS)?
        .into_iter()
        .filter(|r| r["id"].as_str().is_some_and(visible))
        .collect();
    let mut by_kind = counts();
    let mut recallable_kind = counts();
    let mut recallable_total = 0;
    for r in &all {
        let k = string(r, "kind")?;
        increment(&mut by_kind, k, 1);
        if recallable(r) {
            increment(&mut recallable_kind, k, 1);
            recallable_total += 1;
        }
    }
    let inventory = json!({"totalRecords":all.len(),"byKind":by_kind,"recallableRecords":recallable_total,"recallableByKind":recallable_kind,"excludedRecords":all.len()-recallable_total});
    let mut matching = all
        .iter()
        .filter(|r| {
            recallable(r)
                && kind.is_none_or(|k| r["kind"] == k)
                && q.is_none_or(|q| {
                    format!(
                        "{} {}",
                        r["title"].as_str().unwrap_or_default(),
                        r["content"].as_str().unwrap_or_default()
                    )
                    .to_ascii_lowercase()
                    .contains(&q.to_ascii_lowercase())
                })
                && since
                    .as_deref()
                    .is_none_or(|s| r["updatedAt"].as_str().unwrap_or_default() >= s)
                && before
                    .as_deref()
                    .is_none_or(|b| r["updatedAt"].as_str().unwrap_or_default() < b)
        })
        .cloned()
        .collect::<Vec<_>>();
    let total = matching.len();
    let mut matching_kind = counts();
    for r in &matching {
        increment(&mut matching_kind, string(r, "kind")?, 1);
    }
    let balanced = q.is_none() && kind.is_none() && since.is_none() && before.is_none();
    matching.sort_by(recent);
    if balanced {
        let seats = [
            "checkpoint",
            "task",
            "decision",
            "memory",
            "action",
            "artifact",
        ];
        let mut groups = Vec::new();
        for seat in seats {
            let mut group = matching
                .iter()
                .filter(|r| r["kind"] == seat)
                .cloned()
                .collect::<Vec<_>>();
            group.sort_by(|a, b| unsettled(b).cmp(&unsettled(a)).then_with(|| recent(a, b)));
            groups.push(group);
        }
        let mut selected = Vec::new();
        let mut turn = 0;
        while selected.len() < limit {
            let mut any = false;
            for group in &groups {
                if let Some(r) = group.get(turn) {
                    selected.push(r.clone());
                    any = true;
                    if selected.len() == limit {
                        break;
                    }
                }
            }
            if !any {
                break;
            }
            turn += 1;
        }
        matching = selected;
        matching.sort_by(recent);
    } else {
        matching.truncate(limit);
    }
    let records = matching.iter().map(preview).collect::<Vec<_>>();
    let mut work = all
        .iter()
        .filter(|r| unsettled(r))
        .cloned()
        .collect::<Vec<_>>();
    work.sort_by(recent);
    let work_total = work.len();
    work.truncate(20);
    let work = work
        .iter()
        .map(|r| json!({"id":r["id"],"kind":r["kind"],"status":r["status"]}))
        .collect::<Vec<_>>();
    let c = &inventory["recallableByKind"];
    let order = if balanced {
        " Previews take turns across categories, newest first within each, so recent checkpoints do not displace older memories and decisions; selection.omittedByKind lists what was left out."
    } else {
        ""
    };
    let summary = format!(
        "Shared context has {} recallable records: {} memories, {} decisions, {} tasks, {} checkpoints, {} artifacts and {} actions. {} retained historical records are excluded from recall. This request matches {total} records and returns {} previews.{order}",
        inventory["recallableRecords"],
        c["memory"],
        c["decision"],
        c["task"],
        c["checkpoint"],
        c["artifact"],
        c["action"],
        inventory["excludedRecords"],
        records.len()
    );
    let mut returned = counts();
    let mut omitted = matching_kind;
    for r in &records {
        let k = string(r, "kind")?;
        increment(&mut returned, k, 1);
        increment(&mut omitted, k, -1);
    }
    let selection = json!({"query":q,"kind":kind,"updatedSince":since,"updatedBefore":before,"order":if balanced {"balanced"} else {"recent"},"matchingRecords":total,"returnedRecords":records.len(),"omittedRecords":total-records.len(),"returnedByKind":returned,"omittedByKind":omitted});
    let work_summary = json!({"total":work_total,"returned":work.len(),"omitted":work_total-work.len(),"scope":"Workspace-wide tasks in doing/blocked and actions in pending/unknown. Checkpoint next steps and open questions are not counted as unsettled task/action records."});
    let mut result = json!({"workspace":workspace,"access":access.metadata(),"capturedAt":clock.now().to_rfc3339_opts(SecondsFormat::Millis,true),"inventory":inventory,"selection":selection,"summary":summary,"omitted":total-records.len(),"records":records,"unsettledWork":work,"unsettledWorkSummary":work_summary,"unsettledWorkTruncated":work_total>20,"note":"Inventory counts cover all canonical record categories, independently of this request's query/kind filters and preview limit. Zero memory-category records is not an empty workspace when other categories contain records. Unreviewed inbox proposals are excluded. Excerpts are navigation, not complete records; retrieve IDs before relying on details or editing. Unsettled work counts only the task/action states described in unsettledWorkSummary. A handoff does not pause running jobs."});
    pins.sort_by(|a, b| {
        a["pinnedAt"]
            .as_str()
            .cmp(&b["pinnedAt"].as_str())
            .then(a["recordId"].as_str().cmp(&b["recordId"].as_str()))
    });
    let pins_total = pins.len();
    let mut pinned = Vec::new();
    for pin in pins {
        let r = record(tx, string(&pin, "recordId")?)?;
        if !recallable(&r) {
            continue;
        }
        let content = string(&r, "content")?;
        let text = excerpt(content, 2000);
        pinned.push(json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"updatedAt":r["updatedAt"],"content":text,"truncated":text.len()<content.len(),"pinnedBy":pin["actor"],"pinnedAt":pin["pinnedAt"]}));
    }
    result["pinnedSummary"] = json!({"total":pins_total,"shown":pinned.len(),"inactive":pins_total-pinned.len(),"limit":crate::pins::MAX_PINS});
    result["pinned"] = json!(pinned);
    let mut assignments = tx
        .values(ASSIGNMENTS)?
        .into_iter()
        .filter(|a| a["assignee"] == client && a["recordId"].as_str().is_some_and(visible))
        .collect::<Vec<_>>();
    assignments.sort_by(|a, b| {
        a["assignedAt"]
            .as_str()
            .cmp(&b["assignedAt"].as_str())
            .then(a["recordId"].as_str().cmp(&b["recordId"].as_str()))
    });
    let mut assigned = Vec::new();
    let mut total_assigned = 0;
    for a in assignments {
        let r = record(tx, string(&a, "recordId")?)?;
        if r["kind"] != "task"
            || !matches!(r["status"].as_str(), Some("todo" | "doing" | "blocked"))
        {
            continue;
        }
        total_assigned += 1;
        if assigned.len() >= 10 {
            continue;
        }
        let content = string(&r, "content")?;
        let text = content.chars().take(600).collect::<String>();
        assigned.push(json!({"id":r["id"],"title":r["title"],"status":r["status"],"revision":r["revision"],"updatedAt":r["updatedAt"],"content":text,"truncated":text.len()<content.len(),"assignedBy":a["assignedBy"],"assignedAt":a["assignedAt"]}));
    }
    result["assignedToYouSummary"] = json!({"total":total_assigned,"shown":assigned.len()});
    result["assignedToYou"] = json!(assigned);
    if access.mode != AccessMode::Shared {
        result["persona"] = metadata(tx, "persona")?;
        result["harnesses"] = json!(client_names(tx)?);
    } else {
        result["selection"]["scope"] = json!("granted_records");
        result["unsettledWorkSummary"]["scope"] = json!(
            "Granted tasks in doing/blocked and actions in pending/unknown; hidden records are excluded."
        );
        result["note"] = json!(
            "Inventory, selection and unsettled-work counts cover only this client's granted canonical records. Unreviewed inbox proposals are excluded. Excerpts are navigation; retrieve full records before relying on them. A handoff does not pause running jobs."
        );
    }
    if let Some(profile) = profile {
        result["selection"]["profile"] = json!(profile);
        result["selection"]["scope"] = json!("profiled_granted_records");
        result["unsettledWorkSummary"]["scope"] = json!(
            "Visible tasks in doing/blocked and actions in pending/unknown under the current grant and recall profile."
        );
        result["note"] = json!(
            "Inventory and selection counts cover the current grant and recall profile. Excerpts are navigation; retrieve full records before relying on them. A handoff does not pause running jobs."
        );
    }
    Ok(result)
}

fn client_names(tx: &impl View) -> Result<Vec<String>> {
    tx.keys(PRINCIPALS)
}

fn pin(
    tx: &redb::WriteTransaction,
    auth: &Auth,
    args: Value,
    clock: &dyn super::Clock,
) -> Result<Value> {
    contracts::strict(&args, &["id", "pinned"])?;
    let id = string(&args, "id")?;
    contracts::id(id)?;
    let pinned = args["pinned"]
        .as_bool()
        .ok_or_else(|| invalid("pinned must be true or false"))?;
    let p = principal(tx, auth, "read")?;
    p.access.require(id)?;
    let r = record(tx, id)?;
    let kind = string(&r, "kind")?;
    principal(tx, auth, &Kind::parse(kind)?.scope())?;
    if !matches!(kind, "memory" | "decision") {
        return Err(invalid("Only memories and decisions can be pinned"));
    }
    let current = tx.get(PINS, id)?;
    let changed = current.is_some() != pinned;
    let mut result = json!({"id":id,"kind":kind,"pinned":pinned,"changed":changed});
    if pinned {
        let p = if let Some(p) = current {
            p
        } else {
            if !recallable(&r) {
                return Err(invalid(
                    "Retracted, superseded or cancelled records cannot be pinned",
                ));
            }
            if tx
                .open_table(PINS)
                .map_err(unavailable)?
                .len()
                .map_err(unavailable)?
                >= crate::pins::MAX_PINS as u64
            {
                return Err(Error::domain(
                    "RESOURCE_LIMIT",
                    "At most 12 records can be pinned. Unpin one first.",
                ));
            }
            let p = json!({"recordId":id,"actor":auth.client_id,"pinnedAt":clock.now().to_rfc3339_opts(SecondsFormat::Millis,true)});
            put(tx, PINS, id, &p)?;
            p
        };
        result["pinnedBy"] = p["actor"].clone();
        result["pinnedAt"] = p["pinnedAt"].clone();
    } else {
        remove(tx, PINS, id)?;
    }
    put(tx, V1_META, "pins", &json!(true))?;
    Ok(result)
}
fn assign(
    tx: &redb::WriteTransaction,
    auth: &Auth,
    args: Value,
    clock: &dyn super::Clock,
) -> Result<Value> {
    contracts::strict(&args, &["id", "assignee"])?;
    let id = string(&args, "id")?;
    contracts::id(id)?;
    let assignee = match args.get("assignee") {
        Some(Value::Null) => None,
        Some(Value::String(name)) if contracts::valid_client(name) => Some(name.as_str()),
        _ => {
            return Err(invalid(
                "assignee must be a harness name from context, or null to clear",
            ));
        }
    };
    principal(tx, auth, "task:write")?.access.require(id)?;
    let r = record(tx, id)?;
    if r["kind"] != "task" {
        return Err(invalid("Only tasks can be handed to a harness"));
    }
    if let Some(name) = assignee {
        if !matches!(r["status"].as_str(), Some("todo" | "doing" | "blocked")) {
            return Err(invalid(
                "Only open tasks (todo, doing or blocked) can be handed off",
            ));
        }
        let receiver: Principal =
            serde_json::from_value(tx.get(PRINCIPALS, name)?.ok_or_else(|| {
                invalid("No current harness has that name; use one listed in context")
            })?)?;
        receiver.access.validate()?;
        if !receiver.access.allows(id) {
            return Err(invalid("That harness is not allowed to read this task"));
        }
    }
    let current = tx.get(ASSIGNMENTS, id)?;
    let changed = current.as_ref().and_then(|a| a["assignee"].as_str()) != assignee;
    if changed {
        if let Some(name) = assignee {
            put(
                tx,
                ASSIGNMENTS,
                id,
                &json!({"recordId":id,"assignee":name,"assignedBy":auth.client_id,"assignedAt":clock.now().to_rfc3339_opts(SecondsFormat::Millis,true)}),
            )?;
        } else {
            remove(tx, ASSIGNMENTS, id)?;
        }
    }
    put(tx, V1_META, "assignments", &json!(true))?;
    Ok(json!({"id":id,"assignee":assignee,"changed":changed}))
}

fn duplicate(tx: &impl View, draft: &Value, access: &ClientAccess) -> Result<Option<String>> {
    for r in visible_records(tx, access)? {
        if r["kind"] == draft["kind"] && crate::inbox::candidate(&r)? == *draft {
            return Ok(Some(string(&r, "id")?.to_owned()));
        }
    }
    Ok(None)
}
fn full_proposal(tx: &impl View, mut p: Value, access: &ClientAccess) -> Result<Value> {
    let w = metadata(tx, "workspace")?;
    let current = p["targetId"]
        .as_str()
        .map(|id| record(tx, id))
        .transpose()?;
    p["conflict"] = json!(
        p["generation"] != w["generation"]
            || current
                .as_ref()
                .is_some_and(|r| r["revision"] != p["expectedRevision"])
    );
    p["currentRecord"] = json!(current);
    p["duplicateRecordId"] = if p["targetId"].is_null() {
        json!(duplicate(tx, &p["record"], access)?)
    } else {
        Value::Null
    };
    let mut similar = Vec::new();
    for r in visible_records(tx, access)? {
        if r["kind"] == p["record"]["kind"]
            && (string(&r, "title")?.eq_ignore_ascii_case(string(&p["record"], "title")?)
                || string(&r, "content")?.eq_ignore_ascii_case(string(&p["record"], "content")?))
            && r["id"] != p["targetId"]
            && r["id"] != p["duplicateRecordId"]
        {
            similar.push(r["id"].clone());
            if similar.len() >= 5 {
                break;
            }
        }
    }
    p["similarRecordIds"] = json!(similar);
    Ok(p)
}
fn persist_inbox_operation(
    tx: &redb::WriteTransaction,
    actor: &str,
    family: &str,
    v: &Value,
    receipt: &Value,
) -> Result<()> {
    put(
        tx,
        INBOX_OPERATIONS,
        &key(v, actor)?,
        &json!({"actor":actor,"generation":v["generation"],"operationId":v["operationId"],"family":family,"requestHash":contracts::hash(&contracts::canonical(v)),"request":v,"receipt":receipt}),
    )?;
    if family == "propose" {
        let id = string(&receipt["proposal"], "id")?;
        if tx.get(ADMISSIONS, id)?.is_none() {
            put(tx, ADMISSIONS, id, &json!(key(v, actor)?))?;
        }
    }
    claim(tx, actor, v, &format!("v1_{family}"))
}
fn propose(
    tx: &redb::WriteTransaction,
    auth: &Auth,
    raw: Value,
    clock: &dyn super::Clock,
) -> Result<Value> {
    let v = crate::inbox::parse_proposal(raw)?;
    let kind = Kind::parse(string(&v["record"], "kind")?)?;
    let p = match principal(tx, auth, "context:propose") {
        Ok(p) => p,
        Err(e) if e.code() == "FORBIDDEN" => principal(tx, auth, &kind.scope())?,
        Err(e) => return Err(e),
    };
    identity(tx, &v)?;
    if let Some(id) = v["id"].as_str() {
        p.access.require(id)?;
    }
    p.access.references(&v["record"])?;
    if let Some(mut r) = prior(tx, &auth.client_id, &v, "v1_propose")? {
        p.access.receipt(&auth.client_id, &mut r)?;
        return Ok(r);
    }
    let before = v["id"].as_str().map(|id| record(tx, id)).transpose()?;
    if let Some(r) = &before {
        if r["revision"] != v["expectedRevision"] {
            return Err(Error::domain("CONFLICT", "Target revision changed."));
        }
        if r["kind"] != v["record"]["kind"] {
            return Err(Error::domain("KIND_MISMATCH", "Target kind cannot change."));
        }
    }
    references(tx, &v["record"])?;
    let stamp = clock.now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let mut proposal = json!({"id":Uuid::new_v4().to_string(),"revision":1,"status":"pending","actor":auth.client_id,"workspaceId":v["workspaceId"],"generation":v["generation"],"source":v["source"],"basis":v["basis"],"record":v["record"],"targetId":v.get("id"),"expectedRevision":v["expectedRevision"],"before":before,"createdAt":stamp,"updatedAt":stamp,"receipt":null});
    let hash = crate::inbox::dedup(&proposal);
    if let Some(old) = tx.get(PENDING, &hash)? {
        proposal = tx
            .get(
                PROPOSALS,
                old.as_str()
                    .ok_or_else(|| invalid("Invalid proposal index"))?,
            )?
            .ok_or_else(not_found)?;
    } else {
        put(tx, PROPOSALS, string(&proposal, "id")?, &proposal)?;
        put(tx, PENDING, &hash, &proposal["id"])?;
    }
    put(tx, V1_META, "inbox", &json!(true))?;
    let result = json!({"proposal":full_proposal(tx,proposal,&p.access)?,"operationId":v["operationId"],"generation":v["generation"]});
    let mut visible = result.clone();
    p.access.receipt(&auth.client_id, &mut visible)?;
    persist_inbox_operation(tx, &auth.client_id, "propose", &v, &result)?;
    Ok(visible)
}
impl GardenStore {
    /// List pending v1 proposals for the local owner; proposals remain outside canonical recall.
    pub fn v1_inbox_list(&self) -> Result<Value> {
        let tx = self.db.begin_read().map_err(unavailable)?;
        let proposals = tx
            .values(PROPOSALS)?
            .into_iter()
            .filter(|p| p["status"] == "pending")
            .collect::<Vec<_>>();
        let listed=proposals.iter().take(100).map(|p|json!({"id":p["id"],"title":p["record"]["title"],"actor":p["actor"],"status":p["status"],"revision":p["revision"]})).collect::<Vec<_>>();
        Ok(
            json!({"workspace":metadata(&tx,"workspace")?,"omitted":proposals.len()-listed.len(),"proposals":listed}),
        )
    }
    /// Read an original v1 proposal and current owner conflict information.
    pub fn v1_inbox_get(&self, id: &str) -> Result<Value> {
        contracts::id(id)?;
        let tx = self.db.begin_read().map_err(unavailable)?;
        full_proposal(
            &tx,
            tx.get(PROPOSALS, id)?.ok_or_else(not_found)?,
            &ClientAccess::default(),
        )
    }
    /// Save or dismiss one v1 proposal atomically through qualified owner authority.
    pub fn v1_review_proposal(&self, raw: Value) -> Result<Value> {
        let v = crate::inbox::parse_review(raw)?;
        let mut tx = self.db.begin_write().map_err(unavailable)?;
        tx.set_durability(Durability::Immediate)
            .map_err(unavailable)?;
        identity(&tx, &v)?;
        if let Some(r) = prior(&tx, inbox::OWNER, &v, "v1_review")? {
            return Ok(r);
        }
        let mut p = tx
            .get(PROPOSALS, string(&v, "proposalId")?)?
            .ok_or_else(not_found)?;
        if p["revision"] != v["expectedRevision"] || p["status"] != "pending" {
            return Err(Error::domain(
                "CONFLICT",
                "Proposal already resolved or changed.",
            ));
        }
        let save = v["action"] == "save";
        let receipt = if save {
            identity(&tx, &p)?;
            let draft = v.get("record").unwrap_or(&p["record"]).clone();
            if draft["kind"] != p["record"]["kind"] {
                return Err(Error::domain(
                    "KIND_MISMATCH",
                    "Review cannot change proposal kind.",
                ));
            }
            let proposer: Principal =
                serde_json::from_value(tx.get(PRINCIPALS, string(&p, "actor")?)?.ok_or_else(
                    || Error::domain("UNAUTHORIZED", "Proposer no longer has a current grant."),
                )?)?;
            proposer.access.validate()?;
            let kind = Kind::parse(string(&draft, "kind")?)?;
            if !proposer
                .scopes
                .iter()
                .any(|s| s == "context:propose" || *s == kind.scope())
            {
                return Err(Error::domain(
                    "FORBIDDEN",
                    "Proposer needs context:propose or the corresponding kind write scope.",
                ));
            }
            if let Some(id) = p["targetId"].as_str() {
                proposer.access.require(id)?;
            }
            proposer.access.references(&draft)?;
            if p["targetId"].is_null()
                && duplicate(&tx, &draft, &ClientAccess::default())?.is_some()
            {
                return Err(Error::domain(
                    "DUPLICATE_RECORD",
                    "An exact canonical record already exists. Reconcile before saving.",
                ));
            }
            let mut write = json!({"workspaceId":v["workspaceId"],"generation":v["generation"],"operationId":v["operationId"],"expectedRevision":p["expectedRevision"],"record":draft});
            if !p["targetId"].is_null() {
                write["id"] = p["targetId"].clone();
            }
            Some(write_transaction(
                &tx,
                inbox::OWNER,
                &Write::parse(write)?,
                self.clock.as_ref(),
                true,
            )?)
        } else {
            None
        };
        let status = if save { "saved" } else { "dismissed" };
        let result = json!({"proposalId":p["id"],"status":status,"operationId":v["operationId"],"generation":v["generation"],"receipt":receipt});
        remove(&tx, PENDING, &crate::inbox::dedup(&p))?;
        p["status"] = json!(status);
        p["revision"] = json!(2);
        p["updatedAt"] = json!(
            self.clock
                .now()
                .max(contracts::timestamp(string(&p, "updatedAt")?)?)
                .to_rfc3339_opts(SecondsFormat::Millis, true)
        );
        p["receipt"] = result.clone();
        put(&tx, PROPOSALS, string(&p, "id")?, &p)?;
        persist_inbox_operation(&tx, inbox::OWNER, "review", &v, &result)?;
        tx.commit().map_err(unavailable)?;
        Ok(result)
    }
}

fn history(tx: &redb::ReadTransaction, args: Value) -> Result<Value> {
    contracts::strict(&args, &["id", "revision", "limit"])?;
    let id = string(&args, "id")?;
    contracts::id(id)?;
    let wanted = args
        .get("revision")
        .map(|_| integer(&args, "revision", 1, 9007199254740991))
        .transpose()?;
    let limit = args
        .get("limit")
        .map(|_| integer(&args, "limit", 1, 100))
        .transpose()?;
    if wanted.is_some() && limit.is_some() {
        return Err(invalid("Request one revision or a list, not both"));
    }
    let current = record(tx, id)?;
    let revision = integer(&current, "revision", 1, 9007199254740991)?;
    let w = metadata(tx, "workspace")?;
    let generation = string(&w, "generation")?;
    let mut snapshots: BTreeMap<u64, Value> = BTreeMap::new();
    for entry in tx
        .open_table(OPERATIONS)
        .map_err(unavailable)?
        .iter()
        .map_err(unavailable)?
    {
        let (_, bytes) = entry.map_err(unavailable)?;
        let op: Value = serde_json::from_slice(bytes.value())?;
        if op["receipt"]["record"]["id"] != id {
            continue;
        }
        let rev = integer(&op["receipt"]["record"], "revision", 1, 9007199254740991)?;
        if snapshots
            .get(&rev)
            .is_none_or(|kept| kept["generation"] != generation)
        {
            snapshots.insert(rev, op["receipt"].clone());
        }
    }
    let entry = |rev: u64, s: &Value| {
        let r = &s["record"];
        let content = r["content"].as_str().unwrap_or_default();
        let text = excerpt(content, 400);
        let previous = rev.checked_sub(1).and_then(|p| snapshots.get(&p));
        let changed = previous.map_or(Value::Null, |p| {
            json!(
                [
                    "title",
                    "content",
                    "status",
                    "references",
                    "sources",
                    "basis",
                    "nextStep",
                    "completed",
                    "openQuestions",
                    "mediaType",
                    "destination"
                ]
                .into_iter()
                .filter(|k| p["record"].get(*k) != r.get(*k))
                .collect::<Vec<_>>()
            )
        });
        json!({"revision":rev,"status":r["status"],"title":r["title"],"actor":r["actor"],"committedAt":s["committedAt"],"currentGeneration":s["generation"]==generation,"excerpt":text,"truncated":text.len()<content.len(),"changed":changed})
    };
    if let Some(wanted) = wanted {
        let snapshot = snapshots.get(&wanted).ok_or_else(not_found)?;
        Ok(
            json!({"workspace":w,"id":id,"kind":current["kind"],"currentRevision":revision,"current":wanted==revision,"entry":entry(wanted,snapshot),"record":snapshot["record"]}),
        )
    } else {
        let listed = snapshots
            .iter()
            .rev()
            .take(limit.unwrap_or(20) as usize)
            .map(|(rev, s)| entry(*rev, s))
            .collect::<Vec<_>>();
        let missing_count = revision - snapshots.range(..=revision).count() as u64;
        if missing_count > 1000 {
            return Err(Error::domain(
                "RESOURCE_LIMIT",
                "History has too many missing revisions to list. Request this record with an explicit revision instead.",
            ));
        }
        let mut missing = Vec::with_capacity(missing_count as usize);
        let mut next = 1;
        for retained in snapshots.range(..=revision).map(|(rev, _)| *rev) {
            missing.extend(next..retained);
            next = retained + 1;
        }
        missing.extend(next..=revision);
        Ok(
            json!({"workspace":w,"id":id,"kind":current["kind"],"currentRevision":revision,"currentStatus":current["status"],"returnedRevisions":listed.len(),"omittedRevisions":snapshots.len()-listed.len(),"missingRevisions":missing,"revisions":listed,"note":"Revisions come from retained write receipts, newest first. changed lists the client-chosen fields that differ from the revision before, and is null when that revision has no retained snapshot. Request one revision to read it in full. Earlier text is history, not current fact."}),
        )
    }
}
