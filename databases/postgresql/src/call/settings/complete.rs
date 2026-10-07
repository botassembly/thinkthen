//! Explicit complete engine controls, captured before the backend worker.
use super::{Plan, text_of};
use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use std::ffi::CString;
use thinkthen::Error;
static BASE: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static INPUT: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static OUTPUT: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static REFRESH: GucSetting<bool> = GucSetting::<bool>::new(false);
pub(super) fn read(plan: &mut Plan) -> Result<(), Error> {
    plan.base_url = text_of(&BASE).filter(|s| !s.is_empty());
    plan.refresh_cache = REFRESH.get();
    plan.prices = match (
        text_of(&INPUT).filter(|s| !s.is_empty()),
        text_of(&OUTPUT).filter(|s| !s.is_empty()),
    ) {
        (None, None) => None,
        (Some(input), Some(output)) => Some((input, output)),
        _ => {
            return Err(crate::call::usage(
                "prices require both input and output decimal strings",
            ));
        }
    };
    Ok(())
}
pub(super) fn register() {
    for (name, about, setting, context) in [
        (
            c"thinkthen.base_url",
            c"Explicit backend address",
            &BASE,
            GucContext::Suset,
        ),
        (
            c"thinkthen.usd_per_million_input",
            c"Caller input price per million tokens",
            &INPUT,
            GucContext::Userset,
        ),
        (
            c"thinkthen.usd_per_million_output",
            c"Caller output price per million tokens",
            &OUTPUT,
            GucContext::Userset,
        ),
    ] {
        GucRegistry::define_string_guc(name, about, c"", setting, context, GucFlags::default());
    }
    GucRegistry::define_bool_guc(
        c"thinkthen.refresh_cache",
        c"Refresh native cached answers",
        c"",
        &REFRESH,
        GucContext::Suset,
        GucFlags::default(),
    );
}
