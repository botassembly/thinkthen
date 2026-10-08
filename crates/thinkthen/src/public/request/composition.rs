//! One typed composition path resolves sources only after complete header admission.
use super::{
    AdmittedRequest, RequestDefinition, RequestEnvironment, RequestImage, RequestInput,
    RequestItem, RequestOriginal,
};
use crate::{
    CallOptions, Error, ImageInput, InputEvidence, QuestionInput, RawRecord, RecordInput,
    RecordReading,
};
pub(super) type Inputs<'a> =
    Box<dyn Iterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a>;

impl AdmittedRequest {
    pub(super) fn records<'a>(
        &'a self,
        definition: &RequestDefinition,
        environment: RequestEnvironment<'a>,
        controls: CallOptions<'a>,
    ) -> Result<Inputs<'a>, Error> {
        let options = &self.request.call.arguments().options;
        let fields = options
            .field
            .iter()
            .flatten()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let explicit = options.field.is_some()
            || options.context_field.is_some()
            || options.options_field.is_some();
        let reading = if explicit {
            RecordReading::new(
                &fields,
                options.context_field.as_deref(),
                options.options_field.as_deref(),
            )?
        } else {
            authored_reading(definition)?
        };
        let context = context_schema(definition).cloned();
        let reading = context.clone().map_or(reading.clone(), |schema| {
            reading.with_context_schema(schema)
        });
        match &self.request.call.arguments().input {
            RequestInput::Records { items }
            | RequestInput::Units { items }
            | RequestInput::Entities { items } => {
                let rows = items
                    .iter()
                    .map(|item| {
                        controls.admission()?;
                        compose_item(item, &reading, context.as_ref())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Box::new(rows.into_iter().map(Ok)))
            }
            RequestInput::Text { text, images } => {
                let item = RequestItem {
                    original: Some(RequestOriginal::Text { text: text.clone() }),
                    context: None,
                    options: None,
                    images: images.clone(),
                };
                let row = compose_item(&item, &reading, context.as_ref())?;
                Ok(Box::new(std::iter::once(Ok(row))))
            }
            RequestInput::Json { value, images } => {
                let item = RequestItem {
                    original: Some(RequestOriginal::Json {
                        value: value.clone(),
                    }),
                    context: None,
                    options: None,
                    images: images.clone(),
                };
                let row = compose_item(&item, &reading, context.as_ref())?;
                Ok(Box::new(std::iter::once(Ok(row))))
            }
            RequestInput::Source { source } => {
                controls.admission()?;
                let items = crate::read_inputs(
                    &source.paths,
                    crate::InputReaderOptions {
                        reading: source.reading,
                        media: source.media,
                    },
                )?;
                let annotate = matches!(definition, RequestDefinition::Annotate(_)) && !explicit;
                Ok(Box::new(items.map(move |item| {
                    controls.admission()?;
                    let item = item?;
                    if annotate && let crate::SourceItem::Text(text) = item {
                        return Ok(RecordInput {
                            original: QuestionInput::annotation_text(
                                &text.record,
                                crate::SourceLocation::new(
                                    text.file,
                                    Some(text.first_line),
                                    Some(text.last_line),
                                )?,
                            )?,
                            context: None,
                            options: None,
                        });
                    }
                    reading.compose_source(item)
                })))
            }
            RequestInput::Feed { name, images, .. } => {
                let feed = environment
                    .feed
                    .ok_or_else(|| Error::usage("the request requires a caller-supplied feed"))?;
                if feed.name != *name {
                    return Err(Error::usage(
                        "the supplied feed does not match the requested name",
                    ));
                }
                Ok(Box::new(feed.items.map(move |item| {
                    controls.admission()?;
                    let mut item = item?;
                    if !images.is_empty() {
                        if !item.images.is_empty() {
                            return Err(Error::usage(
                                "feed item images conflict with shared attachments",
                            ));
                        }
                        item.images = images.clone();
                    }
                    compose_item(&item, &reading, context.as_ref())
                })))
            }
        }
    }
}
fn compose_item(
    item: &RequestItem,
    reading: &RecordReading,
    schema: Option<&crate::InputDeclaration>,
) -> Result<RecordInput<QuestionInput>, Error> {
    let images = item
        .images
        .iter()
        .map(read_image)
        .collect::<Result<Vec<_>, _>>()?;
    let mut row = match &item.original {
        Some(RequestOriginal::Text { text }) => compose_original(reading, RawRecord::text(text)?)?,
        Some(RequestOriginal::Json { value }) => compose_original(reading, value.clone())?,
        None => {
            reading.admit_images()?;
            RecordInput {
                original: QuestionInput::Images(crate::ImageEvidence::new(None, images.clone())?),
                context: None,
                options: None,
            }
        }
    };
    if !images.is_empty()
        && let QuestionInput::Record(record) = row.original
    {
        row.original = QuestionInput::Record(record.with_images(images)?);
    }
    if let Some(context) = &item.context {
        context.validate(schema)?;
        row.context = Some(context.clone());
    }
    if let Some(options) = &item.options {
        row.options = Some(options.clone());
    }
    Ok(row)
}
fn compose_original(
    reading: &RecordReading,
    original: RawRecord,
) -> Result<RecordInput<QuestionInput>, Error> {
    let row = reading.compose(original)?;
    Ok(RecordInput {
        original: row.original.question_input(),
        context: row.context,
        options: row.options,
    })
}
fn read_image(image: &RequestImage) -> Result<ImageInput, Error> {
    match image {
        RequestImage::Bytes { media, bytes } => ImageInput::new(*media, bytes.clone()),
        RequestImage::File { path, media } => {
            let options = crate::InputReaderOptions {
                reading: crate::ReaderOptions {
                    unit: crate::SourceUnit::File,
                    window: None,
                },
                media: crate::ReaderMedia::Image,
            };
            let mut items = crate::read_inputs([path], options)?;
            let Some(crate::SourceItem::Image(image)) = items.next().transpose()? else {
                return Err(Error::usage("an attachment requires one image file"));
            };
            if items.next().transpose()?.is_some() {
                return Err(Error::usage("an attachment requires one image file"));
            }
            match media {
                Some(media) => ImageInput::new(*media, image.record.bytes()),
                None => Ok(image.record),
            }
        }
    }
}
fn authored_reading(definition: &RequestDefinition) -> Result<RecordReading, Error> {
    match definition {
        RequestDefinition::Find(file) => Ok(file.reading().clone()),
        RequestDefinition::Recognize(file) => Ok(file.reading().clone()),
        RequestDefinition::Atomic(q) => match q {
            crate::LoadedQuestion::Question(q) => question_reading(q),
            crate::LoadedQuestion::Banded(q) => question_reading(&q.0),
        },
        RequestDefinition::Rank(q) => question_reading(q),
        _ => RecordReading::new(&[], None, None),
    }
}
fn question_reading(q: &crate::Question) -> Result<RecordReading, Error> {
    RecordReading::new(
        &q.metadata
            .reading
            .on
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        None,
        None,
    )
}
pub(super) fn context_schema(definition: &RequestDefinition) -> Option<&crate::InputDeclaration> {
    match definition {
        RequestDefinition::Atomic(q) => q.context_schema(),
        RequestDefinition::Rank(q) => q.context_schema(),
        RequestDefinition::DynamicChoose(q) => q.context_schema(),
        RequestDefinition::Find(q) => q.question().context_schema(),
        RequestDefinition::Recognize(q) => q.question().context_schema(),
        RequestDefinition::Recognition(q) => q.context_schema(),
        RequestDefinition::Relate(q) => q.context_schema(),
        RequestDefinition::Annotate(set) => set
            .0
            .questions()
            .iter()
            .find_map(|q| q.metadata().context_schema.as_ref()),
        RequestDefinition::RankSet(set) => set
            .0
            .questions()
            .iter()
            .find_map(|q| q.metadata().context_schema.as_ref()),
        RequestDefinition::DecodedSet { .. } => None,
    }
}
