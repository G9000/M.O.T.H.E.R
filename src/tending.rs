//! On-demand Memory Gardening with an owner-private disposition journal.
mod findings;

use clap::Args;
use findings::{Finding, assess};
use fs2::FileExt;
use mother::{
    config::{check_private_output, read_json},
    error::{Error, Result},
    garden::{config::load_garden_config, store::GardenStore},
    service::{LocalClient, load_service_config},
};
use serde_json::{Value, json};
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;

const REPORT_LIMIT: usize = 12;

#[derive(Debug, Args)]
pub(crate) struct Tend {
    #[arg(long)]
    config: PathBuf,
    /// Existing owner-private disposition directory for this workspace and reviewer.
    #[arg(long)]
    state: PathBuf,
    /// Limit the authorized context slice when no explicit source IDs are selected.
    #[arg(long, default_value_t = 80, value_parser = clap::value_parser!(u32).range(1..=100))]
    limit: u32,
    /// Review one specific record ID. Repeat to select a bounded collection.
    #[arg(long, value_name = "RECORD_UUID")]
    source: Vec<String>,
    /// Open tasks past this many UTC calendar days without an update are flagged.
    #[arg(long, default_value_t = 30, value_parser = clap::value_parser!(i64).range(1..=3650))]
    stale_days: i64,
    /// Optional UTC reference date, suitable for a reproducible pass.
    #[arg(long)]
    as_of: Option<chrono::NaiveDate>,
    /// Remember that this finding was dismissed locally.
    #[arg(long, conflicts_with = "propose")]
    dismiss: Option<Uuid>,
    /// Submit exactly this finding to the existing Living Memory inbox.
    #[arg(long)]
    propose: Option<Uuid>,
}

enum Backend {
    Direct {
        store: Box<GardenStore>,
        auth: mother::contracts::Auth,
    },
    Service(Box<LocalClient>),
}

struct Connection {
    backend: Backend,
    workspace: String,
    generation: String,
    reviewer: String,
}

