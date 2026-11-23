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
            DEFAULT_STAGE_ABOVE, IngestOptions, MediaOrigin,
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
    /// Review and attach an exported file to an existing v1 memory.
    Backfill {
        #[command(subcommand)]
        command: BackfillCommand,
    },
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
    /// Discard interrupted large-file stages; published media is never removed.
    Cleanup {
        #[arg(long)]
        db: PathBuf,
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

#[derive(Debug, Subcommand)]
enum BackfillCommand {
    /// Print the proposed binding for private review. Redirect stdout into a manifest.
    Plan {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        root: PathBuf,
        /// Relative path inside the private import root, using forward slashes.
        #[arg(long)]
        file: String,
        #[arg(long)]
        memory: String,
        #[arg(long)]
        source_file_id: Option<String>,
        /// Show the selection and prompt for immediate owner approval in a terminal.
        #[arg(long)]
        interactive: bool,
        /// Permit this original, including its embedded metadata, in the private owner Garden.
        #[arg(long)]
        allow_original_in_owner_garden: bool,
    },
    /// Withdraw an approved link and retain its history.
    Revoke {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
        /// The original reviewed attachment UUID.
        #[arg(long)]
        approve: String,
    },
    /// Check source bytes, stored bytes, and the approved association.
    Verify {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
    },
    /// Apply exactly the selection identified by the reviewed operation UUID.
    Apply {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        approve: String,
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
    /// Recordings larger than this are written in bounded transactions.
    #[arg(long, default_value_t = DEFAULT_STAGE_ABOVE)]
    stage_above: u64,
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
                stage_above: self.stage_above,
            },
        ))
    }
}

fn read_attachment_plan(path: &Path) -> Result<mother::garden::media_tools::AttachmentPlan> {
    use std::io::Read;
    let mut data = Vec::new();
    std::fs::File::open(path)?
        .take(32769)
        .read_to_end(&mut data)?;
    if data.len() > 32768 {
        return Err(Error::domain(
            "INVALID_INPUT",
            "Attachment manifest exceeds 32 KiB.",
        ));
    }
    Ok(serde_json::from_slice(&data)?)
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
        MediaCommand::Backfill { command } => match command {
            BackfillCommand::Plan {
                db,
                root,
                file,
                memory,
                source_file_id,
                interactive,
                allow_original_in_owner_garden,
            } => {
                use std::io::{IsTerminal, Write};
                let store = open(&db)?;
                let plan = store
                    .plan_attachment(
                        &root,
                        &file,
                        &memory,
                        source_file_id,
                        allow_original_in_owner_garden,
                    )
                    .map_err(domain_error)?;
                print(serde_json::to_value(&plan)?)?;
                if !interactive {
                    return Ok(());
                }
                if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
                    return Err(Error::domain(
                        "INVALID_INPUT",
                        "Interactive approval requires an owner terminal.",
                    ));
                }
                eprint!("Attach this file to the displayed memory? [y/N] ");
                std::io::stderr().flush()?;
                let mut answer = String::new();
                std::io::stdin().read_line(&mut answer)?;
                if answer.trim().eq_ignore_ascii_case("y") {
                    print(
                        store
                            .apply_attachment(&root, &plan, &plan.operation_id)
                            .map_err(domain_error)?,
                    )
                } else {
                    print(json!({"status":"skipped"}))
                }
            }
            BackfillCommand::Revoke {
                db,
                manifest,
                approve,
            } => {
                let plan = read_attachment_plan(&manifest)?;
                print(
                    open(&db)?
                        .revoke_attachment(&plan, &approve)
                        .map_err(domain_error)?,
                )
            }
            BackfillCommand::Verify { db, root, manifest } => {
                let plan = read_attachment_plan(&manifest)?;
                print(
                    open(&db)?
                        .verify_attachment(&root, &plan)
                        .map_err(domain_error)?,
                )
            }
            BackfillCommand::Apply {
                db,
                root,
                manifest,
                approve,
            } => {
                let plan = read_attachment_plan(&manifest)?;
                print(
                    open(&db)?
                        .apply_attachment(&root, &plan, &approve)
                        .map_err(domain_error)?,
                )
            }
        },
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
        MediaCommand::Cleanup { db } => print(open(&db)?.media_cleanup().map_err(domain_error)?),
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
