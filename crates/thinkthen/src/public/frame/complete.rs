//! Complete collections retain original row identities across nullable columns.
use super::column::text;
use super::{request, result};
use crate::public::{
    Call, CallOptions, Edge, Engine, Error, FindSelection, InputEvidence, Question, RawRecord,
    Recognize, Recognized, RecordReading, Relate,
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
) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
    let call = request::execute(
        engine,
        question,
        texts,
        crate::RequestFunction::Filter,
        options,
    )?;
    result::enrich(call, crate::RequestFunction::Filter, texts, |value| {
        let crate::RequestValue::Filtered(rows) = value else {
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
) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
    let positions = request::positions(texts)?;
    crate::public::engine::only(question, &[crate::public::question::Kind::Rank], "rank")?;
    let call = request::execute(
        engine,
        question,
        texts,
        crate::RequestFunction::Rank,
        options,
    )?;
    result::enrich(call, crate::RequestFunction::Rank, texts, |value| {
        let crate::RequestValue::Ranked(rows) = value else {
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
) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
    let positions = request::positions(texts)?;
    let call = request::execute(
        engine,
        question,
        texts,
        crate::RequestFunction::Find,
        options,
    )?;
    result::enrich(call, crate::RequestFunction::Find, texts, |value| {
        let crate::RequestValue::Found(found) = value else {
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
) -> Result<Call<Vec<Option<Recognized>>, crate::RequestValue, Series>, Error> {
    let call = request::execute_definition(
        engine,
        ask.clone().into(),
        texts,
        crate::RequestFunction::Recognize,
        options,
    )?;
    result::enrich(call, crate::RequestFunction::Recognize, texts, |value| {
        let crate::RequestValue::Recognized(rows) = value else {
            return Err(Error::defect("recognition returned another result kind"));
        };
        let mut recognized = Vec::with_capacity(texts.len());
        request::project(texts, &rows, |row| {
            recognized.push(row.map(|row| row.value().clone()));
            Ok(())
        })?;
        Ok(recognized)
    })
}

pub(super) fn relate(
    engine: &Engine,
    ask: &Relate,
    input: &DataFrame,
    name: &str,
    kind: &str,
    options: CallOptions<'_>,
) -> Result<Call<Vec<Edge>, crate::RequestValue, Series>, Error> {
    let column = input
        .column(name)
        .map_err(|_| Error::usage("the entity frame has no selected column"))?
        .as_materialized_series();
    result::enrich(
        related_call(engine, ask, input, name, kind, options)?,
        crate::RequestFunction::Relate,
        column,
        |value| {
            let crate::RequestValue::Related(row) = value else {
                return Err(Error::defect("relate returned another result kind"));
            };
            Ok(row.result().value().to_vec())
        },
    )
}

pub(super) fn related_call(
    engine: &Engine,
    ask: &Relate,
    input: &DataFrame,
    name: &str,
    kind: &str,
    options: CallOptions<'_>,
) -> Result<Call<crate::RequestValue>, Error> {
    let column = |on| {
        input
            .column(on)
            .map(|column| column.as_materialized_series())
            .map_err(|_| Error::usage("the entity frame has no selected column"))
    };
    let names = text(column(name)?)?;
    let kinds = text(column(kind)?)?;
    let reading = RecordReading::new(&[], None, None)?;
    let records = names
        .iter()
        .zip(kinds.iter())
        .filter_map(|(name, kind)| match (name, kind) {
            (None, None) => None,
            (Some(name), Some(kind)) => {
                let original = RawRecord(std::sync::Arc::new(crate::core::Record::from_json(
                    crate::core::Json::Object(vec![
                        (
                            "name".to_owned(),
                            crate::core::Json::String(name.to_owned()),
                        ),
                        (
                            "kind".to_owned(),
                            crate::core::Json::String(kind.to_owned()),
                        ),
                    ]),
                )));
                Some(
                    reading
                        .compose(original)
                        .map(|record| record.map_original(|original| original.question_input())),
                )
            }
            _ => Some(Err(Error::usage("an entity has only one null field"))),
        });
    let ask = ask.clone().record_fields("/name", "/kind")?;
    request::execute_records(
        engine,
        ask.into(),
        crate::RequestFunction::Relate,
        options,
        records,
    )
}
