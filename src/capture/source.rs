//! Claude Code JSONL selection, with no network or transcript persistence.
use super::capture_error;
use crate::error::Result;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, HashSet},
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};
use uuid::Uuid;

const SOURCE_LIMIT: u64 = 32 * 1024 * 1024;

#[derive(Deserialize)]
struct Hook {
    session_id: String,
    transcript_path: PathBuf,
    hook_event_name: String,
}

pub(super) fn read_hook(input: impl Read, root: &Path) -> Result<Uuid> {
    let hook: Hook = serde_json::from_reader(input.take(65_536))?;
    let session_id = Uuid::parse_str(&hook.session_id)
        .map_err(|_| capture_error("The hook did not identify a Claude Code session."))?;
    if hook.hook_event_name != "SessionEnd"
        || !hook.transcript_path.is_absolute()
        || hook.transcript_path.file_name().and_then(session_file) != Some(session_id)
        || hook
            .transcript_path
            .parent()
            .and_then(|p| p.canonicalize().ok())
            != Some(root.canonicalize()?)
    {
        return Err(capture_error(
            "The hook does not match the opted-in Claude Code source.",
        ));
    }
    Ok(session_id)
}

fn session_file(name: &std::ffi::OsStr) -> Option<Uuid> {
    let id = name.to_str()?.strip_suffix(".jsonl")?;
    let parsed = Uuid::parse_str(id).ok()?;
    (id == parsed.hyphenated().to_string()).then_some(parsed)
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Scan {
    expected_sessions: usize,
    readable_sessions: usize,
    unavailable_sessions: usize,
    excluded_sessions: usize,
    skipped_turns_in_window: usize,
    selected_items: usize,
    dates_with_readable_turns: Vec<NaiveDate>,
    dates_without_readable_turns: Vec<NaiveDate>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Decision,
    Correction,
    Milestone,
    Observation,
}

#[derive(Debug)]
pub(super) struct Candidate {
    pub(super) session: Uuid,
    pub(super) turn: Uuid,
    pub(super) timestamp: DateTime<Utc>,
    pub(super) kind: Kind,
    pub(super) title: String,
    pub(super) summary: String,
}

impl Candidate {
    pub(super) fn operation(&self) -> Uuid {
        let digest = Sha256::new()
            .chain_update(b"mother/claude-code/capture/1")
            .chain_update(self.session.as_bytes())
            .chain_update(self.turn.as_bytes())
            .finalize();
        let mut bytes = [0; 16];
        bytes.copy_from_slice(&digest[..16]);
        bytes[6] = (bytes[6] & 0x0f) | 0x80;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Uuid::from_bytes(bytes)
    }
}

struct Session {
    dates: BTreeSet<NaiveDate>,
    candidates: Vec<Candidate>,
    skipped: usize,
    excluded: bool,
}

type Window = Option<(NaiveDate, NaiveDate)>;

pub(super) fn scan(
    root: &Path,
    session: Option<Uuid>,
    window: Window,
) -> Result<(Scan, Vec<Candidate>)> {
    let mut report = Scan::default();
    let mut files = Vec::new();
    if let Some(session) = session {
        files.push(session);
    } else {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if let Some(id) = session_file(&entry.file_name()) {
                files.push(id);
                if files.len() > 10_000 {
                    return Err(capture_error(
                        "The selected source exceeds the supported session count.",
                    ));
                }
            }
        }
    }
    report.expected_sessions = files.len();
    let mut dates = BTreeSet::new();
    let mut selected = Vec::new();
    for id in files {
        let path = root.join(format!("{id}.jsonl"));
        let Ok(session) = read_session(&path, id, window) else {
            report.unavailable_sessions += 1;
            continue;
        };
        report.readable_sessions += 1;
        if session.excluded {
            report.excluded_sessions += 1;
        } else {
            dates.extend(session.dates);
            report.skipped_turns_in_window += session.skipped;
            selected.extend(session.candidates);
        }
        if selected.len() > 10_000 {
            return Err(capture_error(
                "Too many candidates in this date range; choose a smaller window.",
            ));
        }
    }
    report.selected_items = selected.len();
    report.dates_with_readable_turns = dates.iter().copied().collect();
    if let Some((from, before)) = window {
        report.dates_without_readable_turns = from
            .iter_days()
            .take_while(|date| *date < before)
            .filter(|date| !dates.contains(date))
            .collect();
    }
    selected.sort_unstable_by_key(|c| (c.timestamp, c.session, c.turn));
    Ok((report, selected))
}

