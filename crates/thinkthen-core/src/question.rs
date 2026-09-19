//! The question a judgment asks, as one verb over the text it was given.

use serde::Serialize;

use crate::text::QuestionText;

/// The verbs that judge. Version one asks a yes/no question and nothing else.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Verb {
    /// Ask whether the question holds for the evidence.
    Decide,
}

/// What the judgment was asked.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Question {
    verb: Verb,
    text: QuestionText,
}

impl Question {
    /// Ask whether the question holds for the evidence.
    #[must_use]
    pub const fn new_decide(text: QuestionText) -> Self {
        Self {
            verb: Verb::Decide,
            text,
        }
    }

    /// Read the verb back.
    pub(crate) const fn verb(&self) -> Verb {
        self.verb
    }

    /// Read the question text back.
    pub(crate) const fn text(&self) -> &QuestionText {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::{Question, Verb};
    use crate::text::QuestionText;

    #[test]
    fn a_question_keeps_its_verb_and_its_text() {
        let text = QuestionText::new("asks for a refund").expect("not empty");
        let question = Question::new_decide(text.clone());
        assert_eq!(question.verb(), Verb::Decide);
        assert_eq!(question.text(), &text);
    }
}
