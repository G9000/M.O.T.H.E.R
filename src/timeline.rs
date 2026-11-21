//! Local timeline and knowledge-cutoff comparison over authorized Garden revisions.
use crate::relationships::Connection;
use chrono::{DateTime, NaiveDate, Utc};
use clap::{Args, Subcommand, ValueEnum};
use mother::{
    error::{Error, Result},
    garden::contracts::{
        AssertionObject, ExternalReference, GardenDraft, GardenKind, GardenRecord, GetResponse,
        Lifecycle, QueryRequest, QueryResponse, RecordRef, SourceLocator, TimePrecision,
        TypedValue,
    },
};
use serde::Serialize;
use serde_json::json;
use std::{
    collections::BTreeMap,
    io::{self, IsTerminal},
    path::PathBuf,
};

const COMPARE_LIMIT: u32 = 16;
const EVIDENCE_LIMIT: usize = 3;

#[derive(Debug, Args)]
pub(crate) struct Timeline {
    /// Emit JSON even in a terminal.
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List a bounded page of visible claims, sources and records.
    List {
        #[arg(long)]
        config: PathBuf,
        /// What the garden knew at the end of this UTC date.
        #[arg(long)]
        known_on: Option<NaiveDate>,
        #[command(flatten)]
        filter: Filter,
        /// Native cursor from a previous page with the same request.
        #[arg(long)]
        cursor: Option<String>,
        /// Maximum candidates per page before local event-date filtering.
        #[arg(long, default_value_t = 12, value_parser = clap::value_parser!(u32).range(1..=24))]
        limit: u32,
    },
    /// Compare two UTC knowledge cutoffs using bounded authorized candidates.
    Compare {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        earlier: NaiveDate,
        #[arg(long)]
        later: NaiveDate,
        #[command(flatten)]
        filter: Filter,
    },
    /// Read one authorized record or immutable revision and its sources.
    Inspect {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        kind: Kind,
        #[arg(long)]
        id: String,
        /// Exact canonical revision number.
        #[arg(long, conflicts_with = "known_on")]
        revision: Option<u64>,
        /// What the garden knew at the end of this UTC date.
        #[arg(long)]
        known_on: Option<NaiveDate>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Kind {
    Entity,
    Assertion,
    Evidence,
}

impl From<Kind> for GardenKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Entity => Self::Entity,
            Kind::Assertion => Self::Assertion,
            Kind::Evidence => Self::Evidence,
        }
    }
}

#[derive(Debug, Args)]
struct Filter {
    #[arg(long)]
    kind: Option<Kind>,
    /// Exact graph predicate, such as example:milestone.
    #[arg(long)]
    predicate: Option<String>,
    /// Inclusive first UTC event date.
    #[arg(long)]
    from: Option<NaiveDate>,
    /// Inclusive final UTC event date.
    #[arg(long)]
    through: Option<NaiveDate>,
    /// Omit records whose event or observed date is unknown.
    #[arg(long)]
    dated_only: bool,
}

impl Filter {
    fn validate(&self) -> Result<()> {
        if self
            .from
            .zip(self.through)
            .is_some_and(|(start, end)| start > end)
        {
            return Err(input(
                "The first event date must precede or equal the last.",
            ));
        }
        if self.predicate.is_some() && matches!(self.kind, Some(Kind::Entity | Kind::Evidence)) {
            return Err(input("A predicate filter applies only to assertions."));
        }
        for date in [self.from, self.through].into_iter().flatten() {
            validate_date(date)?;
        }
        Ok(())
    }

    fn request(&self, conn: &Connection, known_at: Option<String>, limit: u32) -> QueryRequest {
        let mut request = conn.blank_query();
        request.kinds = self.kind.map(Into::into).into_iter().collect();
        request.predicate.clone_from(&self.predicate);
        request.known_at = known_at;
        request.limit = limit;
        request
    }

