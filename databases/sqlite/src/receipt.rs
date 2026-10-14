//! Generated named relational projections preserve native receipt fields.
use crate::{Failure, receipt_generated::PROJECTIONS};
use rusqlite::types::Value;
use serde_json::Value as Json;
use std::collections::{BTreeMap, HashMap};

pub(crate) type Cells = BTreeMap<usize, Value>;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Scalar {
    Text,
    Integer,
    Unsigned,
    Real,
    Boolean,
    Data,
}
#[derive(Clone, Copy, Debug)]
pub(crate) enum Shape {
    Scalar(Scalar),
    Object(&'static str),
    Array(&'static str),
    Mapping(&'static str),
}
#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name: &'static str,
    pub(crate) shape: Shape,
}
#[derive(Debug)]
pub(crate) struct Projection {
    pub(crate) name: &'static str,
    pub(crate) union: bool,
    pub(crate) fields: &'static [Field],
}

pub(crate) fn projection(name: &str) -> Result<&'static Projection, Failure> {
    PROJECTIONS
        .iter()
        .find(|projection| projection.name == name)
        .ok_or_else(|| Failure::usage(format!("unknown receipt projection {name}")))
}

impl Projection {
    fn columns(&self) -> Vec<(String, &'static str)> {
        let mut columns = vec![(format!("{}_receipt_index", self.name), "INTEGER")];
        for field in self.fields {
            columns.extend(field_columns(self.name, field));
        }
        columns
    }
}

fn field_columns(owner: &str, field: &Field) -> Vec<(String, &'static str)> {
    let name = if owner == "question" && field.name == "name" {
        "question_definition_name".to_owned()
    } else {
        format!("{owner}_{}", field.name)
    };
    let mut columns = vec![(format!("{name}_present"), "INTEGER")];
    match field.shape {
        Shape::Scalar(kind) => {
            if matches!(kind, Scalar::Data) {
                columns.push((format!("{name}_type"), "TEXT"));
            }
            columns.push((
                name,
                match kind {
                    Scalar::Text => "TEXT",
                    Scalar::Integer | Scalar::Boolean => "INTEGER",
                    Scalar::Real => "REAL",
                    Scalar::Unsigned | Scalar::Data => "",
                },
            ));
        }
        Shape::Object(_) => columns.push((format!("{name}_index"), "INTEGER")),
        Shape::Array(_) | Shape::Mapping(_) => {
            columns.push((format!("{name}_first_index"), "INTEGER"));
            columns.push((format!("{name}_count"), "INTEGER"));
        }
    }
    columns
}

pub(crate) fn columns() -> Vec<(String, &'static str)> {
    PROJECTIONS.iter().flat_map(Projection::columns).collect()
}

/// Immutable typed projections retained by their connection's bounded cache.
#[derive(Debug, Default)]
pub(crate) struct Receipt {
    rows: HashMap<&'static str, Vec<Vec<Value>>>,
    pub(super) carrier: Json,
    pub(super) keys: Vec<String>,
    pub(super) ids: crate::tables::Ids,
}

impl Receipt {
    pub(crate) fn add(&mut self, kind: &str, value: &Json) -> Result<i64, Failure> {
        let shape = projection(kind)?;
        let index = self.rows.entry(shape.name).or_default().len();
        self.rows.entry(shape.name).or_default().push(Vec::new());
        self.fill(shape, index, value)?;
        count(index)
    }

    fn fill(&mut self, shape: &Projection, index: usize, value: &Json) -> Result<(), Failure> {
        let mut row = vec![Value::Integer(count(index)?)];
        for field in shape.fields {
            let held = if shape.union {
                Some(value)
            } else {
                value.get(field.name)
            };
            row.push(Value::Integer(i64::from(held.is_some())));
            row.extend(self.field(field.shape, held)?);
        }
        let slot = self
            .rows
            .get_mut(shape.name)
            .and_then(|rows| rows.get_mut(index))
            .ok_or_else(|| Failure::defect("a receipt projection lost its row"))?;
        *slot = row;
        Ok(())
    }

