//! Cache-lock ownership, failure release, and private filesystem state.

use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
use crate::harness::process_has_file;
use crate::harness::{Canned, Listener, Observed, finish, spawn};
use crate::result_assertions::normalized_details;

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);

fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    let _absent = fs::remove_file(&path);
    path
}

fn arguments<'a>(base: &'a str, folder: &'a str) -> [&'a str; 13] {
    [
        "decide",
        QUESTION,
        "--url",
        base,
        "--model",
        "local-1",
        "--cache",
        folder,
        "--details",
        "--max-retries",
        "0",
        "--timeout",
        "1",
    ]
}

fn cached(base: &str, folder: &str, keyed: bool) -> io::Result<std::process::Output> {
    let key = if keyed {
        [("THINKTHEN_API_KEY", "sk-test-value")]
    } else {
        Default::default()
    };
    spawn(&arguments(base, folder), &key, EVIDENCE.as_bytes())
}

struct ReapedChild(Option<Child>);

impl ReapedChild {
    fn id(&self) -> io::Result<u32> {
        self.0
            .as_ref()
            .map(Child::id)
            .ok_or_else(|| io::Error::other("child already reaped"))
    }

    fn is_running(&mut self) -> io::Result<bool> {
        self.0
            .as_mut()
            .ok_or_else(|| io::Error::other("child already reaped"))?
            .try_wait()
            .map(|status| status.is_none())
    }

    fn wait(mut self) -> io::Result<Output> {
        self.0
            .take()
            .ok_or_else(|| io::Error::other("child already reaped"))
            .and_then(|child| finish(child, "a cache-lock run"))
    }

    fn write_input(&mut self, input: &[u8]) -> io::Result<()> {
        self.0
            .as_mut()
            .and_then(|child| child.stdin.take())
            .ok_or_else(|| io::Error::other("no input pipe"))?
            .write_all(input)
    }

    fn stop(&mut self) -> io::Result<()> {
        let mut child = self
            .0
            .take()
            .ok_or_else(|| io::Error::other("child already reaped"))?;
        let killed = child.kill();
        let waited = child.wait();
        killed?;
        waited.map(|_| ())
    }
}

impl Drop for ReapedChild {
    fn drop(&mut self) {
        if let Some(child) = self.0.as_mut() {
            let _killed = child.kill();
            let _reaped = child.wait();
        }
    }
}

#[cfg(target_os = "linux")]
fn wait_until_open(waiter: &ReapedChild, lock: &fs::Metadata) -> io::Result<()> {
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while !process_has_file(waiter.id()?, lock)? {
        if std::time::Instant::now() >= deadline {
            return Err(io::Error::other("waiter never opened the owner inode"));
        }
        thread::yield_now();
    }
    Ok(())
}

fn start(base: &str, folder: &str) -> io::Result<ReapedChild> {
    let child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(arguments(base, folder))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut guarded = ReapedChild(Some(child));
    guarded.write_input(EVIDENCE.as_bytes())?;
    Ok(guarded)
}

fn one_entry(folder: &Path) -> io::Result<PathBuf> {
    let entries = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|value| value == "json")
                && path
                    .file_name()
                    .is_some_and(|value| !value.to_string_lossy().starts_with('.'))
        })
        .collect::<Vec<_>>();
    let [entry] = entries.as_slice() else {
        return Err(io::Error::other(format!(
            "one cache entry, found {entries:?}"
        )));
    };
    Ok(entry.clone())
}

#[derive(Clone, Copy)]
enum FailedReply {
    Backend,
    Decode,
}

fn failure_releases_lock(kind: FailedReply) -> io::Result<()> {
    let cache = folder(match kind {
        FailedReply::Backend => "cache-backend-failure",
        FailedReply::Decode => "cache-decode-failure",
    });
    let named = cache.to_string_lossy().into_owned();
    let next = Arc::new(AtomicUsize::new(0));
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let next = Arc::clone(&next);
            move |_| {
                if next.fetch_add(1, Ordering::SeqCst) == 0 {
                    let failed = match kind {
                        FailedReply::Backend => Canned::status(500, "{}"),
                        FailedReply::Decode => Canned::ok("not json"),
                    };
                    failed.after(200)
                } else {
                    Canned::ok(ANSWER)
                }
            }
        },
        events,
    )?;
    let base = listener.base().to_owned();

    let (first, second) = thread::scope(|scope| -> io::Result<_> {
        let owner = scope.spawn(|| cached(&base, &named, true));
        assert!(matches!(
            observed.recv_timeout(Duration::from_secs(2)),
            Ok(Observed::Request)
        ));
        let waiter = scope.spawn(|| cached(&base, &named, true));
        Ok((
            owner
                .join()
                .map_err(|_| io::Error::other("owner thread failed"))??,
            waiter
                .join()
                .map_err(|_| io::Error::other("waiter thread failed"))??,
        ))
    })?;

    assert_eq!(first.status.code(), Some(4));
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert!(one_entry(&cache)?.is_file());
    Ok(())
}

