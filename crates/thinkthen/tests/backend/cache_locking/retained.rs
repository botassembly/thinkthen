//! Stable digest-lock names through death, failure, success, and replay.

use super::*;

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
    use conformance_backend::Rendezvous;
    use std::os::unix::fs::PermissionsExt as _;

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
fn a_successful_cache_fill_retains_its_private_lock_file() {
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
    let [lock] = files.as_slice() else {
        panic!("one retained lock, found {files:?}");
    };
    assert!(fs::read(lock.path()).expect("empty lock").is_empty());
    assert_eq!(
        lock.metadata().expect("lock mode").permissions().mode() & 0o777,
        0o600
    );
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
            "find",
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
    fs::remove_dir_all(cache.join(".locks")).expect("lock folder removed for the replay check");
    let replayed = spawn(
        &[
            "find",
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
