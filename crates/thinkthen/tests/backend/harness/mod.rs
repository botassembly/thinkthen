//! The compiled binary's spawner, beside the loopback listener it talks to.
//!
//! The listener lives in `conformance/backend`, so every binding starts the
//! same one. Spawning stays here, because only this package's own tests can
//! name the compiled `thinkthen` binary.
use crate::child::ChildEnvironment as _;
use std::io::{self, Write};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, PoisonError};
use std::time::Duration;

pub(crate) use conformance_backend::{Canned, Listener};
// Only the Unix signal and scheduling pages wait on an observed request.
#[cfg(unix)]
pub(crate) use conformance_backend::Observed;

pub(crate) use crate::wait::finish;

/// A recording folder no command can make. Unix cannot make a folder under a
/// device file, and Windows refuses `|` in a name.
pub(crate) const UNMAKEABLE: &str = if cfg!(windows) {
    "recording|folder"
} else {
    "/dev/null/recording"
};

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
    let mut child = command(arguments, environment)
        .stdin(Stdio::piped())
        .spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    Ok(child)
}

/// Run as `spawn` does, with standard input read from a file. Evidence past
/// the pipe buffer then never waits on this process's write, so no input
/// pause can close a batch early on a loaded machine (ticket 0352).
pub(crate) fn spawn_file(
    arguments: &[&str],
    environment: &[(&str, &str)],
    evidence: &std::path::Path,
) -> io::Result<Output> {
    let child = command(arguments, environment)
        .stdin(std::fs::File::open(evidence)?)
        .spawn()?;
    finish(child, &format!("thinkthen {}", arguments.join(" ")))
}

fn command(arguments: &[&str], environment: &[(&str, &str)]) -> Command {
    static SPAWNS: AtomicUsize = AtomicUsize::new(0);
    let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "spawn-home-{}-{}",
        std::process::id(),
        SPAWNS.fetch_add(1, Ordering::Relaxed)
    ));
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .clear_environment()
        .home(home)
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        // A stalled reader thread never closes a batch early (Debt 030).
        .env("THINKTHEN_TEST_INPUT_PAUSE_MS", "10000")
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in environment {
        command.env(name, value);
    }
    command
}

/// Counts the replies written with `Canned::notifying(tally.sender())`, so a
/// held reply can wait for others to be written first (ticket 0352). The
/// wait gives up after `FAILSAFE`, so a missing reply fails on counts.
#[derive(Default)]
pub(crate) struct Tally {
    written: Mutex<usize>,
    changed: Condvar,
}

impl Tally {
    pub(crate) fn new() -> std::sync::Arc<Self> {
        std::sync::Arc::default()
    }

    /// A sender for `Canned::notifying` that counts each written reply.
    pub(crate) fn sender(self: &std::sync::Arc<Self>) -> std::sync::mpsc::Sender<()> {
        let (sender, written) = std::sync::mpsc::channel::<()>();
        let tally = std::sync::Arc::clone(self);
        std::thread::spawn(move || {
            while written.recv().is_ok() {
                *tally.written.lock().unwrap_or_else(PoisonError::into_inner) += 1;
                tally.changed.notify_all();
            }
        });
        sender
    }

    /// Wait until `wanted` replies are written, or `FAILSAFE` passes.
    pub(crate) fn wait_for(&self, wanted: usize) {
        let written = self.written.lock().unwrap_or_else(PoisonError::into_inner);
        let _written = self
            .changed
            .wait_timeout_while(written, FAILSAFE, |written| *written < wanted)
            .unwrap_or_else(PoisonError::into_inner);
    }
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

const FAILSAFE: Duration = Duration::from_secs(25);

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
