use super::{Connection, reference, restart, unavailable};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use mother::{
    error::{Error, Result},
    garden::contracts::{
        AssertionObject, Direction, GardenDraft, GardenKind, GardenRecord, GetResponse, Lifecycle,
        RecordRef, SourceLocator, Traversal,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use serde::Serialize;
use serde_json::json;
use std::{collections::BTreeMap, time::Duration};

const HOPS: u32 = 2;
const EDGES: u32 = 24;
const EVIDENCE: usize = 3;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Neighborhood {
    status: &'static str,
    anchor: Node,
    memory_title: Option<String>,
    maximum_hops: u32,
    maximum_edges: u32,
    coverage: &'static str,
    edges: Vec<Edge>,
    #[serde(skip)]
    sequence: u64,
    #[serde(skip)]
    epoch: String,
}

impl Neighborhood {
    fn check_current(&self, conn: &Connection, memory: Option<&str>) -> Result<()> {
        if let Some(memory) = memory
            && conn.memory(memory)? != self.memory_title
        {
            return Err(restart());
        }
        let current = conn.get(reference(GardenKind::Entity, &self.anchor.id))?;
        if self.sequence != current.coverage.canonical_watermark
            || self.epoch != current.visibility_epoch
        {
            return Err(restart());
        }
        Ok(())
    }
}

#[derive(Clone, Serialize)]
struct Node {
    id: String,
    label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Edge {
    id: String,
    revision: u64,
    previous_revision: Option<u64>,
    direction: &'static str,
    predicate: String,
    from: Node,
    to: Node,
    contributor: String,
    revision_actor: String,
    approver: Option<String>,
    evidence: Vec<Evidence>,
    more_evidence: bool,
}

#[derive(Clone, Serialize)]
struct Evidence {
    id: String,
    revision: u64,
    contributor: String,
    source: String,
    excerpt: Option<String>,
}

pub(super) fn print_json(value: &impl Serialize) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub(super) fn message(status: &str, text: &str) -> Result<()> {
    print_json(&json!({"status":status,"message":text}))
}

pub(super) fn clean_label(text: &str) -> String {
    clean(text, 180)
}

fn clean(text: &str, max: usize) -> String {
    let mut chars = text.chars();
    let mut visible: String = chars.by_ref().take(max).map(|character| {
        if character.is_control() || matches!(character, '\u{061c}' | '\u{200e}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}') {
            ' '
        } else {
            character
        }
    }).collect();
    if chars.next().is_some() {
        visible.push('…');
    }
    visible
}

fn node(record: GardenRecord) -> Result<Node> {
    match record.draft {
        GardenDraft::Entity { display_name, .. }
            if record.metadata.lifecycle == Lifecycle::Active =>
        {
            Ok(Node {
                id: record.metadata.id,
                label: clean_label(&display_name),
            })
        }
        _ => Err(Error::domain(
            "NOT_FOUND",
            "The selected entity is unavailable.",
        )),
    }
}

fn evidence(record: GardenRecord) -> Result<Evidence> {
    match record.draft {
        GardenDraft::Evidence {
            source, excerpt, ..
        } if record.metadata.lifecycle == Lifecycle::Active => {
            let source = match source {
                SourceLocator::Uri { uri } => clean(&uri, 240),
                SourceLocator::Artifact {
                    sha256, media_type, ..
                } => format!("Artifact {} ({})", sha256, clean_label(&media_type)),
            };
            Ok(Evidence {
                id: record.metadata.id,
                revision: record.metadata.revision,
                contributor: clean_label(&record.metadata.contributor),
                source,
                excerpt: excerpt.map(|text| clean(&text, 320)),
            })
        }
        _ => Err(restart()),
    }
}

struct Snapshot<'a> {
    conn: &'a Connection,
    sequence: u64,
    epoch: String,
    nodes: BTreeMap<String, Node>,
    evidence: BTreeMap<String, Evidence>,
}

impl Snapshot<'_> {
    fn get(&self, kind: GardenKind, id: &str) -> Result<GetResponse> {
        self.conn
            .get(reference(kind, id))
            .map_err(|error| {
                if error.code() == "NOT_FOUND" {
                    restart()
                } else {
                    error
                }
            })
            .and_then(|object| {
                if object.coverage.canonical_watermark != self.sequence
                    || object.visibility_epoch != self.epoch
                {
                    Err(restart())
                } else {
                    Ok(object)
                }
            })
    }

    fn node(&mut self, id: &str) -> Result<Node> {
        if let Some(previous) = self.nodes.get(id) {
            return Ok(previous.clone());
        }
        let value = node(self.get(GardenKind::Entity, id)?.record)?;
        self.nodes.insert(id.into(), value.clone());
        Ok(value)
    }

    fn evidence(&mut self, id: &str) -> Result<Evidence> {
        if let Some(previous) = self.evidence.get(id) {
            return Ok(previous.clone());
        }
        let value = evidence(self.get(GardenKind::Evidence, id)?.record)?;
        self.evidence.insert(id.into(), value.clone());
        Ok(value)
    }
}

pub(super) fn read(
    conn: &Connection,
    anchor: &str,
    memory_id: Option<&str>,
) -> Result<Neighborhood> {
    let root = reference(GardenKind::Entity, anchor);
    let anchor = conn.get(root.clone())?;
    let identity = node(anchor.record)?;
    let mut query = conn.blank_query();
    query.kinds = vec![GardenKind::Assertion];
    query.limit = EDGES;
    query.traversal = Some(Traversal {
        start: vec![root],
        direction: Direction::Both,
        max_hops: HOPS,
        max_edges: EDGES,
        work_budget: 96,
    });
    let response = conn.query(&query).map_err(|error| {
        if error.code() == "FORBIDDEN" {
            unavailable()
        } else {
            error
        }
    })?;
    if response.coverage.canonical_watermark != anchor.coverage.canonical_watermark
        || response.page.visibility_epoch != anchor.visibility_epoch
    {
        return Err(restart());
    }
    let mut snapshot = Snapshot {
        conn,
        sequence: anchor.coverage.canonical_watermark,
        epoch: anchor.visibility_epoch,
        nodes: BTreeMap::from([(identity.id.clone(), identity.clone())]),
        evidence: BTreeMap::new(),
    };
    let mut edges = Vec::new();
    let partial = !response.coverage.complete || response.page.next_cursor.is_some();
    for hit in response.hits {
        let GardenDraft::Assertion {
            subject: RecordRef::Existing { id: from, .. },
            predicate,
            object:
                AssertionObject::EntityRef {
                    entity_ref: RecordRef::Existing { id: to, .. },
                },
            evidence: refs,
            ..
        } = hit.record.draft
        else {
            continue;
        };
        let direction = match (from == identity.id, to == identity.id) {
            (true, true) => "loop",
            (true, false) => "outgoing",
            (false, true) => "incoming",
            (false, false) => "neighbor",
        };
        let more_evidence = refs.len() > EVIDENCE;
        let mut evidence = Vec::new();
        for support in refs.into_iter().take(EVIDENCE) {
            if let RecordRef::Existing { id, .. } = support {
                evidence.push(snapshot.evidence(&id)?);
            }
        }
        let metadata = hit.record.metadata;
        edges.push(Edge {
            id: metadata.id,
            revision: metadata.revision,
            previous_revision: metadata.previous_revision,
            direction,
            predicate: clean_label(&predicate),
            from: snapshot.node(&from)?,
            to: snapshot.node(&to)?,
            contributor: clean_label(&metadata.contributor),
            revision_actor: clean_label(&metadata.actor),
            approver: metadata.approver.map(|value| clean_label(&value)),
            evidence,
            more_evidence,
        });
    }
    // Recheck the epoch and canonical cutoff at the end of multi-call hydration.
    let memory_title = match memory_id {
        Some(id) => Some(conn.memory(id)?.ok_or_else(|| {
            Error::domain(
                "NOT_FOUND",
                "The selected memory is unavailable under the current grant.",
            )
        })?),
        None => None,
    };
    snapshot.get(GardenKind::Entity, &identity.id)?;
    Ok(Neighborhood {
        status: if edges.is_empty() {
            "empty"
        } else {
            "available"
        },
        anchor: identity,
        memory_title,
        maximum_hops: HOPS,
        maximum_edges: EDGES,
        coverage: if partial {
            "partial"
        } else {
            "complete_within_neighborhood"
        },
        edges,
        sequence: snapshot.sequence,
        epoch: snapshot.epoch,
    })
}

fn edge_name(edge: &Edge) -> String {
    format!(
        "{} → {} · {}",
        edge.from.label,
        edge.to.label,
        edge.predicate
            .strip_prefix("living-memory:")
            .unwrap_or(&edge.predicate)
    )
}

fn edge_details(edge: &Edge) -> String {
    let mut details = format!(
        "{}\n{}\n{} · revision {}\nOriginal contributor: {}\nCurrent revision actor: {}\n",
        edge_name(edge),
        edge.direction,
        edge.id,
        edge.revision,
        edge.contributor,
        edge.revision_actor
    );
    if let Some(approver) = &edge.approver {
        details.push_str(&format!("Owner approver: {approver}\n"));
    }
    if let Some(previous) = edge.previous_revision {
        details.push_str(&format!(
            "Earlier revision: {previous} on the same assertion ID\n"
        ));
    }
    if edge.evidence.is_empty() {
        details.push_str("\nNo supporting evidence attached.");
    }
    for item in &edge.evidence {
        details.push_str(&format!(
            "\nEvidence {} · revision {}\nContributor: {}\nSource: {}\n",
            item.id, item.revision, item.contributor, item.source
        ));
        if let Some(excerpt) = &item.excerpt {
            details.push_str(excerpt);
            details.push('\n');
        }
    }
    if edge.more_evidence {
        details
            .push_str("\nMore authorized evidence is attached. Use the assertion ID for details.");
    }
    details
}

fn render(
    frame: &mut Frame,
    current: Option<&Neighborhood>,
    failure: Option<&str>,
    state: &mut ListState,
    scroll: u16,
) {
    let area = frame.area();
    let [heading, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(4),
        Constraint::Length(2),
    ])
    .areas(area);
    let help = "↑/↓ Select  ·  PgUp/PgDn Scroll evidence  ·  ←/→ Follow endpoints  ·  Backspace Back  ·  R Refresh  ·  Q Quit";
    frame.render_widget(Paragraph::new(help).wrap(Wrap { trim: true }), footer);
    let Some(view) = current else {
        frame.render_widget(
            Paragraph::new("Living Memory Graph").block(Block::bordered()),
            heading,
        );
        frame.render_widget(
            Paragraph::new(failure.unwrap_or_default()).wrap(Wrap { trim: true }),
            body,
        );
        return;
    };
    let selected = match &view.memory_title {
        Some(title) => format!("{title} → {}", view.anchor.label),
        None => view.anchor.label.clone(),
    };
    let coverage = if view.coverage == "partial" {
        "Partial coverage"
    } else {
        "Within two hops"
    };
    frame.render_widget(
        Paragraph::new(format!("{selected} · {coverage}"))
            .block(Block::bordered().title(" Living Memory Graph ")),
        heading,
    );
    if view.edges.is_empty() {
        frame.render_widget(
            Paragraph::new("No visible relationships in this neighborhood.")
                .wrap(Wrap { trim: true }),
            body,
        );
        return;
    }
    let constraints = [Constraint::Percentage(48), Constraint::Percentage(52)];
    let [list_area, details_area] = if body.width >= 92 {
        Layout::horizontal(constraints).areas(body)
    } else {
        Layout::vertical(constraints).areas(body)
    };
    let rows = view.edges.iter().map(|edge| ListItem::new(edge_name(edge)));
    let list = List::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Directed relationships "),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("› ");
    frame.render_stateful_widget(list, list_area, state);
    let details = state
        .selected()
        .and_then(|index| view.edges.get(index))
        .map_or_else(String::new, edge_details);
    frame.render_widget(
        Paragraph::new(details)
            .block(Block::bordered().title(" Provenance and evidence "))
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0)),
        details_area,
    );
}

