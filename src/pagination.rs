//! Shared retrieval filters, presentation options and opaque pagination cursors for
//! `mother_get_context` and `mother_search_context`.
//!
//! Cursors are positions, not grants: every page re-applies the caller's current
//! authorization and filters. A cursor is bound to the query that issued it, so it
//! cannot be replayed against different filters, another client or a restored
//! workspace generation.
use crate::{
    contracts::{self, Kind},
    error::{Error, Result, invalid},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Default characters (UTF-16 units) of each record excerpt.
pub const DEFAULT_PREVIEW_CHARS: usize = 400;
/// Largest excerpt a caller may request; full records come from `mother_get_record`.
pub const MAX_PREVIEW_CHARS: usize = 2000;
/// Parse the caller's optional limit for the serialized MCP tool result.
pub fn response_budget(args: &Value) -> Result<Option<usize>> {
    args.get("maxResponseBytes")
        .map(|_| contracts::integer(args, "maxResponseBytes", 4096, 12_288).map(|n| n as usize))
        .transpose()
}
/// Records revised behind a cursor that one page may report before a restart is needed.
pub const MAX_CHANGED: usize = 10;
const MAX_CURSOR_CHARS: usize = 2048;
const MAX_WATERMARK_IDS: usize = 32;
const STATUSES: &[&str] = &[
    "active",
    "proposed",
    "confirmed",
    "todo",
    "doing",
    "blocked",
    "done",
    "pending",
    "succeeded",
    "failed",
    "unknown",
];
const HISTORICAL: &[&str] = &["retracted", "superseded", "cancelled"];
const BASES: &[&str] = &["owner_statement", "source", "model_inference"];

/// Server-side record filters shared by context and search.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Filters {
    pub(crate) actor: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) basis: Option<String>,
}

fn optional<'a>(v: &'a Value, key: &str) -> Result<Option<&'a str>> {
    v.get(key).map(|_| contracts::string(v, key)).transpose()
}

impl Filters {
    pub(crate) fn parse(v: &Value) -> Result<Self> {
        let actor = optional(v, "actor")?;
        if let Some(actor) = actor
            && (actor.is_empty() || actor.len() > 200 || actor.chars().any(char::is_control))
        {
            return Err(invalid("actor must be 1-200 printable characters"));
        }
        let status = optional(v, "status")?;
        if let Some(status) = status {
            if HISTORICAL.contains(&status) {
                return Err(invalid(
                    "Context and search cover recallable records only; read retracted, superseded or cancelled versions with mother_get_record_history.",
                ));
            }
            if !STATUSES.contains(&status) {
                return Err(invalid("Unknown record status"));
            }
        }
        let basis = optional(v, "basis")?;
        if basis.is_some_and(|b| !BASES.contains(&b)) {
            return Err(invalid(
                "basis must be owner_statement, source or model_inference",
            ));
        }
        Ok(Self {
            actor: actor.map(str::to_owned),
            status: status.map(str::to_owned),
            basis: basis.map(str::to_owned),
        })
    }
    pub(crate) fn any(&self) -> bool {
        self.actor.is_some() || self.status.is_some() || self.basis.is_some()
    }
    pub(crate) fn matches(&self, r: &Value) -> bool {
        self.actor
            .as_deref()
            .is_none_or(|a| r["actor"].as_str() == Some(a))
            && self
                .status
                .as_deref()
                .is_none_or(|s| r["status"].as_str() == Some(s))
            && self
                .basis
                .as_deref()
                .is_none_or(|b| r["basis"].as_str() == Some(b))
    }
    pub(crate) fn describe(&self) -> Value {
        json!({"actor":self.actor,"status":self.status,"basis":self.basis})
    }
}

/// Which optional sections a caller wants and how long excerpts may be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct View {
    pub(crate) include_persona: bool,
    pub(crate) include_pinned: bool,
    pub(crate) preview_chars: usize,
}

impl View {
    pub(crate) fn parse(v: &Value) -> Result<Self> {
        let flag = |key: &str| match v.get(key) {
            None => Ok(true),
            Some(Value::Bool(b)) => Ok(*b),
            Some(_) => Err(invalid(format!("{key} must be true or false"))),
        };
        let preview_chars = if v.get("previewMaxChars").is_some() {
            contracts::integer(v, "previewMaxChars", 0, MAX_PREVIEW_CHARS as u64)? as usize
        } else {
            DEFAULT_PREVIEW_CHARS
        };
        Ok(Self {
            include_persona: flag("includePersona")?,
            include_pinned: flag("includePinned")?,
            preview_chars,
        })
    }
}

