//! Named reading construction keeps optional values distinct from defaults.
use crate::call;
use pgrx::prelude::*;

crate::descriptions::describe! {
"Construct native reading controls from named SQL parameters. Missing arguments retain saved controls; empty context clears fallback. Validation runs through canonical request admission.";
[parallel_restricted, requires = ["native_reading_type"]];
#[allow(clippy::too_many_arguments, reason = "PostgreSQL exposes the canonical request controls as named fields")]
fn thinkthen_reading(
    model: default!(Option<&str>, "NULL"),
    mode: default!(Option<&str>, "NULL"),
    snippet_pieces: default!(Option<i64>, "NULL"),
    relation_threshold: default!(Option<&str>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    boundary_context: default!(Option<&str>, "NULL"),
    kind_edge_context: default!(Option<&str>, "NULL"),
    relation_context: default!(Option<&str>, "NULL"),
    field: default!(Option<Array<'_, &str>>, "NULL"),
    context_field: default!(Option<&str>, "NULL"),
    options_field: default!(Option<&str>, "NULL"),
    seed_spans: default!(Option<Array<'_, pgrx::composite_type!("thinkthen_seed")>>, "NULL"),
    examples: default!(Option<Array<'_, pgrx::composite_type!("thinkthen_example")>>, "NULL"),
    seed_spans_field: default!(Option<&str>, "NULL"),
    examples_field: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
    max_requests_total: default!(Option<i64>, "NULL"),
    attempts: default!(Option<bool>, "NULL"),
    none: default!(Option<bool>, "NULL"),
    top: default!(Option<i64>, "NULL"),
    files_only: default!(Option<bool>, "NULL"),
    incremental: default!(Option<bool>, "NULL"),
    cancelled: default!(Option<bool>, "NULL"),
) -> pgrx::composite_type!('static, "thinkthen_reading") {
    call::guarded(|| {
        let mut tuple = super::output::tuple("thinkthen_reading");
        macro_rules! set {
            ($($field:ident),+) => {$(
                super::output::optional(&mut tuple, stringify!($field), $field);
            )+};
        }
        set!(model, mode, snippet_pieces, relation_threshold, threshold, context, boundary_context, kind_edge_context, relation_context, field, context_field, options_field, seed_spans, examples, seed_spans_field, examples_field, batch, deadline_ms, max_requests_total, attempts, none, top, files_only, incremental, cancelled);
        tuple
    })
}
}