    fn matches(&self, record: &GardenRecord) -> Result<bool> {
        let event = event_time(&record.draft);
        if event.status == "unknown" {
            return Ok(!self.dated_only);
        }
        let from_bound = self.from.map(day_start);
        let through_bound = self.through.map(day_end);
        let start = event.from.map(timestamp).transpose()?;
        let end = event.until_exclusive.map(timestamp).transpose()?;
        // A validity interval can have either endpoint open. A source observation is an instant.
        let overlaps_start = match event.basis {
            "source_observation" => start.zip(from_bound).is_none_or(|(at, bound)| at >= bound),
            _ => end
                .zip(from_bound)
                .is_none_or(|(until, bound)| until > bound),
        };
        Ok(overlaps_start
            && start
                .zip(through_bound)
                .is_none_or(|(at, bound)| at <= bound))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EventTime<'a> {
    basis: &'static str,
    status: &'static str,
    from: Option<&'a str>,
    until_exclusive: Option<&'a str>,
    source_timezone: Option<&'a str>,
    precision: TimePrecision,
}

fn event_time(draft: &GardenDraft) -> EventTime<'_> {
    let (basis, from, until, zone, precision) = match draft {
        GardenDraft::Assertion { valid_time, .. } => (
            "validity",
            valid_time.valid_from.as_deref(),
            valid_time.valid_until.as_deref(),
            valid_time.source_timezone.as_deref(),
            valid_time.precision,
        ),
        GardenDraft::Evidence { observed_time, .. } => (
            "source_observation",
            observed_time.at.as_deref(),
            None,
            observed_time.source_timezone.as_deref(),
            observed_time.precision,
        ),
        GardenDraft::Entity { .. } => ("unrecorded", None, None, None, TimePrecision::Unknown),
    };
    let status = if (from.is_none() && until.is_none()) || precision == TimePrecision::Unknown {
        "unknown"
    } else if matches!(precision, TimePrecision::Year | TimePrecision::Month) {
        "approximate"
    } else {
        "stated"
    };
    EventTime {
        basis,
        status,
        from,
        until_exclusive: until,
        source_timezone: zone,
        precision,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Entry<'a> {
    id: &'a str,
    kind: GardenKind,
    label: String,
    revision: u64,
    lifecycle: Lifecycle,
    recorded_at: &'a str,
    actor: &'a str,
    contributor: &'a str,
    approver: Option<&'a str>,
    event_time: EventTime<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<&'a SourceLocator>,
    evidence: Vec<Source>,
    more_evidence: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Source {
    id: String,
    revision: u64,
    recorded_at: String,
    actor: String,
    contributor: String,
    source: SourceLocator,
    excerpt: Option<String>,
}

fn preview(text: &str) -> String {
    let mut characters = text.chars();
    let mut value: String = characters.by_ref().take(240).collect();
    if characters.next().is_some() {
        value.push('…');
    }
    value
}

fn entry<'a>(
    context: &Context<'_>,
    record: &'a GardenRecord,
    known_at: Option<&str>,
) -> Result<Entry<'a>> {
    let (label, evidence, source) = match &record.draft {
        GardenDraft::Entity { display_name, .. } => (preview(display_name), &[][..], None),
        GardenDraft::Assertion {
            predicate,
            object,
            evidence,
            ..
        } => {
            let label = match object {
                AssertionObject::Value {
                    value: TypedValue::String { value },
                } => format!("{predicate} · {}", preview(value)),
                AssertionObject::Value { value } => {
                    format!("{predicate} · {}", preview(&serde_json::to_string(value)?))
                }
                AssertionObject::EntityRef { .. } => predicate.clone(),
            };
            (label, evidence.as_slice(), None)
        }
        GardenDraft::Evidence {
            source, excerpt, ..
        } => (
            excerpt
                .as_deref()
                .map_or_else(|| "Source evidence".into(), preview),
            &[][..],
            Some(source),
        ),
    };
    let mut sources = Vec::new();
    // The native read has already checked the complete current and historical footprint.
    for reference in evidence.iter().take(EVIDENCE_LIMIT) {
        let RecordRef::Existing { id, kind } = reference else {
            return Err(restart());
        };
        let evidence = context
            .get(
                ExternalReference {
                    id: id.clone(),
                    kind: *kind,
                },
                None,
                known_at,
            )?
            .ok_or_else(restart)?;
        let GardenDraft::Evidence {
            source, excerpt, ..
        } = evidence.draft
        else {
            return Err(restart());
        };
        sources.push(Source {
            id: evidence.metadata.id,
            revision: evidence.metadata.revision,
            recorded_at: evidence.metadata.transaction_time,
            actor: evidence.metadata.actor,
            contributor: evidence.metadata.contributor,
            source,
            excerpt: excerpt.as_deref().map(preview),
        });
    }
    Ok(Entry {
        id: &record.metadata.id,
        kind: record.draft.kind(),
        label,
        revision: record.metadata.revision,
        lifecycle: record.metadata.lifecycle,
        recorded_at: &record.metadata.transaction_time,
        actor: &record.metadata.actor,
        contributor: &record.metadata.contributor,
        approver: record.metadata.approver.as_deref(),
        event_time: event_time(&record.draft),
        source,
        evidence: sources,
        more_evidence: evidence.len() > EVIDENCE_LIMIT,
    })
}

