//! Owner commands for visual evidence. Every file path here is chosen by the owner
//! on the command line; no MCP request can name a path or trigger these commands.
use clap::{Args, Subcommand};
use mother::{
    error::{Error, Result},
    garden::{
        config::domain_error,
        contracts::{ObservedTime, TimePrecision},
        media_tools::{
            AcquisitionInput, AttachRequest, AttachSource, AttachmentReview, DEFAULT_MAX_BYTES,
            IngestOptions, MediaOrigin,
        },
        store::GardenStore,
    },
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
pub(crate) struct Media {
    #[command(subcommand)]
    command: Box<MediaCommand>,
}

#[derive(Debug, Subcommand)]
enum MediaCommand {
    /// Ingest one image with provenance; the original bytes are never modified.
    Add {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        file: PathBuf,
        #[command(flatten)]
        provenance: Provenance,
    },
    /// Attach a new or already ingested image to an assertion as evidence.
    Attach {
        #[arg(long)]
        db: PathBuf,
        #[arg(long, required_unless_present = "sha256", conflicts_with = "sha256")]
        file: Option<PathBuf>,
        /// Attach an asset that is already in this Garden.
        #[arg(long)]
        sha256: Option<String>,
        /// Assertion ID that gains the image as evidence.
        #[arg(long)]
        to: String,
        /// Current revision of that assertion; a stale value changes nothing.
        #[arg(long)]
        expected_revision: u64,
        /// Why this image supports the assertion.
        #[arg(long)]
        note: Option<String>,
        #[command(flatten)]
        provenance: Provenance,
    },
    /// Approve a proposed attachment by supplying the original file.
    Complete {
        #[arg(long)]
        db: PathBuf,
        /// Pending proposal ID from the graph inbox.
        #[arg(long)]
        proposal: String,
        #[arg(long)]
        file: PathBuf,
        /// Proposal revision shown in the inbox.
        #[arg(long, default_value_t = 1)]
        expected_revision: u64,
        /// Also keep the proposer's description as an attributed interpretation.
        #[arg(long)]
        accept_description: bool,
        #[command(flatten)]
        provenance: Provenance,
    },
    /// List stored images, or those attached to one assertion.
    List {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        record: Option<String>,
    },
    /// Re-hash stored images and report missing, corrupt or dangling media.
    Verify {
        #[arg(long)]
        db: PathBuf,
    },
    /// Write every stored image plus a manifest into a new private directory.
    ExportAll {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Verify and add images, provenance and derivative links from a media export.
    Import {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        dir: PathBuf,
    },
    /// Write one stored image to a new private file for the owner to open.
    Export {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        sha256: String,
        #[arg(long)]
        out: PathBuf,
    },
}

#[derive(Debug, Args)]
struct Provenance {
    /// captured, uploaded, imported, screenshot or ai_generated.
    #[arg(long, default_value = "imported")]
    origin: String,
    /// Where the image came from, in the owner's words.
    #[arg(long)]
    source: Option<String>,
    /// When the pictured event happened (RFC 3339).
    #[arg(long)]
    event_time: Option<String>,
    /// Precision of the event time: year, month, day, minute, second or instant.
    #[arg(long, default_value = "instant", requires = "event_time")]
    event_precision: String,
    /// Reject the file unless its bytes are this media type.
    #[arg(long)]
    media_type: Option<String>,
    /// Also store a copy without location, device and other embedded metadata.
    #[arg(long)]
    strip_metadata: bool,
    #[arg(long, default_value_t = DEFAULT_MAX_BYTES)]
    max_bytes: u64,
    /// Do not record the original filename.
    #[arg(long)]
    no_filename: bool,
}
impl Provenance {
    fn input(&self, file: Option<&Path>) -> Result<(AcquisitionInput, IngestOptions)> {
        let mut input =
            AcquisitionInput::new(MediaOrigin::parse(&self.origin).map_err(domain_error)?);
        input.source.clone_from(&self.source);
        input.declared_media_type.clone_from(&self.media_type);
        if !self.no_filename {
            input.original_filename = file
                .and_then(Path::file_name)
                .and_then(|name| name.to_str())
                .map(str::to_owned);
        }
        if let Some(at) = &self.event_time {
            let precision: TimePrecision = serde_json::from_value(json!(self.event_precision))
                .map_err(|_| Error::domain("INVALID_INPUT", "Unknown event time precision."))?;
            input.event_time = Some(ObservedTime {
                at: Some(at.clone()),
                source_timezone: None,
                precision,
            });
        }
        Ok((
            input,
            IngestOptions {
                max_bytes: self.max_bytes,
                strip_metadata: self.strip_metadata,
            },
        ))
    }
}

fn absolute(path: &Path) -> Result<PathBuf> {
    Ok(std::path::absolute(path)?)
}
fn open(db: &Path) -> Result<GardenStore> {
    let db = absolute(db)?;
    mother::garden::migrate::check_destination(&db)?;
    GardenStore::open_active(db)
}
fn print(value: Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

pub(crate) fn run(args: Media) -> Result<()> {
    match *args.command {
        MediaCommand::Add {
            db,
            file,
            provenance,
        } => {
            let file = absolute(&file)?;
            let (input, options) = provenance.input(Some(&file))?;
            let report = open(&db)?
                .media_add(&file, &input, options)
                .map_err(domain_error)?;
            print(serde_json::to_value(report)?)
        }
        MediaCommand::Attach {
            db,
            file,
            sha256,
            to,
            expected_revision,
            note,
            provenance,
        } => {
            let store = open(&db)?;
            let file = file.as_deref().map(absolute).transpose()?;
            let source = match (&file, sha256) {
                (Some(path), _) => {
                    let (input, options) = provenance.input(Some(path))?;
                    AttachSource::File {
                        path,
                        input,
                        options,
                    }
                }
                (None, Some(sha256)) => AttachSource::Existing { sha256 },
                (None, None) => {
                    return Err(Error::domain("INVALID_INPUT", "Choose --file or --sha256."));
                }
            };
            print(
                store
                    .media_attach(&AttachRequest {
                        source,
                        target: to,
                        expected_revision,
                        note,
                    })
                    .map_err(domain_error)?,
            )
        }
        MediaCommand::Complete {
            db,
            proposal,
            file,
            expected_revision,
            accept_description,
            provenance,
        } => {
            let store = open(&db)?;
            let file = absolute(&file)?;
            let (input, options) = provenance.input(Some(&file))?;
            let workspace = store.owner_inbox_list().map_err(domain_error)?["workspace"].clone();
            let request = json!({"version":2,"workspaceId":workspace["id"],
                "generation":workspace["generation"],"operationId":uuid::Uuid::new_v4().to_string(),
                "proposalId":proposal,"expectedRevision":expected_revision,"action":"approve"});
            let (outcome, ingest) = store
                .review_attachment(
                    request,
                    &AttachmentReview {
                        path: &file,
                        input,
                        options,
                        accept_description,
                    },
                )
                .map_err(domain_error)?;
            print(json!({"ingest":ingest,"outcome":outcome}))
        }
        MediaCommand::List { db, record } => print(
            open(&db)?
                .media_owner_list(record.as_deref())
                .map_err(domain_error)?,
        ),
        MediaCommand::Verify { db } => print(open(&db)?.media_verify().map_err(domain_error)?),
        MediaCommand::ExportAll { db, out } => print(
            open(&db)?
                .media_export_all(&absolute(&out)?)
                .map_err(domain_error)?,
        ),
        MediaCommand::Import { db, dir } => print(
            open(&db)?
                .media_import(&absolute(&dir)?)
                .map_err(domain_error)?,
        ),
        MediaCommand::Export { db, sha256, out } => print(
            open(&db)?
                .media_export(&sha256, &absolute(&out)?)
                .map_err(domain_error)?,
        ),
    }
}
