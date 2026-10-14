//! Native typed results retain the existing complete result contracts.
use crate::{
    Call, CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter, CompleteFound,
    CompleteRank, CompleteRecognized, CompleteRecord, CompleteRelated, CompleteScore,
    CompleteSetRank, CompleteTags, Error, QuestionInput,
};
use serde::Serialize;

/// The actual complete values returned by each requested function.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RequestValue {
    /// Complete decision occurrences.
    Decisions(Vec<CompleteRecord<QuestionInput, CompleteDecision>>),
    /// Complete choose occurrences.
    Choices(Vec<CompleteRecord<QuestionInput, CompleteChoice>>),
    /// Complete tag occurrences.
    Tags(Vec<CompleteRecord<QuestionInput, CompleteTags>>),
    /// Complete score occurrences.
    Scores(Vec<CompleteRecord<QuestionInput, CompleteScore>>),
    /// Complete filter occurrences: passing rows by default, all rows when the
    /// native composed feed explicitly retains rejected results.
    Filtered(Vec<CompleteRecord<QuestionInput, CompleteFilter>>),
    /// Complete ranked occurrences.
    Ranked(Vec<CompleteRecord<QuestionInput, CompleteRank>>),
    /// Complete ranked question-set occurrences.
    SetRanked(Vec<CompleteRecord<QuestionInput, CompleteSetRank>>),
    /// A complete find reading with its original candidate set.
    Found(CompleteFound<QuestionInput>),
    /// Complete annotations and any failed named members.
    Annotations(Vec<CompleteRecord<QuestionInput, CompleteAnnotated>>),
    /// Complete recognition readings.
    Recognized(Vec<CompleteRecord<QuestionInput, CompleteRecognized>>),
    /// Complete relations retaining all entity originals.
    Related(CompleteRecord<Vec<QuestionInput>, CompleteRelated>),
}
/// Joined execution preserves an actual typed completed prefix on streamed failure.
#[derive(Debug)]
pub enum RequestOutcome {
    /// Successful native call with final facts.
    Complete(Call<RequestValue>),
    /// Completed values and the joined terminal failure.
    Failed {
        /// Actual ordered completed prefix.
        completed: RequestValue,
        /// Failure with final native facts.
        error: Error,
    },
}
