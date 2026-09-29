//! The compiled binary answers for its own identity.
#![cfg(feature = "cli")]

use std::process::Command;

#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

#[test]
fn version_flag_prints_the_identity_line_and_exits_zero() {
    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .env_clear()
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

#[test]
fn help_opens_with_the_semantic_commands_introduction() {
    const INTRODUCTION: &str =
        "Semantic commands for the shell: if, grep, and sort that understand meaning";

    for flag in ["-h", "--help"] {
        let output = run::output(
            Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                .arg(flag)
                .env_clear(),
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{flag}");
        assert!(output.stderr.is_empty(), "{flag}");
        let help = String::from_utf8_lossy(&output.stdout);
        assert_eq!(help.lines().next(), Some(INTRODUCTION), "{flag}: {help}");
    }
}

#[test]
fn each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order() {
    const INTRODUCTIONS: [(&str, &str); 10] = [
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
        (
            "annotate",
            "Answer a saved set of questions about every record",
        ),
        (
            "recognize",
            "Find every name in a text and assign one of the given kinds",
        ),
        (
            "relate",
            "Find named relations across one complete entity set",
        ),
    ];

    let mut failures = Vec::new();
    for (verb, sentence) in INTRODUCTIONS {
        for flag in ["-h", "--help"] {
            let output = run::output(
                Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                    .args([verb, flag])
                    .env_clear(),
            )
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

    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .arg("-h")
            .env_clear(),
    )
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
    const ORDER: [&str; 17] = [
        "decide",
        "filter",
        "rank",
        "choose",
        "find",
        "score",
        "tag",
        "annotate",
        "recognize",
        "relate",
        "help",
        "audit",
        "diff",
        "status",
        "check",
        "cache",
        "transform",
    ];
    if listed != ORDER {
        failures.push(format!("root Commands order is {listed:?}"));
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The long or short help one command prints, refused unless it exited 0.
fn help(arguments: &[&str]) -> std::io::Result<String> {
    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(arguments)
            .env_clear(),
    )?;
    if output.status.code() != Some(0) {
        return Err(std::io::Error::other(format!("{arguments:?} failed")));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[test]
fn annotate_opens_with_the_saved_question_set_sentence_and_keeps_each_part_once() {
    const ANNOTATE: &str = "Answer a saved set of questions about every record";
    assert!(
        help(&["--help"])
            .expect("the help prints")
            .contains(&format!("\n  annotate   {ANNOTATE}\n")),
        "the root row"
    );
    let short = help(&["annotate", "-h"]).expect("the help prints");
    assert!(short.starts_with(&format!("{ANNOTATE}\n\n")), "{short}");
    let long = help(&["annotate", "--help"]).expect("the help prints");
    assert!(long.starts_with(&format!("{ANNOTATE}.\n\n")), "{long}");
    for part in [
        "Usage: thinkthen annotate",
        "\nOptions:\n",
        "The answer is one annotated JSON object.",
        "\nExamples:\n",
        "thinkthen annotate checks.json < message.txt",
        "A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers.",
        "A completed run with one or more failed questions exits 6.",
    ] {
        assert_eq!(long.matches(part).count(), 1, "{part}\n{long}");
    }
}

#[test]
fn recognize_and_relate_keep_the_beta_warning_the_cuts_and_the_disclosure() {
    const WORDING: &str = "Without kinds, fixed step-1 wording lists person, organisation, place, product, work, event or other thing; with kinds it names the caller's kinds, but descriptions reach only step 2, and accuracy outside the measured corpora is unknown.";
    let recognize = help(&["recognize", "--help"]).expect("the help prints");
    let page = include_str!("../../../specification/recognize.md");
    for (surface, text) in [("help", recognize.as_str()), ("specification", page)] {
        assert_eq!(text.matches(WORDING).count(), 1, "{surface}: {WORDING}");
    }
    for (verb, said) in [
        (
            "recognize",
            "Relations are beta. `--threshold` gates computed name strength; `--relation-threshold` gates a relation's model probability.\n",
        ),
        (
            "relate",
            "Relations are beta. Every entity leaves together as one complete entity set and sees every other entity admitted by a rule.",
        ),
        (
            "relate",
            "Keep edges whose model probability reaches this cut. [default: 0.5]",
        ),
    ] {
        let long = help(&[verb, "--help"]).expect("the help prints");
        assert_eq!(long.matches(said).count(), 1, "{verb}: {said}\n{long}");
    }
}

#[test]
fn no_page_or_transform_says_unresolved() {
    let has_retired_word = |text: &str| {
        let lower = text.to_lowercase();
        lower.match_indices("unresolved").any(|(start, word)| {
            let before = lower[..start].chars().next_back();
            let after = lower[start + word.len()..].chars().next();
            let identifier = |character: char| character.is_alphanumeric() || character == '_';
            !before.is_some_and(identifier) && !after.is_some_and(identifier)
        })
    };
    assert!(has_retired_word("an unresolved choice"));
    assert!(has_retired_word("\"unresolved\""));
    assert!(!has_retired_word("a_tie_is_unresolved"));
    assert!(!has_retired_word("unresolved_result"));
    const DEFINITION: &str = "`unsure` is the machine name for a not sure answer, in `audit`, `diff`, and the built-in transforms.";
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut defined = Vec::new();
    let mut pages = vec![
        root.join("crates/thinkthen/Cargo.toml"),
        root.join("transforms/README.md"),
    ];
    for entry in std::fs::read_dir(root.join("specification")).expect("the specification") {
        pages.push(entry.expect("a page").path());
    }
    for entry in std::fs::read_dir(root.join("crates/thinkthen/transforms"))
        .expect("the built-in transforms")
    {
        pages.push(entry.expect("a transform").path());
    }
    for page in pages.iter().filter(|page| page.is_file()) {
        let text = std::fs::read_to_string(page).expect("a readable page");
        for _ in text.matches(DEFINITION) {
            defined.push(
                page.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        assert!(
            !has_retired_word(&text),
            "{} says the old word",
            page.display()
        );
        let lower = text.to_lowercase();
        for banned in ["decider model", "decision model"] {
            assert!(!lower.contains(banned), "{} says {banned}", page.display());
        }
    }
    assert_eq!(defined, ["decide.md"]);
    let index = std::fs::read_to_string(root.join("specification/README.md")).expect("the index");
    assert!(index.contains("the five answer kinds |"), "{index}");
    let demos = std::fs::read_to_string(root.join("demos/README.md")).expect("the how-to index");
    assert!(demos.contains(" All ten functions are built. "), "{demos}");
    let readme = std::fs::read_to_string(root.join("README.md")).expect("the README");
    assert!(readme.contains("\n- A yes, a no, a not sure answer, and a broken run stay four different outcomes in the output and in the exit code.\n"), "{readme}");
    let score = std::fs::read_to_string(root.join("specification/score.md")).expect("score.md");
    assert!(score.contains("showed rubric scores rejecting"), "{score}");
}

/// Each sentence names a cost or a stop a user meets only after a run starts.
#[test]
fn the_long_help_names_connections_conflicts_paid_requests_and_models() {
    let table = [
        (
            "decide",
            " A run opens up to one connection for each request in flight, so --jobs N opens up to N connections.\n",
        ),
        (
            "decide",
            " A different fresh answer exits 5 without printing it; the old entry stays.",
        ),
        (
            "recognize",
            "\n\nEach record can make paid requests in three steps: one boundary question per text piece; one kind question per found name when kinds are given, plus an edge question when its span can change; then questions for the relation pairs allowed by rules. --dry-run prints the first record's exact boundary requests and upper bounds for later requests.\n\n",
        ),
        (
            "check",
            " The report names the model asked for, the model sent, and the model each reply names.\n\n",
        ),
    ];
    for (command, sentence) in table {
        let help = help(&[command, "--help"]).expect("the compiled binary runs");
        assert!(help.contains(sentence), "{command}: {sentence}: {help}");
    }
}
