//! The caller constructs typed original columns before asking the public adapter.
use serde_json::Value;
use std::collections::BTreeMap;
use thinkthen::Error;
use thinkthen::polars::prelude::{AnyValue, DataType, Field, Series};

pub(super) fn originals(values: &[Value]) -> Result<Series, Error> {
    let mut dtype = DataType::Null;
    for value in values {
        dtype = merge(dtype, kind(value)?)?;
    }
    if dtype == DataType::Null {
        dtype = DataType::String;
    }
    series("original", values, &dtype)
}
fn kind(value: &Value) -> Result<DataType, Error> {
    Ok(match value {
        Value::Null => DataType::Null,
        Value::Bool(_) => DataType::Boolean,
        Value::String(_) => DataType::String,
        Value::Number(value) => {
            if value.is_i64() {
                DataType::Int64
            } else if value.is_u64() {
                DataType::UInt64
            } else {
                DataType::Float64
            }
        }
        Value::Array(values) => {
            let mut item = DataType::Null;
            for value in values {
                item = merge(item, kind(value)?)?;
            }
            DataType::List(Box::new(item))
        }
        Value::Object(values) => DataType::Struct(
            values
                .iter()
                .map(|(key, value)| Ok(Field::new(key.into(), kind(value)?)))
                .collect::<Result<Vec<_>, Error>>()?,
        ),
    })
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
        _ => Err(crate::fixture::usage(
            "the originals do not fit one typed Polars column",
        )),
    }
}
fn series(name: &str, values: &[Value], dtype: &DataType) -> Result<Series, Error> {
    let values = values
        .iter()
        .map(|value| cell(value, dtype))
        .collect::<Result<Vec<_>, Error>>()?;
    Series::from_any_values_and_dtype(name.into(), &values, dtype, true)
        .map_err(|_| crate::fixture::usage("invalid original column"))
}
fn cell(value: &Value, dtype: &DataType) -> Result<AnyValue<'static>, Error> {
    if value.is_null() {
        return Ok(AnyValue::Null);
    }
    let invalid = || crate::fixture::usage("invalid original field type");
    Ok(match dtype {
        DataType::Null => AnyValue::Null,
        DataType::Boolean => AnyValue::Boolean(value.as_bool().ok_or_else(invalid)?),
        DataType::String => AnyValue::StringOwned(value.as_str().ok_or_else(invalid)?.into()),
        DataType::Int64 => AnyValue::Int64(value.as_i64().ok_or_else(invalid)?),
        DataType::UInt64 => AnyValue::UInt64(value.as_u64().ok_or_else(invalid)?),
        DataType::Float64 => AnyValue::Float64(value.as_f64().ok_or_else(invalid)?),
        DataType::List(inner) => {
            AnyValue::List(series("", value.as_array().ok_or_else(invalid)?, inner)?)
        }
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
        _ => return Err(invalid()),
    })
}
