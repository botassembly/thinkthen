//! The compiled binary's spawner, beside the loopback listener it talks to.
//!
//! The listener lives in `conformance/backend`, so every binding starts the
//! same one. Spawning stays here, because only this package's own tests can
//! name the compiled `thinkthen` binary.
use std::io::{self, Write};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) use conformance_backend::{Canned, Listener, Observed};

/// Whether this Linux process has the expected inode open.
#[cfg(target_os = "linux")]
pub(crate) fn process_has_file(process: u32, expected: &std::fs::Metadata) -> io::Result<bool> {
    use std::os::unix::fs::MetadataExt as _;

    let expected = (expected.dev(), expected.ino());
    for descriptor in std::fs::read_dir(format!("/proc/{process}/fd"))? {
        if std::fs::metadata(descriptor?.path())
            .is_ok_and(|metadata| (metadata.dev(), metadata.ino()) == expected)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Run the compiled binary with no environment but what the case names.
///
/// Every case on this binary drives the tool as a process, so the spawning,
/// the pipes, and the short retry wait live here once. The wait is set for
/// every run, because a case that never retries is not slowed by it.
pub(crate) fn spawn(
    arguments: &[&str],
    environment: &[(&str, &str)],
    evidence: &[u8],
) -> io::Result<Output> {
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
    child.wait_with_output()
}
