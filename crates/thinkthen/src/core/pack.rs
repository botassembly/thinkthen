//! One backend question as its own cache entry, by ADR 0111 sections 2 and 4.
//!
//! An [`Ask`] is one wire question, the state its request shares, and its
//! [`QuestionKey`]. The key hashes the bytes a request carries, and a request
//! body joins those same bytes, so a request's size is a sum. `packer.rs`
//! gathers the asks a cache lacks into requests, and [`read`] turns stored or
//! live wire answers back into one answer per logical question.

use std::fmt;
use std::sync::Arc;

use sha2::{Digest as _, Sha256};

use crate::core::adapters::built_in::{self, DecodeError, EncodeError};
use crate::core::digest::hex;
use crate::core::plan::Plan;
use crate::core::question::Question;
use crate::core::reply::{AnswerOutcome, BackendFailure, BackendFailureCause};
use crate::core::result::ReportedUsage;
use crate::core::text::{Evidence, Url, Withheld};

mod context;
mod packer;
mod split;

pub(crate) use packer::{Entry, PackError, PackLimits, Packer};
pub(crate) use split::{Split, split};

/// The state one request shares, as the body carries it, and its SHA-256.
#[derive(Clone)]
pub(crate) struct State(Arc<Shared>);

struct Shared {
    json: String,
    sha256: [u8; 32],
    evidence_bytes: usize,
    image_wire: Option<built_in::images::ImageWire>,
}

impl State {
    /// Take the compact JSON a body carries after `"state":`, and the byte
    /// length of its text form, which a profile's evidence limit counts.
    pub(crate) fn new(json: String, evidence_bytes: usize) -> Self {
        let sha256 = Sha256::digest(json.as_bytes()).into();
        Self(Arc::new(Shared {
            json,
            sha256,
            evidence_bytes,
            image_wire: None,
        }))
    }

    /// Typed image constituents enter the existing state hash, while the
    /// separately admitted wire state stays specific to this fixed route.
    pub(crate) fn images(
        input: &crate::core::image::ImageState,
        route: built_in::images::ImageRoute,
        model: &str,
        profile: Option<&crate::core::BackendProfile>,
    ) -> Result<Self, EncodeError> {
        let wire = route
            .admit_profiled(model, input, profile)
            .map_err(|error| EncodeError::of(&error))?;
        let json = serde_json::to_string(input).map_err(|error| EncodeError::of(&error))?;
        let evidence_bytes = crate::core::render::json_line(&input.text)
            .map_err(|error| EncodeError::of(&error))?
            .len();
        let sha256 = Self::image_sha256(&json);
        Ok(Self(Arc::new(Shared {
            json,
            sha256,
            evidence_bytes,
            image_wire: Some(wire),
        })))
    }

    pub(crate) fn body<'a>(
        &self,
        model: &str,
        questions: impl Iterator<Item = &'a str>,
    ) -> Vec<u8> {
        let state = self
            .0
            .image_wire
            .as_ref()
            .map_or(self.json(), |wire| &wire.state);
        let mut body = built_in::join(state, model, questions);
        if let Some(images) = self
            .0
            .image_wire
            .as_ref()
            .and_then(|wire| wire.images.as_ref())
        {
            body.pop();
            body.extend_from_slice(b",\"images\":");
            body.extend_from_slice(images.as_bytes());
            body.push(b'}');
        }
        body
    }

    pub(crate) fn base_bytes(&self, model: &str) -> usize {
        let wire = self.0.image_wire.as_ref();
        wire.map_or(self.json().len(), |wire| wire.state.len())
            + model.len()
            + 34
            + wire
                .and_then(|wire| wire.images.as_ref())
                .map_or(0, |images| images.len() + 10)
    }

    pub(crate) fn body_limit(&self) -> Option<usize> {
        self.0.image_wire.as_ref().map(|wire| wire.body_limit)
    }
    pub(crate) fn questions_limit(&self) -> Option<usize> {
        self.0
            .image_wire
            .as_ref()
            .and_then(|wire| wire.questions_limit)
    }

    pub(crate) fn estimated_tokens(&self, model: &str, questions: &[Arc<str>]) -> Option<u64> {
        let wire = self.0.image_wire.as_ref()?;
        if !wire.image_tokens_known {
            return None;
        }
        let text_bytes = self
            .evidence_bytes()
            .checked_add(model.len())?
            .checked_add(34)?;
        let text_bytes = questions.iter().try_fold(text_bytes, |sum, question| {
            sum.checked_add(question.len() + 32)
        })?;
        let text = crate::core::PlanSummary::estimated_input_high(u64::try_from(text_bytes).ok()?)?;
        text.checked_add(
            wire.tokens_per_question
                .checked_mul(u64::try_from(questions.len()).ok()?)?,
        )
    }

    pub(crate) fn json(&self) -> &str {
        &self.0.json
    }

    pub(crate) fn sha256(&self) -> &[u8; 32] {
        &self.0.sha256
    }

    /// Images occupy a separate hash domain from ordinary JSON evidence.
    pub(crate) fn image_sha256(json: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"thinkthen.image-state/1\n");
        hasher.update(json.as_bytes());
        hasher.finalize().into()
    }

    pub(crate) fn key(&self, url: &Url, model: &str, question: &str) -> QuestionKey {
        QuestionKey::complete(url, model, model, self.json(), question)
    }

    pub(crate) fn evidence_bytes(&self) -> usize {
        self.0.evidence_bytes
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.sha256() == other.sha256()
    }
}

