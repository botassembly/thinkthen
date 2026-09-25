#![doc = include_str!("../README.md")]

mod column;
mod error;

pub use error::Error;
pub use polars;
pub use thinkthen;

use polars::prelude::{Column, DataFrame, NamedFrom, Series};
use thinkthen::{
    Answer, CallOptions, DecisionQuestion, Engine, Question, QuestionKind, QuestionSet,
};

use column::{Shape, answered, kind_word, texts};

/// The one member name of the set a single-question column asks.
const MEMBER: &str = "answer";

/// The Series and frame door, implemented for [`thinkthen::Engine`].
pub trait PolarsEngine {
    /// Decide every text of the column: a `Boolean` series under the
    /// column's name, with a null where a band left the answer not sure.
    ///
    /// # Errors
    ///
    /// [`Error::Usage`] for a column that is not text or holds a null, and
    /// [`Error::Engine`] for anything the engine refuses. A failed row ends
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

    /// Ask every question of the set of the text column `on`: the caller's
    /// frame with one new column per question, in set order.
    ///
    /// # Errors
    ///
    /// As [`PolarsEngine::decide_series`], and [`Error::Usage`] when the
    /// frame holds no column `on` or already holds a column a question names.
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
        single(
            self,
            "choose_series",
            QuestionKind::Choose,
            question,
            texts,
            options,
        )
    }

    fn score_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error> {
        single(
            self,
            "score_series",
            QuestionKind::Score,
            question,
            texts,
            options,
        )
    }

    fn tag_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Series, Error> {
        single(
            self,
            "tag_series",
            QuestionKind::Tag,
            question,
            texts,
            options,
        )
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
            .map_err(|_| Error::Usage(format!("the frame holds no column {on}")))?;
        for (name, _) in questions.members() {
            if frame.column(name).is_ok() {
                return Err(Error::Usage(format!(
                    "the frame already holds a column named {name}"
                )));
            }
        }
        let rows = texts(held.as_materialized_series())?;
        let records = self
            .annotate_with(questions, rows, options)
            .collect::<Result<Vec<_>, _>>()?;
        let columns = questions
            .members()
            .enumerate()
            .map(|(place, (name, kind))| {
                answered(name, kind, Shape::Frame, &records, place).map(Into::into)
            })
            .collect::<Result<Vec<Column>, Error>>()?;
        frame
            .hstack(&columns)
            .map_err(|error| Error::Defect(format!("the frame refused a new column: {error}")))
    }
}

/// One choose, score, or tag column through a one-question set.
fn single(
    engine: &Engine,
    method: &str,
    wanted: QuestionKind,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Series, Error> {
    if question.kind() != wanted {
        return Err(Error::Usage(format!(
            "{method} needs a {} question, and this one is a {} question",
            kind_word(wanted),
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
    answered(texts.name().as_str(), wanted, Shape::Series, &records, 0)
}
