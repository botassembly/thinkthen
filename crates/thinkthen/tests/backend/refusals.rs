//! What `filter` and `rank` refuse, what they replay, and what they never say.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};
use crate::keeping::{RECORDS, answered, code, printed, said};

/// The question both commands ask of each record.
const QUESTION: &str = "Does this report a payment failure?";

/// A port nothing listens on, so a connection would be refused at once.
const CLOSED: &str = "http://127.0.0.1:1/v1";

/// A folder this test owns, removed and remade so each run starts empty.
fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

/// A file under the test's own temporary folder, holding this text.
fn written(name: &str, text: &str) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::write(&path, text)?;
    Ok(path)
}

/// Run one command with no address, which a refused command never needs.
fn refused(arguments: &[&str]) -> io::Result<Output> {
    spawn(
        arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        RECORDS.as_bytes(),
    )
}

#[test]
fn a_band_is_refused_by_filter_in_either_home_and_names_the_way_to_three_piles() -> io::Result<()> {
    let output = refused(&[
        "filter",
        QUESTION,
        "--jsonl",
        "--field",
        "/body",
        "--threshold",
        "0.1:0.9",
    ])?;
    assert_eq!(code(&output), 2);
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: --threshold: `filter` takes a single cut, so ask ",
            "`decide --details` and split the three piles with `jq`\n",
        )
    );
    assert_eq!(printed(&output), "");

    let file = written(
        "band-question.json",
        r#"{"decide":"Does this report a payment failure?","threshold":"0.1:0.9"}"#,
    )?;
    let path = format!("@{}", file.display());
    let output = refused(&["filter", &path, "--jsonl", "--field", "/body"])?;
    assert_eq!(code(&output), 5, "a value from the file is a local failure");
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: the question file's `threshold`: `filter` takes a single cut, ",
            "so ask `decide --details` and split the three piles with `jq`\n",
        )
    );

    let output = refused(&["rank", &path, "--jsonl", "--field", "/body"])?;
    assert_eq!(code(&output), 5);
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: the question file's `threshold`: `rank` orders and never ",
            "selects, so put a cut in `filter --threshold`\n",
        )
    );
    Ok(())
}

#[test]
fn rank_refuses_a_rule_from_either_home_and_names_the_command_that_cuts() -> io::Result<()> {
    let output = refused(&[
        "rank",
        QUESTION,
        "--jsonl",
        "--field",
        "/body",
        "--threshold",
        "0.9",
    ])?;
    assert_eq!(code(&output), 2);
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: --threshold: `rank` orders and never selects, ",
            "so put a cut in `filter --threshold`\n",
        )
    );

    let file = written(
        "cut-question.json",
        r#"{"decide":"Does this report a payment failure?","threshold":0.9}"#,
    )?;
    let output = refused(&[
        "rank",
        &format!("@{}", file.display()),
        "--jsonl",
        "--field",
        "/body",
    ])?;
    assert_eq!(code(&output), 5);
    assert!(
        said(&output).contains("`rank` orders and never selects"),
        "{}",
        said(&output)
    );
    Ok(())
}

#[test]
fn each_view_and_the_missing_framing_are_refused_in_the_tools_own_words() -> io::Result<()> {
    let cases = [
        (
            vec!["filter", QUESTION, "--jsonl", "--field", "/body", "--quiet"],
            concat!(
                "thinkthen: --quiet prints nothing, and `filter` answers with the records ",
                "it prints; `decide --quiet` carries one answer in the exit code\n",
            ),
        ),
        (
            vec!["rank", QUESTION, "--jsonl", "--field", "/body", "--quiet"],
            concat!(
                "thinkthen: --quiet prints nothing, and `rank` answers with the records ",
                "it prints; `decide --quiet` carries one answer in the exit code\n",
            ),
        ),
        (
            vec!["filter", QUESTION, "--jsonl", "--field", "/body", "--raw"],
            concat!(
                "thinkthen: --raw prints a bare label, and `filter` prints records; ",
                "`choose --raw` prints a label\n",
            ),
        ),
        (
            vec!["rank", QUESTION, "--jsonl", "--field", "/body", "--raw"],
            concat!(
                "thinkthen: --raw prints a bare label, and `rank` prints records; ",
                "`choose --raw` prints a label\n",
            ),
        ),
        (
            vec!["filter", QUESTION, "--field", "/body"],
            "thinkthen: `filter` maps over a stream, so it takes --lines or --jsonl\n",
        ),
        (
            vec!["rank", QUESTION],
            "thinkthen: `rank` maps over a stream, so it takes --lines or --jsonl\n",
        ),
    ];
    for (arguments, message) in cases {
        let output = refused(&arguments)?;
        assert_eq!(code(&output), 2, "{arguments:?}");
        assert_eq!(said(&output), message, "{arguments:?}");
        assert_eq!(printed(&output), "", "{arguments:?}");
    }
    Ok(())
}

