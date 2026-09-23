//! The folder gate keeps pruning in the digest-lock namespace.

use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use crate::harness::{Canned, Listener, Observed, process_has_file};

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);

struct Reaped(Option<Child>);

impl Reaped {
    fn id(&self) -> io::Result<u32> {
        self.0
            .as_ref()
            .map(Child::id)
            .ok_or_else(|| io::Error::other("child already reaped"))
    }

    fn wait(mut self) -> io::Result<Output> {
        self.0
            .take()
            .ok_or_else(|| io::Error::other("child already reaped"))?
            .wait_with_output()
    }
}

impl Drop for Reaped {
    fn drop(&mut self) {
        if let Some(child) = self.0.as_mut() {
            let _killed = child.kill();
            let _reaped = child.wait();
        }
    }
}

fn folder() -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cache-prune-folder-gate");
    let _absent = fs::remove_dir_all(&path);
    path
}

fn decide(base: &str, folder: &str) -> io::Result<Reaped> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
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
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no input pipe"))?
        .write_all(EVIDENCE.as_bytes())?;
    Ok(Reaped(Some(child)))
}

fn prune(folder: &str) -> io::Result<Reaped> {
    let child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .args(["cache", "prune", folder, "--max-size", "1"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    Ok(Reaped(Some(child)))
}

fn wait_on(child: &Reaped, file: &fs::Metadata, what: &str) -> io::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(2);
    while !process_has_file(child.id()?, file)? {
        assert!(Instant::now() < deadline, "{what} never waited on inode");
        thread::yield_now();
    }
    Ok(())
}

#[test]
fn prune_waits_for_owner_and_original_inode_waiter_before_a_later_fill() {
    let cache = folder();
    let named = cache.to_string_lossy().into_owned();
    let release = Arc::new(Barrier::new(2));
    let calls = Arc::new(AtomicUsize::new(0));
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            let calls = Arc::clone(&calls);
            move |_| {
                if calls.fetch_add(1, Ordering::SeqCst) == 0 {
                    Canned::ok(ANSWER).after_release(Arc::clone(&release))
                } else {
                    Canned::ok(ANSWER)
                }
            }
        },
        events,
    )
    .expect("listener");

    let owner = decide(listener.base(), &named).expect("owner starts");
    assert!(matches!(
        observed.recv_timeout(Duration::from_secs(2)),
        Ok(Observed::Request)
    ));
    let lock = fs::read_dir(cache.join(".locks"))
        .expect("lock folder")
        .next()
        .expect("one lock")
        .expect("lock entry");
    let lock_file = lock.metadata().expect("lock metadata");
    let waiter = decide(listener.base(), &named).expect("waiter starts");
    wait_on(&waiter, &lock_file, "waiter").expect("waiter state");

    let directory = fs::metadata(&cache).expect("cache metadata");
    let pruner = prune(&named).expect("pruner starts");
    wait_on(&pruner, &directory, "pruner").expect("pruner state");
    assert_eq!(
        fs::read_dir(cache.join(".locks")).expect("locks").count(),
        1
    );
    assert!(observed.recv_timeout(Duration::from_millis(100)).is_err());

    release.wait();
    assert_eq!(owner.wait().expect("owner output").status.code(), Some(0));
    assert_eq!(waiter.wait().expect("waiter output").status.code(), Some(0));
    let pruned = pruner.wait().expect("pruner output");
    assert_eq!(pruned.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&pruned.stdout).starts_with("removed 1 entries and "));
    let later = decide(listener.base(), &named).expect("later caller starts");
    assert_eq!(later.wait().expect("later output").status.code(), Some(0));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
