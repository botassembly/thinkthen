//! `audit` and `diff` read the batch setting the command writes in each
//! detailed row, by ADR 0085 items 2 and 3 (Debt 008, ticket 0349).

use std::fs;

use serde_json::{Value, json};

use super::{KEY, QUESTION, answering, details, folder, lines, text};
use crate::harness::{Listener, spawn};
use crate::measure_support::measure;

/// Detailed `decide` rows for `line FIRST` to `line FIRST + 3` at one batch
/// setting, saved to `NAME.jsonl`, and the requests the loopback counted.
fn saved(
    place: &str,
    name: &str,
    question: &str,
    (first, setting): (usize, &str),
) -> (String, usize, Vec<Value>) {
    let listener = Listener::answering(answering).expect("listener");
    let arguments = [
        "decide",
        question,
        "--lines",
        "--details",
        "--no-cache",
        "--batch",
        setting,
        "--url",
        listener.base(),
    ];
    let output = spawn(&arguments, &[KEY], lines(first..=first + 3).as_bytes()).expect("command");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let path = format!("{place}/{name}.jsonl");
    fs::write(&path, &output.stdout).expect("saved results");
    (path, listener.count(), details(&output))
}

#[test]
fn audit_and_diff_read_the_setting_the_command_writes() {
    let place = folder("audited");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/question.json");
    fs::write(
        &file,
        format!(r#"{{"decide":"{QUESTION}","threshold":0.7}}"#),
    )
    .expect("question file");
    let question = format!("@{file}");
    let (one, one_sent, one_rows) = saved(&place, "one", &question, (1, "1"));
    let (max, _, max_rows) = saved(&place, "max", &question, (1, "max"));
    // `audit` refuses a record twice, so the mixed results ask other lines.
    let (later, _, _) = saved(&place, "later", &question, (5, "max"));
    assert_eq!(one_sent, 4, "one record a request");
    for (rows, setting) in [(&one_rows, json!(1)), (&max_rows, json!("max"))] {
        assert_eq!(rows.len(), 4);
        for row in rows {
            assert_eq!(row["meta"]["batch_setting"], setting, "{row}");
            assert!(
                row["meta"].get("batch").is_none(),
                "no batch receipt: {row}"
            );
        }
    }
    let digests = |rows: &[Value]| -> Vec<Value> {
        rows.iter()
            .map(|row| row["meta"]["question_sha256"].clone())
            .collect()
    };
    assert_eq!(
        digests(&one_rows),
        digests(&max_rows),
        "the setting keeps the digest"
    );

    let key = format!("{place}/key.jsonl");
    let labels: String = (1..=8)
        .map(|at| format!("{{\"id\":\"line {at}\",\"value\":{}}}\n", at % 2 == 0))
        .collect();
    fs::write(&key, labels).expect("key");
    let both = format!("{place}/both.jsonl");
    fs::write(
        &both,
        [
            fs::read(&one).expect("one"),
            fs::read(&later).expect("later"),
        ]
        .concat(),
    )
    .expect("both runs");
    let audit = |results: &str, extra: &[&str]| {
        measure(
            &[&["audit", results, &key, "--id", ""][..], extra].concat(),
            b"",
        )
    };
    // Row: the results, and the warning audit prints first. One setting prints none.
    for (results, warned) in [
        (&one, ""),
        (&max, ""),
        (
            &both,
            "thinkthen: audit: warning: the results ran at more than one batch setting (1 and max); a bar tuned over both may fit neither\n",
        ),
    ] {
        let (code, _, stderr) = audit(results, &[]);
        assert_eq!((code, stderr.as_str()), (0, warned), "{results}");
    }

    // `audit --write` over one-a-request rows removes a tuned `max`.
    let tuned = format!("{place}/tuned.json");
    let tuned_text = format!(r#"{{"decide":"{QUESTION}","threshold":0.7,"batch":"max"}}"#);
    fs::write(&tuned, &tuned_text).expect("tuned file");
    let (code, _, stderr) = audit(&one, &["--write", &tuned]);
    assert_eq!(code, 0, "{stderr}");
    assert!(
        stderr.ends_with(
            "thinkthen: audit: removed batch max for the question; the results ran one record a request\n"
        ),
        "{stderr}"
    );
    assert!(!fs::read_to_string(&tuned).expect("tuned").contains("batch"));
    // Over both runs it keeps the setting and says why.
    fs::write(&tuned, &tuned_text).expect("tuned file");
    let (code, _, stderr) = audit(&both, &["--write", &tuned]);
    assert_eq!(code, 0, "{stderr}");
    assert!(
        stderr.contains(
            "thinkthen: audit: kept the batch setting for the question; the results ran at more than one batch setting\n"
        ),
        "{stderr}"
    );
    assert!(
        fs::read_to_string(&tuned)
            .expect("tuned")
            .contains(r#""batch":"max""#)
    );

    // Row: the two runs, and the diff warning. Two cuts on one run print none.
    for (a, b, warned) in [
        (
            &one,
            &max,
            "thinkthen: diff: warning: the runs used different batch settings (1 and max); batching moves answers, so some changes may come from it\n",
        ),
        (&max, &max, ""),
    ] {
        let (code, _, stderr) = measure(&["diff", a, b, "--id", ""], b"");
        assert_eq!((code, stderr.as_str()), (0, warned), "{a} {b}");
    }
}
