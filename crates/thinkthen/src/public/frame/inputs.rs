//! Explicit typed inputs and the one native located-file reader.
use super::{PolarsCallOptions, PolarsEngine};
use crate::public::{
    Call, Engine, Error, InputEvidence, InputReaderOptions, Question, QuestionInput, QuestionKind,
    SourceItem, read_inputs,
};
use polars::prelude::{DataFrame, NamedFrom, Series};
use std::path::PathBuf;

pub(super) fn column(
    engine: &Engine,
    question: &Question,
    inputs: &[Option<QuestionInput>],
    options: PolarsCallOptions<'_>,
) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
    let (question, call) = options.apply(question)?;
    let (function, media) = match question.kind() {
        QuestionKind::Decide => (crate::RequestFunction::Decide, crate::InputFunction::Decide),
        QuestionKind::Choose => (crate::RequestFunction::Choose, crate::InputFunction::Choose),
        QuestionKind::Score => (crate::RequestFunction::Score, crate::InputFunction::Score),
        QuestionKind::Tag => (crate::RequestFunction::Tag, crate::InputFunction::Tag),
        _ => {
            return Err(Error::usage(
                "input columns take decide, choose, score or tag",
            ));
        }
    };
    for input in inputs.iter().flatten() {
        crate::public::images::guard(media, input)?;
    }
    let column = Series::new(
        "input".into(),
        inputs
            .iter()
            .map(|input| input.as_ref().map(|_| "present"))
            .collect::<Vec<_>>(),
    );
    let rows = inputs.iter().flatten().cloned().map(|original| {
        Ok(crate::RecordInput {
            original,
            context: None,
            options: None,
            seed_spans: None,
            examples: None,
        })
    });
    let call =
        super::request::execute_records(engine, question.clone().into(), function, call, rows)?;
    super::result::enrich(call, function, &column, |value| {
        super::eager::project_atomic(value, &column, question.kind())
    })
}

pub(super) fn sources(
    engine: &Engine,
    question: &Question,
    paths: &[PathBuf],
    reading: InputReaderOptions,
    options: PolarsCallOptions<'_>,
) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
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
