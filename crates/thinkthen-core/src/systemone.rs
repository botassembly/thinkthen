//! The `systemone` wire format, as two pure translations of one plan.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
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
    /// The response answered every question but this one.
    #[error("the response carries no answer for question `{0}`")]
    MissingAnswer(String),
    /// The answer holds a shape this question did not ask for.
    #[error("the answer to question `{0}` is not a yes/no answer")]
    WrongKind(String),
    /// The answer holds a number that is not a probability.
    #[error("the answer to question `{0}` holds a probability outside zero to one")]
    ProbabilityOutOfRange(String),
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
    serde_json::to_vec(&request).map_err(|error| EncodeError(error.to_string()))
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
            return Err(DecodeError::MissingAnswer(name));
        };
        let ResponseAnswer::Noul { noul } = *answer else {
            return Err(DecodeError::WrongKind(name));
        };
        let probability =
            Probability::new(noul).map_err(|_| DecodeError::ProbabilityOutOfRange(name))?;
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
            Verb::If => Self {
                kind: QuestionKind::Noul,
                instructions: question.condition().as_str().to_owned(),
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
    use crate::text::{Condition, Evidence, ModelName};
    use proptest::collection::vec;
    use proptest::prelude::{Strategy, any};
    use proptest::{prop_assert_eq, proptest};

    const REQUEST: &str =
        include_str!("../../../specification/fixtures/systemone/if-urgent.request.json");
    const RESPONSE: &str =
        include_str!("../../../specification/fixtures/systemone/if-urgent.response.json");
    const MISSING: &str = include_str!(
        "../../../specification/fixtures/systemone/refused-missing-answer.response.json"
    );
    const WRONG_KIND: &str =
        include_str!("../../../specification/fixtures/systemone/refused-wrong-kind.response.json");
    const OUT_OF_RANGE: &str = include_str!(
        "../../../specification/fixtures/systemone/refused-probability-out-of-range.response.json"
    );

    fn plan_for(state: &str, instructions: &str) -> Plan {
        Plan::new(
            Evidence::new(state).expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            vec![Question::new_if(
                Condition::new(instructions).expect("not blank"),
            )],
        )
        .expect("a question is asked")
    }

    fn urgency_plan() -> Plan {
        plan_for(
            "Help! My payouts have been failing for 3 days.",
            "Does this convey urgency?",
        )
    }

    /// Encode one condition over one evidence and read the bytes back.
    fn round_tripped(state: &str, instructions: &str) -> Request {
        let bytes = encode(&plan_for(state, instructions)).expect("a plan is writable");
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
            (MISSING, DecodeError::MissingAnswer("q1".to_owned())),
            (WRONG_KIND, DecodeError::WrongKind("q1".to_owned())),
            (
                OUT_OF_RANGE,
                DecodeError::ProbabilityOutOfRange("q1".to_owned()),
            ),
        ];
        for (body, expected) in cases {
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

    #[test]
    fn text_that_json_escapes_reaches_the_wire_unchanged() {
        let cases = [
            "a \"quoted\" word",
            r"a back\slash",
            "two\nlines\tapart",
            "a \u{1} control character",
            "καρδία, 心, and 🫀",
            "a \u{2028} line separator",
        ];
        for case in cases {
            let written = round_tripped(case, case);
            assert_eq!(written.state, case, "{case:?}");
            let question = written.questions.get("q1").expect("one named question");
            assert_eq!(question.instructions, case, "{case:?}");
        }
    }

    fn texts() -> impl Strategy<Value = String> {
        vec(any::<char>(), 1..24)
            .prop_map(|chars| chars.into_iter().collect::<String>())
            .prop_filter("text that is not blank", |text| !text.trim().is_empty())
    }

    proptest! {
        #[test]
        fn any_evidence_and_condition_reach_the_wire_unchanged(state in texts(), instructions in texts()) {
            let written = round_tripped(&state, &instructions);
            prop_assert_eq!(&written.state, &state);
            let question = written.questions.get("q1").expect("one named question");
            prop_assert_eq!(&question.instructions, &instructions);
        }
    }
}