#[test]
fn top_takes_a_whole_number_of_one_or_more_and_says_so() -> io::Result<()> {
    let cases = [
        (
            "0",
            "thinkthen: --top prints the first N of the order, and N is 1 or more",
        ),
        (
            "half",
            "error: invalid value 'half' for '--top <N>': invalid digit found in string",
        ),
        // A negative number is not a value at all to the parser, which reads
        // the leading dash as the start of another option.
        ("-1", "error: unexpected argument '-1' found"),
    ];
    for (asked, message) in cases {
        let output = refused(&[
            "rank", QUESTION, "--jsonl", "--field", "/body", "--top", asked,
        ])?;
        assert_eq!(code(&output), 2, "--top {asked}");
        assert_eq!(said(&output).lines().next(), Some(message), "--top {asked}");
        assert_eq!(printed(&output), "", "--top {asked}");
    }
    Ok(())
}

#[test]
fn a_recording_made_by_decide_replays_under_both_commands_with_no_request_sent() -> io::Result<()> {
    let cache = folder("keeping-replay");
    let listener = Listener::answering(|body| {
        let sent = String::from_utf8_lossy(body).into_owned();
        Canned::ok(&answered(if sent.contains("payout") {
            "0.91"
        } else {
            "0.02"
        }))
    })?;
    let base = listener.base().to_owned();

    let made = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            &base,
            "--model",
            "local-1",
            "--jsonl",
            "--field",
            "/body",
            "--record",
            &cache.display().to_string(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        RECORDS.as_bytes(),
    )?;
    assert_eq!(code(&made), 0, "{}", said(&made));
    let paid = listener.requests().len();
    assert_eq!(paid, 4);

    for verb in ["filter", "rank"] {
        let output = spawn(
            &[
                verb,
                QUESTION,
                "--url",
                &base,
                "--model",
                "local-1",
                "--jsonl",
                "--field",
                "/body",
                "--replay",
                &cache.display().to_string(),
            ],
            &[],
            RECORDS.as_bytes(),
        )?;
        assert_eq!(code(&output), 0, "{verb}: {}", said(&output));
        assert!(!printed(&output).is_empty(), "{verb} printed nothing");
        assert_eq!(
            listener.requests().len(),
            0,
            "{verb} sent a request to the listener"
        );
    }
    Ok(())
}

#[test]
fn no_message_from_either_command_ever_carries_the_key_or_a_record() -> io::Result<()> {
    let key = "sk-secret-value";
    let file = written(
        "refused-question.json",
        r#"{"decide":"Does this report a payment failure?","threshold":"0.1:0.9"}"#,
    )?;
    let path = format!("@{}", file.display());
    let bad = written("not-json.jsonl", "{\"id\":\"R-9\",\"body\":\n")?;
    let bad = bad.display().to_string();
    let cases: Vec<Vec<&str>> = vec![
        vec!["filter", QUESTION, "--jsonl", "--field", "/body", "--quiet"],
        vec!["rank", QUESTION, "--jsonl", "--field", "/body", "--raw"],
        vec![
            "filter",
            QUESTION,
            "--jsonl",
            "--field",
            "/body",
            "--threshold",
            "0.1:0.9",
        ],
        vec![
            "rank",
            QUESTION,
            "--jsonl",
            "--field",
            "/body",
            "--threshold",
            "0.9",
        ],
        vec!["filter", &path, "--jsonl", "--field", "/body"],
        vec!["rank", &path, "--jsonl", "--field", "/body"],
        vec!["filter", QUESTION, "--field", "/body"],
        vec!["rank", QUESTION],
        vec!["filter", QUESTION, "--jsonl", "--field", "/nowhere"],
        vec!["rank", QUESTION, "--jsonl", "--field", "$.body"],
        vec![
            "filter", QUESTION, "--jsonl", "--field", "/body", "--input", &bad,
        ],
        vec![
            "rank", QUESTION, "--jsonl", "--field", "/body", "--jobs", "99",
        ],
        vec![
            "filter", QUESTION, "--jsonl", "--field", "/body", "--url", CLOSED,
        ],
        vec![
            "rank", QUESTION, "--jsonl", "--field", "/body", "--url", CLOSED,
        ],
    ];
    for arguments in cases {
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", key), ("RUST_BACKTRACE", "full")],
            RECORDS.as_bytes(),
        )?;
        assert_ne!(code(&output), 0, "{arguments:?} was not refused");
        let both = format!("{}{}", printed(&output), said(&output));
        for secret in [
            key,
            "payout",
            "quick fix",
            "checkout",
            "refund never",
            "bearer",
        ] {
            assert!(
                !both.to_lowercase().contains(&secret.to_lowercase()),
                "{arguments:?} said `{secret}`: {both}"
            );
        }
    }
    Ok(())
}
