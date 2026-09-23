use super::{Labels, LabelsError, Question};
use crate::core::json::Json;
use crate::core::text::{Description, Meaning, QuestionText};

fn meaning(text: &str) -> Option<Meaning> {
    Some(Meaning::new(text).expect("not blank"))
}

fn asking() -> Question {
    Question::Decide {
        text: text(),
        yes: None,
        no: None,
    }
}

fn listed(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn named(values: &[&str]) -> Vec<(String, Option<Description>)> {
    listed(values)
        .into_iter()
        .map(|name| (name, None))
        .collect()
}

fn text() -> QuestionText {
    QuestionText::new("Which team owns this request?").expect("not blank")
}

#[test]
fn a_question_serializes_with_its_verb_and_what_the_verb_needs() {
    let rendered =
        |question: &Question| serde_json::to_string(question).expect("a question serializes");
    assert_eq!(
        rendered(&asking()),
        r#"{"verb":"decide","text":"Which team owns this request?"}"#
    );
    // The two texts are absent from the JSON when the user named neither,
    // so a request built today is byte for byte the request of yesterday.
    assert_eq!(
        rendered(&Question::Decide {
            text: text(),
            yes: meaning("The message asks for money back."),
            no: meaning("The message asks for anything else."),
        }),
        concat!(
            r#"{"verb":"decide","text":"Which team owns this request?","#,
            r#""true":"The message asks for money back.","#,
            r#""false":"The message asks for anything else."}"#,
        )
    );
    assert_eq!(
        rendered(&Question::Decide {
            text: text(),
            yes: None,
            no: meaning("The message asks for anything else."),
        }),
        concat!(
            r#"{"verb":"decide","text":"Which team owns this request?","#,
            r#""false":"The message asks for anything else."}"#,
        )
    );
    assert_eq!(
        rendered(&Question::Choose {
            text: text(),
            options: Labels::options(listed(&["bug", "feature"])).expect("two options"),
        }),
        r#"{"verb":"choose","text":"Which team owns this request?","options":["bug","feature"]}"#
    );
    assert_eq!(
        rendered(&Question::Score {
            text: text(),
            levels: Labels::levels(named(&["None.", "Some."])).expect("two levels"),
        }),
        r#"{"verb":"score","text":"Which team owns this request?","levels":["None.","Some."]}"#
    );
    assert_eq!(
        rendered(&Question::Tag {
            text: text(),
            labels: Labels::tags(vec![
                ("billing".to_owned(), None),
                (
                    "urgent".to_owned(),
                    Some(Description::text("Needs prompt attention."))
                ),
            ])
            .expect("two tags"),
        }),
        r#"{"verb":"tag","text":"Which team owns this request?","labels":["billing","urgent"]}"#
    );
}

#[test]
fn a_list_the_verb_does_not_take_names_its_own_cause() {
    let many = |count: usize| (0..count).map(|place| place.to_string()).collect();
    let cases: [(Result<Labels, LabelsError>, LabelsError); 8] = [
        (Labels::options(listed(&["only"])), LabelsError::OptionCount),
        (Labels::options(Vec::new()), LabelsError::OptionCount),
        (Labels::options(many(256)), LabelsError::OptionCount),
        (Labels::levels(named(&["only"])), LabelsError::LevelCount),
        (
            Labels::levels(many(11).into_iter().map(|name| (name, None)).collect()),
            LabelsError::LevelCount,
        ),
        (
            Labels::options(listed(&["bug", " "])),
            LabelsError::OptionBlank,
        ),
        (
            Labels::options(listed(&["bug", ""])),
            LabelsError::OptionBlank,
        ),
        (
            Labels::options(listed(&["bug", "other", "bug"])),
            LabelsError::OptionDuplicate,
        ),
    ];
    for (made, expected) in cases {
        assert_eq!(made, Err(expected), "{expected}");
    }
}

#[test]
fn tags_take_one_to_twenty_labels_in_order() {
    let many = |count: usize| (0..count).map(|place| (place.to_string(), None)).collect();
    assert_eq!(Labels::tags(Vec::new()), Err(LabelsError::TagCount));
    assert_eq!(Labels::tags(many(1)).expect("one tag").count(), 1);
    assert_eq!(Labels::tags(many(21)), Err(LabelsError::TagCount));
    assert_eq!(Labels::tags(many(20)).expect("twenty tags").count(), 20);
    assert_eq!(
        Labels::tags(vec![("b".to_owned(), None), ("a".to_owned(), None)])
            .expect("two tags")
            .names()
            .collect::<Vec<_>>(),
        [&"b".to_owned(), &"a".to_owned()]
    );
}

#[test]
fn each_list_kind_names_its_own_invalid_member_without_echoing_it() {
    let cases = [
        (
            Labels::options(listed(&["a", " "])),
            "an option is text, not white space",
        ),
        (
            Labels::tags(vec![("a".to_owned(), None), (" ".to_owned(), None)]),
            "a label is text, not white space",
        ),
        (
            Labels::levels(named(&["a", " "])),
            "a level is text, not white space",
        ),
    ];
    for (result, message) in cases {
        assert_eq!(result.expect_err("invalid member").to_string(), message);
    }
}

#[test]
fn a_list_at_each_edge_of_the_range_is_taken_and_keeps_its_order() {
    let many = |count: usize| (0..count).map(|place| place.to_string()).collect();
    assert_eq!(
        Labels::options(listed(&["b", "a"]))
            .expect("two options")
            .names()
            .collect::<Vec<_>>(),
        [&"b".to_owned(), &"a".to_owned()]
    );
    assert_eq!(
        Labels::options(many(255)).expect("255 options").count(),
        255
    );
    assert_eq!(
        Labels::levels(many(10).into_iter().map(|name| (name, None)).collect())
            .expect("ten levels")
            .count(),
        10
    );
}

#[test]
fn a_label_holding_a_control_character_is_refused_without_being_quoted() {
    // `--raw` prints a label as it is, so a label that carries a line feed
    // or an escape would write a line of its own into a caller's output.
    let cases = ["bug\nrm -rf /", "bug\r", "bug\u{1b}[31m", "bug\u{0}"];
    for typed in cases {
        let refused = Labels::options(listed(&["other", typed])).expect_err("a refused list");
        assert_eq!(refused, LabelsError::OptionControl, "{typed:?}");
        assert_eq!(
            Labels::levels(named(&["none", typed])),
            Err(LabelsError::LevelControl),
            "{typed:?}"
        );
        assert_eq!(
            Labels::described(vec![
                ("other".to_owned(), None),
                (typed.to_owned(), Some(Description::text("a description"))),
            ]),
            Err(LabelsError::OptionControl),
            "{typed:?}"
        );
        let said = refused.to_string();
        assert!(!said.contains("rm -rf"), "{said}");
        assert!(!said.contains('\n'), "{said:?}");
    }
    // A label with an ordinary space inside it is still one line of text.
    assert!(Labels::options(listed(&["not stated", "stated"])).is_ok());
}

#[test]
fn a_described_list_keeps_each_description_beside_its_own_label() {
    let described = Labels::described(vec![
        (
            "late".to_owned(),
            Some(Description::text("It arrived late.")),
        ),
        ("lost".to_owned(), None),
    ])
    .expect("two options");
    assert_eq!(
        described.descriptions().collect::<Vec<_>>(),
        [
            (
                &"late".to_owned(),
                Some(&Description::text("It arrived late."))
            ),
            (&"lost".to_owned(), None),
        ]
    );
    assert_eq!(
        Labels::described(vec![
            ("late".to_owned(), None),
            ("late".to_owned(), Some(Description::text("twice"))),
        ]),
        Err(LabelsError::OptionDuplicate)
    );
    // A description that is blank is no description at all, so three
    // spellings of the same list are the same list.
    let blank = Labels::described(vec![
        ("late".to_owned(), Some(Description::text("  "))),
        ("lost".to_owned(), Some(Description::text(String::new()))),
    ])
    .expect("two options");
    assert_eq!(
        blank.descriptions().collect::<Vec<_>>(),
        [(&"late".to_owned(), None), (&"lost".to_owned(), None)]
    );
}

#[test]
fn a_question_shows_its_own_text_and_labels_in_debug_and_nothing_else() {
    let pick = Question::Choose {
        text: text(),
        options: Labels::options(listed(&["bug", "feature"])).expect("two options"),
    };
    let placement = Question::Score {
        text: text(),
        levels: Labels::levels(named(&["None.", "Some."])).expect("two levels"),
    };

    // A question never receives the evidence or the key, so its `Debug` has
    // nothing to hide. This pins that, so a field carrying either one fails.
    for shown in [format!("{pick:?}"), format!("{placement:?}")] {
        assert!(shown.contains("Which team owns this request?"), "{shown}");
        for kept_out in ["state", "Evidence", "key", "api", "sk-"] {
            assert!(!shown.contains(kept_out), "{kept_out} in {shown}");
        }
    }
}

#[test]
fn a_question_never_shows_a_description_or_a_meaning_in_debug() {
    let private = "private marker words";
    let tagged = Question::Tag {
        text: text(),
        labels: Labels::tags(vec![(
            "billing".to_owned(),
            Some(Description::text(private)),
        )])
        .expect("a tag"),
    };
    let decided = Question::Decide {
        text: text(),
        yes: Some(
            Meaning::structured(
                &Json::parse(&format!(r#"{{"means":"{private}"}}"#)).expect("JSON"),
            )
            .expect("a meaning"),
        ),
        no: None,
    };

    for shown in [format!("{tagged:?}"), format!("{decided:?}")] {
        assert!(shown.contains("Which team owns this request?"), "{shown}");
        assert!(!shown.contains(private), "{shown}");
    }
}
