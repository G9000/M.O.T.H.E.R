use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{access::GardenAccess, contracts::*, store::GardenStore},
};
use redb::ReadableTable;
use serde_json::{Value, json};
use uuid::Uuid;

struct Fixture {
    _dir: tempfile::TempDir,
    store: GardenStore,
    ws: String,
    generation: String,
    proposer: Auth,
    writer: Auth,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let ws = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store =
            GardenStore::initialize(dir.path().join("garden.redb"), &ws, &generation).unwrap();
        let proposer = store
            .grant(
                "proposer",
                &["garden:propose".into()],
                &ClientAccess {
                    mode: AccessMode::Shared,
                    subject: Some("agent:a".into()),
                    records: vec![],
                },
                &GardenAccess::default(),
            )
            .unwrap();
        let writer = store
            .grant(
                "writer",
                &[
                    "garden:read".into(),
                    "garden:write".into(),
                    "garden:propose".into(),
                ],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        Self {
            _dir: dir,
            store,
            ws,
            generation,
            proposer,
            writer,
        }
    }
    fn batch(&self, items: Value) -> BatchRequest {
        serde_json::from_value(json!({"version":2,"workspaceId":self.ws,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items})).unwrap()
    }
    fn lookup(&self, op: &str) -> OutcomeRequest {
        serde_json::from_value(json!({"version":2,"workspaceId":self.ws,"generation":self.generation,"operationId":op,"operationGeneration":self.generation,"requiredExtensions":{}})).unwrap()
    }
    fn review(&self, id: &str, action: &str) -> Value {
        json!({"version":2,"workspaceId":self.ws,"generation":self.generation,"operationId":Uuid::new_v4().to_string(),"proposalId":id,"expectedRevision":1,"action":action})
    }
}
fn create(key: &str, name: &str) -> Value {
    json!({"itemKey":key,"mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":name,"aliases":[],"extensions":{}}})
}

#[test]
fn admission_is_separate_idempotent_and_scope_strict() {
    let f = Fixture::new();
    let b = f.batch(json!([create("a", "original")]));
    let p = f.store.propose(&f.proposer, &b).unwrap();
    assert_eq!(p.status, OutcomeStatus::Pending);
    assert_eq!(f.store.propose(&f.proposer, &b).unwrap(), p);
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 0);
    let original = f
        .store
        .receipt(&f.proposer, &f.lookup(&b.operation_id))
        .unwrap();
    assert_eq!(original.receipt.status, OutcomeStatus::Pending);
    assert_eq!(
        f.store
            .get_proposal(&f.proposer, &f.lookup(&b.operation_id))
            .unwrap()
            .original_proposal,
        Some(b.clone())
    );
    let mut changed = b.clone();
    changed.items[0].draft =
        Some(serde_json::from_value(create("a", "changed")["draft"].clone()).unwrap());
    assert_eq!(
        f.store.propose(&f.proposer, &changed).unwrap_err().code,
        GardenErrorCode::OperationReused
    );
    assert_eq!(
        f.store.apply(&f.proposer, &b).unwrap_err().code,
        GardenErrorCode::Forbidden
    );
}

#[test]
fn owner_review_preserves_original_and_marks_edits_additions_without_granting_payload() {
    let f = Fixture::new();
    let b = f.batch(json!([create("a", "original")]));
    let p = f.store.propose(&f.proposer, &b).unwrap();
    let mut review = f.review(&p.proposal_id, "approve");
    review["reviewedItems"] = json!([create("a", "edited"), create("owner", "private addition")]);
    let approved = f.store.review_proposal(review.clone()).unwrap();
    assert_eq!(approved.receipt.status, OutcomeStatus::Approved);
    assert_eq!(
        f.store.review_proposal(review).unwrap().receipt,
        approved.receipt
    );
    assert!(approved.receipt.results[0].owner_edited);
    assert!(!approved.receipt.results[0].owner_added);
    assert!(approved.receipt.results[1].owner_added);
    assert_eq!(approved.visible_records[0].metadata.contributor, "proposer");
    assert_eq!(
        approved.visible_records[0].metadata.approver.as_deref(),
        Some("@owner-review")
    );
    assert_eq!(
        approved.visible_records[1].metadata.contributor,
        "@owner-review"
    );
    let current = f
        .store
        .get_proposal(&f.proposer, &f.lookup(&b.operation_id))
        .unwrap();
    assert_eq!(current.receipt.status, OutcomeStatus::Approved);
    assert_eq!(current.receipt.results, approved.receipt.results);
    assert!(current.visible_records.is_empty());
    assert!(current.payload_omitted);
    assert_eq!(current.original_proposal, Some(b.clone()));
    assert_eq!(
        f.store
            .receipt(&f.proposer, &f.lookup(&b.operation_id))
            .unwrap()
            .receipt
            .status,
        OutcomeStatus::Pending
    );
    assert_eq!(
        f.store.owner_inbox_get(&p.proposal_id).unwrap()["original"],
        serde_json::to_value(b).unwrap()
    );
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 1);
}

