#![cfg(test)]
//! Helpers the command's tests share.

use std::net::Shutdown;
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::process::{Command, Output, Stdio};

/// A command with isolated settings and credentials that retains coverage output.
pub(crate) fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command.env_clear();
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
pub(crate) fn closed_output() -> (Stdio, UnixStream) {
    let (reader, writer) = UnixStream::pair().unwrap();
    // Keep a duplicate alive to model a reader inherited by another child.
    let inherited_reader = reader.try_clone().unwrap();
    drop(reader);
    // Closing the write direction forces EPIPE even while that reader is open.
    writer.shutdown(Shutdown::Write).unwrap();
    (Stdio::from(OwnedFd::from(writer)), inherited_reader)
}
