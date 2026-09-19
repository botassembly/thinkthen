//! The `systemone` wire format, as two pure translations of one plan.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use thiserror::Error;

use crate::answer::Answer;
use crate::plan::Plan;
use crate::probability::Probability;
use crate::question::{Question, Verb};
use crate::reply::Reply;
use crate::result::Usage;
use crate::text::ModelName;

/// Why a plan could not be written as a request body.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("the plan could not be written as JSON: {0}")]
pub struct EncodeError(String);

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
    #[error("the answer to question `{}` is not a yes/no answer", wire_name(*.0))]
    WrongKind(usize),
    /// The answer holds a number that is not a probability.
    #[error("the answer to question `{}` holds a probability outside zero to one", wire_name(*.0))]
    ProbabilityOutOfRange(usize),
}

/// The body one request carries.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq))]
struct Request {
    state: String,
    model: String,
    questions: BTreeMap<String, RequestQuestion>,
}

/// One named question inside a request.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq))]
struct RequestQuestion {
    #[serde(rename = "type")]
    kind: QuestionKind,
    instructions: String,
}

/// The question shapes this adapter sends.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq))]
enum QuestionKind {
    /// A yes/no question, which the vendor calls `noul`.
    #[serde(rename = "noul")]
    Noul,
}

/// The body one response carries.
#[derive(Debug, Deserialize)]
struct Response {
    model: String,
    answers: BTreeMap<String, ResponseAnswer>,
    #[serde(default)]
    usage: Option<ResponseUsage>,
}

/// One named answer inside a response.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ResponseAnswer {
    /// The answer to a yes/no question, as one probability.
    #[serde(rename = "noul")]
    Noul { noul: f64 },
    /// An answer of some other shape, which this version does not read.
    #[serde(other)]
    Other,
}

/// The token counts a response reports.
#[derive(Debug, Deserialize)]
struct ResponseUsage {
    input_tokens: u64,
    output_tokens: u64,
}

/// Write the plan as the request body the backend reads.
///
/// # Errors
///
/// Returns [`EncodeError`] when the body cannot be written as JSON.
pub fn encode(plan: &Plan) -> Result<Vec<u8>, EncodeError> {
    encode_raw(plan).map(|raw| raw.get().as_bytes().to_owned())
}

/// Write the plan as the request body the plan document embeds.
pub(crate) fn encode_raw(plan: &Plan) -> Result<Box<RawValue>, EncodeError> {
    let request = Request {
        state: plan.evidence().as_str().to_owned(),
        model: plan.model().as_str().to_owned(),
        questions: plan
            .questions()
            .iter()
            .enumerate()
            .map(|(place, question)| (wire_name(place), RequestQuestion::asking(question)))
            .collect(),
    };
    serde_json::value::to_raw_value(&request).map_err(|error| EncodeError(error.to_string()))
}

/// Read the response body as one answer per question the plan asked.
///
/// # Errors
///
/// Returns [`DecodeError`] when the body is not a `systemone` response, when it
/// answers a planned question with nothing, when an answer carries the wrong
/// shape, or when a probability falls outside zero to one.
pub fn decode(plan: &Plan, body: &[u8]) -> Result<Reply, DecodeError> {
    let response: Response =
        serde_json::from_slice(body).map_err(|error| DecodeError::Malformed(error.to_string()))?;
    let model = ModelName::new(response.model)
        .map_err(|error| DecodeError::Malformed(error.to_string()))?;
    let mut answers = Vec::with_capacity(plan.questions().len());
    for place in 0..plan.questions().len() {
        let name = wire_name(place);
        let Some(answer) = response.answers.get(&name) else {
            return Err(DecodeError::MissingAnswer(place));
        };
        let ResponseAnswer::Noul { noul } = *answer else {
            return Err(DecodeError::WrongKind(place));
        };
        let probability =
            Probability::new(noul).map_err(|_| DecodeError::ProbabilityOutOfRange(place))?;
        answers.push(Answer::new_yes_no(probability));
    }
    let usage = response
        .usage
        .map(|usage| Usage::new(usage.input_tokens, usage.output_tokens));
    Ok(Reply::new(model, answers, usage))
}

/// The name a question carries on the wire: `q1` onward, in plan order.
fn wire_name(place: usize) -> String {
    format!("q{}", place + 1)
}

