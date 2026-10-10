//! Complete collections retain original row identities across nullable columns.
use super::column::text;
use super::request;
use crate::public::{
    Call, CallOptions, Edge, Engine, Entity, Error, FindSelection, Question, Recognize, Recognized,
    RecordInput, Relate, RequestCall, RequestValue,
};
use polars::prelude::{DataFrame, NamedFrom, Series};

fn frame(columns: Vec<Series>) -> Result<DataFrame, Error> {
    DataFrame::new(
        columns.first().map_or(0, |column| column.len()),
        columns.into_iter().map(Into::into).collect(),
    )
    .map_err(|_| Error::defect("the collection result frame could not be built"))
}
pub(super) fn filter(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<Series>, Error> {
    request::execute(engine, question, texts, RequestCall::Filter, options)?.try_map(|value| {
        let RequestValue::Filtered(rows) = value else {
            return Err(Error::defect("filter returned another result kind"));
        };
        let originals = rows
            .iter()
            .map(|row| request::original(row.original()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Series::new(texts.name().clone(), originals))
    })
}
pub(super) fn rank(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
    let positions = request::positions(texts)?;
    request::execute(engine, question, texts, RequestCall::Rank, options)?.try_map(|value| {
        let RequestValue::Ranked(rows) = value else {
            return Err(Error::defect("rank returned another result kind"));
        };
        let indices = rows
            .iter()
            .map(|row| {
                positions
                    .get(row.ordinal())
                    .copied()
                    .map(|at| at as u64)
                    .ok_or_else(|| Error::defect("rank lost a column position"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let originals = rows
            .iter()
            .map(|row| request::original(row.original()))
            .collect::<Result<Vec<_>, _>>()?;
        let probabilities = rows
            .iter()
            .map(|row| {
                row.result()
                    .canonical
                    .answer()
                    .yes()
                    .ok_or_else(|| Error::defect("rank lost its probability"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        frame(vec![
            Series::new("index".into(), indices),
            Series::new("record".into(), originals),
            Series::new("probability".into(), probabilities),
        ])
    })
}
pub(super) fn find(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
    let positions = request::positions(texts)?;
    request::execute(engine, question, texts, RequestCall::Find, options)?.try_map(|value| {
        let RequestValue::Found(found) = value else {
            return Err(Error::defect("find returned another result kind"));
        };
        let rows = found.candidates();
        let indices = rows
            .iter()
            .enumerate()
            .map(|(at, row)| {
                row.input()
                    .map(|_| {
                        positions
                            .get(at)
                            .copied()
                            .map(|at| at as u64)
                            .ok_or_else(|| Error::defect("find lost a column position"))
                    })
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let originals = rows
            .iter()
            .map(|row| row.input().map(request::original).transpose())
            .collect::<Result<Vec<_>, _>>()?;
        frame(vec![
            Series::new("index".into(), indices),
            Series::new("unit".into(), originals),
            Series::new(
                "probability".into(),
                rows.iter().map(|row| row.probability()).collect::<Vec<_>>(),
            ),
            Series::new(
                "selected".into(),
                rows.iter()
                    .enumerate()
                    .map(|(at, row)| {
                        row.input().is_some() && found.selection() == FindSelection::Unit(at)
                    })
                    .collect::<Vec<_>>(),
            ),
        ])
    })
}

pub(super) fn recognize(
    engine: &Engine,
    ask: &Recognize,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<Vec<Option<Recognized>>>, Error> {
    let cells = text(texts)?;
    let records = cells.iter().flatten().map(|text| RecordInput {
        examples: None,
        seed_spans: None,
        original: text.to_owned(),
        context: None,
        options: None,
    });
    engine
        .recognize_records_complete_with(ask, records, options)?
        .try_map(|rows| {
            let mut rows = rows.into_iter();
            cells
                .iter()
                .map(|cell| {
                    if cell.is_none() {
                        return Ok(None);
                    }
                    rows.next()
                        .map(|row| Some(row.result().value().clone()))
                        .ok_or_else(|| Error::defect("recognition lost a present cell"))
                })
                .collect()
        })
}

pub(super) fn relate(
    engine: &Engine,
    ask: &Relate,
    input: &DataFrame,
    name: &str,
    kind: &str,
    options: CallOptions<'_>,
) -> Result<Call<Vec<Edge>>, Error> {
    let column = |on| {
        input
            .column(on)
            .map(|column| column.as_materialized_series())
            .map_err(|_| Error::usage("the entity frame has no selected column"))
    };
    let names = text(column(name)?)?;
    let kinds = text(column(kind)?)?;
    let entities = names
        .iter()
        .zip(kinds.iter())
        .filter_map(|(name, kind)| match (name, kind) {
            (None, None) => None,
            (Some(name), Some(kind)) => Some(Entity::new(name, kind)),
            _ => Some(Err(Error::usage("an entity has only one null field"))),
        })
        .collect::<Result<Vec<_>, _>>()?;
    engine.relate_with(ask, entities, options)
}
