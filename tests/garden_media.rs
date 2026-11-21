//! Visual memory: provenance, attachment, scoped retrieval, portability and review.
//! Every image here is synthetic and generated inside an owned temporary directory.
use mother::{
    access::{AccessMode, ClientAccess},
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::*,
        media_tools::{
            AcquisitionInput, AttachRequest, AttachSource, AttachmentReview, IngestOptions,
            MediaOrigin, PENDING_EXTENSION, PENDING_URI,
        },
        store::GardenStore,
    },
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn exif(tags: &[u16]) -> Vec<u8> {
    let mut t = b"II*\0\x08\0\0\0".to_vec();
    t.extend_from_slice(&(tags.len() as u16).to_le_bytes());
    for tag in tags {
        t.extend_from_slice(&tag.to_le_bytes());
        t.extend_from_slice(&[2, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
    }
    t.extend_from_slice(&[0; 4]);
    t
}
/// Minimal structurally valid JPEG carrying GPS and device EXIF tags.
fn jpeg_with_gps(seed: u8) -> Vec<u8> {
    let mut app1 = b"Exif\0\0".to_vec();
    app1.extend(exif(&[0x010F, 0x8825]));
    let mut b = vec![0xFF, 0xD8, 0xFF, 0xE1];
    b.extend_from_slice(&((app1.len() + 2) as u16).to_be_bytes());
    b.extend(app1);
    b.extend_from_slice(&[0xFF, 0xC0, 0, 11, 8, 0, 3, 0, 4, 1, 1, 0x11, 0]);
    b.extend_from_slice(&[0xFF, 0xDA, 0, 8, 1, 1, 0, 0, 0x3F, 0]);
    b.extend_from_slice(&[0x12, seed, 0x34, 0xFF, 0xD9]);
    b
}
/// Minimal PNG without metadata, padded with an IDAT of the requested size.
fn png(payload: usize) -> Vec<u8> {
    let mut b = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut chunk = |kind: &[u8], data: &[u8]| {
        b.extend_from_slice(&(data.len() as u32).to_be_bytes());
        b.extend_from_slice(kind);
        b.extend_from_slice(data);
        b.extend_from_slice(&[0; 4]);
    };
    chunk(b"IHDR", &[0, 0, 0, 2, 0, 0, 0, 2, 8, 2, 0, 0, 0]);
    chunk(b"IDAT", &vec![7; payload]);
    chunk(b"IEND", &[]);
    b
}
fn sha(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

struct Fixture {
    dir: tempfile::TempDir,
    store: GardenStore,
    ws: String,
    generation: String,
    writer: Auth,
}
impl Fixture {
    fn new() -> Self {
        let dir =
            tempfile::tempdir_in(std::fs::canonicalize(std::env::temp_dir()).unwrap()).unwrap();
        let ws = Uuid::new_v4().to_string();
        let generation = Uuid::new_v4().to_string();
        let store =
            GardenStore::initialize(dir.path().join("garden.redb"), &ws, &generation).unwrap();
        let writer = store
            .grant(
                "writer",
                &["garden:read".into(), "garden:write".into()],
                &ClientAccess::default(),
                &GardenAccess::All {},
            )
            .unwrap();
        Self {
            dir,
            store,
            ws,
            generation,
            writer,
        }
    }
    fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.dir.path().join(name);
        std::fs::write(&path, bytes).unwrap();
        path
    }
    fn grant(&self, client: &str, scopes: &[&str], garden: GardenAccess) -> Auth {
        self.store
            .grant(
                client,
                &scopes.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
                &ClientAccess {
                    mode: AccessMode::Companion,
                    subject: Some(format!("agent:{client}")),
                    records: vec![],
                },
                &garden,
            )
            .unwrap()
    }
    fn batch(&self, items: Value) -> BatchRequest {
        serde_json::from_value(
            json!({"version":2,"workspaceId":self.ws,"generation":self.generation,
            "operationId":Uuid::new_v4().to_string(),"requiredExtensions":{},"items":items}),
        )
        .unwrap()
    }
    /// One entity and one assertion about it; returns (entity, assertion) IDs.
    fn memory(&self) -> (String, String) {
        let out = self
            .store
            .apply(
                &self.writer,
                &self.batch(json!([
                    {"itemKey":"p","mutation":"create","expectedRevision":0,"draft":{"kind":"entity","entityType":"test:person","displayName":"Synthetic Person","aliases":[],"extensions":{}}},
                    {"itemKey":"m","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion","subject":{"refType":"item","kind":"entity","itemKey":"p"},"predicate":"test:visited","object":{"type":"value","value":{"type":"string","value":"a synthetic beach"}},"validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},"observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"evidence":[],"confidence":null,"extensions":{}}}
                ])),
            )
            .unwrap();
        (
            out.receipt.results[0].reference.id.clone(),
            out.receipt.results[1].reference.id.clone(),
        )
    }
    fn assertion(&self, id: &str) -> GardenRecord {
        let request: GetRequest = serde_json::from_value(json!({"version":2,"workspaceId":self.ws,
            "generation":self.generation,"requiredExtensions":{},
            "reference":{"kind":"assertion","id":id},"revision":null,"knownAt":null}))
        .unwrap();
        self.store.get(&self.writer, &request).unwrap().record
    }
    fn args(&self, extra: Value) -> Value {
        let mut value = json!({"version":2,"workspaceId":self.ws,"generation":self.generation});
        value
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        value
    }
    fn attach(&self, path: &Path, target: &str, revision: u64, strip: bool) -> Value {
        self.store
            .media_attach(&AttachRequest {
                source: AttachSource::File {
                    path,
                    input: AcquisitionInput::new(MediaOrigin::Captured),
                    options: IngestOptions {
                        strip_metadata: strip,
                        ..IngestOptions::default()
                    },
                },
                target: target.into(),
                expected_revision: revision,
                note: Some("synthetic note".into()),
            })
            .unwrap()
    }
}

#[test]
fn ingest_deduplicates_bytes_and_appends_provenance() {
    let f = Fixture::new();
    let bytes = png(16);
    let path = f.file("first.png", &bytes);
    let mut input = AcquisitionInput::new(MediaOrigin::Screenshot);
    input.original_filename = Some("first.png".into());
    let first = f
        .store
        .media_add(&path, &input, IngestOptions::default())
        .unwrap();
    assert_eq!(first.sha256, sha(&bytes));
    assert_eq!(first.media_type, "image/png");
    assert_eq!((first.width, first.height), (Some(2), Some(2)));
    assert!(!first.reused && first.embedded_metadata.is_empty());
    input.original_filename = Some("copy.png".into());
    let second = f
        .store
        .media_add(&path, &input, IngestOptions::default())
        .unwrap();
    assert!(second.reused);
    assert_ne!(first.acquisition_id, second.acquisition_id);
    let listed = f.store.media_owner_list(None).unwrap();
    let assets = listed["assets"].as_array().unwrap();
    assert_eq!(assets.len(), 1);
    let names: Vec<_> = assets[0]["acquisitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["originalFilename"].as_str().unwrap())
        .collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"first.png") && names.contains(&"copy.png"));

    // Mismatched type, unsupported format and oversized input change nothing.
    let mut declared = input.clone();
    declared.declared_media_type = Some("image/jpeg".into());
    assert_eq!(
        f.store
            .media_add(&path, &declared, IngestOptions::default())
            .unwrap_err()
            .code,
        GardenErrorCode::InvalidInput
    );
    let text = f.file("notes.txt", b"not an image");
    assert!(
        f.store
            .media_add(&text, &input, IngestOptions::default())
            .is_err()
    );
    let big = f.file("big.png", &png(4096));
    let capped = IngestOptions {
        max_bytes: 1024,
        strip_metadata: false,
    };
    assert_eq!(
        f.store.media_add(&big, &input, capped).unwrap_err().code,
        GardenErrorCode::LimitExceeded
    );
    let after = f.store.media_owner_list(None).unwrap();
    assert_eq!(after["assets"].as_array().unwrap().len(), 1);
    assert_eq!(
        after["assets"][0]["acquisitions"].as_array().unwrap().len(),
        2
    );
    assert_eq!(f.store.media_verify().unwrap()["ok"], true);
}

