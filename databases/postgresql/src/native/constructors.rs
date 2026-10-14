//! Named native SQL arguments construct records and reading controls.
use crate::call;
use pgrx::prelude::*;

crate::descriptions::describe! {
"Construct one explicit native record. SQL NULL omits the original; JSON null is an authored original. Files must be read by the client.";
[parallel_restricted, requires = ["native_record_types"]];
#[allow(clippy::too_many_arguments, reason = "PostgreSQL exposes defaulted named record fields")]
fn thinkthen_record(
    text: default!(Option<&str>, "NULL"),
    value: default!(Option<crate::ffi::RawJson>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    context_value: default!(Option<crate::ffi::RawJson>, "NULL"),
    options: default!(Option<Array<'_, pgrx::composite_type!("thinkthen_option")>>, "NULL"),
    seed_spans: default!(Option<Array<'_, pgrx::composite_type!("thinkthen_seed")>>, "NULL"),
    examples: default!(Option<Array<'_, pgrx::composite_type!("thinkthen_example")>>, "NULL"),
    images: default!(Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>>, "NULL"),
    source: default!(Option<pgrx::composite_type!("thinkthen_source")>, "NULL"),
    read_error: default!(Option<pgrx::composite_type!("thinkthen_reader_failure")>, "NULL"),
) -> pgrx::composite_type!('static, "thinkthen_record") {
    call::guarded(|| {
        let mut tuple = super::output::tuple("thinkthen_record");
        macro_rules! set {
            ($($field:ident),+) => {$(
                super::output::optional(&mut tuple, stringify!($field), $field);
            )+};
        }
        set!(text, value, context, context_value, options, seed_spans, examples, images, source, read_error);
        tuple
    })
}
}