impl RequestQuestion {
    /// Write one question in the shape its verb asks for.
    fn asking(question: &Question) -> Self {
        match question.verb() {
            Verb::Decide => Self {
                kind: QuestionKind::Noul,
                instructions: question.text().as_str().to_owned(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DecodeError, Request, decode, encode};
    use crate::plan::Plan;
    use crate::question::Question;
    use crate::result::Usage;
    use crate::text::{Evidence, ModelName, QuestionText};
    use proptest::collection::vec;
    use proptest::prelude::{Strategy, any};
    use proptest::{prop_assert_eq, proptest};

    const REQUEST: &str =
        include_str!("../../../specification/fixtures/systemone/decide-urgent.request.json");
    const RESPONSE: &str =
        include_str!("../../../specification/fixtures/systemone/decide-urgent.response.json");
    const MISSING: &str = include_str!(
        "../../../specification/fixtures/systemone/refused-missing-answer.response.json"
    );
    const WRONG_KIND: &str =
        include_str!("../../../specification/fixtures/systemone/refused-wrong-kind.response.json");
    const OUT_OF_RANGE: &str = include_str!(
        "../../../specification/fixtures/systemone/refused-probability-out-of-range.response.json"
    );

    fn plan_for(state: &str, questions: &[&str]) -> Plan {
        Plan::new(
            Evidence::new(state).expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            questions
                .iter()
                .map(|text| Question::new_decide(QuestionText::new(*text).expect("not blank")))
                .collect(),
        )
        .expect("a question is asked")
    }

    fn urgency_plan() -> Plan {
        plan_for(
            "Help! My payouts have been failing for 3 days.",
            &["Does this convey urgency?"],
        )
    }

    /// Encode one question over one evidence and read the bytes back.
    fn round_tripped(state: &str, instructions: &str) -> Request {
        let bytes = encode(&plan_for(state, &[instructions])).expect("a plan is writable");
        serde_json::from_slice(&bytes).expect("a systemone request")
    }

    #[test]
    fn encode_writes_the_request_the_fixture_shows() {
        let bytes = encode(&urgency_plan()).expect("a plan is writable");
        let written: Request = serde_json::from_slice(&bytes).expect("a systemone request");
        let fixture: Request = serde_json::from_str(REQUEST).expect("a systemone request");
        assert_eq!(written, fixture);
    }

    #[test]
    fn decode_reads_the_answer_the_fixture_shows() {
        let reply = decode(&urgency_plan(), RESPONSE.as_bytes()).expect("a systemone response");
        assert_eq!(reply.model().as_str(), "jev-latest");
        let [answer] = reply.answers() else {
            panic!("one answer per planned question");
        };
        assert!((answer.probability().as_f64() - 0.92).abs() < f64::EPSILON);
        assert_eq!(reply.usage(), Some(Usage::new(312, 48)));
    }

    #[test]
    fn each_refused_response_names_its_own_cause() {
        let cases = [
            (MISSING, DecodeError::MissingAnswer(0)),
            (WRONG_KIND, DecodeError::WrongKind(0)),
            (OUT_OF_RANGE, DecodeError::ProbabilityOutOfRange(0)),
        ];
        for (body, expected) in cases {
            assert!(expected.to_string().contains("`q1`"), "{expected}");
            assert_eq!(
                decode(&urgency_plan(), body.as_bytes()),
                Err(expected.clone()),
                "{expected}"
            );
        }
    }

    #[test]
    fn a_body_that_is_not_a_systemone_response_is_refused() {
        let cases = ["", "not json at all", "[]", r#"{"model":"jev-latest"}"#];
        for body in cases {
            assert!(
                matches!(
                    decode(&urgency_plan(), body.as_bytes()),
                    Err(DecodeError::Malformed(_))
                ),
                "{body:?}"
            );
        }
    }

    #[test]
    fn a_response_that_does_not_name_the_model_is_refused() {
        let answers = r#""answers":{"q1":{"type":"noul","noul":0.5}}}"#;
        let cases = [
            format!("{{{answers}"),
            format!(r#"{{"model":"",{answers}"#),
            format!(r#"{{"model":" \t ",{answers}"#),
        ];
        for body in cases {
            assert!(
                matches!(
                    decode(&urgency_plan(), body.as_bytes()),
                    Err(DecodeError::Malformed(_))
                ),
                "{body}"
            );
        }
    }

    #[test]
    fn the_names_carry_the_order_and_the_key_order_carries_nothing() {
        let plan = plan_for("Help!", &["is urgent", "asks for a refund"]);
        let written: Request = serde_json::from_slice(&encode(&plan).expect("a plan is writable"))
            .expect("a systemone request");
        let named = |name: &str| {
            written
                .questions
                .get(name)
                .expect("one question per name")
                .instructions
                .clone()
        };
        assert_eq!(named("q1"), "is urgent");
        assert_eq!(named("q2"), "asks for a refund");

        let body = concat!(
            r#"{"answers":{"q2":{"type":"noul","noul":0.25},"#,
            r#""q1":{"type":"noul","noul":0.75}},"model":"jev-latest"}"#
        );
        let reply = decode(&plan, body.as_bytes()).expect("a systemone response");
        let [first, second] = reply.answers() else {
            panic!("one answer per planned question");
        };
        assert!((first.probability().as_f64() - 0.75).abs() < f64::EPSILON);
        assert!((second.probability().as_f64() - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn a_response_decodes_without_usage_and_past_unknown_fields() {
        let body = concat!(
            r#"{"model":"jev-1.13.0","request_id":"abc","#,
            r#""answers":{"q1":{"type":"noul","noul":0.5,"rationale":"none"},"q9":{"type":"noul","noul":0.1}}}"#
        );
        let reply = decode(&urgency_plan(), body.as_bytes()).expect("a systemone response");
        assert_eq!(reply.model().as_str(), "jev-1.13.0");
        assert_eq!(reply.answers().len(), 1);
        assert_eq!(reply.usage(), None);
    }

    fn texts() -> impl Strategy<Value = String> {
        vec(any::<char>(), 1..24)
            .prop_map(|chars| chars.into_iter().collect::<String>())
            .prop_filter("text that is not blank", |text| !text.trim().is_empty())
    }

    proptest! {
        #[test]
        fn any_evidence_and_question_reach_the_wire_unchanged(state in texts(), instructions in texts()) {
            let written = round_tripped(&state, &instructions);
            prop_assert_eq!(&written.state, &state);
            let question = written.questions.get("q1").expect("one named question");
            prop_assert_eq!(&question.instructions, &instructions);
        }
    }
}
