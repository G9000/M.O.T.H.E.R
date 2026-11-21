//! Local relationship commands. Every graph read and proposal uses the configured client.
mod view;

use clap::{Args, ValueEnum};
use mother::{
    config::{check_private_output, read_json},
    error::{Error, Result},
    garden::{
        config::{GardenClientConfig, domain_error, load_garden_config},
        contracts::{
            AssertionObject, BatchItem, BatchRequest, ExternalReference, GardenDraft, GardenKind,
            GetRequest, GetResponse, Lifecycle, Mutation, ObservedTime, OutcomeStatus,
            ProposalResponse, QueryRequest, QueryResponse, RecordRef, TimePrecision, ValidTime,
        },
        store::GardenStore,
    },
    service::{LocalClient, load_service_config},
};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::json;
use std::{
    io::{self, IsTerminal},
    path::{Path, PathBuf},
};

#[derive(Debug, Args)]
pub(crate) struct Browse {
    #[arg(long)]
    config: PathBuf,
    /// Read the graph representation of an authorized v1 memory.
    #[arg(long, required_unless_present = "entity", conflicts_with = "entity")]
    memory: Option<String>,
    /// Owner-private memory-to-entity mapping file.
    #[arg(long, requires = "memory")]
    mapping: Option<PathBuf>,
    /// Browse an authorized Garden entity directly.
    #[arg(long)]
    entity: Option<String>,
}

#[derive(Debug, Args)]
pub(crate) struct Suggest {
    #[arg(long)]
    config: PathBuf,
    /// Existing graph entity at the start of the directed assertion.
    #[arg(long)]
    from: String,
    /// Existing graph entity at the end of the directed assertion.
    #[arg(long)]
    to: String,
    #[arg(long, value_enum)]
    relationship: Relationship,
    /// Current authorized evidence object; repeat to attach multiple sources.
    #[arg(long, value_name = "EVIDENCE_UUID")]
    evidence: Vec<String>,
    /// UUID for this exact proposal, generated and retained before submission.
    #[arg(long)]
    operation_id: String,
    /// Correct this existing relationship without retiring either endpoint.
    #[arg(long, requires = "expected_revision")]
    correct: Option<String>,
    /// Expected current revision of the relationship being corrected.
    #[arg(long, requires = "correct")]
    expected_revision: Option<u64>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
enum Relationship {
    RelatedTo,
    InspiredBy,
    Contradicts,
    EvolvedInto,
    Supersedes,
}

impl Relationship {
    fn predicate(self) -> &'static str {
        match self {
            Self::RelatedTo => "living-memory:related_to",
            Self::InspiredBy => "living-memory:inspired_by",
            Self::Contradicts => "living-memory:contradicts",
            Self::EvolvedInto => "living-memory:evolved_into",
            Self::Supersedes => "living-memory:supersedes",
        }
    }
}

const RELATIONSHIPS: [Relationship; 5] = [
    Relationship::RelatedTo,
    Relationship::InspiredBy,
    Relationship::Contradicts,
    Relationship::EvolvedInto,
    Relationship::Supersedes,
];

fn input(message: &str) -> Error {
    Error::domain("INVALID_INPUT", message)
}

fn unavailable() -> Error {
    Error::domain(
        "GRAPH_UNAVAILABLE",
        "This connection has no current graph read access. Use an already-authorized graph connection to view relationships.",
    )
}

fn restart() -> Error {
    Error::domain(
        "RESTART_REQUIRED",
        "Graph permissions or data changed during the read. Refresh to start a new neighborhood.",
    )
}

enum Backend {
    Direct {
        store: Box<GardenStore>,
        config: GardenClientConfig,
    },
    Owner(Box<LocalClient>),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identity {
    workspace_id: String,
    generation: String,
}

pub(crate) struct Connection {
    backend: Backend,
    identity: Identity,
}

impl Connection {
    pub(crate) fn open(path: &Path) -> Result<Self> {
        let backend_kind = read_json(path, 16_384)?;
        let (backend, identity) = match backend_kind["backend"].as_str() {
            Some("garden_redb") => {
                let config = load_garden_config(path)?;
                let store = GardenStore::open_active(&config.database_path)?;
                config.authorize(&store)?;
                let identity = store.identity(&config.auth()).map_err(|error| {
                    let error = domain_error(error);
                    if error.code() == "FORBIDDEN" {
                        unavailable()
                    } else {
                        error
                    }
                })?;
                (
                    Backend::Direct {
                        store: Box::new(store),
                        config,
                    },
                    identity,
                )
            }
            Some("garden_service") => {
                let client = LocalClient::new(load_service_config(path)?)?;
                let capabilities = client.call("mother_get_capabilities", &json!({}))?;
                if capabilities["garden"]["available"] != true {
                    return Err(unavailable());
                }
                (
                    Backend::Owner(Box::new(client)),
                    capabilities["workspace"].clone(),
                )
            }
            _ => {
                return Err(input(
                    "Use an explicit native or local owner service configuration.",
                ));
            }
        };
        Ok(Self {
            backend,
            identity: serde_json::from_value(identity)?,
        })
    }

