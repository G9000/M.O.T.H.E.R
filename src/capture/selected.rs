//! One explicitly selected conversation, supplied by the caller without opening a transcript.
use super::capture_error;
use crate::{contracts, error::Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(super) struct Selection {
    pub(super) operation_id: String,
    title: String,
    originals: Vec<Passage>,
    source_ref: Option<String>,
    event_date: Option<NaiveDate>,
    summary: Option<Note>,
    why_it_mattered: Option<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Passage {
    speaker: Speaker,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    spoken_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Speaker {
    User,
    Assistant,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Note {
    author: String,
    text: String,
}

pub(super) const FORMAT: &str = "mother.selected-conversation.v1";

pub(super) fn read(input: impl Read) -> Result<Selection> {
    let mut bytes = Vec::new();
    input.take(96 * 1024 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > 96 * 1024 {
        return Err(capture_error(
            "Selected conversation exceeds the input limit.",
        ));
    }
    let mut selection: Selection = serde_json::from_slice(&bytes).map_err(|_| {
        capture_error("Invalid selected conversation JSON. Check the selection fields.")
    })?;
    contracts::id(&selection.operation_id)?;
    selection.operation_id.make_ascii_lowercase();
    let label = |s: &str| {
        !s.trim().is_empty() && s.encode_utf16().count() <= 200 && !s.chars().any(char::is_control)
    };
    if selection.originals.is_empty()
        || selection.originals.len() > 40
        || selection
            .originals
            .iter()
            .any(|p| p.text.trim().is_empty() || p.author.as_deref().is_some_and(|s| !label(s)))
        || [&selection.summary, &selection.why_it_mattered]
            .into_iter()
            .flatten()
            .any(|n| !label(&n.author) || n.text.trim().is_empty())
        || selection.source_ref.as_deref().is_some_and(|s| !label(s))
    {
        return Err(capture_error(
            "Provide one to forty selected passages with valid attribution and nonempty notes or source labels.",
        ));
    }
    Ok(selection)
}

impl Selection {
    pub(super) fn request(&self, workspace: &Value) -> Result<Value> {
        let mut content = json!({
            "format":FORMAT, "originals":self.originals, "capturedAt":Utc::now()
        });
        if let Some(value) = &self.source_ref {
            content["sourceRef"] = json!(value);
        }
        if let Some(value) = &self.event_date {
            content["eventDate"] = json!(value);
        }
        if let Some(value) = &self.summary {
            content["summary"] = json!(value);
        }
        if let Some(value) = &self.why_it_mattered {
            content["whyItMattered"] = json!(value);
        }
        let record = contracts::normalize_draft(json!({
            "kind":"memory", "status":"active", "title":self.title,
            "basis":"source", "content":content.to_string()
        }))?;
        Ok(json!({
            "operationId":self.operation_id, "workspaceId":workspace["id"],
            "generation":workspace["generation"], "expectedRevision":0,
            "basis":"source", "source":self.source_ref.as_deref().unwrap_or("Owner-selected conversation"),
            "record":record
        }))
    }
}
