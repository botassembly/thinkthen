//! The `tag` command at the binary edge.

use std::process::Command;

#[test]
fn tag_short_help_leads_with_described_labels() {
    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["tag", "-h"])
        .env_clear()
        .output()
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("--label <LABEL=DESCRIPTION>"), "{help}");
    assert!(help.contains("[LABEL]..."), "{help}");
}