pub(super) fn run(conn: &Connection, first: Neighborhood, memory_id: Option<String>) -> Result<()> {
    let mut terminal = ratatui::try_init()?;
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            ratatui::restore();
        }
    }
    let _restore = Restore;
    let mut anchor = (first.anchor.id.clone(), memory_id);
    let mut current = Some(first);
    let mut failure = None;
    let mut visited = Vec::new();
    let mut list = ListState::default().with_selected(Some(0));
    let mut scroll = 0u16;
    loop {
        discard_stale(conn, anchor.1.as_deref(), &mut current, &mut failure);
        terminal.draw(|frame| {
            render(
                frame,
                current.as_ref(),
                failure.as_deref(),
                &mut list,
                scroll,
            )
        })?;
        if !event::poll(Duration::from_secs(2))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind == KeyEventKind::Release {
            continue;
        }
        discard_stale(conn, anchor.1.as_deref(), &mut current, &mut failure);
        let edge = current
            .as_ref()
            .and_then(|view| list.selected().and_then(|index| view.edges.get(index)));
        let mut reload = false;
        match key.code {
            KeyCode::Char('q' | 'Q') | KeyCode::Esc => break,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
            KeyCode::Up => {
                list.select_previous();
                scroll = 0;
            }
            KeyCode::Down => {
                list.select_next();
                scroll = 0;
            }
            KeyCode::PageUp => scroll = scroll.saturating_sub(6),
            KeyCode::PageDown => scroll = scroll.saturating_add(6).min(2_000),
            KeyCode::Char('r' | 'R') => reload = true,
            KeyCode::Backspace => {
                if let Some(previous) = visited.pop() {
                    anchor = previous;
                    reload = true;
                }
            }
            KeyCode::Left | KeyCode::Right => {
                let Some(edge) = edge else {
                    continue;
                };
                let endpoint = if key.code == KeyCode::Left {
                    &edge.from
                } else {
                    &edge.to
                };
                if endpoint.id != anchor.0 {
                    if visited.len() == 32 {
                        visited.remove(0);
                    }
                    visited.push(anchor);
                    anchor = (endpoint.id.clone(), None);
                    reload = true;
                }
            }
            _ => {}
        }
        if reload {
            // Clear the last view before reading after potential permission changes.
            terminal.clear()?;
            match read(conn, &anchor.0, anchor.1.as_deref()) {
                Ok(view) => {
                    current = Some(view);
                    failure = None;
                }
                Err(_) => {
                    current = None;
                    failure = Some("The neighborhood is unavailable under current graph access. Press R to retry or Backspace to return.".to_string());
                }
            }
            list = ListState::default().with_selected(Some(0));
            scroll = 0;
        }
    }
    Ok(())
}

