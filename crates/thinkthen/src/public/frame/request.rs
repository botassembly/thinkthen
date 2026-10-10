//! Native column conversion delegates admission and execution to Request.
use super::column::text;
use crate::{
    Call, CallOptions, CompleteRecord, Engine, Error, InputEvidence, Question, QuestionInput,
    RecordInput, Request, RequestArguments, RequestCall, RequestDefinition, RequestEnvironment,
    RequestFeed, RequestFraming, RequestInput, RequestOptions, RequestOutcome, RequestQuestion,
    RequestValue, Surface,
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
    call: fn(RequestArguments) -> RequestCall,
    controls: CallOptions<'_>,
    rows: impl Iterator<Item = Result<RecordInput<QuestionInput>, Error>>,
) -> Result<Call<RequestValue>, Error> {
    execute_feed(
        engine,
        definition,
        call,
        controls,
        RequestFeed::from_records("column", rows),
    )
}

fn execute_feed(
    engine: &Engine,
    definition: RequestDefinition,
    call: fn(RequestArguments) -> RequestCall,
    controls: CallOptions<'_>,
    feed: RequestFeed<'_>,
) -> Result<Call<RequestValue>, Error> {
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
    match engine.execute_request(
        &request,
        RequestEnvironment {
            controls: controls.surface(Surface::RustPolars),
            feed: Some(feed),
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

/// Retain caller-owned originals while shared Request admits native evidence.
#[allow(
    clippy::type_complexity,
    reason = "the existing complete API retains generic originals and native results"
)]
pub(super) fn complete_records<T: InputEvidence, R>(
    engine: &Engine,
    definition: RequestDefinition,
    records: Vec<RecordInput<T>>,
    controls: CallOptions<'_>,
    function: fn(RequestArguments) -> RequestCall,
    unpack: fn(RequestValue) -> Result<Vec<CompleteRecord<QuestionInput, R>>, Error>,
) -> Result<Call<Vec<CompleteRecord<T, R>>>, Error> {
    let mut originals = Vec::with_capacity(records.len());
    let rows = records.into_iter().map(|record| {
        Ok(record.map_original(|original| {
            let input = original.question_input();
            originals.push(original);
            input
        }))
    });
    let call = execute_feed(
        engine,
        definition,
        function,
        controls,
        RequestFeed::from_records("column", rows).eager(),
    )?;
    call.try_map(|value| {
        let rows = unpack(value)?;
        let mut originals = originals.into_iter();
        let values = rows
            .into_iter()
            .enumerate()
            .map(|(at, row)| {
                if row.ordinal() != at {
                    return Err(Error::defect("a typed column changed its occurrence order"));
                }
                Ok(CompleteRecord {
                    original: originals
                        .next()
                        .ok_or_else(|| Error::defect("a typed column lost its original"))?,
                    ordinal: at,
                    result: row.into_parts().1,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        if originals.next().is_some() {
            return Err(Error::defect("a typed column lost an answer"));
        }
        Ok(values)
    })
}
