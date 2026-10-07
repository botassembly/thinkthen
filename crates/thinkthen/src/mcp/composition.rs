//! Input carriers delegate framing, projection and locations to native readers.
use super::{admission::Invocation, dispatch::PreparedQuestion};
use crate::{
    CallOptions, Error, InputEvidence, QuestionInput, RawRecord, RecordInput, RecordReading,
};

pub(super) type Inputs<'a> =
    Box<dyn Iterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a>;
impl Invocation {
    pub(super) fn records<'a>(
        &'a self,
        prepared: &PreparedQuestion,
        controls: CallOptions<'a>,
    ) -> Result<Inputs<'a>, Error> {
        let options = &self.arguments.options;
        let fields = options.field.as_deref().into_iter().collect::<Vec<_>>();
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
            match prepared {
                PreparedQuestion::Find(file) => file.reading().clone(),
                PreparedQuestion::FindPrepared(_, reading)
                | PreparedQuestion::RecognizePrepared(_, reading) => reading.clone(),
                PreparedQuestion::Recognize(file) => file.reading().clone(),
                _ => RecordReading::new(&[], None, None)?,
            }
        };
        let reading = match prepared {
            PreparedQuestion::Atomic(q) => match q {
                crate::LoadedQuestion::Question(q) => q.context_schema(),
                crate::LoadedQuestion::Banded(q) => q.context_schema(),
            },
            PreparedQuestion::Rank(q) => q.context_schema(),
            _ => None,
        }
        .map_or(reading.clone(), |schema| {
            reading.with_context_schema(schema.clone())
        });
        if let Some(source) = self.source()? {
            let annotate = matches!(prepared, PreparedQuestion::Annotate(_)) && !explicit;
            return Ok(Box::new(source.map(move |item| {
                controls.admission()?;
                let item = item?;
                compose_source(&reading, item, annotate)
            })));
        }
        if let Some(records) = &self.arguments.records {
            return Ok(Box::new(records.iter().map(move |raw| {
                controls.admission()?;
                let row = reading.compose(RawRecord::json(raw.get())?)?;
                Ok(RecordInput {
                    original: row.original.question_input(),
                    context: row.context,
                    options: row.options,
                })
            })));
        }
        controls.admission()?;
        let original = match self.attachments()? {
            Some(images) => QuestionInput::Images(images),
            None => QuestionInput::Text(
                self.arguments
                    .evidence
                    .clone()
                    .ok_or_else(|| Error::usage("missing evidence"))?,
            ),
        };
        if explicit {
            // Evidence is always literal text; structural input requires records/source.
            let QuestionInput::Text(text) = original else {
                return Err(Error::usage("image input cannot accompany field pointers"));
            };
            let row = reading.compose(RawRecord::text(&text)?)?;
            return Ok(Box::new(std::iter::once(Ok(RecordInput {
                original: row.original.question_input(),
                context: row.context,
                options: row.options,
            }))));
        }
        Ok(Box::new(std::iter::once(Ok(RecordInput {
            original,
            context: None,
            options: None,
        }))))
    }
}

fn compose_source(
    reading: &RecordReading,
    item: crate::SourceItem,
    annotate: bool,
) -> Result<RecordInput<QuestionInput>, Error> {
    if annotate && let crate::SourceItem::Text(text) = item {
        let original = QuestionInput::annotation_text(
            &text.record,
            crate::SourceLocation::new(text.file, Some(text.first_line), Some(text.last_line))?,
        )?;
        return Ok(RecordInput {
            original,
            context: None,
            options: None,
        });
    }
    reading.compose_source(item)
}
