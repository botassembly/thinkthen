//! Borrowed native originals enter the admitted Request edge without a result reader.
use super::{AdmittedRequest, RequestDefinition, RequestFunction as Function};
use crate::public::complete::{choice, decision, records::filter, score, tags};
use crate::{
    Batch, CallOptions, CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter,
    CompleteRecord, CompleteScore, CompleteTags, Engine, Error, InputEvidence, InputFunction,
    LoadedQuestion, RecordInput,
};

/// Lazy admitted-request rows retain caller originals and native stopping facts.
pub enum RequestRows<'a, T> {
    /// Complete decision occurrences.
    Decisions(Batch<'a, CompleteRecord<T, CompleteDecision>>),
    /// Complete choice occurrences.
    Choices(Batch<'a, CompleteRecord<T, CompleteChoice>>),
    /// Complete tag occurrences.
    Tags(Batch<'a, CompleteRecord<T, CompleteTags>>),
    /// Complete score occurrences.
    Scores(Batch<'a, CompleteRecord<T, CompleteScore>>),
    /// Passing filter occurrences.
    Filtered(Batch<'a, CompleteRecord<T, CompleteFilter>>),
    /// Complete annotation occurrences.
    Annotations(Batch<'a, CompleteRecord<T, CompleteAnnotated>>),
}

pub(crate) fn start<'a, I, T>(
    engine: &Engine,
    request: &AdmittedRequest,
    records: I,
    controls: CallOptions<'a>,
    recover: crate::public::options::AnnotationRecovery<'a>,
) -> Result<RequestRows<'a, T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    super::native_feed::header(request)?;
    let rows = super::execution::intake(
        engine,
        request,
        records,
        controls,
        |engine, definition, records, controls, context| {
            dispatch(
                &engine,
                request.request.call.function(),
                definition,
                records,
                controls,
                context,
                recover,
            )
        },
    )?;
    Ok(match rows {
        RequestRows::Decisions(batch) => RequestRows::Decisions(
            batch.map_rows(|row| row.map_original(|original| original.original)),
        ),
        RequestRows::Choices(batch) => RequestRows::Choices(
            batch.map_rows(|row| row.map_original(|original| original.original)),
        ),
        RequestRows::Tags(batch) => {
            RequestRows::Tags(batch.map_rows(|row| row.map_original(|original| original.original)))
        }
        RequestRows::Scores(batch) => RequestRows::Scores(
            batch.map_rows(|row| row.map_original(|original| original.original)),
        ),
        RequestRows::Filtered(batch) => RequestRows::Filtered(
            batch.map_rows(|row| row.map_original(|original| original.original)),
        ),
        RequestRows::Annotations(batch) => RequestRows::Annotations(
            batch.map_rows(|row| row.map_original(|original| original.original)),
        ),
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "one shared typed dispatch owns preparation and caller controls"
)]
pub(super) fn dispatch<'a, I, T>(
    engine: &Engine,
    function: Function,
    definition: RequestDefinition,
    records: I,
    controls: CallOptions<'a>,
    context: Option<String>,
    recover: crate::public::options::AnnotationRecovery<'a>,
) -> Result<RequestRows<'a, T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    if !function.streams() {
        return Err(Error::usage(
            "this request does not return independent pulled rows",
        ));
    }
    macro_rules! atomic {
        ($kind:ident, $input:ident, $convert:path, $q:expr) => {
            Ok(RequestRows::$kind(engine.complete_stream(
                InputFunction::$input,
                $q,
                records,
                controls,
                context,
                $convert,
            )?))
        };
    }
    match (function, definition) {
        (Function::Decide, RequestDefinition::Atomic(q)) => {
            atomic!(Decisions, Decide, decision, question(q))
        }
        (Function::Choose, RequestDefinition::Atomic(q)) => {
            atomic!(Choices, Choose, choice, question(q))
        }
        (Function::Tag, RequestDefinition::Atomic(q)) => {
            atomic!(Tags, Tag, tags, question(q))
        }
        (Function::Score, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            atomic!(Scores, Score, score, q)
        }
        (Function::Filter, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            atomic!(Filtered, Filter, filter, q)
        }
        (Function::Choose, RequestDefinition::DynamicChoose(q)) => Ok(RequestRows::Choices(
            engine.dynamic_choose_stream(q, records, controls, context)?,
        )),
        (Function::Annotate, RequestDefinition::Annotate(set)) => Ok(RequestRows::Annotations(
            engine.annotate_stream(set, records, controls, context, recover)?,
        )),
        _ => Err(Error::usage(
            "this request does not return independent pulled rows",
        )),
    }
}
fn question(q: LoadedQuestion) -> crate::Question {
    match q {
        LoadedQuestion::Question(q) => q,
        LoadedQuestion::Banded(q) => q.0,
    }
}

#[cfg(any(test, feature = "polars"))]
pub(crate) fn native<'a, I, T>(
    engine: &Engine,
    definition: RequestDefinition,
    records: I,
    controls: CallOptions<'a>,
    function: Function,
) -> Result<RequestRows<'a, T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    let request = super::admit_native(function, definition)?;
    start(engine, &request, records, controls, None)
}

