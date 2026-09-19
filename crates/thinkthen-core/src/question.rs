//! The question a judgment asks, as one verb over the text it was given.

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::text::{Meaning, QuestionText};

/// Why a list of options or levels is not one the verb takes.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LabelsError {
    /// Fewer than two options, or more than the verb's ceiling.
    #[error("`choose` takes 2 to 255 options")]
    OptionCount,
    /// Fewer than two levels, or more than the verb's ceiling.
    #[error("`score` takes 2 to 10 levels, lowest first")]
    LevelCount,
    /// A label is empty or holds only white space.
    #[error("an option or a level is text, not white space")]
    Blank,
    /// One label was given twice, so no answer could name which one won.
    #[error("a list holds each option and each level once")]
    Duplicate,
    /// A label holds a control character, which `--raw` would print as it is.
    #[error("an option or a level is one line of printable text")]
    Control,
}

/// The most options `choose` picks between, by ADR 0007.
const MOST_OPTIONS: usize = 255;

/// The most levels `score` places on, by ADR 0007.
const MOST_LEVELS: usize = 10;

/// One label, with the description that rides beside it on the wire.
///
/// The command line carries labels alone, so a positional option has no
/// description. A record that holds a map from label to description gives one.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Label {
    name: String,
    description: Option<String>,
}

/// The options `choose` picks from, or the levels `score` places on.
///
/// The order is the user's own. The tool never reorders a list, because option
/// order moves the odds and a run with a changed list is a different
/// measurement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Labels(Vec<Label>);

impl Serialize for Labels {
    /// Write the names alone, so a result names the labels it was asked with.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.names())
    }
}

impl Labels {
    /// Take 2 to 255 option labels, in the order the user gave them.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the list is too short or too long, when a
    /// label is blank, or when one label was given twice.
    pub fn options(values: Vec<String>) -> Result<Self, LabelsError> {
        Self::checked(bare(values), MOST_OPTIONS, LabelsError::OptionCount)
    }

    /// Take 2 to 255 option labels, each with the description a record gave it.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] on the same four counts `options` does.
    ///
    /// A description that is blank is no description, so a list of names, a map
    /// whose values are `null`, and a map whose values are white space all name
    /// the same question. `question-file.md` writes that rule out.
    pub fn described(values: Vec<(String, Option<String>)>) -> Result<Self, LabelsError> {
        let labels = values
            .into_iter()
            .map(|(name, description)| Label {
                name,
                description: description.filter(|text| !text.trim().is_empty()),
            })
            .collect();
        Self::checked(labels, MOST_OPTIONS, LabelsError::OptionCount)
    }

    /// Take 2 to 10 levels, lowest first.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the list is too short or too long, when a
    /// level is blank, or when one level was given twice.
    pub fn levels(values: Vec<String>) -> Result<Self, LabelsError> {
        Self::checked(bare(values), MOST_LEVELS, LabelsError::LevelCount)
    }

    /// Take a list that is long enough, short enough, filled, and unrepeated.
    fn checked(values: Vec<Label>, most: usize, count: LabelsError) -> Result<Self, LabelsError> {
        if values.len() < 2 || values.len() > most {
            return Err(count);
        }
        if values.iter().any(|value| value.name.trim().is_empty()) {
            return Err(LabelsError::Blank);
        }
        // `--raw` prints a label byte for byte, and `--options` lets a record
        // the tool did not write supply one. A label carrying a line feed or
        // an escape would then write a line of its own into the caller's
        // output. The message never quotes the label, because a record is
        // evidence.
        if values
            .iter()
            .any(|value| value.name.chars().any(char::is_control))
        {
            return Err(LabelsError::Control);
        }
        for (place, value) in values.iter().enumerate() {
            if values
                .iter()
                .skip(place + 1)
                .any(|other| other.name == value.name)
            {
                return Err(LabelsError::Duplicate);
            }
        }
        Ok(Self(values))
    }

    /// Read the names back, in the order they were given.
    pub(crate) fn names(&self) -> impl Iterator<Item = &String> {
        self.0.iter().map(|label| &label.name)
    }

    /// Read each name with its description, in the order they were given.
    pub(crate) fn descriptions(&self) -> impl Iterator<Item = (&String, Option<&str>)> {
        self.0
            .iter()
            .map(|label| (&label.name, label.description.as_deref()))
    }

    /// How many labels the list holds.
    pub(crate) fn count(&self) -> usize {
        self.0.len()
    }
}

