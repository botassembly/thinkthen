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
        let metadata = match definition {
            RequestDefinition::Atomic(crate::LoadedQuestion::Question(q))
            | RequestDefinition::Rank(q) => vec![&q.metadata],
            RequestDefinition::Atomic(crate::LoadedQuestion::Banded(q)) => vec![&q.0.metadata],
            RequestDefinition::DynamicChoose(q) => vec![&q.metadata],
            RequestDefinition::Find(q) => vec![&q.question().metadata],
            RequestDefinition::Recognition(q) => vec![&q.0.metadata],
            RequestDefinition::Recognize(q) => vec![&q.question().0.metadata],
            RequestDefinition::Relate(q) => vec![&q.0.metadata],
            RequestDefinition::Annotate(q) => {
                q.0.questions().iter().map(|q| q.metadata()).collect()
            }
            RequestDefinition::RankSet(q) => q.0.questions().iter().map(|q| q.metadata()).collect(),
            RequestDefinition::DecodedSet { .. } => {
                return Err(Error::defect("unresolved definition in admission"));
            }
        };
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
            reading: &reading,
            metadata: &metadata,
            recognize: recognize.as_ref(),
        };
        match &self.request.call.arguments().input {
            RequestInput::Records { items }
            | RequestInput::Units { items }
            | RequestInput::Entities { items } => {
                for item in items {
                    validator.validate(
                        !item.images.is_empty(),
                        item.original.as_ref(),
                        item.context.as_ref(),
                        item.examples.as_ref(),
                        item.seed_spans.as_ref(),
                    )?;
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
        if !has_images {
            for metadata in self.metadata {
                metadata.validate_item(&row.original)?;
            }
        }
        if let Some(context) = &row.context {
            for metadata in self.metadata {
                context.validate(metadata.context_schema.as_ref())?;
            }
        }
        if let (Some(q), Some(examples)) = (self.recognize, &row.examples) {
            q.clone().with_examples(examples.clone())?;
        }
        if let Some(q) = self.recognize {
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
}
