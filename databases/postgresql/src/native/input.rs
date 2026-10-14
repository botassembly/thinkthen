//! SQL NULL and authored JSON null remain different originals.
use super::field;
use crate::call;
use pgrx::prelude::*;
use thinkthen::{Error, RequestItem, RequestOriginal};

extension_sql!(
    r#"
CREATE TYPE thinkthen_option AS (name text, description json);
CREATE TYPE thinkthen_seed AS (start bigint, end bigint, kind text);
CREATE TYPE thinkthen_source AS (file text, first_line bigint, last_line bigint);
CREATE TYPE thinkthen_reader_failure AS (kind text, message text);
CREATE TYPE thinkthen_record AS (
    text text, value json, context text, options thinkthen_option[],
    seed_spans thinkthen_seed[], examples text[], images thinkthen_image_value[],
    source thinkthen_source, read_error thinkthen_reader_failure
);
"#,
    name = "native_record_types",
    requires = ["image_value_type"]
);

pub(super) fn seeds(
    values: Option<Array<'_, pgrx::composite_type!("thinkthen_seed")>>,
) -> Result<Option<Vec<thinkthen::RecognitionSeedSpan>>, Error> {
    values
        .map(|values| {
            values
                .iter()
                .map(|tuple| {
                    let tuple = tuple.ok_or_else(|| call::usage("seed list contains NULL"))?;
                    Ok(thinkthen::RecognitionSeedSpan {
                        start: super::number(
                            field::<i64>(&tuple, "start")?
                                .ok_or_else(|| call::usage("seed start is NULL"))?,
                        )?,
                        end: super::number(
                            field::<i64>(&tuple, "end")?
                                .ok_or_else(|| call::usage("seed end is NULL"))?,
                        )?,
                        kind: field(&tuple, "kind")?,
                    })
                })
                .collect()
        })
        .transpose()
}

#[derive(Debug)]
pub(super) struct Record {
    pub(super) item: RequestItem,
    pub(super) source: Option<thinkthen::SourceLocation>,
}

pub(super) fn deferred(
    records: Array<'_, pgrx::composite_type!("thinkthen_record")>,
) -> Vec<Result<Record, Error>> {
    records.iter().map(record).collect()
}
pub(super) fn records(
    records: Array<'_, pgrx::composite_type!("thinkthen_record")>,
) -> Result<Vec<RequestItem>, Error> {
    deferred(records)
        .into_iter()
        .map(|record| record.map(|record| record.item))
        .collect()
}
fn record(tuple: Option<pgrx::composite_type!("thinkthen_record")>) -> Result<Record, Error> {
    let tuple = tuple.ok_or_else(|| call::usage("record list contains NULL"))?;
    let failure: Option<pgrx::composite_type!("thinkthen_reader_failure")> =
        field(&tuple, "read_error")?;
    if let Some(failure) = failure {
        let kind: String =
            field(&failure, "kind")?.ok_or_else(|| call::usage("reader failure kind is NULL"))?;
        let kind = match kind.as_str() {
            "local" => thinkthen::ErrorKind::Local,
            "usage" => thinkthen::ErrorKind::Usage,
            _ => return Err(call::usage("reader failures are local or usage errors")),
        };
        let message: String = field(&failure, "message")?
            .ok_or_else(|| call::usage("reader failure message is NULL"))?;
        return Err(thinkthen::Error::new(kind, message));
    }
    let text: Option<String> = field(&tuple, "text")?;
    let value: Option<pgrx::datum::Json> = field(&tuple, "value")?;
    let original = match (text, value) {
        (Some(text), None) => Some(RequestOriginal::Text { text }),
        (None, Some(value)) => Some(RequestOriginal::Json {
            value: thinkthen::RawRecord::json(&value.0.to_string())?,
        }),
        (None, None) => None,
        _ => return Err(call::usage("a record has either text or JSON, never both")),
    };
    let options: Option<Array<'_, pgrx::composite_type!("thinkthen_option")>> =
        field(&tuple, "options")?;
    let options = options
        .map(|options| {
            options
                .iter()
                .map(|tuple| {
                    let tuple = tuple.ok_or_else(|| call::usage("option list contains NULL"))?;
                    let description: Option<pgrx::datum::Json> = field(&tuple, "description")?;
                    Ok(thinkthen::RecordOption {
                        name: field(&tuple, "name")?
                            .ok_or_else(|| call::usage("option name is NULL"))?,
                        description: description.map(read_description).transpose()?,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()
                .and_then(thinkthen::RecordOptions::new)
        })
        .transpose()?;
    let images: Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>> =
        field(&tuple, "images")?;
    let images = images
        .map(|images| crate::images::input(images, None).map(|item| item.images))
        .transpose()?
        .unwrap_or_default();
    let examples: Option<Array<'_, &str>> = field(&tuple, "examples")?;
    let item = RequestItem {
        original,
        context: field::<String>(&tuple, "context")?.map(thinkthen::RecordContext::Text),
        options,
        seed_spans: seeds(field(&tuple, "seed_spans")?)?,
        examples: examples
            .map(|values| {
                values
                    .iter()
                    .map(|text| {
                        text.map(brackets)
                            .ok_or_else(|| call::usage("example list contains NULL"))
                    })
                    .collect()
            })
            .transpose()?,
        images,
    };
    let source: Option<pgrx::composite_type!("thinkthen_source")> = field(&tuple, "source")?;
    let source = source
        .map(|source| {
            thinkthen::SourceLocation::new(
                field::<String>(&source, "file")?
                    .ok_or_else(|| call::usage("source file is NULL"))?,
                field::<i64>(&source, "first_line")?
                    .map(super::number)
                    .transpose()?,
                field::<i64>(&source, "last_line")?
                    .map(super::number)
                    .transpose()?,
            )
        })
        .transpose()?;
    Ok(Record { item, source })
}

fn read_description(value: pgrx::datum::Json) -> Result<thinkthen::Description, Error> {
    thinkthen::Description::from_json(&value.0.to_string())
}
fn brackets(text: &str) -> thinkthen::RecognitionExample {
    thinkthen::RecognitionExample::Brackets(text.to_owned())
}

pub(super) fn compose(
    record: Record,
    reading: &thinkthen::RecordReading,
) -> Result<thinkthen::RecordInput<thinkthen::QuestionInput>, Error> {
    let mut row = record.item.compose_record(reading)?;
    if let Some(location) = record.source {
        row.original = match row.original {
            thinkthen::QuestionInput::Record(record) => {
                thinkthen::QuestionInput::Record(record.with_location(location))
            }
            thinkthen::QuestionInput::Images(images)
                if location.first_line().is_none() && images.images().len() == 1 =>
            {
                use thinkthen::InputEvidence as _;
                thinkthen::ImageSourceRecord {
                    record: images
                        .images()
                        .first()
                        .ok_or_else(|| call::defect("image source has no image"))?
                        .clone(),
                    file: location.file().to_owned(),
                }
                .question_input()
            }
            _ => return Err(call::usage("image sources have no text line coordinates")),
        };
    }
    Ok(row)
}