#[test]
fn dismissal_has_no_canonical_mapping_or_retraction() {
    let f = Fixture::new();
    let b = f.batch(json!([create("a", "pending")]));
    let p = f.store.propose(&f.proposer, &b).unwrap();
    let review = f.review(&p.proposal_id, "dismiss");
    let out = f.store.review_proposal(review.clone()).unwrap();
    assert_eq!(out.receipt.status, OutcomeStatus::Dismissed);
    assert!(out.receipt.results.is_empty());
    assert_eq!(
        f.store.review_proposal(review).unwrap().receipt,
        out.receipt
    );
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 0);
    assert_eq!(
        f.store
            .get_proposal(&f.proposer, &f.lookup(&b.operation_id))
            .unwrap()
            .receipt
            .status,
        OutcomeStatus::Dismissed
    );
}

#[test]
fn current_revocation_blocks_review_persistently_and_retry_is_exact() {
    let f = Fixture::new();
    let p = f
        .store
        .propose(&f.proposer, &f.batch(json!([create("a", "pending")])))
        .unwrap();
    f.store.revoke("proposer").unwrap();
    let review = f.review(&p.proposal_id, "approve");
    let out = f.store.review_proposal(review.clone()).unwrap();
    assert_eq!(out.receipt.status, OutcomeStatus::Blocked);
    assert_eq!(
        f.store.review_proposal(review).unwrap().receipt,
        out.receipt
    );
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 0);
    assert!(f.store.owner_inbox_get(&p.proposal_id).unwrap()["blockedError"].is_object());
}

#[test]
fn review_cannot_drop_original_or_refresh_revision_and_conflict_is_atomic() {
    let f = Fixture::new();
    let made = f
        .store
        .apply(&f.writer, &f.batch(json!([create("a", "old")])))
        .unwrap();
    let id = &made.receipt.results[0].reference;
    let b=f.batch(json!([{"itemKey":"a","mutation":"update","id":id,"expectedRevision":1,"draft":create("a","proposal")["draft"]}]));
    let p = f.store.propose(&f.writer, &b).unwrap();
    let mut invalid = f.review(&p.proposal_id, "approve");
    invalid["reviewedItems"] = json!([create("added", "replacement")]);
    assert!(f.store.review_proposal(invalid).is_err());
    f.store.apply(&f.writer,&f.batch(json!([{"itemKey":"a","mutation":"update","id":id,"expectedRevision":1,"draft":create("a","later")["draft"]}]))).unwrap();
    let mut refreshed = f.review(&p.proposal_id, "approve");
    let mut items = serde_json::to_value(&b.items).unwrap();
    items[0]["expectedRevision"] = json!(2);
    refreshed["reviewedItems"] = items;
    assert!(f.store.review_proposal(refreshed).is_err());
    let blocked = f
        .store
        .review_proposal(f.review(&p.proposal_id, "approve"))
        .unwrap();
    assert_eq!(blocked.receipt.status, OutcomeStatus::Blocked);
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 2);
}

