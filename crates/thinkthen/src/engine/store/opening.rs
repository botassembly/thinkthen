//! Keep raw creation descriptors away from SQLite's POSIX locks.

use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;

use rusqlite::{Connection, OpenFlags};

use super::storage;
use crate::engine::error::Error;

static OPENING: Mutex<()> = Mutex::new(());

pub(super) fn open(path: &Path, flags: OpenFlags) -> Result<Connection, Error> {
    let _opening = OPENING.lock().map_err(|_| Error::RecordingStorage)?;
    Connection::open_with_flags(path, flags).map_err(storage)
}

/// Close the raw descriptor before another connection can lock its inode.
pub(super) fn create(path: &Path) -> Result<Connection, Error> {
    let _opening = OPENING.lock().map_err(|_| Error::RecordingStorage)?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    match options.open(path) {
        Ok(file) => drop(file),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(Error::RecordingStorage),
    }
    Connection::open(path).map_err(storage)
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use super::{OPENING, OpenFlags, create, open};

    #[test]
    fn a_connection_waits_until_the_raw_creation_descriptor_closes() {
        for flags in [
            None,
            Some(OpenFlags::default()),
            Some(OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX),
            Some(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX),
        ] {
            waits_for_creation(flags);
        }
    }

    fn waits_for_creation(flags: Option<OpenFlags>) {
        let folder = super::super::tests::scratch();
        std::fs::create_dir_all(folder.path()).expect("folder");
        let path = folder.path().join(super::super::SQLITE);
        let opening = OPENING.lock().expect("creation guard");
        let raw = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("raw creation");
        let (started, starting) = mpsc::channel();
        let (opened, completed) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            started.send(()).expect("starting");
            let connection = flags.map_or_else(|| create(&path), |flags| open(&path, flags));
            opened.send(connection.is_ok()).expect("opened");
        });
        starting.recv().expect("worker started");
        let premature = completed.recv_timeout(Duration::from_millis(50));
        drop(raw);
        drop(opening);
        worker.join().expect("worker");
        assert!(premature.is_err(), "SQLite opened before raw close");
        assert!(completed.recv().expect("connection completed"));
    }
}
