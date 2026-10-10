//! Typed native request execution delegates every judgment to the existing engine.
use super::{
    AdmittedRequest, RequestDefinition, RequestFunction as Function, RequestInput, RequestItem,
    RequestOptions, RequestOutcome, RequestValue,
};
use crate::{Batch, Call, CallOptions, Engine, Error, LoadedQuestion, Question};

/// One runtime feed, owned by the caller and consumed with bounded native scheduling.
pub struct RequestFeed<'a> {
    pub(super) name: String,
    pub(super) contents: FeedContents<'a>,
    pub(super) eager: bool,
    pub(super) image_inputs: bool,
    all_filter_results: bool,
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
    #[cfg(feature = "cli")]
    pub(crate) fn execute_cli_request<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
        sink: &dyn Fn(RequestValue),
        release: &dyn Fn(usize),
        recover: crate::public::options::AnnotationRecovery<'a>,
    ) -> Result<RequestOutcome, Error> {
        self.execute_request_sink(request, environment, Some(sink), Some(release), recover)
    }
    pub(super) fn execute_request_sink<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
        sink: Option<&dyn Fn(RequestValue)>,
        release: Option<&dyn Fn(usize)>,
        recover: crate::public::options::AnnotationRecovery<'a>,
    ) -> Result<RequestOutcome, Error> {
        let all_filter_results = feed_projection(request, environment.feed.as_ref())?;
        let options = &request.request.call.arguments().options;
        let prepared = prepare(
            self,
            request,
            environment.controls,
            environment
                .feed
                .as_ref()
                .is_some_and(|feed| feed.image_inputs),
        )?;
        let Prepared {
            reading: reading_definition,
            definition,
            engine,
            mut controls,
            context,
            image_refusal,
        } = prepared;
        if let Some(context) = &context {
            controls = controls.context(context);
        }
        let eager = environment.feed.as_ref().is_some_and(|feed| feed.eager)
            || !matches!(
                request.request.call.arguments().input,
                RequestInput::Feed { .. } | RequestInput::Source { .. }
            );
        let rows = request.records(&reading_definition, environment, controls, image_refusal)?;
        let rows = rows.enumerate().map(|(at, row)| {
            let row = row.map_err(|error| error.at_record(at))?;
            super::inline::validate_composed(&reading_definition, options, &row)
                .map_err(|error| error.at_record(at))?;
            Ok(row)
        });
        let rows: super::composition::Inputs<'_> = if eager {
            Box::new(rows.collect::<Result<Vec<_>, _>>()?.into_iter().map(Ok))
        } else {
            Box::new(rows)
        };
        controls.eager_inputs = eager && sink.is_some();
        let outcome = dispatch(
            &engine,
            request.request.call.function(),
            &definition,
            rows,
            controls,
            eager && (sink.is_none() || request.request.call.function() == Function::Recognize),
            options,
            sink,
            release,
            recover,
        )?;
        Ok(if all_filter_results {
            outcome
        } else {
            select_filter(outcome, options.files_only)
        })
    }
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
    if controls.cli_reader.is_some()
        && let Some(context) = context.as_deref()
    {
        crate::public::complete::records::cli_context(&engine.inner, context)?;
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

pub(super) fn feed_projection(
    request: &AdmittedRequest,
    feed: Option<&RequestFeed<'_>>,
) -> Result<bool, Error> {
    let args = request.request.call.arguments();
    if let RequestInput::Feed { name, .. } = &args.input
        && feed.is_none_or(|feed| &feed.name != name)
    {
        return Err(Error::usage(
            "the request requires its named caller-supplied feed",
        ));
    }
    let all = feed.is_some_and(|feed| feed.all_filter_results);
    if all
        && (request.request.call.function() != Function::Filter
            || !matches!(args.input, RequestInput::Feed { .. })
            || feed.is_none_or(|feed| !matches!(feed.contents, FeedContents::Records(_))))
    {
        return Err(Error::usage(
            "all filter results require a native composed filter feed",
        ));
    }
    Ok(all)
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

fn complete(call: Call<RequestValue>) -> RequestOutcome {
    RequestOutcome::Complete(call)
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
        ($eager:ident,$q:expr,$value:expr) => {
            Ok(complete(
                engine
                    .$eager($q, rows.collect::<Result<Vec<_>, _>>()?, controls)?
                    .map($value),
            ))
        };
    }

    match function {
        Function::Decide => run!(decide_records_complete_with, q, RequestValue::Decisions),
        Function::Choose => run!(choose_records_complete_with, q, RequestValue::Choices),
        Function::Tag => run!(tag_records_complete_with, q, RequestValue::Tags),
        Function::Score => run!(score_records_complete_with, plain(q)?, RequestValue::Scores),
        Function::Filter => run!(
            filter_records_complete_with,
            plain(q)?,
            RequestValue::Filtered
        ),
        Function::Find => Ok(complete(
            engine
                .try_find_records_complete_with(plain(q)?, rows, controls)?
                .map(RequestValue::Found),
        )),
        _ => Err(Error::defect("atomic question entered another function")),
    }
}
fn plain(q: &LoadedQuestion) -> Result<&Question, Error> {
    match q {
        LoadedQuestion::Question(q) => Ok(q),
        _ => Err(Error::usage("this function takes no band")),
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
    clippy::too_many_lines,
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
    if !eager
        && matches!(
            function,
            Function::Decide
                | Function::Choose
                | Function::Tag
                | Function::Score
                | Function::Filter
                | Function::Annotate
        )
    {
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
            super::pull::Rows::Decisions(batch) => stream(batch, RequestValue::Decisions, sink),
            super::pull::Rows::Choices(batch) => stream(batch, RequestValue::Choices, sink),
            super::pull::Rows::Tags(batch) => stream(batch, RequestValue::Tags, sink),
            super::pull::Rows::Scores(batch) => stream(batch, RequestValue::Scores, sink),
            super::pull::Rows::Filtered(batch) => stream(batch, RequestValue::Filtered, sink),
            super::pull::Rows::Annotations(batch) => stream(batch, RequestValue::Annotations, sink),
        });
    }
    if controls.cli_reader.is_some() {
        let recognition = match definition {
            RequestDefinition::Recognize(file) => Some(file.question()),
            RequestDefinition::Recognition(ask) => Some(ask),
            _ => None,
        };
        if let Some(ask) = recognition {
            return Ok(complete(
                engine
                    .request_recognize_stream(ask, rows, controls, eager, sink)?
                    .map(RequestValue::Recognized),
            ));
        }
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
                .try_find_records_complete_with(file.question(), rows, controls)?
                .map(RequestValue::Found),
        )),
        RequestDefinition::Annotate(set) => Ok(complete(
            engine
                .annotate_records_complete_with(
                    set,
                    rows.collect::<Result<Vec<_>, _>>()?,
                    controls,
                )?
                .map(RequestValue::Annotations),
        )),
        RequestDefinition::Recognize(file) => Ok(complete(
            engine
                .try_recognize_records_complete_with(file.question(), rows, controls)?
                .map(RequestValue::Recognized),
        )),
        RequestDefinition::Recognition(ask) => Ok(complete(
            engine
                .try_recognize_records_complete_with(ask, rows, controls)?
                .map(RequestValue::Recognized),
        )),
        RequestDefinition::Relate(ask) => Ok(complete(
            engine
                .try_relate_records_complete_with(ask, rows, controls)?
                .map(RequestValue::Related),
        )),
        RequestDefinition::DynamicChoose(q) => Ok(complete(
            engine
                .choose_dynamic_records_complete_with(
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
