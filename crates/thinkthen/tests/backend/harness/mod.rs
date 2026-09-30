//! The compiled binary's spawner, beside the loopback listener it talks to.
//!
//! The listener lives in `conformance/backend`, so every binding starts the
//! same one. Spawning stays here, because only this package's own tests can
//! name the compiled `thinkthen` binary.
use std::io::{self, Write};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, PoisonError};
use std::time::Duration;

pub(crate) use conformance_backend::{Canned, Listener, Observed};

pub(crate) use crate::wait::finish;

/// Run the compiled binary with no environment but what the case names.
///
/// Every case on this binary drives the tool as a process, so the spawning,
/// the pipes, the short retry wait, and the run's deadline live here once.
/// The retry wait is set for every run, because a case that never retries is
/// not slowed by it.
pub(crate) fn spawn(
    arguments: &[&str],
    environment: &[(&str, &str)],
    evidence: &[u8],
) -> io::Result<Output> {
    let child = start(arguments, environment, evidence)?;
    finish(child, &format!("thinkthen {}", arguments.join(" ")))
}

/// Run as `spawn` does, one record a request, as every run was before
/// batching. A file whose subject is not batching imports it as `spawn`.
pub(crate) fn spawn_one(
    arguments: &[&str],
    environment: &[(&str, &str)],
    evidence: &[u8],
) -> io::Result<Output> {
    spawn(
        arguments,
        &[environment, &[("THINKTHEN_BATCH", "1")]].concat(),
        evidence,
    )
}

/// Start the compiled binary as `spawn` does, feed it the evidence, and hand
/// back the running child.
pub(crate) fn start(
    arguments: &[&str],
    environment: &[(&str, &str)],
    evidence: &[u8],
) -> io::Result<Child> {
    static SPAWNS: AtomicUsize = AtomicUsize::new(0);
    let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "spawn-home-{}-{}",
        std::process::id(),
        SPAWNS.fetch_add(1, Ordering::Relaxed)
    ));
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("HOME", home)
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in environment {
        command.env(name, value);
    }
    let mut child = command.spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    Ok(child)
}

/// Holds each of the first `wanted` requests until all of them are in flight.
///
/// A reply closure calls `hold` before it answers. A correct run passes on
/// counts alone, however slowly the machine schedules its workers. A run that
/// never reaches `wanted` waits out `FAILSAFE` and then fails on its peak. The
/// failsafe stays under the tool's 30-second request timeout.
pub(crate) struct Gathering {
    arrived: Mutex<usize>,
    all_here: Condvar,
    wanted: usize,
}

const FAILSAFE: Duration = Duration::from_secs(10);

impl Gathering {
    pub(crate) fn new(wanted: usize) -> Self {
        Self {
            arrived: Mutex::new(0),
            all_here: Condvar::new(),
            wanted,
        }
    }

    pub(crate) fn hold(&self) {
        let mut arrived = self.arrived.lock().unwrap_or_else(PoisonError::into_inner);
        *arrived += 1;
        if *arrived == self.wanted {
            // A request past the bound gets a moment to arrive while all are held.
            std::thread::sleep(Duration::from_millis(50));
            self.all_here.notify_all();
        }
        if *arrived <= self.wanted {
            let _held = self
                .all_here
                .wait_timeout_while(arrived, FAILSAFE, |count| *count < self.wanted);
        }
    }
}
