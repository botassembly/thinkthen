//! The thinkthen functions in SQL, over the public Rust API (ticket 0111).
//! Each reads its settings and files on the backend thread, then calls the
//! engine on a detachable worker (`call::run`). `NULL` is "not sure", and a
//! failure raises its kind's SQLSTATE.

use pgrx::datum::{Array, JsonB};
use pgrx::prelude::*;
use thinkthen::{Details, Engine, Error, Judgment, LoadedQuestion, Probabilities, Recognize};

mod call;
#[allow(
    unsafe_code,
    reason = "the one FFI module: interrupt flags, the signal mask, and descriptor opens"
)]
mod ffi;
mod files;
mod relate;
mod warm;

use call::{OrRaise as _, Refusal};
use files::Given;

pgrx::pg_module_magic!();

/// Read a question, set, or spec argument on the backend thread.
fn given(arg: Option<&str>, what: &str) -> Given {
    Given::read(arg, what, call::file_directory().as_deref()).or_raise()
}

/// The question argument, with the function's members joined under `key`.
fn question(arg: Option<&str>, key: &str, members: Option<Array<'_, &str>>) -> LoadedQuestion {
    let members = members.map(|held| held.iter().flatten().map(str::to_owned).collect());
    given(arg, "question")
        .with_members(key, members)
        .and_then(|held| held.parse(thinkthen::Question::from_json))
        .or_raise()
}

fn decide(
    engine: &Engine,
    question: &LoadedQuestion,
    evidence: &str,
    options: thinkthen::CallOptions<'_>,
) -> Result<thinkthen::Answer, Error> {
    match question {
        LoadedQuestion::Question(held) => engine.decide_with(held, evidence, options),
        LoadedQuestion::Banded(held) => engine.decide_with(held, evidence, options),
    }
}

fn details(
    engine: &Engine,
    question: &LoadedQuestion,
    evidence: &str,
    options: thinkthen::CallOptions<'_>,
) -> Result<Details, Error> {
    match question {
        LoadedQuestion::Question(held) => engine.details_with(held, evidence, options),
        LoadedQuestion::Banded(held) => engine.details_with(held, evidence, options),
    }
}

/// One judgment's details, run on the worker.
fn judged(question: LoadedQuestion, evidence: &str) -> Details {
    let evidence = evidence.to_owned();
    call::run(call::read(), move |engine, options| {
        details(engine, &question, &evidence, options)
    })
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
            .map_err(|_| Refusal::of(thinkthen::ErrorKind::Defect, "a result is not JSON"))
            .or_raise(),
    )
}

