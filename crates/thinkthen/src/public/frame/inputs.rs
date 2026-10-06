//! Explicit typed inputs and the one native located-file reader.
use super::column::Answers;
use super::{PolarsCallOptions, PolarsEngine};
use crate::public::{
    Annotated, Call, CallOptions, Details, Engine, Error, InputEvidence, InputFunction,
    InputReaderOptions, Judgment, Question, QuestionInput, QuestionKind, SourceItem, read_inputs,
};
use polars::prelude::{DataFrame, NamedFrom, Series};
use std::path::PathBuf;

pub(super) fn details(
    engine: &Engine,
    question: &Question,
    inputs: &[Option<QuestionInput>],
    options: CallOptions<'_>,
) -> Result<Call<Vec<Option<Details>>>, Error> {
    let function = match question.kind() {
        QuestionKind::Decide => InputFunction::Decide,
        QuestionKind::Choose => InputFunction::Choose,
        QuestionKind::Score => InputFunction::Score,
        QuestionKind::Tag => InputFunction::Tag,
        _ => {
            return Err(Error::usage(
                "input columns take decide, choose, score or tag",
            ));
        }
    };
    // Validate every function/media combination before starting the column;
    // a later image cannot cause a text-only column's earlier rows to send.
    for input in inputs.iter().flatten() {
        crate::public::images::guard(function, input)?;
    }
    let mut batch =
        engine.details_input_many_with(question, inputs.iter().flatten().cloned(), options);
    let mut rows = Vec::with_capacity(inputs.len());
    for input in inputs {
        rows.push(if input.is_none() {
            None
        } else {
            Some(
                batch
                    .next()
                    .ok_or_else(|| Error::defect("input column lost a row"))??
                    .value()
                    .clone(),
            )
        });
    }
    if let Some(extra) = batch.next() {
        extra?;
        return Err(Error::defect("input column added a row"));
    }
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| Error::defect("input column lost facts"))?;
    Ok(Call::new(rows, facts))
}

pub(super) fn column(
    engine: &Engine,
    question: &Question,
    inputs: &[Option<QuestionInput>],
    options: PolarsCallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
    let (question, call, probability) = options.apply(question)?;
    if probability && !matches!(question.kind(), QuestionKind::Decide | QuestionKind::Choose) {
        return Err(Error::usage("probability belongs to decide and choose"));
    }
    details(engine, &question, inputs, call)?.try_map(|rows| {
        let mut answers = Answers::new("value", question.kind(), rows.len())?;
        let mut probabilities = Vec::with_capacity(rows.len());
        for row in &rows {
            let answer = row.as_ref().map(|detail| match detail.value() {
                Judgment::Decision(answer) => Annotated::Decision(*answer),
                Judgment::Choice(answer) => Annotated::Choice(answer.clone()),
                Judgment::Score(answer) => Annotated::Score(*answer),
                Judgment::Tags(answer) => Annotated::Tags(answer.clone()),
            });
            answers.push(answer.as_ref())?;
            if probability {
                probabilities.push(
                    row.as_ref()
                        .map(super::eager::selected_probability)
                        .transpose()?
                        .flatten(),
                );
            }
        }
        let mut columns = vec![answers.finish().into()];
        if probability {
            columns.push(Series::new("probability".into(), probabilities).into());
        }
        DataFrame::new(rows.len(), columns)
            .map_err(|_| Error::defect("input value frame could not be built"))
    })
}

pub(super) fn sources(
    engine: &Engine,
    question: &Question,
    paths: &[PathBuf],
    reading: InputReaderOptions,
    options: PolarsCallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
    let rows = read_inputs(paths, reading)?.collect::<Result<Vec<_>, _>>()?;
    let inputs = rows
        .iter()
        .map(|row| Some(row.question_input()))
        .collect::<Vec<_>>();
    engine
        .input_column(question, &inputs, options)?
        .try_map(|frame| {
            let files = rows
                .iter()
                .map(|row| match row {
                    SourceItem::Text(row) => row.file.as_str(),
                    SourceItem::Image(row) => row.file.as_str(),
                })
                .collect::<Vec<_>>();
            let lines = |first| {
                rows.iter()
                    .map(|row| match row {
                        SourceItem::Text(row) => Some(text_line(row, first)),
                        SourceItem::Image(_) => None,
                    })
                    .collect::<Vec<_>>()
            };
            frame
                .hstack(&[
                    Series::new("file".into(), files).into(),
                    Series::new("first_line".into(), lines(true)).into(),
                    Series::new("last_line".into(), lines(false)).into(),
                ])
                .map_err(|_| Error::defect("located value frame could not be built"))
        })
}

fn text_line(row: &crate::public::SourceRecord<String>, first: bool) -> u64 {
    if first {
        row.first_line as u64
    } else {
        row.last_line as u64
    }
}
