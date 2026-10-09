//! Pure full-input preview for the R judge surface.

use extendr_api::prelude::*;
use thinkthen::{BatchSetting, CallOptions, LoadedQuestion, PlanEstimate};

use crate::calls::{Crossed, question};
use crate::{carry, defect, engine};

pub(crate) fn preview(
    json: &str,
    texts: Vec<String>,
    batch: Option<BatchSetting>,
    context: Option<String>,
) -> Crossed<List> {
    let asked = question(json)?;
    let options = CallOptions::new();
    let options = batch.map_or(options, |value| options.batch(value));
    let options = context
        .as_deref()
        .map_or(options, |value| options.context(value));
    let engine = engine()?;
    let planned = match &asked {
        LoadedQuestion::Question(held) => {
            engine.plan_with(held, texts.iter().map(String::as_str), options)
        }
        LoadedQuestion::Banded(held) => {
            engine.plan_with(held, texts.iter().map(String::as_str), options)
        }
    }
    .map_err(|error| carry(&error))?;
    render(planned)
}

pub(crate) fn render(plan: PlanEstimate) -> Crossed<List> {
    let (lower, upper) = plan.estimated_input_tokens();
    let first_body = plan
        .first_body()
        .map(|body| {
            String::from_utf8(body.to_vec()).map_err(|_| defect("planned body is not UTF-8"))
        })
        .transpose()?;
    Ok(list!(
        records = plan.records() as f64,
        requests = plan.requests() as f64,
        estimated_bytes = plan.estimated_bytes() as f64,
        estimated_input_tokens = list!(lower = lower as f64, upper = upper as f64),
        upper_bound = plan.upper_bound(),
        first_body = Nullable::from(first_body)
    ))
}
