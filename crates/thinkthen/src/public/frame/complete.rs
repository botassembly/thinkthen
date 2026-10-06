//! Complete collections retain original row identities across nullable columns.
use super::column::text;
use crate::public::{
    Call, CallOptions, Edge, Engine, Entity, Error, Evidence, Question, Recognize, Recognized,
    Relate, Tally,
};
use polars::prelude::{DataFrame, IntoSeries, NamedFrom, Series};

struct Indexed(usize, String);
impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.1
    }
}
fn present(texts: &Series) -> Result<Vec<Indexed>, Error> {
    Ok(text(texts)?
        .iter()
        .enumerate()
        .filter_map(|(at, cell)| cell.map(|cell| Indexed(at, cell.to_owned())))
        .collect())
}
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
    let mut batch = engine.filter_with(question, text(texts)?.iter().flatten(), options);
    let mut values = polars::prelude::StringChunkedBuilder::new(texts.name().clone(), texts.len());
    for row in batch.by_ref() {
        values.append_value(row?);
    }
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| Error::defect("filter lost its facts"))?;
    Ok(Call::new(values.finish().into_series(), facts))
}
pub(super) fn rank(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
    engine
        .rank_with(question, present(texts)?, options)?
        .try_map(|rows| {
            frame(vec![
                Series::new(
                    "index".into(),
                    rows.iter()
                        .map(|row| row.input().0 as u64)
                        .collect::<Vec<_>>(),
                ),
                Series::new(
                    "record".into(),
                    rows.iter()
                        .map(|row| row.input().1.as_str())
                        .collect::<Vec<_>>(),
                ),
                Series::new(
                    "probability".into(),
                    rows.iter().map(|row| row.probability()).collect::<Vec<_>>(),
                ),
            ])
        })
}
pub(super) fn find(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
    engine
        .find_with(question, present(texts)?, options)?
        .try_map(|found| {
            let rows = found.candidates();
            let selected = found.selected().map(|row| row.0);
            frame(vec![
                Series::new(
                    "index".into(),
                    rows.iter()
                        .map(|row| row.input().map(|row| row.0 as u64))
                        .collect::<Vec<_>>(),
                ),
                Series::new(
                    "unit".into(),
                    rows.iter()
                        .map(|row| row.input().map(|row| row.1.as_str()))
                        .collect::<Vec<_>>(),
                ),
                Series::new(
                    "probability".into(),
                    rows.iter().map(|row| row.probability()).collect::<Vec<_>>(),
                ),
                Series::new(
                    "selected".into(),
                    rows.iter()
                        .map(|row| row.input().is_some_and(|row| Some(row.0) == selected))
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
    if engine.estimate_reported_cost(0, 0).is_some() {
        return Err(Error::usage(
            "priced recognition collections require native aggregate pricing",
        ));
    }
    let cells = text(texts)?;
    let options = options.started()?;
    let tally = Tally::new();
    let mut rows = Vec::with_capacity(cells.len());
    for cell in cells.iter() {
        let Some(cell) = cell else {
            rows.push(None);
            continue;
        };
        let started = tally.start();
        let result = engine.recognize_with(ask, cell, options);
        let receipt = match &result {
            Ok(call) => Some(call.facts()),
            Err(error) => error.facts(),
        };
        if let Some(receipt) = receipt {
            started
                .finish(receipt)
                .map_err(|error| error.with_facts(tally.facts()))?;
        }
        match result {
            Ok(call) => rows.push(Some(call.into_value())),
            Err(error) => {
                let facts = tally.facts();
                return Err(if facts.records() > 0 || error.facts().is_some() {
                    error.with_facts(facts)
                } else {
                    error
                });
            }
        }
    }
    Ok(Call::new(rows, tally.facts()))
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
