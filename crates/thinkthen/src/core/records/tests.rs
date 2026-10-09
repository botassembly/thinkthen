//! Every framing, every pointer, and every refusal one record can reach.

use super::{Framing, Held, Reading, ReadingError, Record, RecordError};
use crate::core::json::{Json, JsonError};
use crate::core::pointer::Pointer;
use crate::core::question::LabelsError;
use crate::core::render::json_line;
use crate::core::text::{BlankTextError, Description};
use proptest::collection::vec;
use proptest::prelude::{Strategy, any};
use proptest::{prop_assert_eq, proptest};

fn reading(framing: Framing, fields: &[&str]) -> Reading {
    let pointers = fields
        .iter()
        .map(|text| Pointer::new(*text).expect("a pointer"))
        .collect();
    Reading::new(framing, pointers).expect("a framing and its pointers")
}

/// The evidence one record sends under one reading.
fn sent(reading: &Reading, bytes: &[u8]) -> Result<String, RecordError> {
    let record = reading.record(bytes)?;
    Ok(reading.evidence(&record)?.as_text()?.into_owned())
}

/// The `state` value one record sends under one reading.
fn state(reading: &Reading, bytes: &[u8]) -> Result<Json, RecordError> {
    let record = reading.record(bytes)?;
    reading.evidence(&record).map(|evidence| evidence.as_json())
}

