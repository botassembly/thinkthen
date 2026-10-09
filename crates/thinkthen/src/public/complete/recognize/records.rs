//! Recognize located records with one call, one budget and the existing stage scheduler.
use super::rendered;
use crate::engine::facade;
use crate::public::{
    Call, CallOptions, CompleteRecognized, CompleteRecord, Engine, Error, InputEvidence,
    InputFunction, QuestionInput, Recognize, Recognized, RecordInput, options::Stop,
};
use std::sync::Arc;

pub(super) struct Prepared {
    pub(super) input: Arc<QuestionInput>,
    pub(super) text: String,
    pub(super) engine: facade::Engine,
    pub(super) context: Option<String>,
    pub(super) ask: Recognize,
}
pub(super) struct Unit<T> {
    pub(super) original: T,
    pub(super) prepared: Prepared,
}
impl Engine {
    /// Recognize selected native record text while retaining each original and location.
    /// # Errors
    /// All first-stage input/profile boundaries are admitted before sending. Started
    /// stage failures retain the single call's completed prefix and final facts.
    #[allow(
        clippy::type_complexity,
        reason = "Original occurrences retain concrete complete results"
    )]
    pub fn recognize_records_complete_with<I, T>(
        &self,
        ask: &Recognize,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.try_recognize_records_complete_with(ask, records.into_iter().map(Ok), options)
    }

    /// Recognize a fallible native reader through the same eager admission and scheduler.
    /// # Errors
    /// Later reader errors, images and per-record controls refuse before any send.
    #[allow(
        clippy::type_complexity,
        reason = "Original occurrences retain concrete complete results"
    )]
    pub fn try_recognize_records_complete_with<I, T>(
        &self,
        ask: &Recognize,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        ask.0.validate_mode().map_err(Error::refused)?;
        let options = options.started()?;
        options.admission()?;
        let engine =
            crate::public::complete::contextual(self.for_model(ask.0.model.as_ref())?, &options)?;
        let units = self.try_within_admission(
            records.into_iter().enumerate().map(|(at, record)| {
                record
                    .and_then(|record| {
                        prepare(
                            &engine,
                            ask,
                            record,
                            options.context_text(),
                            facade::MAX_TEXT_BYTES,
                        )
                    })
                    .map_err(|error| error.at_record(at))
            }),
            &options,
        )?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(0, |cancel| {
            let cancel = cancel.with_storage_scope();
            let mut rows = Vec::new();
            for (
                at,
                Unit {
                    original,
                    prepared: unit,
                },
            ) in units.enumerate()
            {
                cancel.stop_or_remaining().map_err(Error::from)?;
                let before = stop.facts().attempts().map_or(0, <[_]>::len);
                let found = super::execute(
                    &unit.engine,
                    &unit.ask,
                    &unit.text,
                    &cancel,
                    &stop,
                    (at, Some(&unit.input)),
                )
                .map_err(|error| Error::from(error).at_record(at))?;
                let value = Recognized::from_native(found.value.clone())?;
                stop.observe(crate::public::RecordObservation::Row {
                    index: at,
                    value: crate::public::ObservedRow::Recognized(&value),
                });
                let attempts = stop
                    .facts()
                    .attempts()
                    .map(|attempts| attempts.iter().skip(before).cloned().collect());
                let mut result = rendered(
                    &unit.engine,
                    &unit.ask,
                    found,
                    value,
                    (at, unit.context.as_deref(), attempts),
                )?;
                result.canonical.source = super::super::physical_source(&unit.input);
                result.source_value = source_value(&result.canonical.value, &unit)?;
                rows.push(CompleteRecord {
                    original,
                    ordinal: at,
                    result,
                });
                cancel.finished_records(1);
            }
            Ok(rows)
        })
    }
}
pub(super) fn prepare<T: InputEvidence>(
    engine: &facade::Engine,
    ask: &Recognize,
    record: RecordInput<T>,
    fallback: Option<&str>,
    limit: usize,
) -> Result<Unit<T>, Error> {
    if record.options.is_some() {
        return Err(Error::usage("recognize takes no per-record options"));
    }
    let mut ask = match record.examples {
        Some(examples) => ask.clone().with_examples(examples)?,
        None => ask.clone(),
    };
    if let Some(seeds) = record.seed_spans {
        ask = ask.with_seed_spans(seeds);
    }
    ask.0.metadata.validate_context(record.context.as_ref())?;
    let resolved = crate::public::RecordContext::resolved(record.context.as_ref(), fallback)?;
    let context = resolved
        .as_ref()
        .map(|evidence| {
            evidence
                .as_text()
                .map(|text| text.into_owned())
                .map_err(Error::refused)
        })
        .transpose()?;
    let engine = engine.clone().with_aggregate_context_value(resolved);
    let input = Arc::new(record.original.question_input());
    ask.0.metadata.validate_item(&input)?;
    crate::public::images::guard(InputFunction::Recognize, &input)?;
    let text = match input.as_ref() {
        QuestionInput::Text(text) => text.clone(),
        QuestionInput::Record(record) => {
            admit_source(record)?;
            record.plain().to_owned()
        }
        QuestionInput::Images(_) => return Err(crate::public::complete::wrong()),
    };
    engine
        .admit_recognition_limit(&ask.0, &text, limit)
        .map_err(Error::from)?;
    Ok(Unit {
        original: record.original,
        prepared: Prepared {
            input,
            text,
            engine,
            context,
            ask,
        },
    })
}

pub(super) fn source_value(
    value: &crate::core::RecognizedValue,
    unit: &Prepared,
) -> Result<Option<crate::public::SourceRecognition>, Error> {
    let QuestionInput::Record(input) = unit.input.as_ref() else {
        return Ok(None);
    };
    input
        .location()
        .map(|location| crate::public::SourceRecognition::of(value, &unit.text, location))
        .transpose()
}

fn admit_source(record: &crate::public::RecordEvidence) -> Result<(), Error> {
    let Some(location) = record.location() else {
        return Ok(());
    };
    if record.original().literal().is_none() {
        return Err(Error::usage(
            "located recognize takes literal text units, not decoded JSON fields",
        ));
    }
    if let (Some(first_line), Some(last_line)) = (location.first_line(), location.last_line()) {
        let text = record.plain();
        crate::public::SourceRecord {
            record: text,
            file: String::new(),
            first_line,
            last_line,
        }
        .span_lines(0, text.chars().count())?;
    }
    Ok(())
}
