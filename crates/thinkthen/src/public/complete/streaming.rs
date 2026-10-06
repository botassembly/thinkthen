//! Fallible located originals use the ordinary pulled and joined scheduler.
use super::records::{Held, Prepared, Records, prepare_record};
use crate::core;
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Batch, CallOptions, CompleteChoice, CompleteDecision, CompleteFilter, CompleteRecord,
    CompleteScore, CompleteTags, DecisionQuestion, DetailQuestion, Engine, Error, InputEvidence,
    InputFunction, Question, RecordInput,
};
use crate::public::{options::Stop, pull};
use std::sync::Arc;

struct Original<T> {
    held: Held<T>,
    prepared: Prepared,
}

impl Engine {
    /// Pull fallible originals into complete decisions, retaining contexts and locations.
    /// # Errors
    /// Yields the completed prefix then one joined terminal error with final facts.
    pub fn try_decide_records_complete_with<'a, Q, I, T>(
        &'a self,
        question: &'a Q,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteDecision>>
    where
        Q: DecisionQuestion + ?Sized,
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(self.complete_stream(
            InputFunction::Decide,
            question.question(),
            records,
            options,
            super::decision,
        ))
    }

    /// Pull fallible originals with complete ordered replacement choose shortlists.
    /// # Errors
    /// Yields the completed prefix then one terminal input/backend/stop error.
    pub fn try_choose_records_complete_with<'a, Q, I, T>(
        &'a self,
        question: &'a Q,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteChoice>>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(self.complete_stream(
            InputFunction::Choose,
            question.question(),
            records,
            options,
            super::choice,
        ))
    }

    /// Pull fallible originals with every accepted and rejected tag probability.
    /// # Errors
    /// Unsupported controls/images refuse their row before sending it.
    pub fn try_tag_records_complete_with<'a, Q, I, T>(
        &'a self,
        question: &'a Q,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteTags>>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(self.complete_stream(
            InputFunction::Tag,
            question.question(),
            records,
            options,
            super::tags,
        ))
    }

    /// Pull fallible originals with complete graded score answers and per-record context.
    /// # Errors
    /// Yields the completed prefix then a joined terminal error with final facts.
    pub fn try_score_records_complete_with<'a, I, T>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteScore>>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(self.complete_stream(
            InputFunction::Score,
            question,
            records,
            options,
            super::score,
        ))
    }

    /// Pull complete filter judgments for every original, including rejected rows.
    /// # Errors
    /// Yields the completed prefix then a joined terminal error with final facts.
    pub fn try_filter_records_complete_with<'a, I, T>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteFilter>>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(self.complete_stream(
            InputFunction::Filter,
            question,
            records,
            options,
            super::records::filter,
        ))
    }

    fn complete_stream<'a, I, T, R: 'a>(
        &'a self,
        function: InputFunction,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
        convert: impl Fn(core::CompleteAtomic) -> Result<R, Error> + 'a,
    ) -> Result<Batch<'a, CompleteRecord<T, R>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        super::admitted(function, question)?;
        let setting = crate::public::bulk::selected_batch(question, &options, self.batch)?;
        let engine = self.asking(question)?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let attempts = stop.facts().attempts().is_some();
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = attempts;
        let preparing = Records(Arc::clone(&engine));
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| {
                    prepare_record(function, question, record, options.context_text(), at)
                })
                .map_err(|error| error.at_record(at))?;
            preparing
                .asks(&prepared)
                .map_err(super::records::pipeline_failure)
                .map_err(|error| error.at_record(at))?;
            Ok(Original { held, prepared })
        });
        let asker = Records(Arc::clone(&engine));
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
                let effective = original
                    .as_ref()
                    .map_or(question, |original| &original.held.question);
                let keys = row
                    .as_ref()
                    .map_or_else(|_| Vec::new(), |row| row.keys.clone());
                let judged =
                    crate::public::bulk::judged(stop, effective, engine.backend(), at, row)
                        .map_err(|error| error.at_record(at))?;
                let Original { held, .. } = original
                    .ok_or_else(|| Error::defect("a complete pulled row lost its original"))?;
                let mut run = super::run(&engine, &held.question, self.profile.as_ref());
                run.batch_setting = Some(setting.into());
                run.context_sha256 = held.context_sha256;
                let events = attempts.then(|| judged.answered.attempts.clone());
                let canonical = super::atomic(
                    run,
                    &judged,
                    super::spec(function, &held.question, judged.value.clone(), at),
                    keys,
                    None,
                    events,
                )
                .map_err(|_| super::wrong())?;
                Ok(Some(CompleteRecord {
                    original: held.original,
                    ordinal: at,
                    result: convert(canonical)?,
                }))
            }),
        ))
    }
}
