//! Public wrappers construct native Requests; canonical and native bridges enter
//! execute -> prepare -> Validated -> private scheduling -> compatibility projection.
//! Private dispatch never calls public per-function wrappers. SQL row recovery and
//! typed entities retain distinct private modes under this same execution owner.
use super::RequestOutcome::Complete as complete;
pub(super) use super::native_feed::projection as feed_projection;
use super::{
    AdmittedRequest, RequestDefinition, RequestFunction as Function, RequestInput, RequestItem,
    RequestOptions, RequestOutcome, RequestValue,
};
use crate::public::engine::Sealed as _;
use crate::{Batch, Call, CallOptions, Engine, Error, LoadedQuestion, Question};

/// One runtime feed, owned by the caller and consumed with bounded native scheduling.
pub struct RequestFeed<'a> {
    pub(super) name: String,
    pub(super) contents: FeedContents<'a>,
    pub(super) eager: bool,
    pub(super) image_inputs: bool,
    pub(super) all_filter_results: bool,
    pub(super) record_reading: Option<crate::RecordReading>,
}
pub(super) enum FeedContents<'a> {
    Items(Box<dyn Iterator<Item = Result<RequestItem, Error>> + 'a>),
    Records(super::composition::Inputs<'a>),
    Session(std::sync::Arc<super::session_queue::Queue>),
    #[cfg(feature = "cli")]
    Descriptors(Vec<super::transport::TransportDescriptor>),
}
impl std::fmt::Debug for RequestFeed<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestFeed(<withheld>)")
    }
}
impl<'a> RequestFeed<'a> {
    pub(super) fn session(
        name: String,
        queue: std::sync::Arc<super::session_queue::Queue>,
    ) -> Self {
        Self {
            name,
            contents: FeedContents::Session(queue),
            eager: false,
            image_inputs: false,
            all_filter_results: false,
            record_reading: None,
        }
    }
    #[cfg(feature = "cli")]
    pub(super) fn descriptors(
        name: String,
        descriptors: Vec<super::transport::TransportDescriptor>,
    ) -> Self {
        let image_inputs = descriptors.iter().any(|d| !d.item.images.is_empty());
        Self {
            name,
            contents: FeedContents::Descriptors(descriptors),
            eager: true,
            image_inputs,
            all_filter_results: false,
            record_reading: None,
        }
    }
    /// Supply native item descriptors without collecting or serializing the feed.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        items: impl Iterator<Item = Result<RequestItem, Error>> + 'a,
    ) -> Self {
        Self {
            name: name.into(),
            contents: FeedContents::Items(Box::new(items)),
            eager: false,
            image_inputs: false,
            all_filter_results: false,
            record_reading: None,
        }
    }
    /// Supply already composed native records, retaining original locations and images.
    /// This feed owns framing and projection; additional request framing, projections
    /// or shared attachments refuse before the iterator advances. Runtime admission,
    /// declaration validation, image routes and engine limits still apply to each row.
    #[must_use]
    pub fn from_records(
        name: impl Into<String>,
        records: impl Iterator<Item = Result<crate::RecordInput<crate::QuestionInput>, Error>> + 'a,
    ) -> Self {
        Self {
            name: name.into(),
            contents: FeedContents::Records(Box::new(records)),
            eager: false,
            image_inputs: false,
            all_filter_results: false,
            record_reading: None,
        }
    }
    /// Admit the entire supplied feed before any sends, using native eager execution.
    /// Without this control, joined failures retain the actual completed prefix.
    #[must_use]
    pub fn eager(mut self) -> Self {
        self.eager = true;
        self
    }
    /// Declare image-bearing input so route admission precedes the first reader access.
    /// Every actual image row is still validated when it arrives.
    #[must_use]
    pub fn with_image_inputs(mut self) -> Self {
        self.image_inputs = true;
        self
    }
    /// Retain passing and rejected filter occurrences in the complete native result.
    /// Only a composed-record feed executing filter admits this projection;
    /// invalid combinations refuse before advancement. Ordinary Request filtering
    /// remains unchanged, including file-only selection.
    #[must_use]
    pub fn with_all_filter_results(mut self) -> Self {
        self.all_filter_results = true;
        self
    }
}
/// Runtime controls and feed authority never enter the serialized request.
#[derive(Debug, Default)]
pub struct RequestEnvironment<'a> {
    /// Existing native callbacks, cancellation and surface attribution.
    pub controls: CallOptions<'a>,
    /// Optional named bounded feed.
    pub feed: Option<RequestFeed<'a>>,
}
impl Engine {
    /// Execute an admitted request through existing typed complete methods.
    /// # Errors
    /// Selector and input failures retain existing native kinds; streamed failures
    /// return their actual completed prefix in RequestOutcome::Failed.
    pub fn execute_request<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
    ) -> Result<RequestOutcome, Error> {
        self.execute_request_sink(request, environment, None, None, None)
    }
    pub(crate) fn execute_request_sink<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
        sink: Option<&dyn Fn(RequestValue)>,
        release: Option<&dyn Fn(usize)>,
        recover: crate::public::options::AnnotationRecovery<'a>,
    ) -> Result<RequestOutcome, Error> {
        let all_filter_results = feed_projection(request, environment.feed.as_ref())?;
        let options = &request.request.call.arguments().options;
        let eager = environment.feed.as_ref().is_some_and(|feed| feed.eager)
            || !matches!(
                request.request.call.arguments().input,
                RequestInput::Feed { .. } | RequestInput::Source { .. }
            );
        let owned_feed = environment
            .feed
            .as_ref()
            .is_some_and(|feed| matches!(feed.contents, FeedContents::Session(_)));
        let image_inputs = environment
            .feed
            .as_ref()
            .is_some_and(|feed| feed.image_inputs);
        let outcome = execute(
            self,
            request,
            environment.controls,
            image_inputs,
            |prepared| {
                request.records(
                    &prepared.reading,
                    environment,
                    prepared.controls,
                    prepared.image_refusal.clone(),
                )
            },
            |Prepared {
                 definition,
                 engine,
                 mut controls,
                 context,
                 ..
             },
             rows| {
                if let Some(context) = &context {
                    controls = controls.context(context);
                }
                let rows: super::composition::Inputs<'_> = if eager
                    && !owned_feed
                    && request.request.call.function() != Function::Relate
                {
                    Box::new(rows.collect::<Result<Vec<_>, _>>()?.into_iter().map(Ok))
                } else {
                    Box::new(rows)
                };
                controls.eager_inputs = eager && sink.is_some();
                dispatch(
                    &engine,
                    request.request.call.function(),
                    &definition,
                    rows,
                    controls,
                    eager
                        && (sink.is_none()
                            || request.request.call.function() == Function::Recognize),
                    options,
                    sink,
                    release,
                    recover,
                )
            },
        )?;
        Ok(if all_filter_results {
            outcome
        } else {
            select_filter(outcome, options.files_only)
        })
    }
}
/// All entry points construct a Request and enter this owner. It prepares the
/// definition once, then validates each occurrence before private scheduling.
/// Public compatibility projections never participate in private dispatch.
pub(super) fn execute<'a, I, T, R>(
    engine: &Engine,
    request: &AdmittedRequest,
    controls: CallOptions<'a>,
    image_inputs: bool,
    source: impl FnOnce(&Prepared<'a>) -> Result<I, Error>,
    run: impl FnOnce(Prepared<'a>, Validated<'a, I::IntoIter>) -> Result<R, Error>,
) -> Result<R, Error>
where
    I: IntoIterator<Item = Result<crate::RecordInput<T>, Error>>,
    T: crate::InputEvidence,
{
    let prepared = prepare(engine, request, controls, image_inputs)?;
    let records = source(&prepared)?.into_iter().enumerate();
    let rows = Validated {
        records,
        controls: prepared.controls,
        reading: prepared.reading.clone(),
        options: request.request.call.arguments().options.clone(),
        function: request.request.call.function(),
        native_originals: request.native_originals,
        image_refusal: prepared.image_refusal.clone(),
        engine: prepared.engine.clone(),
        find_maximum: if request.request.call.function() == Function::Find {
            Some(crate::core::Find::maximum(
                prepared.definition.clone().atomic()?.kind
                    == crate::public::question::Kind::FindNone,
            ))
        } else {
            None
        },
        find_bytes: crate::public::SourceBudget::find(),
        seen: 0,
    };
    run(prepared, rows)
}

