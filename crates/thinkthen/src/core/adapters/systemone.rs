//! The `systemone` wire format, as two pure translations of one plan.
//!
//! The two directions live apart. One module writes a plan as bytes and the
//! other reads bytes as answers, and each one keeps its own fixtures.
//!
//! This module owns every word that names the vendor behind this shape: the
//! adapter's name, the address a run reaches when nothing else names one, the
//! model a request carries when nothing else names one, and the path the
//! endpoint sits under. Nothing outside this module names any of the four.

pub(crate) mod backends;
mod recorded;
mod request;
mod response;

use thiserror::Error;

use crate::core::text::ModelName;

pub(crate) use crate::core::adapters::systemone::recorded::decoder;
pub(crate) use crate::core::adapters::systemone::request::encode;
pub(crate) use crate::core::adapters::systemone::request::{
    drops_any, drops_detail, drops_detail_of, encode_raw, join, parts,
};
pub(crate) use crate::core::adapters::systemone::response::{
    decode, decode_answers, decode_observed, decode_questions,
};

/// The name this adapter answers to, in the recording key and the entry.
///
/// Ruling 1 of ADR 0010 leaves one wire shape, so nothing chooses between
/// shapes and this is the only name there is. The binary never needs it,
/// because the address and the entry are both written in this crate.
pub(crate) const NAME: &str = "systemone";

/// The base this adapter is reached at when nothing else names one.
///
/// It ends at the version, as other model clients' bases do. `--url` outranks
/// `THINKTHEN_BASE_URL`, which outranks this. `specification/backends.md`
/// states the order.
pub(crate) const DEFAULT_BASE: &str = "https://api.typesafe.ai/v1";

/// The model this adapter names when `--model` and the question file name none.
pub(crate) const DEFAULT_MODEL: &str = "jev-1.13.0";

/// The built-in mutable name whose current target cannot be known offline.
#[must_use]
pub(crate) fn is_mutable_alias(model: &ModelName) -> bool {
    model.as_str() == "jev-latest"
}

/// The path under a base that this adapter's endpoint sits at.
///
/// It spells the adapter's name here, and another adapter's need not. The
/// address rule reads the base and appends this, so the two are separate
/// values even where they read alike.
pub(crate) const ENDPOINT_PATH: &str = "systemone";

/// The optional request ID response header this built-in adapter understands.
pub(crate) const ATTEMPT_REQUEST_ID_HEADER: &str = "x-typesafe-request-id";

/// True when a reported model is safe and useful in a mixed-version diagnostic.
#[must_use]
pub(crate) fn diagnostic_model(value: &str, requested: &str) -> bool {
    let printable = |text: &str| {
        !text.is_empty()
            && text.len() <= 64
            && text.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/')
            })
    };
    if printable(value) && printable(requested) && value == requested {
        return true;
    }
    let Some(version) = value.strip_prefix("jev-") else {
        return false;
    };
    let parts: Vec<&str> = version.split('.').collect();
    printable(value)
        && (1..=3).contains(&parts.len())
        && parts.iter().all(|part| {
            (1..=4).contains(&part.len()) && part.bytes().all(|byte| byte.is_ascii_digit())
        })
}

/// Why a plan could not be written as a request body.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("the plan could not be written as JSON: {0}")]
pub(crate) struct EncodeError(String);

impl EncodeError {
    /// Say that one document could not be written as JSON.
    pub(crate) fn of(said: &impl ToString) -> Self {
        Self(said.to_string())
    }
}

