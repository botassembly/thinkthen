//! Saved case evidence uses audit's grade and retains its aggregate view.
#![cfg(feature = "cli")]
#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use crate::child::ChildEnvironment as _;
use crate::measure_support;
use crate::wait;

use std::process::{Command, Stdio};

use conformance_backend::{Canned, Listener};
use measure_support::{audit, fixture, fixtures, measure};
use serde_json::{Value, json};

const RESULTS: &str = "cases/results.jsonl";
const KEY: &str = "cases/key.jsonl";

fn rows(text: &str) -> Vec<Value> {
    text.lines()
        .map(|line| serde_json::from_str(line).expect("JSON row"))
        .collect()
}

fn group<'a>(rows: &'a [Value], name: &str) -> &'a Value {
    rows.iter()
        .find(|row| row["group"] == name)
        .expect("aggregate group")
}

#[test]
fn literal_case_rows_keep_every_carrier_and_match_aggregate_directions() {
    let (code, cases, stderr) = audit(&[RESULTS, KEY, "--cases"], b"");
    assert_eq!((code, stderr.as_str()), (0, ""));
    assert_eq!(cases, fixture("cases/expected.jsonl"));

    let (code, aggregate, stderr) = audit(&[RESULTS, KEY], b"");
    assert_eq!((code, stderr.as_str()), (0, ""));
    let groups = rows(&aggregate);
    let decide = group(&groups, "Is it relevant?");
    assert_eq!(
        [
            &decide["rows"],
            &decide["labeled"],
            &decide["unlabeled"],
            &decide["right"],
            &decide["wrong"],
            &decide["unsure"],
            &decide["true_yes"],
            &decide["false_yes"],
            &decide["true_no"],
            &decide["false_no"],
        ],
        [
            &json!(6),
            &json!(5),
            &json!(1),
            &json!(2),
            &json!(2),
            &json!(1),
            &json!(1),
            &json!(1),
            &json!(1),
            &json!(1),
        ]
    );
    let tag = group(&groups, "Which labels?");
    assert_eq!(
        [
            &tag["rows"],
            &tag["wrong"],
            &tag["false_yes"],
            &tag["false_no"]
        ],
        [&json!(2), &json!(2), &json!(1), &json!(1)]
    );
    assert_eq!(group(&groups, "Which labels?/first")["false_yes"], 1);
    assert_eq!(group(&groups, "Which labels?/second")["false_no"], 1);
    assert_eq!(group(&groups, "urgent")["failed"], 1);
    assert_eq!(group(&groups, "kind")["right"], 1);
    let set = group(&groups, "recognize");
    assert_eq!(
        [&set["true_yes"], &set["false_yes"], &set["false_no"]],
        [&json!(1), &json!(1), &json!(1)]
    );
}

#[test]
fn duplicate_identity_refuses_while_missing_input_keeps_line_identity() {
    let source = fixture(RESULTS);
    let duplicate = format!("{source}{}\n", source.lines().next().expect("first line"));
    let (code, stdout, stderr) = audit(&["-", KEY, "--cases"], duplicate.as_bytes());
    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (
            2,
            "",
            "thinkthen: audit: results line 12 repeats a record for one question\n"
        )
    );

    let missing = "{\"value\":{\"entities\":[]},\"question\":{\"verb\":\"recognize\",\"kinds\":{\"X\":\"Kind X\"},\"threshold\":0.3}}\n";
    let (code, stdout, stderr) = audit(
        &["-", KEY, "--cases"],
        format!("{source}{missing}").as_bytes(),
    );
    assert_eq!((code, stderr.as_str()), (0, ""));
    let printed = rows(&stdout);
    let last = printed.last().expect("last case");
    assert_eq!(
        last,
        &json!({
            "case": true,
            "line": 12,
            "id": "12",
            "name": null,
            "question": null,
            "label": null,
            "verb": "recognize",
            "input": null,
            "options": null,
            "said": null,
            "truth": null,
            "outcome": "unlabeled",
            "probability": null,
            "probabilities": null,
            "top_two": null,
            "item_counts": null,
            "usage": null,
            "usage_scope": null
        })
    );
    assert!(last.get("key_value").is_none());
}

