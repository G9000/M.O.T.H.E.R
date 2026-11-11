//! An exclusive advisory file lock that is released when its guard is dropped.
use std::{fs::File, io};

/// Holds an exclusive lock on an open file.
pub(crate) struct FileLock(File);
impl FileLock {
    /// Lock without waiting; fails if another holder exists.
    pub(crate) fn try_exclusive(file: File) -> io::Result<Self> {
        fs2::FileExt::try_lock_exclusive(&file)?;
        Ok(Self(file))
    }
    pub(crate) fn file(&mut self) -> &mut File {
        &mut self.0
    }
}
// Closing the file is not enough: a child process started by another thread holds
// a copy of the descriptor until it execs, which keeps an implicit lock alive.
impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = fs2::FileExt::unlock(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn open(path: &std::path::Path) -> File {
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .unwrap()
    }
    #[test]
    fn dropping_the_guard_releases_the_lock_while_a_duplicate_descriptor_is_open() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("synthetic.lock");
        let mut first = FileLock::try_exclusive(open(&path)).unwrap();
        assert!(FileLock::try_exclusive(open(&path)).is_err());
        // A child process started by another thread briefly holds a copy of every
        // open descriptor. A duplicate stands in for that copy here.
        let inherited = first.file().try_clone().unwrap();
        drop(first);
        assert!(FileLock::try_exclusive(open(&path)).is_ok());
        drop(inherited);
    }
}
