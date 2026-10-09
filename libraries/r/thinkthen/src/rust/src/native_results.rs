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

pub(crate) fn packet(text: &str, verb: &str) -> Crossed<Robj> {
    let value: Value = serde_json::from_str(text)
        .map_err(|_| crate::defect("native complete result could not be decoded"))?;
    let kind = match verb {
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
    };
    let rows = &value["results"];
    let results = if matches!(verb, "find" | "relate") {
        List::from_values([convert(kind, rows)?]).into()
    } else {
        array(rows, |row| convert(kind, row))?
    };
    let packet: Robj = list!(
        results = results,
        facts = convert("completeFacts", &value["facts"])?,
        ordinals = plain(&value["ordinals"])?,
        inputs = plain(&value["inputs"])?
    )
    .into();
    let mut packet = packet;
    packet
        .set_class(["thinkthen_complete_call"])
        .map_err(|_| crate::defect("native complete call class could not be assigned"))?;
    Ok(packet)
}
