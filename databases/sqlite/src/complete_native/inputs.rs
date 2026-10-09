//! Explicit SQL descriptors retain native record order and never infer paths/images.
use super::{defect, usage};
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;
use thinkthen::{
    Error, ImageEvidence, ImageInput, ImageMedia, InputEvidence, InputReaderOptions, ObjectContext,
    QuestionInput, RawRecord, RecordContext, RecordInput, RecordOption, RecordOptions,
    RecordReading, SourceLocation,
};

type Fields = BTreeMap<String, Box<RawValue>>;
pub(super) type Records<'a> =
    Box<dyn Iterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a>;
pub(crate) struct Inputs {
    raw: Fields,
    native_request: bool,
    pub(crate) jsonl: bool,
    pub(crate) incremental: bool,
    pub(crate) attempts: bool,
    pub(crate) cancelled: bool,
}
impl Inputs {
    /// Inspect descriptors only; native route admission precedes file/image readers.
    #[allow(
        dead_code,
        reason = "only SQLite uses shared Request image-route admission before ticket 0500"
    )]
    pub(crate) fn image_inputs(&self) -> Result<bool, Error> {
        if let Some(files) = self.raw.get("files") {
            let files = fields(files.get())?;
            let options = files
                .get("options")
                .map(|value| {
                    serde_json::from_str::<InputReaderOptions>(value.get())
                        .map_err(|_| usage("invalid native reader options"))
                })
                .transpose()?
                .unwrap_or_default();
            return Ok(options.media == thinkthen::ReaderMedia::Image);
        }
        let records: Vec<Box<RawValue>> =
            serde_json::from_str(self.raw.get("records").ok_or_else(defect)?.get())
                .map_err(|_| usage("records is an ordered array"))?;
        for record in records {
            let Ok(record) = fields(record.get()) else {
                continue;
            };
            let Some(images) = record.get("images") else {
                continue;
            };
            let Ok(images) = serde_json::from_str::<Vec<Box<RawValue>>>(images.get()) else {
                continue;
            };
            if !images.is_empty() {
                return Ok(true);
            }
        }
        Ok(false)
    }
    #[allow(
        dead_code,
        reason = "only SQLite adopts native request controls before PostgreSQL ticket 0500"
    )]
    pub(crate) fn parse_request(source: &str) -> Result<Self, Error> {
        let mut inputs = Self::parse(source, true)?;
        inputs.native_request = true;
        Ok(inputs)
    }
    /// Advance host readers only after shared request admission and cancellation checks.
    #[allow(
        dead_code,
        reason = "only SQLite defers readers through Request before PostgreSQL ticket 0500"
    )]
    pub(crate) fn deferred_records(&self, reading: Option<&RecordReading>) -> Records<'_> {
        let reading = reading.cloned();
        Box::new(
            std::iter::once_with(move || self.records(reading.as_ref())).flat_map(|result| {
                match result {
                    Ok(records) => records,
                    Err(error) => Box::new(std::iter::once(Err(error))),
                }
            }),
        )
    }
    pub(crate) fn parse(source: &str, server_files: bool) -> Result<Self, Error> {
        let raw = fields(source)?;
        if raw.keys().any(|key| {
            !matches!(
                key.as_str(),
                "records" | "reading" | "files" | "incremental" | "attempts" | "cancelled"
            )
        }) {
            return Err(usage("unknown complete input field"));
        }
        let mut jsonl = false;
        if let Some(files) = raw.get("files") {
            let files = fields(files.get())?;
            if files
                .keys()
                .any(|k| !matches!(k.as_str(), "paths" | "options" | "format"))
            {
                return Err(usage("unknown complete files field"));
            }
            jsonl = super::file_format::jsonl(files.get("format").map(|value| value.get()))?;
        }
        if raw.contains_key("records") == raw.contains_key("files") {
            return Err(usage(
                "complete inputs takes exactly one of records or files",
            ));
        }
        if !server_files && raw.contains_key("files") {
            return Err(usage(
                "PostgreSQL evidence and images require client-read records",
            ));
        }
        let flag = |key| {
            raw.get(key)
                .map(|v| {
                    serde_json::from_str::<bool>(v.get())
                        .map_err(|_| usage("complete flags are booleans"))
                })
                .transpose()
        };
        let incremental = flag("incremental")?.unwrap_or(false);
        let attempts = flag("attempts")?.unwrap_or(false);
        let cancelled = flag("cancelled")?.unwrap_or(false);
        Ok(Self {
            raw,
            native_request: false,
            jsonl,
            incremental,
            attempts,
            cancelled,
        })
    }
    pub(crate) fn records(&self, reading: Option<&RecordReading>) -> Result<Records<'_>, Error> {
        let reading = if let Some(value) = self.raw.get("reading") {
            let given: Value =
                serde_json::from_str(value.get()).map_err(|_| usage("reading is one object"))?;
            let object = given
                .as_object()
                .ok_or_else(|| usage("reading is one object"))?;
            if object.keys().any(|k| {
                !(matches!(
                    k.as_str(),
                    "fields" | "context" | "options" | "context_schema"
                ) || self.native_request && matches!(k.as_str(), "examples" | "seed_spans"))
            }) {
                return Err(usage("unknown reading field"));
            }
            let fields = given
                .get("fields")
                .map(|fields| {
                    serde_json::from_value::<Vec<String>>(fields.clone())
                        .map_err(|_| usage("reading fields is a text array"))
                })
                .transpose()?
                .unwrap_or_default();
            let fields = fields.iter().map(String::as_str).collect::<Vec<_>>();
            let pointer = |name| {
                given
                    .get(name)
                    .map(|v| v.as_str().ok_or_else(|| usage("reading pointers are text")))
                    .transpose()
            };
            let mut reading =
                RecordReading::new(&fields, pointer("context")?, pointer("options")?)?;
            if let Some(pointer) = pointer("examples")? {
                reading = reading.with_examples_field(pointer)?;
            }
            if let Some(pointer) = pointer("seed_spans")? {
                reading = reading.with_seed_spans_field(pointer)?;
            }
            let raw_reading = super::inputs::fields(value.get())?;
            if let Some(schema) = raw_reading.get("context_schema") {
                reading.with_context_schema(context_schema(schema.get())?)
            } else {
                reading
            }
        } else {
            reading
                .cloned()
                .unwrap_or(RecordReading::new(&[], None, None)?)
        };
        if let Some(files) = self.raw.get("files") {
            let value = fields(files.get())?;
            let paths: Vec<String> = serde_json::from_str(
                value
                    .get("paths")
                    .ok_or_else(|| usage("files requires paths"))?
                    .get(),
            )
            .map_err(|_| usage("file paths is a text array"))?;
            let options = value
                .get("options")
                .map(|v| {
                    serde_json::from_str::<InputReaderOptions>(v.get())
                        .map_err(|_| usage("invalid native reader options"))
                })
                .transpose()?
                .unwrap_or_default();
            let records = thinkthen::read_inputs(paths, options)?;
            return Ok(Box::new(records.map(move |item| {
                item.and_then(|item| compose_file(item, &reading, self.jsonl))
            })));
        }
        let records: Vec<Box<RawValue>> =
            serde_json::from_str(self.raw.get("records").ok_or_else(defect)?.get())
                .map_err(|_| usage("records is an ordered array"))?;
        Ok(Box::new(records.into_iter().map(move |raw| {
            compose(raw.get(), &reading, self.native_request)
        })))
    }
}

