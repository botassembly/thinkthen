//! A saved threshold names the batch setting it was tuned under.

use std::fs;

use serde_json::json;

use super::{KEY, QUESTION, answering, details, folder, text};
use crate::harness::{Canned, Listener, spawn};

#[test]
fn a_tuned_file_warns_at_another_batch() {
    let place = folder("tuned-batch");
    fs::create_dir_all(&place).expect("scratch folder");
    let cases = [
        (
            "default",
            r#"{"threshold":0.7}"#,
            &[][..],
            Some(json!({"tuned_for":1,"running":"max"})),
            "thinkthen: warning: threshold tuned at batch 1 is running at batch max\n",
        ),
        (
            "matched",
            r#"{"threshold":0.7}"#,
            &["--batch", "1"][..],
            None,
            "",
        ),
        (
            "overridden",
            r#"{"threshold":0.7,"batch":10}"#,
            &["--batch", "1"][..],
            Some(json!({"tuned_for":10,"running":1})),
            "thinkthen: warning: threshold tuned at batch 10 is running at batch 1\n",
        ),
        (
            "untuned",
            r#"{"batch":10}"#,
            &["--batch", "1"][..],
            None,
            "",
        ),
    ];
    for (name, fields, extra, warning, expected_stderr) in cases {
        let file = format!("{place}/{name}.json");
        let tail = fields.trim_start_matches('{').trim_end_matches('}');
        fs::write(&file, format!(r#"{{"decide":"{QUESTION}",{tail}}}"#)).expect("question file");
        let listener = Listener::answering(answering).expect("listener");
        let question = format!("@{file}");
        let fixed = [
            "decide",
            question.as_str(),
            "--lines",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
        ];
        let output =
            spawn(&[&fixed[..], extra].concat(), &[KEY], b"line 1\nline 2\n").expect("command");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            text(&output.stderr)
        );
        assert_eq!(text(&output.stderr), expected_stderr, "{name}");
        assert_eq!(
            listener.count(),
            if name == "matched" || name == "overridden" || name == "untuned" {
                2
            } else {
                1
            },
            "{name}"
        );
        let rows = details(&output);
        assert_eq!(rows.len(), 2, "{name}: two detailed rows");
        for row in rows {
            match &warning {
                Some(value) => assert_eq!(&row["meta"]["batch_warning"], value, "{name}"),
                None => assert!(row["meta"].get("batch_warning").is_none(), "{name}"),
            }
        }
    }
}

#[test]
fn batch_warning_uses_the_successful_output_boundary() {
    let place = folder("warning-boundary");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/question.json");
    fs::write(
        &file,
        format!(r#"{{"decide":"{QUESTION}","threshold":0.99,"profile":"old"}}"#),
    )
    .expect("question file");
    let profile = format!("{place}/profile.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"new","max_questions":10}"#,
    )
    .expect("profile");
    let question = format!("@{file}");
    let listener = Listener::answering(answering).expect("listener");
    let output = spawn(
        &[
            "filter",
            &question,
            "--lines",
            "--no-cache",
            "--profile",
            &profile,
            "--url",
            listener.base(),
        ],
        &[KEY],
        b"line 1\nline 2\n",
    )
    .expect("filter run");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(output.stdout.is_empty(), "the filter kept nothing");
    assert_eq!(
        text(&output.stderr),
        "thinkthen: warning: threshold tuned for profile old is running under profile new\nthinkthen: warning: threshold tuned at batch 1 is running at batch max\n"
    );

    let one = spawn(
        &["decide", &question, "--no-cache", "--url", listener.base()],
        &[KEY],
        b"line 1",
    )
    .expect("one document");
    assert_eq!(one.status.code(), Some(1), "{}", text(&one.stderr));
    assert!(one.stderr.is_empty(), "one document has no batch warning");

    let structured = format!("{place}/structured.json");
    fs::write(
        &structured,
        r#"{"decide":{"rule":"names a place"},"threshold":0.7}"#,
    )
    .expect("structured question");
    let output = spawn(
        &[
            "decide",
            &format!("@{structured}"),
            "--lines",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        b"line 1\nline 2\n",
    )
    .expect("structured run");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(
        output.stderr.is_empty(),
        "a structured question runs at batch one"
    );
}

#[test]
fn a_first_failed_batch_has_no_calibration_warning() {
    let place = folder("warning-failure");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/question.json");
    fs::write(
        &file,
        format!(r#"{{"decide":"{QUESTION}","threshold":0.99}}"#),
    )
    .expect("question file");
    let question = format!("@{file}");
    let refusing = Listener::answering(|_| Canned::status(422, "{}")).expect("refusing listener");
    let failed = spawn(
        &[
            "decide",
            &question,
            "--lines",
            "--no-cache",
            "--max-retries",
            "0",
            "--url",
            refusing.base(),
        ],
        &[KEY],
        b"line 1\nline 2\n",
    )
    .expect("failed first batch");
    assert_eq!(failed.status.code(), Some(4));
    assert!(failed.stdout.is_empty());
    assert_eq!(refusing.count(), 1);
    assert!(
        !text(&failed.stderr).contains("warning: threshold tuned"),
        "a first failed result has no calibration warning"
    );
}