/// Take labels the command line gave, which carry no description.
fn bare(values: Vec<String>) -> Vec<Label> {
    values
        .into_iter()
        .map(|name| Label {
            name,
            description: None,
        })
        .collect()
}

/// What the judgment was asked, and which of the three shapes it takes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "verb", rename_all = "snake_case")]
pub enum Question {
    /// Ask whether the question holds for the evidence.
    Decide {
        /// The question the model receives.
        text: QuestionText,
        /// What a yes means, when the user said so. Absent by default.
        #[serde(rename = "true", skip_serializing_if = "Option::is_none")]
        yes: Option<Meaning>,
        /// What a no means, when the user said so. Absent by default.
        #[serde(rename = "false", skip_serializing_if = "Option::is_none")]
        no: Option<Meaning>,
    },
    /// Ask which of a fixed list of labels fits the evidence.
    Choose {
        /// The question the model receives.
        text: QuestionText,
        /// The labels to pick between, in the order the user gave them.
        options: Labels,
    },
    /// Ask where on a list of named levels the evidence sits.
    Score {
        /// The question the model receives.
        text: QuestionText,
        /// The levels, lowest first.
        levels: Labels,
    },
}

#[cfg(test)]
mod tests {
    use super::{Labels, LabelsError, Question};
    use crate::text::{Meaning, QuestionText};

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
                levels: Labels::levels(listed(&["None.", "Some."])).expect("two levels"),
            }),
            r#"{"verb":"score","text":"Which team owns this request?","levels":["None.","Some."]}"#
        );
    }

    #[test]
    fn a_list_the_verb_does_not_take_names_its_own_cause() {
        let many = |count: usize| (0..count).map(|place| place.to_string()).collect();
        let cases: [(Result<Labels, LabelsError>, LabelsError); 8] = [
            (Labels::options(listed(&["only"])), LabelsError::OptionCount),
            (Labels::options(Vec::new()), LabelsError::OptionCount),
            (Labels::options(many(256)), LabelsError::OptionCount),
            (Labels::levels(listed(&["only"])), LabelsError::LevelCount),
            (Labels::levels(many(11)), LabelsError::LevelCount),
            (Labels::options(listed(&["bug", " "])), LabelsError::Blank),
            (Labels::options(listed(&["bug", ""])), LabelsError::Blank),
            (
                Labels::options(listed(&["bug", "other", "bug"])),
                LabelsError::Duplicate,
            ),
        ];
        for (made, expected) in cases {
            assert_eq!(made, Err(expected), "{expected}");
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
        assert_eq!(Labels::levels(many(10)).expect("ten levels").count(), 10);
    }

    #[test]
    fn a_label_holding_a_control_character_is_refused_without_being_quoted() {
        // `--raw` prints a label as it is, so a label that carries a line feed
        // or an escape would write a line of its own into a caller's output.
        let cases = ["bug\nrm -rf /", "bug\r", "bug\u{1b}[31m", "bug\u{0}"];
        for typed in cases {
            let refused = Labels::options(listed(&["other", typed])).expect_err("a refused list");
            assert_eq!(refused, LabelsError::Control, "{typed:?}");
            assert_eq!(
                Labels::levels(listed(&["none", typed])),
                Err(LabelsError::Control),
                "{typed:?}"
            );
            assert_eq!(
                Labels::described(vec![
                    ("other".to_owned(), None),
                    (typed.to_owned(), Some("a description".to_owned())),
                ]),
                Err(LabelsError::Control),
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
            ("late".to_owned(), Some("It arrived late.".to_owned())),
            ("lost".to_owned(), None),
        ])
        .expect("two options");
        assert_eq!(
            described.descriptions().collect::<Vec<_>>(),
            [
                (&"late".to_owned(), Some("It arrived late.")),
                (&"lost".to_owned(), None),
            ]
        );
        assert_eq!(
            Labels::described(vec![
                ("late".to_owned(), None),
                ("late".to_owned(), Some("twice".to_owned())),
            ]),
            Err(LabelsError::Duplicate)
        );
        // A description that is blank is no description at all, so three
        // spellings of the same list are the same list.
        let blank = Labels::described(vec![
            ("late".to_owned(), Some("  ".to_owned())),
            ("lost".to_owned(), Some(String::new())),
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
            levels: Labels::levels(listed(&["None.", "Some."])).expect("two levels"),
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
}