#[test]
fn case_rules_use_saved_probabilities_and_the_existing_set_matcher() {
    let (code, stdout, stderr) = audit(
        &[
            "small/decide.jsonl",
            "small/decide-key.jsonl",
            "--cases",
            "--threshold",
            "0.4",
        ],
        b"",
    );
    assert_eq!((code, stderr.as_str()), (0, ""));
    let decided = rows(&stdout);
    let third = decided.iter().find(|row| row["id"] == "r3").expect("r3");
    assert_eq!(
        (&third["said"], &third["truth"], &third["outcome"]),
        (&json!(true), &json!(true), &json!("right"))
    );

    let files = ["sets/names.jsonl", "sets/names-key.jsonl", "--cases"];
    for (matching, expected) in [("strict", (2, 8, 10)), ("overlap", (6, 4, 6))] {
        let args = [&files[..], &["--match", matching]].concat();
        let (code, stdout, stderr) = audit(&args, b"");
        assert_eq!((code, stderr.as_str()), (0, ""));
        let counts = rows(&stdout).into_iter().fold((0, 0, 0), |sum, row| {
            let count = &row["item_counts"];
            (
                sum.0 + count["matched"].as_u64().expect("matched"),
                sum.1 + count["extra"].as_u64().expect("extra"),
                sum.2 + count["missed"].as_u64().expect("missed"),
            )
        });
        assert_eq!(counts, expected, "{matching}");
    }
}

#[test]
fn aggregate_only_flags_refuse_cases_before_file_reads() {
    let flags: &[&[&str]] = &[
        &["--by", "verb"],
        &["--seed", "7"],
        &["--target", "1"],
        &["--optimize", "precision"],
        &["--curve"],
        &["--pooled"],
        &["--table"],
        &["--write", "missing.json"],
        &["--write-to", "missing.json"],
    ];
    for flag in flags {
        let mut arguments = vec!["missing-results.jsonl", "missing-key.jsonl", "--cases"];
        arguments.extend_from_slice(flag);
        let (code, stdout, stderr) = audit(&arguments, b"");
        assert_eq!((code, stdout.as_str()), (2, ""), "{flag:?}");
        assert!(stderr.contains("cannot be used with"), "{flag:?}: {stderr}");
        assert!(!stderr.contains("cannot read"), "{flag:?}: {stderr}");
    }
}

#[test]
fn case_view_sends_nothing_with_a_configured_backend_and_key() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["audit", RESULTS, KEY, "--cases"])
        .current_dir(fixtures())
        .clear_environment()
        .env("THINKTHEN_BASE_URL", listener.base())
        .env("THINKTHEN_API_KEY", "canary-0257")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("audit child");
    let output = wait::finish(child, "audit cases").expect("audit finishes");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.count(), 0);
    assert_eq!(output.stdout, fixture("cases/expected.jsonl").as_bytes());
    assert!(output.stderr.is_empty());
    assert!(!output.stdout.windows(11).any(|part| part == b"canary-0257"));
}

