//! Column shapes derive from the generated native schema, including its unions.
use crate::{Error, RequestDefinition, RequestFunction};
use polars::prelude::{AnyValue, DataType, Field, Series};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone)]
enum Shape {
    Raw(DataType),
    Object(BTreeMap<String, Shape>),
    List(Box<Shape>),
    Map(Box<Shape>),
    Union(BTreeMap<String, Shape>),
}
impl Shape {
    fn dtype(&self) -> DataType {
        match self {
            Self::Raw(dtype) => dtype.clone(),
            Self::Object(fields) | Self::Union(fields) => DataType::Struct(
                fields
                    .iter()
                    .map(|(name, shape)| Field::new(name.into(), shape.dtype()))
                    .collect(),
            ),
            Self::List(item) => DataType::List(Box::new(item.dtype())),
            Self::Map(item) => DataType::List(Box::new(DataType::Struct(vec![
                Field::new("key".into(), DataType::String),
                Field::new("value".into(), item.dtype()),
            ]))),
        }
    }
    fn kind(&self) -> &'static str {
        match self {
            Self::Object(_) | Self::Map(_) => "object",
            Self::List(_) => "array",
            Self::Raw(DataType::Boolean) => "boolean",
            Self::Raw(DataType::String) => "string",
            Self::Raw(DataType::Float64) => "number",
            Self::Raw(DataType::Int64 | DataType::UInt64) => "integer",
            Self::Raw(DataType::Struct(_)) => "object",
            Self::Raw(DataType::List(_)) => "array",
            _ => "null",
        }
    }
    fn cell(&self, value: &Value) -> Result<AnyValue<'static>, Error> {
        if value.is_null() {
            return Ok(AnyValue::Null);
        }
        match self {
            Self::Raw(dtype) => raw_cell(value, dtype),
            Self::Object(fields) => {
                let object = value.as_object().ok_or_else(invalid)?;
                if object.keys().any(|name| !fields.contains_key(name)) {
                    return Err(invalid());
                }
                struct_cell(fields, |name, shape| {
                    shape.cell(value.get(name).unwrap_or(&Value::Null))
                })
            }
            Self::Union(fields) => union_cell(fields, value),
            Self::List(item) => {
                let values = value.as_array().ok_or_else(invalid)?;
                Ok(AnyValue::List(shaped_series("", values, item)?))
            }
            Self::Map(item) => {
                let values = value.as_object().ok_or_else(invalid)?;
                let fields = vec![
                    Field::new("key".into(), DataType::String),
                    Field::new("value".into(), item.dtype()),
                ];
                let values = values
                    .iter()
                    .map(|(key, value)| {
                        Ok(AnyValue::StructOwned(Box::new((
                            vec![AnyValue::StringOwned(key.into()), item.cell(value)?],
                            fields.clone(),
                        ))))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(AnyValue::List(
                    Series::from_any_values_and_dtype(
                        "".into(),
                        &values,
                        &DataType::Struct(fields),
                        true,
                    )
                    .map_err(|_| invalid())?,
                ))
            }
        }
    }
}
fn union_cell(fields: &BTreeMap<String, Shape>, value: &Value) -> Result<AnyValue<'static>, Error> {
    let mut kind = json_kind(value);
    if kind == "integer" && !fields.contains_key(kind) && fields.contains_key("number") {
        kind = "number";
    }
    if !fields.contains_key(kind) {
        return Err(invalid());
    }
    struct_cell(fields, |name, shape| {
        if name == kind {
            shape.cell(value)
        } else {
            Ok(AnyValue::Null)
        }
    })
}
fn patch(
    fields: &mut BTreeMap<String, Shape>,
    authored: &Value,
    names: &[&str],
) -> Result<(), Error> {
    for (name, value) in names
        .iter()
        .filter_map(|name| authored.get(name).map(|value| (*name, value)))
    {
        fields.insert(name.to_owned(), authored_shape(value)?);
    }
    Ok(())
}
fn judgment_type(question: &Value) -> DataType {
    if question.get("choose").is_some() {
        DataType::String
    } else if question.get("score").is_some() {
        DataType::Float64
    } else if question.get("tag").is_some() {
        DataType::List(Box::new(DataType::String))
    } else {
        DataType::Boolean
    }
}
fn struct_cell(
    fields: &BTreeMap<String, Shape>,
    mut value: impl FnMut(&str, &Shape) -> Result<AnyValue<'static>, Error>,
) -> Result<AnyValue<'static>, Error> {
    let values = fields
        .iter()
        .map(|(name, shape)| value(name, shape))
        .collect::<Result<Vec<_>, Error>>()?;
    let fields = fields
        .iter()
        .map(|(name, shape)| Field::new(name.into(), shape.dtype()))
        .collect();
    Ok(AnyValue::StructOwned(Box::new((values, fields))))
}
fn json_kind(value: &Value) -> &'static str {
    match value {
        Value::Bool(_) => "boolean",
        Value::String(_) => "string",
        Value::Number(number) if number.is_i64() || number.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Null => "null",
    }
}
fn merge(left: Shape, right: Shape) -> Result<Shape, Error> {
    if left.dtype() == DataType::Null {
        return Ok(right);
    }
    if right.dtype() == DataType::Null || left.dtype() == right.dtype() {
        return Ok(left);
    }
    match (left, right) {
        (Shape::Object(mut left), Shape::Object(right)) => {
            for (name, shape) in right {
                let shape = match left.remove(&name) {
                    Some(previous) => merge(previous, shape)?,
                    None => shape,
                };
                left.insert(name, shape);
            }
            Ok(Shape::Object(left))
        }
        (Shape::List(left), Shape::List(right)) => Ok(Shape::List(Box::new(merge(*left, *right)?))),
        (Shape::Map(left), Shape::Map(right)) => Ok(Shape::Map(Box::new(merge(*left, *right)?))),
        (left, right) => {
            if left.kind() == right.kind()
                && !matches!((&left, &right), (Shape::Union(_), _) | (_, Shape::Union(_)))
            {
                return Err(Error::usage(
                    "configured values do not fit one supported typed column",
                ));
            }
            let mut fields = match left {
                Shape::Union(fields) => fields,
                shape => BTreeMap::from([(shape.kind().to_owned(), shape)]),
            };
            let right = match right {
                Shape::Union(fields) => fields,
                shape => BTreeMap::from([(shape.kind().to_owned(), shape)]),
            };
            for (name, shape) in right {
                let shape = match fields.remove(&name) {
                    Some(previous) => merge(previous, shape)?,
                    None => shape,
                };
                fields.insert(name, shape);
            }
            Ok(Shape::Union(fields))
        }
    }
}
pub(super) struct Schema {
    document: Value,
    input: DataType,
    judgment: DataType,
    overrides: BTreeMap<String, Shape>,
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
    fn named_shape(&self, name: &str) -> Result<Shape, Error> {
        if let Some(shape) = self.overrides.get(name) {
            return Ok(shape.clone());
        }
        let value = self
            .document
            .pointer(&format!("/$defs/{name}"))
            .ok_or_else(|| Error::defect("a native result schema is missing"))?;
        self.shape(value)
    }
    pub(super) fn named(&self, name: &str) -> Result<DataType, Error> {
        Ok(self.named_shape(name)?.dtype())
    }
    pub(super) fn override_type(&mut self, name: &str, dtype: DataType) {
        self.overrides.insert(name.to_owned(), Shape::Raw(dtype));
    }
    pub(super) fn property(&self, name: &str, property: &str) -> Result<DataType, Error> {
        let value = self
            .document
            .pointer(&format!("/$defs/{name}/properties/{property}"))
            .ok_or_else(|| Error::defect("a native result property is missing"))?;
        Ok(self.shape(value)?.dtype())
    }
    fn shape(&self, value: &Value) -> Result<Shape, Error> {
        if value == &Value::Bool(true) {
            return Ok(Shape::Raw(self.input.clone()));
        }
        if let Some(reference) = value.get("$ref").and_then(Value::as_str) {
            if reference == "#/$defs/completevalue" && !self.overrides.contains_key("completevalue")
            {
                return Ok(Shape::Raw(self.judgment.clone()));
            }
            return self.named_shape(reference.trim_start_matches("#/$defs/"));
        }
        if let Some(choices) = value
            .get("anyOf")
            .or_else(|| value.get("oneOf"))
            .and_then(Value::as_array)
        {
            let mut shape = Shape::Raw(DataType::Null);
            for choice in choices {
                shape = merge(shape, self.shape(choice)?)?;
            }
            if value.get("properties").is_some() {
                shape = merge(shape, self.object(value)?)?;
            }
            return Ok(shape);
        }
        if let Some(kinds) = value.get("type").and_then(Value::as_array) {
            let mut shape = Shape::Raw(DataType::Null);
            for kind in kinds.iter().filter_map(Value::as_str) {
                shape = merge(shape, self.kind(value, kind)?)?;
            }
            return Ok(shape);
        }
        let kind = value
            .get("type")
            .and_then(Value::as_str)
            .or_else(|| {
                value
                    .get("const")
                    .and_then(|constant| constant.is_string().then_some("string"))
            })
            .unwrap_or("null");
        self.kind(value, kind)
    }
    fn kind(&self, value: &Value, kind: &str) -> Result<Shape, Error> {
        Ok(match kind {
            "null" => Shape::Raw(DataType::Null),
            "boolean" => Shape::Raw(DataType::Boolean),
            "string" => Shape::Raw(DataType::String),
            "number" => Shape::Raw(DataType::Float64),
            "integer" => Shape::Raw(
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
            "array" => {
                Shape::List(Box::new(self.shape(value.get("items").ok_or_else(
                    || Error::defect("an array schema lost its item type"),
                )?)?))
            }
            "object" => return self.object(value),
            _ => return Err(Error::defect("a native schema has no Polars type")),
        })
    }
    fn object(&self, value: &Value) -> Result<Shape, Error> {
        if let Some(properties) = value.get("properties").and_then(Value::as_object) {
            return Ok(Shape::Object(
                properties
                    .iter()
                    .map(|(name, value)| Ok((name.clone(), self.shape(value)?)))
                    .collect::<Result<_, Error>>()?,
            ));
        }
        if let Some(item) = value.get("additionalProperties") {
            return Ok(Shape::Map(Box::new(self.shape(item)?)));
        }
        Ok(Shape::Object(BTreeMap::new()))
    }
    pub(super) fn definition(&mut self, definition: &RequestDefinition) -> Result<(), Error> {
        let authored = serde_json::to_value(definition)
            .map_err(|_| Error::defect("a typed question could not be projected"))?;
        self.labels(&authored)?;
        self.aggregate_question(&authored)?;
        let text = ["decide", "choose", "score", "tag", "rank"]
            .iter()
            .find_map(|key| authored.get(key));
        if let Some(text) = text {
            let Shape::Object(mut question) = self.named_shape("completequestionDocument")? else {
                return Err(invalid());
            };
            question.insert("text".to_owned(), authored_shape(text)?);
            patch(&mut question, &authored, &["true", "false"])?;
            self.overrides.insert(
                "completequestionDocument".to_owned(),
                Shape::Object(question),
            );
            let mut values = Shape::Raw(DataType::Null);
            for name in ["true", "false"] {
                values = merge(
                    values,
                    match authored.get(name) {
                        Some(value) => authored_shape(value)?,
                        None => Shape::Raw(DataType::Boolean),
                    },
                )?;
            }
            self.overrides
                .insert("completeDecideValue".to_owned(), values);
        }
        if let Some(questions) = authored.get("questions").and_then(Value::as_object) {
            let mut answers = BTreeMap::new();
            let mut questions_shape = Shape::Raw(DataType::Null);
            let mut values = BTreeMap::new();
            for (name, question) in questions {
                let definition = RequestDefinition::from_authored_json(&question.to_string())?;
                let judgment = judgment_type(question);
                let mut schema = Schema::new(self.input.clone(), judgment)?;
                schema.definition(&definition)?;
                questions_shape = merge(
                    questions_shape,
                    schema.named_shape("completequestionDocument")?,
                )?;
                answers.insert(
                    name.clone(),
                    schema.named_shape("completeAnnotationMember")?,
                );
                values.insert(name.clone(), schema.named_shape("completeannotatedField")?);
            }
            self.overrides
                .insert("completequestionDocument".to_owned(), questions_shape);
            let Shape::Object(mut annotation) = self.named_shape("completeAnnotation")? else {
                return Err(invalid());
            };
            annotation.insert("answers".to_owned(), Shape::Object(answers));
            annotation.insert("value".to_owned(), Shape::Object(values));
            self.overrides
                .insert("completeAnnotation".to_owned(), Shape::Object(annotation));
        }
        Ok(())
    }
    fn labels(&mut self, authored: &Value) -> Result<(), Error> {
        let mut description = Shape::Raw(DataType::Null);
        let fields = ["options", "labels", "levels"]
            .iter()
            .filter_map(|name| authored.get(name));
        for value in fields
            .filter_map(Value::as_object)
            .flat_map(|labels| labels.values())
        {
            description = merge(description, authored_shape(value)?)?;
        }
        if let Some(kinds) = authored
            .pointer("/recognize/kinds")
            .and_then(Value::as_object)
        {
            for value in kinds.values() {
                description = merge(description, authored_shape(value)?)?;
            }
        }
        let Shape::Object(mut label) = self.named_shape("completeLabel")? else {
            return Err(invalid());
        };
        label.insert("description".to_owned(), description);
        self.overrides
            .insert("completeLabel".to_owned(), Shape::Object(label));
        Ok(())
    }
    fn aggregate_question(&mut self, authored: &Value) -> Result<(), Error> {
        if let Some(text) = authored.get("find") {
            let Shape::Object(mut question) = self.named_shape("completefindQuestionDocument")?
            else {
                return Err(invalid());
            };
            question.insert("text".to_owned(), authored_shape(text)?);
            self.overrides.insert(
                "completefindQuestionDocument".to_owned(),
                Shape::Object(question),
            );
        }
        if let Some(recognition) = authored.get("recognize") {
            let Shape::Object(mut question) =
                self.named_shape("completerecognizeQuestionDocument")?
            else {
                return Err(invalid());
            };
            patch(
                &mut question,
                recognition,
                &["kinds", "instructions", "entity_definition"],
            )?;
            self.overrides.insert(
                "completerecognizeQuestionDocument".to_owned(),
                Shape::Object(question),
            );
        }
        Ok(())
    }
    pub(super) fn records(
        &self,
        name: &str,
        values: &[Value],
        related: bool,
    ) -> Result<Series, Error> {
        let mut shape = self.named_shape(name)?;
        if related && let Shape::Object(fields) = &mut shape {
            fields.insert(
                "input".to_owned(),
                Shape::Raw(DataType::List(Box::new(self.input.clone()))),
            );
        }
        shaped_series("records", values, &shape)
    }
}
fn authored_shape(value: &Value) -> Result<Shape, Error> {
    Ok(match value {
        Value::Null => Shape::Raw(DataType::Null),
        Value::Bool(_) => Shape::Raw(DataType::Boolean),
        Value::String(_) => Shape::Raw(DataType::String),
        Value::Number(number) => Shape::Raw(if number.is_i64() {
            DataType::Int64
        } else if number.is_u64() {
            DataType::UInt64
        } else {
            DataType::Float64
        }),
        Value::Object(fields) => Shape::Object(
            fields
                .iter()
                .map(|(name, value)| Ok((name.clone(), authored_shape(value)?)))
                .collect::<Result<_, Error>>()?,
        ),
        Value::Array(values) => {
            let mut item = Shape::Raw(DataType::Null);
            for value in values {
                item = merge(item, authored_shape(value)?)?;
            }
            Shape::List(Box::new(item))
        }
    })
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
fn invalid() -> Error {
    Error::defect("a canonical result does not match its generated column type")
}
fn shaped_series(name: &str, values: &[Value], shape: &Shape) -> Result<Series, Error> {
    let values = values
        .iter()
        .map(|value| shape.cell(value))
        .collect::<Result<Vec<_>, Error>>()?;
    Series::from_any_values_and_dtype(name.into(), &values, &shape.dtype(), true).map_err(|error| {
        Error::defect(&format!(
            "a typed result column could not be built: {error}"
        ))
    })
}
pub(super) fn series(name: &str, values: &[Value], dtype: &DataType) -> Result<Series, Error> {
    shaped_series(name, values, &Shape::Raw(dtype.clone()))
}
fn raw_cell(value: &Value, dtype: &DataType) -> Result<AnyValue<'static>, Error> {
    if value.is_null() {
        return Ok(AnyValue::Null);
    }
    Ok(match dtype {
        DataType::Boolean => AnyValue::Boolean(value.as_bool().ok_or_else(invalid)?),
        DataType::String => AnyValue::StringOwned(value.as_str().ok_or_else(invalid)?.into()),
        DataType::Float64 => AnyValue::Float64(value.as_f64().ok_or_else(invalid)?),
        DataType::UInt64 => AnyValue::UInt64(value.as_u64().ok_or_else(invalid)?),
        DataType::Int64 => AnyValue::Int64(value.as_i64().ok_or_else(invalid)?),
        DataType::Struct(fields) => {
            let object = value.as_object().ok_or_else(invalid)?;
            if object
                .keys()
                .any(|name| !fields.iter().any(|field| field.name().as_str() == name))
            {
                return Err(invalid());
            }
            AnyValue::StructOwned(Box::new((
                fields
                    .iter()
                    .map(|field| {
                        raw_cell(
                            value.get(field.name().as_str()).unwrap_or(&Value::Null),
                            field.dtype(),
                        )
                    })
                    .collect::<Result<Vec<_>, Error>>()?,
                fields.clone(),
            )))
        }
        DataType::List(inner) => {
            AnyValue::List(series("", value.as_array().ok_or_else(invalid)?, inner)?)
        }
        _ => return Err(invalid()),
    })
}