fn read_session(path: &Path, session: Uuid, window: Window) -> Result<Session> {
    crate::private_paths::single_link(path)?;
    let mut data = String::new();
    File::open(path)?
        .take(SOURCE_LIMIT + 1)
        .read_to_string(&mut data)?;
    if data.len() as u64 > SOURCE_LIMIT || !data.ends_with('\n') {
        return Err(capture_error(
            "A selected transcript is oversized or incomplete.",
        ));
    }
    let mut seen = HashSet::new();
    let mut local_opt_out = false;
    let mut protected = HashSet::new();
    let mut result = Session {
        dates: BTreeSet::new(),
        candidates: Vec::new(),
        skipped: 0,
        excluded: false,
    };
    for line in data.lines().filter(|line| !line.trim().is_empty()) {
        let row: Value = serde_json::from_str(line)?;
        let Some(role @ ("user" | "assistant")) = row["type"].as_str() else {
            continue;
        };
        if [
            "isMeta",
            "isSidechain",
            "isCompactSummary",
            "isVisibleInTranscriptOnly",
            "isSynthetic",
        ]
        .iter()
        .any(|flag| row[*flag] == true)
            || !row["agentId"].is_null()
            || !row["origin"].is_null()
            || !row["parent_tool_use_id"].is_null()
            || !row["toolUseResult"].is_null()
            || !row["userType"].is_null() && row["userType"] != "external"
            || row["message"]["role"] != role
        {
            continue;
        }
        if row["sessionId"]
            .as_str()
            .and_then(|id| Uuid::parse_str(id).ok())
            != Some(session)
        {
            return Err(capture_error(
                "A selected transcript contains an unexpected session.",
            ));
        }
        let timestamp = row["timestamp"]
            .as_str()
            .and_then(|stamp| stamp.parse::<DateTime<Utc>>().ok())
            .ok_or_else(|| capture_error("A selected transcript contains an invalid timestamp."))?;
        let day = timestamp.date_naive();
        let content = &row["message"]["content"];
        let excluded = matches_exclusion(content);
        if role == "user" && excluded {
            if let Some(passage) = plain_text(content).and_then(local_exclusion) {
                local_opt_out = true;
                if passage.len() > 16_384 {
                    result.excluded = true;
                } else {
                    protected.extend(fragments(&words(passage)));
                    if protected.len() > 20_000 {
                        result.excluded = true;
                    }
                }
            } else {
                result.excluded = true;
            }
        }
        if window.is_some_and(|(from, before)| day < from || day >= before) {
            continue;
        }
        result.dates.insert(day);
        let selected = (!excluded)
            .then(|| plain_text(content))
            .flatten()
            .and_then(|text| selection(role, text));
        let Some((kind, title, summary)) = selected else {
            result.skipped += 1;
            continue;
        };
        let turn = row["uuid"]
            .as_str()
            .and_then(|id| Uuid::parse_str(id).ok())
            .ok_or_else(|| capture_error("A selected transcript item has no stable identifier."))?;
        if !seen.insert(turn) {
            return Err(capture_error(
                "A selected transcript repeats a selected identifier.",
            ));
        }
        result.candidates.push(Candidate {
            session,
            turn,
            timestamp,
            kind,
            title,
            summary,
        });
    }
    if result.excluded {
        result.candidates.clear();
    } else if local_opt_out {
        let previously_selected = result.candidates.len();
        result.candidates.retain(|candidate| {
            candidate.kind != Kind::Observation
                && ![&candidate.title, &candidate.summary]
                    .into_iter()
                    .any(|text| {
                        let normalized = words(text);
                        normalized.iter().any(|word| protected.contains(word))
                            || fragments(&normalized)
                                .into_iter()
                                .any(|fragment| protected.contains(&fragment))
                    })
        });
        result.skipped += previously_selected - result.candidates.len();
    }
    Ok(result)
}

fn plain_text(content: &Value) -> Option<&str> {
    match content {
        Value::String(text) => Some(text),
        Value::Array(parts) if parts.len() == 1 && parts[0]["type"] == "text" => {
            parts[0]["text"].as_str()
        }
        _ => None,
    }
}

fn matches_exclusion(content: &Value) -> bool {
    let excludes = |text: &str| {
        let normalized = text
            .to_lowercase()
            .replace(['’', '‘'], "'")
            .replace(['-', '‑', '–'], " ");
        let normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
        [
            "do not remember",
            "don't remember",
            "do not retain",
            "don't retain",
            "off the record",
            "[private]",
            "do not save",
            "don't save",
            "do not capture",
            "don't capture",
            "do not store",
            "don't store",
            "not for memory",
            "forget this",
        ]
        .iter()
        .any(|phrase| normalized.contains(phrase))
    };
    content.as_str().is_some_and(excludes)
        || content.as_array().is_some_and(|parts| {
            parts
                .iter()
                .any(|part| part["type"] == "text" && part["text"].as_str().is_some_and(excludes))
        })
}

