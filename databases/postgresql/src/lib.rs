//! The thinkthen functions in SQL, over the public Rust API (ticket 0111).
//! Each reads its settings and files on the backend thread, then calls the
//! engine on a detachable worker (`call::run`). `NULL` is "not sure", and a
//! failure raises its kind's SQLSTATE.

use pgrx::datum::{Array, JsonB};
use pgrx::prelude::*;
use thinkthen::{Details, Engine, Error, LoadedQuestion, Recognize};

mod call;
mod complete;
#[path = "../../sqlite/src/complete_native/mod.rs"]
mod complete_native;
mod descriptions;
#[allow(
    unsafe_code,
    reason = "the one FFI module: interrupt flags, the signal mask, and descriptor opens"
)]
mod ffi;
mod files;
mod find;
mod forms;
mod images;
mod keyed;
mod question_file;
mod rank_set;
mod relate;
mod removed;
mod request;
mod scalar;

use call::OrRaise as _;
use files::Given;

pgrx::pg_module_magic!();

/// Read a question, set, or spec argument on the backend thread.
fn given(arg: Option<&str>, what: &str) -> Given {
    Given::read(arg, what, call::file_directory().as_deref()).or_raise()
}

fn details(
    engine: &Engine,
    question: &LoadedQuestion,
    evidence: &str,
    options: thinkthen::CallOptions<'_>,
    contextual: bool,
) -> Result<Option<Details>, Error> {
    request::details(
        engine,
        question,
        vec![request::text(evidence.to_owned())],
        options,
        contextual,
    )
    .map(|rows| rows.into_iter().next())
}

fn answer_value(answer: thinkthen::Answer) -> Option<bool> {
    match answer {
        thinkthen::Answer::Yes => Some(true),
        thinkthen::Answer::No => Some(false),
        thinkthen::Answer::Unsure => None,
    }
}

fn jsonb(text: &str) -> JsonB {
    JsonB(
        serde_json::from_str(text)
            .map_err(|_| call::defect("a result is not JSON"))
            .or_raise(),
    )
}

crate::descriptions::describe! {
"Apply an authored question set to one text input and return annotations as jsonb. NULL input returns NULL after controls and set validation; NULL set raises Usage; NULL settings uses defaults. Questions may use privileged/confined @files; evidence files must be read by the client. Failures raise SQL errors.";
[parallel_restricted];
fn thinkthen_annotate(
    set: Option<&str>,
    input: Option<&str>,
    settings: default!(Option<ffi::RawJson>, "NULL"),
) -> Option<JsonB> {
    call::guarded(|| {
        let call = forms::aggregate_controls(settings.as_ref());
        let set = given(set, "question set")
            .parse(thinkthen::QuestionSet::from_json)
            .or_raise();
        let input = input?.to_owned();
        let value = call::run(call, move |engine, options| {
            let result = request::run(
                engine,
                thinkthen::RequestFunction::Annotate,
                set.into(),
                vec![request::document(input)?],
                options,
            )?;
            match result.value() {
                thinkthen::RequestValue::Annotations(rows) => rows
                    .first()
                    .map(|row| row.result().value_json())
                    .transpose(),
                _ => Err(request::wrong_result()),
            }
        });
        value.map(|text| jsonb(&text))
    })
}
}

crate::descriptions::describe! {
"Return this backend session's cumulative requests, cache answers and token counts. Reads totals without building an engine or sending.";
[parallel_restricted];
/// This backend's totals. Tests read differences around a call (0095).
fn thinkthen_usage() -> TableIterator<
    'static,
    (
        name!(requests_sent, i64),
        name!(cache_answers, i64),
        name!(input_tokens, i64),
        name!(output_tokens, i64),
    ),
> {
    call::guarded(|| {
        let wide = |value: u64| i64::try_from(value).unwrap_or(i64::MAX);
        let counts = call::totals();
        TableIterator::once((
            wide(counts.requests_sent()),
            wide(counts.cache_answers()),
            wide(counts.input_tokens()),
            wide(counts.output_tokens()),
        ))
    })
}
}

crate::descriptions::describe! {
"Return this backend session's usage persistence state and advice as jsonb. Observation builds no engine, reads no evidence and sends nothing.";
[parallel_restricted];
/// Current usage persistence for this backend; no engine is built by observation.
fn thinkthen_usage_status() -> JsonB {
    call::guarded(|| {
        let state = call::usage_status();
        let value = match state.advice() {
            Some(advice) => serde_json::json!({"state": state, "advice": advice}),
            None => serde_json::json!({"state": state}),
        };
        JsonB(value)
    })
}
}

type Names = Vec<(String, i32, i32, i32, String, f64)>;

/// One recognize call on the worker, or none for a NULL text.
fn recognized(body: Option<&str>, ask: Recognize) -> Option<thinkthen::Recognized> {
    let body = body?.to_owned();
    Some(call::run(call::read(), move |engine, options| {
        let call = request::run(
            engine,
            thinkthen::RequestFunction::Recognize,
            ask.into(),
            vec![request::text(body)],
            options,
        )?;
        match call.into_value() {
            thinkthen::RequestValue::Recognized(rows) => rows
                .into_iter()
                .next()
                .map(|row| row.result().value().clone())
                .ok_or_else(request::wrong_result),
            _ => Err(request::wrong_result()),
        }
    }))
}

/// Every name the engine finds in the text. `start`, `end`, and `length`
/// count characters.
fn names(body: Option<&str>, ask: Recognize) -> Names {
    let place = |at: usize| i32::try_from(at).unwrap_or(i32::MAX);
    let Some(found) = recognized(body, ask) else {
        return Vec::new();
    };
    found
        .entities()
        .iter()
        .map(|held| {
            (
                held.text().to_owned(),
                place(held.start()),
                place(held.end()),
                place(held.length()),
                held.kind().to_owned(),
                held.strength(),
            )
        })
        .collect()
}

