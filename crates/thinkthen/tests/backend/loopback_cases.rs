//! Every shared case the wire can carry, through the command, against the conformance backend.
//!
//! The success cases run on the case arm, and the backend fault case runs on
//! the refusal arm. Every expected request digest was recorded against the
//! canonical URL, so each is recomputed for the URL the backend served.
//! The injection cases and the question-form cases stay in-process.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use conformance_backend::Backend;
use serde::Deserialize;
use serde_json::Value;
use serde_json::value::RawValue;

use crate::harness::spawn_one as spawn;

use crate::support::shared_keys;

mod selection;
use selection::selected_ids;

const CASES: &str = include_str!("../../../../conformance/cases.json");
const CANONICAL: &str = "https://api.typesafe.ai/v1/systemone";
const KEY: [(&str, &str); 1] = [("THINKTHEN_API_KEY", "sk-loopback-cases")];

/// The cases the command wire does not run: five injections, three question
/// forms, and one case holding one request per `annotate` group. ADR 0111
/// section 5 packs a record's groups into one request, so that recording no
/// longer matches any surface.
const IN_PROCESS: [&str; 9] = [
    "18-annotate-two-groups",
    "20-usage-fault",
    "22-local-fault",
    "23-cancelled-fault",
    "24-deadline-fault",
    "25-defect-fault",
    "29-usage-json-text",
    "30-local-question-file",
    "31-usage-rank-blank-question",
];

type Checked = Result<(), String>;

/// The members whose key order the command sees, kept as written.
#[derive(Deserialize)]
struct Written {
    cases: Vec<Verbatim>,
}

#[derive(Deserialize)]
struct Verbatim {
    question: Option<Box<RawValue>>,
    question_set: Option<Box<RawValue>>,
}

#[test]
fn every_wire_case_passes_through_the_command_on_the_conformance_backend() {
    let document: Value = serde_json::from_str(CASES).expect("the shared cases");
    let cases = document["cases"].as_array().expect("a case list");
    let written: Written = serde_json::from_str(CASES).expect("the shared cases as written");
    let selected = selected_ids(cases).expect("a valid shared case selector");
    let selected_count = selected.as_ref().map_or(cases.len(), BTreeSet::len);
    let backend = Backend::start().expect("backend");
    let mut failures = Vec::new();
    let mut ran = 0;
    let mut not_run = 0;
    for (case, verbatim) in cases.iter().zip(&written.cases) {
        let id = case["id"].as_str().expect("a case id");
        if selected.as_ref().is_some_and(|ids| !ids.contains(id)) {
            continue;
        }
        if IN_PROCESS.contains(&id) {
            not_run += 1;
            writeln!(
                std::io::stderr().lock(),
                "{id}: not run by the command wire (in-process, question form or repacked)"
            )
            .expect("write skipped case to stderr");
            continue;
        }
        ran += 1;
        if let Err(why) = check(&backend, case, verbatim) {
            failures.push(format!("{id}: {why}"));
        }
    }
    writeln!(
        std::io::stderr().lock(),
        "command wire: total={} selected={selected_count} pass={} fail={} not_run={not_run} unselected={}",
        cases.len(),
        ran - failures.len(),
        failures.len(),
        cases.len() - selected_count
    )
    .expect("write case counts to stderr");
    assert_eq!(ran + not_run, selected_count);
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Run one case and compare every row with its expected answers.
fn check(backend: &Backend, case: &Value, verbatim: &Verbatim) -> Checked {
    let id = text(&case["id"]);
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("loopback-{id}"));
    fs::create_dir_all(&folder).map_err(|error| error.to_string())?;
    let (arguments, input) = command(case, verbatim, &folder)?;
    if !case["expect"]["error"].is_null() {
        return refused(backend, &arguments);
    }
    let base = format!("{}/case/{id}/v1", backend.origin());
    let served = format!("{base}/systemone");
    // Every row lists question keys by ADR 0111.
    let renamed = shared_keys::fixture_keys(CANONICAL, &served, list(&case["exchanges"]))?;
    let success = &case["expect"]["success"];
    let answers = recomputed(&success["answers"], &renamed);
    let tail = ["--details", "--url", base.as_str(), "--no-cache"].map(str::to_owned);
    let rows = rows(&[arguments.as_slice(), &tail].concat(), &input, success)?;
    match text(&case["verb"]) {
        "recognize" | "relate" => whole(&rows, &answers),
        "annotate" => annotated(&rows, &answers, success),
        "find" => found(&rows, &answers, case),
        "filter" => {
            let kept = list(&success["operation"]["indexes"]);
            ordered(&rows, &answers, kept.iter().map(Value::as_u64), true)
        }
        "rank" => {
            let ranking = list(&success["operation"]["ranking"]);
            ordered(
                &rows,
                &answers,
                ranking.iter().map(|at| at["index"].as_u64()),
                false,
            )
        }
        _ => {
            ordered(&rows, &answers, (0..answers.len() as u64).map(Some), true)?;
            counted(backend, case, &arguments, &input, &base)
        }
    }
}

