//! Kill the real store writer without unwinding, including inside a durable sync.
use mother::{
    access::ClientAccess,
    contracts::Auth,
    garden::{
        access::GardenAccess,
        contracts::{BatchRequest, GardenKind, OutcomeResponse},
        store::{CommitStage, GardenStore},
    },
};
use redb::StorageBackend;
use std::{
    fs::{self, OpenOptions},
    io,
    ops::Bound,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

const WORKSPACE: &str = "11111111-1111-4111-8111-111111111111";
const GENERATION: &str = "22222222-2222-4222-8222-222222222222";

fn request() -> BatchRequest {
    serde_json::from_str(include_str!("fixtures/garden-v2.json")).unwrap()
}

fn supersession(initial: &OutcomeResponse) -> BatchRequest {
    let person = &initial.receipt.results[0].reference;
    let claim = &initial.receipt.results[2].reference;
    let mut assertion = serde_json::to_value(
        &initial
            .visible_records
            .iter()
            .find(|r| r.draft.kind() == GardenKind::Assertion)
            .unwrap()
            .draft,
    )
    .unwrap();
    assertion["subject"] =
        serde_json::json!({"refType":"item","kind":"entity","itemKey":"replacement"});
    serde_json::from_value(serde_json::json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,
        "operationId":"44444444-4444-4444-8444-444444444444","requiredExtensions":{},"items":[
        {"itemKey":"replacement","mutation":"create","expectedRevision":0,
            "draft":{"kind":"entity","entityType":"example:person","displayName":"Corrected synthetic person","aliases":[],"extensions":{}}},
        {"itemKey":"superseded","mutation":"supersede","id":person,"expectedRevision":1,
            "replacement":{"refType":"item","kind":"entity","itemKey":"replacement"}},
        {"itemKey":"claim","mutation":"update","id":claim,"expectedRevision":1,"draft":assertion}
    ]})).unwrap()
}

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn interrupted_transactions_reopen_as_complete_or_absent() {
    for mode in ["initial", "supersession", "approval", "dismissal"] {
        let lifecycle = mode == "supersession";
        let review_mode = matches!(mode, "approval" | "dismissal");
        for stage in ["before", "staged", "before_commit", "sync", "committed"] {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().canonicalize().unwrap();
            let nonce = uuid::Uuid::new_v4().to_string();
            fs::write(root.join("owned"), &nonce).unwrap();
            let mut child = ChildGuard(
                Command::new(std::env::current_exe().unwrap())
                    .args(["--exact", "recovery_child", "--ignored", "--nocapture"])
                    .env("MOTHER_RECOVERY_ROOT", &root)
                    .env("MOTHER_RECOVERY_NONCE", &nonce)
                    .env("MOTHER_RECOVERY_STAGE", stage)
                    .env("MOTHER_RECOVERY_MODE", mode)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap(),
            );
            let deadline = Instant::now() + Duration::from_secs(20);
            while !root.join("ready").exists() {
                assert!(
                    child.0.try_wait().unwrap().is_none(),
                    "child exited before {stage}"
                );
                assert!(Instant::now() < deadline, "child did not reach {stage}");
                std::thread::sleep(Duration::from_millis(10));
            }
            child.0.kill().unwrap();
            child.0.wait().unwrap();
            let auth: Auth =
                serde_json::from_slice(&fs::read(root.join("auth.json")).unwrap()).unwrap();
            let request: BatchRequest =
                serde_json::from_slice(&fs::read(root.join("request.json")).unwrap()).unwrap();
            let store = GardenStore::open(root.join("garden.redb")).unwrap();
            let stats = store.audit(&auth).unwrap();
            let base = u64::from(lifecycle);
            let review = if review_mode {
                Some(
                    serde_json::from_slice::<serde_json::Value>(
                        &fs::read(root.join("review.json")).unwrap(),
                    )
                    .unwrap(),
                )
            } else {
                None
            };
            let complete = if let Some(review) = &review {
                let proposal = store
                    .owner_inbox_get(review["proposalId"].as_str().unwrap())
                    .unwrap();
                let terminal = if mode == "approval" {
                    "approved"
                } else {
                    "dismissed"
                };
                assert!(proposal["status"] == "pending" || proposal["status"] == terminal);
                proposal["status"] == terminal
            } else {
                stats.sequence == base + 1
            };
            let written = complete && mode != "dismissal";
            assert_eq!(stats.sequence, base + u64::from(written), "{mode}/{stage}");
            assert_eq!(
                stats.current_records,
                if written { 3 + base } else { 3 * base },
                "{stage}"
            );
            assert_eq!(
                stats.history_records,
                3 * (base + u64::from(written)),
                "{stage}"
            );
            assert_eq!(stats.receipts, base + u64::from(written), "{stage}");
            if stage == "committed" {
                assert!(complete);
            }
            if !matches!(stage, "committed" | "sync") {
                assert!(!complete);
            }
            // The same operation UUID reconciles both a lost reply and an aborted write.
            let (applied, replay) = if let Some(review) = review {
                (
                    store.review_proposal(review.clone()).unwrap(),
                    store.review_proposal(review).unwrap(),
                )
            } else {
                (
                    store.apply(&auth, &request).unwrap(),
                    store.apply(&auth, &request).unwrap(),
                )
            };
            assert_eq!(applied, replay);
            let stats = store.audit(&auth).unwrap();
            assert_eq!(
                (
                    stats.sequence,
                    stats.current_records,
                    stats.history_records,
                    stats.receipts
                ),
                if mode == "dismissal" {
                    (0, 0, 0, 0)
                } else {
                    (base + 1, 3 + base, 3 * (base + 1), base + 1)
                }
            );
        }
    }
}

