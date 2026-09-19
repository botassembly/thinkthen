//! The demo runner of the `spec` rung, against fixture pages of its own.

use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The repository the compiled binary was built inside.
fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap_or(Path::new("."))
        .to_owned()
}

/// Run the demo runner over one root of demo folders.
fn demos(root: &str) -> io::Result<Output> {
    let repository = repository();
    let binary = Path::new(env!("CARGO_BIN_EXE_thinkthen"));
    let reachable = match (binary.parent(), env::var("PATH")) {
        (Some(folder), Ok(path)) => format!("{}:{path}", folder.display()),
        (Some(folder), Err(_)) => folder.display().to_string(),
        (None, path) => path.unwrap_or_default(),
    };
    Command::new("sh")
        .arg("--")
        .arg(repository.join("sdlc/scripts/demos"))
        .arg(root)
        .env("PATH", reachable)
        .output()
}

/// Everything the run printed, whichever channel carried it.
fn printed(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn a_green_page_runs_against_its_own_files_and_passes() {
    let output = demos("crates/thinkthen/tests/fixtures/demos").expect("the runner runs");

    let said = printed(&output);
    assert_eq!(output.status.code(), Some(0), "{said}");
    assert!(said.contains("running 01-replay-gate/README.md"), "{said}");
    assert!(said.contains("demos: 1 green, 0 red"), "{said}");
}

#[test]
fn a_page_whose_assertion_is_wrong_fails_the_run() {
    let output = demos("crates/thinkthen/tests/fixtures/demos-wrong").expect("the runner runs");

    let said = printed(&output);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(said.contains("1 failed"), "{said}");
}

#[test]
fn a_green_page_that_names_no_recording_folder_stops_the_run() {
    let output = demos("crates/thinkthen/tests/fixtures/demos-missing").expect("the runner runs");

    let said = printed(&output);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(said.contains("names no folder recording/"), "{said}");
    assert!(!said.contains("running"), "{said}");
}

#[test]
fn the_recorded_demo_runs_and_every_demo_still_red_is_skipped() {
    let output = demos("demos").expect("the runner runs");

    let said = printed(&output);
    assert_eq!(output.status.code(), Some(0), "{said}");
    assert!(said.contains("demos: 1 green, "), "{said}");
    assert!(said.contains("running 01-refund-gate/README.md"), "{said}");
}