/// The command line and standard input that ask one case's question.
fn command(
    case: &Value,
    verbatim: &Verbatim,
    folder: &std::path::Path,
) -> Result<(Vec<String>, String), String> {
    let verb = text(&case["verb"]).to_owned();
    let exchanges = list(&case["exchanges"]);
    let evidence = |at: usize| text(&exchanges[at]["evidence"]).to_owned();
    if verb == "find" {
        let question = &case["question"];
        let mut arguments = vec![verb, text(&question["find"]).to_owned()];
        if question["none"] == Value::Bool(true) {
            arguments.push("--none".to_owned());
        }
        let units = list(&question["units"])
            .iter()
            .map(|unit| text(unit).to_owned());
        return Ok((arguments, lines(&units.collect::<Vec<_>>())));
    }
    let asked = if verb == "annotate" {
        &verbatim.question_set
    } else {
        &verbatim.question
    };
    let asked = asked.as_ref().map_or("", |raw| raw.get());
    let path = folder.join("question.json");
    if verb == "annotate" {
        return annotating(verb, &path, asked, exchanges);
    }
    fs::write(&path, asked).map_err(|error| error.to_string())?;
    let path = path.display().to_string();
    let question = vec![verb.clone(), format!("@{path}")];
    Ok(match (verb.as_str(), exchanges.len()) {
        ("recognize", _) => (question, text(&case["text"]).to_owned()),
        ("relate", _) => (question, case["entities"].to_string()),
        ("filter" | "rank", _) => (
            [question, vec!["--lines".to_owned()]].concat(),
            lines(&(0..exchanges.len()).map(evidence).collect::<Vec<_>>()),
        ),
        (_, 0) => (question, "any text".to_owned()),
        (_, 1) => (question, evidence(0)),
        _ => (
            [question, vec!["--lines".to_owned()]].concat(),
            lines(&(0..exchanges.len()).map(evidence).collect::<Vec<_>>()),
        ),
    })
}

/// An annotate command and its records, one per run of groups.
///
/// Each group reads the part its `on` pointer names, in declared order, so a
/// record with pointers is rebuilt as one JSON object from its groups. The
/// command appends each answer to its record under the question's name, so
/// the parts sit under `/record` and each pointer moves there. No pointer
/// reaches the request, so its bytes stay the case's own.
fn annotating(
    verb: String,
    path: &std::path::Path,
    asked: &str,
    exchanges: &[Value],
) -> Result<(Vec<String>, String), String> {
    let mut pointers: Vec<&str> = Vec::new();
    for part in asked.split(r#""on""#).skip(1) {
        let value = part.trim_start().strip_prefix(':').map(str::trim_start);
        let Some(value) = value.and_then(|value| value.strip_prefix('"')) else {
            continue;
        };
        let pointer = value.split('"').next().unwrap_or_default();
        if !pointers.contains(&pointer) {
            pointers.push(pointer);
        }
    }
    let moved = pointers.iter().fold(asked.to_owned(), |moved, pointer| {
        moved.replace(&format!("\"{pointer}\""), &format!("\"/record{pointer}\""))
    });
    fs::write(path, moved).map_err(|error| error.to_string())?;
    let path = path.display().to_string();
    let groups = pointers.len().max(1);
    let records: Vec<String> = exchanges
        .chunks(groups)
        .map(|record| {
            let evidence = record.iter().map(|exchange| text(&exchange["evidence"]));
            if pointers.is_empty() {
                return evidence.collect();
            }
            let fields = pointers.iter().zip(evidence).map(|(pointer, evidence)| {
                (
                    pointer.trim_start_matches('/').to_owned(),
                    Value::from(evidence),
                )
            });
            serde_json::json!({ "record": Value::Object(fields.collect()) }).to_string()
        })
        .collect();
    Ok(match (records.as_slice(), pointers.is_empty()) {
        ([one], _) => (vec![verb, path], one.clone()),
        (_, true) => (vec![verb, path, "--lines".to_owned()], lines(&records)),
        (_, false) => (vec![verb, path, "--jsonl".to_owned()], lines(&records)),
    })
}

