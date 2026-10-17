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
    schema.definition(definition)?;
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
            let mut values = Vec::new();
            for (name, kind) in set.members() {
                let dtype = match kind {
                    crate::QuestionKind::Decide => DataType::Boolean,
                    crate::QuestionKind::Choose => DataType::String,
                    crate::QuestionKind::Score => DataType::Float64,
                    crate::QuestionKind::Tag => DataType::List(Box::new(DataType::String)),
                    _ => return Err(Error::defect("an annotation has another judgment kind")),
                };
                values.push(Field::new(name.into(), dtype));
            }
            let dtype = DataType::Struct(values);
            schema.override_type("completeannotatedRow", dtype.clone());
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
        RequestValue::SetRanked(rows) => rows!(rows),
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
    let mut rows = documents(value)?;
    if let RequestValue::Annotations(annotations) = value {
        for (document, annotation) in rows.iter_mut().zip(annotations) {
            document["value"] = annotation_values(annotation.result())?;
        }
    }
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
    if rowwise(function) {
        values = restore(column, values)?;
    }
    let dtype = DataType::Struct(vec![
        Field::new("value".into(), dtype),
        Field::new("probability".into(), probability_type(function)),
    ]);
    schema::series(column.name().as_str(), &values, &dtype)
}
fn annotation_values(annotation: &crate::CompleteAnnotated) -> Result<Value, Error> {
    let mut members = serde_json::Map::new();
    for member in annotation.members() {
        let value = match member.value() {
            Some(crate::Judgment::Decision(value)) => json(&super::column::decided(value))?,
            Some(crate::Judgment::Choice(value)) => json(&value)?,
            Some(crate::Judgment::Score(value)) => json(&value)?,
            Some(crate::Judgment::Tags(value)) => json(&value)?,
            None => Value::Null,
        };
        members.insert(member.name().to_owned(), value);
    }
    Ok(Value::Object(members))
}
fn restore(column: &Series, rows: Vec<Value>) -> Result<Vec<Value>, Error> {
    let mut source = rows.into_iter();
    let mut output = Vec::with_capacity(column.len());
    for present in column.is_not_null().iter() {
        output.push(if present == Some(true) {
            source
                .next()
                .ok_or_else(|| Error::defect("a frame lost a present result"))?
        } else {
            Value::Null
        });
    }
    if source.next().is_some() {
        return Err(Error::defect("a frame added a result"));
    }
    Ok(output)
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
        if !all_filter && let RequestValue::Filtered(rows) = &value {
            probabilities = rows
                .iter()
                .zip(probabilities)
                .filter_map(|(row, probability)| row.result().value().then_some(probability))
                .collect();
        }
        if (all_filter && function == RequestFunction::Filter) || rowwise(function) {
            probabilities = restore(column, probabilities)?;
        }
        let probability =
            schema::series("probability", &probabilities, &probability_type(function))?;
        let output = build(&value)?;
        Ok((output, value, probability))
    })
}
fn rowwise(function: RequestFunction) -> bool {
    !matches!(
        function,
        RequestFunction::Filter
            | RequestFunction::Rank
            | RequestFunction::Find
            | RequestFunction::Relate
    )
}
fn original_rows<T: InputEvidence + Serialize>(
    column: &Series,
    records: Vec<Option<RecordInput<T>>>,
) -> Result<Vec<Option<RecordInput<crate::QuestionInput>>>, Error> {
    let mut originals = Vec::with_capacity(records.len());
    let mut rows = Vec::with_capacity(records.len());
    for record in records {
        let Some(record) = record else {
            originals.push(Value::Null);
            rows.push(None);
            continue;
        };
        let original = json(&record.original)?;
        let input = record.original.question_input();
        if original != json(&input)? {
            return Err(Error::usage(
                "the original column differs from its admitted input",
            ));
        }
        originals.push(original);
        rows.push(Some(record.map_original(|_| input)));
    }
    let checked = schema::series(column.name().as_str(), &originals, column.dtype())
        .map_err(|_| Error::usage("the originals do not fit the supplied column type"))?;
    if !checked.equals_missing(column) {
        return Err(Error::usage(
            "the original column differs from its admitted input",
        ));
    }
    Ok(rows)
}
fn map_index(row: &mut Value, positions: &[usize]) -> Result<(), Error> {
    let Some(at) = row
        .get("index")
        .and_then(Value::as_u64)
        .and_then(|at| usize::try_from(at).ok())
    else {
        return Ok(());
    };
    row["index"] = json(
        positions
            .get(at)
            .ok_or_else(|| Error::defect("a record lost its original position"))?,
    )?;
    Ok(())
}
fn records_frame(
    value: &RequestValue,
    function: RequestFunction,
    column: &Series,
    positions: &[usize],
    schema: &Schema,
) -> Result<DataFrame, Error> {
    let mut rows = documents(value)?;
    for row in &mut rows {
        map_index(row, positions)?;
        if function == RequestFunction::Find
            && let Some(candidates) = row.get_mut("candidates").and_then(Value::as_array_mut)
        {
            for candidate in candidates {
                map_index(candidate, positions)?;
            }
        }
    }
    if function == RequestFunction::Filter || rowwise(function) {
        rows = restore(column, rows)?;
    }
    let records = schema.records(
        schema::record_name(function),
        &rows,
        function == RequestFunction::Relate,
    )?;
    DataFrame::new(records.len(), vec![records.into()])
        .map_err(|_| Error::defect("a canonical records frame could not be built"))
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
        let records = original_rows(column, records)?;
        let (records, positions) = present(column, records)?;
        let (schema, _) = types(function, definition, column.dtype())?;
        schema.named(schema::record_name(function))?;
        let rows = records.into_iter().map(Ok);
        let call = request::execute_records(self, definition.clone(), function, options, rows)?;
        enrich_mode(call, function, column, true, |value| {
            records_frame(value, function, column, &positions, &schema)
        })
    }
}
