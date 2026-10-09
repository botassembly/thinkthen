//! Available inline originals and declarations validate without reading image files.
use super::composition::{compose_original, context_schema, reading};
use super::{AdmittedRequest, RequestDefinition, RequestInput, RequestOriginal};
use crate::{Error, RawRecord};
impl AdmittedRequest {
    pub(super) fn admit_inline(&self, definition: &RequestDefinition) -> Result<(), Error> {
        let options = &self.request.call.arguments().options;
        let reading = reading(definition, options)?;
        let schema = context_schema(definition);
        let reading = schema.cloned().map_or(reading.clone(), |schema| {
            reading.with_context_schema(schema)
        });
        let metadata = metadata(definition)?;
        let recognize = match definition {
            RequestDefinition::Recognition(q) => Some(q),
            RequestDefinition::Recognize(q) => Some(q.question()),
            _ => None,
        };
        let recognize = recognize.cloned().map(|q| {
            options
                .seed_spans
                .as_ref()
                .map_or(q.clone(), |seeds| q.with_seed_spans(seeds.clone()))
        });
        if let (Some(q), Some(examples)) = (recognize.as_ref(), &options.examples) {
            q.clone().with_examples(examples.clone())?;
        }
        let validator = Inline {
            definition,
            reading: &reading,
            metadata: &metadata,
            recognize: recognize.as_ref(),
        };
        match &self.request.call.arguments().input {
            RequestInput::Records { items }
            | RequestInput::Units { items }
            | RequestInput::Entities { items } => {
                for (at, item) in items.iter().enumerate() {
                    validator
                        .validate(
                            !item.images.is_empty(),
                            item.original.as_ref(),
                            item.context.as_ref(),
                            item.examples.as_ref(),
                            item.seed_spans.as_ref(),
                        )
                        .map_err(|error| error.at_record(at))?;
                }
            }
            RequestInput::Text { text, images } => validator.validate(
                !images.is_empty(),
                Some(&RequestOriginal::Text { text: text.clone() }),
                None,
                None,
                None,
            )?,
            RequestInput::Json { value, images } => validator.validate(
                !images.is_empty(),
                Some(&RequestOriginal::Json {
                    value: value.clone(),
                }),
                None,
                None,
                None,
            )?,
            RequestInput::Source { .. } | RequestInput::Feed { .. } => {}
        }
        Ok(())
    }
}

struct Inline<'a> {
    definition: &'a RequestDefinition,
    reading: &'a crate::RecordReading,
    metadata: &'a [&'a crate::core::declaration::QuestionMetadata],
    recognize: Option<&'a crate::Recognize>,
}
impl Inline<'_> {
    fn validate(
        &self,
        has_images: bool,
        original: Option<&RequestOriginal>,
        context: Option<&crate::RecordContext>,
        examples: Option<&Vec<crate::RecognitionExample>>,
        seeds: Option<&Vec<crate::RecognitionSeedSpan>>,
    ) -> Result<(), Error> {
        if let Some(context) = context {
            for metadata in self.metadata {
                context.validate(metadata.context_schema.as_ref())?;
            }
        }
        if let (Some(q), Some(examples)) = (self.recognize, examples) {
            q.clone().with_examples(examples.clone())?;
        }
        let Some(original) = original else {
            return Ok(());
        };
        if let RequestOriginal::Text { text } = original
            && text.trim().is_empty()
            && self.recognize.is_some()
        {
            for metadata in self.metadata {
                metadata.validate_item(&crate::QuestionInput::Text(text.clone()))?;
            }
            if let Some(q) = self.recognize {
                let q = seeds.map_or(q.clone(), |seeds| q.clone().with_seed_spans(seeds.clone()));
                crate::core::seed_stretches(&q.0, &crate::core::pieces(text))
                    .map_err(Error::usage)?;
            }
            return Ok(());
        }
        let raw = match original {
            RequestOriginal::Text { text } => RawRecord::text(text)?,
            RequestOriginal::Json { value } => value.clone(),
        };
        let row = compose_original(self.reading, raw)?;
        validate_row(
            self.definition,
            self.metadata,
            self.recognize,
            &row,
            has_images,
            seeds,
        )?;

        Ok(())
    }
}

