//! Shared caller fixture framing; admission and evidence composition stay native.
use serde::Deserialize;
use serde_json::{Map, Value};
use thinkthen::{
    AdmittedRequest, Call, Engine, EngineBuilder, Error, ErrorKind, Facts, InputEvidence,
    InputReaderOptions, QuestionInput, Request, RequestArguments, RequestCall, RequestInput,
    RequestOptions, RequestQuestion, SourceItem,
};

pub(super) type Original = QuestionInput;
#[allow(
    dead_code,
    reason = "Rust and Polars callers use different fixture controls"
)]
#[derive(Deserialize)]
pub(super) struct Fixture {
    pub verb: String,
    pub question: RequestQuestion,
    pub input: RequestInput,
    pub settings: Map<String, Value>,
    #[serde(default)]
    pub options: RequestOptions,
    #[serde(default)]
    pub cancel: bool,
    #[serde(default)]
    pub held_cancel: bool,
    #[serde(default)]
    pub incremental: bool,
    #[serde(default)]
    pub batch_probe: bool,
    pub deadline_ms: Option<i64>,
    pub shared_context: Option<String>,
}
impl Fixture {
    pub(super) fn request(&self) -> Result<Request, Error> {
        self.request_input(self.input.clone())
    }
    pub(super) fn request_input(&self, input: RequestInput) -> Result<Request, Error> {
        let args = RequestArguments {
            question: self.question.clone(),
            input,
            options: self.options.clone(),
        };
        let call = match self.verb.as_str() {
            "decide" => RequestCall::Decide(args),
            "choose" => RequestCall::Choose(args),
            "tag" => RequestCall::Tag(args),
            "score" => RequestCall::Score(args),
            "filter" => RequestCall::Filter(args),
            "rank" => RequestCall::Rank(args),
            "find" => RequestCall::Find(args),
            "annotate" => RequestCall::Annotate(args),
            "recognize" => RequestCall::Recognize(args),
            "relate" => RequestCall::Relate(args),
            _ => return Err(usage("unknown fixture function")),
        };
        Ok(Request::new(call))
    }
}
pub(super) fn usage(message: &str) -> Error {
    Error::new(ErrorKind::Usage, message)
}
pub(super) fn engine(settings: Map<String, Value>) -> Result<Engine, Error> {
    let text = serde_json::to_string(&settings).map_err(|_| usage("invalid fixture settings"))?;
    EngineBuilder::from_settings_json(&text)?.build()
}
pub(super) fn failure(error: &Error) -> Result<String, Error> {
    serde_json::to_string(error.complete().error()).map_err(|_| usage("invalid native failure"))
}
pub(super) fn packet<T: serde::Serialize>(
    value: &T,
    facts: Option<thinkthen::CompleteFacts<'_>>,
) -> Result<String, Error> {
    #[derive(serde::Serialize)]
    struct Packet<'a, T> {
        native: bool,
        results: &'a T,
        facts: Option<thinkthen::CompleteFacts<'a>>,
    }
    serde_json::to_string(&Packet {
        native: true,
        results: value,
        facts,
    })
    .map_err(|_| usage("invalid native results"))
}
pub(super) fn written<T: serde::Serialize>(call: Call<T>) -> Result<(String, Facts), Error> {
    Ok((
        packet(call.value(), call.facts().complete())?,
        call.facts().clone(),
    ))
}
type Inputs<'a> =
    Box<dyn Iterator<Item = Result<thinkthen::RecordInput<QuestionInput>, Error>> + 'a>;
pub(super) fn inputs<'a>(request: &'a AdmittedRequest) -> Result<Inputs<'a>, Error> {
    let reading = request.record_reading()?;
    match &request.request().call.arguments().input {
        RequestInput::Records { items }
        | RequestInput::Units { items }
        | RequestInput::Entities { items } => Ok(Box::new(items.iter().map(move |item| {
            let mut row = item.compose_record(&reading)?;
            if matches!(request.request().call, RequestCall::Annotate(_))
                && item.images.is_empty()
                && let Some(thinkthen::RequestOriginal::Text { text }) = &item.original
            {
                row.original = QuestionInput::annotation_document(text)?;
            }
            Ok(row)
        }))),
        RequestInput::Source { source } => {
            let jsonl = matches!(source.framing, Some(thinkthen::RequestFraming::Jsonl));
            let sources = thinkthen::read_inputs(
                &source.paths,
                InputReaderOptions {
                    reading: source.reading,
                    media: source.media,
                },
            )?;
            Ok(Box::new(sources.map(move |item| {
                let item = item?;
                if jsonl && let SourceItem::Text(source) = item {
                    let original = thinkthen::RawRecord::json(&source.record)?;
                    let mut row = reading.compose(original)?;
                    row.original = row.original.with_location(thinkthen::SourceLocation::new(
                        source.file,
                        Some(source.first_line),
                        Some(source.last_line),
                    )?);
                    return Ok(row.map_original(|original| original.question_input()));
                }
                reading.compose_source(item)
            })))
        }
        _ => Err(usage(
            "the shared fixture requires records or explicit files",
        )),
    }
}

pub(super) fn admit(request: Request) -> Result<AdmittedRequest, Error> {
    let saved = !matches!(
        request.call.arguments().question,
        RequestQuestion::Definition { .. }
    );
    let admitted = request.admit()?;
    if saved {
        let definition = admitted.resolve_question()?;
        admitted.with_resolved_definition(definition)
    } else {
        Ok(admitted)
    }
}