/// Excerpt of at most `max` UTF-16 units, never splitting a character.
pub(crate) fn excerpt(content: &str, max: usize) -> String {
    let mut used = 0;
    content
        .chars()
        .take_while(|c| {
            used += c.len_utf16();
            used <= max
        })
        .collect()
}

/// Context ordering. `Balanced` is the unfiltered default and is a single overview page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Order {
    Balanced,
    /// Newest `updatedAt` first, then `id`.
    Recent,
    /// Oldest `createdAt` first, then `id`. Revisions never move a record.
    Created,
}

impl Order {
    pub(crate) fn parse(v: &str) -> Result<Self> {
        match v {
            "balanced" => Ok(Self::Balanced),
            "recent" => Ok(Self::Recent),
            "created" => Ok(Self::Created),
            _ => Err(invalid("order must be balanced, recent or created")),
        }
    }
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Balanced => "balanced",
            Self::Recent => "recent",
            Self::Created => "created",
        }
    }
    /// The record's position key in this order.
    pub(crate) fn key(self, r: &Value) -> (String, String) {
        let field = if self == Self::Created {
            "createdAt"
        } else {
            "updatedAt"
        };
        (
            r[field].as_str().unwrap_or_default().to_owned(),
            r["id"].as_str().unwrap_or_default().to_owned(),
        )
    }
    /// Whether key `a` comes strictly before key `b` in this order.
    pub(crate) fn before(self, a: &(String, String), b: &(String, String)) -> bool {
        match self {
            Self::Created | Self::Balanced => a < b,
            Self::Recent => a.0 > b.0 || (a.0 == b.0 && a.1 < b.1),
        }
    }
    pub(crate) fn compare(self, a: &Value, b: &Value) -> std::cmp::Ordering {
        let (ka, kb) = (self.key(a), self.key(b));
        if ka == kb {
            std::cmp::Ordering::Equal
        } else if self.before(&ka, &kb) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

/// Opaque cursor state. `k`/`i` locate the last returned record; `pk`/`pi` the request's
/// own position, so a presentation layer that drops records can re-mint a cursor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    v: u8,
    /// "context" or "search".
    t: String,
    /// Hash binding the cursor to its query, client and workspace generation.
    h: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    o: Option<Order>,
    #[serde(default)]
    k: String,
    #[serde(default)]
    i: String,
    #[serde(default)]
    pk: String,
    #[serde(default)]
    pi: String,
    /// Highest `updatedAt` among matching records when the page was issued.
    #[serde(default)]
    w: String,
    /// Matching record IDs whose `updatedAt` equals `w`.
    #[serde(default)]
    wi: Vec<String>,
    /// For a byte-limited page with many tied timestamps, carry a digest of the
    /// complete tie group. Changing its membership requires restarting that page chain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wh: Option<String>,
    /// Search only: result offset and the ranking stamp it is valid for.
    #[serde(default)]
    off: usize,
    #[serde(default)]
    st: String,
}

fn bad_cursor() -> Error {
    invalid("Invalid cursor. Restart pagination without a cursor.")
}

fn stale(message: &str) -> Error {
    Error::domain("CURSOR_STALE", message.to_owned())
}

