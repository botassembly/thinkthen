//! The compiled binary over many records: the framing, the order, and the stop.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn_one as spawn};

mod record_rows;

/// The question every case on this page asks.
const QUESTION: &str = "Does this report a payment failure?";

/// Three JSON records, each with an id no request ever carries.
const RECORDS: &str = concat!(
    "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
    "{\"id\":\"R-2\",\"body\":\"Thanks for the quick fix.\"}\n",
    "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
);

/// The response a backend gives, with the probability the case names.
fn answered(probability: &str) -> String {
    format!(
        concat!(
            r#"{{"model":"jev-1.13.0","answers":{{"q1":{{"type":"noul","noul":{probability}}}}},"#,
            r#""usage":{{"input_tokens":88,"output_tokens":12}}}}"#,
        ),
        probability = probability
    )
}

/// A listener that answers each of these probabilities, one per request.
fn serving(probabilities: &[&str]) -> io::Result<Listener> {
    Listener::serving(
        probabilities
            .iter()
            .map(|p| Canned::ok(&answered(p)))
            .collect(),
    )
}

/// The bound that makes the listener's script line up with the input order.
///
/// This page pins the sequential form: one request out, one answer back, and
/// the listener answering in the order it was asked. `parallel.rs` pins what
/// several requests in flight do, and the default is four.
const ONE_AT_A_TIME: [&str; 2] = ["--jobs", "1"];