fn metadata(
    definition: &RequestDefinition,
) -> Result<Vec<&crate::core::declaration::QuestionMetadata>, Error> {
    Ok(match definition {
        RequestDefinition::Atomic(crate::LoadedQuestion::Question(q))
        | RequestDefinition::Rank(q) => vec![&q.metadata],
        RequestDefinition::Atomic(crate::LoadedQuestion::Banded(q)) => vec![&q.0.metadata],
        RequestDefinition::DynamicChoose(q) => vec![&q.metadata],
        RequestDefinition::Find(q) => vec![&q.question().metadata],
        RequestDefinition::Recognition(q) => vec![&q.0.metadata],
        RequestDefinition::Recognize(q) => vec![&q.question().0.metadata],
        RequestDefinition::Relate(q) => vec![&q.0.metadata],
        RequestDefinition::Annotate(q) => q.0.questions().iter().map(|q| q.metadata()).collect(),
        RequestDefinition::RankSet(q) => q.0.questions().iter().map(|q| q.metadata()).collect(),
        RequestDefinition::DecodedSet { .. } => {
            return Err(Error::defect("unresolved definition in admission"));
        }
    })
}

fn validate_row(
    definition: &RequestDefinition,
    metadata: &[&crate::core::declaration::QuestionMetadata],
    recognize: Option<&crate::Recognize>,
    row: &crate::RecordInput<crate::QuestionInput>,
    has_images: bool,
    seeds: Option<&Vec<crate::RecognitionSeedSpan>>,
) -> Result<(), Error> {
    if !has_images {
        if let RequestDefinition::Annotate(set) = definition {
            let record = match &row.original {
                crate::QuestionInput::Record(record) => record.batch_record(),
                crate::QuestionInput::Text(text) => {
                    crate::public::bulk::annotation::record(&set.0, text)?
                }
                crate::QuestionInput::Images(_) => {
                    return Err(Error::usage(
                        "annotate accepts text only; images are unsupported",
                    ));
                }
            };
            crate::public::bulk::annotation::admit_declarations(&set.0, &record)?;
        } else {
            for metadata in metadata {
                metadata.validate_item(&row.original)?;
            }
        }
    }
    if let Some(context) = &row.context {
        for metadata in metadata {
            context.validate(metadata.context_schema.as_ref())?;
        }
    }
    if let (Some(q), Some(examples)) = (recognize, &row.examples) {
        q.clone().with_examples(examples.clone())?;
    }
    if let Some(q) = recognize {
        let selected = seeds.or(row.seed_spans.as_ref());
        let q = selected.map_or(q.clone(), |seeds| q.clone().with_seed_spans(seeds.clone()));
        let text = match &row.original {
            crate::QuestionInput::Text(text) => text.as_str(),
            crate::QuestionInput::Record(record) => record.plain(),
            crate::QuestionInput::Images(_) => {
                return Err(Error::usage("recognition seeds require text"));
            }
        };
        crate::core::seed_stretches(&q.0, &crate::core::pieces(text)).map_err(Error::usage)?;
    }
    Ok(())
}

pub(super) fn validate_composed(
    definition: &RequestDefinition,
    options: &super::RequestOptions,
    row: &crate::RecordInput<crate::QuestionInput>,
) -> Result<(), Error> {
    let metadata = metadata(definition)?;
    let recognize = match definition {
        RequestDefinition::Recognition(q) => Some(q),
        RequestDefinition::Recognize(q) => Some(q.question()),
        _ => None,
    };
    let recognize = recognize.cloned().map(|q| {
        options
            .seed_spans
            .as_ref()
            .map_or(q.clone(), |seeds| q.with_seed_spans(seeds.clone()))
    });
    if let (Some(q), Some(examples)) = (recognize.as_ref(), &options.examples) {
        q.clone().with_examples(examples.clone())?;
    }
    let has_images = match &row.original {
        crate::QuestionInput::Record(record) => !record.images().is_empty(),
        crate::QuestionInput::Images(images) => !images.images().is_empty(),
        crate::QuestionInput::Text(_) => false,
    };
    validate_row(
        definition,
        &metadata,
        recognize.as_ref(),
        row,
        has_images,
        None,
    )
}
