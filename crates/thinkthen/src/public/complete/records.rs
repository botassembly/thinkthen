//! Original occurrences and per-record controls use the existing eager pipeline.
use super::{admitted, atomic, spec};
use crate::core::{self, Value, pack::Ask};
use crate::engine::{
    facade,
    pipeline::{self, Answered, Asker, Flow},
};
use crate::public::{
    Call, CallOptions, CompleteChoice, CompleteDecision, CompleteFilter, CompleteRecord,
    CompleteScore, CompleteTags, DecisionQuestion, DetailQuestion, Engine, Error, InputEvidence,
    InputFunction, Question, RecordInput,
};
use crate::public::{
    asking::{Decided, Decisions, Miss, Text},
    options::Stop,
    pull,
};
use std::sync::Arc;

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
struct Admitted<T> {
    held: Vec<Held<T>>,
    inputs: Vec<Prepared>,
}
pub(super) struct Held<T> {
    pub(super) original: T,
    pub(super) question: Question,
    pub(super) context_sha256: Option<String>,
    pub(super) source: Option<core::CompletePhysicalSource>,
    pub(super) images: Option<Vec<core::image::Image>>,
}

impl Engine {
    /// Complete decisions for eagerly admitted originals and per-record controls.
    ///
    /// # Errors
    /// Invalid records refuse before sending; started failures retain final facts.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn decide_records_complete_with<Q, I, T>(
        &self,
        question: &Q,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error>
    where
        Q: DecisionQuestion + ?Sized,
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.records_complete(InputFunction::Decide, question.question(), records, options)?
            .try_map(|rows| mapped(rows, super::decision))
    }

    /// Complete choices with whole ordered replacement shortlists per record.
    ///
    /// # Errors
    /// Invalid options or evidence refuse before sending; otherwise as choose_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn choose_records_complete_with<Q, I, T>(
        &self,
        question: &Q,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.records_complete(InputFunction::Choose, question.question(), records, options)?
            .try_map(|rows| mapped(rows, super::choice))
    }

    /// Complete tags for all originals, retaining rejected label probabilities.
    ///
    /// # Errors
    /// Images or record options refuse before sending; otherwise as tag_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn tag_records_complete_with<Q, I, T>(
        &self,
        question: &Q,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.records_complete(InputFunction::Tag, question.question(), records, options)?
            .try_map(|rows| mapped(rows, super::tags))
    }

    /// Complete scores for all originals with separate per-record contexts.
    ///
    /// # Errors
    /// Invalid inputs refuse before sending; started failures retain final facts.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn score_records_complete_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.records_complete(InputFunction::Score, question, records, options)?
            .try_map(|rows| mapped(rows, super::score))
    }

    /// Complete filter observations for every original, including rejected records.
    ///
    /// # Errors
    /// Uses the existing filter admission and call boundaries.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn filter_records_complete_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.records_complete(InputFunction::Filter, question, records, options)?
            .try_map(|rows| mapped(rows, filter))
    }

    #[allow(
        clippy::type_complexity,
        reason = "The private pipeline retains typed original occurrences"
    )]
    pub(super) fn records_complete<I, T>(
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
        let options = options.started()?;
        options.admission()?;
        admitted(function, question)?;
        let setting = crate::public::bulk::selected_batch(question, &options, self.batch)?;
        let engine = self.asking(question)?;
        let Admitted { held, inputs } = prepare(
            function,
            question,
            self.within_admission(records, &options)?,
            &options,
        )?;
        // Plan every admitted row before the invocation starts; no later invalid
        // shortlist/context/image can cause an eager partial send.
        let asker = Records(
            Arc::clone(&engine),
            question.metadata.item_schema.is_some() || question.metadata.context_schema.is_some(),
        );
        for input in &inputs {
            asker.asks(input).map_err(pipeline_failure)?;
        }
        let mut packing = pull::packing(setting, false, false);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let requested_attempts = stop.facts().attempts().is_some();
        packing.detailed = requested_attempts;
        stop.run_call(0, |cancel| {
            let mut rows = Vec::new();
            let mut failure = None;
            let host = pipeline::eager(inputs, |row| {
                let flow = take_row(&stop, engine.backend(), &held, row, &mut rows, &mut failure);
                cancel.finished_records(usize::from(flow == Flow::Continue));
                flow
            });
            engine
                .ask_all(&asker, packing, host, cancel)
                .map_err(Error::from)?;
            if let Some(error) = failure {
                return Err(error);
            }
            if rows.len() != held.len() {
                return Err(Error::defect("an eager call lost an original"));
            }
            Ok(rows)
        })?
        .try_map(|rows| {
            rows.into_iter()
                .zip(held)
                .enumerate()
                .map(|(at, ((judged, keys), item))| {
                    let mut run =
                        super::batch_run(&engine, &item.question, self.profile.as_ref(), setting);
                    run.context_sha256 = item.context_sha256;
                    let attempts = requested_attempts.then(|| judged.answered.attempts.clone());
                    let mut result = atomic(
                        run,
                        &judged,
                        spec(function, &item.question, judged.value.clone(), at),
                        keys,
                        None,
                        attempts,
                    )
                    .map_err(|_| Error::defect("a complete record could not be constructed"))?;
                    result.source = item.source;
                    result.images = item.images;
                    Ok(CompleteRecord {
                        original: item.original,
                        ordinal: at,
                        result,
                    })
                })
                .collect()
        })
    }
}

pub(super) fn pipeline_failure(miss: Miss) -> Error {
    match miss {
        Miss::Refused(error) => error,
        Miss::Failed(_) => Error::defect("admission produced an answer"),
    }
}
fn mapped<T, R>(
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

fn prepare<T: InputEvidence>(
    function: InputFunction,
    question: &Question,
    records: impl Iterator<Item = RecordInput<T>>,
    options: &CallOptions<'_>,
) -> Result<Admitted<T>, Error> {
    let mut held = Vec::new();
    let mut inputs = Vec::new();
    for (at, record) in records.enumerate() {
        options.admission()?;
        let (item, input) = prepare_record(function, question, record, options.context_text(), at)?;
        options.admission()?;
        inputs.push(input);
        held.push(item);
    }
    Ok(Admitted { held, inputs })
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

impl Prepared {
    pub(super) fn duplicate(&self) -> Self {
        Self {
            text: Text {
                at: self.text.at,
                input: self.text.input.clone(),
            },
            question: self.question.clone(),
            context: self.context.clone(),
        }
    }
}

fn take_row<T>(
    stop: &Stop<'_>,
    backend: &core::Backend,
    held: &[Held<T>],
    row: Result<Decided, pipeline::Failed<Miss>>,
    rows: &mut Vec<crate::public::engine::Keyed>,
    failure: &mut Option<Error>,
) -> Flow {
    let at = rows.len();
    let Some(item) = held.get(at) else {
        *failure = Some(Error::defect("an eager row has no original"));
        return Flow::Stop;
    };
    let keys = row
        .as_ref()
        .map_or_else(|_| Vec::new(), |row| row.keys.clone());
    match crate::public::bulk::judged(stop, &item.question, backend, at, row) {
        Ok(judged) => {
            rows.push((judged, keys));
            Flow::Continue
        }
        Err(error) => {
            *failure = Some(error.at_record(at));
            Flow::Stop
        }
    }
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
pub(in crate::public) fn cli_context(engine: &facade::Engine, context: &str) -> Result<(), Error> {
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
