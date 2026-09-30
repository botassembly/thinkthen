//! The Rust Polars door, behind the `polars` feature (ticket 0130).

use polars::prelude::{
    BooleanChunkedBuilder, ChunkedBuilder, Column, DataFrame, Expr, Float64Type, IntoSeries,
    PrimitiveChunkedBuilder, Series,
};

use super::{
    Annotated, Call, CallOptions, DecisionQuestion, Details, Engine, Error, Judgment, PlanEstimate,
    Probabilities, Question, QuestionKind, QuestionSet,
};

mod column;
mod lazy;
mod options;
pub use lazy::PolarsExprOptions;
pub use options::PolarsCallOptions;

use column::{Answers, Failures, decided, kind_word, streamed, text};

/// The one member name of the set a single-question column asks.
const MEMBER: &str = "answer";

/// The Series and frame door, implemented for [`Engine`]. It needs the
/// `polars` feature, and takes the Polars version [`crate::polars`] names.
///
/// Each method reads a text column in place and makes one engine call over
/// the whole column, at the throttle, with answers in input order. Each
/// answer goes into the output column as it arrives, so a call holds its
/// input, its output and the pipeline's window, never a list of every row.
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
    /// Build a lazy decide expression. One collected morsel is one call;
    /// build-time controls are owned by the expression.
    ///
    /// # Errors
    /// Refuses a non-decide question or invalid controls before collection.
    fn decide_expr<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error>;

    /// Build a lazy choose expression over a text column.
    ///
    /// # Errors
    /// Refuses a non-choose question or invalid controls before collection.
    fn choose_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error>;

    /// Build a lazy score expression over a text column.
    ///
    /// # Errors
    /// Refuses a non-score question or probability before collection.
    fn score_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error>;

    /// Build a lazy tag expression over a text column.
    ///
    /// # Errors
    /// Refuses a non-tag question or probability before collection.
    fn tag_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error>;

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
    fn decide_expr<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(
            self,
            QuestionKind::Decide,
            question.question(),
            input,
            options,
        )
    }

    fn choose_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(self, QuestionKind::Choose, question, input, options)
    }

    fn score_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(self, QuestionKind::Score, question, input, options)
    }

    fn tag_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(self, QuestionKind::Tag, question, input, options)
    }

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
        self.plan_with(question, text(texts)?.iter().flatten(), options)
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
        let cells = text(texts)?;
        let rows = cells.len();
        let mut probability =
            PrimitiveChunkedBuilder::<Float64Type>::new("probability".into(), rows);
        let mut values = Answers::new("value", kind, rows)?;
        let batch = self.details_many_with(question, cells.iter().flatten(), options);
        let facts = streamed(batch, cells, |row| {
            let Some(row) = row else {
                probability.append_null();
                return values.push(None);
            };
            probability.append_option(selected_probability(row.value())?);
            let value = match row.value().value() {
                Judgment::Decision(answer) => Annotated::Decision(*answer),
                Judgment::Choice(label) => Annotated::Choice(label.clone()),
                _ => return Err(Error::defect("a probability detail held another value")),
            };
            values.push(Some(&value))
        })?;
        let columns = vec![
            values.finish().into(),
            probability.finish().into_series().into(),
        ];
        Call::new(columns, facts).try_map(|columns| {
            DataFrame::new(rows, columns).map_err(|error| {
                Error::defect(&format!("the probability frame was refused: {error}"))
            })
        })
    }

    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        let cells = text(texts)?;
        let mut answers = BooleanChunkedBuilder::new(texts.name().clone(), cells.len());
        let batch = self.decide_many_with(question, cells.iter().flatten(), options);
        let facts = streamed(batch, cells, |row| {
            answers.append_option(row.and_then(|row| decided(*row.value())));
            Ok(())
        })?;
        Ok(Call::new(answers.finish().into_series(), facts))
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
        let cells = text(held.as_materialized_series())?;
        let names = questions
            .members()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        let mut columns = questions
            .members()
            .map(|(name, kind)| Answers::new(name, kind, cells.len()))
            .collect::<Result<Vec<_>, _>>()?;
        let mut failures = Failures::new(&names, cells.len());
        let batch = self.annotate_with(questions, cells.iter().flatten(), options);
        let facts = streamed(batch, cells, |record| {
            for (place, column) in columns.iter_mut().enumerate() {
                column.push_record(record.as_ref(), place)?;
            }
            failures.push(record.as_ref())
        })?;
        Call::new((columns, failures), facts).try_map(|(columns, failures)| {
            let mut columns = columns
                .into_iter()
                .map(|column| column.finish().into())
                .collect::<Vec<Column>>();
            columns.push(failures.finish()?.into());
            frame
                .hstack(&columns)
                .map_err(|error| Error::defect(&format!("the frame refused a new column: {error}")))
        })
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
    let cells = text(texts)?;
    let set = QuestionSet::builder()
        .question(MEMBER, question.clone())?
        .build()?;
    let mut answers = Answers::new(texts.name().as_str(), wanted, cells.len())?;
    let batch = engine.annotate_with(&set, cells.iter().flatten(), options);
    let facts = streamed(batch, cells, |record| {
        answers.push_record(record.as_ref(), 0)
    })?;
    Ok(Call::new(answers.finish(), facts))
}
