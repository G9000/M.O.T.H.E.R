//! SQLite exists only in the explicit legacy-conversion build.
use super::*;
use crate::{contracts, store};
use rusqlite::{Connection, OpenFlags};
use serde_json::{Value, json};
use std::{collections::BTreeSet, time::Duration};

struct Snapshot {
    bundle: Value,
    restrictions: Vec<GrantRestriction>,
    fingerprint: String,
}
fn digest(value: &Value) -> Result<String> {
    Ok(contracts::hash(&serde_json::to_string(value)?))
}
fn normalized_bundle(mut value: Value, generation: Option<&str>) -> Value {
    if let Some(object) = value.as_object_mut() {
        object.remove("exportedAt");
    }
    if let Some(generation) = generation {
        value["workspace"]["generation"] = json!(generation);
    }
    value
}
fn rows(conn: &Connection, sql: &str) -> Result<Vec<Value>> {
    let mut query = conn.prepare(sql)?;
    query
        .query_map([], |row| row.get::<_, String>(0))?
        .map(|row| Ok(serde_json::from_str(&row?)?))
        .collect()
}
fn table_exists(conn: &Connection, table: &str) -> Result<bool> {
    Ok(conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?)",
        [table],
        |row| row.get(0),
    )?)
}
fn requirement(conn: &Connection, table: &str, client: &str) -> Result<bool> {
    if !table_exists(conn, table)? {
        return Ok(false);
    }
    // Table is selected solely from static owner-code literals below.
    Ok(conn.query_row(
        &format!("SELECT EXISTS(SELECT 1 FROM {table} WHERE client_id=?)"),
        [client],
        |row| row.get(0),
    )?)
}
fn snapshot(source: &Path) -> Result<Snapshot> {
    safe_path(source)?;
    crate::private_paths::single_link(source)?;
    let conn = Connection::open_with_flags(
        source,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    conn.busy_timeout(Duration::from_secs(5))?;
    let tx = conn.unchecked_transaction()?;
    let version: u32 = tx.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if version != 1 {
        return Err(problem(
            "UNSUPPORTED_VERSION",
            "Migration requires the recognized v1 SQLite schema.",
        ));
    }
    let names: Vec<String> = tx
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
        .query_map([], |row| row.get(0))?
        .collect::<std::result::Result<_, _>>()?;
    for name in names {
        if !matches!(
            name.as_str(),
            "metadata"
                | "records"
                | "operations"
                | "clients"
                | "client_access"
                | "client_policy_requirements"
                | "client_garden_access"
                | "client_garden_policy_requirements"
                | "inbox_proposals"
                | "inbox_operations"
                | "pins"
                | "assignments"
        ) {
            return Err(problem(
                "UNSUPPORTED_EXTENSION",
                "Unknown SQLite extension table; migration refused.",
            ));
        }
    }
    let metadata_keys: Vec<String> = tx
        .prepare("SELECT key FROM metadata ORDER BY key")?
        .query_map([], |row| row.get(0))?
        .collect::<std::result::Result<_, _>>()?;
    if metadata_keys
        .iter()
        .any(|key| !matches!(key.as_str(), "workspace" | "persona" | "usage"))
    {
        return Err(problem(
            "UNSUPPORTED_EXTENSION",
            "Unknown SQLite canonical metadata; migration refused.",
        ));
    }
    let records = rows(&tx, "SELECT payload FROM records ORDER BY id")?;
    let mut query = tx.prepare("SELECT actor,generation,operation_id,request_hash,receipt FROM operations ORDER BY actor,generation,operation_id")?;
    let operations = query.query_map([], |row| Ok((row.get::<_,String>(0)?,row.get::<_,String>(1)?,row.get::<_,String>(2)?,row.get::<_,String>(3)?,row.get::<_,String>(4)?)))?
        .map(|row| { let (actor,generation,operation,hash,receipt) = row?; Ok(json!({"actor":actor,"generation":generation,"operationId":operation,"requestHash":hash,"receipt":serde_json::from_str::<Value>(&receipt)?})) }).collect::<Result<Vec<_>>>()?;
    drop(query);
    let inbox = crate::inbox::export(&tx)?;
    let mut sources = BTreeSet::new();
    let mut visit = |record: &Value| {
        for source in record["sources"].as_array().into_iter().flatten() {
            if let Some(uri) = source["uri"].as_str() {
                sources.insert(uri.to_owned());
            }
        }
    };
    for record in &records {
        visit(record);
    }
    for operation in &operations {
        visit(&operation["receipt"]["record"]);
    }
    if let Some(inbox) = &inbox {
        for proposal in inbox["proposals"].as_array().into_iter().flatten() {
            visit(&proposal["record"]);
            visit(&proposal["before"]);
        }
    }
    let mut bundle = json!({"schemaVersion":1,"exportedAt":store::now(),"workspace":store::workspace(&tx)?,"persona":store::persona(&tx)?,"records":records,"operations":operations,"omittedExternalSources":sources});
    if let Some(inbox) = inbox {
        bundle["inbox"] = inbox;
    }
    if let Some(pins) = crate::pins::export(&tx)? {
        bundle["pins"] = pins;
    }
    if let Some(assignments) = crate::handoffs::export(&tx)? {
        bundle["assignments"] = assignments;
    }
    let bundle = crate::portability::validate(bundle)?;
    let mut query = tx.prepare("SELECT id,scopes,revoked FROM clients ORDER BY id")?;
    let clients = query
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
            ))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    drop(query);
    let mut restrictions = Vec::new();
    for (client, raw, revoked) in clients {
        let scopes: Vec<String> = serde_json::from_str(&raw)?;
        if !contracts::valid_client(&client)
            || scopes.is_empty()
            || scopes.iter().any(|scope| !contracts::valid_scope(scope))
        {
            return Err(problem(
                "INVALID_INPUT",
                "Invalid source grant restrictions.",
            ));
        }
        let policy_required = requirement(&tx, "client_policy_requirements", &client)?;
        let garden_required = requirement(&tx, "client_garden_policy_requirements", &client)?;
        let access = crate::access::load(&tx, &client)?;
        let garden = if garden_required
            || scopes
                .iter()
                .any(|scope| contracts::GARDEN_SCOPES.contains(&scope.as_str()))
        {
            Some(crate::garden::access::load(&tx, &client)?)
        } else {
            None
        };
        restrictions.push(GrantRestriction {
            client_id: client,
            scopes,
            access,
            garden,
            policy_required,
            garden_required,
            revoked,
        });
    }
    // Also detect source-key rotations without retaining any credential material.
    let mut keys = tx.prepare("SELECT id,token_hash FROM clients ORDER BY id")?;
    let key_material = keys
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let key_digest = contracts::hash(&serde_json::to_string(&key_material)?);
    drop(keys);
    let fingerprint = digest(
        &json!({"bundle":normalized_bundle(bundle.clone(), None),"restrictions":restrictions,"credentialRotationDigest":key_digest}),
    )?;
    tx.commit()?;
    Ok(Snapshot {
        bundle,
        restrictions,
        fingerprint,
    })
}
fn validate_destination(state: &mut MigrationStatus, expected: &Value) -> Result<()> {
    let destination = GardenStore::open(&state.request.destination).map_err(domain_error)?;
    let actual = destination.v1_export()?;
    if normalized_bundle(actual.clone(), Some(&state.source_generation))
        != normalized_bundle(expected.clone(), None)
    {
        return Err(problem(
            "VALIDATION_FAILED",
            "Destination v1 state does not exactly match the verified source.",
        ));
    }
    let generation = contracts::string(&actual["workspace"], "generation")?;
    if generation == state.source_generation {
        return Err(problem(
            "GENERATION_MISMATCH",
            "Migration destination must have a fresh generation.",
        ));
    }
    state.destination_generation = Some(generation.into());
    state.destination_fingerprint =
        Some(destination.migration_fingerprint().map_err(domain_error)?);
    Ok(())
}
/// Prepare and validate a new, fenced destination from a consistent read-only snapshot.
/// # Errors
/// Existing unrelated files, changing requests, unsupported extensions and invalid state fail.
pub fn prepare(request: MigrationRequest) -> Result<MigrationStatus> {
    prepare_with_hook(request, |_| Ok(()))
}
/// Prepare with trusted failure-injection barriers, never exposed to MCP callers.
/// # Errors
/// Propagates validation, storage and instrumentation failures; retry the identical request.
#[doc(hidden)]
pub fn prepare_with_hook(
    mut request: MigrationRequest,
    mut hook: impl FnMut(MigrationStage) -> Result<()>,
) -> Result<MigrationStatus> {
    contracts::id(&request.operation_id)?;
    request.source = normalized(&request.source)?;
    request.destination = normalized(&request.destination)?;
    request.journal = normalized(&request.journal)?;
    if request.source == request.destination
        || request.source == request.journal
        || request.destination == request.journal
    {
        return Err(problem(
            "INVALID_INPUT",
            "Source, destination and journal must be distinct.",
        ));
    }
    let exists = request.journal.exists();
    if !exists {
        check_source(&request.source)?;
        if request.destination.exists() || sidecar(&request.destination).exists() {
            return Err(problem("CONFLICT", "Migration destination already exists."));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            fs::DirBuilder::new().mode(0o700).create(&request.journal)?;
        }
        #[cfg(not(unix))]
        fs::create_dir(&request.journal)?;
    }
    crate::private_paths::owner_directory(&request.journal)?;
    let _lock = lock(&request.journal)?;
    let mut previous = if exists {
        match status(&request.journal) {
            Ok(state) => Some(state),
            Err(error) if error.code() == "NOT_FOUND" => {
                // An interruption before the initial publication has no committed identity.
                if request.destination.exists() || sidecar(&request.destination).exists() {
                    return Err(fenced());
                }
                None
            }
            Err(error) => return Err(error),
        }
    } else {
        None
    };
    if let Some(state) = &previous {
        if state.request != request {
            return Err(problem(
                "OPERATION_REUSED",
                "A cutover journal cannot be reused with another request.",
            ));
        }
        if state.phase != MigrationPhase::Preparing {
            return Ok(state.clone());
        }
    }
    check_source(&request.source)?;
    let source = snapshot(&request.source)?;
    let mut state = if let Some(state) = previous.take() {
        if state.source_fingerprint != source.fingerprint {
            return Err(problem(
                "SOURCE_CHANGED",
                "Source changed since preparation; prepare a new journal and operation UUID.",
            ));
        }
        state
    } else {
        let mut archive_name = request.source.as_os_str().to_os_string();
        archive_name.push(format!(".archive-{}", request.operation_id));
        let state = MigrationStatus {
            version: 1,
            revision: 0,
            request: request.clone(),
            phase: MigrationPhase::Preparing,
            workspace_id: contracts::string(&source.bundle["workspace"], "id")?.into(),
            source_generation: contracts::string(&source.bundle["workspace"], "generation")?.into(),
            destination_generation: None,
            source_fingerprint: source.fingerprint,
            destination_fingerprint: None,
            source_archive_fingerprint: None,
            archive: PathBuf::from(archive_name),
            source_readonly: fs::metadata(&request.source)?.permissions().readonly(),
            restrictions: source.restrictions,
        };
        if state.archive.exists() {
            return Err(problem("CONFLICT", "The cutover archive already exists."));
        }
        publish_json(&request.journal.join("000000.json"), &state)?;
        state
    };
    hook(MigrationStage::Preparing)?;
    write_fence(&request.destination, &state, "destination")?;
    if !request.destination.exists() {
        let parent = request.destination.parent().ok_or_else(fenced)?;
        let staging = tempfile::Builder::new()
            .prefix(".migration-stage-")
            .tempdir_in(parent)?;
        let store = GardenStore::initialize(
            staging.path().join("import.redb"),
            &state.workspace_id,
            &state.source_generation,
        )
        .map_err(domain_error)?;
        store.import_v1_bundle(source.bundle.clone())?;
        let mut actual = store.v1_export()?;
        actual["exportedAt"] = source.bundle["exportedAt"].clone();
        if actual != source.bundle {
            return Err(problem(
                "VALIDATION_FAILED",
                "Staging import changed source state.",
            ));
        }
        let bundle = staging.path().join("verified-bundle");
        store.backup_to(&bundle).map_err(domain_error)?;
        drop(store);
        GardenStore::verify_backup(&bundle).map_err(domain_error)?;
        drop(GardenStore::restore_backup(&bundle, &request.destination).map_err(domain_error)?);
    }
    validate_destination(&mut state, &source.bundle)?;
    hook(MigrationStage::DestinationValidated)?;
    append(&mut state, MigrationPhase::Prepared)?;
    Ok(state)
}
fn sqlite_sidecar(source: &Path, suffix: &str) -> PathBuf {
    let mut name = source.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}
