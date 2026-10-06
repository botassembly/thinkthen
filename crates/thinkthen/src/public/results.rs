//! Typed results. Each carries what the command's result documents carry,
//! and `Debug` never prints a caller's input.

use std::fmt;

mod aggregate_reading;
mod call;
mod complete;
mod metadata;
mod reading;
pub use aggregate_reading::{RecognitionReading, RelationReading, ResolvedRelationRule};
pub use metadata::{BatchMismatch, ProfileMismatch, ResultMetadata};
pub use reading::{
    FindReading, QuestionContent, ResolvedOption, ResolvedQuestion, ResolvedThreshold,
};
mod complete_annotation;
mod complete_facts;
mod complete_find;
pub use complete_facts::CompleteFacts;
mod complete_recognize;
mod complete_relate;
pub use complete_relate::{CompleteRelated, CompleteRelationMember};
mod complete_record;
pub use complete::{
    CompleteChoice, CompleteDecision, CompleteFilter, CompleteRank, CompleteScore, CompleteTags,
};
pub use complete_annotation::{CompleteAnnotated, CompleteAnnotationMember};
pub use complete_find::CompleteFound;
pub use complete_recognize::{
    CompleteRecognized, NameProbabilities, PairProbability, PieceProbabilities,
    RecognitionProbabilities,
};
pub use complete_record::CompleteRecord;
mod found;
pub use crate::core::{AttemptObservation, AttemptOutcome, CompleteAttempt};
pub use call::{Call, DoorReply, Facts};
pub use found::{Candidate, Found, Picked};
mod ranked;
pub use ranked::{Ranked, RankedRow};
mod set_ranked;
pub use set_ranked::SetRanked;
mod tally;
pub use tally::{Tally, TallyStart};
mod member;
pub(crate) use member::Member;
mod observation;
#[cfg(test)]
pub(crate) use observation::QuestionJson;
pub(crate) use observation::{ObservedQuestion, observe_question};
pub use observation::{ObservedRow, QuestionDetail, RecordObservation};

use serde::Serialize;

use crate::core::{self, Backend, BackendProfile, ProfileWarning, Value, Withheld, json_line};
use crate::engine::facade;
use crate::public::error::Error;
use crate::public::question::Question;
use crate::result_json::{Run, decision};

/// A result's JSON line, written once when the result is made. `Debug`
/// withholds it, because it holds the question and the names.
#[derive(Clone, PartialEq)]
pub(crate) struct Written(String);

impl Written {
    pub(crate) fn of(value: &impl Serialize) -> Result<Self, Error> {
        json_line(value).map(Self).map_err(|_| written())
    }

    pub(crate) fn text(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Debug for Written {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Withheld(self.0.len()).fmt(formatter)
    }
}

fn written() -> Error {
    Error::defect("a result could not be written as JSON")
}

/// A yes or no answer, or unsure when a band leaves it between its sides.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Answer {
    /// The question holds.
    Yes,
    /// The question does not hold.
    No,
    /// The probability fell inside the band.
    Unsure,
}

/// The value one judgment reads as, under the question's rule.
#[derive(Clone, Debug, PartialEq)]
pub enum Judgment {
    /// A `decide` answer.
    Decision(Answer),
    /// A `choose` pick, or `None` when the answer is not sure.
    Choice(Option<String>),
    /// A `score` position on the levels.
    Score(f64),
    /// The `tag` labels that reached the cut.
    Tags(Vec<String>),
}

/// One label and its probability.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedProbability {
    name: String,
    probability: f64,
}

impl NamedProbability {
    /// The option, label, or level.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Its probability.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }
}

/// The probabilities behind one judgment.
#[derive(Clone, Debug, PartialEq)]
pub enum Probabilities {
    /// The probability of yes.
    YesNo {
        /// The probability of yes.
        yes: f64,
    },
    /// Each option, label, or level in declared order.
    Named(Vec<NamedProbability>),
}

/// The token counts the backend reported for one result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Usage {
    input_tokens: u64,
    output_tokens: u64,
}

impl Usage {
    /// Tokens the backend read.
    #[must_use]
    pub fn input_tokens(&self) -> u64 {
        self.input_tokens
    }

    /// Tokens the backend wrote.
    #[must_use]
    pub fn output_tokens(&self) -> u64 {
        self.output_tokens
    }
}

