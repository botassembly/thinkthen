//! One typed frame projection retains the generated native records.
use super::{
    request,
    schema::{self, Schema},
    typed::present,
};
use crate::{
    Call, CallOptions, Engine, Error, InputEvidence, RecordInput, RequestDefinition,
    RequestFunction, RequestValue,
};
use polars::prelude::{DataFrame, DataType, Field, Series};
use serde::Serialize;
use serde_json::Value;

pub(super) fn types(
    function: RequestFunction,
    definition: &RequestDefinition,
    input: &DataType,
) -> Result<(Schema, DataType), Error> {
    let judgment = match definition {
        RequestDefinition::Atomic(q) => match q.kind() {
            crate::QuestionKind::Decide => DataType::Boolean,
            crate::QuestionKind::Choose => DataType::String,
            crate::QuestionKind::Score => DataType::Float64,
            crate::QuestionKind::Tag => DataType::List(Box::new(DataType::String)),
            _ => DataType::UInt64,
        },
        _ => DataType::Boolean,
    };
    let mut schema = Schema::new(input.clone(), judgment)?;
    let value = match function {
        RequestFunction::Filter | RequestFunction::Find => input.clone(),
        RequestFunction::Rank => DataType::UInt64,
        RequestFunction::Annotate => {
            let set = match definition {
                RequestDefinition::Annotate(set)
                | RequestDefinition::DecodedSet {
                    annotation: set, ..
                } => set,
                _ => return Err(Error::usage("annotate needs a question set")),
            };
            let mut answers = Vec::new();
            let mut values = Vec::new();
            for (name, kind) in set.members() {
                let dtype = match kind {
                    crate::QuestionKind::Decide => DataType::Boolean,
                    crate::QuestionKind::Choose => DataType::String,
                    crate::QuestionKind::Score => DataType::Float64,
                    crate::QuestionKind::Tag => DataType::List(Box::new(DataType::String)),
                    _ => return Err(Error::defect("an annotation has another judgment kind")),
                };
                let member =
                    Schema::new(input.clone(), dtype.clone())?.named("completeAnnotationMember")?;
                answers.push(Field::new(name.into(), member));
                values.push(Field::new(name.into(), dtype));
            }
            let dtype = DataType::Struct(values);
            schema.override_type("completeannotatedRow", dtype.clone());
            let mut fields = match schema.named("completeAnnotation")? {
                DataType::Struct(fields) => fields,
                _ => return Err(Error::defect("an annotation schema is not a struct")),
            };
            for field in &mut fields {
                if field.name().as_str() == "answers" {
                    *field = Field::new("answers".into(), DataType::Struct(answers.clone()));
                }
            }
            schema.override_type("completeAnnotation", DataType::Struct(fields));
            dtype
        }
        _ => schema.property(schema::record_name(function), "value")?,
    };
    Ok((schema, value))
}
pub(super) fn probability_type(function: RequestFunction) -> DataType {
    match function {
        RequestFunction::Recognize | RequestFunction::Relate => {
            DataType::List(Box::new(DataType::Float64))
        }
        _ => DataType::Float64,
    }
}
pub(super) fn expression_type(
    function: RequestFunction,
    definition: &RequestDefinition,
    input: &DataType,
) -> Result<DataType, Error> {
    let (_, value) = types(function, definition, input)?;
    Ok(DataType::Struct(vec![
        Field::new("value".into(), value),
        Field::new("probability".into(), probability_type(function)),
    ]))
}
fn json(value: &impl Serialize) -> Result<Value, Error> {
    serde_json::to_value(value)
        .map_err(|_| Error::defect("a native frame result could not be projected"))
}
pub(super) fn probabilities(value: &RequestValue) -> Result<Vec<Value>, Error> {
    macro_rules! rows {
        ($rows:expr) => {
            $rows
                .iter()
                .map(|row| json(&row.result().probability()))
                .collect()
        };
    }
    match value {
        RequestValue::Decisions(rows) => rows!(rows),
        RequestValue::Choices(rows) => rows!(rows),
        RequestValue::Filtered(rows) => rows!(rows),
        RequestValue::Ranked(rows) => rows!(rows),
        RequestValue::Found(found) => Ok(vec![json(&found.probability())?]),
        RequestValue::Recognized(rows) => rows!(rows),
        RequestValue::Related(row) => Ok(vec![json(&row.result().probability())?]),
        RequestValue::Tags(rows) => Ok(vec![Value::Null; rows.len()]),
        RequestValue::Scores(rows) => Ok(vec![Value::Null; rows.len()]),
        RequestValue::Annotations(rows) => Ok(vec![Value::Null; rows.len()]),
        RequestValue::SetRanked(rows) => Ok(vec![Value::Null; rows.len()]),
    }
}
fn documents(value: &RequestValue) -> Result<Vec<Value>, Error> {
    let value = json(value)?;
    Ok(match value {
        Value::Array(rows) => rows,
        value => vec![value],
    })
}
pub(super) fn value_column(
    value: &RequestValue,
    function: RequestFunction,
    definition: &RequestDefinition,
    column: &Series,
) -> Result<Series, Error> {
    let (_, dtype) = types(function, definition, column.dtype())?;
    let probabilities = probabilities(value)?;
    let rows = documents(value)?;
    let positions = request::positions(column)?;
    let mut values = Vec::new();
    for (row, probability) in rows.iter().zip(probabilities) {
        if function == RequestFunction::Filter
            && row.get("value").and_then(Value::as_bool) != Some(true)
        {
            continue;
        }
        let value = match function {
            RequestFunction::Filter => row.get("input").cloned().unwrap_or(Value::Null),
            RequestFunction::Rank => {
                let at = row
                    .get("index")
                    .and_then(Value::as_u64)
                    .and_then(|at| usize::try_from(at).ok())
                    .and_then(|at| positions.get(at))
                    .ok_or_else(|| Error::defect("rank lost its original column position"))?;
                json(at)?
            }
            _ => row.get("value").cloned().unwrap_or(Value::Null),
        };
        values.push(serde_json::json!({"value":value,"probability":probability}));
    }
    if !matches!(
        function,
        RequestFunction::Filter
            | RequestFunction::Rank
            | RequestFunction::Find
            | RequestFunction::Relate
    ) {
        let mut source = values.into_iter();
        values = column
            .is_not_null()
            .iter()
            .map(|present| {
                if present == Some(true) {
                    source
                        .next()
                        .ok_or_else(|| Error::defect("a frame lost a present result"))
                } else {
                    Ok(Value::Null)
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        if source.next().is_some() {
            return Err(Error::defect("a frame added a result"));
        }
    }
    let dtype = DataType::Struct(vec![
        Field::new("value".into(), dtype),
        Field::new("probability".into(), probability_type(function)),
    ]);
    schema::series(column.name().as_str(), &values, &dtype)
}
pub(super) fn enrich<T>(
    call: Call<RequestValue>,
    function: RequestFunction,
    column: &Series,
    build: impl FnOnce(&RequestValue) -> Result<T, Error>,
) -> Result<Call<T, RequestValue, Series>, Error> {
    enrich_mode(call, function, column, false, build)
}
fn enrich_mode<T>(
    call: Call<RequestValue>,
    function: RequestFunction,
    column: &Series,
    all_filter: bool,
    build: impl FnOnce(&RequestValue) -> Result<T, Error>,
) -> Result<Call<T, RequestValue, Series>, Error> {
    call.project(|value| {
        let mut probabilities = probabilities(&value)?;
        if !all_filter {
            if let RequestValue::Filtered(rows) = &value {
                probabilities = rows
                    .iter()
                    .zip(probabilities)
                    .filter_map(|(row, probability)| row.result().value().then_some(probability))
                    .collect();
            }
        }

        if (all_filter && function == RequestFunction::Filter)
            || !matches!(
                function,
                RequestFunction::Filter
                    | RequestFunction::Rank
                    | RequestFunction::Find
                    | RequestFunction::Relate
            )
        {
            let mut source = probabilities.into_iter();
            probabilities = column
                .is_not_null()
                .iter()
                .map(|present| {
                    if present == Some(true) {
                        source
                            .next()
                            .ok_or_else(|| Error::defect("a frame lost a row probability"))
                    } else {
                        Ok(Value::Null)
                    }
                })
                .collect::<Result<Vec<_>, Error>>()?;
        }
        let probability =
            schema::series("probability", &probabilities, &probability_type(function))?;
        let output = build(&value)?;
        Ok((output, value, probability))
    })
}

impl Engine {
    /// Return a canonical typed records column and the unchanged native records.
    /// Originals must equal the admitted values and the supplied column type.
    /// # Errors
    /// Refuses incompatible originals, row counts or null masks before sending.
    pub fn records_column<T: InputEvidence + Serialize>(
        &self,
        function: RequestFunction,
        definition: &RequestDefinition,
        column: &Series,
        records: Vec<Option<RecordInput<T>>>,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame, RequestValue, Series>, Error> {
        let originals = records
            .iter()
            .map(|record| {
                record
                    .as_ref()
                    .map(|record| {
                        let original = json(&record.original)?;
                        if original != json(&record.original.question_input())? {
                            return Err(Error::usage(
                                "the original column differs from its admitted input",
                            ));
                        }
                        Ok(original)
                    })
                    .transpose()
                    .map(|value| value.unwrap_or(Value::Null))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let checked = schema::series(column.name().as_str(), &originals, column.dtype())
            .map_err(|_| Error::usage("the originals do not fit the supplied column type"))?;
        if !checked.equals_missing(column) {
            return Err(Error::usage(
                "the original column differs from its admitted input",
            ));
        }
        let (records, positions) = present(column, records)?;
        let (schema, _) = types(function, definition, column.dtype())?;
        let mut dtype = schema.named(schema::record_name(function))?;
        if function == RequestFunction::Relate {
            if let DataType::Struct(fields) = &mut dtype {
                for field in fields {
                    if field.name().as_str() == "input" {
                        *field = Field::new(
                            "input".into(),
                            DataType::List(Box::new(column.dtype().clone())),
                        );
                    }
                }
            }
        }
        let rows = records
            .into_iter()
            .map(|record| Ok(record.map_original(|original| original.question_input())));
        let call = request::execute_records(self, definition.clone(), function, options, rows)?;
        enrich_mode(call, function, column, true, |value| {
            let mut rows = documents(value)?;
            for row in &mut rows {
                if let Some(at) = row
                    .get("index")
                    .and_then(Value::as_u64)
                    .and_then(|at| usize::try_from(at).ok())
                {
                    row["index"] =
                        json(positions.get(at).ok_or_else(|| {
                            Error::defect("a record lost its original position")
                        })?)?;
                }
                if function == RequestFunction::Find {
                    if let Some(candidates) =
                        row.get_mut("candidates").and_then(Value::as_array_mut)
                    {
                        for candidate in candidates {
                            if let Some(at) = candidate
                                .get("index")
                                .and_then(Value::as_u64)
                                .and_then(|at| usize::try_from(at).ok())
                            {
                                candidate["index"] = json(positions.get(at).ok_or_else(|| {
                                    Error::defect("a candidate lost its original position")
                                })?)?;
                            }
                        }
                    }
                }
            }
            if !matches!(
                function,
                RequestFunction::Rank | RequestFunction::Find | RequestFunction::Relate
            ) {
                let mut source = rows.into_iter();
                rows = column
                    .is_not_null()
                    .iter()
                    .map(|present| {
                        if present == Some(true) {
                            source
                                .next()
                                .ok_or_else(|| Error::defect("a frame lost a record"))
                        } else {
                            Ok(Value::Null)
                        }
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
            }
            let records = schema::series("records", &rows, &dtype)?;
            DataFrame::new(records.len(), vec![records.into()])
                .map_err(|_| Error::defect("a canonical records frame could not be built"))
        })
    }
}
