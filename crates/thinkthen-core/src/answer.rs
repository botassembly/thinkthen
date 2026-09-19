//! The answer a backend gave, in thinkthen's own words.

use serde::{Deserialize, Serialize};

use crate::probability::Probability;

/// The shapes of answer `decide` knows. Version one reads a yes/no answer.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerKind {
    /// A probability that the condition holds.
    YesNo,
}

/// What the backend said, carrying no vendor field name.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
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

    /// Read the kind back.
    #[must_use]
    pub const fn kind(&self) -> AnswerKind {
        self.kind
    }

    /// Read the probability back.
    #[must_use]
    pub const fn probability(&self) -> Probability {
        self.probability
    }
}

#[cfg(test)]
mod tests {
    use super::{Answer, AnswerKind};
    use crate::probability::Probability;

    #[test]
    fn an_answer_keeps_its_kind_and_its_probability() {
        let probability = Probability::new(0.92).expect("inside the range");
        let answer = Answer::new_yes_no(probability);
        assert_eq!(answer.kind(), AnswerKind::YesNo);
        assert_eq!(answer.probability(), probability);
    }
}
