use super::*;

#[test]
fn pagination_hints_are_visible_before_rows_on_the_first_viewport() {
    let mut inbox = fixture();
    inbox.page = Page::List;
    inbox.items=(0..100).map(|i|json!({"id":format!("id-{i:03}"),"status":"dismissed","proposer":"agent","itemCount":1})).collect();
    inbox.next_cursor = Some(uuid::Uuid::new_v4().to_string());
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(100, 18)).unwrap();
    terminal
        .draw(|frame| inbox.render(frame, frame.area()))
        .unwrap();
    let visible = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    for hint in ["n next page", "p previous page", "r first page"] {
        assert!(
            visible.contains(hint),
            "{hint} must be visible before navigating: {visible}"
        );
    }
}

#[test]
fn selected_last_proposal_is_visible_with_long_summary_metadata() {
    let mut inbox = fixture();
    inbox.page = Page::List;
    inbox.items = (0..100)
        .map(|i| {
            json!({"id":format!("id-{i:03}"),"status":"dismissed",
        "generation":"x".repeat(200),"proposer":"agent".repeat(80),"itemCount":1})
        })
        .collect();
    inbox.selected = 99;
    let backend = ratatui::backend::TestBackend::new(100, 18);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| inbox.render(frame, frame.area()))
        .unwrap();
    let visible = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    assert!(
        visible.contains("> id-099"),
        "selected last row must remain visible: {visible}"
    );
}

#[test]
fn owner_can_traverse_a_full_terminal_page_and_return_without_rebasing() {
    let mut inbox = fixture();
    inbox.page = Page::List;
    inbox.items = (0..100)
        .map(|i| json!({"id":format!("closed-{i}"),"status":"dismissed"}))
        .collect();
    let cursor = uuid::Uuid::new_v4().to_string();
    inbox.next_cursor = Some(cursor.clone());
    assert!(inbox.next_page());
    assert_eq!(inbox.page_cursor.as_deref(), Some(cursor.as_str()));
    inbox.items = vec![json!({"id":"later-pending","status":"pending"})];
    assert!(inbox.body().contains("later-pending"));
    assert!(inbox.previous_page());
    assert!(inbox.page_cursor.is_none());
    inbox.next_cursor = Some(cursor);
    assert!(inbox.next_page());
    inbox.reset_page();
    assert!(inbox.page_cursor.is_none());
    assert!(inbox.cursor_history.is_empty());
}

#[test]
fn unchanged_approval_omits_redundant_review_payload() {
    let mut inbox = fixture();
    inbox.prepare_request("approve").unwrap();
    assert!(
        inbox
            .pending
            .as_ref()
            .unwrap()
            .get("reviewedItems")
            .is_none()
    );
}

#[test]
fn oversized_edited_owner_request_is_rejected_before_it_becomes_pending() {
    let mut inbox = fixture();
    inbox.candidate[0]["draft"]["displayName"] = json!("x".repeat(MAX_JSON_BYTES));
    assert!(inbox.prepare_request("approve").is_err());
    assert!(inbox.pending.is_none());
}

#[test]
fn list_exposes_next_previous_and_reset_navigation_after_full_terminal_page() {
    let mut inbox = fixture();
    inbox.page = Page::List;
    inbox.items = (0..100)
        .map(|i| json!({"id":format!("closed-{i}"),"status":"dismissed"}))
        .collect();
    let body = inbox.body();
    assert!(body.contains("n next page"));
    assert!(body.contains("p previous page"));
    assert!(body.contains("r first page"));
}

#[test]
fn resolved_proposals_do_not_offer_an_impossible_dismissal() {
    for status in ["approved", "dismissed", "blocked"] {
        let mut inbox = fixture();
        let mut proposal = inbox.proposal.clone();
        proposal["status"] = json!(status);
        inbox.open_proposal(proposal);
        inbox
            .input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE))
            .unwrap();
        assert_eq!(inbox.page, Page::Review, "already {status}");
        assert!(!inbox.body().contains("d dismiss"));
    }
}