    fn field(&mut self, shape: Shape, held: Option<&Json>) -> Result<Vec<Value>, Failure> {
        Ok(match shape {
            Shape::Scalar(kind) => {
                let mut values = Vec::new();
                if matches!(kind, Scalar::Data) {
                    values.push(held.map_or(Value::Null, data_type));
                }
                values.push(scalar(kind, held)?);
                values
            }
            Shape::Object(kind) => vec![match held.filter(|value| !value.is_null()) {
                Some(value) => Value::Integer(self.add(kind, value)?),
                None => Value::Null,
            }],
            Shape::Array(kind) | Shape::Mapping(kind) => match collection(shape, held) {
                Some(items) => vec![
                    Value::Integer(self.add_collection(kind, &items)?),
                    Value::Integer(count(items.len())?),
                ],
                None => vec![Value::Null, Value::Null],
            },
        })
    }

    fn add_collection(&mut self, kind: &str, items: &[Json]) -> Result<i64, Failure> {
        let shape = projection(kind)?;
        let rows = self.rows.entry(shape.name).or_default();
        let start = rows.len();
        rows.resize(start + items.len(), Vec::new());
        for (offset, item) in items.iter().enumerate() {
            self.fill(shape, start + offset, item)?;
        }
        count(start)
    }

    pub(crate) fn projected(&self, kind: &str, leading: usize) -> Result<Vec<Cells>, Failure> {
        let shape = projection(kind)?;
        let start = PROJECTIONS
            .iter()
            .take_while(|item| item.name != shape.name)
            .map(|item| item.columns().len())
            .sum::<usize>();
        self.rows
            .get(shape.name)
            .into_iter()
            .flatten()
            .map(|held| {
                let row = held
                    .iter()
                    .enumerate()
                    .filter(|(_, value)| !matches!(value, Value::Null))
                    .map(|(at, value)| (leading + start + at, value.clone()))
                    .collect::<Cells>();
                Ok(row)
            })
            .collect()
    }
}

fn collection(shape: Shape, held: Option<&Json>) -> Option<Vec<Json>> {
    match shape {
        Shape::Array(_) => held.and_then(Json::as_array).cloned(),
        Shape::Mapping(_) => held.and_then(Json::as_object).map(|values| {
            values
                .iter()
                .map(|(name, value)| serde_json::json!({"name":name,"value":value}))
                .collect()
        }),
        Shape::Scalar(_) | Shape::Object(_) => None,
    }
}

fn scalar(kind: Scalar, held: Option<&Json>) -> Result<Value, Failure> {
    let Some(value) = held.filter(|value| !value.is_null()) else {
        return Ok(Value::Null);
    };
    Ok(match kind {
        Scalar::Text => value
            .as_str()
            .map_or(Value::Null, |text| Value::Text(text.to_owned())),
        Scalar::Integer => value.as_i64().map_or(Value::Null, Value::Integer),
        Scalar::Unsigned => match value.as_u64() {
            Some(number) => i64::try_from(number)
                .map_or_else(|_| Value::Text(number.to_string()), Value::Integer),
            None => Value::Null,
        },
        Scalar::Real => value.as_f64().map_or(Value::Null, Value::Real),
        Scalar::Boolean => value
            .as_bool()
            .map_or(Value::Null, |value| Value::Integer(i64::from(value))),
        Scalar::Data => match value {
            Json::String(text) => Value::Text(text.clone()),
            Json::Bool(value) => Value::Integer(i64::from(*value)),
            Json::Number(number) => number
                .as_i64()
                .map(Value::Integer)
                .or_else(|| number.as_f64().map(Value::Real))
                .unwrap_or(Value::Null),
            Json::Array(_) | Json::Object(_) => Value::Text(value.to_string()),
            Json::Null => Value::Null,
        },
    })
}

fn count(number: usize) -> Result<i64, Failure> {
    i64::try_from(number)
        .map_err(|_| Failure::defect("a receipt index exceeds SQLite's integer range"))
}

fn data_type(value: &Json) -> Value {
    Value::Text(
        match value {
            Json::Null => "null",
            Json::Bool(_) => "boolean",
            Json::Number(_) => "number",
            Json::String(_) => "string",
            Json::Array(_) => "array",
            Json::Object(_) => "object",
        }
        .to_owned(),
    )
}
