//! Recover independent annotations on the ordinary grouped question scheduler.
use super::{Annotations, complete_row, prepare_record};
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Call, CallOptions, CompleteRecord, Engine, Error, QuestionInput, QuestionSet, RecordInput,
    RequestRecoveredRow, RequestValue, options::Stop, pull,
};
use std::sync::Arc;
impl Engine {
    pub(in crate::public) fn recover_annotations(
        &self,
        questions: &QuestionSet,
        records: Vec<Result<RecordInput<QuestionInput>, Error>>,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let engine = Arc::clone(&self.inner);
        let asker = Annotations {
            recover_missing: true,
            require_usable_groups: false,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let (admitted, rejected) =
            super::super::recovering::prepare(records, |ordinal, record| {
                let (held, prepared) =
                    prepare_record(&questions.0, record, options.context_text(), ordinal)?;
                asker.asks(&prepared)?;
                Ok((held, prepared))
            })?;
        let mut packing = pull::packing(setting, false, true);
        packing.detailed = stop.facts().attempts().is_some();
        super::super::recovering::run(
            (&engine, packing),
            &asker,
            &stop,
            admitted,
            rejected,
            |ordinal, held, row| {
                let result =
                    complete_row(&engine, &questions.0, &stop, &held, (ordinal, false), row)?;
                Ok(RequestValue::Annotations(vec![CompleteRecord {
                    original: held.original,
                    ordinal,
                    result,
                }]))
            },
        )
    }
}