fn fixture() -> Inbox {
    let mut inbox = Inbox::empty(PathBuf::from("synthetic-unused.redb"));
    inbox.workspace = json!({"id":uuid::Uuid::new_v4().to_string(),"generation":uuid::Uuid::new_v4().to_string()});
    inbox.open_proposal(json!({"id":uuid::Uuid::new_v4().to_string(),"revision":1,
        "status":"pending","generation":inbox.workspace["generation"],
        "proposer":{"clientId":"synthetic-agent","subject":"synthetic-subject"},
        "request":{"version":2,"workspaceId":inbox.workspace["id"],"generation":inbox.workspace["generation"],
        "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
          {"itemKey":"original","expectedRevision":0,"mutation":"create","draft":
          {"kind":"entity","entityType":"test:person","displayName":"Original","aliases":[],"extensions":{}}}]}}));
    inbox
}

#[test]
fn uncertain_review_keeps_exact_operation_and_blocks_navigation_edits_and_dismissal() {
    let mut inbox = fixture();
    inbox.page = Page::ConfirmApprove;
    inbox.prepare_request("approve").unwrap();
    let original = inbox.pending.clone().unwrap();
    inbox.finish(Err(GardenError {
        code: GardenErrorCode::StoreUnavailable,
        message: "synthetic response lost".into(),
        restart_required: false,
    }));
    for key in [
        KeyCode::Esc,
        KeyCode::Char('d'),
        KeyCode::Char('e'),
        KeyCode::Char('r'),
    ] {
        assert!(!inbox.input(KeyEvent::new(key, KeyModifiers::NONE)).unwrap());
        assert_eq!(inbox.pending.as_ref(), Some(&original));
    }
    inbox.prepare_request("dismiss").unwrap();
    assert_eq!(inbox.pending.as_ref(), Some(&original));
    assert_eq!(original["action"], "approve");
}

#[test]
fn owner_edits_and_additions_preserve_original_and_appear_in_confirmation() {
    let mut inbox = fixture();
    let original = inbox.proposal["request"].clone();
    inbox.candidate[0]["draft"]["displayName"] = json!("Edited");
    let mut added = inbox.candidate[0].clone();
    added["itemKey"] = json!("owner-added");
    inbox.candidate.as_array_mut().unwrap().push(added);
    inbox.validate_candidate().unwrap();
    inbox.page = Page::ConfirmApprove;
    let body = inbox.body();
    assert!(body.contains("owner-edited"));
    assert!(body.contains("owner-added"));
    assert!(body.contains("synthetic-agent"));
    assert!(body.contains("expectedRevision"));
    assert!(body.contains("Original"));
    assert!(body.contains("Edited"));
    inbox.prepare_request("approve").unwrap();
    assert_eq!(inbox.proposal["request"], original);
    assert_eq!(
        inbox.pending.as_ref().unwrap()["reviewedItems"],
        inbox.candidate
    );
}

#[test]
fn invalid_edits_and_conflicts_never_rebase_the_original_proposal() {
    let mut inbox = fixture();
    inbox.candidate = json!([]);
    assert!(inbox.validate_candidate().is_err());
    inbox = fixture();
    inbox.candidate[0]["expectedRevision"] = json!(9);
    assert!(inbox.validate_candidate().is_err());
    inbox = fixture();
    let original = inbox.proposal.clone();
    inbox.page = Page::ConfirmApprove;
    inbox.prepare_request("approve").unwrap();
    inbox.finish(Err(GardenError {
        code: GardenErrorCode::Conflict,
        message: "changed".into(),
        restart_required: false,
    }));
    assert!(inbox.blocked);
    inbox
        .input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE))
        .unwrap();
    assert_eq!(inbox.page, Page::Review);
    assert!(inbox.pending.is_none());
    assert_eq!(inbox.proposal, original);
}

#[test]
fn review_explains_dismissal_and_shared_retraction_without_local_erasure() {
    let mut inbox = fixture();
    inbox.page = Page::ConfirmDismiss;
    let body = inbox.body();
    assert!(body.contains("Dismiss"));
    assert!(body.contains("retract"));
    assert!(body.contains("history"));
    inbox.prepare_request("dismiss").unwrap();
    assert!(
        inbox
            .pending
            .as_ref()
            .unwrap()
            .get("reviewedItems")
            .is_none()
    );
}

#[test]
fn changed_generation_blocks_approval_without_rewriting_the_original_envelope() {
    let mut inbox = fixture();
    let proposal = inbox.proposal.clone();
    inbox.workspace["generation"] = json!(uuid::Uuid::new_v4().to_string());
    inbox.open_proposal(proposal.clone());
    assert!(inbox.blocked);
    inbox
        .input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE))
        .unwrap();
    assert_eq!(inbox.page, Page::Review);
    assert!(inbox.pending.is_none());
    assert_eq!(inbox.proposal, proposal);
    inbox
        .input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE))
        .unwrap();
    assert_eq!(inbox.page, Page::ConfirmDismiss);
}