pub(in crate::public) struct Validated<'a, I> {
    records: std::iter::Enumerate<I>,
    controls: CallOptions<'a>,
    reading: RequestDefinition,
    options: RequestOptions,
    function: Function,
    native_originals: bool,
    image_refusal: Option<String>,
    engine: Engine,
    find_maximum: Option<usize>,
    find_bytes: crate::public::SourceBudget,
    seen: usize,
}
impl<I, T> Iterator for Validated<'_, I>
where
    I: Iterator<Item = Result<crate::RecordInput<T>, Error>>,
    T: crate::InputEvidence,
{
    type Item = Result<crate::RecordInput<T>, Error>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.records.size_hint();
        if !self.controls.entity_rows {
            return (lower, upper);
        }
        let remaining = crate::core::RelateSpec::MAX_ENTITIES + 1 - self.seen;
        (
            lower.min(remaining),
            Some(upper.map_or(remaining, |upper| upper.min(remaining))),
        )
    }
    fn next(&mut self) -> Option<Self::Item> {
        if self.controls.entity_rows && self.seen == crate::core::RelateSpec::MAX_ENTITIES + 1 {
            return None;
        }
        let (at, row) = self.records.next()?;
        self.seen += 1;
        Some(
            (|| {
                self.controls.admission()?;
                let row = row?;
                let input = row.original.question_input();
                self.admit_find(at, &input)?;
                super::native_feed::validate(
                    self.function,
                    &row,
                    &input,
                    self.image_refusal.as_deref(),
                    self.native_originals && self.controls.eager_inputs,
                )?;
                // Native preparation preserves its reader-before-declaration and
                // staged-plan refusal contracts. Find projects already admitted rows.
                if !self.controls.recovery_rows
                    && !self.controls.entity_rows
                    && (!self.native_originals || self.function == Function::Find)
                {
                    declarations(&self.reading, &self.options, &row, input)?;
                }
                Ok(row)
            })()
            .map_err(|error: Error| error.at_record(at))
            .and_then(|row| self.engine.check_record_limit(at).map(|()| row)),
        )
    }
}

