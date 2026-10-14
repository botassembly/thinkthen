//! Polars types come from the generated native result schema, including empty lists.
use crate::{Error, RequestFunction};
use polars::prelude::{AnyValue, DataType, Field, Series};
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) struct Schema {
    document: Value,
    input: DataType,
    judgment: DataType,
    overrides: BTreeMap<String, DataType>,
}
impl Schema {
    pub(super) fn new(input: DataType, judgment: DataType) -> Result<Self, Error> {
        Ok(Self {
            document: serde_json::from_str(crate::complete_call_schema())
                .map_err(|_| Error::defect("the generated result schema is invalid"))?,
            input,
            judgment,
            overrides: BTreeMap::new(),
        })
    }
    pub(super) fn named(&self, name: &str) -> Result<DataType, Error> {
        if let Some(dtype) = self.overrides.get(name) {
            return Ok(dtype.clone());
        }
        let value = self
            .document
            .pointer(&format!("/$defs/{name}"))
            .ok_or_else(|| Error::defect("a native result schema is missing"))?;
        self.dtype(value)
    }
    pub(super) fn dtype(&self, value: &Value) -> Result<DataType, Error> {
        if value == &Value::Bool(true) {
            return Ok(self.input.clone());
        }
        if let Some(reference) = value.get("$ref").and_then(Value::as_str) {
            if reference == "#/$defs/completevalue" {
                return Ok(self.judgment.clone());
            }
            return self.named(reference.trim_start_matches("#/$defs/"));
        }
        if let Some(choices) = value
            .get("anyOf")
            .or_else(|| value.get("oneOf"))
            .and_then(Value::as_array)
        {
            let mut dtype = DataType::Null;
            for choice in choices {
                dtype = merge(dtype, self.dtype(choice)?)?;
            }
            if value.get("properties").is_some() {
                dtype = merge(dtype, self.object(value)?)?;
            }
            return Ok(dtype);
        }
        let kind = value.get("type").and_then(|kind| {
            kind.as_str().or_else(|| {
                kind.as_array()?
                    .iter()
                    .filter_map(Value::as_str)
                    .find(|kind| *kind != "null")
            })
        });
        match kind {
            Some("null") => Ok(DataType::Null),
            None => Ok(if value.get("const").is_some_and(Value::is_string) {
                DataType::String
            } else {
                DataType::Null
            }),
            Some("boolean") => Ok(DataType::Boolean),
            Some("string") => Ok(DataType::String),
            Some("number") => Ok(DataType::Float64),
            Some("integer") => Ok(
                if value
                    .get("minimum")
                    .and_then(Value::as_i64)
                    .is_some_and(|minimum| minimum >= 0)
                {
                    DataType::UInt64
                } else {
                    DataType::Int64
                },
            ),
            Some("array") => Ok(DataType::List(Box::new(
                self.dtype(
                    value
                        .get("items")
                        .ok_or_else(|| Error::defect("an array schema lost its item type"))?,
                )?,
            ))),
            Some("object") => self.object(value),
            _ => Err(Error::defect("a native schema has no Polars type")),
        }
    }
    pub(super) fn override_type(&mut self, name: &str, dtype: DataType) {
        self.overrides.insert(name.to_owned(), dtype);
    }
    pub(super) fn property(&self, name: &str, property: &str) -> Result<DataType, Error> {
        let value = self
            .document
            .pointer(&format!("/$defs/{name}/properties/{property}"))
            .ok_or_else(|| Error::defect("a native result property is missing"))?;
        self.dtype(value)
    }
    fn object(&self, value: &Value) -> Result<DataType, Error> {
        if let Some(properties) = value.get("properties").and_then(Value::as_object) {
            return Ok(DataType::Struct(
                properties
                    .iter()
                    .map(|(name, value)| Ok(Field::new(name.into(), self.dtype(value)?)))
                    .collect::<Result<Vec<_>, Error>>()?,
            ));
        }
        if let Some(item) = value.get("additionalProperties") {
            return Ok(DataType::List(Box::new(DataType::Struct(vec![
                Field::new("key".into(), DataType::String),
                Field::new("value".into(), self.dtype(item)?),
            ]))));
        }
        Err(Error::defect("an object schema has no declared fields"))
    }
}
fn merge(left: DataType, right: DataType) -> Result<DataType, Error> {
    if left == DataType::Null || left == right {
        return Ok(right);
    }
    if right == DataType::Null {
        return Ok(left);
    }
    match (left, right) {
        (DataType::Struct(left), DataType::Struct(right)) => {
            let mut fields: BTreeMap<_, _> = left
                .into_iter()
                .map(|field| (field.name().clone(), field.dtype().clone()))
                .collect();
            for field in right {
                let name = field.name().clone();
                let dtype = match fields.remove(&name) {
                    Some(previous) => merge(previous, field.dtype().clone())?,
                    None => field.dtype().clone(),
                };
                fields.insert(name, dtype);
            }
            Ok(DataType::Struct(
                fields
                    .into_iter()
                    .map(|(name, dtype)| Field::new(name, dtype))
                    .collect(),
            ))
        }
        (DataType::List(left), DataType::List(right)) => {
            Ok(DataType::List(Box::new(merge(*left, *right)?)))
        }
        _ => Err(Error::defect(
            "a schema union needs a typed question projection",
        )),
    }
}
pub(super) fn record_name(function: RequestFunction) -> &'static str {
    match function {
        RequestFunction::Decide => "completeDecide",
        RequestFunction::Choose => "completeChoose",
        RequestFunction::Score => "completeScore",
        RequestFunction::Tag => "completeTag",
        RequestFunction::Filter => "completeFilter",
        RequestFunction::Rank => "completeRank",
        RequestFunction::Find => "completeFind",
        RequestFunction::Annotate => "completeAnnotation",
        RequestFunction::Recognize => "completeRecognition",
        RequestFunction::Relate => "completeRelation",
    }
}
pub(super) fn series(name: &str, values: &[Value], dtype: &DataType) -> Result<Series, Error> {
    let values = values
        .iter()
        .map(|value| cell(value, dtype))
        .collect::<Result<Vec<_>, Error>>()?;
    Series::from_any_values_and_dtype(name.into(), &values, dtype, true).map_err(|error| {
        Error::defect(&format!(
            "a typed result column could not be built: {error}"
        ))
    })
}
fn cell(value: &Value, dtype: &DataType) -> Result<AnyValue<'static>, Error> {
    if value.is_null() {
        return Ok(AnyValue::Null);
    }
    let invalid = || Error::defect("a canonical result does not match its generated column type");
    Ok(match dtype {
        DataType::Null => AnyValue::Null,
        DataType::Boolean => AnyValue::Boolean(value.as_bool().ok_or_else(invalid)?),
        DataType::String => AnyValue::StringOwned(value.as_str().ok_or_else(invalid)?.into()),
        DataType::Float64 => AnyValue::Float64(value.as_f64().ok_or_else(invalid)?),
        DataType::UInt64 => AnyValue::UInt64(value.as_u64().ok_or_else(invalid)?),
        DataType::Int64 => AnyValue::Int64(value.as_i64().ok_or_else(invalid)?),
        DataType::Struct(fields) => AnyValue::StructOwned(Box::new((
            fields
                .iter()
                .map(|field| {
                    cell(
                        value.get(field.name().as_str()).unwrap_or(&Value::Null),
                        field.dtype(),
                    )
                })
                .collect::<Result<Vec<_>, Error>>()?,
            fields.clone(),
        ))),
        DataType::List(inner) => {
            let mapped;
            let values = if let Some(values) = value.as_array() {
                values
            } else if let Some(fields) = value.as_object() {
                mapped = fields
                    .iter()
                    .map(|(key, value)| serde_json::json!({"key":key,"value":value}))
                    .collect::<Vec<_>>();
                &mapped
            } else {
                return Err(invalid());
            };
            AnyValue::List(series("", values, inner)?)
        }
        _ => return Err(invalid()),
    })
}