impl Cursor {
    pub fn encode(&self) -> String {
        base64::encode_config(
            serde_json::to_vec(self).unwrap_or_default(),
            base64::URL_SAFE_NO_PAD,
        )
    }
    pub fn decode(text: &str) -> Result<Self> {
        if text.is_empty() || text.len() > MAX_CURSOR_CHARS {
            return Err(bad_cursor());
        }
        let bytes =
            base64::decode_config(text, base64::URL_SAFE_NO_PAD).map_err(|_| bad_cursor())?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| bad_cursor())?;
        if cursor.v != 1 {
            return Err(bad_cursor());
        }
        Ok(cursor)
    }
    pub(crate) fn order(&self) -> Option<Order> {
        self.o
    }
    fn parse(v: &Value, kind: &str, binding: &str) -> Result<Option<Self>> {
        let Some(text) = optional(v, "cursor")? else {
            return Ok(None);
        };
        let cursor = Self::decode(text)?;
        if cursor.t != kind {
            return Err(bad_cursor());
        }
        if cursor.h != binding {
            return Err(invalid(
                "This cursor belongs to a different query, client or workspace generation. Repeat the original filters or restart without a cursor.",
            ));
        }
        Ok(Some(cursor))
    }
    /// Re-position a context cursor after the last record that was actually returned.
    /// Used by presentation layers that drop records to fit a response budget.
    #[doc(hidden)]
    pub fn reposition(text: &str, last: Option<&Value>) -> Option<String> {
        let mut cursor = Self::decode(text).ok()?;
        if cursor.t == "search" {
            return None;
        }
        let order = cursor.o?;
        match last {
            Some(r) => (cursor.k, cursor.i) = order.key(r),
            None => (cursor.k, cursor.i) = (cursor.pk.clone(), cursor.pi.clone()),
        }
        Some(cursor.encode())
    }
    /// Move a search cursor back by `dropped` results that were not returned.
    #[doc(hidden)]
    pub fn rewind(text: &str, dropped: usize) -> Option<String> {
        let mut cursor = Self::decode(text).ok()?;
        if cursor.t != "search" {
            return None;
        }
        cursor.off = cursor.off.checked_sub(dropped)?;
        Some(cursor.encode())
    }
}

/// Binding hash of the filters that define one paginated query.
pub(crate) fn binding(parts: &Value) -> String {
    // 128 bits identify the query; the cursor is a position, never an authorization.
    contracts::hash(&parts.to_string())[..32].to_owned()
}

/// One keyset page over already filtered, authorized records.
pub(crate) struct KeysetPage {
    pub(crate) records: Vec<Value>,
    pub(crate) pagination: Value,
}

/// Page `matching` (filtered, authorized, recallable) in `order` after `cursor`.
///
/// Consistency: positions are record keys, not offsets, so removals never shift later
/// pages. In `created` order revisions never move a record and new records arrive at
/// the end. In `recent` order a revised record moves ahead of the cursor. Any matching
/// record changed since the previous page and now positioned behind the cursor is
/// reported once in `pagination.changed`, so nothing is skipped silently.
pub(crate) fn keyset(
    mut matching: Vec<Value>,
    order: Order,
    limit: usize,
    args: &Value,
    binding: &str,
) -> Result<KeysetPage> {
    let cursor = Cursor::parse(args, "context", binding)?;
    if cursor.as_ref().is_some_and(|c| c.o != Some(order)) {
        return Err(invalid(
            "This cursor was issued for a different order. Repeat the original order.",
        ));
    }
    matching.sort_by(|a, b| order.compare(a, b));
    let watermark = matching
        .iter()
        .filter_map(|r| r["updatedAt"].as_str())
        .max()
        .unwrap_or_default()
        .to_owned();
    let ids_at = |stamp: &str| {
        let mut ids: Vec<&str> = matching
            .iter()
            .filter(|r| r["updatedAt"].as_str() == Some(stamp))
            .filter_map(|r| r["id"].as_str())
            .collect();
        ids.sort_unstable();
        ids
    };
    let tied = ids_at(&watermark);
    let digest = |ids: &[&str]| contracts::hash(&ids.join(","));
    let mut watermark_hash = None;
    let mut at_watermark = Vec::new();
    if args.get("maxResponseBytes").is_some() && tied.len() > 4 {
        watermark_hash = Some(digest(&tied));
    } else {
        at_watermark.extend(
            tied.iter()
                .take(MAX_WATERMARK_IDS)
                .map(|id| (*id).to_owned()),
        );
    }
    if let Some(c) = &cursor
        && let Some(previous) = &c.wh
    {
        let current = if c.w == watermark {
            digest(&tied)
        } else {
            digest(&ids_at(&c.w))
        };
        if *previous != current {
            return Err(stale(
                "Records sharing a timestamp changed during pagination. Restart without a cursor.",
            ));
        }
    }
    // An empty position is the start of the order: nothing is behind it.
    let position = cursor
        .as_ref()
        .map(|c| (c.k.clone(), c.i.clone()))
        .filter(|p| !p.0.is_empty() || !p.1.is_empty());
    let behind = |r: &Value| {
        position.as_ref().is_some_and(|p| {
            let key = order.key(r);
            key == *p || order.before(&key, p)
        })
    };
    let mut changed = Vec::new();
    if let Some(c) = &cursor {
        for r in matching.iter().filter(|r| behind(r)) {
            let updated = r["updatedAt"].as_str().unwrap_or_default();
            let id = r["id"].as_str().unwrap_or_default();
            if updated > c.w.as_str()
                || (updated == c.w && c.wh.is_none() && !c.wi.iter().any(|w| w == id))
            {
                changed.push(json!({"id":r["id"],"kind":r["kind"],"title":r["title"],"status":r["status"],"revision":r["revision"],"updatedAt":r["updatedAt"]}));
            }
        }
        if changed.len() > MAX_CHANGED {
            return Err(stale(
                "More records changed during pagination than one page can report. Restart pagination without a cursor.",
            ));
        }
    }
    let start = matching.partition_point(|r| behind(r));
    let position = position.unwrap_or_default();
    let after = matching.split_off(start);
    let remaining = after.len();
    let records: Vec<Value> = after.into_iter().take(limit).collect();
    let (k, i) = records
        .last()
        .map(|r| order.key(r))
        .unwrap_or_else(|| position.clone());
    let next = Cursor {
        v: 1,
        t: "context".into(),
        h: binding.into(),
        o: Some(order),
        k,
        i,
        pk: position.0,
        pi: position.1,
        w: watermark,
        wi: at_watermark,
        wh: watermark_hash,
        off: 0,
        st: String::new(),
    };
    let has_more = remaining > records.len();
    let pagination = json!({
        "order": order.name(),
        "returnedRecords": records.len(),
        "remainingAfterPage": remaining - records.len(),
        "hasMore": has_more,
        "nextCursor": next.encode(),
        "changed": changed,
        "consistency": "keyset_positions",
    });
    Ok(KeysetPage {
        records,
        pagination,
    })
}