struct Context<'a> {
    conn: &'a Connection,
    watermark: u64,
    epoch: &'a str,
}

impl<'a> Context<'a> {
    fn query(conn: &'a Connection, result: &'a QueryResponse) -> Self {
        Self {
            conn,
            watermark: result.coverage.canonical_watermark,
            epoch: &result.page.visibility_epoch,
        }
    }

    fn inspect(conn: &'a Connection, result: &'a GetResponse) -> Self {
        Self {
            conn,
            watermark: result.coverage.canonical_watermark,
            epoch: &result.visibility_epoch,
        }
    }

    fn verify(&self, watermark: u64, epoch: &str) -> Result<()> {
        if self.watermark != watermark || self.epoch != epoch {
            return Err(restart());
        }
        Ok(())
    }

    fn get(
        &self,
        reference: ExternalReference,
        revision: Option<u64>,
        known_at: Option<&str>,
    ) -> Result<Option<GardenRecord>> {
        match self.conn.get_at(reference, revision, known_at) {
            Ok(result) => {
                self.verify(
                    result.coverage.canonical_watermark,
                    &result.visibility_epoch,
                )?;
                Ok(Some(result.record))
            }
            Err(error) if error.code() == "NOT_FOUND" => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn finish(&self, record: &GardenRecord) -> Result<()> {
        self.get(reference(record), Some(record.metadata.revision), None)?
            .ok_or_else(restart)?;
        Ok(())
    }
}

fn reference(record: &GardenRecord) -> ExternalReference {
    ExternalReference {
        kind: record.draft.kind(),
        id: record.metadata.id.clone(),
    }
}

fn input(message: &str) -> Error {
    Error::domain("INVALID_INPUT", message)
}

fn restart() -> Error {
    Error::domain(
        "RESTART_REQUIRED",
        "The timeline changed during this read. Request a fresh page under the current grant.",
    )
}

fn validate_date(date: NaiveDate) -> Result<()> {
    use chrono::Datelike;
    if !(1..=9999).contains(&date.year()) {
        return Err(input("Use a calendar date from year 0001 through 9999."));
    }
    Ok(())
}

fn day_start(date: NaiveDate) -> DateTime<Utc> {
    date.and_time(chrono::NaiveTime::MIN).and_utc()
}

fn day_end(date: NaiveDate) -> DateTime<Utc> {
    day_start(date) + chrono::TimeDelta::nanoseconds(86_399_999_999_999)
}

fn timestamp(value: &str) -> Result<DateTime<Utc>> {
    value.parse().map_err(|_| {
        Error::domain(
            "STORE_UNAVAILABLE",
            "The stored record has an invalid timeline timestamp.",
        )
    })
}

fn cutoff(value: Option<NaiveDate>) -> Result<Option<String>> {
    value
        .map(|date| {
            validate_date(date)?;
            Ok(day_end(date).to_rfc3339_opts(chrono::SecondsFormat::Nanos, true))
        })
        .transpose()
}

const HISTORY_SCOPE: &str = "Authorized, retained native Garden revisions only. Missing or unreadable history cannot establish absence or change.";

fn print(value: &impl Serialize) -> Result<()> {
    let encoded = serde_json::to_string_pretty(value)?;
    let mut safe = String::with_capacity(encoded.len());
    for character in encoded.chars() {
        if character != '\n' && unsafe_terminal_character(character) {
            safe.push_str(&format!("\\u{:04x}", u32::from(character)));
        } else {
            safe.push(character);
        }
    }
    println!("{safe}");
    Ok(())
}

fn unsafe_terminal_character(character: char) -> bool {
    character.is_control()
        || matches!(character,
        '\u{061c}' | '\u{200e}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

fn terminal_text(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if unsafe_terminal_character(character) {
                ' '
            } else {
                character
            }
        })
        .collect()
}

fn display_source(source: &SourceLocator) -> String {
    match source {
        SourceLocator::Uri { uri } => terminal_text(uri),
        SourceLocator::Artifact {
            sha256, media_type, ..
        } => {
            format!(
                "Artifact {} ({})",
                terminal_text(sha256),
                terminal_text(media_type)
            )
        }
    }
}

fn show_entry(value: &Entry<'_>) {
    let event = &value.event_time;
    println!("{} · {:?}", terminal_text(&value.label), value.kind);
    if event.status == "unknown" {
        println!("  Event date unknown");
    } else {
        let start = event.from.unwrap_or("open start");
        let approximate = if event.status == "approximate" {
            "Approximate "
        } else {
            ""
        };
        let basis = if event.basis == "validity" {
            "validity"
        } else {
            "source observation"
        };
        print!(
            "  {approximate}{basis} ({:?})  {}",
            event.precision,
            terminal_text(start)
        );
        if let Some(until) = event.until_exclusive {
            print!(" to {} (exclusive)", terminal_text(until));
        } else if event.basis == "validity" {
            print!(" onward");
        }
        println!();
    }
    println!(
        "  {} · revision {} · {:?}",
        terminal_text(value.id),
        value.revision,
        value.lifecycle
    );
    println!(
        "  Recorded {} by {}",
        terminal_text(value.recorded_at),
        terminal_text(value.actor)
    );
    if value.actor != value.contributor {
        println!(
            "  Original contributor {}",
            terminal_text(value.contributor)
        );
    }
    if let Some(approver) = value.approver {
        println!("  Approved by {}", terminal_text(approver));
    }
    if let Some(source) = value.source {
        println!("  Source {}", display_source(source));
    }
    for source in &value.evidence {
        println!(
            "  Evidence {} · revision {} · {}",
            terminal_text(&source.id),
            source.revision,
            display_source(&source.source)
        );
        println!(
            "    Recorded {} by {}",
            terminal_text(&source.recorded_at),
            terminal_text(&source.actor)
        );
        if let Some(excerpt) = &source.excerpt {
            println!("    {}", terminal_text(excerpt));
        }
    }
    if value.more_evidence {
        println!("  More authorized evidence in the full record.");
    }
}

fn show_coverage(complete: bool, cursor: Option<&str>) {
    if !complete {
        println!("Partial coverage. Refine the filters or continue if a next page is available.");
    }
    if let Some(cursor) = cursor {
        println!(
            "Next page  --cursor {}  (repeat the other list options)",
            terminal_text(cursor)
        );
    }
    println!("{HISTORY_SCOPE}");
}

fn list(
    conn: &Connection,
    filter: &Filter,
    date: Option<NaiveDate>,
    cursor: Option<String>,
    limit: u32,
    json: bool,
) -> Result<()> {
    let mut request = filter.request(conn, cutoff(date)?, limit);
    request.cursor = cursor;
    let result = conn.query(&request)?;
    let context = Context::query(conn, &result);
    let mut hits = Vec::new();
    for hit in &result.hits {
        if filter.matches(&hit.record)? {
            let entry = entry(&context, &hit.record, request.known_at.as_deref())?;
            let start = entry.event_time.from.map(timestamp).transpose()?;
            hits.push((start, entry));
        }
    }
    if let Some(hit) = result.hits.first() {
        context.finish(&hit.record)?;
    }
    hits.sort_by_key(|(start, item)| (start.is_none(), *start, item.id));
    let entries: Vec<_> = hits.into_iter().map(|(_, item)| item).collect();
    let complete = result.coverage.complete && result.page.next_cursor.is_none();
    let reason = result
        .coverage
        .reason
        .as_deref()
        .or_else(|| result.page.next_cursor.as_ref().map(|_| "more_pages"));
    if !json {
        match date {
            Some(date) => println!("Life Timeline · known on {date} UTC"),
            None => println!("Life Timeline · current garden"),
        }
        if entries.is_empty() {
            println!("No matching records on this page.");
        }
        for entry in &entries {
            println!();
            show_entry(entry);
        }
        println!();
        show_coverage(complete, result.page.next_cursor.as_deref());
        return Ok(());
    }
    print(&json!({
        "knownOn":date,"entries":entries,"nextCursor":result.page.next_cursor,
        "coverage":{"complete":complete,"reason":reason,"scope":HISTORY_SCOPE},
    }))
}

#[derive(Serialize)]
struct Change<'a> {
    change: &'static str,
    before: Option<Entry<'a>>,
    after: Entry<'a>,
}

fn compare(
    conn: &Connection,
    filter: &Filter,
    earlier: NaiveDate,
    later: NaiveDate,
    json: bool,
) -> Result<()> {
    if earlier >= later {
        return Err(input("The earlier cutoff must precede the later cutoff."));
    }
    let older = filter.request(conn, cutoff(Some(earlier))?, COMPARE_LIMIT);
    let newer = filter.request(conn, cutoff(Some(later))?, COMPARE_LIMIT);
    let old_page = conn.query(&older)?;
    let new_page = conn.query(&newer)?;
    let context = Context::query(conn, &old_page);
    context.verify(
        new_page.coverage.canonical_watermark,
        &new_page.page.visibility_epoch,
    )?;
    let mut candidates = BTreeMap::new();
    for hit in old_page.hits.iter().chain(&new_page.hits) {
        let reference = reference(&hit.record);
        candidates.insert(reference.id.clone(), reference);
    }
    let before_at = older.known_at.as_deref();
    let after_at = newer.known_at.as_deref();
    let mut pairs = Vec::new();
    for item in candidates.into_values() {
        let before = context.get(item.clone(), None, before_at)?;
        let after = context.get(item.clone(), None, after_at)?;
        let Some(after) = after else { continue };
        let change = match &before {
            Some(before) if before.metadata.revision == after.metadata.revision => continue,
            Some(_) => match after.metadata.lifecycle {
                Lifecycle::Active => "corrected",
                Lifecycle::Superseded => "superseded",
                Lifecycle::Retracted => "retracted",
            },
            // A missing old result can mean hidden history. Verify the record really originated after the earlier day.
            None if after.metadata.lifecycle == Lifecycle::Active => {
                let created = if after.metadata.revision == 1 {
                    Some(after.metadata.transaction_time.clone())
                } else {
                    context
                        .get(item, Some(1), after_at)?
                        .map(|first| first.metadata.transaction_time)
                };
                let Some(created) = created else { continue };
                if timestamp(&created)? <= day_end(earlier) {
                    continue;
                }
                "added"
            }
            None => continue,
        };
        let before_matches = before
            .as_ref()
            .map(|record| filter.matches(record))
            .transpose()?
            .unwrap_or(false);
        if before_matches || filter.matches(&after)? {
            pairs.push((change, before, after));
        }
    }
    let mut changes = Vec::new();
    for (change, before, after) in &pairs {
        changes.push(Change {
            change,
            before: before
                .as_ref()
                .map(|record| entry(&context, record, before_at))
                .transpose()?,
            after: entry(&context, after, after_at)?,
        });
    }
    if let Some(hit) = new_page.hits.first().or_else(|| old_page.hits.first()) {
        context.finish(&hit.record)?;
    }
    let complete = [&old_page, &new_page]
        .into_iter()
        .all(|page| page.coverage.complete && page.page.next_cursor.is_none());
    if !json {
        println!("Life Timeline · {earlier} to {later} UTC");
        if changes.is_empty() {
            println!("No supported changes in this comparison.");
        }
        for change in &changes {
            println!();
            println!("Change · {}", change.change);
            if let Some(before) = &change.before {
                println!("Earlier");
                show_entry(before);
            }
            println!("Later");
            show_entry(&change.after);
        }
        println!();
        show_coverage(complete, None);
        return Ok(());
    }
    print(&json!({
        "earlier":earlier,"later":later,"changes":changes,
        "coverage":{"complete":complete,"reason":if complete { None } else { Some("bounded_comparison") },
            "maximumCandidatesPerPeriod":COMPARE_LIMIT,"scope":HISTORY_SCOPE},
    }))
}

pub(crate) fn run(args: Timeline) -> Result<()> {
    let json = args.json || !io::stdout().is_terminal();
    match args.command {
        Command::List {
            config,
            known_on,
            filter,
            cursor,
            limit,
        } => {
            filter.validate()?;
            list(
                &Connection::open(&config)?,
                &filter,
                known_on,
                cursor,
                limit,
                json,
            )
        }
        Command::Compare {
            config,
            earlier,
            later,
            filter,
        } => {
            filter.validate()?;
            compare(&Connection::open(&config)?, &filter, earlier, later, json)
        }
        Command::Inspect {
            config,
            kind,
            id,
            revision,
            known_on,
        } => {
            let conn = Connection::open(&config)?;
            let known_at = cutoff(known_on)?;
            let result = conn.get_at(
                ExternalReference {
                    kind: kind.into(),
                    id,
                },
                revision,
                known_at.as_deref(),
            )?;
            let context = Context::inspect(&conn, &result);
            let source_cutoff = known_at
                .as_deref()
                .or_else(|| revision.map(|_| result.record.metadata.transaction_time.as_str()));
            let description = entry(&context, &result.record, source_cutoff)?;
            context.finish(&result.record)?;
            if !json {
                println!("Life Timeline · record details");
                if let Some(date) = known_on {
                    println!("Known on {date} UTC");
                }
                show_entry(&description);
                println!();
                for line in serde_json::to_string_pretty(&result.record.draft)?.lines() {
                    println!("{}", terminal_text(line));
                }
                return Ok(());
            }
            print(&json!({ "entry":description,"record":result.record,"knownOn":known_on }))
        }
    }
}
