//! The public Rust API over the private engine facade.
//!
//! The free functions call one lazily built process engine, the one
//! [`default_engine`] returns, built by [`Engine::from_env`].

use std::sync::OnceLock;

mod annotated;
mod asking;
mod batch;
mod builders;
mod bulk;
mod choice;
mod engine;
mod error;
#[cfg(feature = "polars")]
mod frame;
mod native_batch;
mod options;
mod panic;
mod plan;
mod pull;
mod question;
mod recognize;
mod relate;
mod results;
mod set;
mod settings;

pub use crate::core::settings::{For, Settings, SettingsError};
pub use annotated::{Annotated, AnnotatedRecord, Failed, FailureCause, NamedAnnotation};
pub use batch::Batch;
pub use builders::{ChooseBuilder, DecideBuilder, LabelBuilder, ScoreBuilder, TagBuilder};
pub use choice::Choice;
pub use engine::{DecisionQuestion, DetailQuestion, Engine, Evidence};
pub use error::{Error, ErrorDetail, ErrorKind};
#[cfg(feature = "polars")]
pub use frame::{PolarsCallOptions, PolarsEngine, PolarsExprOptions};
pub use native_batch::RecoverableDetails;
pub(crate) use options::budget::process_budget;
pub use options::{
    BatchSetting, CallOptions, CancelToken, EstimatedInputDenial, SendBudget, SendBudgetDenial,
};
pub(crate) use options::{EstimatedReservation, SendReservation};
pub use panic::{contained, uncontained};
pub use plan::PlanEstimate;
pub use question::{
    BandedQuestion, ChooseQuestion, Description, DescriptionBuilder, LoadedQuestion, Question,
    QuestionKind, TagQuestion,
};
pub use recognize::{
    Kind, Recognize, RecognizeBuilder, Recognized, RecognizedEntity, Relation, RelationRule,
};
pub use relate::{Edge, Entity, Relate, RelateBuilder};
#[cfg(test)]
pub(crate) use results::QuestionJson;
pub use results::{
    Answer, AttemptObservation, AttemptOutcome, Call, Candidate, Counters, Details, DoorReply,
    Facts, Found, Judgment, NamedProbability, ObservedRow, Probabilities, QuestionDetail, Ranked,
    RecordObservation, Row, Tally, TallyStart, Usage,
};
pub use set::{QuestionSet, QuestionSetBuilder};
pub use settings::EngineBuilder;

/// The Polars crate the door takes, so a caller names the exact version.
#[cfg(feature = "polars")]
pub use ::polars;

/// The process engine the free functions use, built once from the
/// environment. A failed build is returned and tried again on the next call.
///
/// # Errors
///
/// As [`Engine::from_env`].
pub fn default_engine() -> Result<&'static Engine, Error> {
    static ENGINE: OnceLock<Engine> = OnceLock::new();
    if let Some(engine) = ENGINE.get() {
        return Ok(engine);
    }
    // Two threads that both find the cell empty each build; one engine is
    // dropped unused. `from_env` registers no throttle, so the race costs one
    // extra build and nothing else.
    let built = Engine::from_env()?;
    Ok(ENGINE.get_or_init(|| built))
}

/// [`Engine::decide`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::decide`].
pub fn decide<Q: DecisionQuestion + ?Sized>(
    question: &Q,
    evidence: &str,
) -> Result<Call<Answer>, Error> {
    default_engine()?.decide(question, evidence)
}

/// [`Engine::decide_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::decide`].
pub fn decide_with<Q: DecisionQuestion + ?Sized>(
    question: &Q,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Call<Answer>, Error> {
    default_engine()?.decide_with(question, evidence, options)
}

/// [`Engine::choose`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::choose`].
pub fn choose<C: Choice>(
    question: &ChooseQuestion<C>,
    evidence: &str,
) -> Result<Call<Option<C>>, Error> {
    default_engine()?.choose(question, evidence)
}

/// [`Engine::choose_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::choose`].
pub fn choose_with<C: Choice>(
    question: &ChooseQuestion<C>,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Call<Option<C>>, Error> {
    default_engine()?.choose_with(question, evidence, options)
}

/// [`Engine::score`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::score`].
pub fn score(question: &Question, evidence: &str) -> Result<Call<f64>, Error> {
    default_engine()?.score(question, evidence)
}

/// [`Engine::score_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::score`].
pub fn score_with(
    question: &Question,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Call<f64>, Error> {
    default_engine()?.score_with(question, evidence, options)
}

/// [`Engine::tag`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::tag`].
pub fn tag<C: Choice>(question: &TagQuestion<C>, evidence: &str) -> Result<Call<Vec<C>>, Error> {
    default_engine()?.tag(question, evidence)
}

/// [`Engine::tag_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::tag`].
pub fn tag_with<C: Choice>(
    question: &TagQuestion<C>,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Call<Vec<C>>, Error> {
    default_engine()?.tag_with(question, evidence, options)
}

/// [`Engine::filter`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn filter<'a, I>(question: &'a Question, records: I) -> Batch<'a, I::Item>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    filter_with(question, records, CallOptions::new())
}

/// [`Engine::filter_with`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn filter_with<'a, I>(
    question: &'a Question,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, I::Item>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.filter_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`Engine::rank`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::rank`].
#[allow(
    clippy::type_complexity,
    reason = "the public return carries ranked rows and facts"
)]
pub fn rank<I>(question: &Question, records: I) -> Result<Call<Vec<Ranked<I::Item>>>, Error>
where
    I: IntoIterator,
    I::Item: Evidence,
{
    default_engine()?.rank(question, records)
}

/// [`Engine::rank_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::rank`].
#[allow(
    clippy::type_complexity,
    reason = "the public return carries ranked rows and facts"
)]
pub fn rank_with<I>(
    question: &Question,
    records: I,
    options: CallOptions<'_>,
) -> Result<Call<Vec<Ranked<I::Item>>>, Error>
where
    I: IntoIterator,
    I::Item: Evidence,
{
    default_engine()?.rank_with(question, records, options)
}

