//! Native original occurrences annotate on the existing ordered group pipeline.
use crate::core::{self, pack::Ask};
use crate::engine::{
    facade,
    pipeline::{self, Asker, Flow},
};
use crate::public::{
    Call, CallOptions, CompleteAnnotated, CompleteRecord, Engine, Error, InputEvidence,
    InputFunction, QuestionInput, QuestionSet, RecordInput,
};
use crate::public::{asking::Text, bulk::annotation::Annotating, options::Stop, pull};
use std::sync::Arc;
mod render;
mod streaming;

struct Prepared {
    text: Text,
    explicit_context: bool,
    context: Option<core::Evidence>,
}
struct Held<T> {
    original: T,
    input: core::Record,
    question_input: Arc<QuestionInput>,
    context: Option<core::Evidence>,
}
struct Annotations {
    recover_missing: bool,
    cli_groups: bool,
    engine: Arc<facade::Engine>,
    set: core::QuestionSet,
}
impl Annotations {
    fn asks_with_groups(
        &self,
        input: &Prepared,
        group_done: impl FnMut(usize, usize),
    ) -> Result<Vec<Ask>, Error> {
        let asks = Annotating::new(&self.engine, self.set.clone())
            .with_typed_context(input.context.as_ref(), input.explicit_context)
            .asks_with_groups(&input.text, group_done)?;
        if input.context.is_some() {
            super::records::validate_context(&self.engine, &asks)?;
        }
        Ok(asks)
    }
}
impl Asker for Annotations {
    type Input = Prepared;
    type Row = facade::Annotation;
    type Error = Error;
    fn recovers(&self, error: &Error) -> bool {
        self.recover_missing && error.missed_pointer().is_some()
    }
    fn refuses_batch(&self, error: &Error) -> bool {
        !self.cli_groups || error.missed_pointer().is_none()
    }
    fn validates_batches(&self) -> bool {
        self.set.questions().iter().any(|member| {
            member.metadata().item_schema.is_some() || member.metadata().context_schema.is_some()
        })
    }
    fn label(&self, input: &Prepared) -> usize {
        input.text.at
    }
    fn asks(&self, input: &Prepared) -> Result<Vec<Ask>, Error> {
        self.asks_with_groups(input, |_, _| {})
    }

    fn row(
        &self,
        input: Prepared,
        answers: Vec<pipeline::Answered>,
    ) -> Result<facade::Annotation, Error> {
        Annotating::new(&self.engine, self.set.clone())
            .require_usable_groups(self.cli_groups)
            .row(input.text, answers)
    }
}
impl Engine {
    /// Annotate every original, retaining successful null and failed member identities.
    /// # Errors
    /// Eager invalid records refuse before sending; all-failed rows and stopped calls retain final facts.
    #[allow(
        clippy::type_complexity,
        reason = "Every original retains its concrete complete annotation"
    )]
    pub fn annotate_records_complete_with<I, T>(
        &self,
        questions: &QuestionSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        let options = options.started()?;
        options.admission()?;
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let (held, inputs) = prepare(
            &questions.0,
            self.within_admission(records, &options)?,
            &options,
        )?;
        let engine = Arc::clone(&self.inner);
        let asker = Annotations {
            recover_missing: false,
            cli_groups: false,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        for input in &inputs {
            asker.asks(input)?;
        }
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = stop.facts().attempts().is_some();
        stop.run_call(0, |cancel| {
            let mut rows = Rows {
                held: &held,
                engine: &engine,
                set: &questions.0,
                stop: &stop,
                values: Vec::new(),
                failure: None,
            };
            let host = pipeline::eager(inputs, |row| {
                let flow = rows.take(row);
                cancel.finished_records(usize::from(flow == Flow::Continue));
                flow
            });
            engine
                .ask_all(&asker, packing, host, cancel)
                .map_err(Error::from)?;
            if let Some(error) = rows.failure {
                return Err(error);
            }
            if rows.values.len() != held.len() {
                return Err(Error::defect("an annotate call lost an original"));
            }
            Ok(rows
                .values
                .into_iter()
                .zip(held)
                .enumerate()
                .map(|(at, (result, held))| CompleteRecord {
                    original: held.original,
                    ordinal: at,
                    result,
                })
                .collect())
        })
    }
    /// Annotate plain originals using one optional shared context.
    /// # Errors
    /// As annotate_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Every original retains its concrete complete annotation"
    )]
    pub fn annotate_complete_with<I, T>(
        &self,
        questions: &QuestionSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error>
    where
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.annotate_records_complete_with(
            questions,
            records.into_iter().map(|original| RecordInput {
                examples: None,
                seed_spans: None,
                original,
                context: None,
                options: None,
            }),
            options,
        )
    }
}

type Admission<T> = (Vec<Held<T>>, Vec<Prepared>);
fn prepare<T: InputEvidence>(
    set: &core::QuestionSet,
    records: impl Iterator<Item = RecordInput<T>>,
    options: &CallOptions<'_>,
) -> Result<Admission<T>, Error> {
    let mut held = Vec::new();
    let mut inputs = Vec::new();
    for (at, record) in records.enumerate() {
        options.admission()?;
        let (item, input) = prepare_record(set, record, options.context_text(), at)?;
        options.admission()?;
        held.push(item);
        inputs.push(input);
    }
    Ok((held, inputs))
}
fn failed(row: pipeline::Failed<Error>) -> Error {
    match row {
        pipeline::Failed::Asker(error) => error,
        pipeline::Failed::Pack { error, at } => crate::public::asking::packed(error).at_record(at),
        pipeline::Failed::Engine { error, first, last } => Error::from(error.clone())
            .with_diagnostic(crate::public::error::diagnostic::Diagnostic::EngineRange {
                cause: error,
                first,
                last,
            }),
        pipeline::Failed::Stopped(error) => Error::from(error),
    }
}

