//! Fallible located originals use the ordinary pulled and joined scheduler.
use super::records::{Held, Prepared, Records, prepare_record};
use crate::core;
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Batch, CallOptions, CompleteRecord, Engine, Error, InputEvidence, InputFunction, Question,
    RecordInput,
};
use crate::public::{options::Stop, pull};
use std::sync::Arc;

struct Original<T> {
    held: Held<T>,
    prepared: Prepared,
}

impl Engine {
    #[expect(
        clippy::too_many_arguments,
        reason = "owned context stays separate from borrowed caller controls"
    )]
    pub(crate) fn complete_stream<'a, I, T, R: 'a>(
        &self,
        function: InputFunction,
        question: Question,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
        convert: impl Fn(core::CompleteAtomic) -> Result<R, Error> + 'a,
    ) -> Result<Batch<'a, CompleteRecord<T, R>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        self.complete_stream_project(
            function,
            question,
            records,
            options,
            context,
            move |original, ordinal, canonical| {
                Ok(Some(CompleteRecord {
                    original,
                    ordinal,
                    result: convert(canonical)?,
                }))
            },
        )
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "one private pipeline owns typed result projections"
    )]
    pub(crate) fn complete_stream_project<'a, I, T, R: 'a>(
        &self,
        function: InputFunction,
        question: Question,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
        project: impl Fn(T, usize, core::CompleteAtomic) -> Result<Option<R>, Error> + 'a,
    ) -> Result<Batch<'a, R>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        let scalar = options.scalar_input;
        let setting = if scalar {
            core::Setting::Max
        } else {
            crate::public::bulk::selected_batch(&question, &options, self.batch)?
        };
        let engine = self.asking(&question)?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let attempts = stop.facts().attempts().is_some();
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = attempts;
        let validates =
            question.metadata.item_schema.is_some() || question.metadata.context_schema.is_some();
        let preparing = Records(Arc::clone(&engine), validates);
        let profile = self.profile.clone();
        let most = self.most;
        let preparing_question = question.clone();
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| {
                    prepare_record(
                        function,
                        &preparing_question,
                        record,
                        context.as_deref(),
                        at,
                    )
                })
                .map_err(|error| error.at_record(at))?;
            preparing
                .asks(&prepared)
                .map_err(super::records::pipeline_failure)
                .map_err(|error| error.at_record(at))?;
            Ok(Original { held, prepared })
        });
        let records = self.admit_prepared_stream(records, &options)?;
        let asker = Records(Arc::clone(&engine), validates);
        let call = pull::Call {
            engine: Arc::clone(&engine),
            stop,
            packing,
            most,
            pending_inputs: options.pending_inputs,
        };
        Ok(pull::try_start_prepared(
            call,
            asker,
            records,
            Box::new(|_, original: &Original<T>| Ok(original.prepared.clone())),
            Box::new(move |stop, at, original, row| {
                if original.is_none() {
                    crate::public::bulk::judged(stop, &question, engine.backend(), at, row)?;
                    return Err(Error::defect("a complete pulled row lost its original"));
                }
                let Original { held, .. } = original
                    .ok_or_else(|| Error::defect("a complete pulled row lost its original"))?;
                let canonical = super::records::complete_row(
                    (&engine, profile.as_ref(), setting, scalar, function),
                    stop,
                    &held,
                    at,
                    row,
                )?;
                project(held.original, at, canonical)
            }),
        ))
    }
}
