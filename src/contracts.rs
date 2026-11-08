//! Validated v1 domain inputs and normalization compatible with the original workspace.
use crate::error::{Result, invalid};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
/// Server-bound client identity and secret. Debug output always redacts the token.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Auth {
    pub client_id: String,
    pub token: String,
}
impl std::fmt::Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Auth")
            .field("client_id", &self.client_id)
            .finish_non_exhaustive()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Memory,
    Decision,
    Task,
    Checkpoint,
    Artifact,
    Action,
}
impl Kind {
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "memory" => Ok(Self::Memory),
            "decision" => Ok(Self::Decision),
            "task" => Ok(Self::Task),
            "checkpoint" => Ok(Self::Checkpoint),
            "artifact" => Ok(Self::Artifact),
            "action" => Ok(Self::Action),
            _ => Err(invalid("Unknown record kind")),
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Memory => "memory",
            Self::Decision => "decision",
            Self::Task => "task",
            Self::Checkpoint => "checkpoint",
            Self::Artifact => "artifact",
            Self::Action => "action",
        }
    }
    pub fn scope(self) -> String {
        format!("{}:write", self.name())
    }
}
/// Named scopes accepted by the existing v1 grant format.
pub const SCOPES: [&str; 7] = [
    "read",
    "memory:write",
    "decision:write",
    "task:write",
    "checkpoint:write",
    "artifact:write",
    "action:write",
];
pub(crate) fn valid_client(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 80
        && s.as_bytes()[0].is_ascii_alphanumeric()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}
