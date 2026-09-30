//! The quoted form every request takes, by ADR 0111 section 1: the context,
//! or `QUOTED` without one, as the state, and each record quoted in its own
//! questions. A question written as JSON cannot take the quote, so its record
//! goes alone as the state. The packer in `pack` joins the questions.

use std::num::NonZeroUsize;

use thiserror::Error;

use crate::core::backend_profile::{BackendProfile, LimitKind, ProfileLimit};
use crate::core::json::Json;
use crate::core::plan::{Descriptions, Plan};
use crate::core::question::Question;
use crate::core::render::json_line;
use crate::core::text::{Evidence, ModelName, QuestionText};

/// The state of every quoted request without a context, by ADR 0111.
pub(crate) const QUOTED: &str = "Each question quotes the text it asks about.";

/// How many records a batch may hold: as many as fit, or at most `N`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Setting {
    Max,
    Records(NonZeroUsize),
}

impl Setting {
    /// `max`, or a decimal whole number of at least 1, and nothing else.
    pub(crate) fn parse(text: &str) -> Option<Self> {
        if text == "max" {
            return Some(Self::Max);
        }
        text.bytes()
            .all(|byte| byte.is_ascii_digit())
            .then(|| text.parse().ok().map(Self::Records))
            .flatten()
    }

    /// A question file's `batch`: the string `max` or a whole number of at least 1.
    pub(crate) fn of_json(value: &Json) -> Option<Self> {
        match value {
            Json::String(text) if text == "max" => Some(Self::Max),
            Json::Number(number) => Self::parse(&number.to_string()),
            _ => None,
        }
    }
}

/// One record as a batch reads it: today's evidence for a batch of one, and
/// the JSON value a batch quotes, lists, hashes, and compares for copies.
#[derive(Clone)]
pub(crate) struct BatchRecord {
    pub(crate) evidence: Evidence,
    pub(crate) value: Json,
}

/// Why records could not be planned. No variant holds record or context text.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum BatchError {
    #[error("a request passes a profile limit")]
    Profile(ProfileLimit),
    #[error("a question written as JSON cannot quote a record beside a context")]
    StructuredQuestionWithContext,
    #[error("the context's request passes its limit of {limit} {}: {actual}", .kind.words())]
    ContextOverLimit {
        kind: LimitKind,
        limit: usize,
        actual: usize,
    },
    #[error("{0}")]
    Defect(&'static str),
}

/// These questions with `line`, a record's compact JSON, quoted at the head
/// of each one's text, keeping every option. `None` when a question is
/// written as JSON and cannot take the quote.
fn quote(line: &str, base: &[Question]) -> Result<Option<Vec<Question>>, BatchError> {
    let mut questions = base.to_vec();
    for question in &mut questions {
        let Some(asked) = text(question).as_json().as_str().map(str::to_owned) else {
            return Ok(None);
        };
        *text(question) = QuestionText::new(quoted(line, &asked)).map_err(|_| defect())?;
    }
    Ok(Some(questions))
}

/// One question's text with `line`, a record's compact JSON, quoted at its head.
pub(crate) fn quoted(line: &str, asked: &str) -> String {
    format!("The text is {line}. {asked}")
}

/// Check a quoted record's text against the profile's evidence limit. The
/// request's state is checked when the request is encoded.
fn bounded(profile: Option<&BackendProfile>, record: &Evidence) -> Result<(), BatchError> {
    let Some(profile) = profile else {
        return Ok(());
    };
    let text = record.as_text().map_err(|_| defect())?;
    profile.check_record(&text).map_err(BatchError::Profile)
}

/// The plan one record sends outside a batch, in the same quoted form a
/// batch sends, by ADR 0111 section 1: the context or `QUOTED` as the state,
/// and the record quoted in each question. A question written as JSON takes
/// the record as its state and goes unquoted. A quoted record over the
/// profile's evidence limit is refused.
pub(crate) fn quoted_plan(
    model: (ModelName, Descriptions),
    record: Evidence,
    context: Option<&Evidence>,
    base: Vec<Question>,
    profile: Option<&BackendProfile>,
) -> Result<Plan, BatchError> {
    let value = record.as_json();
    quoted_plan_of(model, record, &value, context, base, profile)
}

/// The same plan with `value` quoted, which for a stream's record is the
/// JSON value a batch has always quoted: a whole JSON record as itself.
pub(crate) fn quoted_plan_of(
    (model, descriptions): (ModelName, Descriptions),
    record: Evidence,
    value: &crate::core::Json,
    context: Option<&Evidence>,
    base: Vec<Question>,
    profile: Option<&BackendProfile>,
) -> Result<Plan, BatchError> {
    let line = json_line(value).map_err(|_| defect())?;
    let (evidence, questions) = match quote(&line, &base)? {
        Some(quoted) => {
            bounded(profile, &record)?;
            let state = match context {
                Some(context) => context.clone(),
                None => Evidence::new(QUOTED).map_err(|_| defect())?,
            };
            (state, quoted)
        }
        None if context.is_some() => return Err(BatchError::StructuredQuestionWithContext),
        None => (record, base),
    };
    Plan::new(evidence, model, descriptions, questions).map_err(|_| defect())
}

fn text(question: &mut Question) -> &mut QuestionText {
    match question {
        Question::Decide { text, .. }
        | Question::Choose { text, .. }
        | Question::Tag { text, .. }
        | Question::Score { text, .. } => text,
    }
}

fn defect() -> BatchError {
    BatchError::Defect("a question could not be quoted")
}

#[cfg(test)]
mod tests;
