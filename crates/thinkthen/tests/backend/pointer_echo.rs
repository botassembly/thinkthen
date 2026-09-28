//! A typed pointer holding a control character is refused, and every refusal
//! echoes the pointer with JSON escapes (ticket 0138).

use std::fs;
use std::path::Path;

use crate::harness::{Listener, spawn};

const PRINTABLE: &str = "a pointer is one line of printable text";

#[test]
fn a_pointer_holding_a_control_character_is_refused_and_never_echoed() {
    let file = Path::new(env!("CARGO_TARGET_TMPDIR")).join("pointer-echo-question.json");
    fs::write(&file, "{\"decide\":\"Is it late?\",\"on\":\"/a\\u001b\"}").expect("question file");
    let at_file = format!("@{}", file.display());
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");
    let cases: [(&[&str], String, i32); 4] = [
        (
            &[
                "decide",
                "Is it late?",
                "--jsonl",
                "--field",
                "/a\u{1b}[31m",
            ],
            format!("--field `/a\\u001b[31m`: {PRINTABLE}"),
            2,
        ),
        (
            &["find", "Which is late?", "--jsonl", "--field", "a\u{1b}"],
            format!("--field `a\\u001b`: {PRINTABLE}"),
            2,
        ),
        (
            &[
                "choose",
                "Which?",
                "--jsonl",
                "--field",
                "/a",
                "--options",
                "/o\u{1b}",
            ],
            format!("--options `/o\\u001b`: {PRINTABLE}"),
            2,
        ),
        (
            &["decide", &at_file, "--jsonl"],
            format!("the question file's `on` `/a\\u001b`: {PRINTABLE}"),
            5,
        ),
    ];
    for (arguments, said, code) in cases {
        let arguments = [arguments, &["--url", listener.base()]].concat();
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            b"{\"a\":\"x\"}\n",
        )
        .expect("the compiled binary runs");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("thinkthen: {said}\n"),
            "{arguments:?}"
        );
        assert_eq!(output.status.code(), Some(code), "{arguments:?}");
    }
    assert_eq!(listener.connections(), 0);
}