#[test]
fn backend_and_decode_failures_release_the_digest_lock() {
    failure_releases_lock(FailedReply::Backend).expect("backend failure releases the lock");
    failure_releases_lock(FailedReply::Decode).expect("decode failure releases the lock");
}

#[test]
fn two_processes_share_one_request_and_the_keyless_waiter_replays() {
    let cache = folder("cache-process-race");
    let named = cache.to_string_lossy().into_owned();
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(|_| Canned::ok(ANSWER).after(500), events)
        .expect("a loopback listener");
    let base = listener.base().to_owned();

    let (owner, waiter) = thread::scope(|scope| {
        let owner = scope.spawn(|| cached(&base, &named, true));
        assert!(matches!(
            observed.recv_timeout(Duration::from_secs(2)),
            Ok(Observed::Request)
        ));
        let waiter_base = &base;
        let waiter_named = &named;
        let waiter = scope.spawn(move || cached(waiter_base, waiter_named, false));
        (
            owner.join().expect("owner joins").expect("owner runs"),
            waiter.join().expect("waiter joins").expect("waiter runs"),
        )
    });

    assert_eq!(owner.status.code(), Some(0));
    assert_eq!(waiter.status.code(), Some(0));
    let (owner_details, owner_replayed, owner_sent) =
        normalized_details(&owner).expect("owner details");
    let (waiter_details, waiter_replayed, waiter_sent) =
        normalized_details(&waiter).expect("waiter details");
    assert!(!owner_replayed);
    assert!(waiter_replayed);
    assert_eq!(owner_sent, 1);
    assert_eq!(waiter_sent, 0);
    let first_requests = listener.requests();
    assert_eq!(first_requests.len(), 1);
    assert_eq!(owner_details, waiter_details);

    let later = cached(&base, &named, false).expect("later keyless replay runs");
    assert_eq!(later.status.code(), Some(0));
    let (later_details, later_replayed, later_sent) =
        normalized_details(&later).expect("later details");
    assert!(later_replayed);
    assert_eq!(later_sent, 0);
    assert_eq!(owner_details, later_details);
    assert!(listener.requests().is_empty());
}

#[cfg(target_os = "linux")]
#[test]
fn waiter_blocks_on_the_owners_original_inode_before_install_and_unlink() {
    use std::sync::Arc;
use conformance_backend::Rendezvous;

    let cache = folder("cache-original-inode-process-race");
    let named = cache.to_string_lossy().into_owned();
    let release = Arc::new(Rendezvous::new(2));
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            move |_| Canned::ok(ANSWER).after_release(Arc::clone(&release))
        },
        events,
    )
    .expect("a loopback listener");
    let mut owner = start(listener.base(), &named).expect("owner starts");
    assert!(matches!(
        observed.recv_timeout(Duration::from_secs(2)),
        Ok(Observed::Request)
    ));
    let locks = fs::read_dir(cache.join(".locks"))
        .expect("lock folder")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    let [lock] = locks.as_slice() else {
        panic!("one owner lock, found {locks:?}")
    };
    let owner_file = lock.metadata().expect("owner lock metadata");

    let mut waiter = start(listener.base(), &named).expect("waiter starts");
    wait_until_open(&waiter, &owner_file).expect("waiter opened the owner inode");
    assert!(owner.is_running().expect("owner state"));
    assert!(waiter.is_running().expect("waiter state"));
    assert!(observed.recv_timeout(Duration::from_millis(100)).is_err());

    release.wait();
    let owner = owner.wait().expect("owner finishes");
    let waiter = waiter.wait().expect("waiter finishes");
    assert_eq!(owner.status.code(), Some(0));
    assert_eq!(waiter.status.code(), Some(0));
    let (owner_details, owner_replayed, owner_sent) =
        normalized_details(&owner).expect("owner details");
    let (waiter_details, waiter_replayed, waiter_sent) =
        normalized_details(&waiter).expect("waiter details");
    assert!(!owner_replayed);
    assert!(waiter_replayed);
    assert_eq!(owner_sent, 1);
    assert_eq!(waiter_sent, 0);
    assert_eq!(owner_details, waiter_details);
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn process_death_releases_the_digest_lock_without_recovery() {
    let cache = folder("cache-killed-owner");
    let named = cache.to_string_lossy().into_owned();
    let next = Arc::new(AtomicUsize::new(0));
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let next = Arc::clone(&next);
            move |_| {
                if next.fetch_add(1, Ordering::SeqCst) == 0 {
                    Canned::ok(ANSWER).after(500)
                } else {
                    Canned::ok(ANSWER)
                }
            }
        },
        events,
    )
    .expect("a loopback listener");
    let mut owner = start(listener.base(), &named).expect("owner starts");
    assert!(matches!(
        observed.recv_timeout(Duration::from_secs(2)),
        Ok(Observed::Request)
    ));

    let waiter = thread::spawn({
        let base = listener.base().to_owned();
        let named = named.clone();
        move || cached(&base, &named, true).expect("waiter runs")
    });
    owner.stop().expect("owner stops and is reaped");
    let output = waiter.join().expect("waiter joins");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert!(one_entry(&cache).expect("one entry").is_file());
}

