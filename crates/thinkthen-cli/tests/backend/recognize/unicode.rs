//! Scalar spans through the real command preserve original Unicode spelling.
use super::{automatic, local, stdout};
use crate::harness::Listener;
use serde_json::{Value, json};

#[test]
fn leading_zero_width_prefixes_leave_names_at_original_scalar_positions() {
    let cases = [
        ("\u{feff}Ada met Acme.", "Ada", 1, 4, 9),
        ("\u{200b}Ada met Acme.", "Ada", 1, 4, 9),
        ("\u{200c}Ada met Acme.", "Ada", 1, 4, 9),
        ("\u{200d}Ada met Acme.", "Ada", 1, 4, 9),
        ("\u{2060}Ada met Acme.", "Ada", 1, 4, 9),
        (
            "\u{feff}\u{200b}\u{200d}A\u{301}da met Acme.",
            "A\u{301}da",
            3,
            7,
            12,
        ),
        (
            "A\u{200c}d\u{200d}a met Acme.",
            "A\u{200c}d\u{200d}a",
            0,
            5,
            10,
        ),
    ];
    for (text, name, start, end, later) in cases {
        let listener = Listener::answering(automatic).expect("listener");
        let output = local(
            &listener,
            &["person", "organization", "--no-cache", "--jsonl"],
            Some("fake"),
            json!(text).to_string().as_bytes(),
        );
        assert_eq!(output.status.code(), Some(0));
        let row: Value = serde_json::from_str(&stdout(&output)).expect("result");
        assert_eq!(row["input"], text);
        let value = &row["value"];
        assert_eq!(
            value,
            &json!({"entities":[
                {"text":name,"start":start,"end":end,"length":end-start,"kind":"person","strength":0.9},
                {"text":"Acme","start":later,"end":later+4,"length":4,"kind":"organization","strength":0.9}
            ]}),
            "{text:?}"
        );
        assert_eq!(listener.requests().len(), 2);
    }
}

#[test]
fn other_leading_format_characters_keep_their_piece_behavior() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = local(
        &listener,
        &["person", "organization", "--no-cache"],
        Some("fake"),
        "\u{200e}Ada met Acme.".as_bytes(),
    );
    assert_eq!(output.status.code(), Some(0));
    let value: Value = serde_json::from_str(&stdout(&output)).expect("result");
    assert_eq!(
        value,
        json!({"entities":[{"text":"Acme","start":9,"end":13,"length":4,"kind":"organization","strength":0.9}]})
    );
    assert_eq!(listener.requests().len(), 2);
}