#[test]
fn lost_approval_response_reconciles_the_saved_operation_without_second_commit() {
    use mother::{access::ClientAccess, garden::access::GardenAccess};
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("synthetic.redb");
    let workspace = uuid::Uuid::new_v4().to_string();
    let generation = uuid::Uuid::new_v4().to_string();
    let mut inbox = fixture();
    inbox.path = path.clone();
    let auth;
    let request;
    let receipt;
    {
        let store = GardenStore::initialize(&path, &workspace, &generation).unwrap();
        auth = store
            .grant(
                "synthetic",
                &["garden:read".into(), "garden:propose".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        let mut batch = inbox.proposal["request"].clone();
        batch["workspaceId"] = json!(workspace);
        batch["generation"] = json!(generation);
        let batch: BatchRequest = serde_json::from_value(batch).unwrap();
        let proposal = store.propose(&auth, &batch).unwrap();
        inbox.workspace = store.owner_inbox_list().unwrap()["workspace"].clone();
        inbox.open_proposal(store.owner_inbox_get(&proposal.proposal_id).unwrap());
        inbox.page = Page::ConfirmApprove;
        inbox.prepare_request("approve").unwrap();
        request = inbox.pending.clone().unwrap();
        receipt = store.review_proposal(request.clone()).unwrap().receipt;
    }
    inbox.finish(Err(GardenError {
        code: GardenErrorCode::StoreUnavailable,
        message: "synthetic lost response".into(),
        restart_required: false,
    }));
    assert_eq!(inbox.pending.as_ref(), Some(&request));
    inbox
        .input(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
        .unwrap();
    assert!(inbox.pending.is_none());
    assert_eq!(inbox.page, Page::List);
    let store = GardenStore::open(&path).unwrap();
    assert_eq!(store.review_proposal(request).unwrap().receipt, receipt);
    assert_eq!(store.audit(&auth).unwrap().sequence, 1);
}

#[test]
fn pending_attachment_asks_for_the_original_file_instead_of_approving() {
    let mut inbox = fixture();
    let target = uuid::Uuid::new_v4().to_string();
    inbox.open_proposal(json!({"id":uuid::Uuid::new_v4().to_string(),"revision":1,
        "status":"pending","generation":inbox.workspace["generation"],"proposer":"synthetic-agent",
        "request":{"version":2,"workspaceId":inbox.workspace["id"],"generation":inbox.workspace["generation"],
        "operationId":uuid::Uuid::new_v4().to_string(),"requiredExtensions":{},"items":[
          {"itemKey":"photo","expectedRevision":0,"mutation":"create","draft":{"kind":"evidence",
          "source":{"type":"uri","uri":PENDING_URI},"excerpt":null,
          "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
          "extensions":{"mother.media:pending":{"schemaVersion":1,"payload":{
            "target":{"kind":"assertion","id":target},"expectedRevision":1,
            "description":"A synthetic harbour"}}}}}]}}));
    assert!(inbox.body().contains("f attach the original file"));
    let key = |code| KeyEvent::new(code, KeyModifiers::NONE);
    inbox.input(key(KeyCode::Char('a'))).unwrap();
    assert_eq!(inbox.page, Page::Review);
    assert!(inbox.message.contains("awaits the original image"));
    inbox.input(key(KeyCode::Char('f'))).unwrap();
    assert_eq!(inbox.page, Page::AttachPath);
    inbox.paste("C:/synthetic/harbour.png");
    inbox.input(key(KeyCode::Enter)).unwrap();
    assert_eq!(inbox.page, Page::ConfirmAttach);
    let body = inbox.body();
    assert!(body.contains(&target) && body.contains("A synthetic harbour"));
    assert!(body.contains("C:/synthetic/harbour.png"));
    assert!(body.contains("location/device metadata: yes"));
    inbox.input(key(KeyCode::Char('x'))).unwrap();
    assert!(inbox.body().contains("attributed interpretation: yes"));
    inbox.input(key(KeyCode::Esc)).unwrap();
    assert_eq!(inbox.page, Page::Review);
    assert!(inbox.pending.is_none(), "no review request was prepared");
}