impl<I> Validated<'_, I> {
    fn admit_find(&mut self, at: usize, input: &crate::QuestionInput) -> Result<(), Error> {
        let Some(maximum) = self.find_maximum else {
            return Ok(());
        };
        if at == maximum {
            return Err(Error::usage(crate::public::bulk::count_message(
                maximum == 254,
            )));
        }
        let bytes = match input {
            crate::QuestionInput::Text(text) => text.len(),
            crate::QuestionInput::Record(record) => record.original().retained_bytes()?,
            crate::QuestionInput::Images(_) => 0,
        };
        self.find_bytes.charge(bytes)
    }
}

fn declarations<T>(
    reading: &RequestDefinition,
    options: &RequestOptions,
    row: &crate::RecordInput<T>,
    input: crate::QuestionInput,
) -> Result<(), Error> {
    super::inline::validate_composed(
        reading,
        options,
        &crate::RecordInput {
            original: input,
            context: row.context.clone(),
            options: row.options.clone(),
            seed_spans: row.seed_spans.clone(),
            examples: row.examples.clone(),
        },
    )
}

/// Snapshot custom input evidence exactly once while retaining caller ownership.
pub(in crate::public) struct Original<T> {
    pub(in crate::public) original: T,
    input: crate::QuestionInput,
}
impl<T> crate::InputEvidence for Original<T> {
    fn question_input(&self) -> crate::QuestionInput {
        self.input.clone()
    }
}
pub(super) fn snapshot<T: crate::InputEvidence>(
    row: crate::RecordInput<T>,
) -> crate::RecordInput<Original<T>> {
    let input = row.original.question_input();
    row.map_original(|original| Original { original, input })
}
#[allow(
    clippy::type_complexity,
    reason = "restoration retains caller originals and complete judgments"
)]
pub(in crate::public) fn restore<T, R>(
    call: Call<Vec<crate::CompleteRecord<Original<T>, R>>>,
) -> Call<Vec<crate::CompleteRecord<T, R>>> {
    call.map(|rows| {
        rows.into_iter()
            .map(|row| row.map_original(|original| original.original))
            .collect()
    })
}
pub(in crate::public) fn restore_found<T>(
    call: Call<crate::CompleteFound<Original<T>>>,
) -> Call<crate::CompleteFound<T>> {
    call.map(|result| result.map_original(|original| original.original))
}

