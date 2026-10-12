//! Pull-based frame calls enter Request through the native borrowed-row scheduler.
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
            Batch::of(
                crate::public::request::pull::native(
                    self,
                    question.question().clone().into(),
                    records.into_iter().map(Ok),
                    options.surface(Surface::RustPolars),
                    crate::RequestFunction::Decide,
                )
                .and_then(|rows| match rows {
                    crate::RequestRows::Decisions(batch) => Ok(batch),
                    _ => Err(Error::defect(
                        "a native decide batch returned another result kind",
                    )),
                }),
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
            Batch::of(
                crate::public::request::pull::native(
                    self,
                    question.question().clone().into(),
                    records.into_iter().map(Ok),
                    options.surface(Surface::RustPolars),
                    crate::RequestFunction::Choose,
                )
                .and_then(|rows| match rows {
                    crate::RequestRows::Choices(batch) => Ok(batch),
                    _ => Err(Error::defect(
                        "a native choose batch returned another result kind",
                    )),
                }),
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
            Batch::of(
                crate::public::request::pull::native(
                    self,
                    question.question().clone().into(),
                    records.into_iter().map(Ok),
                    options.surface(Surface::RustPolars),
                    crate::RequestFunction::Tag,
                )
                .and_then(|rows| match rows {
                    crate::RequestRows::Tags(batch) => Ok(batch),
                    _ => Err(Error::defect(
                        "a native tag batch returned another result kind",
                    )),
                }),
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
            Batch::of(
                crate::public::request::pull::native(
                    self,
                    question.clone().into(),
                    records.into_iter().map(Ok),
                    options.surface(Surface::RustPolars),
                    crate::RequestFunction::Score,
                )
                .and_then(|rows| match rows {
                    crate::RequestRows::Scores(batch) => Ok(batch),
                    _ => Err(Error::defect(
                        "a native score batch returned another result kind",
                    )),
                }),
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
            Batch::of(
                crate::public::request::pull::native(
                    self,
                    question.clone().into(),
                    records.into_iter().map(Ok),
                    options.surface(Surface::RustPolars),
                    crate::RequestFunction::Filter,
                )
                .and_then(|rows| match rows {
                    crate::RequestRows::Filtered(batch) => Ok(batch),
                    _ => Err(Error::defect(
                        "a native filter batch returned another result kind",
                    )),
                }),
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
            Batch::of(
                crate::public::request::pull::native(
                    self,
                    question.clone().into(),
                    records.into_iter().map(Ok),
                    options.surface(Surface::RustPolars),
                    crate::RequestFunction::Annotate,
                )
                .and_then(|rows| match rows {
                    crate::RequestRows::Annotations(batch) => Ok(batch),
                    _ => Err(Error::defect(
                        "a native annotate batch returned another result kind",
                    )),
                }),
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
            crate::public::request::native(
                self,
                crate::RequestFunction::Choose,
                question.clone().into(),
                records.into_iter().map(Ok),
                options.surface(Surface::RustPolars),
                |engine, definition, records, controls| {
                    let q = definition.dynamic_choose()?;
                    Ok(crate::public::request::restore(
                        engine
                            .execute_choose_dynamic_records_complete_with(&q, records, controls)?,
                    ))
                },
            )?,
            positions,
        ))
    }
}