fn local_exclusion(text: &str) -> Option<&str> {
    let (lead, passage) = text.trim().split_once(':')?;
    let lead = lead.to_lowercase().replace('’', "'");
    [
        "do not remember",
        "don't remember",
        "do-not-remember",
        "off the record",
        "[private]",
    ]
    .contains(&lead.as_str())
    .then(|| passage.trim())
    .filter(|passage| {
        let lower = passage.to_lowercase();
        !lower.is_empty()
            && ![
                "anything",
                "everything",
                "this session",
                "the rest of",
                "what follows",
                "the following",
                "the next",
                "all of",
            ]
            .iter()
            .any(|lead| lower.starts_with(lead))
    })
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn fragments(words: &[String]) -> Vec<String> {
    if words.is_empty() {
        return Vec::new();
    }
    words
        .windows(words.len().min(2))
        .map(|window| window.join(" "))
        .collect()
}

fn selection(role: &str, text: &str) -> Option<(Kind, String, String)> {
    let text = text.trim();
    if text.chars().any(char::is_control) {
        return None;
    }
    let Some((prefix, draft)) = text.split_once(" [") else {
        return (role == "user").then(|| ordinary_statement(text)).flatten();
    };
    let kind = match (role, prefix.to_ascii_lowercase().as_str()) {
        ("user", "confirmed decision") => Kind::Decision,
        ("user", "decision correction") => Kind::Correction,
        ("user", "milestone reached") => Kind::Milestone,
        ("assistant", "observation") => Kind::Observation,
        _ => return None,
    };
    let (title, summary) = draft.split_once("]:")?;
    let (title, summary) = (title.trim(), summary.trim());
    if !safe_summary(title, 100) || !safe_summary(summary, 360) {
        return None;
    }
    Some((kind, title.into(), summary.into()))
}

fn ordinary_statement(text: &str) -> Option<(Kind, String, String)> {
    let (kind, phrase, attribution) = [
        ("i've decided to ", Kind::Decision, "Owner decided to"),
        ("i’ve decided to ", Kind::Decision, "Owner decided to"),
        ("i decided to ", Kind::Decision, "Owner decided to"),
        (
            "we've decided to ",
            Kind::Decision,
            "Owner reports the group decided to",
        ),
        (
            "we’ve decided to ",
            Kind::Decision,
            "Owner reports the group decided to",
        ),
        (
            "we decided to ",
            Kind::Decision,
            "Owner reports the group decided to",
        ),
        (
            "we finished ",
            Kind::Milestone,
            "Owner reports the group finished",
        ),
        ("i finished ", Kind::Milestone, "Owner reports finishing"),
        (
            "we completed ",
            Kind::Milestone,
            "Owner reports the group completed",
        ),
        ("i completed ", Kind::Milestone, "Owner reports completing"),
    ]
    .into_iter()
    .find_map(|(prefix, kind, attribution)| {
        let lead = text.get(..prefix.len())?;
        lead.eq_ignore_ascii_case(prefix)
            .then(|| (kind, &text[prefix.len()..], attribution))
    })?;
    let phrase = phrase.trim_end_matches('.').trim();
    let lower = phrase.to_lowercase();
    if phrase.contains('.')
        || phrase.split_whitespace().count() < 2
        || [
            " if ",
            " unless ",
            " but ",
            " or ",
            " for now",
            " might ",
            " could ",
            " changed my mind",
            " instead",
        ]
        .iter()
        .any(|marker| lower.contains(marker))
        || !safe_summary(phrase, 100)
    {
        return None;
    }
    let mut chars = phrase.chars();
    let title = chars.next()?.to_uppercase().chain(chars).collect();
    Some((kind, title, format!("{attribution} {phrase}.")))
}

fn safe_summary(text: &str, max: usize) -> bool {
    if text.is_empty() || text.chars().count() > max {
        return false;
    }
    // Permit short prose only. Reject URL, code and credential syntax before any proposal or log.
    if !text
        .chars()
        .all(|c| c.is_alphanumeric() || " .,'’-".contains(c))
    {
        return false;
    }
    let normalized = text.to_lowercase();
    if [
        "password",
        "passphrase",
        "secret",
        "credential",
        "access token",
        "api key",
        "private key",
        "bearer",
        "seed phrase",
        "recovery code",
        "github pat",
        "sk-",
        "ghp-",
        "ghp_",
        "akia",
        "xoxb-",
        "eyj",
        "maybe",
        "tentative",
        "abandoned",
        "not confirmed",
    ]
    .iter()
    .any(|phrase| normalized.contains(phrase))
    {
        return false;
    }
    !text.split_whitespace().any(|word| {
        let token = word.trim_matches(|c: char| !c.is_alphanumeric());
        token.chars().count() >= 18
            || (token.chars().count() >= 6 && token.chars().all(|c| c.is_ascii_digit()))
            || (token.chars().count() >= 8
                && token.chars().any(|c| c.is_ascii_digit())
                && token.chars().any(|c| c.is_alphabetic()))
    })
}