/// Native eager wrappers use the same Request owner and private primitives as
/// collecting wire dispatch. Originals never enter a serialized representation.
#[allow(
    clippy::type_complexity,
    reason = "the shared owner binds typed originals and private execution"
)]
pub(in crate::public) fn native<I, T, R>(
    engine: &Engine,
    function: Function,
    definition: RequestDefinition,
    records: I,
    controls: CallOptions<'_>,
    run: impl FnOnce(
        &Engine,
        RequestDefinition,
        Vec<crate::RecordInput<Original<T>>>,
        CallOptions<'_>,
    ) -> Result<R, Error>,
) -> Result<R, Error>
where
    I: IntoIterator<Item = Result<crate::RecordInput<T>, Error>>,
    T: crate::InputEvidence,
{
    native_intake(
        engine,
        function,
        definition,
        records,
        controls,
        |engine, definition, rows, controls| {
            let rows = engine.try_within_admission(rows, &controls)?.collect();
            run(engine, definition, rows, controls)
        },
    )
}

type NativeRows<I, T> = std::iter::Map<
    I,
    fn(Result<crate::RecordInput<T>, Error>) -> Result<crate::RecordInput<Original<T>>, Error>,
>;

#[allow(
    clippy::type_complexity,
    reason = "typed intake shares one Request execution owner"
)]
pub(in crate::public) fn native_intake<'a, I, T, R>(
    engine: &Engine,
    function: Function,
    definition: RequestDefinition,
    records: I,
    mut controls: CallOptions<'a>,
    run: impl FnOnce(
        &Engine,
        RequestDefinition,
        Validated<'a, NativeRows<I::IntoIter, T>>,
        CallOptions<'_>,
    ) -> Result<R, Error>,
) -> Result<R, Error>
where
    I: IntoIterator<Item = Result<crate::RecordInput<T>, Error>>,
    T: crate::InputEvidence,
{
    let request = super::admit_native(function, definition)?;
    controls.eager_inputs = true;
    execute(
        engine,
        &request,
        controls,
        false,
        |_| Ok(records.into_iter().map(snapshot_row::<T> as fn(_) -> _)),
        |Prepared {
             engine,
             definition,
             mut controls,
             context,
             ..
         },
         rows| {
            if let Some(context) = &context {
                controls = controls.context(context);
            }
            run(&engine, definition, rows, controls)
        },
    )
}

fn snapshot_row<T: crate::InputEvidence>(
    row: Result<crate::RecordInput<T>, Error>,
) -> Result<crate::RecordInput<Original<T>>, Error> {
    row.map(snapshot)
}

pub(super) struct Prepared<'a> {
    pub reading: RequestDefinition,
    pub definition: RequestDefinition,
    pub engine: Engine,
    pub controls: CallOptions<'a>,
    pub context: Option<String>,
    pub image_refusal: Option<String>,
}

