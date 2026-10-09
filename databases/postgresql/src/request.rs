//! PostgreSQL projections execute the same admitted typed request as complete calls.
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
            controls: controls.surface(thinkthen::Surface::Postgresql),
            feed: None,
        },
    )? {
        RequestOutcome::Complete(call) => Ok(call),
        RequestOutcome::Failed { error, .. } => Err(error),
    }
}

pub(crate) fn wrong_result() -> Error {
    crate::call::defect("the admitted request returned another function's result")
}
