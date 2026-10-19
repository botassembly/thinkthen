//! Established MCP carrier aliases enter the single native Request contract.
use super::{admission::Invocation, inputs::Descriptor};
use crate::RequestFunction;
use crate::RequestDescriptor;
use crate::{
    Error, RequestArguments, RequestFeed, RequestInput,
    RequestItem, RequestOriginal, RequestQuestion, RequestSource,
};

impl Invocation {
    pub(super) fn request(&self) -> Result<(RequestArguments, Option<RequestFeed<'static>>), Error> {
        let a = &self.arguments;
        let questions = usize::from(a.question.is_some())
            + usize::from(a.question_file.is_some())
            + usize::from(a.question_name.is_some())
            + usize::from(a.question_reference.is_some());
        if questions != 1 {
            return Err(Error::new(crate::ErrorKind::Usage,
                "give exactly one question, file, name or explicit reference",
            ));
        }
        let forms = usize::from(a.evidence.is_some())
            + usize::from(a.records.is_some())
            + usize::from(a.source.is_some())
            + usize::from(a.inputs.is_some());
        if forms > 1 || (forms == 0 && a.images.is_empty()) {
            return Err(Error::new(crate::ErrorKind::Usage,
                "give one of evidence, records, source or inputs, or explicit images",
            ));
        }
        if !a.images.is_empty() && (a.records.is_some() || a.source.is_some() || a.inputs.is_some())
        {
            return Err(Error::new(crate::ErrorKind::Usage,
                "attachments cannot accompany records, source or inputs",
            ));
        }
        let question = self.question_selector()?;
        let input = self.input_selector()?;
        let arguments = RequestArguments {
            question,
            input,
            options: a.options.native.clone(),
        };
        let feed = a
            .inputs
            .as_ref()
            .map(|inputs| {
                let descriptors = inputs
                    .iter()
                    .map(Descriptor::native)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok::<_, Error>(RequestFeed::from_descriptors("mcp-inputs", descriptors))
            })
            .transpose()?;
        Ok((arguments, feed))
    }
    fn question_selector(&self) -> Result<RequestQuestion, Error> {
        let a = &self.arguments;
        Ok(if let Some(path) = &a.question_file {
            RequestQuestion::File { path: path.clone() }
        } else if let Some(name) = &a.question_name {
            RequestQuestion::Name { name: name.clone() }
        } else if let Some(reference) = &a.question_reference {
            RequestQuestion::Reference {
                reference: reference.clone(),
            }
        } else {
            let raw = a
                .question
                .as_ref()
                .ok_or_else(|| Error::new(crate::ErrorKind::Usage, "missing question"))?;
            if raw.get().starts_with('"') {
                RequestQuestion::Text {
                    text: serde_json::from_str(raw.get())
                        .map_err(|_| Error::new(crate::ErrorKind::Usage, "invalid question"))?,
                }
            } else {
                RequestQuestion::Definition {
                    value: crate::RequestDefinition::from_authored_json(raw.get())?,
                }
            }
        })
    }
    fn input_selector(&self) -> Result<RequestInput, Error> {
        let a = &self.arguments;
        Ok(if let Some(source) = &a.source {
            RequestInput::Source {
                source: source.native(),
            }
        } else if a.inputs.is_some() {
            RequestInput::Feed {
                name: "mcp-inputs".into(),
                framing: crate::RequestFraming::Document,
                reading: crate::ReaderOptions::default(),
                images: Vec::new(),
            }
        } else if let Some(text) = &a.evidence
            && a.records.is_none()
            && !matches!(
                self.tool,
                RequestFunction::Find | RequestFunction::Rank | RequestFunction::Relate
            )
        {
            RequestInput::Text {
                text: text.clone(),
                images: a
                    .images
                    .iter()
                    .map(|path| crate::RequestImage::File {
                        path: path.clone(),
                        media: None,
                    })
                    .collect(),
            }
        } else {
            let items = if let Some(records) = &a.records {
                records
                    .iter()
                    .map(|raw| record_item(raw))
                    .collect::<Result<Vec<_>, Error>>()?
            } else {
                let images = a
                    .images
                    .iter()
                    .map(|path| crate::RequestImage::File {
                        path: path.clone(),
                        media: None,
                    })
                    .collect();
                vec![item(
                    a.evidence
                        .as_ref()
                        .map(|text| RequestOriginal::Text { text: text.clone() }),
                    images,
                )]
            };
            RequestInput::Records { items }
        })
    }
}
fn item(original: Option<RequestOriginal>, images: Vec<crate::RequestImage>) -> RequestItem {
    RequestItem {
        original,
        images,
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
    }
}
impl super::admission::Source {
    pub(super) fn native(&self) -> RequestSource {
        RequestSource {
            framing: self.framing,
            paths: self.paths.clone(),
            reading: self.reading,
            media: self.media,
        }
    }
}
impl Descriptor {
    fn native(&self) -> Result<RequestDescriptor, Error> {
        if usize::from(self.text.is_some())
            + usize::from(self.json.is_some())
            + usize::from(self.source.is_some())
            > 1
        {
            return Err(Error::new(crate::ErrorKind::Usage, "a descriptor selects one original"));
        }
        let original = if let Some(text) = &self.text {
            Some(RequestOriginal::Text { text: text.clone() })
        } else {
            self.json
                .as_ref()
                .map(|raw| {
                    crate::RawRecord::json(raw.get()).map(|value| RequestOriginal::Json { value })
                })
                .transpose()?
        };
        let mut item = item(
            original,
            self.images.iter().map(|image| image.native()).collect(),
        );
        let fields: std::collections::BTreeMap<_, _> = [
            ("examples", &self.examples),
            ("seed_spans", &self.seed_spans),
        ]
        .into_iter()
        .filter_map(|(name, raw)| raw.as_ref().map(|raw| (name, raw)))
        .collect();
        let json = serde_json::to_string(&fields)
            .map_err(|_| Error::new(crate::ErrorKind::Usage, "invalid per-item controls"))?;
        let controls: RequestItem =
            serde_json::from_str(&json).map_err(|_| Error::new(crate::ErrorKind::Usage, "invalid per-item controls"))?;
        if let Some(raw) = &self.context {
            let context: RequestItem =
                serde_json::from_str(&format!("{{\"context\":{}}}", raw.get())).map_err(|_| {
                    Error::new(crate::ErrorKind::Usage, "the per-item context does not match context_schema")
                })?;
            item.context = context.context;
        }
        item.examples = controls.examples;
        item.seed_spans = controls.seed_spans;
        if let Some(raw) = &self.options {
            item.options = Some(crate::RecordOptions::project(raw.get(), "")?);
        }
        Ok(RequestDescriptor {
            item,
            source: self.source.as_ref().map(super::admission::Source::native),
        })
    }
}

fn record_item(raw: &serde_json::value::RawValue) -> Result<RequestItem, Error> {
    Ok(item(
        Some(RequestOriginal::Json {
            value: crate::RawRecord::json(raw.get())?,
        }),
        Vec::new(),
    ))
}
