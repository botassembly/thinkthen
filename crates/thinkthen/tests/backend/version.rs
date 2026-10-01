//! The compiled binary answers for its own identity.
#![cfg(feature = "cli")]

use crate::child::ChildEnvironment as _;
use std::process::Command;

use crate::run;

#[test]
fn version_flag_prints_the_identity_line_and_exits_zero() {
    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .clear_environment()
            .arg("--version"),
    )
    .expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!("thinkthen ", env!("CARGO_PKG_VERSION"), "\n")
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}
