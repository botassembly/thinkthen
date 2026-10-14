//! SQLite admits canonical saved headers before its authorized descriptor reader.
use thinkthen::{
    AdmittedRequest, Error, Request, RequestArguments, RequestInput, RequestOptions,
    RequestQuestion,
};

pub(crate) fn admit(verb: &str, source: &str) -> Result<Option<AdmittedRequest>, Error> {
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
    let request = Request::named(verb, args)?;
    request.admit().map(Some)
}
