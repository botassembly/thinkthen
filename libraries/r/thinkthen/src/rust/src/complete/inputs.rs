//! Originals and ancillary inputs cross the native record composer once.
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use serde_json::value::RawValue;
use thinkthen::{
    Engine, Error, ImageInput, ImageMedia, InputEvidence, InputReaderOptions, QuestionInput,
    RawRecord, RecordContext, RecordInput, RecordReading, RequestItem, SourceItem,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Input {
    kind: InputKind,
    #[serde(default, deserialize_with = "present")]
    records: Option<Option<Vec<Item>>>,
    #[serde(default, deserialize_with = "present")]
    paths: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "present")]
    options: Option<Option<InputReaderOptions>>,
    #[serde(default, deserialize_with = "present")]
    jsonl: Option<Option<bool>>,
}
fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    reader: D,
) -> Result<Option<Option<T>>, D::Error> {
    Option::<T>::deserialize(reader).map(Some)
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum InputKind {
    Records,
    Files,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Item {
    content: Content,
    #[serde(default)]
    images: Vec<Image>,
    context: Option<Context>,
    options: Option<Vec<Choice>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Content {
    kind: ContentKind,
    #[serde(default = "null")]
    value: Box<RawValue>,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContentKind {
    Text,
    Json,
    Images,
}
fn null() -> Box<RawValue> {
    RawValue::NULL.to_owned()
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Image {
    media: ImageMedia,
    bytes: Vec<u8>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum Context {
    Text(String),
    Json(Box<RawValue>),
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Choice {
    name: String,
    #[serde(
        default,
        deserialize_with = "description",
        skip_serializing_if = "Option::is_none"
    )]
    description: Option<Box<RawValue>>,
}

fn description<'de, D: serde::Deserializer<'de>>(
    reader: D,
) -> Result<Option<Box<RawValue>>, D::Error> {
    Box::<RawValue>::deserialize(reader).map(Some)
}

pub(crate) struct Original {
    pub(super) value: Box<RawValue>,
    pub(super) input: QuestionInput,
}
impl InputEvidence for Original {
    fn question_input(&self) -> QuestionInput {
        self.input.clone()
    }
}
impl Serialize for Original {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}
fn compose(
    item: Item,
    reading: &RecordReading,
    annotation: bool,
) -> Result<RecordInput<Original>, Error> {
    // The host envelope retains its accepted spelling and Serde field checks.
    // Request owns descriptor admission, projection and attachment composition.
    let mut descriptor = std::collections::BTreeMap::new();
    let (value, annotation_text) = match item.content.kind {
        ContentKind::Text => {
            let text: String = serde_json::from_str(item.content.value.get())
                .map_err(|_| super::usage("text requires a string"))?;
            let value =
                serde_json::value::to_raw_value(&text).map_err(|_| super::usage("invalid text"))?;
            descriptor.insert("text", value.clone());
            let annotation_text = (annotation && item.images.is_empty()).then_some(text);
            (value, annotation_text)
        }
        ContentKind::Json => {
            let value = item.content.value;
            // Transport objects must not consume the original's nesting budget.
            descriptor.insert(
                "json_text",
                serde_json::value::to_raw_value(value.get())
                    .map_err(|_| super::usage("invalid JSON text"))?,
            );
            (value, None)
        }
        ContentKind::Images if !item.images.is_empty() => (null(), None),
        _ => {
            return Err(super::usage(
                "give text, JSON or explicit images for each item",
            ));
        }
    };
    // Legacy empty attachments mean absence; SQL descriptors treat an explicit
    // empty images array as an image-only refusal.
    if !item.images.is_empty() {
        descriptor.insert(
            "images",
            serde_json::value::to_raw_value(&item.images)
                .map_err(|_| super::usage("invalid images"))?,
        );
    }
    let descriptor = serde_json::to_string(&descriptor)
        .map_err(|_| super::usage("invalid record descriptor"))?;
    let request = RequestItem::from_record_descriptor(&descriptor)?;
    let mut record = request.compose_record(reading)?;
    if let Some(text) = annotation_text {
        record.original = QuestionInput::annotation_document(&text)?;
    }
    // The host's tagged JSON context denotes an object even when its value is
    // a string. The shared untagged descriptor cannot express that distinction.
    record.context = match item.context {
        None => record.context,
        Some(Context::Text(text)) => Some(RecordContext::Text(text)),
        Some(Context::Json(value)) => Some(RecordContext::Object(thinkthen::ObjectContext::new(
            &RawRecord::json(value.get())?,
        )?)),
    };
    if let Some(choices) = item.options {
        let source =
            serde_json::to_string(&choices).map_err(|_| super::usage("invalid options"))?;
        record.options = request.with_options_descriptor(&source)?.options;
    }
    Ok(RecordInput {
        seed_spans: record.seed_spans,
        examples: record.examples,
        original: Original {
            value,
            input: record.original,
        },
        context: record.context,
        options: record.options,
    })
}

pub(crate) type Rows<'a> = Box<dyn Iterator<Item = Result<RecordInput<Original>, Error>> + 'a>;
pub(crate) fn iter<'a>(
    engine: &'a Engine,
    input: Input,
    reading: Option<&RecordReading>,
    annotation: bool,
) -> Result<Rows<'a>, Error> {
    prepare(Some(engine), input, reading, annotation)
}
// Frame batches prepare their actual column before native pulling owns admission.
pub(crate) fn prepare<'a>(
    engine: Option<&'a Engine>,
    input: Input,
    reading: Option<&RecordReading>,
    annotation: bool,
) -> Result<Rows<'a>, Error> {
    match input.kind {
        InputKind::Records
            if input.paths.is_some() || input.options.is_some() || input.jsonl.is_some() =>
        {
            return Err(super::usage("records cannot include file source fields"));
        }
        InputKind::Files if input.records.is_some() => {
            return Err(super::usage("files cannot include records"));
        }
        _ => {}
    }
    let reading = reading
        .cloned()
        .unwrap_or(RecordReading::new(&[], None, None)?);
    Ok(match input.kind {
        InputKind::Records => {
            let records = match input.records {
                None => Vec::new(),
                Some(Some(records)) => records,
                Some(None) => return Err(super::usage("records require a list")),
            };
            Box::new(records.into_iter().enumerate().map(move |(at, item)| {
                if let Some(engine) = engine {
                    engine.check_record_limit(at)?;
                }
                compose(item, &reading, annotation)
            }))
        }
        InputKind::Files => {
            let paths = match input.paths {
                None => Vec::new(),
                Some(Some(paths)) => paths,
                Some(None) => return Err(super::usage("files require paths")),
            };
            let jsonl = match input.jsonl {
                None => false,
                Some(Some(jsonl)) => jsonl,
                Some(None) => return Err(super::usage("jsonl requires true or false")),
            };
            let sources = thinkthen::read_inputs(
                paths,
                input
                    .options
                    .flatten()
                    .ok_or_else(|| super::usage("files require reader options"))?,
            )?;
            Box::new(sources.enumerate().map(move |(at, item)| {
                if let Some(engine) = engine {
                    engine.check_record_limit(at)?;
                }
                source(item?, &reading, jsonl, annotation)
            }))
        }
    })
}
fn source(
    item: SourceItem,
    reading: &RecordReading,
    jsonl: bool,
    annotation: bool,
) -> Result<RecordInput<Original>, Error> {
    let value = match &item {
        SourceItem::Text(s) => serde_json::value::to_raw_value(&s.record),
        SourceItem::Image(_) => serde_json::value::to_raw_value(&Value::Null),
    }
    .map_err(|_| super::usage("invalid source value"))?;
    if let SourceItem::Text(s) = &item
        && annotation
        && !jsonl
    {
        let location =
            thinkthen::SourceLocation::new(s.file.clone(), Some(s.first_line), Some(s.last_line))?;
        return Ok(RecordInput {
            seed_spans: None,
            examples: None,
            original: Original {
                value,
                input: QuestionInput::annotation_text(&s.record, location)?,
            },
            context: None,
            options: None,
        });
    }
    let record = match item {
        SourceItem::Text(s) if jsonl => {
            let mut r = reading.compose(RawRecord::json(&s.record)?)?;
            r.original = r.original.with_location(thinkthen::SourceLocation::new(
                s.file,
                Some(s.first_line),
                Some(s.last_line),
            )?);
            let value = serde_json::value::to_raw_value(r.original.original())
                .map_err(|_| super::usage("invalid original"))?;
            return Ok(RecordInput {
                seed_spans: None,
                examples: None,
                original: Original {
                    value,
                    input: r.original.question_input(),
                },
                context: r.context,
                options: r.options,
            });
        }
        source => reading.compose_source(source)?,
    };
    Ok(RecordInput {
        seed_spans: None,
        examples: None,
        original: Original {
            value,
            input: record.original,
        },
        context: record.context,
        options: record.options,
    })
}
pub(crate) fn read(
    engine: &Engine,
    input: Input,
    reading: Option<&RecordReading>,
    annotation: bool,
) -> Result<Vec<RecordInput<Original>>, Error> {
    iter(engine, input, reading, annotation)?.collect()
}

#[derive(Serialize)]
pub(crate) struct InputView {
    original: Box<RawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<thinkthen::SourceLocation>,
    images: Vec<ImageInput>,
}
impl Original {
    pub(crate) fn view(&self) -> InputView {
        let (location, images) = match &self.input {
            QuestionInput::Text(_) => (None, Vec::new()),
            QuestionInput::Record(record) => (record.location().cloned(), record.images().to_vec()),
            QuestionInput::Images(images) => (images.location().cloned(), images.images().to_vec()),
        };
        InputView {
            original: self.value.clone(),
            location,
            images,
        }
    }
}
