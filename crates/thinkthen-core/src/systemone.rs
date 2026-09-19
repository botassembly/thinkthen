//! The `systemone` wire format, as two pure translations of one plan.
//!
//! The two directions live apart. One module writes a plan as bytes and the
//! other reads bytes as answers, and each one keeps its own fixtures.

mod request;
mod response;

use thiserror::Error;

pub use crate::systemone::request::encode;
pub(crate) use crate::systemone::request::encode_raw;
pub use crate::systemone::response::decode;

/// The name this wire shape answers to, in an address and in a recording entry.
///
/// Ruling 1 of ADR 0010 leaves one wire shape, so nothing chooses between
/// shapes and this is the only name there is. The binary never needs it,
/// because the address and the entry are both written in this crate.
pub(crate) const NAME: &str = "systemone";

/// Why a plan could not be written as a request body.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("the plan could not be written as JSON: {0}")]
pub struct EncodeError(String);

impl EncodeError {
    /// Say that one document could not be written as JSON.
    pub(crate) fn of(said: &impl ToString) -> Self {
        Self(said.to_string())
    }
}

/// Why a response body is not an answer to the plan that was sent.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DecodeError {
    /// The bytes are not a `systemone` response.
    #[error("the response is not a systemone response: {0}")]
    Malformed(String),
    /// The response answered every question but the one in this place.
    #[error("the response carries no answer for question `{}`", wire_name(*.0))]
    MissingAnswer(usize),
    /// The answer holds a shape this question did not ask for.
    #[error("the answer to question `{}` is not the shape the question asked for", wire_name(*.0))]
    WrongKind(usize),
    /// The answer gives no probability for an option or a level that was sent.
    #[error(
        "the answer to question `{}` leaves an option or a level without a probability",
        wire_name(*.0)
    )]
    MissingProbability(usize),
    /// The answer holds a number that is not a probability.
    #[error("the answer to question `{}` holds a probability outside zero to one", wire_name(*.0))]
    ProbabilityOutOfRange(usize),
}

/// The name a question carries on the wire: `q1` onward, in plan order.
fn wire_name(place: usize) -> String {
    format!("q{}", place + 1)
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::plan::Plan;
    use crate::question::{Labels, Question};
    use crate::text::{Evidence, ModelName, QuestionText};

    /// The teams a routing question picks between, in the order they are sent.
    pub(crate) const TEAMS: [&str; 4] = ["billing", "shipping", "account", "other"];

    /// The levels a placement question uses, lowest first.
    pub(crate) const LEVELS: [&str; 3] = [
        "None.",
        "Work continues with a workaround.",
        "Work is blocked.",
    ];

    /// The labels a case names, as the list a verb takes.
    fn labels(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    /// One plan of one question over one evidence.
    fn one(state: &str, question: Question) -> Plan {
        Plan::new(
            Evidence::new(state).expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            vec![question],
        )
        .expect("a question is asked")
    }

    /// A plan of one yes/no question per text, over one evidence.
    pub(crate) fn plan_for(state: &str, questions: &[&str]) -> Plan {
        Plan::new(
            Evidence::new(state).expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            questions
                .iter()
                .map(|text| Question::Decide {
                    text: QuestionText::new(*text).expect("not blank"),
                    yes: None,
                    no: None,
                })
                .collect(),
        )
        .expect("a question is asked")
    }

    /// The `decide-urgent` fixture's own plan.
    pub(crate) fn urgency_plan() -> Plan {
        plan_for(
            "Help! My payouts have been failing for 3 days.",
            &["Does this convey urgency?"],
        )
    }

    /// The `choose-team` fixture's own plan.
    pub(crate) fn team_plan() -> Plan {
        one(
            "Subject: card declined on renewal\n\nMy yearly plan tried to renew last night and the charge bounced.\n",
            Question::Choose {
                text: QuestionText::new("Which team owns this request?").expect("not blank"),
                options: Labels::options(labels(&TEAMS)).expect("four options"),
            },
        )
    }

    /// The `score-disruption` fixture's own plan.
    pub(crate) fn disruption_plan() -> Plan {
        one(
            "The nightly import died again at 02:14. It stops on the row with an empty postcode. We need a fix before the next run.",
            Question::Score {
                text: QuestionText::new("How much disruption does this report?")
                    .expect("not blank"),
                levels: Labels::levels(labels(&LEVELS)).expect("three levels"),
            },
        )
    }
}