#[test]
#[ignore = "private helper executed only by the recovery parent"]
fn recovery_child() {
    let root = PathBuf::from(std::env::var_os("MOTHER_RECOVERY_ROOT").expect("parent-owned root"));
    let nonce = std::env::var("MOTHER_RECOVERY_NONCE").unwrap();
    let stage = std::env::var("MOTHER_RECOVERY_STAGE").unwrap();
    let mode = std::env::var("MOTHER_RECOVERY_MODE").unwrap();
    let root = root.canonicalize().unwrap();
    assert!(root.starts_with(std::env::temp_dir().canonicalize().unwrap()));
    assert_eq!(fs::read_to_string(root.join("owned")).unwrap(), nonce);
    let armed = Arc::new(AtomicBool::new(false));
    let file = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(root.join("garden.redb"))
        .unwrap();
    let store = GardenStore::initialize_with_backend(
        BarrierBackend {
            inner: redb::backends::FileBackend::new(file).unwrap(),
            armed: armed.clone(),
            ready: root.join("ready"),
        },
        WORKSPACE,
        GENERATION,
    )
    .unwrap();
    let auth = store
        .grant(
            "synthetic-recovery",
            &[
                "garden:read".into(),
                "garden:write".into(),
                "garden:propose".into(),
            ],
            &ClientAccess::default(),
            &GardenAccess::All {},
        )
        .unwrap();
    let request = if mode == "supersession" {
        supersession(&store.apply(&auth, &request()).unwrap())
    } else {
        request()
    };
    fs::write(root.join("auth.json"), serde_json::to_vec(&auth).unwrap()).unwrap();
    fs::write(
        root.join("request.json"),
        serde_json::to_vec(&request).unwrap(),
    )
    .unwrap();
    let hook = |at| {
        if stage == "sync" && at == CommitStage::BeforeCommit {
            armed.store(true, Ordering::Release);
        }
        if matches!(
            (stage.as_str(), at),
            ("before", CommitStage::BeforeWrite)
                | ("staged", CommitStage::Staged)
                | ("before_commit", CommitStage::BeforeCommit)
                | ("committed", CommitStage::Committed)
        ) {
            barrier(&root.join("ready"));
        }
    };
    if matches!(mode.as_str(), "approval" | "dismissal") {
        let proposal = store.propose(&auth, &request).unwrap();
        let review = serde_json::json!({"version":2,"workspaceId":WORKSPACE,"generation":GENERATION,
            "operationId":"55555555-5555-4555-8555-555555555555","proposalId":proposal.proposal_id,
            "expectedRevision":1,"action":if mode == "approval" {"approve"} else {"dismiss"}});
        fs::write(
            root.join("review.json"),
            serde_json::to_vec(&review).unwrap(),
        )
        .unwrap();
        store.review_with_hook(review, hook).unwrap();
    } else {
        store.apply_with_hook(&auth, &request, hook).unwrap();
    }
    panic!("child crossed requested interruption barrier");
}

fn barrier(ready: &std::path::Path) {
    fs::write(ready, b"ready").unwrap();
    std::thread::park_timeout(Duration::from_secs(40));
    panic!("parent did not terminate recovery child");
}

#[derive(Debug)]
struct BarrierBackend {
    inner: redb::backends::FileBackend,
    armed: Arc<AtomicBool>,
    ready: PathBuf,
}
impl StorageBackend for BarrierBackend {
    fn len(&self) -> io::Result<u64> {
        self.inner.len()
    }
    fn read(&self, offset: u64, out: &mut [u8]) -> io::Result<()> {
        self.inner.read(offset, out)
    }
    fn set_len(&self, len: u64) -> io::Result<()> {
        self.inner.set_len(len)
    }
    fn write(&self, offset: u64, data: &[u8]) -> io::Result<()> {
        self.inner.write(offset, data)
    }
    fn sync_data(&self) -> io::Result<()> {
        if self.armed.load(Ordering::Acquire) {
            barrier(&self.ready);
        }
        self.inner.sync_data()
    }
    fn close(&self) -> io::Result<()> {
        self.inner.close()
    }
    fn try_lock_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> Result<bool, redb::BackendError> {
        self.inner.try_lock_range(start, end)
    }
    fn try_lock_shared_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> Result<bool, redb::BackendError> {
        self.inner.try_lock_shared_range(start, end)
    }
    fn lock_range(&self, start: Bound<u64>, end: Bound<u64>) -> Result<(), redb::BackendError> {
        self.inner.lock_range(start, end)
    }
    fn lock_shared_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> Result<(), redb::BackendError> {
        self.inner.lock_shared_range(start, end)
    }
    fn unlock_range(&self, start: Bound<u64>, end: Bound<u64>) -> Result<(), redb::BackendError> {
        self.inner.unlock_range(start, end)
    }
    fn query_lock_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> Result<bool, redb::BackendError> {
        self.inner.query_lock_range(start, end)
    }
}
