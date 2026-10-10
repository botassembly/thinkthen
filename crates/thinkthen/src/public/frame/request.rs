//! Native column conversion delegates admission and execution to Request.
use super::column::text;
use crate::{
    Call, CallOptions, Engine, Error, Question, QuestionInput, RecordInput, Request,
    RequestArguments, RequestCall, RequestEnvironment, RequestFeed, RequestFraming, RequestInput,
    RequestOptions, RequestOutcome, RequestQuestion, RequestValue, Surface,
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
    let cells = text(column)?;
    let request = Request::new(call(RequestArguments {
        question: RequestQuestion::Definition {
            value: question.clone().into(),
        },
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
