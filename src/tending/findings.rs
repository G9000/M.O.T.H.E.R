use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Evidence {
    pub(super) id: String,
    pub(super) revision: u64,
    author: String,
    recorded_at: String,
    updated_at: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Kind {
    OverlappingAccounts,
    PossibleContradiction,
    PossibleMilestoneGap,
    StaleFollowUp,
}

impl Kind {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::OverlappingAccounts => "Overlapping accounts",
            Self::PossibleContradiction => "Possible contradiction",
            Self::PossibleMilestoneGap => "Possible milestone gap",
            Self::StaleFollowUp => "Stale follow-up",
        }
    }

    fn reason(self) -> &'static str {
        match self {
            Self::OverlappingAccounts => {
                "Separate records use the same topic and substantially overlapping wording."
            }
            Self::PossibleContradiction => {
                "The reviewed records make apparently opposite statements about the same topic."
            }
            Self::PossibleMilestoneGap => {
                "A record refers to a milestone. This reviewed slice contains no separately recorded support identified by the selector."
            }
            Self::StaleFollowUp => {
                "An open task has passed the configured age threshold since its last update."
            }
        }
    }

    fn action(self) -> &'static str {
        match self {
            Self::OverlappingAccounts => {
                "Compare the original accounts and add a note about how they are related."
            }
            Self::PossibleContradiction => {
                "Read both accounts together and check whether dates, wording or evidence resolve the difference."
            }
            Self::PossibleMilestoneGap => {
                "Check whether you have an authorized supporting source, or leave the outcome open."
            }
            Self::StaleFollowUp => {
                "Check whether the task is still relevant and record the current next step separately."
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Finding {
    pub(super) id: String,
    pub(super) kind: Kind,
    pub(super) evidence: Vec<Evidence>,
    pub(super) reason: &'static str,
    pub(super) suggested_action: &'static str,
    basis: &'static str,
}

struct Source<'a> {
    evidence: Evidence,
    kind: &'a str,
    status: &'a str,
    title_key: String,
    content: &'a str,
    has_linked_source: bool,
}

impl<'a> Source<'a> {
    fn from_record(value: &'a Value) -> Option<Self> {
        let kind = value["kind"].as_str()?;
        let status = value["status"].as_str()?;
        let title = value["title"].as_str()?;
        if !matches!(kind, "memory" | "decision" | "task")
            || matches!(status, "retracted" | "superseded" | "cancelled")
            || title.starts_with("Gardening annotation | ")
        {
            return None;
        }
        Some(Self {
            evidence: Evidence {
                id: value["id"].as_str()?.to_owned(),
                revision: value["revision"].as_u64()?,
                author: value["actor"].as_str()?.to_owned(),
                recorded_at: value["createdAt"].as_str()?.to_owned(),
                updated_at: value["updatedAt"].as_str()?.to_owned(),
            },
            kind,
            status,
            title_key: normalized(title),
            content: value["content"].as_str()?,
            has_linked_source: ["references", "sources"].iter().any(|key| {
                value[*key]
                    .as_array()
                    .is_some_and(|items| !items.is_empty())
            }),
        })
    }
}

fn normalized(text: &str) -> String {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join(" ")
}

fn claim(text: &str) -> Option<(bool, String)> {
    // Compare one short declarative assertion. Quoted, conditional and compound text
    // is not parsed as a position.
    let text = text.trim().trim_end_matches('.');
    if text.chars().count() > 160
        || !text
            .chars()
            .all(|ch| ch.is_alphanumeric() || matches!(ch, ' ' | '-'))
        || [
            " if ", " unless ", " but ", " or ", " maybe ", " might ", " could ",
        ]
        .iter()
        .any(|word| text.to_lowercase().contains(word))
    {
        return None;
    }
    let phrase = normalized(text);
    for (positive, negative) in [
        (" was ", " was not "),
        (" is ", " is not "),
        (" did ", " did not "),
        (" were ", " were not "),
    ] {
        if let Some((subject, predicate)) = phrase.split_once(negative) {
            return (!subject.is_empty() && !predicate.is_empty())
                .then(|| (false, format!("{subject}{positive}{predicate}")));
        }
        if let Some((subject, predicate)) = phrase.split_once(positive) {
            return (!subject.is_empty() && !predicate.is_empty())
                .then_some((true, phrase.clone()));
        }
    }
    None
}

fn overlap(left: &str, right: &str) -> bool {
    let left = normalized(left);
    let right = normalized(right);
    let left: BTreeSet<_> = left.split_whitespace().collect();
    let right: BTreeSet<_> = right.split_whitespace().collect();
    let shared = left.intersection(&right).count();
    shared >= 4 && shared * 5 >= left.union(&right).count() * 3
}

fn milestone_gap(source: &Source<'_>, titles: &BTreeMap<&str, Vec<&Source<'_>>>) -> bool {
    if source.has_linked_source {
        return false;
    }
    source
        .content
        .lines()
        .filter_map(|line| {
            let (lead, name) = line.trim().split_once(':')?;
            lead.eq_ignore_ascii_case("milestone").then_some(name)
        })
        .take(3)
        .any(|name| {
            let topic = normalized(name);
            !topic.is_empty()
                && !titles.get(topic.as_str()).is_some_and(|matches| {
                    matches.iter().any(|other| {
                        other.evidence.id != source.evidence.id && other.kind != "task"
                    })
                })
        })
}

fn finding(workspace: &str, generation: &str, kind: Kind, mut evidence: Vec<Evidence>) -> Finding {
    evidence.sort_by(|a, b| a.id.cmp(&b.id));
    let fingerprint = serde_json::json!([
        workspace,
        generation,
        kind,
        evidence
            .iter()
            .map(|source| (&source.id, source.revision))
            .collect::<Vec<_>>()
    ]);
    let hash = Sha256::new()
        .chain_update(b"mother/memory-gardening/v2")
        .chain_update(fingerprint.to_string())
        .finalize();
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&hash[..16]);
    uuid[6] = (uuid[6] & 0x0f) | 0x80;
    uuid[8] = (uuid[8] & 0x3f) | 0x80;
    Finding {
        id: Uuid::from_bytes(uuid).to_string(),
        kind,
        evidence,
        reason: kind.reason(),
        suggested_action: kind.action(),
        basis: "suggested_interpretation",
    }
}

