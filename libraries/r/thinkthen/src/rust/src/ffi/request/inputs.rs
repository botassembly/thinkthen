//! Convert separate host values; the shared Rust grammar owns their admission.
use crate::calls::Crossed;
use extendr_api::prelude::*;
use thinkthen::{Request, RequestArguments, RequestInput, RequestQuestion};
fn decode<T: serde::de::DeserializeOwned>(text: &str) -> Crossed<T> {
    serde_json::from_str(text).map_err(|_| crate::usage("invalid canonical request"))
}
fn flag(value: &Robj, name: &str) -> bool {
    value
        .get_attrib(name)
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSelector {
    kind: String,
    value: Box<serde_json::value::RawValue>,
}
fn authored(text: &str) -> Crossed<RequestQuestion> {
    thinkthen::RequestDefinition::from_request_json(text)
        .map(|value| RequestQuestion::Definition { value })
        .map_err(|error| crate::carry(&error))
}
fn question(value: &Robj) -> Crossed<RequestQuestion> {
    let text = super::text_of(value, "question")?;
    if flag(value, "selector") {
        if let Ok(selector) = serde_json::from_str::<AuthoredSelector>(&text)
            && selector.kind == "definition"
        {
            return authored(selector.value.get());
        }
        RequestQuestion::from_json(&text).map_err(|error| crate::carry(&error))
    } else if flag(value, "literal") {
        Ok(RequestQuestion::Text {
            text: decode(&text)?,
        })
    } else {
        authored(&text)
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
fn input(
    value: &Robj,
    function: thinkthen::RequestFunction,
) -> Crossed<(RequestInput, Option<thinkthen::RequestColumn>)> {
    if value.rtype() == Rtype::Strings {
        return RequestInput::from_json(&super::text_of(value, "input")?)
            .map(|input| (input, None))
            .map_err(|error| crate::carry(&error));
    }
    let values =
        List::try_from(value.clone()).map_err(|_| crate::usage("invalid canonical request"))?;
    let mut cells = Vec::new();
    for (_, value) in values.iter() {
        let item = if flag(&value, "missing") {
            None
        } else {
            Some(item(&value)?)
        };
        cells.push(item);
    }
    let (column, items) = if flag(value, "column") {
        let (column, items) =
            thinkthen::RequestColumn::new(function, cells).map_err(|e| crate::carry(&e))?;
        (Some(column), items)
    } else {
        (None, cells.into_iter().flatten().collect())
    };
    let scalar = value
        .get_attrib("scalar")
        .filter(|value| !value.is_null())
        .filter(|_| {
            column
                .as_ref()
                .is_none_or(|column| column.positions().len() == 1)
        })
        .map(|value| item(&value))
        .transpose()?;
    Ok((function.host_input(items, scalar), column))
}
fn item(value: &Robj) -> Crossed<thinkthen::RequestItem> {
    Ok(thinkthen::RequestItem {
        original: Some(original(value)?),
        context: None,
        options: None,
        seed_spans: None,
        examples: None,
        images: Vec::new(),
    })
}

pub(super) fn request(
    function: &str,
    asked: &Robj,
    source: &Robj,
    options: &Robj,
) -> Crossed<(Request, Option<thinkthen::RequestColumn>)> {
    let function: thinkthen::RequestFunction = function.parse().map_err(|e| crate::carry(&e))?;
    let (input, column) = input(source, function)?;
    let args = RequestArguments {
        question: question(asked)?,
        input,
        options: if options.is_null() {
            Default::default()
        } else {
            decode(&super::text_of(options, "options")?)?
        },
    };
    Ok((Request::for_function(function, args), column))
}
