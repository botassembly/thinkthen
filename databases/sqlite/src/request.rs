//! SQLite projections execute the same admitted typed request as complete calls.
use thinkthen::{
    Call, CallOptions, Engine, Error, Request, RequestArguments, RequestCall, RequestDefinition,
    RequestEnvironment, RequestFunction, RequestInput, RequestItem, RequestOptions,
    RequestOriginal, RequestOutcome, RequestQuestion, RequestValue,
};

pub(crate) fn text(text: String) -> RequestItem {
    RequestItem {
        original: Some(RequestOriginal::Text { text }),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: Vec::new(),
    }
}

pub(crate) fn json(value: serde_json::Value) -> Result<RequestItem, Error> {
    let mut item = text(String::new());
    item.original = Some(RequestOriginal::Json {
        value: thinkthen::RawRecord::json(&value.to_string())?,
    });
    Ok(item)
}

pub(crate) fn document(text: String) -> Result<RequestItem, Error> {
    match thinkthen::QuestionInput::annotation_document(&text)? {
        thinkthen::QuestionInput::Record(record) => {
            let mut item = self::text(String::new());
            item.original = Some(RequestOriginal::Json {
                value: record.original().clone(),
            });
            Ok(item)
        }
        _ => Ok(self::text(text)),
    }
}

pub(crate) fn run(
    engine: &Engine,
    function: RequestFunction,
    definition: RequestDefinition,
    items: Vec<RequestItem>,
    controls: CallOptions<'_>,
) -> Result<Call<RequestValue>, Error> {
    let input = match function {
        RequestFunction::Find => RequestInput::Units { items },
        RequestFunction::Relate => RequestInput::Entities { items },
        _ => RequestInput::Records { items },
    };
    let args = RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input,
        options: RequestOptions::default(),
    };
    let call = match function {
        RequestFunction::Decide => RequestCall::Decide(args),
        RequestFunction::Choose => RequestCall::Choose(args),
        RequestFunction::Tag => RequestCall::Tag(args),
        RequestFunction::Score => RequestCall::Score(args),
        RequestFunction::Filter => RequestCall::Filter(args),
        RequestFunction::Rank => RequestCall::Rank(args),
        RequestFunction::Find => RequestCall::Find(args),
        RequestFunction::Annotate => RequestCall::Annotate(args),
        RequestFunction::Recognize => RequestCall::Recognize(args),
        RequestFunction::Relate => RequestCall::Relate(args),
    };
    let request = Request::new(call).admit()?;
    match engine.execute_request(
        &request,
        RequestEnvironment {
            controls: controls.surface(thinkthen::Surface::Sqlite),
            feed: None,
        },
    )? {
        RequestOutcome::Complete(call) => Ok(call),
        RequestOutcome::Failed { error, .. } => Err(error),
    }
}

pub(crate) fn wrong_result() -> Error {
    Error::new(
        thinkthen::ErrorKind::Defect,
        "the admitted request returned another function's result",
    )
}

pub(crate) fn details(
    engine: &Engine,
    question: &thinkthen::LoadedQuestion,
    items: Vec<RequestItem>,
    options: CallOptions<'_>,
    records: bool,
) -> Result<Vec<thinkthen::Details>, Error> {
    let function = match question {
        thinkthen::LoadedQuestion::Banded(_) => RequestFunction::Decide,
        thinkthen::LoadedQuestion::Question(question) => match question.kind() {
            thinkthen::QuestionKind::Decide => RequestFunction::Decide,
            thinkthen::QuestionKind::Choose => RequestFunction::Choose,
            thinkthen::QuestionKind::Tag => RequestFunction::Tag,
            thinkthen::QuestionKind::Score => RequestFunction::Score,
            _ => return Err(wrong_result()),
        },
    };
    let call = run(engine, function, question.clone().into(), items, options)?;
    macro_rules! project {
        ($rows:expr) => {
            $rows
                .into_iter()
                .map(|row| {
                    if records {
                        row.legacy_details()
                    } else {
                        row.result().legacy_details()
                    }
                })
                .collect()
        };
    }
    match call.into_value() {
        RequestValue::Decisions(rows) => project!(rows),
        RequestValue::Choices(rows) => project!(rows),
        RequestValue::Tags(rows) => project!(rows),
        RequestValue::Scores(rows) => project!(rows),
        _ => Err(wrong_result()),
    }
}
