//! Default record rows keep their parsed input beside each bare value.

use std::fs;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};

const KEY: [(&str, &str); 1] = [("THINKTHEN_API_KEY", "sk-test-value")];
const YES: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const CHOICE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"a","#,
    r#""probabilities":{"a":0.9,"b":0.1}}}}"#,
);
const TAG: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
    r#""q2":{"type":"noul","noul":0.1}}}"#,
);
const SCORE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"score","score":0.8,"#,
    r#""legend":{"0":"low","1":"high"},"probabilities":{"0":0.2,"1":0.8}}}}"#,
);

struct Framing<'a> {
    arguments: &'a [&'a str],
    input: &'a [u8],
    record: &'a str,
}

type VerbCase = (
    &'static str,
    &'static [&'static str],
    &'static str,
    &'static str,
);
type AnnotateCase = (&'static [&'static str], &'static [u8], &'static str);

const FRAMINGS: [Framing<'static>; 4] = [
    Framing {
        arguments: &["--lines"],
        input: b"hello\n",
        record: r#""hello""#,
    },
    Framing {
        arguments: &["--jsonl"],
        input: br#"{"id":1,"body":"hello"}
"#,
        record: r#"{"id":1,"body":"hello"}"#,
    },
    Framing {
        arguments: &["--csv"],
        input: b"id,body\n1,hello\n",
        record: r#"{"id":"1","body":"hello"}"#,
    },
    Framing {
        arguments: &["--tsv"],
        input: b"id\tbody\n1\thello\n",
        record: r#"{"id":"1","body":"hello"}"#,
    },
];

#[test]
fn four_value_verbs_wrap_each_record_under_input_and_value() {
    let verbs: [VerbCase; 4] = [
        (YES, &["decide", "Is it accepted?"], "true", "decide"),
        (CHOICE, &["choose", "Which?", "a", "b"], r#""a""#, "choose"),
        (TAG, &["tag", "Which?", "a", "b"], r#"["a"]"#, "tag"),
        (
            SCORE,
            &["score", "How much?", "low", "high"],
            "0.8",
            "score",
        ),
    ];
    for framing in &FRAMINGS {
        for (response, verb, value, name) in verbs {
            let listener = Listener::serving(vec![Canned::ok(response)]).expect("listener");
            let fixed = ["--url", listener.base(), "--model", "local-1"];
            let output = spawn(
                &[verb, framing.arguments, &fixed].concat(),
                &KEY,
                framing.input,
            )
            .expect("command runs");
            assert_eq!(
                output.status.code(),
                Some(0),
                "{name} {:?}",
                framing.arguments
            );
            assert_eq!(
                String::from_utf8_lossy(&output.stdout),
                format!("{{\"input\":{},\"value\":{value}}}\n", framing.record),
                "{name} {:?}",
                framing.arguments,
            );
        }
    }
}

fn question_set() -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("record-values-questions.json");
    let _written = fs::write(
        &path,
        r#"{"version":1,"questions":{"accepted":{"decide":"Is it accepted?"}}}"#,
    );
    path
}

#[test]
fn annotate_wraps_non_objects_in_record_mode_and_still_enriches_objects() {
    let set = question_set();
    let cases: [AnnotateCase; 5] = [
        (
            &["--lines"],
            b"hello\n",
            r#"{"input":"hello","value":{"accepted":true}}
"#,
        ),
        (
            &["--jsonl"],
            b"[1,2]\n",
            r#"{"input":[1,2],"value":{"accepted":true}}
"#,
        ),
        (
            &["--jsonl"],
            b"7\n",
            r#"{"input":7,"value":{"accepted":true}}
"#,
        ),
        (
            &["--csv"],
            b"id,body\n1,hello\n",
            r#"{"id":"1","body":"hello","accepted":true}
"#,
        ),
        (
            &["--tsv"],
            b"id\tbody\n1\thello\n",
            r#"{"id":"1","body":"hello","accepted":true}
"#,
        ),
    ];
    for (framing, input, expected) in cases {
        let listener = Listener::serving(vec![Canned::ok(YES)]).expect("listener");
        let set_name = set.to_string_lossy();
        let fixed = ["--url", listener.base(), "--model", "local-1"];
        let output = spawn(
            &[&["annotate", &set_name], framing, &fixed].concat(),
            &KEY,
            input,
        )
        .expect("annotate runs");
        assert_eq!(output.status.code(), Some(0), "{framing:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            expected,
            "{framing:?}"
        );
    }
}

#[test]
fn one_document_annotation_keeps_every_established_shape() {
    let set = question_set();
    let cases: [(&[u8], &str); 4] = [
        (
            b"plain text",
            r#"{"accepted":true}
"#,
        ),
        (
            b"7",
            r#"{"accepted":true}
"#,
        ),
        (
            b"[1,2]",
            r#"{"accepted":true}
"#,
        ),
        (
            br#"{"id":1,"body":"hello"}"#,
            r#"{"id":1,"body":"hello","accepted":true}
"#,
        ),
    ];
    for (input, expected) in cases {
        let listener = Listener::serving(vec![Canned::ok(YES)]).expect("listener");
        let set_name = set.to_string_lossy();
        let output = spawn(
            &[
                "annotate",
                &set_name,
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &KEY,
            input,
        )
        .expect("annotate runs");
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
    }
}