/// Search pages: offset over one ranking, valid while that ranking is unchanged.
pub(crate) fn search_offset(args: &Value, binding: &str, stamp: &str) -> Result<usize> {
    match Cursor::parse(args, "search", binding)? {
        None => Ok(0),
        Some(c) if c.st == stamp => Ok(c.off),
        Some(_) => Err(stale(
            "Search results changed since this cursor was issued (records, grant or semantic index progress). Restart without a cursor, or use mother_get_context with order \"created\" for a stable enumeration.",
        )),
    }
}

pub(crate) fn search_cursor(binding: &str, stamp: &str, offset: usize) -> String {
    Cursor {
        v: 1,
        t: "search".into(),
        h: binding.into(),
        o: None,
        k: String::new(),
        i: String::new(),
        pk: String::new(),
        pi: String::new(),
        w: String::new(),
        wi: Vec::new(),
        wh: None,
        off: offset,
        st: stamp.into(),
    }
    .encode()
}

/// Validate an optional kind filter.
pub(crate) fn kind(v: &Value) -> Result<Option<String>> {
    let kind = optional(v, "kind")?;
    if let Some(k) = kind {
        Kind::parse(k)?;
    }
    Ok(kind.map(str::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_budget_cursor_can_cross_a_large_group_with_one_timestamp() {
        let records = (0..80u128)
            .map(|i| {
                json!({
                    "id": uuid::Uuid::from_u128(i+1).to_string(),
                    "updatedAt":"2026-01-02T03:04:05.000Z"
                })
            })
            .collect::<Vec<_>>();
        let mut args = json!({"maxResponseBytes":4096});
        let mut seen = Vec::new();
        for _ in 0..records.len() {
            let page = keyset(records.clone(), Order::Recent, 3, &args, "same-query").unwrap();
            seen.extend(page.records);
            if page.pagination["hasMore"] == false {
                break;
            }
            let cursor = page.pagination["nextCursor"].as_str().unwrap();
            assert!(cursor.len() < 650);
            args["cursor"] = json!(cursor);
        }
        assert_eq!(seen, records);
        let mut changed = records;
        changed.remove(0);
        assert_eq!(
            keyset(changed, Order::Recent, 3, &args, "same-query")
                .err()
                .unwrap()
                .code(),
            "CURSOR_STALE"
        );
    }
}