crate::descriptions::describe! {
"Recognize names of the supplied kinds in text; return text, character offsets, kind and strength as rows. NULL body gives no rows after kind validation. Failures raise SQL errors.";
[parallel_restricted];
/// Every name of the given kinds in the text, as rows.
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the columns from the signature, not an alias"
)]
fn thinkthen_recognize(
    body: Option<&str>,
    kinds: Option<Array<'_, &str>>,
) -> TableIterator<
    'static,
    (
        name!(text, String),
        name!(start, i32),
        name!(end, i32),
        name!(length, i32),
        name!(kind, String),
        name!(strength, f64),
    ),
> {
    call::guarded(|| {
        let mut ask = Recognize::builder();
        for kind in kinds.iter().flat_map(|held| held.iter().flatten()) {
            ask = thinkthen::Kind::new(kind, None)
                .and_then(|kind| ask.kind(kind))
                .or_raise();
        }
        TableIterator::new(names(body, ask.build().or_raise()))
    })
}
}

crate::descriptions::describe! {
"Recognize names using an authored JSON spec or privileged/confined @file; return names, character offsets, kinds and strengths. NULL body gives no rows after spec validation. Failures raise SQL errors.";
[name = "thinkthen_recognize", parallel_restricted];
/// Every name a version-one recognize spec asks for, as rows.
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the columns from the signature, not an alias"
)]
fn thinkthen_recognize_spec(
    body: Option<&str>,
    spec: Option<&str>,
) -> TableIterator<
    'static,
    (
        name!(text, String),
        name!(start, i32),
        name!(end, i32),
        name!(length, i32),
        name!(kind, String),
        name!(strength, f64),
    ),
> {
    call::guarded(|| {
        let ask = given(spec, "recognize spec")
            .parse(Recognize::from_json)
            .or_raise();
        TableIterator::new(names(body, ask))
    })
}
}

crate::descriptions::describe! {
"Return relations between names recognized in text using an authored JSON spec or privileged/confined @file. NULL body gives no rows after spec validation. Failures raise SQL errors.";
[parallel_restricted];
/// The spec's relations over one text, as rows.
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the inline name! tuple; an alias hides the columns"
)]
fn thinkthen_relations(
    body: Option<&str>,
    spec: Option<&str>,
) -> TableIterator<
    'static,
    (
        name!(relation, String),
        name!(source_text, String),
        name!(source_kind, String),
        name!(target_text, String),
        name!(target_kind, String),
        name!(probability, f64),
        name!(either, bool),
    ),
> {
    call::guarded(|| {
        let ask = given(spec, "recognize spec")
            .parse(Recognize::from_json)
            .or_raise();
        let Some(found) = recognized(body, ask) else {
            return TableIterator::new(Vec::new());
        };
        let rows: Vec<_> = found
            .relations()
            .unwrap_or_default()
            .iter()
            .map(|held| {
                let (source, target) = (held.source(), held.target());
                (
                    held.relation().to_owned(),
                    source.text().to_owned(),
                    source.kind().to_owned(),
                    target.text().to_owned(),
                    target.kind().to_owned(),
                    held.probability(),
                    held.either(),
                )
            })
            .collect();
        TableIterator::new(rows)
    })
}
}

#[cfg(feature = "panic-probe")]
crate::descriptions::describe! {
"Test-build probe: raise a defect SQL error while preserving the session. Sends nothing.";
[];
/// R1-10: a test build's panic becomes XX000, and the session lives.
#[allow(clippy::panic, reason = "the probe's whole job is one panic")]
fn thinkthen_panic_probe() {
    call::guarded(|| {
        panic!("the panic probe fired");
    })
}
}

/// Register the settings. Nothing else runs here: the engine builds on a
/// backend's first call, after the fork, and `_PG_init` never sends.
#[pg_guard]
extern "C-unwind" fn _PG_init() {
    call::register();
}

// Revoke the default PUBLIC grant on every function the extension owns, and
// re-revoke on each later DDL event that creates one (R1-18, R2-4, R3-10).
// The README's narrowed grant names the same functions through `pg_depend`.
extension_sql!(
    r#"
DO $thinkthen_revoke$
DECLARE
    signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d
          ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e
          ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('REVOKE ALL ON FUNCTION %s FROM PUBLIC', signature);
    END LOOP;
END
$thinkthen_revoke$;

-- The same revoke, scoped to the functions this DDL event created that the
-- extension owns, so an administrator's grant elsewhere survives.
CREATE FUNCTION thinkthen_guard_public() RETURNS event_trigger
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog AS $thinkthen_guard$
DECLARE
    signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_event_trigger_ddl_commands() c
        JOIN pg_proc p ON p.oid = c.objid
        JOIN pg_depend d
          ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e
          ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('REVOKE ALL ON FUNCTION %s FROM PUBLIC', signature);
    END LOOP;
END
$thinkthen_guard$;

REVOKE ALL ON FUNCTION thinkthen_guard_public() FROM PUBLIC;
COMMENT ON FUNCTION thinkthen_guard_public() IS
    'Internal extension event-trigger guard: revoke default PUBLIC execution on newly created extension-owned functions. Runs no model call and reads no evidence file.';

CREATE EVENT TRIGGER thinkthen_guard_public
    ON ddl_command_end
    WHEN TAG IN ('CREATE FUNCTION', 'CREATE PROCEDURE', 'CREATE AGGREGATE')
    EXECUTE FUNCTION thinkthen_guard_public();
"#,
    name = "revoke_public",
    finalize,
);
