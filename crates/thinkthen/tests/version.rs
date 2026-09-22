//! The compiled binary answers for its own identity.

use std::process::Command;

#[test]
fn version_flag_prints_the_identity_line_and_exits_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .arg("--version")
        .output()
        .expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!("thinkthen ", env!("CARGO_PKG_VERSION"), "\n")
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn help_opens_with_the_semantic_commands_introduction() {
    const INTRODUCTION: &str =
        "Semantic commands for the shell: if, grep, and sort that understand meaning";

    for flag in ["-h", "--help"] {
        let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .arg(flag)
            .env_clear()
            .output()
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{flag}");
        assert!(output.stderr.is_empty(), "{flag}");
        let help = String::from_utf8_lossy(&output.stdout);
        assert_eq!(help.lines().next(), Some(INTRODUCTION), "{flag}: {help}");
    }
}
