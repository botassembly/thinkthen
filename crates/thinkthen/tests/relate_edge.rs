//! The public relate command boundary.

use std::process::Command;

#[test]
fn help_names_the_beta_complete_set_and_secrecy_contract() {
    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["relate", "--help"])
        .output()
        .expect("binary runs");
    assert_eq!(output.status.code(), Some(0));
    let help = String::from_utf8_lossy(&output.stdout);
    for required in [
        "Relations are beta",
        "complete entity set",
        "NAME=SOURCE_KIND:TARGET_KIND",
        "--kind-field",
        "Entries contain the judged text",
    ] {
        assert!(help.contains(required), "{required}\n{help}");
    }
}