pub(super) fn prepare<'a>(
    base: &Engine,
    request: &AdmittedRequest,
    controls: CallOptions<'a>,
    image_inputs: bool,
) -> Result<Prepared<'a>, Error> {
    let options = &request.request.call.arguments().options;
    let context = options
        .context
        .clone()
        .or_else(|| controls.context_text().map(str::to_owned));
    let controls = scalar_controls(options, controls)?.started()?;
    controls.admission()?;
    let mut definition = request.resolve_question()?;
    request.admit_inline(&definition)?;
    let reading_definition = definition.clone();
    apply(&mut definition, options)?;
    controls.admission()?;
    if image_inputs && !request.request.call.function().images() {
        return Err(Error::usage(format!(
            "{} accepts text only; images are unsupported",
            request.request.call.function().name()
        )));
    }
    let mut engine = base.clone();
    if let Some(model) = &options.model {
        engine.inner = base.for_model(Some(
            &crate::core::ModelName::new(model).map_err(Error::refused)?,
        ))?;
    }
    if !controls.recovery_rows
        && !controls.entity_rows
        && let Some(context) = context.as_deref().filter(|text| !text.is_empty())
    {
        crate::public::complete::records::shared_context(&engine.inner, context)?;
    }
    let image_refusal = image_route(&engine, &definition)
        .err()
        .map(|error| error.detail().message().to_owned());
    if request.attachment_limit.is_none()
        && (image_descriptors(&request.request.call.arguments().input) || image_inputs)
        && let Some(message) = &image_refusal
    {
        return Err(Error::usage(message.clone()));
    }
    Ok(Prepared {
        reading: reading_definition,
        definition,
        engine,
        controls,
        context,
        image_refusal,
    })
}

pub(super) fn controls<'a>(
    options: &'a RequestOptions,
    controls: CallOptions<'a>,
) -> Result<CallOptions<'a>, Error> {
    let mut controls = scalar_controls(options, controls)?;
    if let Some(context) = &options.context {
        controls = controls.context(context);
    }
    Ok(controls)
}
fn scalar_controls<'a>(
    options: &RequestOptions,
    mut controls: CallOptions<'a>,
) -> Result<CallOptions<'a>, Error> {
    if options.attempts {
        controls = controls.attempts(true);
    }
    if options.max_requests_total.is_some() {
        controls = controls.max_requests_total(options.max_requests_total);
    }
    if let Some(ms) = options.deadline_ms {
        controls = controls.deadline_ms(ms)?;
    }
    if let Some(batch) = &options.batch {
        controls = controls.batch(batch.native()?);
    }
    Ok(controls)
}

fn stream<T>(
    mut batch: Batch<'_, T>,
    value: impl Fn(Vec<T>) -> RequestValue,
    sink: Option<&dyn Fn(RequestValue)>,
) -> RequestOutcome {
    if let Some(sink) = sink {
        for row in batch.by_ref() {
            match row {
                Ok(row) => sink(value(vec![row])),
                Err(error) => {
                    return RequestOutcome::Failed {
                        completed: value(vec![]),
                        error,
                    };
                }
            }
        }
        return match batch.facts().cloned() {
            Some(facts) => RequestOutcome::Complete(Call::new(value(vec![]), facts)),
            None => RequestOutcome::Failed {
                completed: value(vec![]),
                error: Error::defect("a completed batch has no facts"),
            },
        };
    }
    match batch.into_outcome() {
        crate::public::batch::Outcome::Complete(call) => RequestOutcome::Complete(call.map(value)),
        crate::public::batch::Outcome::Failed { completed, error } => RequestOutcome::Failed {
            completed: value(completed),
            error,
        },
    }
}
fn atomic<'a>(
    engine: &'a Engine,
    function: Function,
    q: &'a LoadedQuestion,
    rows: super::composition::Inputs<'a>,
    controls: CallOptions<'a>,
) -> Result<RequestOutcome, Error> {
    macro_rules! run {
        ($input:ident, $convert:path, $value:expr) => {
            Ok(complete(
                engine
                    .records_complete(
                        crate::InputFunction::$input,
                        q.question(),
                        rows.collect::<Result<Vec<_>, _>>()?,
                        controls,
                    )?
                    .try_map(|rows| crate::public::complete::records::mapped(rows, $convert))?
                    .map($value),
            ))
        };
    }
    match function {
        Function::Decide => run!(
            Decide,
            crate::public::complete::decision,
            RequestValue::Decisions
        ),
        Function::Choose => run!(
            Choose,
            crate::public::complete::choice,
            RequestValue::Choices
        ),
        Function::Tag => run!(Tag, crate::public::complete::tags, RequestValue::Tags),
        Function::Score => run!(Score, crate::public::complete::score, RequestValue::Scores),
        Function::Filter => run!(
            Filter,
            crate::public::complete::records::filter,
            RequestValue::Filtered
        ),
        Function::Find => Ok(complete(
            engine
                .execute_find_records_complete_with(q.question(), rows, controls)?
                .map(RequestValue::Found),
        )),
        _ => Err(Error::defect("atomic question entered another function")),
    }
}

