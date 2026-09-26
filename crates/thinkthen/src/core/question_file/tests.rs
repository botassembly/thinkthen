//! Every refusal the question-file grammar makes, and the key it names.

use super::{QuestionFile, QuestionFileError as Refused, Source, Verb};
use crate::core::json::JsonError;
use crate::core::text::BlankTextError;
use crate::core::threshold::ThresholdError;

fn refused(text: &str) -> Refused {
    QuestionFile::parse(text).expect_err("a refused question file")
}

#[test]
fn every_refusal_of_the_grammar_names_the_key_at_fault() {
    let cases: [(&str, Refused, &str); 12] = [
        (
            "not json",
            Refused::NotJson(JsonError::Syntax { line: 1, column: 2 }),
            "the question file is not valid JSON: the JSON at line 1 column 2 is not one",
        ),
        ("[1,2]", Refused::NotAnObject, "one JSON object"),
        (r#"{"model":"m"}"#, Refused::NoVerb, "`decide`"),
        (
            r#"{"decide":"a","choose":"b"}"#,
            Refused::TwoVerbs("decide", "choose"),
            "`decide` and `choose`",
        ),
        (
            r#"{"decide":"a","nonsense":1}"#,
            Refused::UnknownKey("nonsense".to_owned()),
            "nonsense",
        ),
        (
            r#"{"choose":"a","options":["x","y"],"true":"t"}"#,
            Refused::KeyNotForVerb {
                key: "true".to_owned(),
                verb: Verb::Choose,
            },
            "takes no key `true`",
        ),
        (
            r#"{"score":"a","levels":["x","y"],"threshold":0.5}"#,
            Refused::KeyNotForVerb {
                key: "threshold".to_owned(),
                verb: Verb::Score,
            },
            "takes no key `threshold`",
        ),
        (
            r#"{"decide":7}"#,
            Refused::Shape {
                key: "decide",
                wanted: "is text, an object, or a list",
            },
            "`decide` in the question file is text, an object, or a list",
        ),
        (
            r#"{"decide":"  "}"#,
            Refused::Blank {
                origin: Source::File,
                key: "decide",
                error: BlankTextError::QuestionText,
            },
            "the question file's `decide`",
        ),
        (
            r#"{"choose":"a","options":"x"}"#,
            Refused::Shape {
                key: "options",
                wanted: "is a list of labels, or a map from each label to its description",
            },
            "map from each label",
        ),
        (
            r#"{"decide":"a","threshold":90}"#,
            Refused::Threshold {
                origin: Source::File,
                error: ThresholdError::CutOutOfRange,
            },
            "the question file's `threshold`",
        ),
        (
            r#"{"decide":"a","on":"body"}"#,
            Refused::Pointer {
                origin: Source::File,
                key: "on",
                typed: "body".to_owned(),
                error: crate::core::pointer::PointerError::NotAPointer,
            },
            "the question file's `on`",
        ),
    ];
    for (text, expected, said) in cases {
        let refusal = refused(text);
        assert_eq!(refusal, expected, "{text}");
        assert!(refusal.to_string().contains(said), "{refusal} :: {text}");
    }
}

#[test]
fn an_unknown_key_is_json_escaped_inside_one_diagnostic_line() {
    let error = refused(r#"{"decide":"a","line\nbreak":1}"#);
    assert_eq!(
        error.to_string(),
        r#"a question file takes no key `line\nbreak`"#
    );
    assert_eq!(error.to_string().lines().count(), 1);
}