#[test]
fn operation_ids_cannot_cross_direct_proposal_review_families() {
    let f = Fixture::new();
    let direct = f.batch(json!([create("a", "direct")]));
    f.store.apply(&f.writer, &direct).unwrap();
    assert_eq!(
        f.store.propose(&f.writer, &direct).unwrap_err().code,
        GardenErrorCode::OperationReused
    );
    let pbatch = f.batch(json!([create("a", "proposal")]));
    let p = f.store.propose(&f.writer, &pbatch).unwrap();
    assert_eq!(
        f.store.apply(&f.writer, &pbatch).unwrap_err().code,
        GardenErrorCode::OperationReused
    );
    let review = f.review(&p.proposal_id, "dismiss");
    f.store.review_proposal(review.clone()).unwrap();
    let mut changed = review;
    changed["action"] = json!("approve");
    assert_eq!(
        f.store.review_proposal(changed).unwrap_err().code,
        GardenErrorCode::OperationReused
    );
}

#[test]
fn equal_operation_uuid_is_isolated_between_actors() {
    let f = Fixture::new();
    let batch = f.batch(json!([create("a", "same UUID")]));
    f.store.apply(&f.writer, &batch).unwrap();
    let second = f
        .store
        .grant(
            "second",
            &["garden:write".into()],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let result = f.store.apply(&second, &batch).unwrap();
    assert_eq!(result.receipt.actor, "second");
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 2);
}
#[test]
fn owner_pages_visit_proposals_after_first_hundred() {
    let f = Fixture::new();
    for n in 0..105 {
        let request = f.batch(json!([create("a", &format!("proposal {n}"))]));
        let proposal = f.store.propose(&f.proposer, &request).unwrap();
        if n < 100 {
            f.store
                .review_proposal(f.review(&proposal.proposal_id, "dismiss"))
                .unwrap();
        }
    }
    let first = f.store.owner_inbox_page(None).unwrap();
    assert_eq!(first["proposals"].as_array().unwrap().len(), 100);
    assert_eq!(first["omitted"], 5);
    let cursor = first["nextCursor"].as_str().unwrap();
    let next = f.store.owner_inbox_page(Some(cursor)).unwrap();
    assert_eq!(next["proposals"].as_array().unwrap().len(), 5);
    assert!(next["nextCursor"].is_null());
    let mut ids = std::collections::BTreeSet::new();
    let mut pending = 0;
    for page in [&first, &next] {
        for item in page["proposals"].as_array().unwrap() {
            assert!(ids.insert(item["id"].as_str().unwrap()));
            if item["status"] == "pending" {
                pending += 1;
            }
        }
    }
    assert_eq!(ids.len(), 105);
    assert_eq!(pending, 5);
    assert!(f.store.owner_inbox_page(Some("invalid")).is_err());
}
#[test]
fn audit_detects_broken_proposal_review_mapping() {
    use redb::{Database, TableDefinition};
    let f = Fixture::new();
    let path = f._dir.path().join("garden.redb");
    let p = f
        .store
        .propose(&f.proposer, &f.batch(json!([create("a", "pending")])))
        .unwrap();
    f.store
        .review_proposal(f.review(&p.proposal_id, "approve"))
        .unwrap();
    let writer = f.writer.clone();
    drop(f.store);
    let db = Database::open(&path).unwrap();
    let tx = db.begin_write().unwrap();
    {
        let mut proposals = tx
            .open_table(TableDefinition::<&str, &[u8]>::new("garden_inbox"))
            .unwrap();
        let mut value: Value = serde_json::from_slice(
            proposals
                .get(p.proposal_id.as_str())
                .unwrap()
                .unwrap()
                .value(),
        )
        .unwrap();
        value["outcome"]["results"][0]["ownerEdited"] = json!(true);
        proposals
            .insert(
                p.proposal_id.as_str(),
                serde_json::to_vec(&value).unwrap().as_slice(),
            )
            .unwrap();
    }
    tx.commit().unwrap();
    drop(db);
    let store = GardenStore::open(path).unwrap();
    assert_eq!(
        store.audit(&writer).unwrap_err().code,
        GardenErrorCode::StoreUnavailable
    );
}
#[test]
fn replacement_grant_with_same_subject_does_not_revive_pending_authority() {
    let f = Fixture::new();
    let request = f.batch(json!([create("a", "old authority")]));
    let proposal = f.store.propose(&f.proposer, &request).unwrap();
    let replacement = f
        .store
        .grant(
            "proposer",
            &["garden:propose".into()],
            &ClientAccess {
                mode: AccessMode::Shared,
                subject: Some("agent:a".into()),
                records: vec![],
            },
            &GardenAccess::default(),
        )
        .unwrap();
    let out = f
        .store
        .review_proposal(f.review(&proposal.proposal_id, "approve"))
        .unwrap();
    assert_eq!(out.receipt.status, OutcomeStatus::Blocked);
    assert_eq!(
        f.store
            .get_proposal(&replacement, &f.lookup(&request.operation_id))
            .unwrap()
            .receipt
            .status,
        OutcomeStatus::Blocked
    );
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 0);
}