#[test]
fn saved_decision_meanings_grade_the_run_verdict_and_keep_explicit_cuts() {
    let base: Value = serde_json::from_str(
        fixture("small/decide.jsonl")
            .lines()
            .next()
            .expect("first decision"),
    )
    .expect("decision row");
    for meaning in [
        json!("authored yes"),
        Value::Null,
        json!(false),
        json!({"decision":"yes"}),
        json!({"failed":false}),
        json!({"failed":{"kind":"backend","cause":"missing_answer"}}),
        json!(["yes"]),
    ] {
        for (probability, threshold, expected) in [
            (0.9, json!(0.5), [1, 0, 0]),
            (0.7, json!(0.8), [0, 1, 0]),
            (0.7, json!("0.7:0.9"), [0, 0, 1]),
            (0.9, json!("0.7:0.9"), [1, 0, 0]),
            (0.6, json!("0.7:0.9"), [0, 1, 0]),
        ] {
            let mut saved = base.clone();
            saved["schema"] = json!("thinkthen.result/2");
            saved["value"] = meaning.clone();
            saved["threshold"] = threshold;
            saved["answer"]["probability"] = json!(probability);
            let input = serde_json::to_vec(&saved).expect("saved row");
            for (arguments, grade) in [
                (vec!["-", "small/decide-key.jsonl"], expected),
                (
                    vec!["-", "small/decide-key.jsonl", "--threshold", "0.5"],
                    [1, 0, 0],
                ),
            ] {
                let (code, stdout, stderr) = audit(&arguments, &input);
                assert_eq!((code, stderr.as_str()), (0, ""));
                let report = rows(&stdout).remove(0);
                assert_eq!(report["failed"], 0, "{saved}");
                assert_eq!(
                    [
                        report["right"].as_u64(),
                        report["wrong"].as_u64(),
                        report["unsure"].as_u64()
                    ],
                    grade.map(Some),
                    "{saved}"
                );
            }
            let (code, stdout, stderr) =
                measure(&["runs", "diff", "-", "small/decide.jsonl"], &input);
            assert_eq!((code, stderr.as_str()), (0, ""));
            let reports = rows(&stdout);
            let summary = &reports.last().expect("diff summary")["summary"];
            assert_eq!(summary["records"], 1, "{saved}");
            assert_eq!(
                summary["changed"],
                u64::from(expected != [1, 0, 0]),
                "{saved}"
            );
        }
    }
}

#[test]
fn saved_decisions_without_probability_refuse_without_echoing_content() {
    for probability in [None, Some(Value::Null)] {
        let mut saved = json!({
            "schema":"thinkthen.result/2", "input":{"id":"secret-id","body":"secret-body"},
            "question":{"verb":"decide","text":"secret-question"},
            "value":"secret-value",
            "answer":{"kind":"yes_no"}, "threshold":0.5,
        });
        if let Some(probability) = probability {
            saved["answer"]["probability"] = probability;
        }
        let input = serde_json::to_vec(&saved).expect("saved decision");
        for (command, location) in [
            (
                vec!["runs", "audit", "-", "small/decide-key.jsonl"],
                "results",
            ),
            (vec!["runs", "diff", "-", "small/decide.jsonl"], "first run"),
            (
                vec!["runs", "diff", "small/decide.jsonl", "-"],
                "second run",
            ),
        ] {
            let (code, stdout, stderr) = measure(&command, &input);
            assert_eq!(code, 2);
            assert!(stdout.is_empty());
            assert_eq!(
                stderr,
                format!(
                    "thinkthen: {}: {location} line 1 holds a probability outside 0 to 1 or an empty distribution\n",
                    command[1],
                )
            );
        }
    }
}

#[test]
fn actual_saved_failures_and_legacy_value_markers_remain_failed() {
    let base: Value = serde_json::from_str(
        fixture("small/decide.jsonl")
            .lines()
            .next()
            .expect("decision"),
    )
    .expect("saved row");
    let failure = json!({"kind":"backend","cause":"missing_answer"});
    for route in ["explicit", "bare", "legacy"] {
        let mut saved = base.clone();
        if route != "legacy" {
            saved["schema"] = json!("thinkthen.result/2");
        }
        if route == "explicit" {
            saved["failure"] = failure.clone();
        } else {
            saved["value"] = json!({"failed":failure});
        }
        if route == "bare" {
            saved.as_object_mut().expect("saved row").remove("answer");
        }
        let input = serde_json::to_vec(&saved).expect("saved failure");
        let (code, stdout, stderr) = audit(&["-", "small/decide-key.jsonl"], &input);
        assert_eq!((code, stderr.as_str()), (0, ""));
        assert_eq!(rows(&stdout)[0]["failed"], 1, "{route}");
        let (code, stdout, stderr) = measure(&["runs", "diff", "-", "small/decide.jsonl"], &input);
        assert_eq!(code, 0);
        assert_eq!(
            stderr,
            "thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names\n"
        );
        assert_eq!(
            rows(&stdout).last().expect("summary")["summary"]["records"],
            0,
            "{route}"
        );
    }
}
