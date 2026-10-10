//! Typed complete native calls retain dataframe presentation separately.
use super::column::text;
use crate::public::{
    Call, CallOptions, CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter,
    CompleteFound, CompleteRank, CompleteRecognized, CompleteRecord, CompleteRelated,
    CompleteScore, CompleteSetRank, CompleteTags, DecisionQuestion, DetailQuestion, Engine, Error,
    InputEvidence, Question, QuestionSet, RankSet, Recognize, RecordInput, Relate, RequestCall,
    RequestValue, Surface,
};
use polars::prelude::Series;

#[allow(
    clippy::type_complexity,
    reason = "native input ownership and original positions remain separate"
)]
pub(super) fn present<T>(
    column: &Series,
    records: Vec<Option<RecordInput<T>>>,
) -> Result<(Vec<RecordInput<T>>, Vec<usize>), Error> {
    if column.len() != records.len() {
        return Err(Error::usage(
            "the typed input column has a different row count",
        ));
    }
    let nulls = column.is_null();
    let mut inputs = Vec::new();
    let mut positions = Vec::new();
    for (at, (missing, record)) in nulls.iter().zip(records).enumerate() {
        if missing != Some(record.is_none()) {
            return Err(Error::usage(
                "the typed input column has different null positions",
            ));
        }
        if let Some(record) = record {
            positions.push(at);
            inputs.push(record);
        }
    }
    Ok((inputs, positions))
}

pub(super) fn texts(column: &Series) -> Result<Vec<Option<RecordInput<String>>>, Error> {
    Ok(text(column)?
        .iter()
        .map(|cell| {
            cell.map(|text| RecordInput {
                examples: None,
                seed_spans: None,
                original: text.to_owned(),
                context: None,
                options: None,
            })
        })
        .collect())
}

impl Engine {
    /// Complete decide for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn decide_input_column_complete<Q: DecisionQuestion + ?Sized, T: InputEvidence>(
        &self,
        question: &Q,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteDecision>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.question().clone().into(),
            records,
            options,
            RequestCall::Decide,
            |value| match value {
                RequestValue::Decisions(rows) => Ok(rows),
                _ => Err(Error::defect(
                    "a decide column returned another result kind",
                )),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete choose for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn choose_input_column_complete<Q: DetailQuestion + ?Sized, T: InputEvidence>(
        &self,
        question: &Q,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteChoice>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.question().clone().into(),
            records,
            options,
            RequestCall::Choose,
            |value| match value {
                RequestValue::Choices(rows) => Ok(rows),
                _ => Err(Error::defect(
                    "a choose column returned another result kind",
                )),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete tag for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn tag_input_column_complete<Q: DetailQuestion + ?Sized, T: InputEvidence>(
        &self,
        question: &Q,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteTags>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.question().clone().into(),
            records,
            options,
            RequestCall::Tag,
            |value| match value {
                RequestValue::Tags(rows) => Ok(rows),
                _ => Err(Error::defect("a tag column returned another result kind")),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete score for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn score_input_column_complete<T: InputEvidence>(
        &self,
        question: &Question,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteScore>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.clone().into(),
            records,
            options,
            RequestCall::Score,
            |value| match value {
                RequestValue::Scores(rows) => Ok(rows),
                _ => Err(Error::defect("a score column returned another result kind")),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete filter for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn filter_input_column_complete<T: InputEvidence>(
        &self,
        question: &Question,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteFilter>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.clone().into(),
            records,
            options,
            RequestCall::Filter,
            |value| match value {
                RequestValue::Filtered(rows) => Ok(rows),
                _ => Err(Error::defect(
                    "a filter column returned another result kind",
                )),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete rank for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn rank_input_column_complete<T: InputEvidence>(
        &self,
        question: &Question,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteRank>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_occurrences(
            self,
            question.clone().into(),
            records,
            options,
            RequestCall::Rank,
            |value| match value {
                RequestValue::Ranked(rows) => Ok(rows),
                _ => Err(Error::defect("a rank column returned another result kind")),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete rank set for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn rank_set_input_column_complete<T: InputEvidence>(
        &self,
        question: &RankSet,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_occurrences(
            self,
            question.clone().into(),
            records,
            options,
            RequestCall::Rank,
            |value| match value {
                RequestValue::SetRanked(rows) => Ok(rows),
                _ => Err(Error::defect(
                    "a rank_set column returned another result kind",
                )),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete annotate for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn annotate_input_column_complete<T: InputEvidence>(
        &self,
        question: &QuestionSet,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.clone().into(),
            records,
            options,
            RequestCall::Annotate,
            |value| match value {
                RequestValue::Annotations(rows) => Ok(rows),
                _ => Err(Error::defect(
                    "a annotate column returned another result kind",
                )),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete recognize for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn recognize_input_column_complete<T: InputEvidence>(
        &self,
        question: &Recognize,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = super::request::complete_records(
            self,
            question.clone().into(),
            records,
            options,
            RequestCall::Recognize,
            |value| match value {
                RequestValue::Recognized(rows) => Ok(rows),
                _ => Err(Error::defect(
                    "a recognize column returned another result kind",
                )),
            },
        )?;
        Ok((call, positions))
    }

    /// Complete relate for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn relate_input_column_complete<T: InputEvidence>(
        &self,
        question: &Relate,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<CompleteRecord<Vec<T>, CompleteRelated>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = self.relate_records_complete_with(
            question,
            records,
            options.surface(Surface::RustPolars),
        )?;
        Ok((call, positions))
    }

    /// Complete find for a typed input column associated with actual frame rows.
    /// The Series supplies presentation; records supply explicit native evidence and controls.
    /// The returned positions map compact native ordinals to original nullable frame rows.
    /// # Errors
    /// Refuses mismatched row counts/nulls before sending; otherwise returns native call errors.
    #[allow(
        clippy::type_complexity,
        reason = "native typed judgments and nullable frame positions stay separate"
    )]
    pub fn find_input_column_complete<T: InputEvidence>(
        &self,
        question: &Question,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<(Call<CompleteFound<T>>, Vec<usize>), Error> {
        let (records, positions) = present(column, records)?;
        let call = self.find_records_complete_with(
            question,
            records,
            options.surface(Surface::RustPolars),
        )?;
        Ok((call, positions))
    }
}
