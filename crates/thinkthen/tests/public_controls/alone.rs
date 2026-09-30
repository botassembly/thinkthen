//! Rerun one row alone in a fresh copy of this test binary.

use std::process::Stdio;

use super::{child, wait};

/// Names the one row a rerun child runs.
const ALONE: &str = "THINKTHEN_TEST_CONTROLS_ALONE";

/// Run `body` alone in a fresh copy of this test binary, because the first
/// explicit throttle selects the process's width for good (ticket 0077).
/// Nextest already runs each row alone; `cargo test` and `package` do not.
pub(super) fn alone(path: &str, body: impl FnOnce()) {
    if std::env::var_os(ALONE).is_some_and(|chosen| chosen == path) {
        body();
        return;
    }
    let binary = std::env::current_exe().expect("this test binary");
    let child = child::command(
        binary.to_str().expect("a UTF-8 test binary path"),
        &["HOME", "XDG_CACHE_HOME", "TMPDIR"],
    )
    .args([
        "--exact",
        path,
        "--include-ignored",
        "--nocapture",
        "--test-threads=1",
    ])
    .env(ALONE, path)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("the child starts");
    let output = wait::finish(child, path).expect("the child ends");
    let said = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{said}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(said.contains("1 passed"), "the child ran {path}");
}
