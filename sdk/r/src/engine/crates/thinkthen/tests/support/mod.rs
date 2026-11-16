#![cfg(test)]
//! Helpers the command's tests share.

#[cfg(unix)]
#[allow(dead_code, reason = "Only child cleanup tests observe process IDs.")]
pub(crate) mod process;

#[cfg(unix)]
use std::net::Shutdown;
#[cfg(unix)]
use std::os::fd::OwnedFd;
#[cfg(unix)]
use std::os::unix::net::UnixStream;
#[cfg(unix)]
use std::process::Stdio;
use std::process::{Command, Output};

/// A command with isolated settings and credentials that retains coverage output.
#[allow(
    dead_code,
    reason = "Library tests isolate their own test binary instead."
)]
pub(crate) fn command() -> Command {
    isolated(env!("CARGO_BIN_EXE_thinkthen"))
}

/// A process with isolated settings and credentials that retains coverage output.
pub(crate) fn isolated(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut command = Command::new(program);
    command.env_clear();
    for name in [
        "PATH",
        "SystemRoot",
        "WINDIR",
        "PATHEXT",
        "COMSPEC",
        "TEMP",
        "TMP",
    ] {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    command
}

/// The error code the command reported on standard error, or "" when none.
#[allow(
    dead_code,
    reason = "Some command tests only inspect successful output."
)]
pub(crate) fn reported(output: &Output) -> String {
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();
    let rest = stderr.strip_prefix("thinkthen: ").unwrap_or_default();
    rest.split(": ").next().unwrap_or_default().to_owned()
}

/// A failed output stream and a duplicate peer to hold until the command exits.
#[allow(
    dead_code,
    reason = "Only command tests for output failures use this fixture."
)]
#[cfg(unix)]
pub(crate) fn closed_output() -> (Stdio, UnixStream) {
    let (reader, writer) = UnixStream::pair().unwrap();
    // Keep a duplicate alive to model a reader inherited by another child.
    let inherited_reader = reader.try_clone().unwrap();
    drop(reader);
    // Closing the write direction forces EPIPE even while that reader is open.
    writer.shutdown(Shutdown::Write).unwrap();
    (Stdio::from(OwnedFd::from(writer)), inherited_reader)
}

/// Reap a stuck owned command instead of leaving a blocking pipe wait unbounded.
#[cfg(unix)]
#[allow(
    dead_code,
    reason = "only interruption tests need a whole-command deadline"
)]
pub(crate) struct Deadline {
    done: Option<std::sync::mpsc::Sender<()>>,
    watcher: Option<std::thread::JoinHandle<bool>>,
}
#[cfg(unix)]
impl Drop for Deadline {
    fn drop(&mut self) {
        let _ = self.done.take().unwrap().send(());
        let expired = self.watcher.take().unwrap().join().unwrap();
        if !std::thread::panicking() {
            assert!(!expired, "command exceeded its blocking-wait deadline");
        }
    }
}
#[cfg(unix)]
#[allow(
    dead_code,
    reason = "only interruption tests need a whole-command deadline"
)]
pub(crate) fn deadline(pid: u32) -> Deadline {
    let (done, received) = std::sync::mpsc::channel();
    let watcher = std::thread::spawn(move || {
        if received
            .recv_timeout(std::time::Duration::from_secs(30))
            .is_ok()
        {
            return false;
        }
        let _ = Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .status();
        true
    });
    Deadline {
        done: Some(done),
        watcher: Some(watcher),
    }
}

/// A record failure always prints its row or its readable code and message.
#[allow(dead_code, reason = "Only command failure cases inspect this output.")]
pub(crate) fn failed_stdout(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.is_empty(),
        "a failed result disappeared: {output:?}"
    );
    if stdout.trim_start().starts_with('{') {
        for line in stdout.lines() {
            let row: serde_json::Value = serde_json::from_str(line).unwrap();
            assert_eq!(row["status"], "failed");
            assert!(row["error"]["code"].as_str().unwrap().contains('.'));
            assert!(!row["error"]["message"].as_str().unwrap().is_empty());
        }
    } else {
        assert!(
            stdout.contains(&format!("failed: {}: ", reported(output))),
            "{stdout}"
        );
    }
}
