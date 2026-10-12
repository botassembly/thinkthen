//! Serialize actual native outcomes, retaining completed prefixes and final facts.
use crate::calls::Crossed;
use extendr_api::prelude::*;
use serde::Serialize;
use thinkthen::{CompleteError, CompleteFacts, RequestOutcome, RequestValue};

#[derive(Serialize)]
struct Packet<'a> {
    results: &'a RequestValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    facts: Option<CompleteFacts<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<CompleteError<'a>>,
}

pub(crate) fn written(outcome: &RequestOutcome) -> Crossed<String> {
    let packet = match outcome {
        RequestOutcome::Complete(call) => Packet {
            results: call.value(),
            facts: call.facts().complete(),
            failure: None,
        },
        RequestOutcome::Failed { completed, error } => Packet {
            results: completed,
            facts: error.facts().and_then(thinkthen::Facts::complete),
            failure: Some(error.complete()),
        },
    };
    serde_json::to_string(&packet)
        .map_err(|_| crate::defect("native request outcome could not be written"))
}

pub(crate) fn attach(column: &thinkthen::RequestColumn, value: Robj) -> Crossed<Robj> {
    let list =
        List::try_from(value).map_err(|_| crate::defect("native presentation is not an R list"))?;
    let class = list.get_attrib("class");
    let mut fields = list.iter().collect::<Vec<_>>();
    fields.push((
        "positions",
        column
            .positions()
            .iter()
            .map(|at| *at as f64)
            .collect::<Vec<_>>()
            .into(),
    ));
    fields.push(("length", (column.len() as f64).into()));
    let mut value: Robj = List::from_pairs(fields).into();
    if let Some(class) = class {
        value
            .set_attrib("class", class)
            .map_err(|_| crate::defect("native presentation class could not be assigned"))?;
    }
    Ok(value)
}
pub(crate) fn view(value: Robj, view: Option<thinkthen::RequestColumnView>) -> Crossed<Robj> {
    use thinkthen::RequestColumnValues;
    let Some(view) = view else {
        return Ok(value);
    };
    let list =
        List::try_from(value).map_err(|_| crate::defect("native presentation is not an R list"))?;
    let class = list.get_attrib("class");
    let mut fields = list.iter().collect::<Vec<_>>();
    let values: Robj = match view.values {
        RequestColumnValues::Decisions(values) => values
            .into_iter()
            .map(|value| value.map_or(Rbool::na(), Rbool::from))
            .collect::<Vec<_>>()
            .into(),
        RequestColumnValues::Choices(values) => values
            .into_iter()
            .map(|value| value.map_or(Rstr::na(), Rstr::from))
            .collect::<Vec<_>>()
            .into(),
        RequestColumnValues::Scores(values) => values
            .into_iter()
            .map(|value| value.map_or(Rfloat::na(), Rfloat::from))
            .collect::<Vec<_>>()
            .into(),
        RequestColumnValues::Tags(values) => {
            List::from_values(values.into_iter().map(Robj::from)).into()
        }
    };
    fields.push(("value", values));
    if let Some(values) = view.probability {
        fields.push((
            "probability",
            values
                .into_iter()
                .map(|value| value.map_or(Rfloat::na(), Rfloat::from))
                .collect::<Vec<_>>()
                .into(),
        ));
    }
    let mut value: Robj = List::from_pairs(fields).into();
    if let Some(class) = class {
        value
            .set_attrib("class", class)
            .map_err(|_| crate::defect("native presentation class could not be assigned"))?;
    }
    Ok(value)
}
