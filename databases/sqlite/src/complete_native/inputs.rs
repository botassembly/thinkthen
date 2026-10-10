//! Explicit SQL descriptors retain native record order and never infer paths/images.
use super::{defect, usage};
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;
use thinkthen::{
    Error, InputEvidence, InputReaderOptions, QuestionInput, RecordInput, RecordReading,
    SourceLocation,
};

type Fields = BTreeMap<String, Box<RawValue>>;
pub(super) type Records<'a> =
    Box<dyn Iterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a>;
pub(crate) struct Inputs {
    raw: Fields,
    native_record: Option<thinkthen::RequestItem>,
    native_request: bool,
    pub(crate) framing: Option<thinkthen::RequestFraming>,
    pub(crate) incremental: bool,
    pub(crate) attempts: bool,
    pub(crate) cancelled: bool,
}
impl Inputs {
    /// Carry one native host record without a JSON descriptor or byte array.
    #[allow(dead_code, reason = "native BLOB complete inputs enter through SQLite")]
    pub(crate) fn from_record(record: thinkthen::RequestItem) -> Self {
        Self {
            raw: Fields::new(),
            native_record: Some(record),
            native_request: true,
            framing: None,
            incremental: false,
            attempts: false,
            cancelled: false,
        }
    }
    /// Inspect descriptors only; native route admission precedes file/image readers.
    #[allow(
        dead_code,
        reason = "only SQLite uses shared Request image-route admission before ticket 0500"
    )]
    pub(crate) fn image_inputs(&self) -> Result<bool, Error> {
        if let Some(record) = &self.native_record {
            return Ok(!record.images.is_empty());
        }
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
        let mut framing = None;
        if let Some(files) = raw.get("files") {
            let files = fields(files.get())?;
            if files
                .keys()
                .any(|k| !matches!(k.as_str(), "paths" | "options" | "format"))
            {
                return Err(usage("unknown complete files field"));
            }
            framing = super::file_format::framing(files.get("format").map(|value| value.get()))?;
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
            native_record: None,
            native_request: false,
            framing,
            incremental,
            attempts,
            cancelled,
        })
    }
    pub(crate) fn records(&self, reading: Option<&RecordReading>) -> Result<Records<'_>, Error> {
        if let Some(record) = &self.native_record {
            let default = RecordReading::new(&[], None, None)?;
            return Ok(Box::new(std::iter::once(
                record.compose_record(reading.unwrap_or(&default)),
            )));
        }
        let reading = self.record_reading(reading)?;
        if let Some(files) = self.raw.get("files") {
            return self.file_records(files.get(), reading);
        }
        let records: Vec<Box<RawValue>> =
            serde_json::from_str(self.raw.get("records").ok_or_else(defect)?.get())
                .map_err(|_| usage("records is an ordered array"))?;
        Ok(Box::new(records.into_iter().map(move |raw| {
            compose(raw.get(), &reading, self.native_request)
        })))
    }
    /// Parse SQL reading through the native validators before advancing readers.
    pub(crate) fn record_reading(
        &self,
        reading: Option<&RecordReading>,
    ) -> Result<RecordReading, Error> {
        Ok(if let Some(value) = self.raw.get("reading") {
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
        })
    }
    fn file_records(&self, files: &str, reading: RecordReading) -> Result<Records<'_>, Error> {
        let value = fields(files)?;
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
        if let Some(framing) = self.framing {
            let source = thinkthen::RequestSource {
                paths: paths.into_iter().map(Into::into).collect(),
                reading: options.reading,
                media: options.media,
                framing: Some(framing),
            };
            let records = source.read_framed(Default::default())?;
            return Ok(Box::new(records.map(move |item| {
                item.and_then(|source| compose_framed(source, &reading))
            })));
        }
        let records = thinkthen::read_inputs(paths, options)?;
        Ok(Box::new(records.map(move |item| {
            item.and_then(|item| reading.compose_source(item))
        })))
    }
}

fn compose_framed(
    source: thinkthen::SourceRecord<thinkthen::RawRecord>,
    reading: &RecordReading,
) -> Result<RecordInput<QuestionInput>, Error> {
    let mut record = reading.compose(source.record)?;
    record.original = record.original.with_location(SourceLocation::new(
        source.file,
        Some(source.first_line),
        Some(source.last_line),
    )?);
    Ok(record.map_original(|original| original.question_input()))
}

fn compose(
    raw: &str,
    reading: &RecordReading,
    native_request: bool,
) -> Result<RecordInput<QuestionInput>, Error> {
    let mut fields = fields(raw)?;
    if let Some(error) = fields.get("read_error") {
        if fields.len() != 1 {
            return Err(usage("a terminal reader error has no record fields"));
        }
        return Err(super::reader_error::decode(error.get())?);
    }
    if !native_request && (fields.contains_key("examples") || fields.contains_key("seed_spans")) {
        return Err(usage("invalid complete record descriptor"));
    }
    let source = fields.remove("source");
    let descriptor = serde_json::to_string(&fields).map_err(|_| defect())?;
    let item = thinkthen::RequestItem::from_record_descriptor(&descriptor)?;
    let mut composed = item.compose_record(reading)?;
    if let Some(source) = source {
        composed.original = located(composed.original, &source)?;
    }
    Ok(composed)
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
fn context_schema(source: &str) -> Result<thinkthen::InputDeclaration, Error> {
    let question = thinkthen::Question::from_json(&format!(
        "{{\"decide\":\"Context declaration\",\"context_schema\":{source}}}"
    ))?;
    let thinkthen::LoadedQuestion::Question(question) = question else {
        return Err(defect());
    };
    question.context_schema().cloned().ok_or_else(defect)
}
