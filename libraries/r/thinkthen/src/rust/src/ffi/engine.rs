//! Convert the public R constructor settings without changing positional arguments.

use super::{Crossed, Robj, text_of};

pub(super) fn configure(value: Robj) -> Crossed<()> {
    let settings = serde_json::from_str(&text_of(&value, "settings")?)
        .map_err(|_| crate::usage("settings JSON is one object"))?;
    crate::choose_engine(crate::Settings(settings))
}

/// One observation of the existing selected engine, not a facts snapshot.
pub(super) fn status(finish: bool) -> Crossed<Robj> {
    let engine = crate::engine()?;
    let state = if finish {
        engine.finish_usage_status()
    } else {
        engine.usage_persistence()
    };
    let value = serde_json::to_value(state)
        .map_err(|_| crate::defect("native usage state serialization failed"))?;
    let name = crate::native_results::plain(&value)?;
    let advice: Robj = state.advice().map_or_else(|| ().into(), Into::into);
    crate::native_results::tagged(
        extendr_api::List::from_pairs([("state", name), ("advice", advice)]).into(),
        "UsageStatus",
        "thinkthen_usage_status",
    )
}
