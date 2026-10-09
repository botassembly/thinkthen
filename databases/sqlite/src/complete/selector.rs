//! SQLite admits canonical saved headers before its authorized descriptor reader.
use thinkthen::{
    AdmittedRequest, Error, Request, RequestArguments, RequestCall, RequestInput, RequestOptions,
    RequestQuestion,
};

pub(super) fn admit(verb: &str, source: &str) -> Result<Option<AdmittedRequest>, Error> {
    if !source.starts_with('@') {
        return Ok(None);
    }
    let question = match source.strip_prefix("@@") {
        Some(name) => RequestQuestion::Name {
            name: name.to_owned(),
        },
        None => RequestQuestion::Reference {
            reference: source.to_owned(),
        },
    };
    let args = RequestArguments {
        question,
        input: RequestInput::Feed {
            name: "sqlite".to_owned(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        options: RequestOptions::default(),
    };
    let call = match verb {
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
        _ => return Err(super::super::complete_native::defect()),
    };
    Request::new(call).admit().map(Some)
}
