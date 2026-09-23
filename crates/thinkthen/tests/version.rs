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

#[test]
fn each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order() {
    const INTRODUCTIONS: [(&str, &str); 9] = [
        ("decide", "Answer one yes or no question about a text"),
        ("filter", "Keep the records where the answer is yes"),
        ("rank", "Sort records by how likely the answer is yes"),
        ("choose", "Pick one option from your list"),
        (
            "find",
            "Pick the one line or record that best answers a question",
        ),
        ("score", "Place a text on a scale you name"),
        ("tag", "Name every label that fits"),
        ("annotate", "Fill out a question set for every record"),
        (
            "recognize",
            "Find every name in a text and assign one of the given kinds",
        ),
    ];

    let mut failures = Vec::new();
    for (verb, sentence) in INTRODUCTIONS {
        for flag in ["-h", "--help"] {
            let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                .args([verb, flag])
                .env_clear()
                .output()
                .expect("the compiled binary runs");
            assert_eq!(output.status.code(), Some(0), "{verb} {flag}");
            assert!(output.stderr.is_empty(), "{verb} {flag}");
            let help = String::from_utf8_lossy(&output.stdout);
            let opens = help.strip_prefix(sentence).is_some_and(|rest| {
                rest.starts_with('\n')
                    || rest
                        .strip_prefix('.')
                        .is_some_and(|after| after.starts_with([' ', '\n']))
            });
            if !opens {
                failures.push(format!(
                    "{verb} {flag} opens with {:?}",
                    help.lines().next().unwrap_or_default()
                ));
            }
        }
    }

    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .arg("-h")
        .env_clear()
        .output()
        .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(0));
    let help = String::from_utf8_lossy(&output.stdout);
    let listed: Vec<&str> = help
        .split("Commands:")
        .nth(1)
        .and_then(|rest| rest.split("\n\n").next())
        .expect("a Commands: section")
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    const ORDER: [&str; 12] = [
        "status",
        "decide",
        "filter",
        "rank",
        "choose",
        "find",
        "score",
        "tag",
        "annotate",
        "recognize",
        "cache",
        "help",
    ];
    if listed != ORDER {
        failures.push(format!("root Commands order is {listed:?}"));
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
