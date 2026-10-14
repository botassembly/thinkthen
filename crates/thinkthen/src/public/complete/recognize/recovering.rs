//! Independent recognition failures retain the existing per-record stage scheduler.
use super::records::{answer, prepare};
use crate::public::{
    Call, CallOptions, Engine, Error, QuestionInput, Recognize, RecordInput, RequestRecoveredRow,
    RequestValue, options::Stop,
};
impl Engine {
    pub(in crate::public) fn recover_recognition(
        &self,
        ask: &Recognize,
        records: Vec<Result<RecordInput<QuestionInput>, Error>>,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        ask.0.validate_mode().map_err(Error::refused)?;
        let engine = super::super::contextual(self.for_model(ask.0.model.as_ref())?, &options)?;
        let limit = options
            .cli_text_limit
            .unwrap_or(crate::engine::facade::MAX_TEXT_BYTES);
        let (admitted, rejected) = super::super::recovering::prepare(records, |_, record| {
            let unit = prepare(&engine, ask, record, options.context_text())?;
            unit.prepared.admit(limit)?;
            Ok((unit, ()))
        })?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(admitted.len() + rejected.len(), |cancel| {
            let cancel = cancel.with_storage_scope();
            let mut output = rejected;
            for (ordinal, unit, ()) in admitted {
                cancel.stop_or_remaining().map_err(Error::from)?;
                let outcome = answer(ordinal, unit, &stop, &cancel, false)
                    .map(|row| RequestValue::Recognized(vec![row]));
                output.push(crate::public::request::recovery_row(ordinal, outcome)?);
            }
            output.sort_by_key(RequestRecoveredRow::ordinal);
            Ok(output)
        })
    }
}
