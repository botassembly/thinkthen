//! Reading a text column in place, and writing one question's answers as a column.

use crate::public::{Annotated, AnnotatedRecord, Answer, Error, NamedAnnotation, QuestionKind};
use polars::prelude::{
    BooleanChunked, DataType, IdxCa, IdxSize, IntoSeries, ListBuilderTrait,
    ListStringChunkedBuilder, NamedFrom, NewChunkedArray, Series, StructChunked,
};

/// Keep positions of null input rows, which require no backend question.
pub(crate) fn nullable(column: &Series) -> Result<Vec<Option<&str>>, Error> {
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
    Ok(strings.iter().collect())
}

/// Reinsert null positions without asking for those rows or changing the
/// returned Series' native Boolean, String, Float64, List or Struct type.
pub(crate) fn restore(series: Series, cells: &[Option<&str>]) -> Result<Series, Error> {
    let mut next = 0usize;
    let mut places = Vec::with_capacity(cells.len());
    for cell in cells {
        places.push(if cell.is_some() {
            let place =
                IdxSize::try_from(next).map_err(|_| Error::usage("the frame has too many rows"))?;
            next += 1;
            Some(place)
        } else {
            None
        });
    }
    if next != series.len() {
        return Err(Error::defect("a frame answer lost a non-null input row"));
    }
    let indices = IdxCa::from_iter_options("row".into(), places.into_iter());
    series
        .take(&indices)
        .map_err(|error| Error::defect(&format!("the frame could not restore null rows: {error}")))
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

/// One member's answers over every record, as a column of the member's kind.
pub(crate) fn answered(
    name: &str,
    kind: QuestionKind,
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
    let mismatch = || Error::defect(&format!("the member {name} answered another kind"));
    let series = match kind {
        QuestionKind::Decide => {
            let cells = values.iter().map(|value| match value {
                Annotated::Decision(Answer::Yes) => Ok(Some(true)),
                Annotated::Decision(Answer::No) => Ok(Some(false)),
                Annotated::Decision(Answer::Unsure) => Ok(None),
                Annotated::Failed(_) => Ok(None),
                _ => Err(mismatch()),
            });
            Series::new(name.into(), cells.collect::<Result<Vec<_>, _>>()?)
        }
        QuestionKind::Choose => {
            let cells = values.iter().map(|value| match value {
                Annotated::Choice(label) => Ok(label.as_deref()),
                Annotated::Failed(_) => Ok(None),
                _ => Err(mismatch()),
            });
            Series::new(name.into(), cells.collect::<Result<Vec<_>, _>>()?)
        }
        QuestionKind::Score => {
            let cells = values.iter().map(|value| match value {
                Annotated::Score(position) => Ok(Some(*position)),
                Annotated::Failed(_) => Ok(None),
                _ => Err(mismatch()),
            });
            Series::new(name.into(), cells.collect::<Result<Vec<_>, _>>()?)
        }
        QuestionKind::Tag => {
            let mut lists = ListStringChunkedBuilder::new(name.into(), values.len(), values.len());
            for value in &values {
                match value {
                    Annotated::Tags(labels) => {
                        lists.append_values_iter(labels.iter().map(String::as_str))
                    }
                    Annotated::Failed(_) => lists.append_null(),
                    _ => return Err(mismatch()),
                }
            }
            lists.finish().into_series()
        }
        QuestionKind::Rank | QuestionKind::Find => return Err(mismatch()),
    };
    Ok(series)
}

/// A fixed-field failure map. Explicit parent validity distinguishes a row
/// with no failures from a row whose other question fields are null.
pub(crate) fn failed(names: &[&str], records: &[AnnotatedRecord<&str>]) -> Result<Series, Error> {
    let mut fields = Vec::with_capacity(names.len());
    let mut any = vec![false; records.len()];
    for (place, name) in names.iter().enumerate() {
        let mut kinds = Vec::with_capacity(records.len());
        let mut causes = Vec::with_capacity(records.len());
        let mut present = Vec::with_capacity(records.len());
        for (any, record) in any.iter_mut().zip(records) {
            let value = record
                .values()
                .get(place)
                .ok_or_else(|| Error::defect(&format!("a record has no member {name}")))?;
            if matches!(value.value(), Annotated::Failed(_)) {
                let (kind, cause) = marker(record, name)?;
                kinds.push(Some(kind));
                causes.push(Some(cause));
                present.push(true);
                *any = true;
            } else {
                kinds.push(None);
                causes.push(None);
                present.push(false);
            }
        }
        let marker = struct_with_validity(
            "failed",
            &[
                Series::new("kind".into(), kinds),
                Series::new("cause".into(), causes),
            ],
            &present,
        )?;
        fields.push(struct_with_validity(name, &[marker], &present)?);
    }
    struct_with_validity("failed", &fields, &any)
}

fn marker(record: &AnnotatedRecord<&str>, name: &str) -> Result<(String, String), Error> {
    let json: serde_json::Value = serde_json::from_str(&record.value_json())
        .map_err(|error| Error::defect(&format!("the record's JSON did not parse: {error}")))?;
    let failed = json
        .get(name)
        .and_then(|value| value.get("failed"))
        .ok_or_else(|| Error::defect(&format!("the record has no failure marker for {name}")))?;
    let field = |key| {
        failed
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| Error::defect(&format!("the failure marker has no {key}")))
    };
    Ok((field("kind")?, field("cause")?))
}

fn struct_with_validity(name: &str, fields: &[Series], present: &[bool]) -> Result<Series, Error> {
    let mask = BooleanChunked::from_slice("valid".into(), present);
    let bitmap = mask
        .downcast_iter()
        .next()
        .map(|array| array.values().clone());
    StructChunked::from_series(name.into(), present.len(), fields.iter())
        .map(|column| column.with_outer_validity(bitmap).into_series())
        .map_err(|error| Error::defect(&format!("the failed column could not be built: {error}")))
}
