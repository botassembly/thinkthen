//! Typed results. Each carries what the command's result documents carry,
//! and `Debug` never prints a caller's input.

use std::fmt;

use crate::core::{
    self, AnnotatedValue, BackendFailureCause, Threshold, Value, question_sha256_with_profile,
};
use crate::engine::facade;
use crate::public::error::{Error, ErrorKind};

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
    /// A `choose` pick, or `None` when the pick is unresolved.
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
    pub const fn probability(&self) -> f64 {
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Usage {
    input_tokens: u64,
    output_tokens: u64,
}

impl Usage {
    /// Tokens the backend read.
    #[must_use]
    pub const fn input_tokens(&self) -> u64 {
        self.input_tokens
    }

    /// Tokens the backend wrote.
    #[must_use]
    pub const fn output_tokens(&self) -> u64 {
        self.output_tokens
    }
}

/// This process's totals since it began, or since it was forked. Failed calls
/// and retries count, and nothing resets them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Counters {
    requests_sent: u64,
    cache_answers: u64,
    input_tokens: u64,
    output_tokens: u64,
}

impl Counters {
    pub(crate) const ZERO: Self = Self {
        requests_sent: 0,
        cache_answers: 0,
        input_tokens: 0,
        output_tokens: 0,
    };

    pub(crate) const fn of(counts: &crate::engine::usage::Counts) -> Self {
        Self {
            requests_sent: counts.requests_sent,
            cache_answers: counts.cache_answers,
            input_tokens: counts.input_tokens,
            output_tokens: counts.output_tokens,
        }
    }

    /// Live attempts sent.
    #[must_use]
    pub const fn requests_sent(&self) -> u64 {
        self.requests_sent
    }

    /// Answers the cache gave without a send.
    #[must_use]
    pub const fn cache_answers(&self) -> u64 {
        self.cache_answers
    }

    /// Tokens the backend reported reading.
    #[must_use]
    pub const fn input_tokens(&self) -> u64 {
        self.input_tokens
    }

    /// Tokens the backend reported writing.
    #[must_use]
    pub const fn output_tokens(&self) -> u64 {
        self.output_tokens
    }
}

/// One judgment with the probabilities and request facts behind it.
#[derive(Clone, Debug, PartialEq)]
pub struct Details {
    value: Judgment,
    probabilities: Probabilities,
    nearest: Option<String>,
    model: String,
    question_sha256: String,
    requests: Vec<String>,
    requests_sent: u64,
    cached: bool,
    usage: Option<Usage>,
}

impl Details {
    pub(crate) fn of(
        judged: &facade::Judgment,
        question: &core::Question,
        threshold: Option<Threshold>,
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
        Ok(Self {
            value: judgment(&judged.value),
            probabilities,
            nearest: answer.level().map(str::to_owned),
            model: reply.model().as_str().to_owned(),
            question_sha256: question_sha256_with_profile(question, threshold, None)
                .map_err(|_| Error::defect("a question could not be digested"))?,
            requests: vec![judged.answered.request.as_str().to_owned()],
            requests_sent: judged.answered.requests_sent,
            cached: judged.answered.replayed,
            usage: reply.usage().map(usage),
        })
    }

    /// The value under the question's rule.
    #[must_use]
    pub const fn value(&self) -> &Judgment {
        &self.value
    }

    /// The probabilities the value was read from.
    #[must_use]
    pub const fn probabilities(&self) -> &Probabilities {
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

    /// The recording digest of each request behind the result.
    #[must_use]
    pub fn requests(&self) -> &[String] {
        &self.requests
    }

    /// Attempts sent for this result, retries included.
    #[must_use]
    pub const fn requests_sent(&self) -> u64 {
        self.requests_sent
    }

    /// True when the cache or a recording answered.
    #[must_use]
    pub const fn cached(&self) -> bool {
        self.cached
    }

    /// The token counts the backend reported, when it reported them.
    #[must_use]
    pub const fn usage(&self) -> Option<&Usage> {
        self.usage.as_ref()
    }

    /// Questions that failed inside the result. A single judgment has none.
    #[must_use]
    pub const fn failed_questions(&self) -> usize {
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

const fn usage(counts: core::Usage) -> Usage {
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
}

impl<T, V> fmt::Debug for Row<T, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Row").finish_non_exhaustive()
    }
}

impl<T, V> Row<T, V> {
    pub(crate) const fn new(input: T, value: V) -> Self {
        Self { input, value }
    }

    /// The record as given.
    #[must_use]
    pub const fn input(&self) -> &T {
        &self.input
    }

    /// Its answer.
    #[must_use]
    pub const fn value(&self) -> &V {
        &self.value
    }

    /// The record and its answer.
    #[must_use]
    pub fn into_parts(self) -> (T, V) {
        (self.input, self.value)
    }
}

/// One ranked record and its probability of yes.
#[derive(Clone, PartialEq)]
pub struct Ranked<T> {
    input: T,
    probability: f64,
}

withheld_debug!(Ranked<T> { probability });

impl<T> Ranked<T> {
    pub(crate) const fn new(input: T, probability: f64) -> Self {
        Self { input, probability }
    }

    /// The record as given.
    #[must_use]
    pub const fn input(&self) -> &T {
        &self.input
    }

    /// Its probability of yes.
    #[must_use]
    pub const fn probability(&self) -> f64 {
        self.probability
    }

