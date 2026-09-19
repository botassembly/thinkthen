//! The answer a backend gave, in thinkthen's own words.

use serde::Serialize;

use crate::probability::Probability;

/// The shapes of answer `decide` knows. Version one reads a yes/no answer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AnswerKind {
    /// A probability that the condition holds.
    YesNo,
}

/// What the backend said, carrying no vendor field name.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Answer {
    kind: AnswerKind,
    probability: Probability,
}

impl Answer {
    /// Take a probability as the answer to a yes/no question.
    #[must_use]
    pub const fn new_yes_no(probability: Probability) -> Self {
        Self {
            kind: AnswerKind::YesNo,
            probability,
        }
    }

    /// Read the probability back.
    #[must_use]
    pub const fn probability(&self) -> Probability {
        self.probability
    }
}

#[cfg(test)]
mod tests {
    use super::Answer;
    use crate::probability::Probability;

    #[test]
    fn an_answer_keeps_the_probability_it_was_given() {
        let probability = Probability::new(0.92).expect("inside the range");
        assert_eq!(Answer::new_yes_no(probability).probability(), probability);
    }
}