#[test]
fn each_framing_says_what_one_record_is() {
    let document = reading(Framing::Document, &[]);
    assert_eq!(
        sent(&document, b"two\nlines\n").as_deref(),
        Ok("two\nlines\n")
    );
    let lines = reading(Framing::Lines, &[]);
    assert_eq!(sent(&lines, b"one line\n").as_deref(), Ok("one line"));
    assert_eq!(sent(&lines, b"no line feed").as_deref(), Ok("no line feed"));
    let jsonl = reading(Framing::Jsonl, &[]);
    assert_eq!(
        sent(&jsonl, br#"{"id":"T-91","body":"Payouts failed."}"#).as_deref(),
        Ok(r#"{"id":"T-91","body":"Payouts failed."}"#)
    );
}

#[test]
fn a_carriage_return_before_the_line_feed_is_stripped() {
    let lines = reading(Framing::Lines, &[]);
    assert_eq!(sent(&lines, b"one line\r\n").as_deref(), Ok("one line"));
    assert_eq!(
        sent(&lines, b"kept\rinside\n").as_deref(),
        Ok("kept\rinside")
    );
    let jsonl = reading(Framing::Jsonl, &["/a"]);
    assert_eq!(sent(&jsonl, b"{\"a\":\"x\"}\r\n").as_deref(), Ok("x"));
}

#[test]
fn one_pointer_sends_the_value_it_names_and_several_send_an_object() {
    let one = reading(Framing::Jsonl, &["/body"]);
    let line = br#"{"id":"T-91","body":"Payouts failed.","count":3}"#;
    assert_eq!(sent(&one, line).as_deref(), Ok("Payouts failed."));
    let number = reading(Framing::Jsonl, &["/count"]);
    assert_eq!(sent(&number, line).as_deref(), Ok("3"));
    let several = reading(Framing::Jsonl, &["/body", "/id"]);
    assert_eq!(
        sent(&several, line).as_deref(),
        Ok(r#"{"body":"Payouts failed.","id":"T-91"}"#)
    );
}

#[test]
fn a_pointer_without_jsonl_reads_the_whole_input_as_one_json_value() {
    let document = reading(Framing::Document, &["/a/text"]);
    assert_eq!(
        sent(&document, b"{\n  \"a\": {\"text\": \"inner\"}\n}\n").as_deref(),
        Ok("inner")
    );
}

/// A pointer selects, so the selected value decides the state's type: an
/// object or a list travels as that JSON value, a string as its text, and a
/// number, `true`, `false`, or `null` as its compact spelling in a string.
#[test]
fn a_pointer_to_an_object_or_a_list_sends_the_json_value_it_named() {
    let line = br#"{"meta":{"a":1,"b":[true]},"items":[1,"x"],"body":"Payouts failed."}"#;
    let object = reading(Framing::Jsonl, &["/meta"]);
    assert_eq!(
        state(&object, line),
        Ok(Json::parse(r#"{"a":1,"b":[true]}"#).expect("JSON"))
    );
    let list = reading(Framing::Jsonl, &["/items"]);
    assert_eq!(
        state(&list, line),
        Ok(Json::parse(r#"[1,"x"]"#).expect("JSON"))
    );
}

#[test]
fn several_pointers_send_one_object_in_the_order_the_pointers_were_given() {
    let line = br#"{"id":"T-91","body":"Payouts failed.","meta":{"a":1}}"#;
    let several = reading(Framing::Jsonl, &["/body", "/id", "/meta"]);
    assert_eq!(
        state(&several, line),
        Ok(Json::parse(r#"{"body":"Payouts failed.","id":"T-91","meta":{"a":1}}"#).expect("JSON"))
    );
}

#[test]
fn a_string_or_a_scalar_stays_the_string_state_it_always_sent() {
    let line = br#"{"body":"Payouts failed.","count":3,"ok":false,"none":null}"#;
    for (field, expected) in [
        ("/body", "Payouts failed."),
        ("/count", "3"),
        ("/ok", "false"),
        ("/none", "null"),
    ] {
        let one = reading(Framing::Jsonl, &[field]);
        assert_eq!(
            state(&one, line).as_ref().map(Json::as_str),
            Ok(Some(expected)),
            "{field}"
        );
    }
}

#[test]
fn the_root_pointer_selects_the_whole_record_as_the_value_it_is() {
    let root = reading(Framing::Jsonl, &[""]);
    let line = br#"{"id":"T-91","body":"Payouts failed."}"#;
    assert_eq!(
        state(&root, line),
        Ok(Json::parse(r#"{"id":"T-91","body":"Payouts failed."}"#).expect("JSON"))
    );
}

#[test]
fn no_pointer_keeps_the_whole_record_as_text_even_when_it_is_json() {
    let jsonl = reading(Framing::Jsonl, &[]);
    let line = br#"{"id":"T-91","body":"Payouts failed."}"#;
    assert_eq!(
        state(&jsonl, line).as_ref().map(Json::as_str),
        Ok(Some(r#"{"id":"T-91","body":"Payouts failed."}"#))
    );
    let document = reading(Framing::Document, &[]);
    assert_eq!(
        state(&document, line).as_ref().map(Json::as_str),
        Ok(Some(r#"{"id":"T-91","body":"Payouts failed."}"#))
    );
}

#[test]
fn an_empty_object_or_list_is_evidence_but_a_blank_string_is_not() {
    let line = br#"{"empty":{},"bare":[],"blank":"  "}"#;
    for (field, expected) in [("/empty", "{}"), ("/bare", "[]")] {
        let one = reading(Framing::Jsonl, &[field]);
        assert_eq!(
            state(&one, line),
            Ok(Json::parse(expected).expect("JSON")),
            "{field}"
        );
    }
    let blank = reading(Framing::Jsonl, &["/blank"]);
    assert_eq!(
        state(&blank, line).unwrap_err(),
        RecordError::Blank(BlankTextError::Evidence)
    );
}

#[test]
fn the_evidence_debug_withholds_the_text_and_the_json_it_holds() {
    let jsonl = reading(Framing::Jsonl, &["/meta"]);
    let record = jsonl
        .record(br#"{"meta":{"secret":"marker-7b3ac5"}}"#)
        .expect("a record");
    let evidence = jsonl.evidence(&record).expect("structured evidence");
    let shown = format!("{evidence:?}");
    assert!(!shown.contains("marker-7b3ac5"), "{shown}");
    assert!(!shown.contains("secret"), "{shown}");
    let lines = reading(Framing::Lines, &[]);
    let record = lines.record(b"text marker-7b3ac5\n").expect("a record");
    let evidence = lines.evidence(&record).expect("text evidence");
    assert!(!format!("{evidence:?}").contains("marker-7b3ac5"));
}

#[test]
fn the_framing_and_the_pointers_are_refused_when_they_cannot_act_together() {
    let pointer = |text: &str| Pointer::new(text).expect("a pointer");
    assert_eq!(
        Reading::new(Framing::Lines, vec![pointer("/body")]),
        Err(ReadingError::TextHasNoMembers)
    );
    assert_eq!(
        Reading::new(Framing::Jsonl, vec![pointer("/a/text"), pointer("/b/text")]),
        Err(ReadingError::KeyClash("text".to_owned()))
    );
    assert!(Reading::new(Framing::Jsonl, vec![pointer("/a"), pointer("/b")]).is_ok());
}

#[test]
fn a_record_the_tool_refuses_names_no_part_of_itself() {
    let jsonl = reading(Framing::Jsonl, &["/body"]);
    let cases = [
        (
            &b"{\"id\":1,\"id\":2}"[..],
            RecordError::Json(JsonError::DuplicateName {
                path: "id".to_owned(),
            }),
        ),
        (
            &b"{\"body\":1e999}"[..],
            RecordError::Json(JsonError::NotFinite),
        ),
        (
            &b"not json"[..],
            RecordError::Json(JsonError::Syntax { line: 1, column: 2 }),
        ),
        (&b"\xff\xfe"[..], RecordError::NotUtf8),
        (
            &b"{\"other\":\"x\"}"[..],
            RecordError::Missed("/body".to_owned()),
        ),
        (
            &b"{\"body\":\"  \"}"[..],
            RecordError::Blank(BlankTextError::Evidence),
        ),
    ];
    for (bytes, expected) in cases {
        let refused = sent(&jsonl, bytes).expect_err("a refused record");
        assert_eq!(refused, expected);
        let said = refused.to_string();
        assert!(!said.contains("Payouts"), "{said}");
        assert!(!said.contains("other"), "{said}");
    }
}

#[test]
fn syntax_names_a_whole_input_but_keeps_jsonl_record_wording() {
    let syntax = JsonError::Syntax {
        line: 1,
        column: 10,
    };
    assert_eq!(
        reading(Framing::Document, &["/a"]).record(b"{\"a\":\"x\",}\n"),
        Err(RecordError::InputJson {
            line: 1,
            column: 10,
        })
    );
    assert_eq!(
        reading(Framing::Jsonl, &["/a"]).record(b"{\"a\":\"x\",}\n"),
        Err(RecordError::Json(syntax.clone()))
    );
    assert_eq!(
        RecordError::InputJson {
            line: 1,
            column: 10,
        }
        .to_string(),
        "the input is not valid JSON: the JSON at line 1 column 10 is not one"
    );
    assert_eq!(
        RecordError::Json(syntax).to_string(),
        "the record is not valid JSON"
    );
}

/// The line that ended the record is not part of it, and the size is read
/// before the bytes are, so huge bytes that are not text are too large first.
#[test]
#[ignore = "large-input boundary runs in the release suite"]
fn release_only_a_record_over_the_limit_is_refused_and_one_at_the_limit_is_taken() {
    let limit = super::MAX_RECORD_BYTES;
    let over = Err(RecordError::TooLarge);
    let wide = |byte: u8, size: usize, ending: &[u8]| [&vec![byte; size], ending].concat();
    let read = |framing, bytes: &[u8]| sent(&reading(framing, &[]), bytes).map(|t| t.len());
    assert_eq!(read(Framing::Document, &wide(b'x', limit, b"")), Ok(limit));
    assert_eq!(read(Framing::Lines, &wide(b'x', limit, b"\n")), Ok(limit));
    assert_eq!(read(Framing::Lines, &wide(b'x', limit, b"\r\n")), Ok(limit));
    let past = limit + 1;
    assert_eq!(read(Framing::Document, &wide(b'x', past, b"")), over);
    assert_eq!(read(Framing::Lines, &wide(b'x', past, b"\n")), over);
    assert_eq!(read(Framing::Lines, &wide(b'x', past, b"\r\n")), over);
    assert_eq!(read(Framing::Lines, &wide(0xff, past, b"\n")), over);
}

#[test]
fn a_record_carries_its_own_candidate_list_as_a_list_or_as_a_map() {
    let jsonl = reading(Framing::Jsonl, &["/note"]);
    let pointer = Pointer::new("/codes").expect("a pointer");
    let listed = jsonl
        .record(br#"{"note":"n","codes":["late","lost"]}"#)
        .expect("a record")
        .choices(&pointer)
        .expect("a candidate list");
    assert_eq!(
        json_line(&listed).expect("a list is writable"),
        r#"["late","lost"]"#
    );
    assert_eq!(
        listed.descriptions().collect::<Vec<_>>(),
        [(&"late".to_owned(), None), (&"lost".to_owned(), None)]
    );

    let mapped = jsonl
        .record(br#"{"note":"n","codes":{"late":"It arrived late.","lost":"It never came."}}"#)
        .expect("a record")
        .choices(&pointer)
        .expect("a candidate list");
    assert_eq!(
        json_line(&mapped).expect("a list is writable"),
        r#"["late","lost"]"#
    );
    assert_eq!(
        mapped.descriptions().collect::<Vec<_>>(),
        [
            (
                &"late".to_owned(),
                Some(&Description::text("It arrived late.")),
            ),
            (
                &"lost".to_owned(),
                Some(&Description::text("It never came."))
            ),
        ]
    );
}

#[test]
fn a_candidate_list_the_verb_does_not_take_names_no_part_of_the_record() {
    let jsonl = reading(Framing::Jsonl, &["/note"]);
    let pointer = Pointer::new("/codes").expect("a pointer");
    let cases: [(&[u8], RecordError); 6] = [
        (br#"{"note":"n"}"#, RecordError::Missed("/codes".to_owned())),
        (
            br#"{"note":"n","codes":"late"}"#,
            RecordError::OptionsShape("/codes".to_owned()),
        ),
        (
            br#"{"note":"n","codes":["late",7]}"#,
            RecordError::OptionsShape("/codes".to_owned()),
        ),
        (
            br#"{"note":"n","codes":{"late":null}}"#,
            RecordError::Options(LabelsError::OptionCount),
        ),
        (
            br#"{"note":"n","codes":["late"]}"#,
            RecordError::Options(LabelsError::OptionCount),
        ),
        (
            br#"{"note":"n","codes":["late","  "]}"#,
            RecordError::Options(LabelsError::OptionBlank),
        ),
    ];
    for (bytes, expected) in cases {
        let record = jsonl.record(bytes).expect("a record");
        let refused = record.choices(&pointer).expect_err("a refused list");
        assert_eq!(refused, expected);
        let said = refused.to_string();
        assert!(!said.contains("late"), "{said}");
        assert!(!said.contains("arrived"), "{said}");
    }
}

#[test]
fn a_record_mode_plan_names_the_framing_and_the_pointers() {
    let jsonl = reading(Framing::Jsonl, &["/body", "/id"]);
    assert_eq!(
        json_line(&jsonl.plan()).expect("a plan is writable"),
        r#"{"framing":"jsonl","field":["/body","/id"]}"#
    );
    let lines = reading(Framing::Lines, &[]);
    assert_eq!(
        json_line(&lines.plan()).expect("a plan is writable"),
        r#"{"framing":"lines","field":[]}"#
    );
}

#[test]
fn a_record_is_written_back_as_it_arrived() {
    let jsonl = reading(Framing::Jsonl, &["/body"]);
    let record = jsonl
        .record(br#"{"id":"T-91","body":"Payouts failed."}"#)
        .expect("a record");
    assert_eq!(
        json_line(&record).expect("a record is writable"),
        r#"{"id":"T-91","body":"Payouts failed."}"#
    );
    let lines = reading(Framing::Lines, &[]);
    let record = lines.record(b"one line\n").expect("a record");
    assert_eq!(record, Record(Held::Text("one line".to_owned())));
    assert_eq!(
        json_line(&record).expect("a record is writable"),
        r#""one line""#
    );
}

fn texts() -> impl Strategy<Value = String> {
    vec(any::<char>(), 1..24)
        .prop_map(|characters| characters.into_iter().collect::<String>())
        .prop_filter("a line that is one line and is not blank", |text| {
            !text.trim().is_empty() && !text.contains(['\n', '\r'])
        })
}

proptest! {
    /// Any text line reaches the evidence unchanged, and any JSON record
    /// hands its pointed member over as the string it holds.
    #[test]
    fn framing_a_line_keeps_the_line(text in texts()) {
        let lines = reading(Framing::Lines, &[]);
        let framed = sent(&lines, format!("{text}\n").as_bytes());
        prop_assert_eq!(framed.as_deref(), Ok(text.as_str()));
        let jsonl = reading(Framing::Jsonl, &["/body"]);
        let line = json_line(&super::Json::String(text.clone())).expect("a string is writable");
        let pointed = sent(&jsonl, format!("{{\"body\":{line}}}\n").as_bytes());
        prop_assert_eq!(pointed.as_deref(), Ok(text.as_str()));
    }
}

#[test]
fn a_record_comes_back_as_it_arrived_with_its_line_ending_gone() {
    let jsonl = reading(Framing::Jsonl, &["/body"]);
    let odd = b"{ \"id\" : \"T-4\" ,  \"body\":\"Payouts failed.\" }\r\n";
    assert_eq!(
        jsonl.as_it_arrived(odd).expect("text"),
        "{ \"id\" : \"T-4\" ,  \"body\":\"Payouts failed.\" }"
    );
    let lines = reading(Framing::Lines, &[]);
    assert_eq!(
        lines.as_it_arrived(b"one line  \n").expect("text"),
        "one line  "
    );
    let whole = reading(Framing::Document, &[]);
    assert_eq!(
        whole.as_it_arrived(b"a whole input\n").expect("text"),
        "a whole input\n"
    );
    assert_eq!(
        lines.as_it_arrived(&[0xff, 0xfe]).expect_err("not text"),
        RecordError::NotUtf8
    );
}
