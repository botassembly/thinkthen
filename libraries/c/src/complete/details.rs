//! Additive full native details where frozen canonical layouts have one slot.
use super::{metadata, questions};
use crate::current::{ImageHandle, Position, Storage};
use crate::failures::Failure;
use crate::ffi::carriers::{
    ContentV1, DetailsV1, InputViewV1, InputViewsV1, OptionalContentV1, OptionalU64V1,
    ReportedUsageV1, RowV1, SourceDetailV1, SourceDetailsV1,
};
use crate::ffi::values as abi;
use thinkthen::{QuestionDetail, QuestionInput, ResultMetadata};
impl Storage {
    fn partial_usage(&mut self, value: Option<thinkthen::ReportedUsage>) -> ReportedUsageV1 {
        let count = |value: Option<u64>| OptionalU64V1 {
            present: i32::from(value.is_some()),
            value: value.unwrap_or(0),
        };
        ReportedUsageV1 {
            present: i32::from(value.is_some()),
            input_tokens: count(value.and_then(thinkthen::ReportedUsage::input_tokens)),
            output_tokens: count(value.and_then(thinkthen::ReportedUsage::output_tokens)),
        }
    }
    fn source_details(&mut self, values: &[thinkthen::QuestionSource]) -> SourceDetailsV1 {
        let values = values
            .iter()
            .map(|source| SourceDetailV1 {
                origin: metadata::origin(source.origin()),
                answered_by: self.string(source.answered_by()),
                batch_size: metadata::size(source.batch_size().map(|n| n as usize)),
            })
            .collect();
        let (data, len) = self.array(values);
        SourceDetailsV1 { data, len }
    }
    pub(super) fn native_inputs<'a>(
        &mut self,
        inputs: impl Iterator<Item = &'a QuestionInput>,
    ) -> Result<InputViewsV1, Failure> {
        let inputs = inputs
            .map(|input| self.native_input(input))
            .collect::<Result<Vec<_>, _>>()?;
        let (data, len) = self.array(inputs);
        Ok(InputViewsV1 { data, len })
    }
    fn native_input(&mut self, input: &QuestionInput) -> Result<InputViewV1, Failure> {
        let (original, images, location) = match input {
            QuestionInput::Text(text) => (
                OptionalContentV1 {
                    present: 1,
                    value: ContentV1 {
                        kind: abi::THINKTHEN_CONTENT_TEXT_V1,
                        data: self.string(text),
                    },
                },
                &[][..],
                None,
            ),
            QuestionInput::Images(images) => (
                images
                    .text()
                    .map(|text| OptionalContentV1 {
                        present: 1,
                        value: ContentV1 {
                            kind: abi::THINKTHEN_CONTENT_TEXT_V1,
                            data: self.string(text),
                        },
                    })
                    .unwrap_or_default(),
                images.images(),
                images.location(),
            ),
            QuestionInput::Record(record) => {
                let original = if let Some(text) = record.original().literal() {
                    OptionalContentV1 {
                        present: 1,
                        value: ContentV1 {
                            kind: abi::THINKTHEN_CONTENT_TEXT_V1,
                            data: self.string(text),
                        },
                    }
                } else {
                    self.optional_native_content(record.original().content())?
                };
                (original, record.images(), record.location())
            }
        };
        let position = location.map(|p| Position {
            file: p.file().to_owned(),
            first_line: p.first_line(),
            last_line: p.last_line(),
        });
        let images = images
            .iter()
            .map(|image| ImageHandle {
                native: image.clone(),
                filename: location.map(|p| p.file().to_owned()),
            })
            .collect::<Vec<_>>();
        Ok(InputViewV1 {
            original,
            position: self.position(position.as_ref()),
            images: self.images(&images),
        })
    }
    pub(super) fn row_details<'a>(
        &mut self,
        common: RowV1,
        meta: ResultMetadata<'_>,
        inputs: impl Iterator<Item = &'a QuestionInput>,
        raw: Option<&str>,
    ) -> Result<(), Failure> {
        let detail = self.details(common, meta, inputs, raw)?;
        self.1.push(detail);
        self.2.push(common);
        Ok(())
    }
    pub(super) fn details<'a>(
        &mut self,
        common: RowV1,
        meta: ResultMetadata<'_>,
        inputs: impl Iterator<Item = &'a QuestionInput>,
        raw: Option<&str>,
    ) -> Result<DetailsV1, Failure> {
        Ok(DetailsV1 {
            question: common.question,
            threshold: common.threshold,
            raw_pick: self.optional_string(raw),
            usage: self.partial_usage(meta.usage()),
            question_sources: self.source_details(meta.identity().question_sources()),
            observations: common.meta.observations,
            inputs: self.native_inputs(inputs)?,
        })
    }
    pub(super) fn question_details(&mut self, d: QuestionDetail<'_>) -> Result<DetailsV1, Failure> {
        Ok(DetailsV1 {
            question: self.primitive(d.question(), d.threshold())?,
            threshold: questions::threshold(d.threshold()),
            raw_pick: self.optional_string(d.raw_pick()),
            usage: self.partial_usage(d.reported_usage()),
            question_sources: self.source_details(d.question_sources()),
            observations: self.identities(d.observations()),
            inputs: self.native_inputs(d.inputs())?,
        })
    }
}