#[test]
fn scope_and_subject_replacement_block_approval_and_current_receipt_access() {
    for changed_subject in [false, true] {
        let f = Fixture::new();
        let request = f.batch(json!([create("a", "old authority")]));
        let proposal = f.store.propose(&f.proposer, &request).unwrap();
        let access = ClientAccess {
            mode: AccessMode::Shared,
            subject: Some(
                if changed_subject {
                    "agent:b"
                } else {
                    "agent:a"
                }
                .into(),
            ),
            records: vec![],
        };
        let scopes = if changed_subject {
            vec!["garden:propose".into()]
        } else {
            vec!["garden:read".into()]
        };
        let replacement = f
            .store
            .grant("proposer", &scopes, &access, &GardenAccess::default())
            .unwrap();
        let out = f
            .store
            .review_proposal(f.review(&proposal.proposal_id, "approve"))
            .unwrap();
        assert_eq!(out.receipt.status, OutcomeStatus::Blocked);
        assert_eq!(
            f.store
                .get_proposal(&replacement, &f.lookup(&request.operation_id))
                .unwrap_err()
                .code,
            if changed_subject {
                GardenErrorCode::NotFound
            } else {
                GardenErrorCode::Forbidden
            }
        );
        assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 0);
    }
}

#[test]
fn old_generation_stored_proposal_blocks_against_new_owner_generation() {
    use redb::{Database, TableDefinition};
    let f = Fixture::new();
    let path = f._dir.path().join("garden.redb");
    let request = f.batch(json!([create("a", "old generation")]));
    let proposal = f.store.propose(&f.proposer, &request).unwrap();
    let mut review = f.review(&proposal.proposal_id, "approve");
    let next_generation = Uuid::new_v4().to_string();
    review["generation"] = json!(next_generation);
    let writer = f.writer.clone();
    drop(f.store);
    let db = Database::open(&path).unwrap();
    let tx = db.begin_write().unwrap();
    {
        let mut meta = tx
            .open_table(TableDefinition::<&str, &[u8]>::new("garden_metadata"))
            .unwrap();
        let mut state: Value =
            serde_json::from_slice(meta.get("state").unwrap().unwrap().value()).unwrap();
        // A qualified restore explicitly retains historical generations for audit;
        // changing only the current generation would be corrupt state.
        meta.insert(
            "retained_generations",
            serde_json::to_vec(&json!([state["generation"]]))
                .unwrap()
                .as_slice(),
        )
        .unwrap();
        state["generation"] = json!(next_generation);
        meta.insert("state", serde_json::to_vec(&state).unwrap().as_slice())
            .unwrap();
    }
    tx.commit().unwrap();
    drop(db);
    let store = GardenStore::open(path).unwrap();
    let out = store.review_proposal(review.clone()).unwrap();
    assert_eq!(out.receipt.status, OutcomeStatus::Blocked);
    assert_eq!(store.review_proposal(review).unwrap().receipt, out.receipt);
    assert_eq!(
        store.owner_inbox_get(&proposal.proposal_id).unwrap()["blockedError"]["code"],
        "GENERATION_MISMATCH"
    );
    assert_eq!(store.audit(&writer).unwrap().sequence, 0);
}

