//! Pure full-input preview for the R judge surface.

use crate::calls::Crossed;
use crate::defect;
use extendr_api::prelude::*;
use thinkthen::PlanEstimate;

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