impl Eq for State {}

impl fmt::Debug for State {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("State")
            .field("sha256", &hex(self.sha256()))
            .field("json", &Withheld(self.json().len()))
            .finish()
    }
}

mod key;
pub(crate) use key::QuestionKey;

/// One wire question: its request's state, its bytes as sent, the logical
/// question that reads its answer alone, and its key.
#[derive(Clone)]
pub(crate) struct Ask {
    pub(crate) state: State,
    pub(crate) question: Arc<str>,
    pub(crate) decoder: Question,
    pub(crate) key: QuestionKey,
}

impl fmt::Debug for Ask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Ask")
            .field("key", &self.key)
            .field("question", &Withheld(self.question.len()))
            .finish_non_exhaustive()
    }
}

/// The model part every key of one backend carries: its compact JSON string.
pub(crate) fn model_json(model: &str) -> Result<String, EncodeError> {
    serde_json::to_string(model).map_err(|error| EncodeError::of(&error))
}

/// The state a request of this evidence carries, as the body writes it.
pub(crate) fn state(evidence: &Evidence) -> Result<State, EncodeError> {
    let json =
        serde_json::to_string(&evidence.as_json()).map_err(|error| EncodeError::of(&error))?;
    let text = evidence
        .as_text()
        .map_err(|error| EncodeError::of(&error))?;
    Ok(State::new(json, text.len()))
}

/// The wire questions one plan sends, in order, each with its key.
pub(crate) fn asks(url: &Url, plan: &Plan) -> Result<Vec<Ask>, EncodeError> {
    let parts = built_in::parts(plan)?;
    let evidence = plan
        .evidence()
        .as_text()
        .map_err(|error| EncodeError::of(&error))?;
    let state = match plan.images() {
        Some(images) => State::images(
            images,
            plan.image_route(),
            plan.model().as_str(),
            plan.image_profile(),
        )?,
        None => State::new(parts.state, evidence.len()),
    };
    let decoders = plan.questions().iter().flat_map(decoders);
    Ok(parts
        .questions
        .into_iter()
        .zip(decoders)
        .map(|(question, decoder)| Ask {
            key: state.key(url, &parts.model, &question),
            state: state.clone(),
            question: Arc::from(question),
            decoder,
        })
        .collect())
}

/// The logical question that reads each of this question's wire answers
/// alone: a yes/no question per tag label, and the question itself otherwise.
fn decoders(question: &Question) -> Vec<Question> {
    match question {
        Question::Tag { text, labels } => (0..labels.count())
            .map(|_| Question::Decide {
                text: text.clone(),
                yes: None,
                no: None,
            })
            .collect(),
        other => vec![other.clone()],
    }
}

/// How many wire questions one logical question sends.
pub(crate) fn wire_count(question: &Question) -> usize {
    match question {
        Question::Tag { labels, .. } => labels.count(),
        _ => 1,
    }
}

/// One wire answer as a store keeps it: the answer's JSON as received, or the
/// cause that failed it.
pub(crate) type Stored<'a> = Result<&'a str, BackendFailureCause>;

/// One outcome per logical question from its wire answers, in order. A
/// logical question fails with its first failed wire answer's cause.
pub(crate) fn read(
    questions: &[Question],
    answers: &[Stored<'_>],
    model: &str,
) -> Result<Vec<AnswerOutcome>, DecodeError> {
    let mut rest = answers;
    let mut outcomes = Vec::with_capacity(questions.len());
    for question in questions {
        let (own, after) = rest
            .split_at_checked(wire_count(question))
            .ok_or(DecodeError::MissingAnswer(answers.len()))?;
        rest = after;
        if let Some(cause) = own.iter().find_map(|answer| answer.err()) {
            outcomes.push(AnswerOutcome::Failed(BackendFailure::new(cause)));
            continue;
        }
        let body = synthetic(model, own.iter().filter_map(|answer| answer.ok()))?;
        let decoded = built_in::decode_questions(std::slice::from_ref(question), &body).reply;
        match decoded?.outcomes() {
            [outcome] => outcomes.push(outcome.clone()),
            _ => return Err(DecodeError::UnexpectedAnswer),
        }
    }
    Ok(outcomes)
}

/// A response body carrying these answers as `q1` onward.
fn synthetic<'a>(
    model: &str,
    answers: impl Iterator<Item = &'a str>,
) -> Result<Vec<u8>, DecodeError> {
    let model = serde_json::to_string(model).map_err(|_| DecodeError::NoModel)?;
    let mut body = format!("{{\"model\":{model},\"answers\":{{");
    for (place, answer) in answers.enumerate() {
        if place > 0 {
            body.push(',');
        }
        body.push_str(&format!("\"{}\":{answer}", built_in::wire_name(place)));
    }
    body.push_str("}}");
    Ok(body.into_bytes())
}

/// Each question's even share of a request's usage, the remainder to the
/// earliest, by ADR 0111 section 3.
pub(crate) fn shares(usage: Option<ReportedUsage>, questions: usize) -> Vec<Option<ReportedUsage>> {
    (0..questions)
        .map(|position| usage.map(|usage| usage.share(questions, position)))
        .collect()
}

#[cfg(test)]
mod tests;