/// One engine's totals since it was built, or since the process was forked.
/// Failed calls and retries count, and nothing resets them. Add the totals of
/// several engines with `+` or `Sum`, which saturate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "usage"))]
pub struct Counters {
    requests_sent: u64,
    retries: u64,
    input_tokens: u64,
    output_tokens: u64,
    cache_answers: u64,
}

impl Counters {
    /// No sends, retries, cache answers or tokens.
    pub const ZERO: Self = Self {
        requests_sent: 0,
        retries: 0,
        cache_answers: 0,
        input_tokens: 0,
        output_tokens: 0,
    };

    pub(crate) const fn of(counts: &crate::engine::usage::Counts) -> Self {
        Self {
            requests_sent: counts.requests_sent,
            retries: counts.retries,
            cache_answers: counts.cache_answers,
            input_tokens: counts.input_tokens,
            output_tokens: counts.output_tokens,
        }
    }

    /// Live attempts sent.
    #[must_use]
    pub fn requests_sent(&self) -> u64 {
        self.requests_sent
    }

    /// Live attempts that retried after a retriable status; a subset of sent requests.
    #[must_use]
    pub fn retries(&self) -> u64 {
        self.retries
    }

    /// Answers the cache gave without a send.
    #[must_use]
    pub fn cache_answers(&self) -> u64 {
        self.cache_answers
    }

    /// Tokens the backend reported reading.
    #[must_use]
    pub fn input_tokens(&self) -> u64 {
        self.input_tokens
    }

    /// Tokens the backend reported writing.
    #[must_use]
    pub fn output_tokens(&self) -> u64 {
        self.output_tokens
    }
}

/// One judgment with the probabilities and request facts behind it.
#[derive(Clone, PartialEq)]
pub struct Details {
    sources: Vec<core::QuestionSource>,
    observations: Vec<core::Observation>,
    value: Judgment,
    probabilities: Probabilities,
    nearest: Option<String>,
    model: String,
    question_sha256: String,
    profile_warning: Option<ProfileWarning>,
    requests: Vec<String>,
    requests_sent: u64,
    cached: bool,
    usage: Option<Usage>,
    reported_usage: Option<core::ReportedUsage>,
    confidence: Option<f64>,
    url: String,
    json: Written,
    scalar_json: Option<Written>,
}

impl Details {
    pub(crate) fn of(
        judged: &facade::Judgment,
        question: &Question,
        backend: &Backend,
        profile: Option<&BackendProfile>,
        requests: Vec<String>,
    ) -> Result<Self, Error> {
        let answer = &judged.answer;
        let probabilities = match (answer.yes(), answer.named()) {
            (Some(yes), _) => Probabilities::YesNo { yes },
            (None, Some(named)) => Probabilities::Named(
                named
                    .into_iter()
                    .map(|(name, probability)| NamedProbability {
                        name: name.to_owned(),
                        probability,
                    })
                    .collect(),
            ),
            (None, None) => return Err(Error::defect("an answer carried no probability")),
        };
        let reply = &judged.answered.reply;
        let warning =
            ProfileWarning::between(question.profile.as_ref(), profile.map(BackendProfile::name));
        let run = Run {
            backend,
            tuned_for: question.profile.as_ref(),
            warning: warning.clone(),
            batch_setting: None,
            batch_warning: None,
            context_sha256: None,
        };
        let (json, question_sha256) = decision(
            run,
            judged,
            question.core.clone(),
            question.threshold,
            judged.value.clone(),
            None,
            requests.clone(),
        )
        .map_err(|_| written())?;
        Ok(Self {
            sources: judged.answered.sources.clone(),
            observations: judged.answered.observations.clone(),
            value: judgment(&judged.value),
            probabilities,
            nearest: answer.level().map(str::to_owned),
            model: reply.model().as_str().to_owned(),
            question_sha256,
            profile_warning: warning,
            requests,
            requests_sent: judged.answered.requests_sent,
            cached: judged.answered.replayed,
            usage: reply.usage().map(usage),
            reported_usage: reply.reported_usage(),
            confidence: answer.confidence().map(|held| held.as_f64()),
            url: backend.url().as_str().to_owned(),
            json: Written(json),
            scalar_json: None,
        })
    }

    /// The `thinkthen.result/1` line that `--details` prints for this one text.
    #[must_use]
    pub fn to_json(&self) -> String {
        self.json.text()
    }

