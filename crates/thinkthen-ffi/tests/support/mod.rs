#![cfg(test)]
//! Native consumers share one isolated process boundary.
use std::process::Command;
#[allow(
    dead_code,
    reason = "feature-disabled native test crates have no process to start"
)]
pub(crate) fn command(binary: &str) -> Command {
    let test = std::env::current_exe().unwrap();
    let executable = test
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("{binary}{}", std::env::consts::EXE_SUFFIX));
    let mut command = Command::new(executable);
    command.env_clear();
    // Keep OS process requirements and coverage, while omitting home, config and credentials.
    for name in [
        "SystemRoot",
        "COMSPEC",
        "PATHEXT",
        "PATH",
        "TMP",
        "TEMP",
        "LLVM_PROFILE_FILE",
    ] {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    command
}
