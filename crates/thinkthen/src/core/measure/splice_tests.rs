#![allow(clippy::expect_used, reason = "a failed row stops the table")]

use super::{remove, splice};

#[test]
fn removing_a_batch_key_keeps_the_other_file_bytes() {
    let cases = [
        (
            r#"{"batch":"max","decide":"q"}"#,
            r#"{"decide":"q"}"#,
            r#""max""#,
        ),
        (
            r#"{"decide":"q","batch":10,"on":["/a"]}"#,
            r#"{"decide":"q","on":["/a"]}"#,
            "10",
        ),
        (
            "{\r\n  \"decide\": \"q\",\r\n  \"batch\": \"max\"\r\n}\r\n",
            "{\r\n  \"decide\": \"q\"\r\n}\r\n",
            r#""max""#,
        ),
        (r#"{"batch":1}"#, "{}", "1"),
    ];
    for (input, expected, old) in cases {
        assert_eq!(
            remove(input, &["batch"]),
            Some((expected.to_owned(), old.to_owned()))
        );
    }
    assert_eq!(remove(r#"{"decide":"q"}"#, &["batch"]), None);
}

/// Each row sets `threshold` to 0.5. `⟨OLD→NEW⟩` marks the bytes that change,
/// and an empty OLD marks an insert. A row that opens with `a/` or `b/` sets
/// `questions.a.threshold` or `questions.b.threshold` instead.
const ROWS: [&str; 13] = [
    r#"{"decide":"q","threshold":⟨0.9→0.5⟩}"#,
    "{\n  \"decide\": \"q\",\n  \"threshold\": ⟨1→0.5⟩\n}\n",
    r#"{"decide": "q", "threshold": ⟨"0.3:0.7"→0.5⟩, "on": ["/a"]}"#,
    r#"{"decide": "q", "threshold" : ⟨null→0.5⟩}"#,
    r#"{"decide": "q"⟨→, "threshold": 0.5⟩}"#,
    r#"{"decide":"q","on":["/a"]⟨→,"threshold":0.5⟩}"#,
    "{\r\n\t\"decide\" :  \"q\" ,\r\n\t\"on\" :  [\"/a\"]⟨→,\r\n\t\"threshold\" :  0.5⟩\r\n}",
    r#"{"decide": "a \"threshold\": 1, \\", "threshold": ⟨0.9→0.5⟩}"#,
    r#"{"questions": {"a": {"decide": "q", "threshold": 0.1}}, "threshold": ⟨0.9→0.5⟩}"#,
    r#"{"decide": "café ☕", "threshold": ⟨0.9→0.5⟩}"#,
    r#"b/{"threshold": 0.95, "questions": {"a": {"threshold": 0.1}, "b": {"choose": "q"⟨→, "threshold": 0.5⟩}}}"#,
    r#"a/{"threshold": 0.95, "questions": {"a": {"decide": "q", "threshold": ⟨0.1→0.5⟩}}}"#,
    "{\"decide\": \"q\"⟨→, \"threshold\": 0.5⟩}  \n\t ",
];

#[test]
fn splice_changes_one_value_and_keeps_every_other_byte() {
    for row in ROWS {
        let (name, row) = match row.split_once('/').filter(|(name, _)| name.len() == 1) {
            Some((name, rest)) => (Some(name), rest),
            None => (None, row),
        };
        let (head, rest) = row.split_once('⟨').expect("a marked change");
        let (change, tail) = rest.split_once('⟩').expect("a closed change");
        let (old, new) = change.split_once('→').expect("an arrow");
        let path = name.map_or(vec!["threshold"], |name| {
            vec!["questions", name, "threshold"]
        });
        let spliced = splice(&format!("{head}{old}{tail}"), &path, "0.5");
        let held = (!old.is_empty()).then(|| old.to_owned());
        assert_eq!(spliced, Some((format!("{head}{new}{tail}"), held)), "{row}");
    }
    assert_eq!(
        splice(r#"{"decide": "q"}"#, &["questions", "a", "threshold"], "1"),
        None
    );
}
