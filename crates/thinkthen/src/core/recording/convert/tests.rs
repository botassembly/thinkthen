//! The edge table for `--quote`'s test of an instruction that already quotes
//! its record (ticket 0364).

use super::{Requote, quotes_a_record, requote};
use crate::core::json::Json;

/// The state's record, an instruction, and whether it already quotes a record.
const CASES: &[(&str, &str, &str, bool)] = &[
    (
        "the record's own JSON",
        r#""Yesterday""#,
        r#"The text is "Yesterday". Is it a song?"#,
        true,
    ),
    (
        "a context's string record",
        r#""the context""#,
        r#"The text is "a. b". Is it?"#,
        true,
    ),
    (
        "a context's object record",
        r#""the context""#,
        r#"The text is {"a":"x. y"}. Is it?"#,
        true,
    ),
    (
        "a context's list record",
        r#""the context""#,
        r#"The text is [1,2]. Is it?"#,
        true,
    ),
    (
        "the question's own words",
        r#""Yesterday""#,
        "The text is the title of a song by the Beatles. Is it?",
        false,
    ),
    (
        "a scalar question word",
        r#""Yesterday""#,
        "The text is true. Is it?",
        false,
    ),
    (
        "a null question word",
        r#""Yesterday""#,
        "The text is null. Is it?",
        false,
    ),
    (
        "a number question word",
        r#""Yesterday""#,
        "The text is 42. Is it?",
        false,
    ),
    (
        "a scalar record that is the state",
        "42",
        "The text is 42. Is it?",
        true,
    ),
    // Leftover risk 1: a context exchange over the scalar record 42 is quoted again.
    (
        "a context's scalar record",
        r#""the context""#,
        "The text is 42. Is it?",
        false,
    ),
    // Leftover risk 2: a question that opens with a quoted word is left unquoted.
    (
        "a question opening with a quoted word",
        r#""Yesterday""#,
        r#"The text is "urgent". Is it?"#,
        true,
    ),
    ("no prefix", r#""Yesterday""#, "Is it a song?", false),
    (
        "no stop after the value",
        r#""Yesterday""#,
        r#"The text is "Yesterday""#,
        false,
    ),
];

#[test]
fn an_instruction_already_quotes_a_record_only_at_a_whole_json_value() {
    for (name, line, asked, expected) in CASES {
        assert_eq!(quotes_a_record(line, asked), *expected, "{name}");
    }
}

#[test]
fn requote_quotes_a_plain_question_and_leaves_a_quoted_or_json_one() {
    let question = |instructions: &str| {
        Json::parse(&format!(
            r#"{{"type":"noul","instructions":{instructions}}}"#
        ))
        .ok()
    };
    let plain = question(r#""The text is the title of a song. Is it?""#);
    let quoted = question(r#""The text is \"a\". Is it?""#);
    let object = question(r#"{"ask":"Is it?"}"#);
    assert_eq!(
        requote(r#""Yesterday""#, &plain.into_iter().collect::<Vec<_>>()),
        Requote::Quoted(vec![r#"{"type":"noul","instructions":"The text is \"Yesterday\". The text is the title of a song. Is it?"}"#.to_owned()])
    );
    assert_eq!(
        requote(r#""ctx""#, &quoted.into_iter().collect::<Vec<_>>()),
        Requote::Already
    );
    assert_eq!(
        requote(r#""ctx""#, &object.into_iter().collect::<Vec<_>>()),
        Requote::Unquotable
    );
}
