//! Borrowed native originals enter the admitted Request edge without a result reader.
use super::{AdmittedRequest, RequestDefinition, RequestFunction as Function};
use crate::{
    Batch, CallOptions, CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter,
    CompleteRecord, CompleteScore, CompleteTags, Engine, Error, InputEvidence, InputFunction,
    LoadedQuestion, QuestionInput, RecordInput,
};

pub(crate) enum Rows<'a, T> {
    Decisions(Batch<'a, CompleteRecord<T, CompleteDecision>>),
    Choices(Batch<'a, CompleteRecord<T, CompleteChoice>>),
    Tags(Batch<'a, CompleteRecord<T, CompleteTags>>),
    Scores(Batch<'a, CompleteRecord<T, CompleteScore>>),
    Filtered(Batch<'a, CompleteRecord<T, CompleteFilter>>),
    Annotations(Batch<'a, CompleteRecord<T, CompleteAnnotated>>),
}

struct Original<T> {
    original: T,
    input: QuestionInput,
}
impl<T> InputEvidence for Original<T> {
    fn question_input(&self) -> QuestionInput {
        self.input.clone()
    }
}
fn restore<T, R>(row: CompleteRecord<Original<T>, R>) -> CompleteRecord<T, R> {
    CompleteRecord {
        original: row.original.original,
        ordinal: row.ordinal,
        result: row.result,
    }
}

pub(crate) fn start<'a, I, T>(
    engine: &Engine,
    request: &AdmittedRequest,
    records: I,
    controls: CallOptions<'a>,
    recover: crate::public::options::AnnotationRecovery<'a>,
) -> Result<Rows<'a, T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    super::native_feed::header(request)?;
    let prepared = super::execution::prepare(engine, request, controls, false)?;
    let function = request.request.call.function();
    let options = request.request.call.arguments().options.clone();
    let super::execution::Prepared {
        reading,
        definition,
        engine,
        controls,
        context,
        image_refusal,
    } = prepared;
    let records = records.into_iter().enumerate().map(move |(at, row)| {
        let prepare = || {
            controls.admission()?;
            let row = row?;
            let input = row.original.question_input();
            super::native_feed::validate(function, &row, &input, image_refusal.as_deref())?;
            let original = row.original;
            let snapshot = RecordInput {
                original: input,
                context: row.context,
                options: row.options,
                seed_spans: row.seed_spans,
                examples: row.examples,
            };
            super::inline::validate_composed(&reading, &options, &snapshot)?;
            Ok(snapshot.map_original(|input| Original { original, input }))
        };
        prepare().map_err(|error: Error| error.at_record(at))
    });
    let rows = dispatch(
        &engine, function, definition, records, controls, context, recover,
    )?;
    Ok(match rows {
        Rows::Decisions(batch) => Rows::Decisions(batch.map_rows(restore)),
        Rows::Choices(batch) => Rows::Choices(batch.map_rows(restore)),
        Rows::Tags(batch) => Rows::Tags(batch.map_rows(restore)),
        Rows::Scores(batch) => Rows::Scores(batch.map_rows(restore)),
        Rows::Filtered(batch) => Rows::Filtered(batch.map_rows(restore)),
        Rows::Annotations(batch) => Rows::Annotations(batch.map_rows(restore)),
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
) -> Result<Rows<'a, T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    macro_rules! atomic {
        ($kind:ident, $input:ident, $convert:ident, $q:expr) => {
            Ok(Rows::$kind(engine.complete_stream(
                InputFunction::$input,
                $q,
                records,
                controls,
                context,
                crate::public::complete::$convert,
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
        (Function::Tag, RequestDefinition::Atomic(q)) => atomic!(Tags, Tag, tags, question(q)),
        (Function::Score, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            atomic!(Scores, Score, score, q)
        }
        (Function::Filter, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            Ok(Rows::Filtered(engine.complete_stream(
                InputFunction::Filter,
                q,
                records,
                controls,
                context,
                crate::public::complete::records::filter,
            )?))
        }
        (Function::Choose, RequestDefinition::DynamicChoose(q)) => Ok(Rows::Choices(
            engine.dynamic_choose_stream(q, records, controls, context)?,
        )),
        (Function::Annotate, RequestDefinition::Annotate(set)) => Ok(Rows::Annotations(
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

pub(crate) fn native<'a, I, T>(
    engine: &Engine,
    definition: RequestDefinition,
    records: I,
    controls: CallOptions<'a>,
    function: fn(super::RequestArguments) -> super::RequestCall,
) -> Result<Rows<'a, T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    T: InputEvidence + 'a,
{
    let request = super::Request::new(function(super::RequestArguments {
        question: super::RequestQuestion::Definition { value: definition },
        input: super::RequestInput::Feed {
            name: "native".into(),
            framing: super::RequestFraming::Document,
            reading: crate::ReaderOptions::default(),
            images: vec![],
        },
        options: super::RequestOptions::default(),
    }))
    .admit()?;
    start(engine, &request, records, controls, None)
}