/// Each text as one line.
fn lines(texts: &[String]) -> String {
    texts.iter().map(|line| format!("{line}\n")).collect()
}

/// Run the command and read each printed row, refusing a broken exit.
fn rows(arguments: &[String], input: &str, success: &Value) -> Result<Vec<Value>, String> {
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let output = spawn(&arguments, &KEY, input.as_bytes()).map_err(|error| error.to_string())?;
    let code = output.status.code();
    let partial = success["failed_questions"].as_u64().unwrap_or(0) > 0;
    if !(matches!(code, Some(0 | 1 | 3)) || partial && code == Some(6)) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("exit {code:?}: {stderr}"));
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).map_err(|error| format!("{error}: {line}")))
        .collect()
}

/// The backend fault case: the refusal arm fails the call as a backend refusal.
fn refused(backend: &Backend, arguments: &[String]) -> Checked {
    let base = format!("{}/arm/refuse/v1", backend.origin());
    let tail = ["--url", base.as_str(), "--no-cache"].map(str::to_owned);
    let arguments = [arguments, &tail].concat();
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let output = spawn(&arguments, &KEY, b"any text").map_err(|error| error.to_string())?;
    same(
        "stderr",
        &Value::from(String::from_utf8_lossy(&output.stderr)),
        &Value::from(
            "thinkthen: the backend answered with status 422: the backend refused the request as malformed or too large\n",
        ),
    )?;
    same("exit", &Value::from(output.status.code()), &Value::from(4))
}

/// One row per expected answer, in the given exchange order.
fn ordered(
    rows: &[Value],
    answers: &[Value],
    order: impl Iterator<Item = Option<u64>>,
    valued: bool,
) -> Checked {
    let order: Vec<Option<u64>> = order.collect();
    same(
        "row count",
        &Value::from(rows.len()),
        &Value::from(order.len()),
    )?;
    for (row, exchange) in rows.iter().zip(order) {
        let want = answers
            .iter()
            .find(|answer| answer["exchange"].as_u64() == exchange)
            .ok_or("no expected answer for a printed row")?;
        if valued {
            same("value", &row["value"], &want["bare"])?;
        }
        same("answer", &row["answer"], &want["details"]["answer"])?;
        same(
            "question_sha256",
            &row["meta"]["question_sha256"],
            &want["details"]["question_sha256"],
        )?;
        meta(row, want)?;
    }
    Ok(())
}

/// A recognize or relate case prints one whole result.
fn whole(rows: &[Value], answers: &[Value]) -> Checked {
    let [row] = rows else {
        return Err(format!("{} rows", rows.len()));
    };
    let want = answers.first().ok_or("no expected answer")?;
    same("value", &row["value"], &want["bare"])?;
    same(
        "question_sha256",
        &row["meta"]["question_sha256"],
        &want["details"]["question_sha256"],
    )?;
    every_key(row, want)
}

/// One annotated row per record, each answer under its name.
fn annotated(rows: &[Value], answers: &[Value], success: &Value) -> Checked {
    let first = list(&success["answers"]).first().ok_or("no answer")?;
    let groups = list(&first["details"]["requests"]).len() as u64;
    let mut failed = 0;
    for want in answers {
        let record = want["exchange"].as_u64().ok_or("no exchange")? / groups.max(1);
        let row = rows.get(record as usize).ok_or("a record printed no row")?;
        let name = text(&want["name"]);
        same(name, &row["value"][name], &want["bare"])?;
        let detail = &row["answers"][name];
        if want["details"]["failure"].is_null() {
            same(name, &detail["answer"], &want["details"]["answer"])?;
        } else {
            same(name, &detail["failure"], &want["details"]["failure"])?;
            failed += 1;
        }
        meta(row, want)?;
    }
    same(
        "failed",
        &Value::from(failed),
        &Value::from(success["failed_questions"].as_u64().unwrap_or(0)),
    )
}

/// A find case prints the selected unit, or null, beside the wire leader.
fn found(rows: &[Value], answers: &[Value], case: &Value) -> Checked {
    let [row] = rows else {
        return Err(format!("{} rows", rows.len()));
    };
    let want = answers.first().ok_or("no expected answer")?;
    let selected = case["expect"]["success"]["operation"]["selected"].as_u64();
    let unit = selected.map_or(Value::Null, |at| {
        case["question"]["units"][at as usize].clone()
    });
    same("value", &row["value"], &unit)?;
    for field in ["pick", "probabilities"] {
        same(
            field,
            &row["answer"][field],
            &want["details"]["answer"][field],
        )?;
    }
    same(
        "question_sha256",
        &row["meta"]["question_sha256"],
        &want["details"]["question_sha256"],
    )?;
    every_key(row, want)
}