pub(super) fn apply(
    definition: &mut RequestDefinition,
    options: &RequestOptions,
) -> Result<(), Error> {
    match definition {
        RequestDefinition::Atomic(q) => match q {
            LoadedQuestion::Question(q) => apply_question(q, options)?,
            LoadedQuestion::Banded(q) => apply_question(&mut q.0, options)?,
        },
        RequestDefinition::Rank(q) => apply_question(q, options)?,
        RequestDefinition::Find(q) => {
            let (mut question, _) = q.clone().into_parts();
            apply_question(&mut question, options)?;
            // Selection remains in the admitted request's reading, before engine preparation.
            if options.none {
                question = question.offering_none()?;
            }
            *definition = RequestDefinition::Atomic(LoadedQuestion::Question(question));
        }
        RequestDefinition::Recognize(q) => {
            let (mut ask, _) = q.clone().into_parts();
            super::admission::apply_recognition(&mut ask.0, options)?;
            if let Some(model) = &options.model {
                ask.0.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
            }
            if let Some(seeds) = &options.seed_spans {
                ask = ask.with_seed_spans(seeds.clone());
            }
            if let Some(examples) = &options.examples {
                ask = ask.with_examples(examples.clone())?;
            }
            *definition = RequestDefinition::Recognition(ask);
        }
        RequestDefinition::Recognition(q) => {
            super::admission::apply_recognition(&mut q.0, options)?;
            if let Some(seeds) = &options.seed_spans {
                *q = q.clone().with_seed_spans(seeds.clone());
            }
            if let Some(examples) = &options.examples {
                *q = q.clone().with_examples(examples.clone())?;
            }
            if let Some(model) = &options.model {
                q.0.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
            }
        }
        RequestDefinition::Relate(q) => {
            if let Some(model) = &options.model {
                q.0.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
            }
        }
        RequestDefinition::DynamicChoose(q) => {
            if options.field.is_some() {
                *q = q.clone().without_authored_on();
            }
            if let Some(model) = &options.model {
                *q = q.clone().with_model_override(model)?;
            }
            if let Some(rule) = &options.threshold {
                *q = q.clone().cut_at(rule.native()?.bounds().0)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn apply_question(q: &mut Question, options: &RequestOptions) -> Result<(), Error> {
    if let Some(model) = &options.model {
        q.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
    }
    if options.field.is_some() {
        q.metadata.reading.on.clear();
    }
    if let Some(rule) = &options.threshold {
        q.override_threshold(rule.native()?);
    }
    if q.kind() == crate::QuestionKind::Find && options.none {
        *q = q.clone().offering_none()?;
    }
    Ok(())
}

fn select_filter(outcome: RequestOutcome, files_only: bool) -> RequestOutcome {
    let select = |value: RequestValue| match value {
        RequestValue::Filtered(rows) => {
            let mut files = std::collections::BTreeSet::new();
            RequestValue::Filtered(
                rows.into_iter()
                    .filter(|row| selected_filter(row, files_only, &mut files))
                    .collect(),
            )
        }
        other => other,
    };
    match outcome {
        RequestOutcome::Complete(call) => RequestOutcome::Complete(call.map(select)),
        RequestOutcome::Failed { completed, error } => RequestOutcome::Failed {
            completed: select(completed),
            error,
        },
    }
}
pub(super) fn selected_filter(
    row: &crate::CompleteRecord<crate::QuestionInput, crate::CompleteFilter>,
    files_only: bool,
    files: &mut std::collections::BTreeSet<String>,
) -> bool {
    row.result().value()
        && (!files_only
            || matches!(row.original(), crate::QuestionInput::Record(record) if record.location().is_some_and(|source| files.insert(source.file().to_owned()))))
}

#[expect(
    clippy::too_many_arguments,
    reason = "one typed dispatch preserves all ten existing native scheduler contracts"
)]
fn dispatch<'a>(
    engine: &'a Engine,
    function: Function,
    definition: &'a RequestDefinition,
    rows: super::composition::Inputs<'a>,
    controls: CallOptions<'a>,
    eager: bool,
    options: &RequestOptions,
    sink: Option<&dyn Fn(RequestValue)>,
    release: Option<&dyn Fn(usize)>,
    recover: crate::public::options::AnnotationRecovery<'a>,
) -> Result<RequestOutcome, Error> {
    if !eager && function.streams() {
        let rows = super::pull::dispatch(
            engine,
            function,
            definition.clone(),
            rows,
            controls,
            controls.context_text().map(str::to_owned),
            recover,
        )?;
        return Ok(match rows {
            super::pull::RequestRows::Decisions(batch) => {
                stream(batch, RequestValue::Decisions, sink)
            }
            super::pull::RequestRows::Choices(batch) => stream(batch, RequestValue::Choices, sink),
            super::pull::RequestRows::Tags(batch) => stream(batch, RequestValue::Tags, sink),
            super::pull::RequestRows::Scores(batch) => stream(batch, RequestValue::Scores, sink),
            super::pull::RequestRows::Filtered(batch) => {
                stream(batch, RequestValue::Filtered, sink)
            }
            super::pull::RequestRows::Annotations(batch) => {
                stream(batch, RequestValue::Annotations, sink)
            }
        });
    }
    let outcome = match definition {
        RequestDefinition::Atomic(q) => atomic(engine, function, q, rows, controls),
        RequestDefinition::Rank(q) => Ok(complete(
            engine
                .request_rank_records_complete_with(q, rows, controls, options.top, release)?
                .map(|mut rows| {
                    if let Some(n) = options.top {
                        rows.truncate(n);
                    }
                    RequestValue::Ranked(rows)
                }),
        )),
        RequestDefinition::RankSet(q) => Ok(complete(
            engine
                .request_rank_set_records_complete_with(q, rows, controls, options.top, release)?
                .map(|mut rows| {
                    if let Some(n) = options.top {
                        rows.truncate(n);
                    }
                    RequestValue::SetRanked(rows)
                }),
        )),
        RequestDefinition::Find(file) => Ok(complete(
            engine
                .execute_find_records_complete_with(file.question(), rows, controls)?
                .map(RequestValue::Found),
        )),
        RequestDefinition::Annotate(set) => Ok(complete(
            engine
                .execute_annotate_records_complete_with(
                    set,
                    rows.collect::<Result<Vec<_>, _>>()?,
                    controls,
                )?
                .map(RequestValue::Annotations),
        )),
        RequestDefinition::Recognize(file) => {
            recognize(engine, file.question(), rows, controls, eager, sink)
        }
        RequestDefinition::Recognition(ask) => recognize(engine, ask, rows, controls, eager, sink),
        RequestDefinition::Relate(ask) => Ok(complete(
            engine
                .execute_relate_records_complete_with(ask, rows, controls)?
                .map(RequestValue::Related),
        )),
        RequestDefinition::DynamicChoose(q) => Ok(complete(
            engine
                .execute_choose_dynamic_records_complete_with(
                    q,
                    rows.collect::<Result<Vec<_>, _>>()?,
                    controls,
                )?
                .map(RequestValue::Choices),
        )),
        RequestDefinition::DecodedSet { .. } => {
            Err(Error::defect("unresolved authored set entered execution"))
        }
    }?;
    Ok(outcome)
}

pub(super) fn image_descriptors(input: &RequestInput) -> bool {
    match input {
        RequestInput::Text { images, .. }
        | RequestInput::Json { images, .. }
        | RequestInput::Feed { images, .. } => !images.is_empty(),
        RequestInput::Records { items }
        | RequestInput::Units { items }
        | RequestInput::Entities { items } => items.iter().any(|item| !item.images.is_empty()),
        RequestInput::Source { source } => source.media == crate::ReaderMedia::Image,
    }
}
pub(super) fn image_route(engine: &Engine, definition: &RequestDefinition) -> Result<(), Error> {
    let configured = match definition {
        RequestDefinition::Atomic(LoadedQuestion::Question(q)) | RequestDefinition::Rank(q) => {
            engine.asking(q)?
        }
        RequestDefinition::Atomic(LoadedQuestion::Banded(q)) => engine.asking(&q.0)?,
        RequestDefinition::DynamicChoose(q) => engine.for_model(q.model.as_ref())?,
        _ => return Err(Error::usage("this function takes text only")),
    };
    let backend = configured.backend();
    backend
        .image_route()
        .admit_header(backend.asked().0.as_str(), configured.profile())
        .map_err(Error::refused)
}

fn recognize<'a>(
    engine: &'a Engine,
    ask: &'a crate::Recognize,
    rows: super::composition::Inputs<'a>,
    controls: CallOptions<'a>,
    eager: bool,
    sink: Option<&dyn Fn(RequestValue)>,
) -> Result<RequestOutcome, Error> {
    let call = if sink.is_some() || !eager {
        engine.request_recognize_stream(ask, rows, controls, eager, sink)?
    } else {
        engine.execute_recognize_records_complete_with(ask, rows, controls)?
    };
    Ok(complete(call.map(RequestValue::Recognized)))
}