fn nonempty_wal(source: &Path) -> Result<bool> {
    match fs::metadata(sqlite_sidecar(source, "-wal")) {
        Ok(meta) => Ok(meta.len() != 0),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}
/// Fence the final matching source and explicitly activate its independently validated destination.
/// # Errors
/// Writers, changed source state, incomplete archive moves and altered destinations fail closed.
pub fn activate(journal: &Path, operation: &str) -> Result<MigrationStatus> {
    activate_with_hook(journal, operation, |_| Ok(()))
}
/// Activate with trusted failure-injection barriers for crash/retry qualification.
/// # Errors
/// Propagates validation and instrumentation failure; a published source fence remains active.
#[doc(hidden)]
pub fn activate_with_hook(
    journal: &Path,
    operation: &str,
    mut hook: impl FnMut(MigrationStage) -> Result<()>,
) -> Result<MigrationStatus> {
    let _lock = lock(journal)?;
    let mut state = status(journal)?;
    check_operation(&state, operation)?;
    if state.phase == MigrationPhase::Activated {
        check_destination(&state.request.destination)?;
        return Ok(state);
    }
    if !matches!(
        state.phase,
        MigrationPhase::Prepared | MigrationPhase::Fenced
    ) {
        return Err(fenced());
    }
    crate::private_paths::single_link(&state.request.destination)?;
    let destination = GardenStore::open(&state.request.destination).map_err(domain_error)?;
    if Some(destination.migration_fingerprint().map_err(domain_error)?)
        != state.destination_fingerprint
    {
        return Err(problem(
            "VALIDATION_FAILED",
            "Prepared destination changed before activation.",
        ));
    }
    if state.request.source.exists() {
        if nonempty_wal(&state.request.source)? {
            return Err(problem(
                "SOURCE_NOT_QUIESCENT",
                "Stop source writers and close their connections before activation.",
            ));
        }
        let mut conn = Connection::open_with_flags(
            &state.request.source,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        conn.busy_timeout(Duration::from_secs(5))?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        if nonempty_wal(&state.request.source)? {
            return Err(problem(
                "SOURCE_NOT_QUIESCENT",
                "Source still has an active write-ahead log.",
            ));
        }
        let source = snapshot(&state.request.source)?;
        if source.fingerprint != state.source_fingerprint {
            return Err(problem(
                "SOURCE_CHANGED",
                "Source changed since preparation; prepare a new journal and operation UUID.",
            ));
        }
        state.source_archive_fingerprint = Some(file_fingerprint(&state.request.source)?);
        write_fence(&state.request.source, &state, "source")?;
        if state.phase != MigrationPhase::Fenced {
            append(&mut state, MigrationPhase::Fenced)?;
        }
        hook(MigrationStage::SourceFenced)?;
        // Release without a commit: this connection exists only to hold admission.
        tx.rollback()?;
        conn.close().map_err(|(_, error)| Error::from(error))?;
        for suffix in ["-wal", "-shm", "-journal"] {
            if sqlite_sidecar(&state.request.source, suffix).exists() {
                return Err(problem(
                    "SOURCE_NOT_QUIESCENT",
                    "Source is fenced; close remaining SQLite connections and retry activation.",
                ));
            }
        }
        if state.archive.exists() {
            return Err(problem(
                "RECONCILIATION_REQUIRED",
                "Both original source and archive exist.",
            ));
        }
        write_fence(&state.archive, &state, "archive")?;
        fs::rename(&state.request.source, &state.archive)?;
        sync_directory(state.archive.parent().ok_or_else(fenced)?)?;
        hook(MigrationStage::SourceArchived)?;
    } else if !state.archive.exists() {
        return Err(problem(
            "RECONCILIATION_REQUIRED",
            "Neither the source nor its archive is available.",
        ));
    }
    // Recovery after a rename revalidates the archived authority before activation.
    if snapshot(&state.archive)?.fingerprint != state.source_fingerprint {
        return Err(problem(
            "RECONCILIATION_REQUIRED",
            "Archived source does not match the prepared snapshot.",
        ));
    }
    let mut permissions = fs::metadata(&state.archive)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&state.archive, permissions)?;
    append(&mut state, MigrationPhase::Activated)?;
    hook(MigrationStage::Activated)?;
    drop(destination);
    Ok(state)
}
