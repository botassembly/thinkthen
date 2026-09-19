//! The question a judgment asks, as one verb over the text it was given.

use serde::Serialize;
use thiserror::Error;

use crate::text::QuestionText;

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
}

/// The most options `choose` picks between, by ADR 0007.
const MOST_OPTIONS: usize = 255;

/// The most levels `score` places on, by ADR 0007.
const MOST_LEVELS: usize = 10;

/// The options `choose` picks from, or the levels `score` places on.
///
/// The order is the user's own. The tool never reorders a list, because option
/// order moves the odds and a run with a changed list is a different
/// measurement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Labels(Vec<String>);

impl Labels {
    /// Take 2 to 255 option labels, in the order the user gave them.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the list is too short or too long, when a
    /// label is blank, or when one label was given twice.
    pub fn options(values: Vec<String>) -> Result<Self, LabelsError> {
        Self::checked(values, MOST_OPTIONS, LabelsError::OptionCount)
    }

    /// Take 2 to 10 levels, lowest first.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the list is too short or too long, when a
    /// level is blank, or when one level was given twice.
    pub fn levels(values: Vec<String>) -> Result<Self, LabelsError> {
        Self::checked(values, MOST_LEVELS, LabelsError::LevelCount)
    }

    /// Take a list that is long enough, short enough, filled, and unrepeated.
    fn checked(values: Vec<String>, most: usize, count: LabelsError) -> Result<Self, LabelsError> {
        if values.len() < 2 || values.len() > most {
            return Err(count);
        }
        if values.iter().any(|value| value.trim().is_empty()) {
            return Err(LabelsError::Blank);
        }
        for (place, value) in values.iter().enumerate() {
            if values.iter().skip(place + 1).any(|other| other == value) {
                return Err(LabelsError::Duplicate);
            }
        }
        Ok(Self(values))
    }

    /// Read the labels back, in the order they were given.
    pub(crate) fn as_slice(&self) -> &[String] {
        &self.0
    }
}

/// What the judgment was asked, and which of the three shapes it takes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "verb", rename_all = "snake_case")]
pub enum Question {
    /// Ask whether the question holds for the evidence.
    Decide {
        /// The question the model receives.
        text: QuestionText,
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
    use crate::text::QuestionText;

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
            rendered(&Question::Decide { text: text() }),
            r#"{"verb":"decide","text":"Which team owns this request?"}"#
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
                .as_slice(),
            ["b".to_owned(), "a".to_owned()]
        );
        assert_eq!(
            Labels::options(many(255))
                .expect("255 options")
                .as_slice()
                .len(),
            255
        );
        assert_eq!(
            Labels::levels(many(10))
                .expect("ten levels")
                .as_slice()
                .len(),
            10
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