#[pg_extern(parallel_restricted)]
fn thinkthen_decide(question: Option<&str>, evidence: Option<&str>) -> Option<bool> {
    let question = self::question(question, "", None);
    let evidence = evidence?.to_owned();
    answer_value(call::run(call::read(), move |engine, options| {
        decide(engine, &question, &evidence, options)
    }))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_probability(question: Option<&str>, evidence: Option<&str>) -> Option<f64> {
    let question = self::question(question, "", None);
    match judged(question, evidence?).probabilities() {
        Probabilities::YesNo { yes } => Some(*yes),
        Probabilities::Named(_) => call::raise(Refusal::usage(
            "thinkthen_probability takes a decide question",
        )),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_choose(
    question: Option<&str>,
    evidence: Option<&str>,
    options: Option<Array<'_, &str>>,
) -> Option<String> {
    let question = self::question(question, "options", options);
    match judged(question, evidence?).value() {
        Judgment::Choice(pick) => pick.clone(),
        _ => call::raise(Refusal::usage("thinkthen_choose takes a choose question")),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_score(
    question: Option<&str>,
    evidence: Option<&str>,
    levels: Option<Array<'_, &str>>,
) -> Option<f64> {
    let question = self::question(question, "levels", levels);
    match judged(question, evidence?).value() {
        Judgment::Score(position) => Some(*position),
        _ => call::raise(Refusal::usage("thinkthen_score takes a score question")),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_tag(
    question: Option<&str>,
    evidence: Option<&str>,
    labels: Option<Array<'_, &str>>,
) -> Option<Vec<String>> {
    let question = self::question(question, "labels", labels);
    match judged(question, evidence?).value() {
        Judgment::Tags(held) => Some(held.clone()),
        _ => call::raise(Refusal::usage("thinkthen_tag takes a tag question")),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_annotate(set: Option<&str>, evidence: Option<&str>) -> Option<JsonB> {
    let set = given(set, "question set")
        .parse(thinkthen::QuestionSet::from_json)
        .or_raise();
    let evidence = evidence?.to_owned();
    let value = call::run(call::read(), move |engine, options| {
        let mut records = engine.annotate_with(&set, [evidence.as_str()], options);
        records
            .next()
            .transpose()
            .map(|record| record.map(|held| held.value_json()))
    });
    value.map(|text| jsonb(&text))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_details(question: Option<&str>, evidence: Option<&str>) -> Option<JsonB> {
    let question = self::question(question, "", None);
    Some(jsonb(&judged(question, evidence?).to_json()))
}

/// This backend's totals. Tests read differences around a call (0095).
#[pg_extern(parallel_restricted)]
fn thinkthen_usage() -> TableIterator<
    'static,
    (
        name!(requests_sent, i64),
        name!(cache_answers, i64),
        name!(input_tokens, i64),
        name!(output_tokens, i64),
    ),
> {
    let wide = |value: u64| i64::try_from(value).unwrap_or(i64::MAX);
    let [sent, cached, input, output] = call::totals();
    TableIterator::once((wide(sent), wide(cached), wide(input), wide(output)))
}

type Names = Vec<(String, i32, i32, i32, String, f64)>;

/// One recognize call on the worker, or none for a NULL text.
fn recognized(body: Option<&str>, ask: Recognize) -> Option<thinkthen::Recognized> {
    let body = body?.to_owned();
    Some(call::run(call::read(), move |engine, options| {
        engine.recognize_with(&ask, &body, options)
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

/// Every name of the given kinds in the text, as rows.
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the columns from the signature, not an alias"
)]
#[pg_extern(parallel_restricted)]
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
    let mut ask = Recognize::builder();
    for kind in kinds.iter().flat_map(|held| held.iter().flatten()) {
        ask = thinkthen::Kind::new(kind, None)
            .and_then(|kind| ask.kind(kind))
            .or_raise();
    }
    TableIterator::new(names(body, ask.build().or_raise()))
}

/// Every name a version-one recognize spec asks for, as rows.
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the columns from the signature, not an alias"
)]
#[pg_extern(name = "thinkthen_recognize", parallel_restricted)]
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
    let ask = given(spec, "recognize spec")
        .parse(Recognize::from_json)
        .or_raise();
    TableIterator::new(names(body, ask))
}

/// The spec's relations over one text, as rows.
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the inline name! tuple; an alias hides the columns"
)]
#[pg_extern(parallel_restricted)]
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
    ),
> {
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
            )
        })
        .collect();
    TableIterator::new(rows)
}

/// Decide each distinct text once, on one worker. A batch over the total's
/// remaining requests sends only those, then refuses.
fn decide_distinct(question: LoadedQuestion, mut distinct: Vec<String>) -> Vec<Option<bool>> {
    let call = call::read();
    let short = call.within(distinct.len()).map(|left| {
        distinct.truncate(left);
        call.spent()
    });
    let decided = call::run(call, move |engine, options| {
        let rows = match &question {
            LoadedQuestion::Question(held) => engine
                .decide_many_with(held, distinct.iter().map(String::as_str), options)
                .collect::<Result<Vec<_>, _>>(),
            LoadedQuestion::Banded(held) => engine
                .decide_many_with(held, distinct.iter().map(String::as_str), options)
                .collect::<Result<Vec<_>, _>>(),
        };
        rows.map(|rows| {
            rows.into_iter()
                .map(|row| answer_value(*row.value()))
                .collect()
        })
    });
    if let Some(refusal) = short {
        call::raise(refusal);
    }
    decided
}

/// The array form: one row per element, its place from 0, one worker.
#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn thinkthen_decide_array(
    question: Option<&str>,
    evidences: Option<Array<'_, &str>>,
) -> TableIterator<'static, (name!(i, i32), name!(decided, Option<bool>))> {
    let question = self::question(question, "", None);
    let rows: Vec<Option<String>> = evidences
        .iter()
        .flat_map(|held| held.iter().map(|text| text.map(str::to_owned)))
        .collect();
    let mut distinct: Vec<String> = rows.iter().flatten().cloned().collect();
    distinct.sort_unstable();
    distinct.dedup();
    let found = |text: &String| distinct.binary_search(text).ok();
    let at: Vec<Option<usize>> = rows
        .iter()
        .map(|text| text.as_ref().and_then(found))
        .collect();
    let decided = decide_distinct(question, distinct);
    let out: Vec<_> = at
        .iter()
        .enumerate()
        .map(|(place, at)| {
            let value = at.and_then(|at| decided.get(at).copied().flatten());
            (i32::try_from(place).unwrap_or(i32::MAX), value)
        })
        .collect();
    TableIterator::new(out)
}

/// R1-10: a test build's panic becomes XX000, and the session lives.
#[cfg(feature = "panic-probe")]
#[pg_extern]
#[allow(clippy::panic, reason = "the probe's whole job is one panic")]
fn thinkthen_panic_probe() {
    panic!("the panic probe fired");
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

CREATE EVENT TRIGGER thinkthen_guard_public
    ON ddl_command_end
    WHEN TAG IN ('CREATE FUNCTION', 'CREATE PROCEDURE', 'CREATE AGGREGATE')
    EXECUTE FUNCTION thinkthen_guard_public();
"#,
    name = "revoke_public",
    finalize,
);
