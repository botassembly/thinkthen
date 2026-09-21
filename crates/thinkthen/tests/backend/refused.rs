//! What `filter` and `rank` refuse, what they replay, and what they never say.
//!
//! `refusals.rs` drives every refusal of the three judging verbs from one
//! table. The two record verbs take no operands and refuse the two views that
//! table drives, so their refusals are pinned here, sentence by sentence.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};
use crate::keeping::{RECORDS, answered, code, printed, said};
use crate::secrecy::{KEY, nothing_leaked};

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
fn record_verbs_name_their_own_required_question_kind() -> io::Result<()> {
    let file = written(
        "wrong-kind-question.json",
        r#"{"choose":"Which team?","options":["a","b"]}"#,
    )?;
    let question = format!("@{}", file.display());
    for command in ["filter", "rank"] {
        let output = refused(&[command, &question, "--lines"])?;
        assert_eq!(code(&output), 2, "{command}");
        assert_eq!(
            said(&output),
            format!(
                "thinkthen: `{command}` reads a `decide` question, but the question file holds a `choose` question\n"
            )
        );
        assert!(printed(&output).is_empty());
    }
    Ok(())
}

#[test]
fn a_rank_plan_names_only_sources_rank_takes() -> io::Result<()> {
    let file = written(
        "rank-plan-question.json",
        r#"{"decide":"Does this report a payment failure?","true":"yes side","false":"no side"}"#,
    )?;
    let question = format!("@{}", file.display());
    let output = refused(&["rank", &question, "--lines", "--dry-run"])?;
    assert_eq!(code(&output), 0);
    let plan = printed(&output);
    assert!(
        plan.contains(r#""from":{"question":"file","true":"file","false":"file","on":"default","model":"default"}"#),
        "{plan}"
    );
    assert!(!plan.contains(r#""threshold":"default""#), "{plan}");
    assert!(said(&output).is_empty());
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
            "thinkthen: `filter` maps over a stream, so it takes --lines, --jsonl, --csv, or --tsv\n",
        ),
        (
            vec!["rank", QUESTION],
            "thinkthen: `rank` maps over a stream, so it takes --lines, --jsonl, --csv, or --tsv\n",
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
    for asked in ["0", "half", "-1", ""] {
        for spelling in [
            vec![format!("--top={asked}")],
            vec!["--top".into(), asked.into()],
        ] {
            let listener = Listener::answering(|_| Canned::ok("{}"))?;
            let mut arguments = vec![
                "rank".to_owned(),
                QUESTION.to_owned(),
                "--jsonl".to_owned(),
                "--field".to_owned(),
                "/body".to_owned(),
            ];
            arguments.extend(spelling);
            arguments.extend(["--url".to_owned(), listener.base().to_owned()]);
            let borrowed = arguments.iter().map(String::as_str).collect::<Vec<_>>();
            let output = refused(&borrowed)?;
            assert_eq!(code(&output), 2, "--top {asked}");
            assert_eq!(
                said(&output),
                "thinkthen: `--top` prints the first N of the order, and N is a whole number of 1 or more\n",
                "--top {asked}"
            );
            assert_eq!(printed(&output), "", "--top {asked}");
            assert_eq!(listener.connections(), 0, "--top {asked}");
            assert!(listener.requests().is_empty(), "--top {asked}");
        }
    }

    for spelling in [["--top", "3"], ["--top", "-1"], ["--top=-1", ""]] {
        let listener = Listener::answering(|_| Canned::ok("{}"))?;
        let mut arguments = vec!["filter", QUESTION, "--jsonl"];
        arguments.push(spelling[0]);
        if !spelling[1].is_empty() {
            arguments.push(spelling[1]);
        }
        arguments.extend(["--url", listener.base()]);
        let output = refused(&arguments)?;
        assert_eq!(code(&output), 2);
        assert_eq!(
            said(&output),
            "thinkthen: `filter` keeps records and has no order to cut, so --top belongs to `rank`\n"
        );
        assert_eq!(listener.connections(), 0);
        assert!(listener.requests().is_empty());
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
    let key = KEY;
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
        vec![
            "rank", QUESTION, "--jsonl", "--field", "/body", "--top", "0",
        ],
        vec![
            "rank", QUESTION, "--jsonl", "--field", "/body", "--top", "half",
        ],
    ];
    let into = folder("refused-runs");
    fs::create_dir_all(&into)?;
    for arguments in cases {
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", key), ("RUST_BACKTRACE", "full")],
            RECORDS.as_bytes(),
        )?;
        assert_ne!(code(&output), 0, "{arguments:?} was not refused");
        // The reader `secrecy.rs` owns checks both channels and every file the
        // run wrote, so a refusal here is held to the sweep's whole claim.
        nothing_leaked(&format!("{arguments:?}"), &output, &into);
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
