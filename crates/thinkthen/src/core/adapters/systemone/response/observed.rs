use crate::core::{Plan, Reply, Usage};

use super::{DecodeError, Response, decode_response};

pub(crate) struct Decoded {
    pub(crate) usage: Option<Usage>,
    pub(crate) reply: Result<Reply, DecodeError>,
}

pub(crate) fn decode_observed(plan: &Plan, body: &[u8]) -> Decoded {
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
    let reply = decode_response(plan, response, usage);
    Decoded { usage, reply }
}
