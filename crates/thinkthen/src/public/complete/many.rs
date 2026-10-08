//! Plain-original complete batches delegate to the one native record carrier.
use crate::public::{
    Call, CallOptions, CompleteChoice, CompleteDecision, CompleteFilter, CompleteRecord,
    CompleteScore, CompleteTags, DecisionQuestion, DetailQuestion, Engine, Error, InputEvidence,
    Question, RecordInput,
};

fn records<T>(original: T) -> RecordInput<T> {
    RecordInput {
        examples: None,
        original,
        context: None,
        options: None,
    }
}
impl Engine {
    /// Complete decisions for every eagerly admitted text or image original.
    ///
    /// # Errors
    /// As decide_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn decide_many_complete_with<Q, I, T>(
        &self,
        question: &Q,
        inputs: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error>
    where
        Q: DecisionQuestion + ?Sized,
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.decide_records_complete_with(question, inputs.into_iter().map(records), options)
    }
    /// Complete choices for every eagerly admitted text or image original.
    ///
    /// # Errors
    /// As choose_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn choose_many_complete_with<Q, I, T>(
        &self,
        question: &Q,
        inputs: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.choose_records_complete_with(question, inputs.into_iter().map(records), options)
    }
    /// Complete labels for every original, including rejected probabilities.
    ///
    /// # Errors
    /// As tag_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn tag_many_complete_with<Q, I, T>(
        &self,
        question: &Q,
        inputs: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.tag_records_complete_with(question, inputs.into_iter().map(records), options)
    }
    /// Complete scores for every eagerly admitted text or image original.
    ///
    /// # Errors
    /// As score_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn score_many_complete_with<I, T>(
        &self,
        question: &Question,
        inputs: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error>
    where
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.score_records_complete_with(question, inputs.into_iter().map(records), options)
    }
    /// Complete filter readings for every original, including rejections.
    ///
    /// # Errors
    /// As filter_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn filter_complete_with<I, T>(
        &self,
        question: &Question,
        inputs: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error>
    where
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.filter_records_complete_with(question, inputs.into_iter().map(records), options)
    }
}
