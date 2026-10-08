use super::{ExampleError, RecognitionExample, render_examples};
use crate::core::RecognizeSpec;
use proptest::prelude::*;

fn spec() -> RecognizeSpec {
    RecognizeSpec::from_parts(vec![("person".into(), None)], Vec::new(), None, None).unwrap()
}

#[test]
fn annotations_admit_only_whole_nonoverlapping_pieces_and_declared_kinds() {
    for (json, error) in [
        (
            r#"{"text":"Ada","entities":[{"start":1,"end":3,"kind":"person"}]}"#,
            ExampleError::Edge,
        ),
        (
            r#"{"text":"Ada","entities":[{"start":0,"end":2,"kind":"person"}]}"#,
            ExampleError::Edge,
        ),
        (
            r#"{"text":"Ada","entities":[{"start":0,"end":3,"kind":"other"}]}"#,
            ExampleError::Kind,
        ),
        (
            r#"{"text":"Ada","entities":[{"start":0,"end":3,"kind":"person"},{"start":0,"end":3,"kind":"person"}]}"#,
            ExampleError::Overlap,
        ),
        (
            r#"{"text":"Ada","entities":[],"kinds":[]}"#,
            ExampleError::Vocabulary,
        ),
        (
            r#"{"text":"Ada","entities":[],"kinds":["person","person"]}"#,
            ExampleError::Vocabulary,
        ),
    ] {
        let example = serde_json::from_str(json).unwrap();
        assert_eq!(
            render_examples(&spec(), &[example]).unwrap_err().cause,
            error
        );
    }
    for invalid in [
        r#"{"text":"Ada","entities":[],"kinds":null}"#,
        r#"{"text":"Ada","text":"Bob","entities":[]}"#,
        r#"{"text":"Ada","entities":[],"unknown":1}"#,
        r#"{"text":"Ada","entities":[{"start":0,"end":3,"end":4,"kind":"person"}]}"#,
    ] {
        assert!(serde_json::from_str::<RecognitionExample>(invalid).is_err());
    }
    for invalid in [
        "[[person|Ada]",
        "[[person|]]",
        "[[person|[[person|Ada]]]]",
        "[[person|[Ada | person]]]",
        "[[person|Ada] | person]",
        "[[|Ada]]",
        "Ada\\x",
        "Ada]]",
        "[Ada|person",
        "[|person]",
        "[Ada|]",
        "[Ada|person|other]",
        "[[Ada]|person]",
    ] {
        assert_eq!(
            super::brackets::parse(invalid).unwrap_err(),
            ExampleError::Brackets
        );
    }
}

#[test]
fn ordinary_bracket_literals_remain_text() {
    for source in ["Ada [note]", "[[person|Ada [note] here]]"] {
        let parsed = super::brackets::parse(source).unwrap();
        assert!(parsed.text.contains("[note]"));
    }
    let parsed = super::brackets::parse("[[person|Ada [note]]]").unwrap();
    assert_eq!(parsed.text, "Ada [note]");
    assert_eq!(parsed.entities[0].start, 0);
    assert_eq!(parsed.entities[0].end, 10);
}

#[test]
fn equivalent_scalar_spans_and_brackets_render_identically_with_other_kinds_out() {
    let bracket = RecognitionExample::Brackets("[[person|Zoë A\u{301}]] met Orbit.".into());
    let explicit: RecognitionExample = serde_json::from_str(r#"{"text":"Zoë Á met Orbit.","entities":[{"start":0,"end":6,"kind":"person"},{"start":11,"end":16,"kind":"organization"}],"kinds":["person","organization"]}"#).unwrap();
    let expected = render_examples(&spec(), &[bracket]).unwrap();
    let original_form = RecognitionExample::Brackets("[Zoë A\u{301} | person] met Orbit.".into());
    assert_eq!(
        render_examples(&spec(), &[original_form]).unwrap(),
        expected
    );
    assert_eq!(render_examples(&spec(), &[explicit]).unwrap(), expected);
    assert!(expected[0].contains("Answer: \"BEGIN\"; kind: \"person\""));
    assert!(expected[0].contains("Answer: \"END\"; kind: \"person\""));
    assert!(!expected[0].contains("kind: \"organization\""));
    assert!(expected[0].contains("Snippet: Zoë Á met [[Orbit]].\nAnswer: \"OUT\""));
}

proptest! {
    #[test]
    fn escaped_bracket_text_preserves_every_scalar(text in "[a-zA-Z0-9\\[\\]\\|\\\\é]{1,40}") {
        let escaped: String = text.chars().flat_map(|c| {
            if matches!(c, '\\' | '[' | ']' | '|') { vec!['\\', c] } else { vec![c] }
        }).collect();
        let parsed = super::brackets::parse(&format!("[[person|{escaped}]]")).unwrap();
        prop_assert_eq!(&parsed.text, &text);
        prop_assert_eq!(parsed.entities[0].start, 0);
        prop_assert_eq!(parsed.entities[0].end, text.chars().count());
    }
}