/// Run `decide` against one URL over the records on standard input.
fn decide(base: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = ["decide", QUESTION, "--url", base, "--model", "local-1"];
    spawn(
        &[&asked[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

/// Run `decide` with no address at all, which a plan needs none of.
fn planned(arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = ["decide", QUESTION, "--plan"];
    spawn(&[&asked[..], arguments].concat(), &[], input.as_bytes())
}

/// The record each request quoted in its question, as compact JSON, in the
/// order the listener read them. Every request sends the fixed state.
fn quoted(listener: &Listener) -> Vec<String> {
    let lead = "The text is ";
    let tail = format!(". {QUESTION}");
    listener
        .requests()
        .iter()
        .filter_map(|request| {
            let body: serde_json::Value = serde_json::from_slice(&request.body).ok()?;
            assert_eq!(
                body.get("state").and_then(serde_json::Value::as_str),
                Some("Each question quotes the text it asks about.")
            );
            let asked = body.pointer("/questions/q1/instructions")?.as_str()?;
            Some(
                asked
                    .strip_prefix(lead)?
                    .strip_suffix(tail.as_str())?
                    .to_owned(),
            )
        })
        .collect()
}

/// What the run printed on standard output.
fn printed(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn value_row(input: &str, value: &str) -> String {
    format!("{{\"input\":{input},\"value\":{value}}}\n")
}

fn record_values(values: &[bool]) -> String {
    RECORDS
        .lines()
        .zip(values)
        .map(|(record, value)| value_row(record, &value.to_string()))
        .collect()
}

/// What the run said on standard error.
fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A file under the test's own temporary folder, holding this text.
fn written(name: &str, text: &str) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::write(&path, text)?;
    Ok(path)
}

#[test]
fn one_pointer_sends_the_value_and_several_send_an_object_keyed_by_the_last_part() {
    let listener = serving(&["0.97"]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--jsonl", "--field", "/body", "--field", "/id"],
        "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        quoted(&listener),
        [r#"{"body":"The payout failed again.","id":"R-1"}"#]
    );
}

#[test]
fn a_record_whose_framing_and_pointers_cannot_act_together_sends_nothing() {
    let cases: [(&[&str], &str); 2] = [
        (&["--lines", "--field", "/body"], "has no members"),
        (
            &["--jsonl", "--field", "/a/text", "--field", "/b/text"],
            "two pointers end in `text`",
        ),
    ];

    for (arguments, said_part) in cases {
        let listener = serving(&["0.97"]).expect("a loopback listener");
        let output = decide(listener.base(), arguments, RECORDS).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(printed(&output).is_empty(), "{arguments:?}");
        assert!(said(&output).contains(said_part), "{}", said(&output));
        assert!(listener.requests().is_empty(), "{arguments:?}");
    }
}

#[test]
fn a_pointer_in_another_language_is_refused_by_name_before_any_request() {
    let cases = ["$.body", "#/id", "/*", "/a/*", "/list/-1"];

    for pointer in cases {
        let listener = serving(&["0.97"]).expect("a loopback listener");
        let output = decide(listener.base(), &["--jsonl", "--field", pointer], RECORDS)
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{pointer}");
        assert!(printed(&output).is_empty(), "{pointer}");
        let message = said(&output);
        assert!(message.contains("RFC 6901"), "{message}");
        assert!(
            message.contains(&format!("--field `{pointer}`")),
            "{message}"
        );
        assert!(listener.requests().is_empty(), "{pointer}");
    }
}

#[test]
fn a_record_the_tool_refuses_stops_the_run_and_sends_nothing_for_itself() {
    let good = "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n";
    let cases: [(&str, &str); 5] = [
        ("{\"id\":1,\"id\":2}\n", "each member name once"),
        ("{\"body\":1e999}\n", "`NaN` and `Infinity`"),
        ("not json\n", "not valid JSON"),
        ("{\"other\":\"x\"}\n", "holds nothing at `/body`"),
        ("{\"body\":\"   \"}\n", "the evidence is empty or blank"),
    ];

    for (bad, said_part) in cases {
        let listener = serving(&["0.97", "0.97"]).expect("a loopback listener");
        let input = format!("{good}{bad}");
        let output = decide(listener.base(), &["--jsonl", "--field", "/body"], &input)
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{bad}");
        assert_eq!(printed(&output), record_values(&[true]), "{bad}");
        assert_eq!(listener.requests().len(), 1, "{bad}");
        let message = said(&output);
        assert!(message.contains(said_part), "{message}");
        assert!(
            message.contains("stopped at record 2; 1 record finished"),
            "{message}"
        );
        assert!(!message.contains("payout"), "{message}");
        if bad == "not json\n" {
            assert_eq!(
                message,
                concat!(
                    "thinkthen: the record is not valid JSON\n",
                    "thinkthen: stopped at record 2; 1 record finished\n",
                )
            );
            assert!(!message.contains("not json"), "{message}");
        }
    }
}

#[test]
fn a_record_that_is_not_text_stops_the_run_at_exit_five() {
    let listener = serving(&["0.97", "0.97"]).expect("a loopback listener");
    let mut input = b"{\"body\":\"The payout failed again.\"}\n{\"body\":\"".to_vec();
    input.extend_from_slice(&[0xff, 0xfe]);
    input.extend_from_slice(b"\"}\n");
    let output = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--field",
            "/body",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        &input,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        printed(&output),
        value_row(r#"{"body":"The payout failed again."}"#, "true")
    );
    assert_eq!(listener.requests().len(), 1);
    let message = said(&output);
    assert!(message.contains("not valid UTF-8"), "{message}");
    assert!(
        message.contains("stopped at record 2; 1 record finished"),
        "{message}"
    );
}

#[test]
fn a_backend_failure_stops_the_run_and_the_rows_before_it_stay_printed() {
    let listener = Listener::serving(vec![
        Canned::ok(&answered("0.97")),
        Canned::status(500, "{}"),
    ])
    .expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--max-retries",
            "0",
            "--jobs",
            "1",
        ],
        RECORDS,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert_eq!(printed(&output), record_values(&[true]));
    let message = said(&output);
    assert!(message.contains("status 500"), "{message}");
    assert!(
        message.contains("stopped at record 2; 1 record finished"),
        "{message}"
    );
}

/// A stream whose last record has no line feed after it is judged like any other.
///
/// The reader ends the stream where it finds no line feed, so this case pins
/// that a file with no final newline still judges its last record.
#[test]
fn a_last_record_with_no_line_feed_after_it_is_judged() {
    let listener = serving(&["0.97", "0.97"]).expect("a loopback listener");

    let output = decide(listener.base(), &["--lines"], "first line\nsecond line")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        printed(&output),
        value_row(r#""first line""#, "true") + &value_row(r#""second line""#, "true")
    );
    let mut sent = quoted(&listener);
    sent.sort();
    assert_eq!(sent, [r#""first line""#, r#""second line""#]);
}

#[test]
fn an_empty_input_succeeds_with_no_output_and_no_request() {
    for arguments in [["--lines"], ["--jsonl"]] {
        let listener = serving(&["0.97"]).expect("a loopback listener");
        let output = decide(listener.base(), &arguments, "").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{arguments:?}");
        assert!(printed(&output).is_empty(), "{arguments:?}");
        assert!(said(&output).is_empty(), "{arguments:?}");
        assert!(listener.requests().is_empty(), "{arguments:?}");
    }
}

#[test]
fn no_records_answer_sets_the_exit_code() {
    let listener = serving(&["0.02", "0.02", "0.02"]).expect("a loopback listener");
    let output = decide(listener.base(), &["--jsonl", "--field", "/body"], RECORDS)
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(printed(&output), record_values(&[false, false, false]));
}

#[test]
fn quiet_over_records_is_a_usage_error_because_no_answer_reaches_the_exit_code() {
    for framing in ["--lines", "--jsonl"] {
        let listener = serving(&["0.97"]).expect("a loopback listener");
        let output =
            decide(listener.base(), &[framing, "--quiet"], RECORDS).expect("the binary runs");

        assert_eq!(output.status.code(), Some(2), "{framing}");
        assert!(said(&output).contains("--quiet"), "{}", said(&output));
        assert!(listener.requests().is_empty(), "{framing}");
    }
    // One document still carries its answer in the exit code, so it takes it.
    let listener = serving(&["0.02"]).expect("a loopback listener");
    let output =
        decide(listener.base(), &["--quiet"], "The payout failed again.").expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    assert!(printed(&output).is_empty());
}

#[test]
fn a_record_row_carries_the_whole_record_under_input() {
    let listener = serving(&["0.97"]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--jsonl", "--field", "/body", "--details"],
        "{\"id\":\"R-1\",\"body\":\"The payout failed again.\",\"seen\":false}\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let row = printed(&output);
    assert!(
        row.starts_with(concat!(
            r#"{"schema":"thinkthen.result/1","value":true,"#,
            r#""input":{"id":"R-1","body":"The payout failed again.","seen":false},"#,
            r#""question":{"verb":"decide","text":"Does this report a payment failure?"},"#,
        )),
        "{row}"
    );
    assert!(
        row.contains(&format!(
            r#""tool":"thinkthen {}""#,
            env!("CARGO_PKG_VERSION")
        )),
        "{row}"
    );
    // Only the pointed value left the machine.
    assert_eq!(quoted(&listener), [r#""The payout failed again.""#]);
}

#[test]
fn a_document_row_carries_no_input_because_one_document_is_no_stream() {
    let listener = serving(&["0.97"]).expect("a loopback listener");
    let output = decide(listener.base(), &["--details"], "The payout failed again.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert!(
        !printed(&output).contains(r#""input""#),
        "{}",
        printed(&output)
    );
}

#[test]
fn the_record_mode_plan_shows_the_first_record_and_names_the_framing() {
    let output =
        planned(&["--jsonl", "--field", "/body"], RECORDS).expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        printed(&output),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","input":{"framing":"jsonl","field":["/body"]},"#,
            r#""request":{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","#,
            r#""questions":{"q1":{"type":"noul","#,
            r#""instructions":"The text is \"The payout failed again.\". Does this report a payment failure?"}}}}"#,
            "\n",
            r#"{"records":3,"requests":3,"estimated_bytes":631,"estimated_input_tokens":{"lower":325,"upper":573},"upper_bound":false}"#,
            "\n",
        )
    );

    let output =
        planned(&["--lines"], "first line\nsecond line\n").expect("the compiled binary runs");
    assert!(
        printed(&output).contains(r#""input":{"framing":"lines","field":[]},"#),
        "{}",
        printed(&output)
    );
    // A plan over one document carries four fields and names no framing.
    let output = planned(&[], "one whole document").expect("the compiled binary runs");
    assert!(
        !printed(&output).contains(r#""input""#),
        "{}",
        printed(&output)
    );
}

#[test]
fn input_reads_the_records_from_the_file_it_names() {
    let path = written("streaming-records.jsonl", RECORDS).expect("a file of records");
    let listener = serving(&["0.97", "0.02", "0.80"]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--jobs",
            "1",
            "--input",
            &path.to_string_lossy(),
        ],
        "",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(printed(&output), record_values(&[true, false, true]));

    let missing = Path::new(env!("CARGO_TARGET_TMPDIR")).join("streaming-absent.jsonl");
    let output = decide(
        listener.base(),
        &["--jsonl", "--input", &missing.to_string_lossy()],
        "",
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(5));
    assert!(said(&output).contains("--input"), "{}", said(&output));
}

#[test]
fn a_pointer_without_jsonl_reads_the_whole_input_as_one_json_value() {
    let listener = serving(&["0.97"]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--field", "/a/text"],
        "{\n  \"a\": {\"text\": \"The payout failed again.\"}\n}\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(quoted(&listener), [r#""The payout failed again.""#]);
}

#[test]
fn choose_raw_prints_an_empty_line_for_an_unresolved_record() {
    const PICKED: &str = concat!(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
        r#""probabilities":{"billing":0.9,"shipping":0.1}}}}"#,
    );
    const CLOSE: &str = concat!(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
        r#""probabilities":{"billing":0.55,"shipping":0.45}}}}"#,
    );
    let listener =
        Listener::serving(vec![Canned::ok(PICKED), Canned::ok(CLOSE)]).expect("a listener");
    let output = spawn(
        &[
            "choose",
            "Which team owns this?",
            "billing",
            "shipping",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--threshold",
            "0.8",
            "--raw",
            "--lines",
            "--jobs",
            "1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first line\nsecond line\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(printed(&output), "billing\n\n");
}

#[test]
fn the_row_count_equals_the_record_count_for_every_run_that_finishes() {
    // The property the whole record surface rests on: one row per record, in
    // input order, whatever the view and whatever the answers.
    for count in 0..9_usize {
        let probabilities: Vec<&str> = (0..count)
            .map(|place| if place % 2 == 0 { "0.97" } else { "0.02" })
            .collect();
        let input: String = (0..count)
            .map(|place| format!("{{\"body\":\"record {place}\"}}\n"))
            .collect();

        for view in [
            &["--field", "/body"][..],
            &["--field", "/body", "--details"],
        ] {
            let listener = serving(&probabilities).expect("a loopback listener");
            let arguments = [&["--jsonl"][..], view].concat();
            let output =
                decide(listener.base(), &arguments, &input).expect("the compiled binary runs");

            assert_eq!(output.status.code(), Some(0), "{count} records, {view:?}");
            assert_eq!(
                printed(&output).lines().count(),
                count,
                "{count} records, {view:?}"
            );
        }
    }
}
