//! Native column conversion delegates admission and execution to Request.
use super::column::text;
use crate::RequestFunction;
use crate::{
    Call, CallOptions, CompleteRecord, Engine, Error, Question, QuestionInput, RecordInput,
    RequestArguments, RequestDefinition, RequestEnvironment, RequestFeed, RequestFraming,
    RequestInput, RequestOptions, RequestQuestion, RequestValue, Surface,
};
use polars::prelude::Series;

pub(super) fn positions(column: &Series) -> Result<Vec<usize>, Error> {
    Ok(text(column)?
        .iter()
        .enumerate()
        .filter_map(|(at, cell)| cell.map(|_| at))
        .collect())
}

pub(super) fn execute(
    engine: &Engine,
    question: &Question,
    column: &Series,
    call: crate::RequestFunction,
    controls: CallOptions<'_>,
) -> Result<Call<RequestValue>, Error> {
    execute_definition(engine, question.clone().into(), column, call, controls)
}

pub(super) fn execute_definition(
    engine: &Engine,
    definition: RequestDefinition,
    column: &Series,
    call: crate::RequestFunction,
    controls: CallOptions<'_>,
) -> Result<Call<RequestValue>, Error> {
    let cells = text(column)?;
    let rows = cells.iter().flatten().map(|cell| {
        Ok(RecordInput {
            original: QuestionInput::Text(cell.to_owned()),
            context: None,
            options: None,
            seed_spans: None,
            examples: None,
        })
    });
    execute_records(engine, definition, call, controls, rows)
}

pub(super) fn execute_records(
    engine: &Engine,
    definition: RequestDefinition,
    call: crate::RequestFunction,
    controls: CallOptions<'_>,
    rows: impl Iterator<Item = Result<RecordInput<QuestionInput>, Error>>,
) -> Result<Call<RequestValue>, Error> {
    execute_feed(
        engine,
        definition,
        call,
        controls,
        RequestFeed::from_records("column", rows),
        true,
    )
}

fn execute_feed(
    engine: &Engine,
    definition: RequestDefinition,
    call: crate::RequestFunction,
    controls: CallOptions<'_>,
    feed: RequestFeed<'_>,
    complete: bool,
) -> Result<Call<RequestValue>, Error> {
    let none = match &definition {
        RequestDefinition::Atomic(crate::LoadedQuestion::Question(question)) => {
            matches!(question.kind, crate::public::question::Kind::FindNone)
        }
        RequestDefinition::Find(find) => {
            matches!(
                find.question().kind,
                crate::public::question::Kind::FindNone
            )
        }
        _ => false,
    };
    let options = RequestOptions {
        none,
        ..RequestOptions::default()
    };
    let arguments = RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input: RequestInput::Feed {
            name: "column".into(),
            framing: RequestFraming::Document,
            reading: crate::ReaderOptions::default(),
            images: vec![],
        },
        options,
    };
    // Typed complete filters retain every observation, including rejected rows.
    let feed = if complete && call == RequestFunction::Filter {
        feed.with_all_filter_results()
    } else {
        feed
    };
    engine.call(
        call.name(),
        arguments,
        RequestEnvironment {
            controls: controls.surface(Surface::RustPolars),
            feed: Some(feed),
        },
    )
}

pub(super) fn original(input: &QuestionInput) -> Result<&str, Error> {
    match input {
        QuestionInput::Text(text) => Ok(text),
        _ => Err(Error::defect("the text column lost its native original")),
    }
}

/// Restore nullable physical positions around ordered native occurrences.
pub(super) fn project<R>(
    column: &Series,
    rows: &[CompleteRecord<QuestionInput, R>],
    mut push: impl FnMut(Option<&R>) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut rows = rows.iter().enumerate();
    for cell in text(column)?.iter() {
        let result = if cell.is_some() {
            let (at, row) = rows
                .next()
                .ok_or_else(|| Error::defect("a frame answer lost a non-null input row"))?;
            if row.ordinal() != at {
                return Err(Error::defect("a frame answer changed its occurrence order"));
            }
            Some(row.result())
        } else {
            None
        };
        push(result)?;
    }
    if rows.next().is_some() {
        return Err(Error::defect(
            "a frame call answered more rows than it read",
        ));
    }
    Ok(())
}
