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
        let fields = options
            .field
            .iter()
            .flat_map(super::admission::Fields::values)
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
            match prepared {
                PreparedQuestion::Find(file) => file.reading().clone(),
                PreparedQuestion::FindPrepared(_, reading)
                | PreparedQuestion::RecognizePrepared(_, reading) => reading.clone(),
                PreparedQuestion::Recognize(file) => file.reading().clone(),
                PreparedQuestion::Atomic(q) => match q {
                    crate::LoadedQuestion::Question(q) => authored_reading(q)?,
                    crate::LoadedQuestion::Banded(q) => authored_reading(&q.0)?,
                },
                PreparedQuestion::Rank(q) => authored_reading(q)?,
                _ => RecordReading::new(&[], None, None)?,
            }
        };
        let reading = declared_context(prepared).map_or(reading.clone(), |schema| {
            reading.with_context_schema(schema.clone())
        });
        if let Some(source) = self.source()? {
            let annotate = matches!(prepared, PreparedQuestion::Annotate(_)) && !explicit;
            return Ok(source_records(
                reading,
                source,
                controls,
                annotate,
                self.tool == super::tools::Tool::Rank,
            ));
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

pub(super) fn source_records<'a>(
    reading: RecordReading,
    mut source: impl Iterator<Item = Result<crate::SourceItem, Error>> + 'a,
    controls: CallOptions<'a>,
    annotate: bool,
    rank: bool,
) -> Inputs<'a> {
    let mut remaining = crate::core::MAX_RECORD_BYTES;
    let mut stopped = false;
    Box::new(std::iter::from_fn(move || {
        if stopped {
            return None;
        }
        let next = controls
            .admission()
            .and_then(|()| source.next().transpose());
        let result = match next {
            Ok(None) => return None,
            Ok(Some(item)) => (|| {
                if rank {
                    charge_rank(&item, &mut remaining)?;
                }
                compose_source(&reading, item, annotate)
            })(),
            Err(error) => Err(error),
        };
        stopped = result.is_err();
        Some(result)
    }))
}

fn charge_rank(item: &crate::SourceItem, remaining: &mut usize) -> Result<(), Error> {
    if let crate::SourceItem::Text(text) = item {
        *remaining = remaining.checked_sub(text.record.len()).ok_or_else(|| {
            Error::usage("source rank reads at most 16 MiB across all input records")
        })?;
    }
    Ok(())
}

fn declared_context(prepared: &PreparedQuestion) -> Option<&crate::InputDeclaration> {
    match prepared {
        PreparedQuestion::Atomic(q) => q.context_schema(),
        PreparedQuestion::Rank(q) => q.context_schema(),
        PreparedQuestion::Dynamic(q) => q.context_schema(),
        PreparedQuestion::Find(file) => file.question().context_schema(),
        PreparedQuestion::FindPrepared(q, _) => q.context_schema(),
        PreparedQuestion::Recognize(file) => file.question().context_schema(),
        PreparedQuestion::RecognizePrepared(q, _) => q.context_schema(),
        PreparedQuestion::Relate(q) => q.context_schema(),
        PreparedQuestion::Annotate(set) => set
            .0
            .questions()
            .iter()
            .find_map(|q| q.metadata().context_schema.as_ref()),
        PreparedQuestion::RankSet(set) => set
            .0
            .questions()
            .iter()
            .find_map(|q| q.metadata().context_schema.as_ref()),
    }
}

fn authored_reading(question: &crate::Question) -> Result<RecordReading, Error> {
    let fields = question
        .metadata
        .reading
        .on
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    RecordReading::new(&fields, None, None)
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
