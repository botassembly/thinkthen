//! The question a judgment asks, as one verb over what it names.

use serde::Serialize;

use crate::text::Condition;

/// The verbs `decide` knows. Version one judges a condition and nothing else.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Verb {
    /// Ask whether a condition holds for the evidence.
    If,
}

/// What the judgment was asked.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Question {
    verb: Verb,
    condition: Condition,
}

impl Question {
    /// Ask whether a condition holds for the evidence.
    #[must_use]
    pub const fn new_if(condition: Condition) -> Self {
        Self {
            verb: Verb::If,
            condition,
        }
    }

    /// Read the verb back.
    pub(crate) const fn verb(&self) -> Verb {
        self.verb
    }

    /// Read the condition back.
    pub(crate) const fn condition(&self) -> &Condition {
        &self.condition
    }
}

#[cfg(test)]
mod tests {
    use super::{Question, Verb};
    use crate::text::Condition;

    #[test]
    fn a_question_keeps_its_verb_and_its_condition() {
        let condition = Condition::new("asks for a refund").expect("not empty");
        let question = Question::new_if(condition.clone());
        assert_eq!(question.verb(), Verb::If);
        assert_eq!(question.condition(), &condition);
    }
}
