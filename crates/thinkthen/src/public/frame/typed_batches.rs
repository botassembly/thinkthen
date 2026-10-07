//! Pull-based frame calls share the existing native scheduler.
use super::typed::present;
use crate::public::{
    Batch, Call, CallOptions, CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter,
    CompleteRecord, CompleteScore, CompleteTags, DecisionQuestion, DetailQuestion, Engine, Error,
    InputEvidence, Question, QuestionSet, RecordChooseQuestion, RecordInput, Surface,
};
use polars::prelude::Series;
impl Engine {
    /// Prepare pull-based decide for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn decide_input_column_batch<'a, Q: DecisionQuestion + ?Sized, T: InputEvidence + 'a>(
        &'a self,
        question: &'a Q,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteDecision>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_decide_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Prepare pull-based choose for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn choose_input_column_batch<'a, Q: DetailQuestion + ?Sized, T: InputEvidence + 'a>(
        &'a self,
        question: &'a Q,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteChoice>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_choose_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Prepare pull-based choose dynamic for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn choose_dynamic_input_column_batch<'a, T: InputEvidence + 'a>(
        &'a self,
        question: &'a RecordChooseQuestion,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteChoice>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_choose_dynamic_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Prepare pull-based tag for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn tag_input_column_batch<'a, Q: DetailQuestion + ?Sized, T: InputEvidence + 'a>(
        &'a self,
        question: &'a Q,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteTags>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_tag_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Prepare pull-based score for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn score_input_column_batch<'a, T: InputEvidence + 'a>(
        &'a self,
        question: &'a Question,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteScore>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_score_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Prepare pull-based filter for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn filter_input_column_batch<'a, T: InputEvidence + 'a>(
        &'a self,
        question: &'a Question,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteFilter>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_filter_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Prepare pull-based annotate for the actual nullable frame column.
    /// Native pulling controls sends; preparing the frame sends nothing.
    /// # Errors
    /// Refuses mismatched frame rows/nulls before starting; pull errors retain native facts.
    #[allow(
        clippy::type_complexity,
        reason = "typed native batch and frame positions stay separate"
    )]
    pub fn annotate_input_column_batch<'a, T: InputEvidence + 'a>(
        &'a self,
        question: &'a QuestionSet,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'a>,
    ) -> Result<(Batch<'a, CompleteRecord<T, CompleteAnnotated>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.try_annotate_records_complete_with(
                question,
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
            ),
            positions,
        ))
    }

    /// Complete per-record dynamic choose candidates associated with actual frame rows.
    /// # Errors
    /// Refuses invalid frame associations/candidates; otherwise returns native errors.
    #[allow(
        clippy::type_complexity,
        reason = "typed native results and frame positions stay separate"
    )]
    pub fn choose_dynamic_input_column_complete<T: InputEvidence>(
        &self,
        question: &RecordChooseQuestion,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteChoice>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        Ok((
            self.choose_dynamic_records_complete_with(
                question,
                records,
                options.surface(Surface::RustPolars),
            )?,
            positions,
        ))
    }
}
