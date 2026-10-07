//! Typed complete native calls retain dataframe presentation separately.
use super::typed::texts;
use crate::public::{
    Call, CallOptions, CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter,
    CompleteFound, CompleteRank, CompleteRecognized, CompleteRecord, CompleteRelated,
    CompleteScore, CompleteSetRank, CompleteTags, DecisionQuestion, DetailQuestion, Engine, Error,
    Question, QuestionSet, RankSet, Recognize, Relate,
};
use polars::prelude::Series;

impl Engine {
    /// Complete decide over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn decide_series_complete<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<Vec<CompleteRecord<String, CompleteDecision>>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.decide_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete choose over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn choose_series_complete<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<Vec<CompleteRecord<String, CompleteChoice>>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.choose_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete tag over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn tag_series_complete<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<String, CompleteTags>>>, Vec<usize>), Error> {
        self.tag_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete score over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn score_series_complete(
        &self,
        question: &Question,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<String, CompleteScore>>>, Vec<usize>), Error> {
        self.score_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete filter over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn filter_series_complete(
        &self,
        question: &Question,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<Vec<CompleteRecord<String, CompleteFilter>>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.filter_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete rank over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn rank_series_complete(
        &self,
        question: &Question,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<String, CompleteRank>>>, Vec<usize>), Error> {
        self.rank_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete rank set over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn rank_set_series_complete(
        &self,
        question: &RankSet,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<Vec<CompleteRecord<String, CompleteSetRank>>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.rank_set_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete annotate over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn annotate_series_complete(
        &self,
        question: &QuestionSet,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<Vec<CompleteRecord<String, CompleteAnnotated>>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.annotate_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete recognize over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn recognize_series_complete(
        &self,
        question: &Recognize,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<Vec<CompleteRecord<String, CompleteRecognized>>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.recognize_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete relate over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn relate_series_complete(
        &self,
        question: &Relate,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<
        (
            Call<CompleteRecord<Vec<String>, CompleteRelated>>,
            Vec<usize>,
        ),
        Error,
    > {
        self.relate_input_column_complete(question, column, texts(column)?, options)
    }

    /// Complete find over a text Series, retaining original nullable positions.
    /// # Errors
    /// Refuses non-text columns before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn find_series_complete(
        &self,
        question: &Question,
        column: &Series,
        options: CallOptions<'_>,
    ) -> Result<(Call<CompleteFound<String>>, Vec<usize>), Error> {
        self.find_input_column_complete(question, column, texts(column)?, options)
    }
}