/// Scalar results project the shared private record primitive without batch metadata.
pub(crate) fn scalar(
    engine: &Engine,
    function: crate::InputFunction,
    question: &Question,
    input: &crate::QuestionInput,
    mut controls: CallOptions<'_>,
) -> Result<Call<crate::core::CompleteAtomic>, Error> {
    controls.without_context("a single-document call")?;
    controls.scalar_input = true;
    native_atomic(
        engine,
        function,
        question,
        std::iter::once(crate::public::record_input::plain(input.clone())),
        controls,
    )?
    .try_map(|mut rows| {
        rows.pop()
            .map(|row| row.result)
            .ok_or_else(|| Error::defect("a single call returned no row"))
    })
}

#[allow(
    clippy::type_complexity,
    reason = "native originals retain their complete result"
)]
pub(in crate::public) fn native_atomic<I, T>(
    engine: &Engine,
    function: crate::InputFunction,
    question: &Question,
    records: I,
    controls: CallOptions<'_>,
) -> Result<Call<Vec<crate::CompleteRecord<T, crate::core::CompleteAtomic>>>, Error>
where
    I: IntoIterator<Item = crate::RecordInput<T>>,
    T: crate::InputEvidence,
{
    let definition = if function == crate::InputFunction::Rank {
        RequestDefinition::Rank(question.clone())
    } else if question.kind == crate::public::NativeQuestionKind::Banded {
        crate::LoadedQuestion::Banded(crate::BandedQuestion(question.clone())).into()
    } else {
        question.clone().into()
    };
    native(
        engine,
        Function::from_input(function),
        definition,
        records.into_iter().map(Ok),
        controls,
        |engine, definition, records, controls| {
            let q = definition.atomic()?;
            Ok(restore(
                engine.records_complete(function, &q, records, controls)?,
            ))
        },
    )
}
