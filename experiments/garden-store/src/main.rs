//! A synthetic-only storage qualification executable with temporary owned storage.
mod bench;
#[cfg(feature = "sqlite-baseline")]
mod sqlite;

use garden_store::{
    Batch, CommitStage, Mutation, RedbStore, Result, synthetic_assertion, synthetic_entity,
};
use std::{
    env,
    io::{self, Write},
    ops::Bound,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub records: u64,
    pub edges: u64,
    pub seed: u64,
    pub sqlite: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("garden-store: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--internal-crash-child") {
        if args.len() != 2 {
            return Err("invalid private child arguments".into());
        }
        return crash_child(&args[1]);
    }
    if args.first().map(String::as_str) == Some("--internal-reopen-child") {
        let config = parse(&args[1..])?;
        let root = child_root()?;
        let output = bench::reopened(config, &root.join("garden.redb"))?;
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }
    let config = parse(&args)?;
    println!("{}", serde_json::to_string_pretty(&bench::run(config)?)?);
    Ok(())
}

fn parse(args: &[String]) -> Result<Config> {
    let mut config = Config {
        records: 10_000,
        edges: 5,
        seed: 14,
        sqlite: false,
    };
    let mut synthetic = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--synthetic" if !synthetic => synthetic = true,
            "--records" => config.records = args.next().ok_or("--records requires a value")?.parse()?,
            "--edges-per-record" => config.edges = args.next().ok_or("--edges-per-record requires a value")?.parse()?,
            "--seed" => config.seed = args.next().ok_or("--seed requires a value")?.parse()?,
            "--sqlite" => config.sqlite = true,
            _ => return Err(format!("unsupported argument: {arg}; usage: --synthetic [--records 10000|100000|1000000] [--edges-per-record 5] [--seed 14] [--sqlite]").into()),
        }
    }
    if !synthetic {
        return Err("--synthetic is required; all databases are disposable temporary data".into());
    }
    if ![10_000, 100_000, 1_000_000].contains(&config.records) {
        return Err("--records must be 10000, 100000, or 1000000".into());
    }
    if config.edges == 0 || config.edges > 32 {
        return Err("--edges-per-record must be between 1 and 32".into());
    }
    #[cfg(not(feature = "sqlite-baseline"))]
    if config.sqlite {
        return Err("--sqlite requires the optional sqlite-baseline feature".into());
    }
    Ok(config)
}

pub(crate) fn child_root() -> Result<PathBuf> {
    let root = PathBuf::from(
        env::var_os("GARDEN_SYNTHETIC_CHILD_ROOT").ok_or("private child root missing")?,
    )
    .canonicalize()?;
    let temp = env::temp_dir().canonicalize()?;
    let nonce = env::var("GARDEN_SYNTHETIC_CHILD_NONCE")?;
    if root == temp
        || !root.starts_with(&temp)
        || !nonce.starts_with("synthetic-")
        || std::fs::read_to_string(root.join("garden-synthetic-owner"))? != nonce
    {
        return Err(
            "private child requires its parent's marked synthetic temporary directory".into(),
        );
    }
    Ok(root)
}

fn crash_child(stage_name: &str) -> Result<()> {
    let target = match stage_name {
        "before-write" => CommitStage::BeforeWrite,
        "staged" => CommitStage::Staged,
        "before-commit" | "commit-sync" => CommitStage::BeforeCommit,
        "committed" => CommitStage::Committed,
        _ => return Err("unknown private crash barrier".into()),
    };
    let root = child_root()?;
    let armed = Arc::new(AtomicBool::new(false));
    let store = if stage_name == "commit-sync" {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join("crash.redb"))?;
        RedbStore::open_with_backend(CommitBarrierBackend {
            inner: redb::backends::FileBackend::new(file)?,
            armed: Arc::clone(&armed),
        })?
    } else {
        RedbStore::open(root.join("crash.redb"))?
    };
    let batch = Batch {
        operation_id: 1,
        entities: (1..=32).map(synthetic_entity).collect(),
        assertions: (1..=32)
            .map(|id| Mutation {
                assertion: synthetic_assertion(id, 0, 32, 1, 14),
                expected_revision: 0,
            })
            .collect(),
    };
    store.apply_with_hook(&batch, |stage| {
        if stage_name == "commit-sync" && stage == CommitStage::BeforeCommit {
            armed.store(true, Ordering::Release);
            return;
        }
        if stage == target {
            println!("ready:{stage_name}");
            io::stdout()
                .flush()
                .expect("private barrier stdout must flush");
            // The parent kills this process. No unwind or clean transaction drop runs.
            loop {
                std::thread::park();
            }
        }
    })?;
    Err("private child unexpectedly crossed the requested barrier".into())
}

/// Delegates real file I/O, pausing once the commit calls its first armed durable sync.
#[derive(Debug)]
struct CommitBarrierBackend {
    inner: redb::backends::FileBackend,
    armed: Arc<AtomicBool>,
}

impl redb::StorageBackend for CommitBarrierBackend {
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
            println!("ready:commit-sync");
            io::stdout().flush()?;
            loop {
                std::thread::park();
            }
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
    ) -> std::result::Result<bool, redb::BackendError> {
        self.inner.try_lock_range(start, end)
    }
    fn try_lock_shared_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> std::result::Result<bool, redb::BackendError> {
        self.inner.try_lock_shared_range(start, end)
    }
    fn lock_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> std::result::Result<(), redb::BackendError> {
        self.inner.lock_range(start, end)
    }
    fn lock_shared_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> std::result::Result<(), redb::BackendError> {
        self.inner.lock_shared_range(start, end)
    }
    fn unlock_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> std::result::Result<(), redb::BackendError> {
        self.inner.unlock_range(start, end)
    }
    fn query_lock_range(
        &self,
        start: Bound<u64>,
        end: Bound<u64>,
    ) -> std::result::Result<bool, redb::BackendError> {
        self.inner.query_lock_range(start, end)
    }
}