fn compose(
    raw: &str,
    reading: &RecordReading,
    native_request: bool,
) -> Result<RecordInput<QuestionInput>, Error> {
    let fields = fields(raw)?;
    if let Some(error) = fields.get("read_error") {
        if fields.len() != 1 {
            return Err(usage("a terminal reader error has no record fields"));
        }
        return Err(super::reader_error::decode(error.get())?);
    }
    if fields.keys().any(|k| {
        !(matches!(
            k.as_str(),
            "text"
                | "document"
                | "json_text"
                | "json"
                | "context"
                | "options"
                | "images"
                | "source"
        ) || native_request && matches!(k.as_str(), "examples" | "seed_spans"))
    }) || ["text", "json", "document", "json_text"]
        .iter()
        .filter(|key| fields.contains_key(**key))
        .count()
        > 1
    {
        return Err(usage("invalid complete record descriptor"));
    }
    let images = fields
        .get("images")
        .map(|v| images(v.get()))
        .transpose()?
        .unwrap_or_default();
    let original = if let Some(json) = fields.get("json_text") {
        let text =
            serde_json::from_str::<String>(json.get()).map_err(|_| usage("JSON text is text"))?;
        Some(RawRecord::json(&text)?)
    } else if let Some(document) = fields.get("document") {
        let text = serde_json::from_str::<String>(document.get())
            .map_err(|_| usage("document is text"))?;
        Some(RawRecord::json(&text).or_else(|_| RawRecord::text(&text))?)
    } else {
        match (fields.get("text"), fields.get("json")) {
            (Some(text), None) => Some(RawRecord::text(
                &serde_json::from_str::<String>(text.get())
                    .map_err(|_| usage("record text is literal text"))?,
            )?),
            (None, Some(json)) => Some(RawRecord::json(json.get())?),
            (None, None) => None,
            _ => return Err(usage("one original per record")),
        }
    };
    let composed = match original {
        Some(original) => {
            let record = reading.compose(original)?;
            let evidence = if !fields.contains_key("images") {
                record.original
            } else {
                record.original.with_images(images)?
            };
            RecordInput {
                seed_spans: record.seed_spans,
                examples: record.examples,
                original: evidence.question_input(),
                context: record.context,
                options: record.options,
            }
        }
        None => RecordInput {
            seed_spans: None,
            examples: None,
            original: QuestionInput::Images(ImageEvidence::new(None, images)?),
            context: None,
            options: None,
        },
    };
    supplements(composed, &fields)
}
fn supplements(
    mut composed: RecordInput<QuestionInput>,
    fields: &Fields,
) -> Result<RecordInput<QuestionInput>, Error> {
    if let Some(source) = fields.get("source") {
        composed.original = located(composed.original, source)?;
    }
    if let Some(value) = fields.get("context") {
        composed.context = Some(context(value)?);
    }
    if let Some(value) = fields.get("options") {
        composed.options = Some(options(value)?);
    }
    if let Some(value) = fields.get("seed_spans") {
        composed.seed_spans = Some(
            serde_json::from_str(value.get())
                .map_err(|_| usage("seed spans is an ordered array"))?,
        );
    }
    if let Some(value) = fields.get("examples") {
        composed.examples = Some(
            serde_json::from_str(value.get()).map_err(|_| usage("examples is an ordered array"))?,
        );
    }
    Ok(composed)
}
fn images(source: &str) -> Result<Vec<ImageInput>, Error> {
    let images: Vec<Value> =
        serde_json::from_str(source).map_err(|_| usage("images is an explicit ordered array"))?;
    images
        .into_iter()
        .map(|value| {
            let media = match value.get("media").and_then(Value::as_str) {
                Some("image/png") => ImageMedia::Png,
                Some("image/jpeg") => ImageMedia::Jpeg,
                _ => return Err(usage("image media is image/png or image/jpeg")),
            };
            let bytes = serde_json::from_value::<Vec<u8>>(
                value
                    .get("bytes")
                    .cloned()
                    .ok_or_else(|| usage("image requires compressed bytes"))?,
            )
            .map_err(|_| usage("image bytes is an integer array"))?;
            ImageInput::new(media, bytes)
        })
        .collect()
}

