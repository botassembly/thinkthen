//! R representation only; result facts and semantics belong to Rust's public types.
use crate::calls::Crossed;
use crate::results_generated::convert;
use extendr_api::prelude::*;
use serde_json::Value;

type Field = (&'static str, bool, fn(&Value) -> Crossed<Robj>);

pub(crate) fn tagged(mut value: Robj, name: &str, family: &str) -> Crossed<Robj> {
    value
        .set_class([format!("thinkthen_{name}"), family.to_owned()])
        .map_err(|_| crate::defect("native R class could not be assigned"))?;
    Ok(value)
}

pub(crate) fn plain(value: &Value) -> Crossed<Robj> {
    Ok(match value {
        Value::Null => ().into(),
        Value::Bool(value) => (*value).into(),
        Value::String(value) => value.as_str().into(),
        Value::Number(value) => value
            .as_f64()
            .ok_or_else(|| crate::defect("native result number cannot be represented in R"))?
            .into(),
        Value::Array(_) => return array(value, plain),
        Value::Object(_) => return mapping(value, plain),
    })
}

pub(crate) fn array(value: &Value, read: impl Fn(&Value) -> Crossed<Robj>) -> Crossed<Robj> {
    if value.is_null() {
        return plain(value);
    }
    let values = value
        .as_array()
        .ok_or_else(|| crate::defect("native result is not an array"))?;
    let held = values.iter().map(read).collect::<Crossed<Vec<_>>>()?;
    Ok(List::from_values(held).into())
}

pub(crate) fn mapping(value: &Value, read: impl Fn(&Value) -> Crossed<Robj>) -> Crossed<Robj> {
    if value.is_null() {
        return plain(value);
    }
    let values = value
        .as_object()
        .ok_or_else(|| crate::defect("native result is not an object"))?;
    let held = values
        .iter()
        .map(|(key, value)| Ok((key.as_str(), read(value)?)))
        .collect::<Crossed<Vec<_>>>()?;
    Ok(List::from_pairs(held).into())
}

pub(crate) fn object(value: &Value, name: &str, fields: &[Field]) -> Crossed<Robj> {
    let values = value
        .as_object()
        .ok_or_else(|| crate::defect("native result is not an object"))?;
    let mut held = values
        .iter()
        .filter(|(key, _)| !fields.iter().any(|(name, _, _)| *name == key.as_str()))
        .map(|(key, value)| Ok((key.as_str(), plain(value)?)))
        .collect::<Crossed<Vec<_>>>()?;
    for (key, required, read) in fields {
        let item = match values.get(*key) {
            Some(value) => read(value)?,
            None if !required => tagged(List::new(0).into(), "absent", "thinkthen_absent")?,
            None => {
                return Err(crate::defect(
                    "native result has no required generated member",
                ));
            }
        };
        held.push((key, item));
    }
    tagged(List::from_pairs(held).into(), name, "thinkthen_complete")
}

fn result_kind(verb: &str) -> Crossed<&'static str> {
    Ok(match verb {
        "decide" => "completeAtomic_DecideValue",
        "choose" => "completeAtomic_Nullable_string",
        "tag" => "completeAtomic_Array_of_string",
        "score" => "completeAtomic_double",
        "filter" => "completeAtomic_boolean",
        "rank" => "completeAtomic_NonZeroUsize",
        "find" => "completeFind",
        "annotate" => "completeAnnotation",
        "recognize" => "completeRecognition",
        "relate" => "completeRelation",
        _ => return Err(crate::defect("native result has no named function")),
    })
}

fn decoded(text: &str) -> Crossed<Value> {
    serde_json::from_str(text)
        .map_err(|_| crate::defect("native complete result could not be decoded"))
}

// R conditions retain their flat complete-error carrier. The shared graph's
// error and facts converters own each nested field; only R's layout differs.
fn failed(value: &Value) -> Crossed<Robj> {
    let mut error = value.get("error").unwrap_or(value).clone();
    if let Some(Value::String(message)) = error.get_mut("message") {
        *message = message.replace('\0', "\\u0000");
    }
    let error = List::try_from(convert("completeError", &error)?)
        .map_err(|_| crate::defect("native error is not an R list"))?;
    let mut fields = error
        .iter()
        .filter(|(key, _)| *key != "facts")
        .collect::<Vec<_>>();
    let facts = match value.get("facts") {
        Some(facts) => convert("completeFacts", facts)?,
        None => tagged(List::new(0).into(), "absent", "thinkthen_absent")?,
    };
    fields.push(("facts", facts));
    tagged(
        List::from_pairs(fields).into(),
        "CallError",
        "thinkthen_complete",
    )
}

pub(crate) fn failure(text: &str) -> Crossed<Robj> {
    failed(&decoded(text)?)
}

fn function_kind(function: thinkthen::RequestFunction) -> Crossed<&'static str> {
    let name = serde_json::to_value(function)
        .map_err(|_| crate::defect("native function could not be written"))?;
    result_kind(
        name.as_str()
            .ok_or_else(|| crate::defect("native function has no name"))?,
    )
}

pub(crate) fn request_packet(text: &str, function: thinkthen::RequestFunction) -> Crossed<Robj> {
    let value = decoded(text)?;
    let kind = function_kind(function)?;
    let rows = value
        .get("results")
        .ok_or_else(|| crate::defect("native outcome has no results"))?;
    let results = if rows.is_array() {
        array(rows, |row| convert(kind, row))?
    } else {
        List::from_values([convert(kind, rows)?]).into()
    };
    let mut fields = vec![("results", results)];
    if let Some(facts) = value.get("facts") {
        fields.push(("facts", convert("completeFacts", facts)?));
    }
    if let Some(error) = value.get("failure") {
        fields.push(("failure", failed(error)?));
    }
    tagged(
        List::from_pairs(fields).into(),
        "Call",
        "thinkthen_complete",
    )
}

pub(crate) fn request_event(text: &str) -> Crossed<Robj> {
    let value = decoded(text)?;
    let kind = value
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| crate::defect("native session packet has no kind"))?;
    let mut fields = vec![("kind", kind.into())];
    if matches!(kind, "row" | "aggregate") {
        let function: thinkthen::RequestFunction = serde_json::from_value(
            value
                .get("function")
                .cloned()
                .ok_or_else(|| crate::defect("native session packet has no function"))?,
        )
        .map_err(|_| crate::defect("native session packet has an unknown function"))?;
        let payload = value
            .get("value")
            .ok_or_else(|| crate::defect("native session packet has no value"))?;
        let result = function_kind(function)?;
        fields.push((
            "value",
            if payload.is_array() {
                array(payload, |row| convert(result, row))?
            } else {
                convert(result, payload)?
            },
        ));
    }
    if let Some(facts) = value.get("facts") {
        fields.push(("facts", convert("completeFacts", facts)?));
    }
    if let Some(error) = value.get("failure") {
        fields.push(("failure", failed(error)?));
    }
    tagged(
        List::from_pairs(fields).into(),
        "SessionEvent",
        "thinkthen_complete",
    )
}
