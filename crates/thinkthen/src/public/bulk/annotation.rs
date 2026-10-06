//! Grouped annotation of each input record on the question pipeline, by ADR
//! 0111 section 5. Every group's questions share the fixed state, so records
//! and groups pack together, and each row gathers its questions back.

use std::sync::Arc;

use super::{Values, evidence, observe_annotated, observe_annotated_questions, selected_set_batch};
use crate::core::{self, pack, pack::Ask, quoted_plan};
use crate::engine::facade::{self, Annotation, GroupAnswer, QuestionAnswer};
use crate::engine::pipeline::{Answered, Asker, Failed, Flow};
use crate::public::annotated::AnnotatedRecord;
use crate::public::asking::{Text, backend_failed, packed};
use crate::public::batch::Batch;
use crate::public::engine::{Engine, Evidence};
use crate::public::error::{Error, ErrorKind};
use crate::public::options::{CallOptions, Stop};
use crate::public::pull;
use crate::public::results::{Call, ObservedQuestion, Written};
use crate::public::set::QuestionSet;

/// What one question set needs of each text beside the text.
pub(crate) struct Annotating {
    set: core::QuestionSet,
    groups: Vec<Vec<usize>>,
    backend: core::Backend,
    profile: Option<core::BackendProfile>,
    context: Option<core::Json>,
}

impl Annotating {
    pub(crate) fn with_context(mut self, context: Option<&str>) -> Result<Self, Error> {
        self.context = context
            .filter(|text| !text.is_empty())
            .map(|text| {
                evidence(text)?;
                Ok::<_, Error>(core::Json::String(text.to_owned()))
            })
            .transpose()?;
        Ok(self)
    }

    pub(crate) fn with_typed_context(mut self, context: Option<&core::Evidence>) -> Self {
        self.context = context.map(core::Evidence::as_json);
        self
    }

    pub(crate) fn new(engine: &facade::Engine, set: core::QuestionSet) -> Self {
        Self {
            groups: set.groups(),
            set,
            backend: engine.backend().clone(),
            profile: engine.profile().cloned(),
            context: None,
        }
    }

    fn question(&self, place: usize) -> Result<core::Question, Error> {
        self.set
            .questions()
            .get(place)
            .map(|named| named.question().clone())
            .ok_or_else(|| Error::defect("a group points outside its set"))
    }
}

impl Asker for Annotating {
    type Input = Text;
    type Row = Annotation;
    type Error = Error;

    fn label(&self, text: &Text) -> usize {
        text.at
    }

    fn asks(&self, text: &Text) -> Result<Vec<Ask>, Error> {
        let record = match &text.input {
            crate::public::QuestionInput::Record(record) if record.images().is_empty() => {
                record.batch_record()
            }
            _ => record(
                &self.set,
                text.plain(crate::public::InputFunction::Annotate)?,
            )?,
        };
        let mut asks = Vec::new();
        for places in &self.groups {
            let evidence =
                self.set
                    .group_evidence(places, &record)
                    .map_err(|error| match error {
                        core::PartError::Declaration(_) => {
                            Error::usage("the item does not match item_schema")
                        }
                        core::PartError::Record(error) => Error::refused(error),
                        core::PartError::Reading(_) => {
                            Error::defect("a checked question set could not read its parts")
                        }
                    })?;
            let questions = places
                .iter()
                .map(|&place| self.question(place))
                .collect::<Result<Vec<_>, _>>()?;
            let plan = quoted_plan(
                self.backend.asked(),
                evidence,
                None,
                questions,
                self.profile.as_ref(),
            )
            .map_err(|error| match error {
                core::BatchError::Profile(_) => Error::refused(error),
                _ => Error::defect("an annotate group asks nothing"),
            })?;
            asks.extend(
                pack::asks(self.backend.url(), &plan)
                    .map_err(|_| Error::defect("a request could not be written as JSON"))?,
            );
        }
        if let Some(context) = &self.context {
            let model = pack::model_json(self.backend.model().as_str())
                .map_err(|_| Error::defect("an aggregate model could not be written"))?;
            for ask in &mut asks {
                ask.state = ask
                    .state
                    .with_context_value(context)
                    .map_err(|_| Error::defect("an aggregate context could not be written"))?;
                ask.key = ask.state.key(self.backend.url(), &model, &ask.question);
            }
        }
        Ok(asks)
    }

