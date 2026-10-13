//! Named SQL fields use the native settings grammar and one schema description.
use crate::{Failure, question};
use rusqlite::types::ValueRef;
use std::borrow::Cow;
use std::ffi::{CStr, CString};

enum Kind {
    Text,
    Integer,
    Boolean,
    Json,
}
const FIELDS: &[(&str, &str, Kind)] = &[
    ("model", "model", Kind::Text),
    ("context", "context", Kind::Text),
    ("batch", "batch", Kind::Integer),
    ("deadline_ms", "deadline_ms", Kind::Integer),
    ("threshold", "threshold", Kind::Text),
    ("true_reading", "true", Kind::Text),
    ("false_reading", "false", Kind::Text),
    ("options", "options", Kind::Json),
    ("levels", "levels", Kind::Json),
    ("labels", "labels", Kind::Json),
    ("none", "none", Kind::Boolean),
];
pub(crate) const COUNT: usize = FIELDS.len();

pub(crate) fn schema(base: &'static CStr) -> Result<Cow<'static, CStr>, Failure> {
    let base = base
        .to_str()
        .map_err(|_| Failure::defect("table schema is not text"))?;
    let base = base
        .strip_suffix(')')
        .ok_or_else(|| Failure::defect("table schema lost its columns"))?;
    let fields = FIELDS
        .iter()
        .map(|(name, _, _)| format!(", {name} HIDDEN"))
        .collect::<String>();
    Ok(Cow::Owned(
        CString::new(format!("{base}{fields})"))
            .map_err(|_| Failure::defect("table schema contains a zero byte"))?,
    ))
}

pub(crate) fn settings<'a>(
    source: &'a [u8],
    values: &[ValueRef<'_>],
) -> Result<Cow<'a, [u8]>, Failure> {
    if values.iter().all(|value| matches!(value, ValueRef::Null)) {
        return Ok(Cow::Borrowed(source));
    }
    question::call_settings(ValueRef::Text(source))?;
    let text =
        std::str::from_utf8(source).map_err(|_| Failure::usage("the settings are UTF-8 text"))?;
    let fields: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(text).map_err(|_| Failure::usage("the settings are one object"))?;
    let mut output = text
        .trim_end()
        .strip_suffix('}')
        .ok_or_else(|| Failure::usage("the settings are one object"))?
        .to_owned();
    let mut comma = !fields.is_empty();
    for ((name, key, kind), value) in FIELDS.iter().zip(values) {
        if matches!(value, ValueRef::Null) {
            continue;
        }
        if fields.contains_key(*key) {
            return Err(Failure::usage(format!(
                "{name} is supplied in settings and by name"
            )));
        }
        let value = match (kind, value) {
            (Kind::Text, ValueRef::Text(bytes)) => serde_json::to_string(
                &std::str::from_utf8(bytes)
                    .map_err(|_| Failure::usage(format!("{name} is UTF-8 text")))?,
            )
            .map_err(|_| Failure::defect("named text could not be encoded"))?,
            (Kind::Integer, ValueRef::Integer(value)) => value.to_string(),
            (Kind::Boolean, ValueRef::Integer(value @ (0 | 1))) => (*value == 1).to_string(),
            (Kind::Json, ValueRef::Text(bytes)) => {
                let text = std::str::from_utf8(bytes)
                    .map_err(|_| Failure::usage(format!("{name} is UTF-8 JSON text")))?;
                let members: serde_json::Value = serde_json::from_str(text)
                    .map_err(|_| Failure::usage(format!("{name} is a JSON member list or map")))?;
                if !members.is_array() && !members.is_object() {
                    return Err(Failure::usage(format!(
                        "{name} is a JSON member list or map"
                    )));
                }
                text.to_owned()
            }
            _ => return Err(Failure::usage(format!("{name} has the wrong SQL type"))),
        };
        if comma {
            output.push(',');
        }
        output.push_str(&format!("\"{key}\":{value}"));
        comma = true;
    }
    output.push('}');
    question::call_settings(ValueRef::Text(output.as_bytes()))?;
    Ok(Cow::Owned(output.into_bytes()))
}
