//! Rebuildable, bounded vector cache. It never supplies canonical record text.
use crate::{
    config::create_private_file,
    embedding::DIMENSIONS,
    error::{Error, Result},
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

pub(super) const MAX_PASSAGES: usize = 32_000;
const MAX_WINDOWS: usize = 32;
const WINDOW_BYTES: usize = 8 + DIMENSIONS * 4;
#[derive(Debug, Clone)]
pub(super) struct Vector {
    pub start: usize,
    pub end: usize,
    pub values: Vec<f32>,
}
#[derive(Debug, Clone)]
pub(super) struct Entry {
    pub partial: bool,
    pub vectors: Vec<Vector>,
}
#[derive(Debug)]
pub(super) struct Cache {
    conn: Connection,
    scope: String,
}
fn invalid() -> Error {
    Error::domain(
        "SEARCH_CACHE",
        "Derived search cache is unavailable; canonical records are unchanged.",
    )
}

pub(super) fn path(database: &Path) -> PathBuf {
    let mut name = database.as_os_str().to_os_string();
    name.push(".search.sqlite");
    PathBuf::from(name)
}
impl Cache {
    pub fn open(path: &Path, scope: &str) -> Result<Self> {
        if std::fs::metadata(path).is_ok_and(|m| m.len() > 128 * 1024 * 1024) {
            return Err(invalid());
        }
        match create_private_file(path) {
            Ok(_) => {}
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
        let mut conn = Connection::open(path)?;
        conn.busy_timeout(Duration::from_millis(100))?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let version: i64 = tx.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if version == 0 {
            let tables: i64 = tx.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |r| r.get(0),
            )?;
            if tables != 0 {
                return Err(invalid());
            }
            tx.execute_batch("CREATE TABLE scope (id INTEGER PRIMARY KEY CHECK(id=1), value TEXT NOT NULL); CREATE TABLE vectors (id TEXT PRIMARY KEY, revision INTEGER NOT NULL, digest TEXT NOT NULL, partial INTEGER NOT NULL, windows INTEGER NOT NULL, payload BLOB NOT NULL); PRAGMA user_version=1;")?;
        } else if version != 1 {
            return Err(invalid());
        }
        let current: Option<String> = tx
            .query_row("SELECT value FROM scope WHERE id=1", [], |r| r.get(0))
            .optional()?;
        if current.as_deref() != Some(scope) {
            tx.execute("DELETE FROM vectors", [])?;
            tx.execute(
                "INSERT INTO scope VALUES (1,?) ON CONFLICT(id) DO UPDATE SET value=excluded.value",
                [scope],
            )?;
        }
        tx.commit()?;
        let page_size: i64 = conn.query_row("PRAGMA page_size", [], |r| r.get(0))?;
        if page_size <= 0 {
            return Err(invalid());
        }
        conn.pragma_update(None, "max_page_count", (128_i64 * 1024 * 1024) / page_size)?;
        Ok(Self {
            conn,
            scope: scope.to_owned(),
        })
    }
    pub fn prune(&mut self, current: &HashMap<&str, (u64, &str)>) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, revision, digest FROM vectors")?;
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, i64>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        drop(stmt);
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        ensure_scope(&tx, &self.scope)?;
        for (id, revision, digest) in rows {
            if current
                .get(id.as_str())
                .is_none_or(|v| i64::try_from(v.0).ok() != Some(revision) || v.1 != digest)
            {
                tx.execute("DELETE FROM vectors WHERE id=?", [id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }
    pub fn load(&self, id: &str, revision: u64, digest: &str, text: &str) -> Result<Option<Entry>> {
        let row: Option<(bool,i64,Vec<u8>)> = self.conn.query_row("SELECT partial,windows,payload FROM vectors WHERE id=? AND revision=? AND digest=? AND (SELECT value FROM scope WHERE id=1)=?",params![id,i64::try_from(revision).map_err(|_|invalid())?,digest,self.scope],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
        row.map(|(partial, count, bytes)| {
            decode(
                partial,
                usize::try_from(count).map_err(|_| invalid())?,
                &bytes,
                text,
            )
        })
        .transpose()
    }
    pub fn count(&self) -> Result<usize> {
        let count: i64 =
            self.conn
                .query_row("SELECT COALESCE(SUM(windows),0) FROM vectors", [], |r| {
                    r.get(0)
                })?;
        usize::try_from(count).map_err(|_| invalid())
    }
    pub fn put(&mut self, id: &str, revision: u64, digest: &str, entry: &Entry) -> Result<bool> {
        if entry.vectors.len() > MAX_WINDOWS {
            return Err(invalid());
        }
        let mut bytes = Vec::with_capacity(entry.vectors.len() * WINDOW_BYTES);
        for v in &entry.vectors {
            validate_vector(&v.values)?;
            let start = u32::try_from(v.start).map_err(|_| invalid())?;
            let end = u32::try_from(v.end).map_err(|_| invalid())?;
            if start >= end {
                return Err(invalid());
            }
            bytes.extend_from_slice(&start.to_le_bytes());
            bytes.extend_from_slice(&end.to_le_bytes());
            for n in &v.values {
                bytes.extend_from_slice(&n.to_le_bytes());
            }
        }
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        ensure_scope(&tx, &self.scope)?;
        let used: i64 = tx.query_row(
            "SELECT COALESCE(SUM(windows),0) FROM vectors WHERE id<>?",
            [id],
            |r| r.get(0),
        )?;
        if usize::try_from(used).map_err(|_| invalid())? + entry.vectors.len() > MAX_PASSAGES {
            return Ok(false);
        }
        tx.execute("INSERT INTO vectors VALUES (?,?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET revision=excluded.revision,digest=excluded.digest,partial=excluded.partial,windows=excluded.windows,payload=excluded.payload",params![id,i64::try_from(revision).map_err(|_|invalid())?,digest,entry.partial,entry.vectors.len() as i64,bytes])?;
        tx.commit()?;
        Ok(true)
    }
}
fn ensure_scope(conn: &Connection, expected: &str) -> Result<()> {
    let current: String = conn.query_row("SELECT value FROM scope WHERE id=1", [], |r| r.get(0))?;
    if current != expected {
        return Err(invalid());
    }
    Ok(())
}
pub(super) fn validate_vector(values: &[f32]) -> Result<()> {
    if values.len() != DIMENSIONS || values.iter().any(|n| !n.is_finite()) {
        return Err(invalid());
    }
    let norm: f32 = values.iter().map(|v| v * v).sum();
    if !(0.98..=1.02).contains(&norm) {
        return Err(invalid());
    }
    Ok(())
}
fn decode(partial: bool, count: usize, bytes: &[u8], text: &str) -> Result<Entry> {
    if count > MAX_WINDOWS || bytes.len() != count * WINDOW_BYTES {
        return Err(invalid());
    }
    let mut vectors = Vec::with_capacity(count);
    for raw in bytes.chunks_exact(WINDOW_BYTES) {
        let start = u32::from_le_bytes(raw[0..4].try_into().map_err(|_| invalid())?) as usize;
        let end = u32::from_le_bytes(raw[4..8].try_into().map_err(|_| invalid())?) as usize;
        if start >= end
            || end > text.len()
            || !text.is_char_boundary(start)
            || !text.is_char_boundary(end)
        {
            return Err(invalid());
        }
        let values: Vec<f32> = raw[8..]
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        validate_vector(&values)?;
        vectors.push(Vector { start, end, values });
    }
    Ok(Entry { partial, vectors })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn entry() -> Entry {
        let mut values = vec![0.; DIMENSIONS];
        values[0] = 1.;
        Entry {
            partial: false,
            vectors: vec![Vector {
                start: 0,
                end: 3,
                values,
            }],
        }
    }
    #[test]
    fn another_process_changing_profile_invalidates_open_cache_handles() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("cache");
        let mut old = Cache::open(&p, "old-profile").unwrap();
        old.put("r", 1, "h", &entry()).unwrap();
        let mut newer = Cache::open(&p, "new-profile").unwrap();
        newer.put("r", 1, "h", &entry()).unwrap();
        assert!(old.load("r", 1, "h", "abc").unwrap().is_none());
        assert!(old.put("r", 1, "h", &entry()).is_err());
        assert!(old.prune(&HashMap::new()).is_err());
        assert_eq!(newer.count().unwrap(), 1);
    }
    #[test]
    fn zero_token_outcomes_are_persisted_without_blocking_later_records() {
        let d = tempfile::tempdir().unwrap();
        let mut c = Cache::open(&d.path().join("cache"), "scope").unwrap();
        assert!(
            c.put(
                "empty",
                1,
                "h",
                &Entry {
                    partial: false,
                    vectors: vec![]
                }
            )
            .unwrap()
        );
        assert!(
            c.load("empty", 1, "h", "\u{fffd}")
                .unwrap()
                .unwrap()
                .vectors
                .is_empty()
        );
        assert!(c.put("normal", 1, "h", &entry()).unwrap());
        assert_eq!(c.count().unwrap(), 1);
    }
    #[test]
    fn cache_is_revision_digest_and_scope_bound() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("cache.sqlite");
        let mut c = Cache::open(&p, "workspace-generation-profile").unwrap();
        assert!(c.put("record", 1, "hash", &entry()).unwrap());
        assert!(c.load("record", 1, "hash", "abc").unwrap().is_some());
        assert!(c.load("record", 2, "hash", "abc").unwrap().is_none());
        assert!(c.load("record", 1, "different", "abc").unwrap().is_none());
        drop(c);
        let c = Cache::open(&p, "workspace-generation-profile").unwrap();
        assert_eq!(c.count().unwrap(), 1);
        drop(c);
        let c = Cache::open(&p, "different-generation").unwrap();
        assert_eq!(c.count().unwrap(), 0);
    }
    #[test]
    fn corrupt_vectors_offsets_and_unrelated_files_are_rejected() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("cache.sqlite");
        let mut c = Cache::open(&p, "scope").unwrap();
        let mut e = entry();
        e.vectors[0].values[0] = f32::NAN;
        assert!(c.put("r", 1, "h", &e).is_err());
        c.put("r", 1, "h", &entry()).unwrap();
        assert!(c.load("r", 1, "h", "é").is_err());
        c.conn
            .execute("UPDATE vectors SET payload=x'00'", [])
            .unwrap();
        assert!(c.load("r", 1, "h", "abc").is_err());
        let p2 = d.path().join("unrelated.sqlite");
        let c2 = Connection::open(&p2).unwrap();
        c2.execute("CREATE TABLE owner_data(value)", []).unwrap();
        drop(c2);
        assert!(Cache::open(&p2, "scope").is_err());
    }
    #[test]
    fn stale_entries_are_pruned_and_admission_is_bounded() {
        let d = tempfile::tempdir().unwrap();
        let mut c = Cache::open(&d.path().join("cache"), "scope").unwrap();
        c.put("r", 1, "h", &entry()).unwrap();
        c.prune(&HashMap::from([("r", (2, "h"))])).unwrap();
        assert_eq!(c.count().unwrap(), 0);
        c.put("old", 1, "h", &entry()).unwrap();
        c.conn
            .execute("UPDATE vectors SET windows=?", [MAX_PASSAGES as i64])
            .unwrap();
        assert!(!c.put("new", 1, "h", &entry()).unwrap());
    }
}