    fn row(&self, _text: Text, answers: Vec<Answered>) -> Result<Annotation, Error> {
        let mut rest = answers.as_slice();
        let mut groups = Vec::with_capacity(self.groups.len());
        for places in &self.groups {
            let mut questions = Vec::with_capacity(places.len());
            for &place in places {
                let question = self.question(place)?;
                let (own, after) = rest
                    .split_at_checked(pack::wire_count(&question))
                    .ok_or_else(|| Error::defect("an annotate question lost its answers"))?;
                rest = after;
                questions.push(QuestionAnswer::read(place, question, own).map_err(Error::from)?);
            }
            groups.push(GroupAnswer::of_questions(questions));
        }
        facade::assemble(&self.set, groups, self.backend.model()).map_err(Error::from)
    }
}

/// The public error for one annotate row with no annotation.
fn failure(failed: Failed<Error>) -> Error {
    match failed {
        Failed::Asker(error) => error,
        Failed::Pack { error, .. } => packed(error),
        Failed::Engine { error, .. } | Failed::Stopped(error) => Error::from(error),
    }
}

impl Engine {
    /// Each record with every value of the set, lazily, in input order. A
    /// member with an `on` pointer reads that part of the record's JSON text,
    /// and a record missing a part is refused before its first request. A
    /// question the backend failed reads [`Annotated::Failed`](crate::Annotated::Failed).
    pub fn annotate<'a, I>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
    ) -> Batch<'a, AnnotatedRecord<I::Item>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.annotate_with(questions, records, CallOptions::new())
    }

    /// [`Engine::annotate`] under these controls.
    pub fn annotate_with<'a, I>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, AnnotatedRecord<I::Item>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.try_annotate_with(questions, records.into_iter().map(Ok), options)
    }

    /// Fallible input for [`Engine::annotate_with`], preserving input order.
    /// A reader failure ends the batch after its earlier admitted rows.
    pub fn try_annotate_with<'a, I, T>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, AnnotatedRecord<T>>
    where
        I: IntoIterator<Item = Result<T, Error>> + 'a,
        T: Evidence + 'a,
    {
        Batch::of((|| {
            let setting = selected_set_batch(&questions.0, &options, self.batch)?;
            let stop = Stop::begin(options)?.with_prices(self.prices);
            let (engine, set) = (Arc::clone(&self.inner), questions.0.clone());
            let asker =
                Annotating::new(&engine, set.clone()).with_context(options.context_text())?;
            let call = pull::Call {
                packing: pull::packing(setting, false, false),
                engine: Arc::clone(&engine),
                stop,
                most: self.most,
            };
            Ok(pull::try_start(
                call,
                asker,
                records.into_iter(),
                Box::new(move |stop, index, item, row| {
                    row_of(&set, &engine, stop, index, item, row).map(Some)
                }),
            ))
        })())
    }
}

impl Engine {
    /// [`Engine::annotate_with`] over texts in hand, for an SQL host. A
    /// record refused before its request, such as one missing an `on` part,
    /// fails alone, and every other record is still asked, answered and
    /// stored, by ADR 0111 section 4. Each row holds its text's place. Any
    /// other failure ends the call, as it ends `annotate_with`. More texts
    /// than the engine's record limit are refused before any send.
    ///
    /// # Errors
    ///
    /// Returns the call's fatal error or a refusal before any send.
    #[doc(hidden)]
    pub fn annotate_each_with(
        &self,
        questions: &QuestionSet,
        texts: &[String],
        options: CallOptions<'_>,
    ) -> Result<Call<EachRow>, Error> {
        let setting = selected_set_batch(&questions.0, &options, self.batch)?;
        let inputs = self
            .within_limit(texts)?
            .enumerate()
            .map(|(at, text)| Text {
                at,
                input: crate::public::QuestionInput::Text(text.clone()),
            })
            .collect();
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let (engine, set) = (Arc::clone(&self.inner), &questions.0);
        let asker = Annotating::new(&engine, set.clone()).with_context(options.context_text())?;
        let packing = pull::packing(setting, false, false);
        stop.run_call(texts.len(), |cancel| {
            let mut rows = Vec::with_capacity(texts.len());
            let mut ended = None;
            let host = crate::engine::pipeline::eager(inputs, |row: pull::Row<Annotating>| {
                let row = each(set, &engine, &stop, rows.len(), row);
                keep(&mut rows, &mut ended, row)
            });
            engine
                .ask_all(&asker, packing, host, cancel)
                .map_err(Error::from)?;
            if let Some(error) = ended {
                return Err(error);
            }
            if rows.len() != texts.len() {
                return Err(Error::defect("an annotate call lost its rows"));
            }
            Ok(rows)
        })
    }
}