pub(super) fn assess(
    records: &[Value],
    workspace: &str,
    generation: &str,
    today: NaiveDate,
    stale_days: i64,
) -> Vec<Finding> {
    let mut sources = records
        .iter()
        .filter_map(Source::from_record)
        .collect::<Vec<_>>();
    sources.sort_by(|a, b| a.evidence.id.cmp(&b.evidence.id));
    let mut titles = BTreeMap::<&str, Vec<&Source<'_>>>::new();
    for source in &sources {
        titles.entry(&source.title_key).or_default().push(source);
    }
    let mut groups = BTreeMap::<Kind, Vec<Finding>>::new();
    for source in &sources {
        let single = |kind| finding(workspace, generation, kind, vec![source.evidence.clone()]);
        if milestone_gap(source, &titles) {
            groups
                .entry(Kind::PossibleMilestoneGap)
                .or_default()
                .push(single(Kind::PossibleMilestoneGap));
        }
        if source.kind == "task"
            && matches!(source.status, "todo" | "doing" | "blocked")
            && source
                .evidence
                .updated_at
                .parse::<DateTime<Utc>>()
                .is_ok_and(|date| (today - date.date_naive()).num_days() >= stale_days)
        {
            groups
                .entry(Kind::StaleFollowUp)
                .or_default()
                .push(single(Kind::StaleFollowUp));
        }
        if source.kind == "task" || source.title_key.is_empty() {
            continue;
        }
        for other in &titles[source.title_key.as_str()] {
            if other.kind == "task" || other.evidence.id <= source.evidence.id {
                continue;
            }
            let opposite = matches!((claim(source.content),claim(other.content)),
                (Some((left,a)),Some((right,b))) if a == b && left != right);
            let kind = if opposite {
                Kind::PossibleContradiction
            } else if overlap(source.content, other.content) {
                Kind::OverlappingAccounts
            } else {
                continue;
            };
            groups.entry(kind).or_default().push(finding(
                workspace,
                generation,
                kind,
                vec![source.evidence.clone(), other.evidence.clone()],
            ));
        }
    }
    // Interleave categories so a large cluster of overlaps cannot hide other actionable findings.
    let mut groups = groups.into_values().map(Vec::into_iter).collect::<Vec<_>>();
    let mut result = Vec::new();
    loop {
        let before = result.len();
        for group in &mut groups {
            if let Some(item) = group.next() {
                result.push(item);
            }
        }
        if result.len() == before {
            break;
        }
    }
    result
}
