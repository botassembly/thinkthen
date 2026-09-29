//! The Rust Polars door, behind the `polars` feature (ticket 0130).

use polars::prelude::{Column, DataFrame, NamedFrom, Series};

use super::{
    Answer, Batch, Call, CallOptions, DecisionQuestion, Details, Engine, Error, Judgment,
    PlanEstimate, Probabilities, Question, QuestionKind, QuestionSet,
};

mod column;
mod options;
pub use options::PolarsCallOptions;

use column::{answered, failed, kind_word, nullable, restore};

/// The one member name of the set a single-question column asks.
const MEMBER: &str = "answer";

/// The Series and frame door, implemented for [`Engine`]. It needs the
/// `polars` feature, and takes the Polars version [`crate::polars`] names.
///
/// Each method reads a text column in place and makes one engine call over
/// the whole column, at the throttle, with answers in input order.
///
/// ```no_run
/// use thinkthen::polars::prelude::{NamedFrom, Series};
/// use thinkthen::{CallOptions, EngineBuilder, PolarsEngine, Question};
///
/// fn main() -> Result<(), thinkthen::Error> {
///     let engine = EngineBuilder::from_env()?
///         .base_url("http://127.0.0.1:8080/v1")?
///         .model("your-model")?
///         .throttle(8)?
///         .max_requests(Some(10_000))?
///         .cache_at("answers")?
///         .build()?;
///     let notes = Series::new("note".into(), ["Please refund my order.", "Thanks, all good."]);
///     let refund = Question::decide("Does the writer ask for a refund?")?.cut();
///     let asked = engine.decide_series(&refund, &notes, CallOptions::new())?;
///     assert_eq!(asked.value().len(), 2);
///     assert_eq!(asked.facts().records(), 2);
///     Ok(())
/// }
/// ```
pub trait PolarsEngine {
    /// Judge a column with per-call threshold and meaning overrides. The
    /// returned frame has `value` and, when requested, `probability` columns.
    /// Score and tag refuse a probability request before any send.
    ///
    /// # Errors
    /// Returns the same call error as the matching Series door.
    fn column_with(
        &self,
        question: &Question,
        texts: &Series,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;

    /// Preview the same packed request bodies a column call would prepare.
    /// No key, cache or network is read.
    ///
    /// # Errors
    /// Refuses invalid text or an ineligible question before any send.
    fn plan_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<PlanEstimate, Error>;

    /// Judge a decide or choose column once and return `value` beside its
    /// nullable `probability` column. Score and tag have no selected-answer
    /// probability and are refused before a send.
    ///
    /// # Errors
    /// As [`PolarsEngine::decide_series`], or [`Error::Usage`] for score/tag.
    fn probability_frame(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;

    /// Decide every text of the column: a `Boolean` series under the
    /// column's name, with a null where a band left the answer not sure.
    ///
    /// # Errors
    ///
    /// [`Error::Usage`] for a column that is not text, and the engine's error
    /// for anything the engine refuses. Null cells stay null without a send.
    /// A failed row ends
    /// the call with the engine's error.
    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error>;

    /// Pick one option for every text: a `String` series with a null where
    /// nothing fits.
    ///
    /// # Errors
    ///
    /// As [`PolarsEngine::decide_series`], and [`Error::Usage`] for a
    /// question that is not a choose question.
    fn choose_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error>;

    /// Place every text on the levels: a `Float64` series of positions from
    /// 0 to one less than the number of levels.
    ///
    /// # Errors
    ///
    /// As [`PolarsEngine::choose_series`], for a score question.
    fn score_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error>;

    /// Test every label on every text: a `List(String)` series.
    ///
    /// # Errors
    ///
    /// As [`PolarsEngine::choose_series`], for a tag question.
    fn tag_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error>;

    /// Ask every question of the text column `on`: the caller's frame with
    /// one typed column per question, followed by nullable `failed` markers.
    ///
    /// # Errors
    ///
    /// As [`PolarsEngine::decide_series`], and [`Error::Usage`] when the
    /// frame holds no column `on`, a result column already exists, or a
    /// question is named `failed`.
    fn annotate_frame(
        &self,
        questions: &QuestionSet,
        frame: &DataFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;
}

impl PolarsEngine for Engine {
    fn column_with(
        &self,
        question: &Question,
        texts: &Series,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        let (question, call, probability) = options.apply(question)?;
        if probability {
            return self.probability_frame(&question, texts, call);
        }
        let value = match question.kind() {
            QuestionKind::Decide => self.decide_series(&question, texts, call)?,
            QuestionKind::Choose => self.choose_series(&question, texts, call)?,
            QuestionKind::Score => self.score_series(&question, texts, call)?,
            QuestionKind::Tag => self.tag_series(&question, texts, call)?,
            _ => {
                return Err(Error::usage(
                    "this column door takes decide, choose, score or tag",
                ));
            }
        };
        value.try_map(|value| {
            DataFrame::new(value.len(), vec![value.into()])
                .map_err(|error| Error::defect(&format!("the value frame was refused: {error}")))
        })
    }

    fn plan_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<PlanEstimate, Error> {
        let cells = nullable(texts)?;
        self.plan_with(question, cells.iter().flatten().copied(), options)
    }

    fn probability_frame(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        let kind = question.kind();
        if !matches!(kind, QuestionKind::Decide | QuestionKind::Choose) {
            return Err(Error::usage(format!(
                "probability belongs to decide and choose, not {}",
                kind_word(kind)
            )));
        }
        let cells = nullable(texts)?;
        completed(self.details_many_with(question, cells.iter().flatten().copied(), options))?
            .try_map(|rows| {
                let probability = rows
                    .iter()
                    .map(|row| selected_probability(row.value()))
                    .collect::<Result<Vec<_>, _>>()?;
                let values = match kind {
                    QuestionKind::Decide => Series::new(
                        "value".into(),
                        rows.iter()
                            .map(|row| match row.value().value() {
                                Judgment::Decision(Answer::Yes) => Ok(Some(true)),
                                Judgment::Decision(Answer::No) => Ok(Some(false)),
                                Judgment::Decision(Answer::Unsure) => Ok(None),
                                _ => Err(Error::defect("a decide detail held another value")),
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                    QuestionKind::Choose => Series::new(
                        "value".into(),
                        rows.iter()
                            .map(|row| match row.value().value() {
                                Judgment::Choice(value) => Ok(value.as_deref()),
                                _ => Err(Error::defect("a choose detail held another value")),
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                    _ => return Err(Error::defect("a probability kind changed")),
                };
                let values = restore(values, &cells)?;
                let probability = restore(Series::new("probability".into(), probability), &cells)?;
                DataFrame::new(cells.len(), vec![values.into(), probability.into()]).map_err(
                    |error| Error::defect(&format!("the probability frame was refused: {error}")),
                )
            })
    }

    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        let cells = nullable(texts)?;
        completed(self.decide_many_with(question, cells.iter().flatten().copied(), options))?
            .map(|row| {
                row.into_iter()
                    .map(|row| match row.value() {
                        Answer::Yes => Some(true),
                        Answer::No => Some(false),
                        Answer::Unsure => None,
                    })
                    .collect::<Vec<_>>()
            })
            .map(|answers| Series::new(texts.name().clone(), answers))
            .try_map(|series| restore(series, &cells))
    }

    fn choose_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        single(self, QuestionKind::Choose, question, texts, options)
    }

    fn score_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        single(self, QuestionKind::Score, question, texts, options)
    }

    fn tag_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        single(self, QuestionKind::Tag, question, texts, options)
    }

    fn annotate_frame(
        &self,
        questions: &QuestionSet,
        frame: &DataFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        let held = frame
            .column(on)
            .map_err(|_| Error::usage(format!("the frame holds no column {on}")))?;
        if questions.members().any(|(name, _)| name == "failed") {
            return Err(Error::usage(
                "the question name failed is reserved for frame failures",
            ));
        }
        for (name, _) in questions
            .members()
            .chain(std::iter::once(("failed", QuestionKind::Decide)))
        {
            if frame.column(name).is_ok() {
                return Err(Error::usage(format!(
                    "the frame already holds a column named {name}"
                )));
            }
        }
        let cells = nullable(held.as_materialized_series())?;
        completed(self.annotate_with(questions, cells.iter().flatten().copied(), options))?.try_map(
            |records| {
                let mut columns = questions
                    .members()
                    .enumerate()
                    .map(|(place, (name, kind))| {
                        answered(name, kind, &records, place)
                            .and_then(|series| restore(series, &cells))
                            .map(Into::into)
                    })
                    .collect::<Result<Vec<Column>, Error>>()?;
                let names = questions
                    .members()
                    .map(|(name, _)| name)
                    .collect::<Vec<_>>();
                columns.push(restore(failed(&names, &records)?, &cells)?.into());
                frame.hstack(&columns).map_err(|error| {
                    Error::defect(&format!("the frame refused a new column: {error}"))
                })
            },
        )
    }
}

fn selected_probability(details: &Details) -> Result<Option<f64>, Error> {
    match (details.value(), details.probabilities()) {
        (Judgment::Decision(_), Probabilities::YesNo { yes }) => Ok(Some(*yes)),
        (Judgment::Choice(Some(selected)), Probabilities::Named(options)) => options
            .iter()
            .find(|option| option.name() == selected)
            .map(|option| Some(option.probability()))
            .ok_or_else(|| Error::defect("the chosen label has no probability")),
        (Judgment::Choice(None), Probabilities::Named(_)) => Ok(None),
        _ => Err(Error::defect("a probability detail held another shape")),
    }
}

/// One choose, score, or tag column through a one-question set.
fn single(
    engine: &Engine,
    wanted: QuestionKind,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<Series>, Error> {
    if question.kind() != wanted {
        let word = kind_word(wanted);
        return Err(Error::usage(format!(
            "{word}_series needs a {word} question, and this one is a {} question",
            kind_word(question.kind())
        )));
    }
    let cells = nullable(texts)?;
    let set = QuestionSet::builder()
        .question(MEMBER, question.clone())?
        .build()?;
    completed(engine.annotate_with(&set, cells.iter().flatten().copied(), options))?.try_map(
        |records| {
            restore(
                answered(texts.name().as_str(), wanted, &records, 0)?,
                &cells,
            )
        },
    )
}

/// Consume the one call before a host conversion can fail.
fn completed<T>(mut batch: Batch<'_, T>) -> Result<Call<Vec<T>>, Error> {
    let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| Error::defect("a completed Polars call has no final facts"))?;
    Ok(Call::new(rows, facts))
}
