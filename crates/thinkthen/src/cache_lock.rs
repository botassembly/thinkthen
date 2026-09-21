//! One operating-system lock for one request digest in a cache folder.

use std::fs::{self, File};
use std::io;
use std::path::Path;

/// An exclusive digest lock, released when its file closes.
#[derive(Debug)]
pub(crate) struct CacheLock {
    _file: File,
}

/// Wait for exclusive ownership of one digest in this cache folder.
///
/// The empty lock file stays in place. Removing it could let later callers
/// lock a new file while an earlier waiter still holds the old file.
pub(crate) fn acquire(folder: &Path, digest: &str) -> io::Result<CacheLock> {
    let locks = folder.join(".locks");
    make_private(&locks)?;
    let mut options = File::options();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(locks.join(digest))?;
    file.lock()?;
    Ok(CacheLock { _file: file })
}

/// Read, optionally lock and read again, or perform live work under the lock.
pub(crate) fn coalesce<T, E, G>(
    mut replay: impl FnMut() -> Result<Option<T>, E>,
    acquire: impl FnOnce() -> Result<Option<G>, E>,
    live: impl FnOnce() -> Result<T, E>,
) -> Result<(T, bool), E> {
    if let Some(answer) = replay()? {
        return Ok((answer, true));
    }
    let guard = acquire()?;
    if guard.is_some()
        && let Some(answer) = replay()?
    {
        return Ok((answer, true));
    }
    let answer = live()?;
    drop(guard);
    Ok((answer, false))
}

/// Make the cache and lock folders readable by their owner alone.
fn make_private(folder: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(folder)?;
        fs::set_permissions(folder, fs::Permissions::from_mode(0o700))
    }
    #[cfg(not(unix))]
    fs::create_dir_all(folder)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    use super::{acquire, coalesce};

    type Answer = &'static str;

    fn replay(recorded: &Mutex<Option<Answer>>) -> io::Result<Option<Answer>> {
        Ok(recorded
            .lock()
            .map_err(|_| io::Error::other("recorded answer lock poisoned"))?
            .to_owned())
    }

    fn own(
        recorded: &Mutex<Option<Answer>>,
        folder: &Path,
        started: mpsc::Sender<()>,
        release: mpsc::Receiver<()>,
    ) -> io::Result<(Answer, bool)> {
        coalesce(
            || replay(recorded),
            || acquire(folder, "digest").map(Some),
            || {
                started.send(()).map_err(io::Error::other)?;
                release.recv().map_err(io::Error::other)?;
                *recorded
                    .lock()
                    .map_err(|_| io::Error::other("recorded answer lock poisoned"))? =
                    Some("answer");
                Ok("answer")
            },
        )
    }

    fn wait(
        recorded: &Mutex<Option<Answer>>,
        folder: &Path,
        attempted: mpsc::Sender<()>,
        fills: &AtomicUsize,
    ) -> io::Result<(Answer, bool)> {
        coalesce(
            || replay(recorded),
            || {
                attempted.send(()).map_err(io::Error::other)?;
                acquire(folder, "digest").map(Some)
            },
            || {
                fills.fetch_add(1, Ordering::SeqCst);
                Ok("wrong answer")
            },
        )
    }

    #[test]
    fn a_waiter_rereads_after_the_owner_fills_under_the_real_lock() {
        let folder = std::env::temp_dir().join(format!("thinkthen-lock-{}", std::process::id()));
        let _absent = fs::remove_dir_all(&folder);
        let recorded: Mutex<Option<&'static str>> = Mutex::new(None);
        let waiter_fills = AtomicUsize::new(0);
        let (fill_send, fill_started) = mpsc::channel();
        let (release_send, release) = mpsc::channel();
        let (attempt_send, attempted) = mpsc::channel();

        let (owner, waiter, owner_started, waiter_attempted) = thread::scope(|scope| {
            let owner_recorded = &recorded;
            let owner_folder = &folder;
            let owner = scope.spawn(move || own(owner_recorded, owner_folder, fill_send, release));
            let owner_started = fill_started.recv_timeout(Duration::from_secs(2)).is_ok();
            let waiter_recorded = &recorded;
            let waiter_folder = &folder;
            let waiter_fills = &waiter_fills;
            let waiter = scope
                .spawn(move || wait(waiter_recorded, waiter_folder, attempt_send, waiter_fills));
            let waiter_attempted = attempted.recv_timeout(Duration::from_secs(2)).is_ok();
            let _released = release_send.send(());
            (
                owner.join().expect("owner joins"),
                waiter.join().expect("waiter joins"),
                owner_started,
                waiter_attempted,
            )
        });

        assert!(owner_started);
        assert!(waiter_attempted);
        assert_eq!(owner.expect("owner result"), ("answer", false));
        assert_eq!(waiter.expect("waiter result"), ("answer", true));
        assert_eq!(waiter_fills.load(Ordering::SeqCst), 0);
        fs::remove_dir_all(folder).expect("test lock folder removed");
    }
}