pub(crate) fn hex_digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(crate) fn hash(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}
pub(crate) fn object(v: &Value) -> Result<&Map<String, Value>> {
    v.as_object().ok_or_else(|| invalid("Expected an object"))
}
pub(crate) fn strict(v: &Value, allowed: &[&str]) -> Result<()> {
    if object(v)?.keys().any(|k| !allowed.contains(&k.as_str())) {
        return Err(invalid("Unknown input field"));
    }
    Ok(())
}
pub(crate) fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid(format!("Missing or invalid {key}")))
}
pub(crate) fn integer(v: &Value, key: &str, min: u64, max: u64) -> Result<u64> {
    let number = v
        .get(key)
        .ok_or_else(|| invalid(format!("Invalid {key}")))?;
    let n = number
        .as_u64()
        .or_else(|| {
            number.as_f64().and_then(|f| {
                if f.is_finite() && f.fract() == 0.0 && (0.0..=9_007_199_254_740_991.0).contains(&f)
                {
                    Some(f as u64)
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| invalid(format!("Invalid {key}")))?;
    if n < min || n > max {
        return Err(invalid(format!("Invalid {key}")));
    }
    Ok(n)
}
pub(crate) fn id(s: &str) -> Result<()> {
    let special =
        s == "00000000-0000-0000-0000-000000000000" || s == "ffffffff-ffff-ffff-ffff-ffffffffffff";
    let version_and_variant = s
        .as_bytes()
        .get(14)
        .is_some_and(|c| (b'1'..=b'8').contains(c))
        && s.as_bytes().get(19).is_some_and(|c| b"89abAB".contains(c));
    if s.len() != 36 || uuid::Uuid::parse_str(s).is_err() || !(special || version_and_variant) {
        return Err(invalid("Invalid UUID"));
    }
    Ok(())
}
fn js_space(c: char) -> bool {
    matches!(c,'\u{0009}'..='\u{000d}'|'\u{0020}'|'\u{00a0}'|'\u{1680}'|'\u{2000}'..='\u{200a}'|'\u{2028}'|'\u{2029}'|'\u{202f}'|'\u{205f}'|'\u{3000}'|'\u{feff}')
}
/// Trim search input with the v1 JavaScript whitespace semantics and UTF-16 bounds.
pub fn normalize_search_query(source: &str) -> Result<String> {
    let query = source.trim_matches(js_space);
    if query.is_empty() || query.encode_utf16().count() > 200 {
        return Err(invalid("Invalid search query"));
    }
    Ok(query.to_owned())
}
fn text(v: &mut Value, key: &str, min: usize, max: usize, trim: bool) -> Result<()> {
    let source = string(v, key)?;
    let s = if trim {
        source.trim_matches(js_space)
    } else {
        source
    };
    let n = s.encode_utf16().count();
    if n < min || n > max {
        return Err(invalid(format!("Invalid {key} length")));
    }
    v[key] = Value::String(s.into());
    Ok(())
}
fn choice(v: &Value, key: &str, values: &[&str]) -> Result<()> {
    if !values.contains(&string(v, key)?) {
        return Err(invalid(format!("Invalid {key}")));
    }
    Ok(())
}
pub(crate) fn timestamp(s: &str) -> Result<DateTime<Utc>> {
    if !s.ends_with('Z') {
        return Err(invalid("Expected UTC timestamp"));
    }
    DateTime::parse_from_rfc3339(s)
        .map(|t| t.with_timezone(&Utc))
        .map_err(|_| invalid("Invalid timestamp"))
}
pub(crate) fn normalize_draft(mut v: Value) -> Result<Value> {
    let kind = Kind::parse(string(&v, "kind")?)?;
    let mut allowed = vec![
        "kind",
        "title",
        "content",
        "status",
        "references",
        "sources",
    ];
    match kind {
        Kind::Memory => allowed.push("basis"),
        Kind::Checkpoint => allowed.extend(["nextStep", "completed", "openQuestions"]),
        Kind::Artifact => allowed.push("mediaType"),
        Kind::Action => allowed.push("destination"),
        _ => {}
    }
    strict(&v, &allowed)?;
    text(&mut v, "title", 1, 240, true)?;
    text(&mut v, "content", 1, 12000, true)?;
    let states: &[&str] = match kind {
        Kind::Memory | Kind::Artifact => &["active", "retracted"],
        Kind::Decision => &["proposed", "confirmed", "superseded", "retracted"],
        Kind::Task => &["todo", "doing", "blocked", "done", "cancelled"],
        Kind::Checkpoint => &["active", "superseded"],
        Kind::Action => &["pending", "succeeded", "failed", "unknown"],
    };
    choice(&v, "status", states)?;
    for key in ["references", "sources"] {
        if v.get(key).is_none() {
            v[key] = Value::Array(vec![]);
        }
    }
    let refs = v["references"]
        .as_array()
        .ok_or_else(|| invalid("Invalid references"))?;
    if refs.len() > 30 {
        return Err(invalid("Too many references"));
    }
    for r in refs {
        id(r.as_str().ok_or_else(|| invalid("Invalid reference"))?)?;
    }
    let sources = v["sources"]
        .as_array_mut()
        .ok_or_else(|| invalid("Invalid sources"))?;
    if sources.len() > 20 {
        return Err(invalid("Too many sources"));
    }
    for source in sources {
        strict(source, &["label", "uri"])?;
        text(source, "label", 1, 200, true)?;
        let uri = string(source, "uri")?;
        if !uri.starts_with("https://") || url::Url::parse(uri).is_err() {
            return Err(invalid("Sources must use HTTPS"));
        }
    }
    match kind {
        Kind::Memory => choice(
            &v,
            "basis",
            &["owner_statement", "source", "model_inference"],
        )?,
        Kind::Artifact => choice(&v, "mediaType", &["text/plain", "text/markdown"])?,
        Kind::Action => text(&mut v, "destination", 1, 500, true)?,
        Kind::Checkpoint => {
            text(&mut v, "nextStep", 1, 4000, true)?;
            for key in ["completed", "openQuestions"] {
                if v.get(key).is_none() {
                    v[key] = Value::Array(vec![]);
                }
                let a = v[key]
                    .as_array()
                    .ok_or_else(|| invalid("Invalid checkpoint list"))?;
                if a.len() > 30
                    || a.iter()
                        .any(|s| s.as_str().is_none_or(|s| s.encode_utf16().count() > 2000))
                {
                    return Err(invalid("Invalid checkpoint list"));
                }
            }
        }
        _ => {}
    }
    Ok(v)
}
#[derive(Debug)]
pub(crate) struct Write {
    pub value: Value,
    pub kind: Kind,
}
impl Write {
    pub fn parse(mut v: Value) -> Result<Self> {
        strict(
            &v,
            &[
                "workspaceId",
                "generation",
                "operationId",
                "id",
                "expectedRevision",
                "record",
            ],
        )?;
        for key in ["workspaceId", "generation", "operationId"] {
            id(string(&v, key)?)?;
        }
        let rev = integer(&v, "expectedRevision", 0, 9007199254740991)?;
        v["expectedRevision"] = serde_json::json!(rev);
        if let Some(rid) = v.get("id") {
            id(rid.as_str().ok_or_else(|| invalid("Invalid record ID"))?)?;
        } else if rev != 0 {
            return Err(invalid("New records require revision zero"));
        }
        v["record"] = normalize_draft(
            v.get("record")
                .cloned()
                .ok_or_else(|| invalid("Missing record"))?,
        )?;
        Ok(Self {
            kind: Kind::parse(string(&v["record"], "kind")?)?,
            value: v,
        })
    }
}
pub(crate) fn parse_record(v: Value) -> Result<Value> {
    let mut draft = object(&v)?.clone();
    for key in ["id", "revision", "actor", "createdAt", "updatedAt"] {
        draft.remove(key);
    }
    id(string(&v, "id")?)?;
    let revision = integer(&v, "revision", 1, 9007199254740991)?;
    let actor = string(&v, "actor")?;
    if actor.is_empty() || actor.encode_utf16().count() > 80 {
        return Err(invalid("Invalid actor"));
    }
    let created = timestamp(string(&v, "createdAt")?)?;
    let updated = timestamp(string(&v, "updatedAt")?)?;
    if updated < created {
        return Err(invalid("Invalid record timestamps"));
    }
    let mut out = normalize_draft(Value::Object(draft))?;
    for key in ["id", "revision", "actor", "createdAt", "updatedAt"] {
        out[key] = v[key].clone();
    }
    out["revision"] = serde_json::json!(revision);
    Ok(out)
}
pub(crate) fn validate_workspace(v: &Value) -> Result<()> {
    strict(v, &["id", "generation", "schemaVersion", "createdAt"])?;
    id(string(v, "id")?)?;
    id(string(v, "generation")?)?;
    integer(v, "schemaVersion", 1, 1)?;
    timestamp(string(v, "createdAt")?)?;
    Ok(())
}
pub(crate) fn validate_persona(v: &Value) -> Result<()> {
    strict(v, &["content", "hash"])?;
    let c = string(v, "content")?;
    if c.is_empty()
        || c.encode_utf16().count() > 32000
        || !hex_digest(string(v, "hash")?)
        || hash(c) != string(v, "hash")?
    {
        return Err(invalid("Persona digest or content is invalid"));
    }
    Ok(())
}
// serde_json's default ordered map sorts these closed ASCII field names exactly as v1.
pub(crate) fn canonical(v: &Value) -> String {
    v.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uuid_validation_matches_v1_versions() {
        assert!(id("00000000-0000-0000-0000-000000000000").is_ok());
        assert!(id("ffffffff-ffff-ffff-ffff-ffffffffffff").is_ok());
        assert!(id("12345678-1234-9123-8123-123456789abc").is_err());
        assert!(id("12345678-1234-4123-0123-123456789abc").is_err());
    }
    #[test]
    fn utf16_limits_and_unknown_fields() {
        let s = "🫖".repeat(121);
        assert!(normalize_draft(serde_json::json!({"kind":"memory","title":s,"content":"x","status":"active","basis":"owner_statement"})).is_err());
        assert!(normalize_draft(serde_json::json!({"kind":"memory","title":"x","content":"x","status":"active","basis":"owner_statement","actor":"owner"})).is_err());
    }
}
