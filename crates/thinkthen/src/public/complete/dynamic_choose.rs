//! Whole per-record choose shortlists enter the ordinary pulled scheduler lazily.
use super::records::{Held, Prepared, Records, prepare_record};
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Batch, CallOptions, CompleteChoice, CompleteRecord, Engine, Error, InputEvidence,
    InputFunction, RecordChooseQuestion, RecordInput,
};
use crate::public::{asking, bulk, options::Stop, pull};
use std::sync::Arc;

struct Original<T> {
    held: Held<T>,
    prepared: Prepared,
}

fn prepare<T: InputEvidence>(
    question: &RecordChooseQuestion,
    record: RecordInput<T>,
    fallback: Option<&str>,
    at: usize,
) -> Result<Original<T>, Error> {
    let candidates = record
        .options
        .as_ref()
        .ok_or_else(|| Error::usage("record choose requires candidates on every original"))?;
    let effective = question.with_options(candidates)?;
    let (held, prepared) = prepare_record(InputFunction::Choose, &effective, record, fallback, at)?;
    Ok(Original { held, prepared })
}

impl Engine {
    /// Pull fallible originals with a mandatory whole ordered shortlist on every record.
    /// # Errors
    /// Yields the completed prefix then one joined terminal error with final facts.
    pub fn try_choose_dynamic_records_complete_with<'a, I, T>(
        &'a self,
        question: &'a RecordChooseQuestion,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteChoice>>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(self.dynamic_choose_stream(
            question.clone(),
            records,
            options,
            options.context_text().map(str::to_owned),
        ))
    }

    pub(crate) fn dynamic_choose_stream<'a, I, T>(
        &self,
        question: RecordChooseQuestion,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
    ) -> Result<Batch<'a, CompleteRecord<T, CompleteChoice>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        let setting = bulk::selected_batch_file(question.batch.as_ref(), &options, self.batch)?;
        let engine = self.for_model(question.model.as_ref())?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let attempts = stop.facts().attempts().is_some();
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = attempts;
        let validates =
            question.metadata.item_schema.is_some() || question.metadata.context_schema.is_some();
        let preparing = Records(Arc::clone(&engine), validates);
        let profile = self.profile.clone();
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let original = record
                .and_then(|record| prepare(&question, record, context.as_deref(), at))
                .map_err(|error| error.at_record(at))?;
            preparing
                .asks(&original.prepared)
                .map_err(super::records::pipeline_failure)
                .map_err(|error| error.at_record(at))?;
            Ok(original)
        });
        let records = self.admit_prepared_stream(records, &options)?;
        let asker = Records(Arc::clone(&engine), validates);
        let call = pull::Call {
            engine: Arc::clone(&engine),
            stop,
            packing,
            most: self.most,
        };
        Ok(pull::try_start_prepared(
            call,
            asker,
            records,
            Box::new(|_, original: &Original<T>| Ok(original.prepared.duplicate())),
            Box::new(move |stop, at, original, row| {
                let held = match original {
                    Some(original) => original.held,
                    None => {
                        return Err(row
                            .err()
                            .map_or_else(
                                || Error::defect("a complete pulled row lost its original"),
                                asking::failure,
                            )
                            .at_record(at));
                    }
                };
                let keys = row
                    .as_ref()
                    .map_or_else(|_| Vec::new(), |row| row.keys.clone());
                let judged = bulk::judged(stop, &held.question, engine.backend(), at, row)
                    .map_err(|error| error.at_record(at))?;
                let mut run = super::batch_run(&engine, &held.question, profile.as_ref(), setting);
                run.context_sha256 = held.context_sha256;
                let events = attempts.then(|| judged.answered.attempts.clone());
                let mut canonical = super::atomic(
                    run,
                    &judged,
                    super::spec(
                        InputFunction::Choose,
                        &held.question,
                        judged.value.clone(),
                        at,
                    ),
                    keys,
                    None,
                    events,
                )
                .map_err(|_| super::wrong())?;
                canonical.source = held.source;
                canonical.images = held.images;
                Ok(Some(CompleteRecord {
                    original: held.original,
                    ordinal: at,
                    result: super::choice(canonical)?,
                }))
            }),
        ))
    }
}

pub(super) fn preview_asks(
    engine: &Arc<crate::engine::facade::Engine>,
    question: &RecordChooseQuestion,
    record: RecordInput<crate::QuestionInput>,
    fallback: Option<&str>,
    at: usize,
) -> Result<(Vec<crate::core::pack::Ask>, bool), Error> {
    let original = prepare(question, record, fallback, at)?;
    let dropped = crate::core::adapters::built_in::drops_detail_of(
        engine.backend().descriptions(),
        &original.held.question.core,
    );
    let asks = Records(Arc::clone(engine), false)
        .asks(&original.prepared)
        .map_err(super::records::pipeline_failure)?;
    Ok((asks, dropped))
}