#[test]
fn stripping_keeps_the_original_and_adds_a_clean_derivative() {
    let f = Fixture::new();
    let original = jpeg_with_gps(1);
    let path = f.file("photo.jpg", &original);
    let report = f
        .store
        .media_add(
            &path,
            &AcquisitionInput::new(MediaOrigin::Captured),
            IngestOptions {
                strip_metadata: true,
                ..IngestOptions::default()
            },
        )
        .unwrap();
    assert_eq!(report.embedded_metadata, ["device", "exif", "gps"]);
    let derivative = report.derivative.unwrap();
    assert_ne!(derivative.sha256, report.sha256);
    let out = f.dir.path().join("exported.jpg");
    f.store.media_export(&report.sha256, &out).unwrap();
    assert_eq!(std::fs::read(&out).unwrap(), original);
    assert!(f.store.media_export(&report.sha256, &out).is_err());
    let clean = f.dir.path().join("clean.jpg");
    f.store.media_export(&derivative.sha256, &clean).unwrap();
    let clean = std::fs::read(&clean).unwrap();
    assert!(mother::media::inspect(&clean).unwrap().embedded.is_empty());
    let listed = f.store.media_owner_list(None).unwrap();
    let source = listed["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["asset"]["sha256"] == report.sha256.as_str())
        .unwrap();
    assert_eq!(source["derivatives"], json!([derivative.sha256]));
    assert_eq!(f.store.media_verify().unwrap()["ok"], true);
}

#[test]
fn attach_revises_the_assertion_atomically_and_rejects_stale_revisions() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let path = f.file("beach.png", &png(8));
    let result = f.attach(&path, &memory, 1, false);
    assert_eq!(result["outcome"]["receipt"]["status"], "applied");
    let record = f.assertion(&memory);
    assert_eq!(record.metadata.revision, 2);
    let GardenDraft::Assertion { evidence, .. } = &record.draft else {
        panic!("assertion expected")
    };
    assert_eq!(evidence.len(), 1);
    let sequence = f.store.audit(&f.writer).unwrap().sequence;

    let other = f.file("other.png", &png(9));
    let stale = f.store.media_attach(&AttachRequest {
        source: AttachSource::File {
            path: &other,
            input: AcquisitionInput::new(MediaOrigin::Captured),
            options: IngestOptions::default(),
        },
        target: memory.clone(),
        expected_revision: 1,
        note: None,
    });
    assert_eq!(stale.unwrap_err().code, GardenErrorCode::Conflict);
    assert_eq!(f.store.audit(&f.writer).unwrap().sequence, sequence);
    assert_eq!(
        f.store.media_owner_list(None).unwrap()["assets"]
            .as_array()
            .unwrap()
            .len(),
        1,
        "a failed attach stores no bytes or provenance"
    );

    // One asset, many memories: the second link reuses the stored bytes.
    let (_, second) = f.memory();
    let sha256 = result["ingest"]["sha256"].as_str().unwrap().to_owned();
    f.store
        .media_attach(&AttachRequest {
            source: AttachSource::Existing {
                sha256: sha256.clone(),
            },
            target: second.clone(),
            expected_revision: 1,
            note: None,
        })
        .unwrap();
    let listed = f.store.media_owner_list(None).unwrap();
    assert_eq!(listed["assets"][0]["uses"].as_array().unwrap().len(), 2);
    let per_record = f.store.media_owner_list(Some(&second)).unwrap();
    assert_eq!(per_record["assets"][0]["asset"]["sha256"], sha256.as_str());

    // A locator that disagrees with the stored asset is refused by the graph writer.
    let bad = f.batch(json!([{"itemKey":"e","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence",
        "source":{"type":"artifact","sha256":sha256,"sizeBytes":1,"mediaType":"image/png"},"excerpt":null,
        "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}}]));
    assert_eq!(
        f.store.apply(&f.writer, &bad).unwrap_err().code,
        GardenErrorCode::InvalidInput
    );
}

#[test]
fn metadata_and_bytes_are_separately_scoped_and_visibility_bound() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let gps = f.file("gps.jpg", &jpeg_with_gps(2));
    let attached = f.attach(&gps, &memory, 1, false);
    let sha256 = attached["ingest"]["sha256"].as_str().unwrap().to_owned();

    let plain = f.grant("plain", &["garden:read"], GardenAccess::All {});
    assert_eq!(f.store.media_tools(&plain).unwrap(), Vec::<&str>::new());
    assert_eq!(
        f.store
            .media_list(&plain, &f.args(json!({})))
            .unwrap_err()
            .code,
        GardenErrorCode::Forbidden
    );
    let meta = f.grant("meta", &["garden:read", "media:meta"], GardenAccess::All {});
    assert_eq!(
        f.store.media_tools(&meta).unwrap(),
        ["mother_garden_media_list"]
    );
    let listed = f.store.media_list(&meta, &f.args(json!({}))).unwrap();
    assert_eq!(listed["assets"][0]["sha256"], sha256.as_str());
    assert_eq!(
        listed["assets"][0]["embeddedMetadata"],
        json!(["device", "exif", "gps"])
    );
    assert_eq!(
        f.store
            .media_get(&meta, &f.args(json!({"sha256":sha256})))
            .unwrap_err()
            .code,
        GardenErrorCode::Forbidden
    );

    // An allowlisted client that cannot see the citing evidence learns nothing.
    let narrow = f.grant(
        "narrow",
        &["garden:read", "media:meta", "media:bytes"],
        GardenAccess::Allowlist {
            objects: vec![ExternalReference {
                kind: GardenKind::Assertion,
                id: memory.clone(),
            }],
        },
    );
    assert_eq!(
        f.store.media_list(&narrow, &f.args(json!({}))).unwrap()["assets"],
        json!([])
    );
    assert_eq!(
        f.store
            .media_get(&narrow, &f.args(json!({"sha256":sha256})))
            .unwrap_err()
            .code,
        GardenErrorCode::NotFound
    );

    // Bytes: metadata-bearing originals need a derivative or an explicit originals grant.
    let bytes = f.grant(
        "bytes",
        &["garden:read", "media:meta", "media:bytes"],
        GardenAccess::All {},
    );
    assert_eq!(
        f.store
            .media_get(&bytes, &f.args(json!({"sha256":sha256})))
            .unwrap_err()
            .code,
        GardenErrorCode::Forbidden
    );
    assert_eq!(
        f.store
            .media_get(
                &bytes,
                &f.args(json!({"sha256":sha256,"variant":"original"}))
            )
            .unwrap_err()
            .code,
        GardenErrorCode::Forbidden
    );
    let originals = f.grant(
        "originals",
        &[
            "garden:read",
            "media:meta",
            "media:bytes",
            "media:originals",
        ],
        GardenAccess::All {},
    );
    let got = f
        .store
        .media_get(
            &originals,
            &f.args(json!({"sha256":sha256,"variant":"original"})),
        )
        .unwrap();
    assert_eq!(got["variant"], "original");
    assert_eq!(got["data"], mother::media::base64(&jpeg_with_gps(2)));

    // A stripped derivative is served by default and matches its digest.
    let (_, second) = f.memory();
    let clean = f.attach(&f.file("strip.jpg", &jpeg_with_gps(3)), &second, 1, true);
    let original = clean["ingest"]["sha256"].as_str().unwrap();
    let derivative = clean["ingest"]["derivative"]["sha256"].as_str().unwrap();
    let got = f
        .store
        .media_get(&bytes, &f.args(json!({"sha256":original})))
        .unwrap();
    assert_eq!(got["variant"], "derivative");
    assert_eq!(got["sha256"], derivative);
    assert_eq!(got["originalSha256"], original);

    // Oversized images are refused rather than truncated.
    let (_, third) = f.memory();
    let large = f.attach(&f.file("large.png", &png(2_200_000)), &third, 1, false);
    let large = large["ingest"]["sha256"].as_str().unwrap();
    assert_eq!(
        f.store
            .media_get(&bytes, &f.args(json!({"sha256":large})))
            .unwrap_err()
            .code,
        GardenErrorCode::LimitExceeded
    );

    // Scope chains and shared-client limits are enforced at grant time.
    let refused = |scopes: &[&str], mode: AccessMode| {
        f.store
            .grant(
                "refused",
                &scopes.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
                &ClientAccess {
                    mode,
                    subject: Some("agent:refused".into()),
                    records: vec![],
                },
                &GardenAccess::default(),
            )
            .unwrap_err()
            .code
    };
    assert_eq!(
        refused(&["media:meta"], AccessMode::Companion),
        GardenErrorCode::InvalidInput
    );
    assert_eq!(
        refused(&["garden:read", "media:bytes"], AccessMode::Companion),
        GardenErrorCode::InvalidInput
    );
    assert_eq!(
        refused(
            &["garden:read", "media:meta", "media:bytes"],
            AccessMode::Shared
        ),
        GardenErrorCode::InvalidInput
    );
}

#[test]
fn backup_restore_preserves_media_and_provenance() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let attached = f.attach(&f.file("kept.jpg", &jpeg_with_gps(4)), &memory, 1, true);
    let bundle = f.dir.path().join("bundle");
    f.store.backup_to(&bundle).unwrap();
    GardenStore::verify_backup(&bundle).unwrap();
    let restored =
        GardenStore::restore_backup(&bundle, &f.dir.path().join("restored.redb")).unwrap();
    assert_eq!(restored.media_verify().unwrap()["ok"], true);
    let before = f.store.media_owner_list(None).unwrap();
    let after = restored.media_owner_list(None).unwrap();
    assert_eq!(before, after);
    let sha256 = attached["ingest"]["sha256"].as_str().unwrap();
    let out = f.dir.path().join("restored.jpg");
    restored.media_export(sha256, &out).unwrap();
    assert_eq!(std::fs::read(out).unwrap(), jpeg_with_gps(4));
}

fn pending(f: &Fixture, target: &str, description: Option<&str>) -> BatchRequest {
    f.batch(json!([{"itemKey":"photo","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence",
        "source":{"type":"uri","uri":PENDING_URI},"excerpt":null,
        "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
        "extensions":{PENDING_EXTENSION:{"schemaVersion":1,"payload":{
            "target":{"kind":"assertion","id":target},"expectedRevision":1,"description":description}}}}}]))
}

#[test]
fn conversation_attachments_wait_for_the_owner_original() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let harness = f.grant(
        "harness",
        &["garden:read", "garden:propose"],
        GardenAccess::All {},
    );
    let request = pending(
        &f,
        &memory,
        Some("Two people on a synthetic beach at sunset"),
    );

    // A pending marker can never become canonical directly.
    assert_eq!(
        f.store.apply(&f.writer, &request).unwrap_err().code,
        GardenErrorCode::InvalidInput
    );
    let proposal = f.store.propose(&harness, &request).unwrap();
    let review = |action: &str| {
        json!({"version":2,"workspaceId":f.ws,"generation":f.generation,
            "operationId":Uuid::new_v4().to_string(),"proposalId":proposal.proposal_id,
            "expectedRevision":1,"action":action})
    };
    assert_eq!(
        f.store.review_proposal(review("approve")).unwrap_err().code,
        GardenErrorCode::InvalidInput
    );
    assert_eq!(
        f.store.owner_inbox_get(&proposal.proposal_id).unwrap()["status"],
        "pending",
        "ordinary approval does not block the proposal"
    );

    let original = png(12);
    let path = f.file("from-conversation.png", &original);
    let (outcome, ingest) = f
        .store
        .review_attachment(
            review("approve"),
            &AttachmentReview {
                path: &path,
                input: AcquisitionInput::new(MediaOrigin::Uploaded),
                options: IngestOptions::default(),
                accept_description: true,
            },
        )
        .unwrap();
    assert_eq!(outcome.receipt.status, OutcomeStatus::Approved);
    assert_eq!(ingest.sha256, sha(&original));
    let record = f.assertion(&memory);
    assert_eq!(record.metadata.revision, 2);
    assert_eq!(
        record.metadata.approver.as_deref(),
        Some(mother::garden::inbox::OWNER_ACTOR)
    );
    let description = outcome
        .visible_records
        .iter()
        .find(|r| {
            matches!(&r.draft, GardenDraft::Assertion { predicate, .. } if predicate == "mother.media:description")
        })
        .expect("accepted description is an attributed interpretation");
    let GardenDraft::Assertion { extensions, .. } = &description.draft else {
        unreachable!()
    };
    assert_eq!(
        extensions["mother.media:interpretation"].payload["producer"],
        json!({"kind":"model","name":"harness"})
    );
    assert_eq!(f.store.media_verify().unwrap()["ok"], true);

    // Backups of an approved attachment proposal remain auditable.
    f.store
        .backup_to(&f.dir.path().join("after-review"))
        .unwrap();

    // Dismissing a different pending proposal stores no bytes.
    let (_, other) = f.memory();
    let second = f
        .store
        .propose(&harness, &pending(&f, &other, None))
        .unwrap();
    let dismissed = f
        .store
        .review_proposal(
            json!({"version":2,"workspaceId":f.ws,"generation":f.generation,
            "operationId":Uuid::new_v4().to_string(),"proposalId":second.proposal_id,
            "expectedRevision":1,"action":"dismiss"}),
        )
        .unwrap();
    assert_eq!(dismissed.receipt.status, OutcomeStatus::Dismissed);
    assert_eq!(
        f.store.media_owner_list(None).unwrap()["assets"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn pending_proposals_need_a_visible_target_and_a_single_attachment() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let blind = f.grant("blind", &["garden:propose"], GardenAccess::default());
    assert_eq!(
        f.store
            .propose(&blind, &pending(&f, &memory, None))
            .unwrap_err()
            .code,
        GardenErrorCode::NotFound
    );
    let harness = f.grant("harness", &["garden:propose"], GardenAccess::All {});
    let mut twice = pending(&f, &memory, None);
    let mut copy = twice.items[0].clone();
    copy.item_key = "again".into();
    twice.items.push(copy);
    assert_eq!(
        f.store.propose(&harness, &twice).unwrap_err().code,
        GardenErrorCode::InvalidInput
    );
    let mut unmarked = pending(&f, &memory, None);
    if let Some(GardenDraft::Evidence { source, .. }) = &mut unmarked.items[0].draft {
        *source = SourceLocator::Uri {
            uri: "https://example.test/photo".into(),
        };
    }
    assert_eq!(
        f.store.propose(&harness, &unmarked).unwrap_err().code,
        GardenErrorCode::InvalidInput
    );
}

#[test]
fn interpretations_are_attributed_claims_about_image_evidence() {
    let f = Fixture::new();
    let (person, memory) = f.memory();
    let attached = f.attach(&f.file("caption.png", &png(5)), &memory, 1, false);
    let evidence = attached["outcome"]["receipt"]["results"][0]["reference"]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let caption = |extensions: Value, evidence_id: &str| {
        f.batch(json!([{"itemKey":"c","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion",
            "subject":{"refType":"existing","kind":"entity","id":person},"predicate":"mother.media:caption",
            "object":{"type":"value","value":{"type":"string","value":"A synthetic shoreline"}},
            "validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},
            "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
            "evidence":[{"refType":"existing","kind":"evidence","id":evidence_id}],
            "confidence":{"score":0.6,"basis":"model_inference"},"extensions":extensions}}]))
    };
    let attributed = json!({"mother.media:interpretation":{"schemaVersion":1,"payload":{
        "producer":{"kind":"model","name":"synthetic-vision"},"method":"caption"}}});
    assert_eq!(
        f.store
            .apply(&f.writer, &caption(json!({}), &evidence))
            .unwrap_err()
            .code,
        GardenErrorCode::InvalidInput,
        "an unattributed caption is rejected"
    );
    let text_evidence = f
        .store
        .apply(
            &f.writer,
            &f.batch(json!([{"itemKey":"t","mutation":"create","expectedRevision":0,"draft":{"kind":"evidence",
                "source":{"type":"uri","uri":"https://example.test/note"},"excerpt":"synthetic",
                "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},"extensions":{}}}])),
        )
        .unwrap()
        .receipt
        .results[0]
        .reference
        .id
        .clone();
    assert_eq!(
        f.store
            .apply(&f.writer, &caption(attributed.clone(), &text_evidence))
            .unwrap_err()
            .code,
        GardenErrorCode::InvalidInput,
        "interpretations cite image evidence only"
    );
    let first = f
        .store
        .apply(&f.writer, &caption(attributed.clone(), &evidence))
        .unwrap();
    let second = f
        .store
        .apply(&f.writer, &caption(attributed, &evidence))
        .unwrap();
    assert_ne!(
        first.receipt.results[0].reference.id, second.receipt.results[0].reference.id,
        "several interpretations of one image coexist"
    );
    let retract = f.batch(
        json!([{"itemKey":"r","mutation":"retract","expectedRevision":1,
        "id":{"kind":"assertion","id":first.receipt.results[0].reference.id}}]),
    );
    f.store.apply(&f.writer, &retract).unwrap();
    assert_eq!(f.store.media_verify().unwrap()["ok"], true);
}

#[test]
fn bundles_written_before_media_support_still_restore() {
    let f = Fixture::new();
    f.memory();
    let bundle = f.dir.path().join("legacy");
    f.store.backup_to(&bundle).unwrap();
    let path = bundle.join("manifest.json");
    let mut manifest: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    for section in [
        "garden_media_assets",
        "garden_media_acquisitions",
        "garden_media_derivatives",
    ] {
        assert_eq!(manifest["sections"][section], 0);
        manifest["sections"]
            .as_object_mut()
            .unwrap()
            .remove(section);
    }
    std::fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
    GardenStore::verify_backup(&bundle).unwrap();
    let restored = GardenStore::restore_backup(&bundle, &f.dir.path().join("legacy.redb")).unwrap();
    assert_eq!(restored.media_verify().unwrap()["ok"], true);

    // Unknown sections are still refused.
    manifest["sections"]["garden_media_unknown"] = json!(0);
    std::fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
    assert!(GardenStore::verify_backup(&bundle).is_err());
}

#[test]
fn verification_reports_corrupt_media_and_backup_refuses_it() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let attached = f.attach(&f.file("fragile.png", &png(32)), &memory, 1, false);
    let sha256 = attached["ingest"]["sha256"].as_str().unwrap().to_owned();
    let database = f.dir.path().join("garden.redb");
    let Fixture { dir, store, .. } = f;
    drop(store);
    {
        let db = redb::Database::open(&database).unwrap();
        let tx = db.begin_write().unwrap();
        {
            let mut chunks = tx
                .open_table(redb::TableDefinition::<(&str, u64), &[u8]>::new(
                    "garden_blob_chunks",
                ))
                .unwrap();
            chunks
                .insert((sha256.as_str(), 0), [9u8; 3].as_slice())
                .unwrap();
        }
        tx.commit().unwrap();
    }
    let store = GardenStore::open(&database).unwrap();
    let report = store.media_verify().unwrap();
    assert_eq!(report["ok"], false);
    assert_eq!(report["problems"][0]["kind"], "corrupt_bytes");
    assert_eq!(report["problems"][0]["sha256"], sha256.as_str());
    assert!(store.backup_to(&dir.path().join("refused")).is_err());
    assert!(!dir.path().join("refused").exists());
}

#[test]
fn media_export_round_trips_into_another_garden_and_reports_missing_bytes() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let attached = f.attach(&f.file("trip.jpg", &jpeg_with_gps(7)), &memory, 1, true);
    let original = attached["ingest"]["sha256"].as_str().unwrap().to_owned();
    let derivative = attached["ingest"]["derivative"]["sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    let out = f.dir.path().join("media-export");
    let report = f.store.media_export_all(&out).unwrap();
    assert_eq!(
        (&report["assets"], &report["files"]),
        (&json!(2), &json!(2))
    );
    assert_eq!(report["missing"], json!([]));
    assert_eq!(
        std::fs::read(out.join(format!("{original}.jpg"))).unwrap(),
        jpeg_with_gps(7)
    );
    assert!(f.store.media_export_all(&out).is_err(), "never overwrites");

    let other = Fixture::new();
    let imported = other.store.media_import(&out).unwrap();
    assert_eq!(imported["assetsImported"], 2);
    assert_eq!(imported["acquisitionsAdded"], 2);
    assert_eq!(imported["derivativesLinked"], 1);
    let strip = |value: Value| -> Vec<Value> {
        let mut assets: Vec<Value> = value["assets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| json!([a["asset"]["sha256"], a["acquisitions"], a["derivatives"]]))
            .collect();
        assets.sort_by_key(|a| a[0].to_string());
        assets
    };
    assert_eq!(
        strip(f.store.media_owner_list(None).unwrap()),
        strip(other.store.media_owner_list(None).unwrap())
    );
    assert_eq!(other.store.media_verify().unwrap()["ok"], true);
    let again = other.store.media_import(&out).unwrap();
    assert_eq!(
        (
            again["assetsImported"].clone(),
            again["acquisitionsAdded"].clone()
        ),
        (json!(0), json!(0))
    );
    assert_eq!(again["acquisitionsAlreadyPresent"], 2);
    let _ = derivative;

    // A tampered file commits nothing.
    let third = Fixture::new();
    std::fs::write(out.join(format!("{original}.jpg")), jpeg_with_gps(8)).unwrap();
    assert_eq!(
        third.store.media_import(&out).unwrap_err().code,
        GardenErrorCode::InvalidInput
    );
    assert_eq!(
        third.store.media_owner_list(None).unwrap()["assets"],
        json!([])
    );
}

#[test]
fn media_export_lists_unreadable_bytes_instead_of_fabricating_them() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let attached = f.attach(&f.file("lost.png", &png(20)), &memory, 1, false);
    let sha256 = attached["ingest"]["sha256"].as_str().unwrap().to_owned();
    let database = f.dir.path().join("garden.redb");
    let Fixture { dir, store, .. } = f;
    drop(store);
    {
        let db = redb::Database::open(&database).unwrap();
        let tx = db.begin_write().unwrap();
        tx.open_table(redb::TableDefinition::<(&str, u64), &[u8]>::new(
            "garden_blob_chunks",
        ))
        .unwrap()
        .insert((sha256.as_str(), 0), [1u8, 2].as_slice())
        .unwrap();
        tx.commit().unwrap();
    }
    let store = GardenStore::open(&database).unwrap();
    let out = dir.path().join("partial");
    let report = store.media_export_all(&out).unwrap();
    assert_eq!(report["files"], 0);
    assert_eq!(report["missing"][0]["kind"], "unreadable_bytes");
    assert_eq!(report["missing"][0]["sha256"], sha256.as_str());
    assert!(!out.join(format!("{sha256}.png")).exists());
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(out.join("manifest.json")).unwrap()).unwrap();
    assert!(manifest["assets"][0]["file"].is_null());
}

/// Synthetic MP4: ftyp, a large mdat, and a trailing moov carrying a location atom.
fn mp4(payload: usize) -> Vec<u8> {
    let mut b = vec![0, 0, 0, 16];
    b.extend_from_slice(b"ftypisom");
    b.extend_from_slice(&[0, 0, 2, 0]);
    b.extend_from_slice(&((payload + 8) as u32).to_be_bytes());
    b.extend_from_slice(b"mdat");
    b.extend((0..payload).map(|i| (i % 251) as u8));
    let moov = b"udta\xA9xyz+12.3456+045.6789/";
    b.extend_from_slice(&((moov.len() + 8) as u32).to_be_bytes());
    b.extend_from_slice(b"moov");
    b.extend_from_slice(moov);
    b
}
fn mp3(frames: usize) -> Vec<u8> {
    let mut b = Vec::new();
    for _ in 0..frames {
        b.extend_from_slice(&[0xFF, 0xFB, 0x90, 0x00]);
        b.extend_from_slice(&[0x55; 413]);
    }
    b
}

#[test]
fn recordings_stream_into_the_store_and_keep_their_metadata_report() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    // Larger than both 4 MiB scan windows: the location atom is only in the tail.
    let video = mp4(9 * 1024 * 1024);
    let path = f.file("clip.mp4", &video);
    let attached = f.attach(&path, &memory, 1, true);
    let ingest = &attached["ingest"];
    assert_eq!(ingest["sha256"], sha(&video).as_str());
    assert_eq!(ingest["mediaType"], "video/mp4");
    assert_eq!(ingest["embeddedMetadata"], json!(["gps"]));
    assert_eq!(ingest["derivative"], Value::Null);
    assert_eq!(
        ingest["derivativeUnsupported"], true,
        "recordings are never rewritten"
    );
    let out = f.dir.path().join("clip-copy.mp4");
    f.store
        .media_export(ingest["sha256"].as_str().unwrap(), &out)
        .unwrap();
    assert_eq!(std::fs::read(&out).unwrap(), video);
    let capped = f.file("small-cap.mp4", &mp4(4096));
    assert_eq!(
        f.store
            .media_add(
                &capped,
                &AcquisitionInput::new(MediaOrigin::Captured),
                IngestOptions {
                    max_bytes: 1024,
                    strip_metadata: false,
                },
            )
            .unwrap_err()
            .code,
        GardenErrorCode::LimitExceeded
    );
    let mut declared = AcquisitionInput::new(MediaOrigin::Captured);
    declared.declared_media_type = Some("audio/mpeg".into());
    assert!(
        f.store
            .media_add(&path, &declared, IngestOptions::default())
            .is_err()
    );
    let bundle = f.dir.path().join("with-video");
    f.store.backup_to(&bundle).unwrap();
    let restored = GardenStore::restore_backup(&bundle, &f.dir.path().join("video.redb")).unwrap();
    assert_eq!(restored.media_verify().unwrap()["ok"], true);
    let exported = f.dir.path().join("recordings-export");
    f.store.media_export_all(&exported).unwrap();
    assert!(exported.join(format!("{}.mp4", sha(&video))).exists());
}

#[test]
fn audio_is_returned_over_mcp_and_video_is_not() {
    let f = Fixture::new();
    let (_, memory) = f.memory();
    let audio = mp3(4);
    let clip = f.attach(&f.file("voice.mp3", &audio), &memory, 1, false);
    let (_, second) = f.memory();
    let video = f.attach(&f.file("clean.mp4", &mp4(64)), &second, 1, false);
    let reader = f.grant(
        "listener",
        &[
            "garden:read",
            "media:meta",
            "media:bytes",
            "media:originals",
        ],
        GardenAccess::All {},
    );
    let got = f
        .store
        .media_get(&reader, &f.args(json!({"sha256":clip["ingest"]["sha256"]})))
        .unwrap();
    assert_eq!(got["mediaType"], "audio/mpeg");
    assert_eq!(got["data"], mother::media::base64(&audio));
    assert_eq!(
        f.store
            .media_get(
                &reader,
                &f.args(json!({"sha256":video["ingest"]["sha256"],"variant":"original"}))
            )
            .unwrap_err()
            .code,
        GardenErrorCode::Forbidden
    );
    let listed = f.store.media_list(&reader, &f.args(json!({}))).unwrap();
    assert_eq!(listed["assets"].as_array().unwrap().len(), 2);
}

#[test]
fn transcripts_are_attributed_interpretations_of_recordings() {
    let f = Fixture::new();
    let (person, memory) = f.memory();
    let clip = f.attach(&f.file("note.mp3", &mp3(2)), &memory, 1, false);
    let evidence = clip["outcome"]["receipt"]["results"][0]["reference"]["id"].clone();
    let batch = f.batch(json!([{"itemKey":"t","mutation":"create","expectedRevision":0,"draft":{"kind":"assertion",
        "subject":{"refType":"existing","kind":"entity","id":person},"predicate":"mother.media:transcript",
        "object":{"type":"value","value":{"type":"string","value":"A synthetic spoken note"}},
        "validTime":{"validFrom":null,"validUntil":null,"sourceTimezone":null,"precision":"unknown"},
        "observedTime":{"at":null,"sourceTimezone":null,"precision":"unknown"},
        "evidence":[{"refType":"existing","kind":"evidence","id":evidence}],"confidence":null,
        "extensions":{"mother.media:interpretation":{"schemaVersion":1,"payload":{
            "producer":{"kind":"model","name":"synthetic-speech"},"method":"transcription"}}}}}]));
    assert_eq!(
        f.store.apply(&f.writer, &batch).unwrap().receipt.status,
        OutcomeStatus::Applied
    );
}