pub(in crate::public) fn annotation<'a, I, T, R: 'a>(
    engine: &Engine,
    questions: crate::QuestionSet,
    records: I,
    controls: CallOptions<'a>,
    project: impl Fn(T, usize, CompleteAnnotated) -> Result<Option<R>, Error> + 'a,
) -> Result<Batch<'a, R>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    let request = super::admit_native(Function::Annotate, questions.into())?;
    super::execution::intake(
        engine,
        &request,
        records,
        controls,
        |engine, definition, rows, controls, context| {
            let set = definition.annotation()?;
            engine.annotate_stream_project(
                set,
                rows,
                controls,
                context,
                None,
                move |original, at, result| project(original.original, at, result),
            )
        },
    )
}

/// Native rows project the same admitted Request execution and scheduler.
pub(in crate::public) fn project<'a, I, T, R: 'a>(
    engine: &Engine,
    function: InputFunction,
    question: crate::Question,
    records: I,
    controls: CallOptions<'a>,
    project: impl Fn(T, usize, crate::core::CompleteAtomic) -> Result<Option<R>, Error> + 'a,
) -> Result<Batch<'a, R>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    let request = super::admit_native(Function::from_input(function), question.into())?;
    super::execution::intake(
        engine,
        &request,
        records,
        controls,
        |engine, definition, rows, controls, context| {
            let q = definition.atomic()?;
            engine.complete_stream_project(
                function,
                q,
                rows,
                controls,
                context,
                move |original, at, result| project(original.original, at, result),
            )
        },
    )
}

impl<T> std::fmt::Debug for RequestRows<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestRows").finish_non_exhaustive()
    }
}
impl<T> RequestRows<'_, T> {
    /// Final native facts become present after exhaustion or terminal failure.
    #[must_use]
    pub fn facts(&self) -> Option<&crate::Facts> {
        match self {
            Self::Decisions(b) => b.facts(),
            Self::Choices(b) => b.facts(),
            Self::Tags(b) => b.facts(),
            Self::Scores(b) => b.facts(),
            Self::Filtered(b) => b.facts(),
            Self::Annotations(b) => b.facts(),
        }
    }
}
impl Engine {
    /// Execute a lazy record request through shared admission and native scheduling.
    /// Originals, observers, cancellation, completed prefixes and reader failures retain
    /// the collecting request contract; final facts settle when the batch terminates.
    /// # Errors
    /// Refuses incompatible request headers before advancing the supplied reader.
    pub fn execute_request_rows<'a, I, T>(
        &self,
        request: &AdmittedRequest,
        records: I,
        controls: CallOptions<'a>,
    ) -> Result<RequestRows<'a, T>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        start(self, request, records, controls, None)
    }
}
