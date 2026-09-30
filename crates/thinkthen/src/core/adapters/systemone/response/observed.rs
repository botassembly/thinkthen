use crate::core::{Plan, Question, Reply, Usage};

use super::{DecodeError, Response, decode_each, decode_response};

pub(crate) struct Decoded {
    pub(crate) usage: Option<Usage>,
    pub(crate) reply: Result<Reply, DecodeError>,
}

pub(crate) fn decode_observed(plan: &Plan, body: &[u8]) -> Decoded {
    decode_questions(plan.questions(), body)
}

pub(crate) fn decode_questions(questions: &[Question], body: &[u8]) -> Decoded {
    let response: Response = match serde_json::from_slice(body) {
        Ok(response) => response,
        Err(error) => {
            return Decoded {
                usage: None,
                reply: Err(DecodeError::Malformed(error.line(), error.column())),
            };
        }
    };
    let usage = response
        .usage
        .as_ref()
        .map(|usage| Usage::new(usage.input_tokens, usage.output_tokens));
    let reply = decode_response(questions, response, usage);
    Decoded { usage, reply }
}

/// Each question's answer or the error that failed it, and the reply's
/// usage. The reply is refused whole as [`decode_questions`] refuses it.
pub(crate) fn decode_answers(
    questions: &[Question],
    body: &[u8],
) -> (Option<Usage>, Result<super::EachAnswer, DecodeError>) {
    let response: Response = match serde_json::from_slice(body) {
        Ok(response) => response,
        Err(error) => {
            return (
                None,
                Err(DecodeError::Malformed(error.line(), error.column())),
            );
        }
    };
    let usage = response
        .usage
        .as_ref()
        .map(|usage| Usage::new(usage.input_tokens, usage.output_tokens));
    (usage, decode_each(questions, &response))
}
