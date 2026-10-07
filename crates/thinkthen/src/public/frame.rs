//! The Rust Polars door, behind the `polars` feature (ticket 0130).

use polars::prelude::{DataFrame, Expr, LazyFrame, Series};

use super::{
    Call, CallOptions, DecisionQuestion, Edge, Error, PlanEstimate, Question, QuestionSet,
    Recognize, Recognized, Relate,
};

use crate::public::{Details, InputReaderOptions, QuestionInput};
use std::path::PathBuf;

mod column;
mod complete;
mod eager;
mod inputs;
mod lazy;
mod options;
mod typed;
mod typed_batches;
mod typed_series;
pub use lazy::PolarsExprOptions;
pub use options::PolarsCallOptions;

/// The one member name of the set a single-question column asks.
const MEMBER: &str = "answer";

/// The Series and frame door, implemented for [`Engine`]. It needs the
/// `polars` feature, and takes the Polars version [`crate::polars`] names.
///
/// Row-wise text judgments read a column in place at the native throttle.
/// Whole-set rank/find and explicit source helpers materialize their logical
/// collection once; they never judge each lazy morsel as a complete set.
/// Nullable outputs retain their input positions and names.
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
    /// Judge ordered explicit text/images, retaining null rows and selected
    /// probabilities. Ordinary strings and bytes do not imply images.
    /// # Errors
    /// Returns native admission/call errors; text-only image refusals send nothing.
    fn input_column(
        &self,
        question: &Question,
        inputs: &[Option<QuestionInput>],
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;

    /// Complete typed native probabilities for every explicit present input.
    /// # Errors
    /// Returns the same validation and call errors as input_column.
    fn input_details_column(
        &self,
        question: &Question,
        inputs: &[Option<QuestionInput>],
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Option<Details>>>, Error>;

    /// Materialize selected files through the single native reader. Text lines
    /// stay physical; whole-file images have null text line coordinates.
    /// # Errors
    /// Returns native reader, media admission and call errors.
    fn source_column(
        &self,
        question: &Question,
        paths: &[PathBuf],
        reading: InputReaderOptions,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;

    /// Keep matching present cells, preserving their input name and order.
    /// Null inputs ask no question and are omitted.
    /// # Errors
    /// Returns native validation, cancellation or request errors.
    fn filter_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error>;

    /// Rank the complete present collection. Output has original zero-based
    /// `index`, `record` and `probability`; ties keep duplicate input order.
    /// # Errors
    /// Returns native rank admission and call errors.
    fn rank_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;

    /// Compare every present candidate in one native find call. Output keeps
    /// every candidate's `index`, `unit`, `probability` and real-unit `selected` flag.
    /// A synthetic none candidate has null index and unit.
    /// # Errors
    /// Returns native find admission and call errors.
    fn find_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error>;

    /// Recognize each present text, retaining complete spans and relations;
    /// null cells remain None. All calls share one fixed deadline.
    /// # Errors
    /// Returns native errors with aggregate facts, or checked tally overflow.
    fn recognize_series(
        &self,
        ask: &Recognize,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Option<Recognized>>>, Error>;

    /// Relate the complete entity collection from text name/kind columns.
    /// Paired null cells are omitted; a partly null entity is refused.
    /// # Errors
    /// Returns native entity validation, relation and call errors.
    fn relate_frame(
        &self,
        ask: &Relate,
        frame: &DataFrame,
        name: &str,
        kind: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Edge>>, Error>;

    /// Materialize a lazy frame's whole logical collection before ranking.
    /// # Errors
    /// Returns collection errors or native rank errors.
    fn rank_lazy(
        &self,
        question: &Question,
        frame: LazyFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        let frame = frame
            .collect()
            .map_err(|_| Error::usage("the lazy frame could not be collected"))?;
        let texts = frame
            .column(on)
            .map_err(|_| Error::usage("the frame has no selected column"))?;
        self.rank_series(question, texts.as_materialized_series(), options)
    }

    /// Materialize a lazy frame's whole logical collection before finding.
    /// # Errors
    /// Returns collection errors or native find errors.
    fn find_lazy(
        &self,
        question: &Question,
        frame: LazyFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        let frame = frame
            .collect()
            .map_err(|_| Error::usage("the lazy frame could not be collected"))?;
        let texts = frame
            .column(on)
            .map_err(|_| Error::usage("the frame has no selected column"))?;
        self.find_series(question, texts.as_materialized_series(), options)
    }
}
