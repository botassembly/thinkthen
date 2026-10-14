//! Project every field of the native no-send plan into ordinary R values.
use crate::calls::Crossed;
use extendr_api::prelude::*;
use thinkthen::PlanEstimate;
pub(crate) fn render(plan: PlanEstimate) -> Crossed<Robj> {
    let value = serde_json::to_value(plan)
        .map_err(|_| crate::defect("native plan could not be converted"))?;
    crate::native_results::plan(&value)
}