    /// The record.
    #[must_use]
    pub fn into_input(self) -> T {
        self.input
    }
}

/// One `find` unit, or the synthetic `none`, and its probability.
#[derive(Clone, PartialEq)]
pub struct Candidate<T> {
    input: Option<T>,
    probability: f64,
}

withheld_debug!(Candidate<T> { probability });

impl<T> Candidate<T> {
    /// The unit as given, or `None` for the synthetic `none`.
    #[must_use]
    pub const fn input(&self) -> Option<&T> {
        self.input.as_ref()
    }

    /// Its probability.
    #[must_use]
    pub const fn probability(&self) -> f64 {
        self.probability
    }

    /// True for the synthetic `none` candidate.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.input.is_none()
    }
}

/// Every `find` candidate in input order and the one selected.
#[derive(Clone, PartialEq)]
pub struct Found<T> {
    candidates: Vec<Candidate<T>>,
    selected: Option<usize>,
}

withheld_debug!(Found<T> { selected });

impl<T> Found<T> {
    pub(crate) fn new(units: Vec<T>, found: &facade::Found) -> Result<Self, Error> {
        let probabilities = found.selection.probabilities();
        if probabilities.len() != units.len() {
            return Err(Error::defect("a find answer did not cover its units"));
        }
        let candidates = units
            .into_iter()
            .zip(probabilities)
            .map(|(input, (_, probability))| Candidate {
                input: Some(input),
                probability: *probability,
            })
            .collect();
        Ok(Self {
            candidates,
            selected: found.selection.selected(),
        })
    }

    /// The selected unit, or `None` when nothing was selected.
    #[must_use]
    pub fn selected(&self) -> Option<&T> {
        self.candidates.get(self.selected?)?.input()
    }

    /// Every candidate, in input order.
    #[must_use]
    pub fn candidates(&self) -> &[Candidate<T>] {
        &self.candidates
    }

    /// The selected unit.
    #[must_use]
    pub fn into_selected(self) -> Option<T> {
        self.candidates.into_iter().nth(self.selected?)?.input
    }
}

/// One named value of an annotated record.
#[derive(Clone, Debug, PartialEq)]
pub enum Annotated {
    /// A `decide` answer.
    Decision(Answer),
    /// A `choose` pick, or `None` when unresolved.
    Choice(Option<String>),
    /// A `score` position.
    Score(f64),
    /// The `tag` labels that reached the cut.
    Tags(Vec<String>),
    /// The backend failed this one question.
    Failed(Failed),
}

/// One question of a set and its value.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedAnnotation {
    name: String,
    value: Annotated,
}

impl NamedAnnotation {
    /// The member name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Its value.
    #[must_use]
    pub const fn value(&self) -> &Annotated {
        &self.value
    }
}

/// One record and every value the set gave it, in set order.
#[derive(Clone, PartialEq)]
pub struct AnnotatedRecord<T> {
    input: T,
    values: Vec<NamedAnnotation>,
}

withheld_debug!(AnnotatedRecord<T> { values });

impl<T> AnnotatedRecord<T> {
    pub(crate) fn new(input: T, values: Vec<(String, AnnotatedValue)>) -> Self {
        let values = values
            .into_iter()
            .map(|(name, value)| NamedAnnotation {
                name,
                value: match value {
                    AnnotatedValue::Answered(Value::YesNo(held)) => {
                        Annotated::Decision(answer(&Value::YesNo(held)))
                    }
                    AnnotatedValue::Answered(Value::Choice(label)) => Annotated::Choice(label),
                    AnnotatedValue::Answered(Value::Score(position)) => Annotated::Score(position),
                    AnnotatedValue::Answered(Value::Tag(labels)) => Annotated::Tags(labels),
                    AnnotatedValue::Failed(failed) => {
                        Annotated::Failed(Failed(cause(failed.cause())))
                    }
                },
            })
            .collect();
        Self { input, values }
    }

    /// The record as given.
    #[must_use]
    pub const fn input(&self) -> &T {
        &self.input
    }

    /// Each member's value, in set order.
    #[must_use]
    pub fn values(&self) -> &[NamedAnnotation] {
        &self.values
    }

    /// The record.
    #[must_use]
    pub fn into_input(self) -> T {
        self.input
    }
}

/// One question the backend failed inside an otherwise answered record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Failed(FailureCause);

impl Failed {
    /// Always [`ErrorKind::Backend`].
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        ErrorKind::Backend
    }

    /// What the backend's answer broke.
    #[must_use]
    pub const fn cause(&self) -> FailureCause {
        self.0
    }
}

/// Why the backend's answer to one question could not be read.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureCause {
    /// The reply omitted the answer.
    MissingAnswer,
    /// The reply answered another kind of question.
    WrongKind,
    /// An option or level had no probability.
    MissingProbability,
    /// A probability fell outside zero to one.
    InvalidProbability,
    /// A distribution did not total one.
    InvalidDistribution,
    /// A distribution named an option that was not sent.
    UnexpectedProbability,
}

const fn cause(cause: BackendFailureCause) -> FailureCause {
    match cause {
        BackendFailureCause::MissingAnswer => FailureCause::MissingAnswer,
        BackendFailureCause::WrongKind => FailureCause::WrongKind,
        BackendFailureCause::MissingProbability => FailureCause::MissingProbability,
        BackendFailureCause::InvalidProbability => FailureCause::InvalidProbability,
        BackendFailureCause::InvalidDistribution => FailureCause::InvalidDistribution,
        BackendFailureCause::UnexpectedProbability => FailureCause::UnexpectedProbability,
    }
}
