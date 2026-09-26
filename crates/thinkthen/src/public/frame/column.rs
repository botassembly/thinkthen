//! Reading a text column in place, and writing one question's answers as a column.

use std::collections::BTreeMap;

use crate::public::{Annotated, AnnotatedRecord, Answer, Error, NamedAnnotation, QuestionKind};
use polars::prelude::{
    DataType, IntoSeries, ListBuilderTrait, ListStringChunkedBuilder, NamedFrom, Series,
};
use serde_json::value::RawValue;

/// Where a column goes. A tag column in a frame holds the JSON array text,
/// as the Python door writes it. A tag series is a `List(String)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Shape {
    Series,
    Frame,
}

/// Each row's text, borrowed from the column's own buffers across every
/// chunk and slice offset. A column that is not text, or that holds a null,
/// refuses before any request.
pub(crate) fn texts(column: &Series) -> Result<impl Iterator<Item = &str> + '_, Error> {
    let refused = || {
        Error::usage(format!(
            "the column {} is {}, not text",
            column.name(),
            column.dtype()
        ))
    };
    if column.dtype() != &DataType::String {
        return Err(refused());
    }
    let strings = column.str().map_err(|_| refused())?;
    if strings.null_count() > 0 {
        return Err(Error::usage(
            "the column holds nulls; the engine needs text, and NA rows are the caller's to drop"
                .to_owned(),
        ));
    }
    Ok(strings.iter().flatten())
}

/// The lowercase word for a question kind.
pub(crate) const fn kind_word(kind: QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Decide => "decide",
        QuestionKind::Choose => "choose",
        QuestionKind::Tag => "tag",
        QuestionKind::Score => "score",
        QuestionKind::Rank => "rank",
        QuestionKind::Find => "find",
    }
}

/// One member's answers over every record, as a column of the member's
/// kind. When any row failed, the whole column widens to `String`.
pub(crate) fn answered(
    name: &str,
    kind: QuestionKind,
    shape: Shape,
    records: &[AnnotatedRecord<&str>],
    place: usize,
) -> Result<Series, Error> {
    let values = records
        .iter()
        .map(|record| {
            record
                .values()
                .get(place)
                .map(NamedAnnotation::value)
                .ok_or_else(|| Error::defect(&format!("a record has no member {name}")))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if values
        .iter()
        .any(|value| matches!(value, Annotated::Failed(_)))
    {
        let cells = records
            .iter()
            .zip(&values)
            .map(|(record, value)| widened(name, record, value))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(Series::new(name.into(), cells));
    }
    let mismatch = || Error::defect(&format!("the member {name} answered another kind"));
    let series = match (kind, shape) {
        (QuestionKind::Decide, _) => {
            let cells = values.iter().map(|value| match value {
                Annotated::Decision(Answer::Yes) => Ok(Some(true)),
                Annotated::Decision(Answer::No) => Ok(Some(false)),
                Annotated::Decision(Answer::Unsure) => Ok(None),
                _ => Err(mismatch()),
            });
            Series::new(name.into(), cells.collect::<Result<Vec<_>, _>>()?)
        }
        (QuestionKind::Choose, _) => {
            let cells = values.iter().map(|value| match value {
                Annotated::Choice(label) => Ok(label.as_deref()),
                _ => Err(mismatch()),
            });
            Series::new(name.into(), cells.collect::<Result<Vec<_>, _>>()?)
        }
        (QuestionKind::Score, _) => {
            let cells = values.iter().map(|value| match value {
                Annotated::Score(position) => Ok(*position),
                _ => Err(mismatch()),
            });
            Series::new(name.into(), cells.collect::<Result<Vec<_>, _>>()?)
        }
        (QuestionKind::Tag, Shape::Series) => {
            let mut lists = ListStringChunkedBuilder::new(name.into(), values.len(), values.len());
            for value in &values {
                let Annotated::Tags(labels) = value else {
                    return Err(mismatch());
                };
                lists.append_values_iter(labels.iter().map(String::as_str));
            }
            lists.finish().into_series()
        }
        (QuestionKind::Tag, Shape::Frame) => {
            let cells = records
                .iter()
                .map(|record| member(name, record).map(|raw| raw.get().to_owned()))
                .collect::<Result<Vec<_>, _>>()?;
            Series::new(name.into(), cells)
        }
        (QuestionKind::Rank | QuestionKind::Find, _) => return Err(mismatch()),
    };
    Ok(series)
}

/// One cell of a widened column: the member's text from `value_json`,
/// unchanged, except that a choice holds its plain label and a not-sure or
/// nothing-fits answer stays null.
fn widened(
    name: &str,
    record: &AnnotatedRecord<&str>,
    value: &Annotated,
) -> Result<Option<String>, Error> {
    if let Annotated::Choice(label) = value {
        return Ok(label.clone());
    }
    let raw = member(name, record)?;
    Ok((raw.get() != "null").then(|| raw.get().to_owned()))
}

/// The member's raw JSON text in the record's `value_json`.
fn member(name: &str, record: &AnnotatedRecord<&str>) -> Result<Box<RawValue>, Error> {
    let json = record.value_json();
    let mut members: BTreeMap<String, Box<RawValue>> = serde_json::from_str(&json)
        .map_err(|error| Error::defect(&format!("the record's JSON did not parse: {error}")))?;
    members
        .remove(name)
        .ok_or_else(|| Error::defect(&format!("the record's JSON has no member {name}")))
}