#[test]
fn approval_mapping_and_historical_payload_survive_later_canonical_revision() {
    let f = Fixture::new();
    let request = f.batch(json!([create("a", "approved original")]));
    let proposal = f.store.propose(&f.writer, &request).unwrap();
    let review = f.review(&proposal.proposal_id, "approve");
    let out = f.store.review_proposal(review.clone()).unwrap();
    let reference = &out.receipt.results[0].reference;
    f.store.apply(&f.writer,&f.batch(json!([{"itemKey":"a","mutation":"update","id":reference,"expectedRevision":1,"draft":create("a","later value")["draft"]}]))).unwrap();
    let current = f
        .store
        .get_proposal(&f.writer, &f.lookup(&request.operation_id))
        .unwrap();
    assert_eq!(current.receipt.results, out.receipt.results);
    assert_eq!(current.visible_records, out.visible_records);
    assert_eq!(current.visible_records[0].metadata.revision, 1);
    assert_eq!(
        f.store.review_proposal(review).unwrap().receipt,
        out.receipt
    );
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, 2);
}
#[test]
fn current_hidden_evidence_filters_original_approved_assertion_payload() {
    let f = Fixture::new();
    let observed = json!({"at":null,"sourceTimezone":null,"precision":"unknown"});
    let evidence = json!({"itemKey":"evidence","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence","source":{"type":"uri","uri":"https://example.test/original"},"excerpt":"original evidence","observedTime":observed,"extensions":{}}});
    let assertion = json!({"itemKey":"assertion","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":"subject"},"predicate":"test:claim","object":{"type":"entityRef","entityRef":{"refType":"item","kind":"entity","itemKey":"subject"}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":observed,"evidence":[{"refType":"item","kind":"evidence","itemKey":"evidence"}],"confidence":null,"extensions":{}}});
    let request = f.batch(json!([
        create("subject", "public subject"),
        evidence,
        assertion
    ]));
    let proposal = f.store.propose(&f.writer, &request).unwrap();
    let approved = f
        .store
        .review_proposal(f.review(&proposal.proposal_id, "approve"))
        .unwrap();
    let assertion_mapping = approved
        .receipt
        .results
        .iter()
        .find(|r| r.item_key == "assertion")
        .unwrap();
    let mut hidden_evidence = evidence;
    hidden_evidence["itemKey"] = json!("hidden");
    hidden_evidence["draft"]["excerpt"] = json!("private current evidence");
    let hidden = f
        .store
        .apply(&f.writer, &f.batch(json!([hidden_evidence])))
        .unwrap();
    let mut current_draft = approved
        .visible_records
        .iter()
        .find(|r| r.draft.kind() == GardenKind::Assertion)
        .unwrap()
        .draft
        .clone();
    if let GardenDraft::Assertion { evidence, .. } = &mut current_draft {
        evidence.clear();
        evidence.push(RecordRef::Existing {
            kind: GardenKind::Evidence,
            id: hidden.receipt.results[0].reference.id.clone(),
        });
    }
    let update=f.batch(json!([{"itemKey":"assertion","mutation":"update","id":assertion_mapping.reference,"expectedRevision":1,"draft":current_draft}]));
    f.store.apply(&f.writer, &update).unwrap();
    let visible = approved
        .receipt
        .results
        .iter()
        .map(|r| r.reference.clone())
        .collect();
    let replacement = f
        .store
        .grant(
            "writer",
            &["garden:propose".into()],
            &ClientAccess::default(),
            &GardenAccess::Allowlist { objects: visible },
        )
        .unwrap();
    let current = f
        .store
        .get_proposal(&replacement, &f.lookup(&request.operation_id))
        .unwrap();
    assert_eq!(current.receipt.results, approved.receipt.results);
    assert_eq!(current.visible_records.len(), 2);
    assert!(
        current
            .visible_records
            .iter()
            .all(|record| record.draft.kind() != GardenKind::Assertion)
    );
    assert!(current.payload_omitted);
    assert!(
        !serde_json::to_string(&current)
            .unwrap()
            .contains("private current evidence")
    );
}
