//! Eager original occurrences project the shared prepared record stream.
use crate::core::{self, Value, pack::Ask};
use crate::engine::{
    facade,
    pipeline::{self, Answered, Asker},
};
use crate::public::{
    Call, CallOptions, CompleteFilter, CompleteRecord, Engine, Error, InputEvidence, InputFunction,
    Question, RecordInput,
};
use crate::public::{
    asking::{Decided, Decisions, Miss, Text},
    pull,
};
use std::sync::Arc;

#[derive(Clone)]
pub(super) struct Prepared {
    text: Text,
    question: Question,
    context: Option<core::Evidence>,
}
pub(super) struct Records(pub(super) Arc<facade::Engine>, pub(super) bool);
impl Asker for Records {
    type Input = Prepared;
    type Row = Decided;
    type Error = Miss;
    fn validates_batches(&self) -> bool {
        self.1
    }
    fn label(&self, input: &Prepared) -> usize {
        input.text.at
    }
    fn asks(&self, input: &Prepared) -> Result<Vec<Ask>, Miss> {
        let asks =
            Decisions::new(&self.0, &input.question, input.context.clone()).asks(&input.text)?;
        if input.context.is_some() {
            validate_context(&self.0, &asks).map_err(Miss::Refused)?;
        }
        Ok(asks)
    }
    fn row(&self, input: Prepared, answers: Vec<Answered>) -> Result<Decided, Miss> {
        Decisions::new(&self.0, &input.question, input.context).row(input.text, answers)
    }
}
pub(super) struct Held<T> {
    pub(super) original: T,
    pub(super) question: Question,
    pub(super) context_sha256: Option<String>,
    pub(super) source: Option<core::CompletePhysicalSource>,
    pub(super) images: Option<Vec<core::image::Image>>,
}

impl Engine {
    #[allow(
        clippy::type_complexity,
        reason = "The private pipeline retains typed original occurrences"
    )]
    pub(in crate::public) fn records_complete<I, T>(
        &self,
        function: InputFunction,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, core::CompleteAtomic>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        let mut options = options.started()?;
        options.eager_inputs = true;
        options.pending_inputs = true;
        let context = options.context_text().map(str::to_owned);
        self.complete_stream(
            function,
            question.clone(),
            records.into_iter().map(Ok),
            options,
            context,
            Ok,
        )?
        .into_call()
    }
}

pub(super) fn pipeline_failure(miss: Miss) -> Error {
    match miss {
        Miss::Refused(error) => error,
        Miss::Failed(_) => Error::defect("admission produced an answer"),
    }
}
pub(in crate::public) fn mapped<T, R>(
    rows: Vec<CompleteRecord<T, core::CompleteAtomic>>,
    mut map: impl FnMut(core::CompleteAtomic) -> Result<R, Error>,
) -> Result<Vec<CompleteRecord<T, R>>, Error> {
    rows.into_iter()
        .map(|row| {
            Ok(CompleteRecord {
                original: row.original,
                ordinal: row.ordinal,
                result: map(row.result)?,
            })
        })
        .collect()
}

pub(crate) fn filter(canonical: core::CompleteAtomic) -> Result<CompleteFilter, Error> {
    let Value::YesNo(Some(value)) = *canonical.value() else {
        return Err(super::wrong());
    };
    Ok(CompleteFilter { canonical, value })
}

pub(super) fn prepare_record<T: InputEvidence>(
    function: InputFunction,
    question: &Question,
    record: RecordInput<T>,
    fallback: Option<&str>,
    at: usize,
) -> Result<(Held<T>, Prepared), Error> {
    record
        .admit_recognition_controls(
            crate::public::request::RequestFunction::from_input(function),
            "record examples are admitted only for recognize",
            "record seed spans are admitted only for recognize",
        )
        .map_err(|error| error.at_record(at))?;
    let question = record.options.as_ref().map_or_else(
        || Ok(question.clone()),
        |options| options.replacing(question),
    )?;
    question
        .metadata
        .validate_context(record.context.as_ref())
        .map_err(|error| error.at_record(at))?;
    let context = crate::public::RecordContext::resolved(record.context.as_ref(), fallback)?;
    let context_sha256 = context
        .as_ref()
        .map(crate::public::record_context::digest)
        .transpose()?;
    let input = record.original.question_input();
    question
        .admit_input(&input)
        .map_err(|error| error.at_record(at))?;
    crate::public::images::guard(function, &input)?;
    if let crate::public::QuestionInput::Text(text) = &input {
        crate::public::engine::evidence(text)?;
    }
    Ok((
        Held {
            source: super::physical_source(&input),
            images: super::ancillary_images(&input),
            original: record.original,
            question: question.clone(),
            context_sha256,
        },
        Prepared {
            text: Text { at, input },
            question,
            context,
        },
    ))
}