    /// Actual sources and original wire-question counts in question order.
    #[must_use]
    pub fn question_sources(&self) -> &[core::QuestionSource] {
        &self.sources
    }

    /// Accepted observation or failed occurrence identities in question order.
    #[must_use]
    pub fn observations(&self) -> &[core::Observation] {
        &self.observations
    }

    /// Independently reported counts, retaining unknown input or output.
    #[must_use]
    pub const fn reported_usage(&self) -> Option<core::ReportedUsage> {
        self.reported_usage
    }

    /// The value under the question's rule.
    #[must_use]
    pub fn value(&self) -> &Judgment {
        &self.value
    }

    /// The probabilities the value was read from.
    #[must_use]
    pub fn probabilities(&self) -> &Probabilities {
        &self.probabilities
    }

    /// The level a `score` leads with; `None` for every other kind.
    #[must_use]
    pub fn nearest(&self) -> Option<&str> {
        self.nearest.as_deref()
    }

    /// The model the backend said answered.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// The digest of the question and its rule, as `--details` prints it.
    #[must_use]
    pub fn question_sha256(&self) -> &str {
        &self.question_sha256
    }

    /// Saved and selected runtime profile names when both exist and differ.
    #[must_use]
    pub fn profile_warning(&self) -> Option<(&str, &str)> {
        self.profile_warning
            .as_ref()
            .map(|warning| (warning.tuned_for(), warning.running()))
    }

    /// The question key of each answer behind the result, by ADR 0111.
    #[must_use]
    pub fn requests(&self) -> &[String] {
        &self.requests
    }

    /// Attempts sent for this result, retries included.
    #[must_use]
    pub fn requests_sent(&self) -> u64 {
        self.requests_sent
    }

    /// True when the cache or a recording answered.
    #[must_use]
    pub fn cached(&self) -> bool {
        self.cached
    }

    /// The token counts the backend reported, when it reported them.
    #[must_use]
    pub fn usage(&self) -> Option<&Usage> {
        self.usage.as_ref()
    }

    /// The backend's own confidence in a choice or score, when it sent one.
    #[must_use]
    pub fn confidence(&self) -> Option<f64> {
        self.confidence
    }

    /// The address that answered.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Questions that failed inside the result. A single judgment has none.
    #[must_use]
    pub fn failed_questions(&self) -> usize {
        0
    }
}

pub(crate) const fn answer(value: &Value) -> Answer {
    match value {
        Value::YesNo(Some(true)) => Answer::Yes,
        Value::YesNo(Some(false)) => Answer::No,
        _ => Answer::Unsure,
    }
}

pub(crate) fn judgment(value: &Value) -> Judgment {
    match value {
        Value::YesNo(_) => Judgment::Decision(answer(value)),
        Value::Choice(label) => Judgment::Choice(label.clone()),
        Value::Score(position) => Judgment::Score(*position),
        Value::Tag(labels) => Judgment::Tags(labels.clone()),
    }
}

pub(crate) const fn usage(counts: core::Usage) -> Usage {
    let (input_tokens, output_tokens) = counts.token_counts();
    Usage {
        input_tokens,
        output_tokens,
    }
}

/// Write a result holding a caller's input, withholding the input.
macro_rules! withheld_debug {
    ($name:ident<$($generic:ident),+> { $($field:ident),* }) => {
        impl<$($generic),+> fmt::Debug for $name<$($generic),+> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    $(.field(stringify!($field), &self.$field))*
                    .finish_non_exhaustive()
            }
        }
    };
}

/// One `decide_many` record and its answer.
#[derive(Clone, PartialEq)]
pub struct Row<T, V> {
    input: T,
    value: V,
    probability: f64,
}

impl<T, V> fmt::Debug for Row<T, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Row").finish_non_exhaustive()
    }
}

impl<T, V> Row<T, V> {
    pub(crate) const fn new(input: T, value: V, probability: f64) -> Self {
        Self {
            input,
            value,
            probability,
        }
    }

    /// The record as given.
    #[must_use]
    pub fn input(&self) -> &T {
        &self.input
    }

    /// Its answer.
    #[must_use]
    pub fn value(&self) -> &V {
        &self.value
    }

    /// The record and its answer.
    #[must_use]
    pub fn into_parts(self) -> (T, V) {
        (self.input, self.value)
    }
}

impl<T> Row<T, Answer> {
    /// The probability of yes the answer was read from.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }
}

pub(super) use withheld_debug;
