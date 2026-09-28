//! The Rust Polars door, behind the `polars` feature (ticket 0130).

use polars::prelude::{Column, DataFrame, NamedFrom, Series};

use super::{
    Answer, CallOptions, DecisionQuestion, Engine, Error, Question, QuestionKind, QuestionSet,
};

mod column;

use column::{answered, failed, kind_word, texts};

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
///     assert_eq!(asked.len(), 2);
///     Ok(())
/// }
/// ```
pub trait PolarsEngine {
    /// Decide every text of the column: a `Boolean` series under the
    /// column's name, with a null where a band left the answer not sure.
    ///
    /// # Errors
    ///
    /// [`Error::Usage`] for a column that is not text or holds a null, and
    /// the engine's error for anything the engine refuses. A failed row ends
    /// the call with the engine's error.
    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error>;

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
    ) -> Result<Series, Error>;

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
    ) -> Result<Series, Error>;

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
    ) -> Result<Series, Error>;

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
    ) -> Result<DataFrame, Error>;
}

impl PolarsEngine for Engine {
    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error> {
        let rows = column::texts(texts)?;
        let answers = self
            .decide_many_with(question, rows, options)
            .map(|row| {
                row.map(|row| match row.value() {
                    Answer::Yes => Some(true),
                    Answer::No => Some(false),
                    Answer::Unsure => None,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Series::new(texts.name().clone(), answers))
    }

    fn choose_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error> {
        single(self, QuestionKind::Choose, question, texts, options)
    }

    fn score_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error> {
        single(self, QuestionKind::Score, question, texts, options)
    }

    fn tag_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error> {
        single(self, QuestionKind::Tag, question, texts, options)
    }

    fn annotate_frame(
        &self,
        questions: &QuestionSet,
        frame: &DataFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<DataFrame, Error> {
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
        let rows = texts(held.as_materialized_series())?;
        let records = self
            .annotate_with(questions, rows, options)
            .collect::<Result<Vec<_>, _>>()?;
        let mut columns = questions
            .members()
            .enumerate()
            .map(|(place, (name, kind))| answered(name, kind, &records, place).map(Into::into))
            .collect::<Result<Vec<Column>, Error>>()?;
        let names = questions
            .members()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        columns.push(failed(&names, &records)?.into());
        frame
            .hstack(&columns)
            .map_err(|error| Error::defect(&format!("the frame refused a new column: {error}")))
    }
}

/// One choose, score, or tag column through a one-question set.
fn single(
    engine: &Engine,
    wanted: QuestionKind,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Series, Error> {
    if question.kind() != wanted {
        let word = kind_word(wanted);
        return Err(Error::usage(format!(
            "{word}_series needs a {word} question, and this one is a {} question",
            kind_word(question.kind())
        )));
    }
    let rows = column::texts(texts)?;
    let set = QuestionSet::builder()
        .question(MEMBER, question.clone())?
        .build()?;
    let records = engine
        .annotate_with(&set, rows, options)
        .collect::<Result<Vec<_>, _>>()?;
    answered(texts.name().as_str(), wanted, &records, 0)
}
