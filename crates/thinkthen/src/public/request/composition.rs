//! One typed composition path resolves sources only after complete header admission.
use super::transport::AttachmentBudget;
#[cfg(feature = "cli")]
use super::transport::TransportDescriptor;
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
    /// Obtain typed projection for a caller composing native records.
    /// This reads only admitted inline preparation and never opens a saved selector.
    /// # Errors
    /// Refuses unresolved saved selectors; projection validation retains Usage errors.
    pub fn record_reading(&self) -> Result<RecordReading, Error> {
        let definition = self.definition.as_ref().ok_or_else(|| {
            Error::usage("native record reading requires an admitted inline definition")
        })?;
        reading(definition, &self.request.call.arguments().options)
    }
    pub(super) fn records<'a>(
        &'a self,
        definition: &RequestDefinition,
        environment: RequestEnvironment<'a>,
        controls: CallOptions<'a>,
        image_refusal: Option<String>,
    ) -> Result<Inputs<'a>, Error> {
        let options = &self.request.call.arguments().options;
        let mut budget = AttachmentBudget::new(self.attachment_limit);
        let explicit = options.field.is_some()
            || options.context_field.is_some()
            || options.options_field.is_some()
            || options.examples_field.is_some()
            || options.seed_spans_field.is_some();
        let reading = reading(definition, options)?;
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
                        compose_item(item, &reading, context.as_ref(), &mut budget)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Box::new(rows.into_iter().map(Ok)))
            }
            RequestInput::Text { text, images } => singleton(
                RequestOriginal::Text { text: text.clone() },
                images,
                &reading,
                context.as_ref(),
                &mut budget,
            ),
            RequestInput::Json { value, images } => singleton(
                RequestOriginal::Json {
                    value: value.clone(),
                },
                images,
                &reading,
                context.as_ref(),
                &mut budget,
            ),
            RequestInput::Source { source } => {
                controls.admission()?;
                let items = read_source(source, budget.remaining())?;
                let annotate = matches!(definition, RequestDefinition::Annotate(_)) && !explicit;
                let rank = self.request.call.function() == super::RequestFunction::Rank;
                Ok(super::transport::source_records(
                    reading, items, controls, annotate, rank,
                ))
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
                #[cfg(feature = "cli")]
                if let super::execution::FeedContents::Descriptors(descriptors) = feed.contents {
                    return compose_descriptors(
                        &descriptors,
                        &reading,
                        context.as_ref(),
                        &mut budget,
                        controls,
                    );
                }
                let super::execution::FeedContents::Items(items) = feed.contents else {
                    return super::native_feed::records(
                        self,
                        feed.contents,
                        controls,
                        image_refusal,
                    );
                };
                Ok(Box::new(items.map(move |item| {
                    controls.admission()?;
                    let item = attach_shared(item?, images)?;
                    super::admission::admit_item(self.request.call.function(), &item, options)?;
                    admit_image_route(&item, image_refusal.as_deref())?;
                    compose_item(&item, &reading, context.as_ref(), &mut budget)
                })))
            }
        }
    }
}
fn singleton(
    original: RequestOriginal,
    images: &[RequestImage],
    reading: &RecordReading,
    schema: Option<&crate::InputDeclaration>,
    budget: &mut AttachmentBudget,
) -> Result<Inputs<'static>, Error> {
    if !images.is_empty()
        && let RequestOriginal::Text { text } = &original
    {
        reading.admit_images()?;
        let images = images
            .iter()
            .map(|image| read_image(image, budget))
            .collect::<Result<Vec<_>, _>>()?;
        let row = RecordInput {
            original: QuestionInput::Images(crate::ImageEvidence::new(Some(text.clone()), images)?),
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
        };
        return Ok(Box::new(std::iter::once(Ok(row))));
    }
    let item = RequestItem {
        original: Some(original),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: images.to_vec(),
    };
    let row = compose_item(&item, reading, schema, budget)?;
    Ok(Box::new(std::iter::once(Ok(row))))
}
pub(super) fn compose_item(
    item: &RequestItem,
    reading: &RecordReading,
    schema: Option<&crate::InputDeclaration>,
    budget: &mut AttachmentBudget,
) -> Result<RecordInput<QuestionInput>, Error> {
    let images = item
        .images
        .iter()
        .map(|image| read_image(image, budget))
        .collect::<Result<Vec<_>, _>>()?;
    let mut row = match &item.original {
        Some(RequestOriginal::Text { text }) if text.trim().is_empty() => {
            reading.admit_images()?;
            RecordInput {
                original: QuestionInput::Text(text.clone()),
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
            }
        }
        Some(RequestOriginal::Text { text }) => compose_original(reading, RawRecord::text(text)?)?,
        Some(RequestOriginal::Json { value }) => compose_original(reading, value.clone())?,
        None => {
            reading.admit_images()?;
            RecordInput {
                original: QuestionInput::Text(String::new()),
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
            }
        }
    };
    if let Some(context) = &item.context {
        context.validate(schema)?;
        row.context = Some(context.clone());
    }
    if let Some(options) = &item.options {
        row.options = Some(options.clone());
    }
    if let Some(seeds) = &item.seed_spans {
        row.seed_spans = Some(seeds.clone());
    }
    if let Some(examples) = &item.examples {
        row.examples = Some(examples.clone());
    }
    if !images.is_empty() {
        row.original = match row.original {
            QuestionInput::Record(record) => QuestionInput::Record(record.with_images(images)?),
            QuestionInput::Text(text) => QuestionInput::Images(crate::ImageEvidence::new(
                item.original.as_ref().map(|_| text),
                images,
            )?),
            QuestionInput::Images(_) => {
                return Err(Error::defect("images entered before attachment resolution"));
            }
        };
    }
    Ok(row)
}
pub(super) fn compose_original(
    reading: &RecordReading,
    original: RawRecord,
) -> Result<RecordInput<QuestionInput>, Error> {
    let row = reading.compose(original)?;
    Ok(RecordInput {
        original: row.original.question_input(),
        context: row.context,
        options: row.options,
        examples: row.examples,
        seed_spans: row.seed_spans,
    })
}
fn read_image(image: &RequestImage, budget: &mut AttachmentBudget) -> Result<ImageInput, Error> {
    match image {
        RequestImage::Bytes { media, bytes } => {
            budget.charge(bytes.len())?;
            ImageInput::new(*media, bytes.clone())
        }
        RequestImage::File { path, media } => {
            let options = crate::InputReaderOptions {
                reading: crate::ReaderOptions {
                    unit: crate::SourceUnit::File,
                    window: None,
                },
                media: crate::ReaderMedia::Image,
            };
            let mut items = match budget.remaining() {
                Some(remaining) => crate::SourceItems::bounded_images([path], options, remaining)?,
                None => crate::read_inputs([path], options)?,
            };
            let Some(crate::SourceItem::Image(image)) = items.next().transpose()? else {
                return Err(Error::usage("an attachment requires one image file"));
            };
            if items.next().transpose()?.is_some() {
                return Err(Error::usage("an attachment requires one image file"));
            }
            budget.charge(image.record.bytes().len())?;
            match media {
                Some(media) => ImageInput::new(*media, image.record.0.bytes.clone()),
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
        RequestDefinition::DynamicChoose(q) => metadata_reading(&q.metadata),
        _ => RecordReading::new(&[], None, None),
    }
}
fn question_reading(q: &crate::Question) -> Result<RecordReading, Error> {
    metadata_reading(&q.metadata)
}
fn metadata_reading(
    metadata: &crate::core::declaration::QuestionMetadata,
) -> Result<RecordReading, Error> {
    RecordReading::new(
        &metadata
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

pub(super) fn reading(
    definition: &RequestDefinition,
    options: &super::RequestOptions,
) -> Result<RecordReading, Error> {
    let fields = options
        .field
        .iter()
        .flatten()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let explicit = options.field.is_some()
        || options.context_field.is_some()
        || options.options_field.is_some()
        || options.examples_field.is_some()
        || options.seed_spans_field.is_some();
    let reading = if explicit {
        RecordReading::new(
            &fields,
            options.context_field.as_deref(),
            options.options_field.as_deref(),
        )?
    } else {
        authored_reading(definition)?
    };
    let reading = if let Some(pointer) = &options.examples_field {
        reading.with_examples_field(pointer)?
    } else {
        reading
    };
    let reading = if let Some(pointer) = &options.seed_spans_field {
        reading.with_seed_spans_field(pointer)?
    } else {
        reading
    };
    Ok(reading)
}

pub(super) fn source_row(
    item: crate::SourceItem,
    annotate: bool,
    reading: &RecordReading,
) -> Result<RecordInput<QuestionInput>, Error> {
    if annotate && let crate::SourceItem::Text(text) = item {
        return Ok(RecordInput {
            original: QuestionInput::annotation_text(
                &text.record,
                crate::SourceLocation::new(text.file, Some(text.first_line), Some(text.last_line))?,
            )?,
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
        });
    }
    reading.compose_source(item)
}
fn attach_shared(mut item: RequestItem, images: &[RequestImage]) -> Result<RequestItem, Error> {
    if images.is_empty() {
        return Ok(item);
    }
    if !item.images.is_empty() {
        return Err(Error::usage(
            "feed item images conflict with shared attachments",
        ));
    }
    item.images = images.to_vec();
    Ok(item)
}

fn admit_image_route(item: &RequestItem, refusal: Option<&str>) -> Result<(), Error> {
    if !item.images.is_empty()
        && let Some(message) = refusal
    {
        return Err(Error::usage(message));
    }
    Ok(())
}

#[cfg(feature = "cli")]
fn compose_descriptor(
    descriptor: &TransportDescriptor,
    reading: &RecordReading,
    schema: Option<&crate::InputDeclaration>,
    budget: &mut AttachmentBudget,
) -> Result<RecordInput<QuestionInput>, Error> {
    let Some(source) = &descriptor.source else {
        return compose_item(&descriptor.item, reading, schema, budget);
    };
    let images = descriptor
        .item
        .images
        .iter()
        .map(|image| read_image(image, budget))
        .collect::<Result<Vec<_>, _>>()?;
    let mut items = crate::read_inputs(
        &source.paths,
        crate::InputReaderOptions {
            reading: source.reading,
            media: source.media,
        },
    )?;
    let original = items
        .next()
        .transpose()?
        .ok_or_else(|| Error::usage("an input source requires exactly one item"))?;
    if items.next().transpose()?.is_some() {
        return Err(Error::usage("an input source requires exactly one item"));
    }
    let mut row = reading.compose_source(original)?;
    if let QuestionInput::Record(record) = row.original {
        row.original = QuestionInput::Record(record.with_images(images)?);
    }
    if let Some(context) = &descriptor.item.context {
        context.validate(schema)?;
        row.context = Some(context.clone());
    }
    if let Some(options) = &descriptor.item.options {
        row.options = Some(options.clone());
    }
    Ok(row)
}

#[cfg(feature = "cli")]
fn compose_descriptors(
    descriptors: &[TransportDescriptor],
    reading: &RecordReading,
    schema: Option<&crate::InputDeclaration>,
    budget: &mut AttachmentBudget,
    controls: CallOptions<'_>,
) -> Result<Inputs<'static>, Error> {
    let rows = descriptors
        .iter()
        .enumerate()
        .map(|(at, descriptor)| {
            controls.admission()?;
            compose_descriptor(descriptor, reading, schema, budget)
                .map_err(|error| error.at_record(at))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Box::new(rows.into_iter().map(Ok)))
}

fn read_source(
    source: &super::RequestSource,
    remaining: Option<usize>,
) -> Result<crate::SourceItems, Error> {
    let options = crate::InputReaderOptions {
        reading: source.reading,
        media: source.media,
    };
    match remaining {
        Some(remaining) => crate::SourceItems::bounded_images(&source.paths, options, remaining),
        None => crate::read_inputs(&source.paths, options),
    }
}