/// The model of a whole-call row, and every question key of every request
/// the case recorded, in order.
fn every_key(row: &Value, want: &Value) -> Checked {
    same("model", &row["meta"]["model"], &want["details"]["model"])?;
    let keys: Vec<Value> = list(&want["details"]["requests"])
        .iter()
        .flat_map(|request| list(request).iter().cloned())
        .collect();
    same("requests", &row["meta"]["requests"], &Value::from(keys))
}

/// The model and the recomputed request names a row carries. A record
/// function's row lists its own question keys, in order, out of the keys of
/// the requests the case recorded.
fn meta(row: &Value, want: &Value) -> Checked {
    same("model", &row["meta"]["model"], &want["details"]["model"])?;
    let wanted = &want["details"]["requests"];
    if !list(wanted).iter().any(Value::is_array) {
        return same("requests", &row["meta"]["requests"], wanted);
    }
    let printed = list(&row["meta"]["requests"]);
    let mut rest = list(wanted).iter().flat_map(list);
    if !printed.is_empty() && printed.iter().all(|key| rest.any(|held| held == key)) {
        Ok(())
    } else {
        Err(format!(
            "requests: printed {printed:?}, expected keys out of {wanted}"
        ))
    }
}

/// A counters case repeats its call through one cache folder and counts both sides.
fn counted(
    backend: &Backend,
    case: &Value,
    arguments: &[String],
    input: &str,
    base: &str,
) -> Checked {
    let counters = &case["expect"]["success"]["counters"];
    let Some(calls) = counters["calls"].as_u64() else {
        return Ok(());
    };
    let cache = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("loopback-cache-{}", text(&case["id"])));
    let _fresh = fs::remove_dir_all(&cache);
    let cache = cache.display().to_string();
    let tail = ["--details", "--url", base, "--cache", cache.as_str()].map(str::to_owned);
    let before = backend.count();
    let mut cached = 0;
    for _ in 0..calls {
        let rows = rows(&[arguments, &tail].concat(), input, &Value::Null)?;
        cached += rows
            .iter()
            .filter(|row| row["meta"]["cached"] == Value::Bool(true))
            .count();
    }
    same(
        "requests",
        &Value::from(backend.count() - before),
        &counters["requests"],
    )?;
    same(
        "cache answers",
        &Value::from(cached),
        &counters["cache_answers"],
    )
}

/// Expected answers with each canonical request digest swapped for the served one.
fn recomputed(answers: &Value, renamed: &BTreeMap<String, Value>) -> Vec<Value> {
    fn swap(value: &Value, renamed: &BTreeMap<String, Value>) -> Value {
        match value {
            Value::String(held) => renamed.get(held).cloned().unwrap_or_else(|| value.clone()),
            Value::Array(items) => items.iter().map(|item| swap(item, renamed)).collect(),
            Value::Object(fields) => fields
                .iter()
                .map(|(name, field)| (name.clone(), swap(field, renamed)))
                .collect(),
            other => other.clone(),
        }
    }
    list(answers)
        .iter()
        .map(|answer| swap(answer, renamed))
        .collect()
}

/// Compare two JSON values, holding numbers to a rounding tolerance.
fn same(what: &str, actual: &Value, expected: &Value) -> Checked {
    fn close(one: &Value, other: &Value) -> bool {
        match (one, other) {
            (Value::Number(one), Value::Number(other)) => {
                (one.as_f64().unwrap_or(f64::NAN) - other.as_f64().unwrap_or(f64::NAN)).abs() < 1e-9
            }
            (Value::Array(one), Value::Array(other)) => {
                one.len() == other.len()
                    && one.iter().zip(other).all(|(one, other)| close(one, other))
            }
            (Value::Object(one), Value::Object(other)) => {
                one.len() == other.len()
                    && one
                        .iter()
                        .all(|(name, field)| other.get(name).is_some_and(|held| close(field, held)))
            }
            _ => one == other,
        }
    }
    if close(actual, expected) {
        Ok(())
    } else {
        Err(format!("{what}: printed {actual}, expected {expected}"))
    }
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_default()
}

fn list(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}
