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

#[test]
fn tag_and_annotate_help_leads_with_the_description_and_ends_with_examples() {
    for (verb, description, examples) in [
        (
            "tag",
            "Return every applicable label as one JSON array",
            [
                "thinkthen tag 'Which topics?' --label billing='About charges.' --label urgent='Needs prompt attention.' < message.txt",
                "thinkthen tag 'Which topics?' billing urgent < message.txt",
            ],
        ),
        (
            "annotate",
            "Ask every question in a saved set and print one annotated JSON object",
            [
                "thinkthen annotate checks.json < message.txt",
                "thinkthen annotate checks.json --input message.txt",
            ],
        ),
    ] {
        for flag in ["-h", "--help"] {
            let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                .args([verb, flag])
                .env_clear()
                .output()
                .expect("the compiled binary runs");

            assert_eq!(output.status.code(), Some(0), "{verb} {flag}");
            assert!(output.stderr.is_empty(), "{verb} {flag}");
            let help = String::from_utf8_lossy(&output.stdout);
            assert!(
                help.starts_with(description),
                "{verb} {flag} does not open with its description: {help}"
            );
            let usage = help.find("Usage:").expect("a usage line");
            let options = help.find("Options:").expect("an Options: section");
            let heading = help.find("Examples:").expect("an Examples: heading");
            assert!(
                usage < options && options < heading,
                "{verb} {flag} does not order usage, options, Examples:: {help}"
            );
            let mut after = heading;
            for example in examples {
                assert_eq!(
                    help.matches(example).count(),
                    1,
                    "{verb} {flag} must print {example} once: {help}"
                );
                let at = help
                    .find(example)
                    .unwrap_or_else(|| panic!("{verb} {flag} lost {example}"));
                assert!(
                    at > after,
                    "{verb} {flag} prints {example} out of order: {help}"
                );
                after = at;
            }
        }
    }
}
