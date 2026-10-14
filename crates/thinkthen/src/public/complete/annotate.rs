//! Native original occurrences annotate on the existing ordered group pipeline.
use crate::core::{self, pack::Ask};
use crate::engine::{
    facade,
    pipeline::{self, Asker},
};
use crate::public::{
    Call, CallOptions, CompleteAnnotated, CompleteRecord, Engine, Error, InputEvidence,
    InputFunction, QuestionInput, QuestionSet, RecordInput,
};
use crate::public::{asking::Text, bulk::annotation::Annotating, options::Stop, pull};
use std::sync::Arc;
mod recovering;
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
    require_usable_groups: bool,
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
        !self.recover_missing || error.missed_pointer().is_none()
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
            .require_usable_groups(self.require_usable_groups)
            .row(input.text, answers)
    }
}
impl Engine {
    #[allow(
        clippy::type_complexity,
        reason = "the private Request primitive retains original and complete result types"
    )]
    pub(crate) fn execute_annotate_records_complete_with<I, T>(
        &self,
        questions: &QuestionSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        let mut options = options.started()?;
        options.eager_inputs = true;
        let context = options.context_text().map(str::to_owned);
        self.annotate_stream(
            questions.clone(),
            records.into_iter().map(Ok),
            options,
            context,
            None,
        )?
        .into_call()
    }
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

fn complete_row<T>(
    engine: &facade::Engine,
    set: &core::QuestionSet,
    stop: &Stop<'_>,
    held: &Held<T>,
    (at, legacy): (usize, bool),
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
    let all_failed =
        annotation.failed_questions == annotation.details.len() && !annotation.details.is_empty();
    if all_failed && legacy {
        crate::public::bulk::observe_annotated_questions(&observed, at, stop)?;
    } else {
        crate::public::bulk::observe_annotated(&observed, at, stop)?;
    }
    if all_failed {
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
        require_usable_groups: true,
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
        require_usable_groups: true,
    }
    .asks_with_groups(&input, |group, count| {
        groups.extend(std::iter::repeat_n(group, count))
    })?;
    Ok(asks.into_iter().zip(groups).collect())
}
