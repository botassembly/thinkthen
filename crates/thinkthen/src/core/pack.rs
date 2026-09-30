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
use crate::core::result::Usage;
use crate::core::text::{Evidence, Url, Withheld};

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
        }))
    }

    pub(crate) fn json(&self) -> &str {
        &self.0.json
    }

    pub(crate) fn sha256(&self) -> &[u8; 32] {
        &self.0.sha256
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

/// The SHA-256 of the adapter, the address, the model, the state and one
/// question, each as sent and joined by one line feed.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct QuestionKey([u8; 32]);

impl QuestionKey {
    /// Hash the five parts. `model`, `state` and `question` are compact JSON,
    /// so none holds a raw line feed, and the address refuses control bytes.
    pub(crate) fn of(url: &Url, model: &str, state: &str, question: &str) -> Self {
        let mut hasher = Sha256::new();
        for (place, part) in [built_in::NAME, url.as_str(), model, state, question]
            .into_iter()
            .enumerate()
        {
            if place > 0 {
                hasher.update(b"\n");
            }
            hasher.update(part.as_bytes());
        }
        Self(hasher.finalize().into())
    }

    pub(crate) const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The key as 64 lowercase hex figures.
    pub(crate) fn hex(&self) -> String {
        hex(&self.0)
    }

    /// Read 64 lowercase hex figures back.
    pub(crate) fn parse(text: &str) -> Option<Self> {
        let digits = text.as_bytes();
        if digits.len() != 64 {
            return None;
        }
        let figure = |byte: u8| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        let mut bytes = [0; 32];
        for (slot, pair) in bytes.iter_mut().zip(digits.chunks(2)) {
            let [high, low] = pair else { return None };
            *slot = (figure(*high)? << 4) | figure(*low)?;
        }
        Some(Self(bytes))
    }
}

impl fmt::Debug for QuestionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.hex())
    }
}

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
    let state = State::new(parts.state, evidence.len());
    let decoders = plan.questions().iter().flat_map(decoders);
    Ok(parts
        .questions
        .into_iter()
        .zip(decoders)
        .map(|(question, decoder)| Ask {
            key: QuestionKey::of(url, &parts.model, state.json(), &question),
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
pub(crate) fn shares(usage: Option<Usage>, questions: usize) -> Vec<Option<Usage>> {
    (0..questions)
        .map(|position| usage.map(|usage| usage.share(questions, position)))
        .collect()
}

#[cfg(test)]
mod tests;
