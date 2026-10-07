//! Admission composes typed native records and preserves originals and locations.
use crate::current::{Content, Input, SourceHandle};
use crate::failures::Failure;
use thinkthen::{
    InputEvidence, QuestionInput, RawRecord, RecordInput, RecordOption, RecordOptions,
    RecordReading, SourceLocation,
};
#[derive(Clone)]
pub(crate) struct Original {
    pub(super) retained: Input,
    pub(super) native: QuestionInput,
}
impl InputEvidence for Original {
    fn question_input(&self) -> QuestionInput {
        self.native.clone()
    }
}
pub(super) fn records(
    engine: &thinkthen::Engine,
    source: &SourceHandle,
    kind: u32,
    reading: Option<&RecordReading>,
) -> Result<Vec<RecordInput<Original>>, Failure> {
    source
        .read()?
        .enumerate()
        .map(|(at, record)| {
            engine.check_record_limit(at)?;
            compose(record?, kind, reading).map_err(Failure::from)
        })
        .collect()
}
pub(super) fn compose(
    retained: Input,
    kind: u32,
    reading: Option<&RecordReading>,
) -> Result<RecordInput<Original>, thinkthen::Error> {
    let native = match retained.original.as_ref() {
        Some(Content::Text(text)) if kind == 8 && retained.images.is_empty() => {
            // Native annotation keeps structural JSON and literal text distinct at every location.
            if let Some(position) = &retained.position {
                QuestionInput::annotation_text(
                    text,
                    SourceLocation::new(
                        position.file.clone(),
                        position.first_line,
                        position.last_line,
                    )?,
                )?
            } else {
                QuestionInput::Text(text.clone())
            }
        }
        Some(Content::Text(text))
            if reading.is_some() && retained.position.is_some() && retained.images.is_empty() =>
        {
            source_input(&retained, text, reading)?
        }
        Some(content) => {
            let original = match content {
                Content::Text(text) => RawRecord::text(text)?,
                Content::Json(raw) => RawRecord::json(raw.get())?,
            };
            let default = RecordReading::new(&[], None, None)?;
            let mut evidence = reading.unwrap_or(&default).compose(original)?.original;
            if let Some(position) = &retained.position {
                evidence = evidence.with_location(SourceLocation::new(
                    position.file.clone(),
                    position.first_line,
                    position.last_line,
                )?);
            }
            if !retained.images.is_empty() {
                evidence = evidence.with_images(
                    retained
                        .images
                        .iter()
                        .map(|image| image.native.clone())
                        .collect(),
                )?;
            }
            evidence.question_input()
        }
        None if retained.position.is_some() && retained.images.len() == 1 => {
            let file = retained
                .position
                .as_ref()
                .map(|p| p.file.clone())
                .unwrap_or_default();
            let image = retained.images.first().ok_or_else(|| {
                thinkthen::Error::new(
                    thinkthen::ErrorKind::Defect,
                    "native image source lost its image",
                )
            })?;
            thinkthen::ImageSourceRecord {
                record: image.native.clone(),
                file,
            }
            .question_input()
        }
        None => QuestionInput::Images(thinkthen::ImageEvidence::new(
            None,
            retained
                .images
                .iter()
                .map(|image| image.native.clone())
                .collect(),
        )?),
    };
    let context = match retained.context.as_ref() {
        None => None,
        Some(Content::Text(text)) => Some(thinkthen::RecordContext::Text(text.clone())),
        Some(Content::Json(raw)) => Some(thinkthen::RecordContext::Object(
            thinkthen::ObjectContext::new(&RawRecord::json(raw.get())?)?,
        )),
    };
    let options = options(&retained.options)?;
    Ok(RecordInput {
        original: Original { retained, native },
        context,
        options,
    })
}

fn options(choices: &[crate::current::Choice]) -> Result<Option<RecordOptions>, thinkthen::Error> {
    let options = if choices.is_empty() {
        None
    } else {
        let choices = choices
            .iter()
            .map(|choice| {
                if choice.weight.is_some() {
                    return Err(thinkthen::Error::new(
                        thinkthen::ErrorKind::Usage,
                        "record options do not accept weights",
                    ));
                }
                Ok(RecordOption {
                    name: choice.name.clone(),
                    description: choice
                        .description
                        .as_ref()
                        .map(|content| match content {
                            Content::Text(text) => thinkthen::Description::text(text),
                            Content::Json(raw) => thinkthen::Description::from_json(raw.get()),
                        })
                        .transpose()?,
                })
            })
            .collect::<Result<Vec<_>, thinkthen::Error>>()?;
        Some(RecordOptions::new(choices)?)
    };
    Ok(options)
}

fn source_input(
    retained: &Input,
    text: &str,
    reading: Option<&RecordReading>,
) -> Result<QuestionInput, thinkthen::Error> {
    let position = retained.position.as_ref().ok_or_else(|| {
        thinkthen::Error::new(
            thinkthen::ErrorKind::Defect,
            "native source lost its position",
        )
    })?;
    let source = thinkthen::SourceRecord {
        record: text.to_owned(),
        file: position.file.clone(),
        first_line: position.first_line.ok_or_else(|| {
            thinkthen::Error::new(
                thinkthen::ErrorKind::Defect,
                "native text source lost its first line",
            )
        })?,
        last_line: position.last_line.ok_or_else(|| {
            thinkthen::Error::new(
                thinkthen::ErrorKind::Defect,
                "native text source lost its last line",
            )
        })?,
    };
    Ok(reading
        .ok_or_else(|| {
            thinkthen::Error::new(
                thinkthen::ErrorKind::Defect,
                "native selected source lost its reading",
            )
        })?
        .compose_source(thinkthen::SourceItem::Text(source))?
        .original)
}
