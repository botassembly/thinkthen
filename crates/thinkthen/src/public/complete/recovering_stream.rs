//! Recovering lazy rows retain the existing bounded pull scheduler and renderer.
use super::records::{Held, Prepared, Records, prepare_record};
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Batch, CallOptions, Engine, Error, InputFunction, Question, QuestionInput, RecordInput,
    RequestRecoveredRow, options::Stop, pull,
};
use std::sync::Arc;

struct Original {
    held: Held<QuestionInput>,
    prepared: Prepared,
}
impl Engine {
    pub(in crate::public) fn recover_atomic_stream<'a, I>(
        &self,
        function: InputFunction,
        question: Question,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
    ) -> Result<Batch<'a, RequestRecoveredRow>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a,
    {
        let setting = crate::public::bulk::selected_batch(&question, &options, self.batch)?;
        let engine = self.asking(&question)?;
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| {
                    prepare_record(function, &question, record, context.as_deref(), at)
                })
                .map_err(|error| error.at_record(at))?;
            Ok(Original { held, prepared })
        });
        self.recover_prepared_stream(engine, setting, function, records, options)
    }
    pub(in crate::public) fn recover_dynamic_choose_stream<'a, I>(
        &self,
        question: crate::RecordChooseQuestion,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
    ) -> Result<Batch<'a, RequestRecoveredRow>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a,
    {
        let setting = crate::public::bulk::selected_batch_file(
            question.batch.as_ref(),
            &options,
            self.batch,
        )?;
        let engine = self.for_model(question.model.as_ref())?;
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| {
                    super::dynamic_choose::prepare_parts(&question, record, context.as_deref(), at)
                })
                .map_err(|error| error.at_record(at))?;
            Ok(Original { held, prepared })
        });
        self.recover_prepared_stream(engine, setting, InputFunction::Choose, records, options)
    }
    fn recover_prepared_stream<'a, I>(
        &self,
        engine: Arc<crate::engine::facade::Engine>,
        setting: crate::core::Setting,
        function: InputFunction,
        records: I,
        options: CallOptions<'a>,
    ) -> Result<Batch<'a, RequestRecoveredRow>, Error>
    where
        I: Iterator<Item = Result<Original, Error>> + 'a,
    {
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let mut packing = pull::packing(setting, false, true);
        packing.detailed = stop.facts().attempts().is_some();
        let preparing = Records(Arc::clone(&engine), true);
        let records = records.enumerate().map(move |(at, original)| {
            let original = original?;
            preparing
                .asks(&original.prepared)
                .map_err(super::records::pipeline_failure)
                .map_err(|error| error.at_record(at))?;
            Ok(original)
        });
        let records = self.admit_prepared_stream(records, &options)?;
        let asker = Records(Arc::clone(&engine), true);
        let profile = self.profile.clone();
        let call = pull::Call {
            engine: Arc::clone(&engine),
            stop,
            packing,
            most: self.most,
            pending_inputs: options.pending_inputs,
        };
        Ok(pull::try_start_prepared(
            call,
            asker,
            records,
            Box::new(|_, original: &Original| Ok(original.prepared.clone())),
            Box::new(move |stop, at, original, row| {
                let original = match original {
                    Some(original) => original,
                    None => {
                        return Err(row
                            .err()
                            .map_or_else(
                                || Error::defect("a recovering row lost its original"),
                                crate::public::asking::failure,
                            )
                            .at_record(at));
                    }
                };
                let cause = super::records::recovery_cause(&row);
                let outcome = super::records::complete_row(
                    (&engine, profile.as_ref(), setting, false, function),
                    stop,
                    &original.held,
                    at,
                    row,
                )
                .and_then(|canonical| {
                    super::recovering::atomic_value(function, original.held.original, at, canonical)
                });
                crate::public::request::recovery_row(at, outcome)
                    .map(|row| Some(row.with_cause(cause)))
            }),
        ))
    }
}