fn discard_stale(
    conn: &Connection,
    memory: Option<&str>,
    current: &mut Option<Neighborhood>,
    failure: &mut Option<String>,
) {
    if current
        .as_ref()
        .is_some_and(|view| view.check_current(conn, memory).is_err())
    {
        *current = None;
        *failure = Some("The displayed neighborhood changed or access is unavailable. Press R to load it again.".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relationships::Connection;
    use mother::{
        access::{AccessMode, ClientAccess},
        garden::{access::GardenAccess, contracts::BatchRequest, store::GardenStore},
        service::{LocalService, ServiceClientConfig},
    };
    use ratatui::{Terminal, backend::TestBackend};
    use std::{
        fs,
        sync::atomic::{AtomicBool, Ordering},
    };
    use uuid::Uuid;

    #[test]
    fn cached_evidence_is_cleared_before_interaction_after_grant_replacement() {
        let directory =
            tempfile::tempdir_in(fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(directory.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        let db = directory.path().join("synthetic.redb");
        let ws = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store = GardenStore::initialize(&db, &ws, &generation).unwrap();
        let all = GardenAccess::All {};
        let access = ClientAccess::default();
        let writer = store
            .grant(
                "writer",
                &["garden:read".into(), "garden:write".into()],
                &access,
                &all,
            )
            .unwrap();
        let reader = store
            .grant("viewer", &["garden:read".into()], &access, &all)
            .unwrap();
        let entity = |key, label| {
            json!({"itemKey":key,"mutation":"create","expectedRevision":0,
            "draft":{"kind":"entity","entityType":"test:idea","displayName":label,"aliases":[],"extensions":{}}})
        };
        let batch: BatchRequest = serde_json::from_value(json!({"version":2,"workspaceId":ws,"generation":generation,
            "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
                entity("a","Anchor idea"), entity("b","Neighbor idea"),
                {"itemKey":"source","mutation":"create","expectedRevision":0,"draft":{
                    "kind":"evidence","source":{"type":"uri","uri":"urn:synthetic:proof"},
                    "excerpt":"REVOKED EVIDENCE",
                    "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}
                }},
                {"itemKey":"link","mutation":"create","expectedRevision":0,"draft":{
                    "kind":"assertion","predicate":"living-memory:related_to",
                    "subject":{"refType":"item","kind":"entity","itemKey":"a"},
                    "object":{"type":"entityRef","entityRef":{"refType":"item","kind":"entity","itemKey":"b"}},
                    "validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},
                    "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
                    "evidence":[{"refType":"item","kind":"evidence","itemKey":"source"}],"confidence":null,"extensions":{}
                }}
            ]})).unwrap();
        let created = store.apply(&writer, &batch).unwrap().receipt.results;
        let anchor = created
            .iter()
            .find(|item| item.item_key == "a")
            .unwrap()
            .reference
            .id
            .clone();
        drop(store);
        let endpoint = directory.path().join("owner.json");
        let service = LocalService::bind(&db, &endpoint).unwrap();
        let config = directory.path().join("client.private.json");
        let bridge = |auth: &mother::contracts::Auth| ServiceClientConfig {
            version: 3,
            backend: "garden_service".into(),
            endpoint_path: endpoint.clone(),
            workspace_id: ws.clone(),
            generation: generation.clone(),
            client_id: auth.client_id.clone(),
            token: auth.token.clone(),
        };
        let stop = AtomicBool::new(false);
        std::thread::scope(|scope| {
            struct Stop<'a>(&'a AtomicBool);
            impl Drop for Stop<'_> {
                fn drop(&mut self) {
                    self.0.store(true, Ordering::Release);
                }
            }
            scope.spawn(|| service.serve(&stop).unwrap());
            let _stop = Stop(&stop);
            fs::write(&config, serde_json::to_vec(&bridge(&reader)).unwrap()).unwrap();
            let conn = Connection::open(&config).unwrap();
            let mut current = Some(read(&conn, &anchor, None).unwrap());
            assert!(edge_details(&current.as_ref().unwrap().edges[0]).contains("REVOKED EVIDENCE"));
            let new_auth = service
                .store()
                .grant(
                    "viewer",
                    &["garden:read".into()],
                    &ClientAccess {
                        mode: AccessMode::Shared,
                        subject: Some("synthetic".into()),
                        records: vec![],
                    },
                    &GardenAccess::Allowlist {
                        objects: created
                            .iter()
                            .filter(|item| item.item_key != "source")
                            .map(|item| item.reference.clone())
                            .collect(),
                    },
                )
                .unwrap();

            // This is the same revalidation performed before cached evidence can redraw on a key event.
            let mut failure = None;
            discard_stale(&conn, None, &mut current, &mut failure);
            assert!(current.is_none());
            let mut terminal = Terminal::new(TestBackend::new(116, 28)).unwrap();
            terminal
                .draw(|frame| {
                    render(
                        frame,
                        current.as_ref(),
                        failure.as_deref(),
                        &mut ListState::default(),
                        0,
                    )
                })
                .unwrap();
            let buffer = terminal.backend().buffer();
            assert!(!format!("{buffer:?}").contains("REVOKED EVIDENCE"));

            // The replacement grant can see both endpoints and the assertion ID, but not its evidence footprint.
            fs::write(&config, serde_json::to_vec(&bridge(&new_auth)).unwrap()).unwrap();
            let refreshed = read(&Connection::open(&config).unwrap(), &anchor, None).unwrap();
            assert!(refreshed.edges.is_empty());
            assert_eq!(refreshed.coverage, "complete_within_neighborhood");
        });
    }
}