/// [`Engine::find`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::find`].
pub fn find<I>(question: &Question, units: I) -> Result<Call<Found<I::Item>>, Error>
where
    I: IntoIterator,
    I::Item: Evidence,
{
    default_engine()?.find(question, units)
}

/// [`Engine::find_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::find`].
pub fn find_with<I>(
    question: &Question,
    units: I,
    options: CallOptions<'_>,
) -> Result<Call<Found<I::Item>>, Error>
where
    I: IntoIterator,
    I::Item: Evidence,
{
    default_engine()?.find_with(question, units, options)
}

/// [`Engine::annotate`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn annotate<'a, I>(
    questions: &'a QuestionSet,
    records: I,
) -> Batch<'a, AnnotatedRecord<I::Item>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    annotate_with(questions, records, CallOptions::new())
}

/// [`Engine::annotate_with`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn annotate_with<'a, I>(
    questions: &'a QuestionSet,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, AnnotatedRecord<I::Item>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.annotate_with(questions, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`Engine::recognize`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::recognize`].
pub fn recognize(ask: &Recognize, evidence: &str) -> Result<Call<Recognized>, Error> {
    default_engine()?.recognize(ask, evidence)
}

/// [`Engine::recognize_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::recognize`].
pub fn recognize_with(
    ask: &Recognize,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Call<Recognized>, Error> {
    default_engine()?.recognize_with(ask, evidence, options)
}

/// [`Engine::relate`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::relate`].
pub fn relate<I>(ask: &Relate, entities: I) -> Result<Call<Vec<Edge>>, Error>
where
    I: IntoIterator<Item = Entity>,
{
    default_engine()?.relate(ask, entities)
}

/// [`Engine::relate_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::relate`].
pub fn relate_with<I>(
    ask: &Relate,
    entities: I,
    options: CallOptions<'_>,
) -> Result<Call<Vec<Edge>>, Error>
where
    I: IntoIterator<Item = Entity>,
{
    default_engine()?.relate_with(ask, entities, options)
}

/// [`Engine::decide_many`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn decide_many<'a, I, Q: DecisionQuestion + ?Sized>(
    question: &'a Q,
    records: I,
) -> Batch<'a, Row<I::Item, Answer>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    decide_many_with(question, records, CallOptions::new())
}

/// [`Engine::decide_many_with`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(
    question: &'a Q,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, Answer>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.decide_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`Engine::choose_many`] on the [`default_engine`]; a failed build is the first row.
pub fn choose_many<'a, I, C: Choice>(
    question: &'a ChooseQuestion<C>,
    records: I,
) -> Batch<'a, Row<I::Item, Option<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    choose_many_with(question, records, CallOptions::new())
}

/// [`Engine::choose_many_with`] on the [`default_engine`].
pub fn choose_many_with<'a, I, C: Choice>(
    question: &'a ChooseQuestion<C>,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, Option<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.choose_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`Engine::score_many`] on the [`default_engine`]; a failed build is the first row.
pub fn score_many<'a, I>(question: &'a Question, records: I) -> Batch<'a, Row<I::Item, f64>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    score_many_with(question, records, CallOptions::new())
}

/// [`Engine::score_many_with`] on the [`default_engine`].
pub fn score_many_with<'a, I>(
    question: &'a Question,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, f64>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.score_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`Engine::tag_many`] on the [`default_engine`]; a failed build is the first row.
pub fn tag_many<'a, I, C: Choice>(
    question: &'a TagQuestion<C>,
    records: I,
) -> Batch<'a, Row<I::Item, Vec<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    tag_many_with(question, records, CallOptions::new())
}

/// [`Engine::tag_many_with`] on the [`default_engine`].
pub fn tag_many_with<'a, I, C: Choice>(
    question: &'a TagQuestion<C>,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, Vec<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.tag_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`Engine::details`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::details`].
pub fn details<Q: DetailQuestion + ?Sized>(
    question: &Q,
    evidence: &str,
) -> Result<Call<Details>, Error> {
    default_engine()?.details(question, evidence)
}

/// [`Engine::details_with`] on the [`default_engine`].
///
/// # Errors
///
/// As [`default_engine`] and [`Engine::details`].
pub fn details_with<Q: DetailQuestion + ?Sized>(
    question: &Q,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Call<Details>, Error> {
    default_engine()?.details_with(question, evidence, options)
}

/// The [`default_engine`]'s totals.
///
/// # Errors
///
/// As [`default_engine`].
pub fn usage() -> Result<Counters, Error> {
    Ok(default_engine()?.usage())
}