    fn get(&self, reference: ExternalReference) -> Result<GetResponse> {
        self.get_at(reference, None, None)
    }

    pub(crate) fn get_at(
        &self,
        reference: ExternalReference,
        revision: Option<u64>,
        known_at: Option<&str>,
    ) -> Result<GetResponse> {
        reference.validate().map_err(domain_error)?;
        let request = GetRequest {
            version: 2,
            workspace_id: self.identity.workspace_id.clone(),
            generation: self.identity.generation.clone(),
            required_extensions: Default::default(),
            reference,
            revision,
            known_at: known_at.map(str::to_owned),
        };
        let result = match &self.backend {
            Backend::Direct { store, config } => {
                store.get(&config.auth(), &request).map_err(domain_error)
            }
            Backend::Owner(client) => call(client, "mother_garden_get", &request),
        };
        result.map_err(|error| {
            if error.code() == "FORBIDDEN" {
                unavailable()
            } else {
                error
            }
        })
    }

    pub(crate) fn blank_query(&self) -> QueryRequest {
        QueryRequest {
            version: 2,
            workspace_id: self.identity.workspace_id.clone(),
            generation: self.identity.generation.clone(),
            required_extensions: Default::default(),
            ids: vec![],
            kinds: vec![],
            entity_type: None,
            predicate: None,
            property: None,
            valid_at: None,
            known_at: None,
            text: None,
            vector: None,
            traversal: None,
            limit: 1,
            cursor: None,
        }
    }

    pub(crate) fn query(&self, request: &QueryRequest) -> Result<QueryResponse> {
        match &self.backend {
            Backend::Direct { store, config } => {
                store.query(&config.auth(), request).map_err(domain_error)
            }
            Backend::Owner(client) => call(client, "mother_garden_query", request),
        }
    }

