#![cfg(test)]
//! Observe termination separately from reaping in command cleanup tests.

use std::process::Command;

/// A process can still be observed after termination when its parent has not reaped it.
pub(crate) fn alive(pid: &str) -> bool {
    state(pid).is_some_and(|state| !matches!(state, 'Z' | 'X'))
}

/// None means absent; an observed state remains available for stricter reaping checks.
pub(crate) fn state(pid: &str) -> Option<char> {
    // kill -0 also succeeds for zombies adopted by a container without an init.
    let output = Command::new("ps")
        .args(["-o", "stat=", "-p", pid])
        .output()
        .unwrap();
    assert!(
        output.stderr.is_empty(),
        "process observation failed: {output:?}"
    );
    let state = String::from_utf8(output.stdout).unwrap();
    if output.status.success() {
        Some(
            state
                .trim()
                .chars()
                .next()
                .expect("ps returned no process state"),
        )
    } else {
        assert_eq!(output.status.code(), Some(1));
        assert!(
            state.trim().is_empty(),
            "unsuccessful process observation: {state}"
        );
        None
    }
}

/// Capture only the named processes, including ancestry, before red-case cleanup.
pub(crate) fn evidence(pids: &[&str]) -> String {
    let output = Command::new("ps")
        .args(["-o", "pid,ppid,pgid,stat,comm", "-p", &pids.join(",")])
        .output()
        .unwrap();
    format!(
        "status: {}\n{}{}",
        output.status,
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    )
}