#[cfg(target_os = "linux")]
#[test]
fn a_failed_owner_keeps_the_empty_lock_name_and_a_waiter_sends_nothing() {
    use std::os::unix::fs::PermissionsExt as _;
    use conformance_backend::Rendezvous;

    let cache = folder("cache-recording-failure");
    let named = cache.to_string_lossy().into_owned();
    let release = Arc::new(Rendezvous::new(2));
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            move |_| Canned::ok(ANSWER).after_release(Arc::clone(&release))
        },
        events,
    )
    .expect("a loopback listener");
    let owner = start(listener.base(), &named).expect("owner starts");
    assert!(matches!(
        observed.recv_timeout(Duration::from_secs(2)),
        Ok(Observed::Request)
    ));
    let lock = fs::read_dir(cache.join(".locks"))
        .expect("lock folder")
        .find_map(Result::ok)
        .expect("owner lock");
    let mut waiter = start(listener.base(), &named).expect("waiter starts");
    let owner_file = lock.metadata().expect("owner lock metadata");
    wait_until_open(&waiter, &owner_file).expect("waiter opened the owner inode");
    assert!(waiter.is_running().expect("waiter state"));
    // A directory at the entry stops the install even for root; mode 0500 does not.
    let entry = cache.join(lock.file_name()).with_extension("json");
    fs::create_dir(&entry).expect("entry path blocked");

    release.wait();
    let failed = owner.wait().expect("owner finishes");
    let refused = waiter.wait().expect("waiter finishes");
    assert_eq!(failed.status.code(), Some(5));
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(listener.requests().len(), 1);
    let locks = fs::read_dir(cache.join(".locks"))
        .expect("lock folder")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    let [lock] = locks.as_slice() else {
        panic!("one retained lock, found {locks:?}")
    };
    assert!(fs::read(lock.path()).expect("empty lock").is_empty());
    assert_eq!(
        lock.metadata().expect("lock metadata").permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(
        fs::metadata(cache.join(".locks"))
            .expect("lock folder metadata")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
}

#[test]
fn lock_setup_fails_before_a_key_or_request_is_needed() {
    let cache = folder("cache-lock-setup-failure");
    fs::create_dir(&cache).expect("valid cache folder");
    fs::write(cache.join(".locks"), b"not a lock directory").expect("blocking lock path");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("a listener");
    let output = cached(listener.base(), &cache.to_string_lossy(), false).expect("binary runs");

    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
    );
    assert!(listener.requests().is_empty());
}

#[cfg(unix)]
#[test]
fn a_successful_cache_fill_removes_its_private_lock_file() {
    use std::os::unix::fs::PermissionsExt as _;

    let cache = folder("cache-private-lock");
    let locks = cache.join(".locks");
    fs::create_dir_all(&locks).expect("preexisting lock directory");
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o750))
        .expect("existing cache permissions");
    fs::set_permissions(&locks, fs::Permissions::from_mode(0o777)).expect("wide test permissions");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("a listener");
    let output = cached(listener.base(), &cache.to_string_lossy(), true).expect("binary runs");
    assert_eq!(output.status.code(), Some(0));
    let files = fs::read_dir(&locks)
        .expect("lock directory")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    assert!(files.is_empty(), "completed locks leave: {files:?}");
    assert_eq!(
        fs::metadata(&cache)
            .expect("cache mode")
            .permissions()
            .mode()
            & 0o777,
        0o750
    );
    assert_eq!(
        fs::metadata(&locks)
            .expect("lock mode")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
}

#[test]
fn replay_only_creates_no_lock_directory() {
    let cache = folder("replay-without-lock");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("a listener");
    let recorded = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--record",
            &cache.to_string_lossy(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        EVIDENCE.as_bytes(),
    )
    .expect("record run");
    assert_eq!(recorded.status.code(), Some(0));
    fs::remove_dir(cache.join(".locks")).expect("empty lock folder removed for the replay check");
    let replayed = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--replay",
            &cache.to_string_lossy(),
        ],
        &[],
        EVIDENCE.as_bytes(),
    )
    .expect("replay run");

    assert_eq!(replayed.status.code(), Some(0));
    assert!(!cache.join(".locks").exists());
    assert_eq!(listener.requests().len(), 1);
}
