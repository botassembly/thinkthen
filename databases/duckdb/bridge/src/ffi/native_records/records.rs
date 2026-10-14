//! Native originals and ancillary fields retain canonical composition and source metadata.
use crate::complete_native;
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;
use thinkthen::{Error, InputEvidence, RequestOriginal};

type Fields = BTreeMap<String, Box<RawValue>>;

#[derive(Debug)]
pub(super) struct Record {
    pub(super) item: thinkthen::RequestItem,
    pub(super) location: Option<thinkthen::SourceLocation>,
}

pub(super) fn fields(raw: &str) -> Result<Fields, Error> {
    match thinkthen::Settings::parse(&format!("{{\"sql_record\":{raw}}}")) {
        Ok(_) | Err(thinkthen::SettingsError::UnknownKey(_)) => {}
        Err(error) => return Err(complete_native::usage(&error.to_string())),
    }
    serde_json::from_str(raw).map_err(|_| complete_native::usage("native SQL record is one object"))
}

pub(super) fn read_record(raw: &str) -> Result<Record, Error> {
    let mut given = fields(raw)?;
    let json_present = given
        .remove("json_present")
        .map(|value| value.get() == "true")
        .unwrap_or(true);
    given.retain(|key, value| value.get() != "null" || key == "json" && json_present);
    if given.contains_key("read_error") {
        let descriptor = serde_json::to_string(&given).map_err(|_| complete_native::defect())?;
        let input =
            complete_native::Inputs::parse_request(&format!("{{\"records\":[{descriptor}]}}"))?;
        let mut records = input.records(None)?;
        return match records.next() {
            Some(Err(error)) => Err(error),
            _ => Err(complete_native::defect()),
        };
    }
    let source = given.remove("source");
    let mut originals = Vec::new();
    for name in ["text", "json", "document"] {
        if let Some(value) = given.remove(name) {
            let original = match name {
                "json" => format!("{{\"kind\":\"json\",\"value\":{}}}", value.get()),
                "text" => format!("{{\"kind\":\"text\",\"text\":{}}}", value.get()),
                _ => {
                    let document: String = serde_json::from_str(value.get())
                        .map_err(|_| complete_native::usage("document is text"))?;
                    let value = thinkthen::RawRecord::json(&document)
                        .or_else(|_| thinkthen::RawRecord::text(&document))?;
                    serde_json::to_string(&match value.literal() {
                        Some(text) => RequestOriginal::Text {
                            text: text.to_owned(),
                        },
                        None => RequestOriginal::Json { value },
                    })
                    .map_err(|_| complete_native::defect())?
                }
            };
            originals.push(original);
        }
    }
    if originals.len() > 1 {
        return Err(complete_native::usage("one original per record"));
    }
    if let Some(images) = given.get_mut("images") {
        *images = image_descriptors(images.get())?;
    }
    let mut descriptor = String::from("{");
    if let Some(original) = originals.first() {
        descriptor.push_str("\"original\":");
        descriptor.push_str(original);
    }
    for (name, value) in given {
        if descriptor.len() > 1 {
            descriptor.push(',');
        }
        descriptor.push_str(&serde_json::to_string(&name).map_err(|_| complete_native::defect())?);
        descriptor.push(':');
        descriptor.push_str(value.get());
    }
    descriptor.push('}');
    let item = serde_json::from_str(&descriptor).map_err(|_| {
        complete_native::usage("native record fields, options, context or images are invalid")
    })?;
    let location = source
        .map(|value| source_location(value.get()))
        .transpose()?;
    Ok(Record { item, location })
}

fn image_descriptors(raw: &str) -> Result<Box<RawValue>, Error> {
    let values: Vec<Value> = serde_json::from_str(raw)
        .map_err(|_| complete_native::usage("images is an ordered native collection"))?;
    let images = values
        .into_iter()
        .map(|image| {
            let media = image
                .get("media")
                .and_then(Value::as_str)
                .ok_or_else(|| complete_native::usage("image requires a media type"))?;
            let bytes = image
                .get("data")
                .and_then(Value::as_str)
                .ok_or_else(|| complete_native::usage("images require native BLOB content"))?;
            Ok(serde_json::json!({"kind":"bytes","media":media,"bytes":bytes}))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    RawValue::from_string(serde_json::to_string(&images).map_err(|_| complete_native::defect())?)
        .map_err(|_| complete_native::defect())
}

fn coordinate(fields: &mut Fields, name: &str) -> Result<Option<usize>, Error> {
    fields
        .remove(name)
        .filter(|value| value.get() != "null")
        .map(|value| {
            serde_json::from_str(value.get())
                .map_err(|_| complete_native::usage("source coordinate is a positive whole number"))
        })
        .transpose()
}

fn source_location(raw: &str) -> Result<thinkthen::SourceLocation, Error> {
    let mut fields = fields(raw)?;
    let file = fields
        .remove("file")
        .ok_or_else(|| complete_native::usage("source requires a file identity"))?;
    let file = serde_json::from_str(file.get())
        .map_err(|_| complete_native::usage("source file is text"))?;
    let location = thinkthen::SourceLocation::new(
        file,
        coordinate(&mut fields, "first_line")?,
        coordinate(&mut fields, "last_line")?,
    )?;
    if !fields.is_empty() {
        return Err(complete_native::usage("unknown source field"));
    }
    Ok(location)
}

pub(super) fn compose(
    record: Record,
    reading: &thinkthen::RecordReading,
) -> Result<thinkthen::RecordInput<thinkthen::QuestionInput>, Error> {
    let mut row = record.item.compose_record(reading)?;
    if let Some(location) = record.location {
        row.original = match row.original {
            thinkthen::QuestionInput::Record(record) => {
                thinkthen::QuestionInput::Record(record.with_location(location))
            }
            thinkthen::QuestionInput::Images(images)
                if location.first_line().is_none() && images.images().len() == 1 =>
            {
                thinkthen::ImageSourceRecord {
                    record: images
                        .images()
                        .first()
                        .cloned()
                        .ok_or_else(complete_native::defect)?,
                    file: location.file().to_owned(),
                }
                .question_input()
            }
            _ => {
                return Err(complete_native::usage(
                    "source coordinates require a text record",
                ));
            }
        };
    }
    Ok(row)
}