/// One row per text of [`Engine::annotate_each_with`]: its record or its refusal.
type EachRow = Vec<Result<AnnotatedRecord<usize>, Error>>;

/// One text's row for an SQL host, or the error that ends the call. A record
/// refused before its request is the row's own failure.
fn each(
    set: &core::QuestionSet,
    engine: &facade::Engine,
    stop: &Stop<'_>,
    at: usize,
    row: pull::Row<Annotating>,
) -> Result<Result<AnnotatedRecord<usize>, Error>, Error> {
    match row {
        Err(Failed::Asker(error)) if error.kind() == ErrorKind::Usage => Ok(Err(error)),
        Err(Failed::Pack { error, .. }) => Ok(Err(packed(error))),
        row => row_of(set, engine, stop, at, Some(at), row).map(Ok),
    }
}

/// Keep one row, or hold the error that ends the call and stop.
fn keep<T>(rows: &mut Vec<T>, ended: &mut Option<Error>, row: Result<T, Error>) -> Flow {
    match row {
        Ok(row) => {
            rows.push(row);
            Flow::Continue
        }
        Err(error) => {
            *ended = Some(error);
            Flow::Stop
        }
    }
}

/// One record's annotated row on the calling thread. A record whose every
/// question failed stops the batch after its question events.
fn row_of<T>(
    set: &core::QuestionSet,
    engine: &facade::Engine,
    stop: &Stop<'_>,
    index: usize,
    item: Option<T>,
    row: pull::Row<Annotating>,
) -> Result<AnnotatedRecord<T>, Error> {
    let annotation = row.map_err(failure)?;
    let all_failed = annotation
        .values
        .iter()
        .all(|(_, value)| matches!(value, core::AnnotatedValue::Failed(_)));
    let value = rendered(set, engine, annotation, stop.observing())?;
    if all_failed {
        observe_annotated_questions(&value, index, stop)?;
        return Err(backend_failed());
    }
    observe_annotated(&value, index, stop)?;
    let item = item.ok_or_else(|| Error::defect("a row arrived with no record"))?;
    Ok(AnnotatedRecord::new(item, value.values, value.json))
}

/// Render a row once all its questions arrive.
pub(crate) fn rendered(
    set: &core::QuestionSet,
    engine: &facade::Engine,
    annotation: Annotation,
    observing: bool,
) -> Result<Values, Error> {
    let observed = if observing {
        let model = annotation
            .model
            .as_ref()
            .unwrap_or(engine.backend().model());
        annotation
            .details
            .iter()
            .zip(&annotation.receipts)
            .zip(set.questions())
            .map(|(((name, entry), receipt), named)| {
                let detail = ObservedQuestion::from_annotated(
                    entry,
                    set.profile(),
                    engine.backend(),
                    model,
                    (receipt.usage, receipt.requests_sent, receipt.replayed),
                )?
                .with_receipt(&receipt.trace)
                .with_threshold(named.threshold())
                .with_declarations(named.metadata());
                Ok((name.clone(), detail))
            })
            .collect::<Result<Vec<_>, Error>>()?
    } else {
        Vec::new()
    };
    let json = Written::of(&core::NamedValues::new(annotation.values.clone()))?;
    Ok(Values {
        values: annotation.values,
        json,
        observed,
    })
}

/// One record as its groups read it. Only a part group parses the text, once,
/// as the command reads a whole document, so a root-only set sends it as given.
pub(crate) fn record(_set: &core::QuestionSet, text: &str) -> Result<core::BatchRecord, Error> {
    let evidence = evidence(text)?;
    let usage = |error: core::RecordError| Error::usage(error.to_string());
    let reading = core::Reading::new(core::Framing::Document, Vec::new())
        .map_err(|_| Error::defect("a document reading takes no pointer"))?;
    let held = reading.annotation_record(text.as_bytes()).map_err(usage)?;
    let value = reading.batch_record(&held).map_err(usage)?.value;
    Ok(core::BatchRecord { evidence, value })
}