impl Connection {
    fn open(path: &Path) -> Result<Self> {
        let config = read_json(path, 16_384)?;
        let (backend, reviewer, identity) = match config["backend"].as_str() {
            Some("garden_redb") => {
                let config = load_garden_config(path)?;
                let store = GardenStore::open_active(&config.database_path)?;
                let auth = config.auth();
                let identity = store.v1_identity(&auth)?;
                let reviewer = auth.client_id.clone();
                (
                    Backend::Direct {
                        store: Box::new(store),
                        auth,
                    },
                    reviewer,
                    identity,
                )
            }
            Some("garden_service") => {
                let config = load_service_config(path)?;
                let reviewer = config.client_id.clone();
                let client = LocalClient::new(config)?;
                let cap = client.call("mother_get_capabilities", &json!({}))?;
                if cap["v1"]["available"] != true {
                    return Err(input(
                        "Memory Gardening needs an authorized v1 context connection.",
                    ));
                }
                (
                    Backend::Service(Box::new(client)),
                    reviewer,
                    cap["workspace"].clone(),
                )
            }
            _ => {
                return Err(input(
                    "Use an explicit native or local owner service configuration.",
                ));
            }
        };
        let identity_value = |name| {
            identity[name]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| input("The current v1 workspace identity is unavailable."))
        };
        Ok(Self {
            backend,
            reviewer,
            workspace: identity_value("workspaceId")?,
            generation: identity_value("generation")?,
        })
    }

    fn call(&self, tool: &str, args: Value) -> Result<Value> {
        match &self.backend {
            Backend::Direct { store, auth } => store.v1_call(auth, tool, args),
            Backend::Service(client) => client.call(tool, &args),
        }
    }

    fn read(&self, id: &str) -> Result<Option<Value>> {
        match self.call("mother_get_record", json!({"id":id})) {
            Ok(value) => Ok(Some(value)),
            Err(error) if error.code() == "NOT_FOUND" => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn receipt(&self, id: &str) -> Result<Option<Value>> {
        match self.call("mother_get_receipt", json!({"operationId":id})) {
            Ok(value)
                if value["proposal"]["id"].is_string()
                    && value["operationId"] == id
                    && value["generation"] == self.generation =>
            {
                Ok(Some(value))
            }
            Ok(_) => Err(input(
                "The finding operation is already used by another request.",
            )),
            Err(error) if error.code() == "NOT_FOUND" => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn reviewed_sources(&self, args: &Tend) -> Result<Vec<Value>> {
        let ids = if args.source.is_empty() {
            let context = self.call("mother_get_context", json!({"limit":args.limit}))?;
            let Some(records) = context["records"].as_array() else {
                return Err(input("The authorized context response is unavailable."));
            };
            records
                .iter()
                .filter_map(|r| r["id"].as_str())
                .map(str::to_owned)
                .collect::<Vec<_>>()
        } else {
            args.source.clone()
        };
        let mut unique = std::collections::BTreeSet::new();
        let mut records = Vec::new();
        for id in ids {
            if Uuid::parse_str(&id).is_err() {
                return Err(input("Source IDs must be UUIDs."));
            }
            if unique.insert(id.clone())
                && let Some(record) = self.read(&id)?
            {
                records.push(record);
            }
        }
        Ok(records)
    }

    fn current(&self, finding: &Finding) -> Result<bool> {
        for source in &finding.evidence {
            let Some(record) = self.read(&source.id)? else {
                return Ok(false);
            };
            if record["revision"].as_u64() != Some(source.revision)
                || matches!(
                    record["status"].as_str(),
                    Some("retracted" | "superseded" | "cancelled")
                )
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn propose(&self, finding: &Finding) -> Result<Value> {
        if let Backend::Service(client) = &self.backend
            && client.call("mother_get_capabilities", &json!({}))?["v1"]["gardeningEvidenceReview"]
                != 1
        {
            return Err(Error::domain(
                "GARDENING_REVIEW_UNAVAILABLE",
                "The running owner service does not support revision-bound gardening review.",
            ));
        }
        let id = &finding.id;
        let source = mother::gardening::source(
            finding
                .evidence
                .iter()
                .map(|source| (source.id.as_str(), source.revision)),
        );
        let references: Vec<_> = finding.evidence.iter().map(|source| &source.id).collect();
        let request = json!({
            "operationId":id,"workspaceId":self.workspace,"generation":self.generation,"expectedRevision":0,
            "source":source,"basis":"model_inference","record":{
                "kind":"memory","status":"active","basis":"model_inference",
                "title":format!("Gardening annotation | {}",finding.kind.label()),
                "content":format!("Reviewing agent: {}\nFinding: {}\n\nSuggested interpretation. {}\n\nReview question: {}",
                    self.reviewer,id,finding.reason,finding.suggested_action),
                "references":references,
            }
        });
        match self.call("mother_propose_memory", request) {
            Ok(value) => Ok(value),
            Err(error)
                if error.code() == "STORE_UNAVAILABLE" || matches!(error, Error::Json(_)) =>
            {
                match self.receipt(id) {
                    Ok(Some(value)) => Ok(value),
                    _ => Err(Error::domain(
                        "GARDENING_UNCONFIRMED",
                        "The original admission is unconfirmed. Rerun the same finding and scope to reconcile its original receipt before retrying.",
                    )),
                }
            }
            Err(error) => Err(error),
        }
    }
}

fn input(message: &str) -> Error {
    Error::domain("INVALID_INPUT", message)
}

struct Journal {
    root: PathBuf,
    _lock: File,
}

impl Journal {
    fn open(path: &Path, connection: &Connection) -> Result<Self> {
        let root = std::path::absolute(path)?;
        let lock_path = root.join("gardening.lock");
        check_private_output(&lock_path)?;
        let lock = match mother::config::create_private_file(&lock_path) {
            Ok(file) => file,
            Err(Error::Io(error)) if error.kind() == io::ErrorKind::AlreadyExists => {
                check_private_output(&lock_path)?;
                OpenOptions::new().read(true).write(true).open(&lock_path)?
            }
            Err(error) => return Err(error),
        };
        FileExt::try_lock_exclusive(&lock).map_err(|_| {
            Error::domain(
                "GARDENING_BUSY",
                "Another pass uses this state directory. Retry after it finishes.",
            )
        })?;
        let instance = Self { root, _lock: lock };
        let identity = json!({"version":1,"workspaceId":connection.workspace,"generation":connection.generation,"reviewer":connection.reviewer});
        let path = instance.root.join("identity.json");
        match read_state(&path)? {
            Some(value) if value != identity => {
                return Err(input(
                    "The gardening state belongs to another workspace, restore generation, or reviewer.",
                ));
            }
            None => {
                for entry in std::fs::read_dir(&instance.root)? {
                    if entry?.file_name() != "gardening.lock" {
                        return Err(input(
                            "Use an empty private directory for new gardening state.",
                        ));
                    }
                }
                atomic_write(&path, &identity)?;
            }
            _ => {}
        }
        Ok(instance)
    }

    fn dismiss_path(&self, id: &str) -> PathBuf {
        self.root.join(format!("dismissed-{id}.json"))
    }

    fn dismissed(&self, id: &str) -> Result<bool> {
        match read_state(&self.dismiss_path(id))? {
            Some(record) if record == json!({"findingId":id,"status":"dismissed"}) => Ok(true),
            Some(_) => Err(input("A gardening disposition is invalid.")),
            None => Ok(false),
        }
    }

    fn dismiss(&self, id: &str) -> Result<()> {
        atomic_write(
            &self.dismiss_path(id),
            &json!({"findingId":id,"status":"dismissed"}),
        )
    }

    fn admitted(&self, id: &str) -> Result<bool> {
        match read_state(&self.root.join(format!("admitted-{id}.json")))? {
            Some(record) if record == json!({"findingId":id,"status":"admission_recorded"}) => {
                Ok(true)
            }
            Some(_) => Err(input("A gardening disposition is invalid.")),
            None => Ok(false),
        }
    }

    fn mark_admitted(&self, id: &str) -> Result<()> {
        if self.admitted(id)? {
            return Ok(());
        }
        atomic_write(
            &self.root.join(format!("admitted-{id}.json")),
            &json!({"findingId":id,"status":"admission_recorded"}),
        )
    }
}

fn read_state(path: &Path) -> Result<Option<Value>> {
    check_private_output(path)?;
    match read_json(path, 8_192) {
        Ok(value) => Ok(Some(value)),
        Err(Error::Io(error)) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn atomic_write(destination: &Path, value: &Value) -> Result<()> {
    check_private_output(destination)?;
    let parent = destination
        .parent()
        .ok_or_else(|| input("The gardening state requires a private directory."))?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    serde_json::to_writer(temporary.as_file_mut(), value)?;
    temporary.as_file_mut().write_all(b"\n")?;
    temporary.as_file().sync_all()?;
    temporary
        .persist_noclobber(destination)
        .map_err(|error| error.error)?;
    #[cfg(unix)]
    File::open(parent)?.sync_all()?;
    Ok(())
}

fn output(value: Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

pub(crate) fn run(args: Tend) -> Result<()> {
    if args.source.len() > 100 {
        return Err(input("Select at most one hundred source IDs."));
    }
    let conn = Connection::open(&args.config)?;
    let journal = Journal::open(&args.state, &conn)?;
    let today = args
        .as_of
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let records = conn.reviewed_sources(&args)?;
    let findings = assess(
        &records,
        &conn.workspace,
        &conn.generation,
        today,
        args.stale_days,
    );
    if let Some(selected) = args.dismiss.or(args.propose) {
        let id = selected.to_string();
        if journal.dismissed(&id)? {
            return output(json!({"status":"dismissed","findingId":id}));
        }
        let finding = findings.iter().find(|finding| finding.id == id)
            .ok_or_else(|| Error::domain("FINDING_UNAVAILABLE","The finding is unavailable in this authorized pass. Run a fresh review with the intended scope."))?;
        if !conn.current(finding)? {
            return Err(Error::domain(
                "FINDING_UNAVAILABLE",
                "The source revisions are no longer current or authorized.",
            ));
        }
        if let Some(value) = conn.receipt(&id)? {
            journal.mark_admitted(&id)?;
            return output(
                json!({"status":"admission_recorded","findingId":id,"proposalId":value["proposal"]["id"],
                "next":"Inspect its current outcome in the Living Memory inbox."}),
            );
        }
        if args.dismiss.is_some() {
            journal.dismiss(&id)?;
            return output(json!({"status":"dismissed","findingId":id}));
        }
        let proposal = conn.propose(finding)?;
        if proposal["proposal"]["id"]
            .as_str()
            .and_then(|id| Uuid::parse_str(id).ok())
            .is_none()
        {
            return Err(Error::domain(
                "GARDENING_UNCONFIRMED",
                "The original admission is unconfirmed. Reconcile its original receipt before retrying.",
            ));
        }
        journal.mark_admitted(&id)?;
        return output(
            json!({"status":"admission_recorded","findingId":id,"proposalId":proposal["proposal"]["id"],
            "next":"Open mother garden v1-inbox --db <garden.redb> for owner review."}),
        );
    }

    let mut available = Vec::new();
    let mut has_more = false;
    for finding in findings {
        if !journal.dismissed(&finding.id)?
            && !journal.admitted(&finding.id)?
            && conn.current(&finding)?
            && conn.receipt(&finding.id)?.is_none()
        {
            if available.len() == REPORT_LIMIT {
                has_more = true;
                break;
            }
            available.push(finding);
        }
    }
    output(json!({
        "reviewingAgent":conn.reviewer,"asOf":today,
        "scope":"Bounded authorized slice. Possible gaps make no claim about unreviewed, unavailable or hidden material.",
        "hasMoreFindings":has_more,
        "findings":available,
    }))
}