    fn memory(&self, id: &str) -> Result<Option<String>> {
        let args = json!({"id":id});
        let result = match &self.backend {
            Backend::Direct { store, config } => {
                store.v1_call(&config.auth(), "mother_get_record", args)
            }
            Backend::Owner(client) => client.call("mother_get_record", &args),
        };
        match result {
            Ok(record) if record["kind"] == "memory" && record["status"] == "active" => {
                Ok(record["title"].as_str().map(view::clean_label))
            }
            Ok(_) => Ok(None),
            Err(error)
                if matches!(
                    error.code(),
                    "NOT_FOUND" | "FORBIDDEN" | "COMPAT_NOT_INITIALIZED" | "INVALID_INPUT"
                ) =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    fn propose(&self, request: &BatchRequest) -> Result<ProposalResponse> {
        match &self.backend {
            Backend::Direct { store, config } => {
                store.propose(&config.auth(), request).map_err(domain_error)
            }
            Backend::Owner(client) => call(client, "mother_garden_propose", request),
        }
    }
}

fn call<T: DeserializeOwned>(
    client: &LocalClient,
    tool: &str,
    request: &impl serde::Serialize,
) -> Result<T> {
    serde_json::from_value(client.call(tool, &serde_json::to_value(request)?)?).map_err(Into::into)
}

fn reference(kind: GardenKind, id: &str) -> ExternalReference {
    ExternalReference {
        kind,
        id: id.into(),
    }
}

fn existing(kind: GardenKind, id: String) -> RecordRef {
    RecordRef::Existing { kind, id }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MappingFile {
    version: u32,
    workspace_id: String,
    generation: String,
    mappings: Vec<Mapping>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Mapping {
    memory_id: String,
    entity_id: String,
}

fn resolve(conn: &Connection, path: &Path, memory: &str) -> Result<Option<String>> {
    let path = std::path::absolute(path)?;
    check_private_output(&path)?;
    let map: MappingFile = serde_json::from_value(read_json(&path, 64 * 1024)?)?;
    if map.version != 1
        || map.mappings.len() > 256
        || map.workspace_id != conn.identity.workspace_id
        || map.generation != conn.identity.generation
    {
        return Err(input(
            "The mapping file does not match this workspace and restore generation, or exceeds its limits.",
        ));
    }
    let mut ids = std::collections::HashSet::new();
    let mut result = None;
    for binding in map.mappings {
        reference(GardenKind::Entity, &binding.memory_id)
            .validate()
            .map_err(domain_error)?;
        reference(GardenKind::Entity, &binding.entity_id)
            .validate()
            .map_err(domain_error)?;
        if !ids.insert(binding.memory_id.clone()) {
            return Err(input("The mapping file repeats a memory ID."));
        }
        if binding.memory_id == memory {
            result = Some(binding.entity_id);
        }
    }
    Ok(result)
}

pub(crate) fn browse(args: Browse) -> Result<()> {
    let conn = Connection::open(&args.config)?;
    let anchor = if let Some(memory_id) = args.memory.as_deref() {
        reference(GardenKind::Entity, memory_id)
            .validate()
            .map_err(domain_error)?;
        if conn.memory(memory_id)?.is_none() {
            return view::message(
                "mapping_unavailable",
                "This connection cannot read the selected memory and its graph mapping.",
            );
        }
        let Some(path) = args.mapping else {
            return view::message(
                "mapping_missing",
                "Select an owner mapping file for this memory.",
            );
        };
        let Some(entity) = resolve(&conn, &path, memory_id)? else {
            return view::message(
                "mapping_missing",
                "This memory has no entity in the selected owner mapping.",
            );
        };
        entity
    } else {
        args.entity
            .ok_or_else(|| input("Select a memory or entity."))?
    };

    let terminal = io::stdin().is_terminal() && io::stdout().is_terminal();
    let first = match view::read(&conn, &anchor, args.memory.as_deref()) {
        Err(error) if error.code() == "NOT_FOUND" => {
            return view::message(
                "entity_unavailable",
                "This entity is unavailable under the current grant.",
            );
        }
        result => result?,
    };
    if terminal {
        view::run(&conn, first, args.memory)
    } else {
        view::print_json(&first)
    }
}

pub(crate) fn suggest(args: Suggest) -> Result<()> {
    let conn = Connection::open(&args.config)?;
    reference(GardenKind::Assertion, &args.operation_id)
        .validate()
        .map_err(domain_error)?;
    if args.evidence.len() > 8 {
        return Err(input(
            "Attach at most eight evidence objects to one suggestion.",
        ));
    }
    let (valid_time, observed_time, confidence, extensions, evidence) = if let (
        Some(id),
        Some(expected),
    ) =
        (&args.correct, args.expected_revision)
    {
        let response = conn.get(reference(GardenKind::Assertion, id))?;
        let record = response.record;
        if record.metadata.lifecycle != Lifecycle::Active || record.metadata.revision != expected {
            return Err(Error::domain(
                "CONFLICT",
                "The relationship changed. Read its current revision before proposing a new correction.",
            ));
        }
        let GardenDraft::Assertion {
            predicate,
            object: AssertionObject::EntityRef { .. },
            valid_time,
            observed_time,
            confidence,
            extensions,
            evidence,
            ..
        } = record.draft
        else {
            return Err(input("Select an entity-to-entity relationship to correct."));
        };
        if !RELATIONSHIPS
            .iter()
            .any(|relationship| relationship.predicate() == predicate)
        {
            return Err(input(
                "Only Living Memory predicates can be corrected with this command.",
            ));
        }
        (valid_time, observed_time, confidence, extensions, evidence)
    } else {
        (
            ValidTime {
                valid_from: None,
                valid_until: None,
                source_timezone: None,
                precision: TimePrecision::Unknown,
            },
            ObservedTime {
                at: None,
                source_timezone: None,
                precision: TimePrecision::Unknown,
            },
            None,
            Default::default(),
            vec![],
        )
    };
    let draft = GardenDraft::Assertion {
        subject: existing(GardenKind::Entity, args.from),
        predicate: args.relationship.predicate().into(),
        object: AssertionObject::EntityRef {
            entity_ref: existing(GardenKind::Entity, args.to),
        },
        valid_time,
        observed_time,
        confidence,
        extensions,
        evidence: if args.evidence.is_empty() {
            evidence
        } else {
            args.evidence
                .into_iter()
                .map(|id| existing(GardenKind::Evidence, id))
                .collect()
        },
    };
    let (mutation, id) = match args.correct {
        Some(id) => (
            Mutation::Update,
            Some(reference(GardenKind::Assertion, &id)),
        ),
        None => (Mutation::Create, None),
    };
    let request = BatchRequest {
        version: 2,
        workspace_id: conn.identity.workspace_id.clone(),
        generation: conn.identity.generation.clone(),
        operation_id: args.operation_id,
        required_extensions: Default::default(),
        items: vec![BatchItem {
            item_key: "relationship".into(),
            mutation,
            id,
            expected_revision: args.expected_revision.unwrap_or(0),
            draft: Some(draft),
            replacement: None,
        }],
    };
    let value = serde_json::to_value(&request)?;
    mother::garden::contracts::GardenRequest::parse(
        mother::garden::contracts::GardenTool::Propose,
        &value,
    )
    .map_err(domain_error)?;
    let proposal = conn.propose(&request).map_err(|error| {
        if error.code() == "STORE_UNAVAILABLE" {
            Error::domain("PROPOSAL_UNCONFIRMED", format!("Check the original graph receipt with operation ID {} and generation {} before resubmitting.", request.operation_id, request.generation))
        } else { error }
    })?;
    if proposal.status != OutcomeStatus::Pending {
        return Err(input(
            "The proposal admission did not return a pending receipt.",
        ));
    }
    view::print_json(&json!({
        "status":"admission_recorded",
        "admissionStatus":"pending",
        "proposalId":proposal.proposal_id,
        "operationId":request.operation_id,
        "operationGeneration":request.generation,
        "next":"Use mother_garden_get_proposal for the current outcome. If still pending, review it with mother garden inbox --db <garden.redb>."
    }))
}
