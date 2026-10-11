//! Convert separate host values; the shared Rust grammar owns their admission.
use crate::calls::Crossed;
use extendr_api::prelude::*;
use thinkthen::{Request, RequestArguments, RequestCall, RequestInput, RequestQuestion};
fn decode<T: serde::de::DeserializeOwned>(text: &str) -> Crossed<T> {
    serde_json::from_str(text).map_err(|error| {
        crate::usage(
            if error
                .to_string()
                .starts_with("per-item context is text or an object")
            {
                "the per-item context does not match context_schema"
            } else {
                "invalid canonical request"
            },
        )
    })
}
fn flag(value: &Robj, name: &str) -> bool {
    value
        .get_attrib(name)
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}
#[derive(serde::Deserialize)]
struct AuthoredSelector {
    kind: String,
    value: Box<serde_json::value::RawValue>,
}
fn question(value: &Robj) -> Crossed<RequestQuestion> {
    let text = super::text_of(value, "question")?;
    if flag(value, "selector") {
        if let Ok(selector) = serde_json::from_str::<AuthoredSelector>(&text)
            && selector.kind == "definition"
        {
            return thinkthen::RequestDefinition::from_authored_json(selector.value.get())
                .map(|value| RequestQuestion::Definition { value })
                .map_err(|e| crate::carry(&e));
        }
        decode(&text)
    } else if flag(value, "literal") {
        Ok(RequestQuestion::Text {
            text: decode(&text)?,
        })
    } else {
        thinkthen::RequestDefinition::from_authored_json(&text)
            .map(|value| RequestQuestion::Definition { value })
            .map_err(|e| crate::carry(&e))
    }
}
fn original(value: &Robj) -> Crossed<thinkthen::RequestOriginal> {
    let text = super::text_of(value, "input")?;
    if flag(value, "selector") {
        decode(&text)
    } else if flag(value, "text") {
        Ok(thinkthen::RequestOriginal::Text {
            text: decode(&text)?,
        })
    } else {
        thinkthen::RawRecord::json(&text)
            .map(|value| thinkthen::RequestOriginal::Json { value })
            .map_err(|e| crate::carry(&e))
    }
}
fn input(value: &Robj) -> Crossed<RequestInput> {
    if value.rtype() == Rtype::Strings {
        return decode(&super::text_of(value, "input")?);
    }
    let values =
        List::try_from(value.clone()).map_err(|_| crate::usage("invalid canonical request"))?;
    let mut items = Vec::new();
    for (_, value) in values.iter() {
        items.push(thinkthen::RequestItem {
            original: Some(original(&value)?),
            context: None,
            options: None,
            seed_spans: None,
            examples: None,
            images: Vec::new(),
        });
    }
    if !flag(value, "collection")
        && let Some(item) = items.pop()
    {
        return match item.original {
            Some(thinkthen::RequestOriginal::Text { text }) => Ok(RequestInput::Text {
                text,
                images: Vec::new(),
            }),
            Some(thinkthen::RequestOriginal::Json { value }) => Ok(RequestInput::Json {
                value,
                images: Vec::new(),
            }),
            None => Err(crate::defect("native scalar input lost its original")),
        };
    }
    Ok(RequestInput::Records { items })
}
pub(super) fn request(
    function: &str,
    asked: &Robj,
    source: &Robj,
    options: &Robj,
) -> Crossed<Request> {
    let args = RequestArguments {
        question: question(asked)?,
        input: input(source)?,
        options: if options.is_null() {
            Default::default()
        } else {
            decode(&super::text_of(options, "options")?)?
        },
    };
    let call = match function {
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
        _ => return Err(crate::usage("invalid canonical request")),
    };
    Ok(Request::new(call))
}
