//! Native column conversion delegates admission and execution to Request.
use super::column::text;
use crate::{
    Call, CallOptions, CompleteRecord, Engine, Error, Question, QuestionInput, RecordInput,
    Request, RequestArguments, RequestCall, RequestDefinition, RequestEnvironment, RequestFeed,
    RequestFraming, RequestInput, RequestOptions, RequestOutcome, RequestQuestion, RequestValue,
    Surface,
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
    call: fn(RequestArguments) -> RequestCall,
    controls: CallOptions<'_>,
) -> Result<Call<RequestValue>, Error> {
    execute_definition(engine, question.clone().into(), column, call, controls)
}

pub(super) fn execute_definition(
    engine: &Engine,
    definition: RequestDefinition,
    column: &Series,
    call: fn(RequestArguments) -> RequestCall,
    controls: CallOptions<'_>,
) -> Result<Call<RequestValue>, Error> {
    let cells = text(column)?;
    let request = Request::new(call(RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input: RequestInput::Feed {
            name: "column".into(),
            framing: RequestFraming::Document,
            reading: crate::ReaderOptions::default(),
            images: vec![],
        },
        options: RequestOptions::default(),
    }))
    .admit()?;
    let rows = cells.iter().flatten().map(|cell| {
        Ok(RecordInput {
            original: QuestionInput::Text(cell.to_owned()),
            context: None,
            options: None,
            seed_spans: None,
            examples: None,
        })
    });
    match engine.execute_request(
        &request,
        RequestEnvironment {
            controls: controls.surface(Surface::RustPolars),
            feed: Some(RequestFeed::from_records("column", rows)),
        },
    )? {
        RequestOutcome::Complete(call) => Ok(call),
        RequestOutcome::Failed { error, .. } => Err(error),
    }
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