struct Rows<'a, 'o, T> {
    held: &'a [Held<T>],
    engine: &'a facade::Engine,
    set: &'a core::QuestionSet,
    stop: &'a Stop<'o>,
    values: Vec<CompleteAnnotated>,
    failure: Option<Error>,
}
impl<T: InputEvidence> Rows<'_, '_, T> {
    fn take(&mut self, row: pull::Row<Annotations>) -> Flow {
        match self.completed(row) {
            Ok(value) => {
                self.values.push(value);
                Flow::Continue
            }
            Err(error) => {
                self.failure = Some(error.at_record(self.values.len()));
                Flow::Stop
            }
        }
    }
    fn completed(&self, row: pull::Row<Annotations>) -> Result<CompleteAnnotated, Error> {
        let at = self.values.len();
        let held = self.held.get(at).ok_or_else(super::wrong)?;
        complete_row(self.engine, self.set, self.stop, held, at, row)
    }
}

fn complete_row<T>(
    engine: &facade::Engine,
    set: &core::QuestionSet,
    stop: &Stop<'_>,
    held: &Held<T>,
    at: usize,
    row: pull::Row<Annotations>,
) -> Result<CompleteAnnotated, Error> {
    let annotation = row.map_err(failed)?;
    let mut observed = crate::public::bulk::annotation::rendered(
        set,
        engine,
        annotation.clone(),
        stop.observing(),
    )?;
    for (_, detail) in &mut observed.observed {
        *detail = detail.clone().with_input(Arc::clone(&held.question_input));
    }
    crate::public::bulk::observe_annotated(&observed, at, stop)?;
    if annotation.failed_questions == annotation.details.len() && !annotation.details.is_empty() {
        return Err(crate::public::asking::backend_failed());
    }
    render::complete(
        engine,
        set,
        annotation,
        held,
        at,
        stop.facts().attempts().is_some(),
    )
}

fn prepare_record<T: InputEvidence>(
    set: &core::QuestionSet,
    record: RecordInput<T>,
    fallback: Option<&str>,
    at: usize,
) -> Result<(Held<T>, Prepared), Error> {
    record
        .admit_recognition_controls(
            crate::public::request::RequestFunction::from_input(InputFunction::Annotate),
            "record examples are admitted only for recognize",
            "record seed spans are admitted only for recognize",
        )
        .map_err(|error| error.at_record(at))?;
    if record.options.is_some() {
        return Err(Error::usage("record options are admitted only for choose"));
    }
    for member in set.questions() {
        member
            .metadata()
            .validate_context(record.context.as_ref())
            .map_err(|error| error.at_record(at))?;
    }
    let question_input = record.original.question_input();
    crate::public::images::guard(InputFunction::Annotate, &question_input)?;
    let input = match &question_input {
        QuestionInput::Text(text) => {
            crate::public::engine::evidence(text)?;
            let reading =
                core::Reading::new(core::Framing::Document, Vec::new()).map_err(Error::refused)?;
            reading
                .annotation_record(text.as_bytes())
                .map_err(Error::refused)?
        }
        QuestionInput::Record(record) => record.original().0.as_ref().clone(),
        QuestionInput::Images(_) => return Err(super::wrong()),
    };
    let context = crate::public::RecordContext::resolved(record.context.as_ref(), fallback)?;
    let snapshot = Arc::new(question_input.clone());
    Ok((
        Held {
            original: record.original,
            input,
            question_input: snapshot,
            context: context.clone(),
        },
        Prepared {
            explicit_context: record.context.is_some(),
            text: Text {
                at,
                input: question_input,
            },
            context,
        },
    ))
}

pub(super) fn preview_asks(
    engine: &Arc<facade::Engine>,
    set: &QuestionSet,
    record: RecordInput<crate::QuestionInput>,
    fallback: Option<&str>,
    at: usize,
) -> Result<(Vec<crate::core::pack::Ask>, bool), Error> {
    let (_, input) = prepare_record(&set.0, record, fallback, at)?;
    let dropped = set.0.questions().iter().any(|q| {
        crate::core::adapters::built_in::drops_detail_of(
            engine.backend().descriptions(),
            q.question(),
        )
    });
    let asks = Annotations {
        engine: Arc::clone(engine),
        set: set.0.clone(),
        recover_missing: false,
        cli_groups: false,
    }
    .asks(&input)?;
    Ok((asks, dropped))
}

pub(in crate::public) fn preview_grouped_asks(
    engine: &Arc<facade::Engine>,
    set: &QuestionSet,
    record: RecordInput<crate::QuestionInput>,
    fallback: Option<&str>,
    at: usize,
) -> Result<Vec<(crate::core::pack::Ask, usize)>, Error> {
    let (_, input) = prepare_record(&set.0, record, fallback, at)?;
    let mut groups = Vec::new();
    let asks = Annotations {
        engine: Arc::clone(engine),
        set: set.0.clone(),
        recover_missing: false,
        cli_groups: false,
    }
    .asks_with_groups(&input, |group, count| {
        groups.extend(std::iter::repeat_n(group, count))
    })?;
    Ok(asks.into_iter().zip(groups).collect())
}
