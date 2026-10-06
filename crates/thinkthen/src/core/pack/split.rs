//! One live reply split into one wire answer per question it was asked.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::value::RawValue;

use crate::core::adapters::built_in::{self, DecodeError};
use crate::core::question::Question;
use crate::core::result::ReportedUsage;
use crate::core::text::ModelName;

/// One wire answer: its JSON as received, or the error that failed it.
pub(crate) type WireAnswer = Result<String, DecodeError>;

/// A reply's model, its whole-request usage, and one answer per wire question.
pub(crate) struct Split {
    pub(crate) model: ModelName,
    pub(crate) usage: Option<ReportedUsage>,
    pub(crate) answers: Vec<WireAnswer>,
}

impl std::fmt::Debug for Split {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Split")
            .field("usage", &self.usage)
            .field("answers", &self.answers.len())
            .finish_non_exhaustive()
    }
}

/// The answers a reply names, each kept as the bytes it arrived as.
#[derive(Deserialize)]
struct Raw {
    answers: BTreeMap<String, Box<RawValue>>,
}

/// Read a reply to questions each decoded alone by `decoders`, in order.
///
/// # Errors
///
/// Returns the decoder's error, with any usage the reply reported, when the
/// body is no reply or no question has a usable answer. Nothing of such a
/// reply is kept.
pub(crate) fn split(
    decoders: &[Question],
    body: &[u8],
) -> Result<Split, (DecodeError, Option<ReportedUsage>)> {
    let (usage, decoded) = built_in::decode_answers(decoders, body);
    let (model, each) = decoded.map_err(|error| (error, usage))?;
    let raw: Raw = serde_json::from_slice(body)
        .map_err(|error| (DecodeError::Malformed(error.line(), error.column()), usage))?;
    let answers = each
        .into_iter()
        .enumerate()
        .map(|(place, answer)| {
            let _answer = answer?;
            raw.answers
                .get(&built_in::wire_name(place))
                .map(|answer| answer.get().to_owned())
                .ok_or(DecodeError::MissingAnswer(place))
        })
        .collect();
    Ok(Split {
        model,
        usage,
        answers,
    })
}
