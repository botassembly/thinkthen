//! The demo runner of the `spec` rung, against fixture pages of its own.
#![cfg(feature = "cli")]

use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

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
    run::output(
        // The demos read the default cache folder under `HOME`.
        child::command("sh", &["HOME"])
            .arg("--")
            .arg(repository.join("sdlc/scripts/demos"))
            .arg(root)
            .env("PATH", reachable),
    )
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
fn every_recorded_demo_runs_and_every_demo_still_red_is_skipped() {
    let output = demos("demos").expect("the runner runs");

    let said = printed(&output);
    // The counts move with every ticket that turns a page green, so the run
    // names the pages instead. A page that failed would leave a non-zero code.
    assert_eq!(output.status.code(), Some(0), "{said}");
    assert!(said.contains(" green, "), "{said}");
    for page in [
        "01-refund-gate",
        "16-triage-pipeline",
        "12-keep-going",
        "19-no-or-could-not-ask",
        "21-options-from-the-record",
        "27-test-with-no-network",
    ] {
        assert!(
            said.contains(&format!("running {page}/README.md")),
            "{said}"
        );
    }
}