pub(super) fn validate_context(engine: &facade::Engine, asks: &[Ask]) -> Result<(), Error> {
    let limits = engine.pack_limits(pull::packing(core::Setting::Max, true, false));
    let model =
        core::pack::model_json(engine.backend().model().as_str()).map_err(|_| super::wrong())?;
    let mut packer = core::pack::Packer::new(limits, model);
    let mut closed = Vec::new();
    for ask in asks {
        packer
            .check_state(&ask.state)
            .map_err(|error| context_failure(engine, error))?;
    }
    let entries = asks
        .iter()
        .map(|ask| core::pack::Entry {
            state: ask.state.clone(),
            question: Arc::clone(&ask.question),
            options: pipeline::options(ask),
            item: (),
        })
        .collect();
    packer
        .add(entries, &mut closed)
        .map_err(|error| context_failure(engine, error))?;
    Ok(())
}

/// Retain the actual native context refusal for the CLI formatter.
pub(in crate::public) fn context_failure(
    engine: &facade::Engine,
    error: core::pack::PackError,
) -> Error {
    let mut error = crate::public::asking::packed(error);
    match error.take_diagnostic() {
        Some(super::super::error::diagnostic::Diagnostic::Context {
            initial,
            kind,
            limit,
            actual,
            ..
        }) => {
            let profile = engine
                .profile()
                .filter(|held| match kind {
                    core::LimitKind::EvidenceBytes => held.max_evidence_bytes == Some(limit),
                    core::LimitKind::RequestBytes => held.max_request_bytes == Some(limit),
                    core::LimitKind::Questions => held.max_questions == Some(limit),
                    core::LimitKind::Options => false,
                })
                .map(|held| held.name().clone());
            error = error.with_diagnostic(super::super::error::diagnostic::Diagnostic::Context {
                initial,
                kind,
                limit,
                actual,
                profile,
            });
        }
        Some(diagnostic) => error = error.with_diagnostic(diagnostic),
        None => {}
    }
    error
}
pub(in crate::public) fn shared_context(
    engine: &facade::Engine,
    context: &str,
) -> Result<(), Error> {
    let evidence = crate::public::engine::evidence(context)?;
    let state = core::pack::state(&evidence)
        .map_err(|_| super::wrong())?
        .with_api(engine.backend().api_type());
    let model =
        core::pack::model_json(engine.backend().model().as_str()).map_err(|_| super::wrong())?;
    let packer = core::pack::Packer::<()>::new(
        engine.pack_limits(pull::packing(core::Setting::Max, true, false)),
        model,
    );
    packer
        .check_state(&state)
        .map_err(|error| context_failure(engine, error))
}

/// One renderer owns canonical row fields for ordinary and recovering calls.
pub(super) fn complete_row<T>(
    (engine, profile, setting, scalar, function): (
        &facade::Engine,
        Option<&core::BackendProfile>,
        core::Setting,
        bool,
        InputFunction,
    ),
    stop: &crate::public::options::Stop<'_>,
    held: &Held<T>,
    at: usize,
    row: pull::Row<Records>,
) -> Result<core::CompleteAtomic, Error> {
    let question = &held.question;
    let keys = row
        .as_ref()
        .map_or_else(|_| Vec::new(), |row| row.keys.clone());
    let judged = crate::public::bulk::judged(stop, question, engine.backend(), at, row)
        .map_err(|error| error.at_record(at))?;
    let mut run = if scalar {
        super::run(engine, question, profile)
    } else {
        super::batch_run(engine, question, profile, setting)
    };
    run.context_sha256 = held.context_sha256.clone();
    let events = stop
        .facts()
        .attempts()
        .is_some()
        .then(|| judged.answered.attempts.clone());
    let mut canonical = super::atomic(
        run,
        &judged,
        super::spec(function, question, judged.value.clone(), at),
        keys,
        None,
        events,
    )
    .map_err(|_| super::wrong())?;
    canonical.source = held.source.clone();
    canonical.images = held.images.clone();
    Ok(canonical)
}

pub(super) fn recovery_cause(row: &pull::Row<Records>) -> Option<crate::FailureCause> {
    let Err(pipeline::Failed::Asker(Miss::Failed(decided))) = row else { return None; };
    let core::AnswerOutcome::Failed(failure) = &decided.outcome else { return None; };
    Some(crate::public::annotated::cause(core::FailedValue::new(failure.clone()).cause()))
}