/// Why a response body is not an answer to the plan that was sent.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum DecodeError {
    /// The bytes are not a `systemone` response.
    ///
    /// The message names the place and never the text. A backend can send back
    /// whatever was sent to it, and a JSON reader quotes the value it stopped
    /// on, so quoting the reader would print the evidence.
    #[error("the response is not a systemone response: the JSON at line {0} column {1} is not one")]
    Malformed(usize, usize),
    /// The response names no model, so nothing says what answered.
    #[error("the response names no model")]
    NoModel,
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
    /// The answer's probabilities do not make a complete distribution.
    #[error(
        "the answer to question `{}` has probability total {total}, member count {members}, and tolerance {tolerance}; the total differs from one by more than the tolerance",
        wire_name(*place)
    )]
    DistributionTotal {
        /// The zero-based question place used to derive its wire name.
        place: usize,
        /// The computed sum in its shortest round-trip decimal form.
        total: String,
        /// The number of members in that sum.
        members: usize,
        /// The accepted distance in its shortest round-trip decimal form.
        tolerance: String,
    },
    /// The answer names a probability for a label the question did not send.
    #[error(
        "the answer to question `{}` has a probability for an option or level the question did not send",
        wire_name(*.0)
    )]
    UnexpectedProbability(usize),
    /// The response carries an answer name the request did not send.
    #[error("the response carries an unexpected answer name")]
    UnexpectedAnswer,
}

/// The wire type one question travels as. The backend check names its
/// one-question probes by it.
pub(crate) const fn wire_type(question: &crate::core::question::Question) -> &'static str {
    use crate::core::question::Question;
    match question {
        Question::Decide { .. } | Question::Tag { .. } => "noul",
        Question::Choose { .. } => "choice",
        Question::Score { .. } => "score",
    }
}

/// The name a question carries on the wire: `q1` onward, in plan order.
pub(crate) fn wire_name(place: usize) -> String {
    format!("q{}", place + 1)
}

/// The place a wire name spells, read back as `wire_name` writes it, if any.
pub(crate) fn wire_place(name: &str) -> Option<usize> {
    let place = name
        .strip_prefix('q')?
        .parse::<usize>()
        .ok()?
        .checked_sub(1)?;
    (wire_name(place) == name).then_some(place)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::diagnostic_model;
    use crate::core::plan::Plan;
    use crate::core::question::{Labels, Question};
    use crate::core::text::{Description, Evidence, ModelName, QuestionText};

    #[test]
    fn only_safe_expected_or_versioned_model_names_reach_diagnostics() {
        for accepted in ["jev-latest", "jev-1", "jev-1.2", "jev-1234.2.30"] {
            assert!(
                diagnostic_model(accepted, "jev-latest"),
                "expected {accepted:?} to be safe"
            );
        }
        for refused in [
            "",
            "other-model",
            "jev-",
            "jev-1.2.3.4",
            "jev-12345",
            "jev-1a",
            "jev-1\u{1b}",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        ] {
            assert!(
                !diagnostic_model(refused, "jev-latest"),
                "expected {refused:?} to be hidden"
            );
        }
        assert!(diagnostic_model("other-model", "other-model"));
        assert!(!diagnostic_model("other\u{1b}", "other\u{1b}"));
    }

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
        Plan::authored(
            Evidence::new(state).expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            vec![question],
        )
        .expect("a question is asked")
    }

    /// A plan of one yes/no question per text, over one evidence.
    pub(crate) fn plan_for(state: &str, questions: &[&str]) -> Plan {
        Plan::authored(
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
                levels: Labels::levels(
                    labels(&LEVELS)
                        .into_iter()
                        .map(|name| (name, None))
                        .collect(),
                )
                .expect("three levels"),
            },
        )
    }

    /// A tag plan with one bare and one described label.
    pub(crate) fn tag_plan() -> Plan {
        one(
            "The invoice failed and work is blocked.",
            Question::Tag {
                text: QuestionText::new("Which topics?").expect("not blank"),
                labels: Labels::tags(vec![
                    (r#"bill\"ing"#.to_owned(), None),
                    (
                        "urgent".to_owned(),
                        Some(Description::text("The item needs prompt attention.")),
                    ),
                ])
                .expect("two tags"),
            },
        )
    }
}