fn fields(source: &str) -> Result<Fields, Error> {
    // Native ordered JSON parsing detects duplicates without imposing the
    // per-record byte limit on a container of many bounded records/images.
    match thinkthen::Settings::parse(&format!("{{\"sql_descriptor\":{source}}}")) {
        Ok(_) | Err(thinkthen::SettingsError::UnknownKey(_)) => {}
        Err(error) => return Err(usage(&error.to_string())),
    }
    serde_json::from_str(source).map_err(|_| usage("descriptor is one object with unique fields"))
}

fn compose_file(
    item: thinkthen::SourceItem,
    reading: &RecordReading,
    jsonl: bool,
) -> Result<RecordInput<QuestionInput>, Error> {
    if !jsonl {
        return reading.compose_source(item);
    }
    let thinkthen::SourceItem::Text(source) = item else {
        return Err(usage("jsonl requires text media"));
    };
    let record = reading.compose(RawRecord::json(&source.record)?)?;
    Ok(RecordInput {
        seed_spans: record.seed_spans,
        examples: record.examples,
        original: record
            .original
            .with_location(SourceLocation::new(
                source.file,
                Some(source.first_line),
                Some(source.last_line),
            )?)
            .question_input(),
        context: record.context,
        options: record.options,
    })
}
fn located(original: QuestionInput, source: &RawValue) -> Result<QuestionInput, Error> {
    let value: Value =
        serde_json::from_str(source.get()).map_err(|_| usage("source is one object"))?;
    let file = value
        .get("file")
        .and_then(Value::as_str)
        .ok_or_else(|| usage("source requires a file identity"))?
        .to_owned();
    let coordinate = |name| {
        value
            .get(name)
            .map(|v| {
                v.as_u64()
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or_else(|| usage("source line is a positive integer"))
            })
            .transpose()
    };
    let location = SourceLocation::new(file, coordinate("first_line")?, coordinate("last_line")?)?;
    Ok(match original {
        QuestionInput::Record(record) => QuestionInput::Record(record.with_location(location)),
        QuestionInput::Images(images)
            if location.first_line().is_none() && images.images().len() == 1 =>
        {
            thinkthen::ImageSourceRecord {
                record: images.images().first().ok_or_else(defect)?.clone(),
                file: location.file().to_owned(),
            }
            .question_input()
        }
        _ => return Err(usage("image sources have no text line coordinates")),
    })
}
fn context(context: &RawValue) -> Result<RecordContext, Error> {
    let value: Value =
        serde_json::from_str(context.get()).map_err(|_| usage("invalid per-record context"))?;
    Ok(match value {
        Value::String(text) => RecordContext::Text(text),
        Value::Object(_) => {
            RecordContext::Object(ObjectContext::new(&RawRecord::json(context.get())?)?)
        }
        _ => RecordContext::Object(ObjectContext::new(&RawRecord::json(context.get())?)?),
    })
}
fn options(options: &RawValue) -> Result<RecordOptions, Error> {
    let values: Vec<Value> =
        serde_json::from_str(options.get()).map_err(|_| usage("options is an ordered array"))?;
    let options = values
        .into_iter()
        .map(|v| {
            let (name, description) = if let Some(name) = v.as_str() {
                (name.to_owned(), None)
            } else {
                (
                    v.get("name")
                        .and_then(Value::as_str)
                        .ok_or_else(|| usage("option requires a name"))?
                        .to_owned(),
                    v.get("description")
                        .map(|v| thinkthen::Description::from_json(&v.to_string()))
                        .transpose()?,
                )
            };
            Ok(RecordOption { name, description })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    RecordOptions::new(options)
}

fn context_schema(source: &str) -> Result<thinkthen::InputDeclaration, Error> {
    let question = thinkthen::Question::from_json(&format!(
        "{{\"decide\":\"Context declaration\",\"context_schema\":{source}}}"
    ))?;
    let thinkthen::LoadedQuestion::Question(question) = question else {
        return Err(defect());
    };
    question.context_schema().cloned().ok_or_else(defect)
}
