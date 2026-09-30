//! Every applicable shared case through the C door, on the conformance backend.
//!
//! Each case runs in its own `tests/c/driver.c` process, through the JSON door
//! and, where one fits, a typed function. Expected request digests were
//! recorded against the canonical URL, so each is recomputed for the URL the
//! backend served. A record function's row lists question keys by ADR 0111,
//! so its digests become the keys of the request each digest named. Two cases
//! do not apply to the door:
//!
//! - `18-annotate-two-groups` recorded each group in its own request, and ADR
//!   0111 section 5 packs a record's groups into one, as the command's wire run
//!   and the public API consumer also skip it.
//! - `25-defect-fault` injects an internal invariant failure, which no outside
//!   boundary reaches. The panic test in `src/failures.rs` covers the kind.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

use conformance_backend::Backend;
use serde_json::value::RawValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::{compile, crate_dir, run, scratch, text};

#[path = "batching.rs"]
mod batching;
#[path = "portable.rs"]
mod portable;

const CASES: &str = include_str!("../../../../conformance/cases.json");
const CANONICAL: &str = "https://api.typesafe.ai/v1/systemone";
const SKIPPED: [&str; 2] = ["18-annotate-two-groups", "25-defect-fault"];

type Checked<T = ()> = Result<T, String>;
pub(crate) type Members = BTreeMap<String, Box<RawValue>>;
/// One driver answer: its code and its bytes.
type Reply = (i32, String);
type Judge<'a> = Box<dyn Fn(&[Reply]) -> Checked + 'a>;

/// The requests one case sends through the driver, in order.
#[derive(Default)]
pub(crate) struct Script(pub(crate) Vec<u8>);

impl Script {
    pub(crate) fn ask(&mut self, verb: &str, fields: &[&str]) {
        self.0
            .extend(format!("{verb} {}\n", fields.len()).as_bytes());
        for field in fields {
            self.0
                .extend(format!("{}\n{field}\n", field.len()).as_bytes());
        }
    }
}

#[test]
fn every_applicable_shared_case_passes_through_the_door() {
    let written: Members = serde_json::from_str(CASES).expect("the shared cases");
    let cases: Vec<Members> = serde_json::from_str(written["cases"].get()).expect("a case list");
    let selected = selected_ids(&cases).expect("a valid shared case selector");
    let selected_count = selected.as_ref().map_or(cases.len(), BTreeSet::len);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("the conformance backend");
    let mut failures = Vec::new();
    let mut ran = 0;
    let mut not_run = 0;
    for case in &cases {
        let id = string(case, "id");
        if selected.as_ref().is_some_and(|ids| !ids.contains(&id)) {
            continue;
        }
        if SKIPPED.contains(&id.as_str()) {
            not_run += 1;
            writeln!(
                std::io::stderr().lock(),
                "{id}: not run by the C door (internal injection or repacked)"
            )
            .expect("write skipped case to stderr");
            continue;
        }
        ran += 1;
        let mut script = Script::default();
        script.ask("env", &["THINKTHEN_BATCH", "1"]);
        let cache = scratch(&format!("case-{id}"));
        script.ask("env", &["THINKTHEN_CACHE", &cache.display().to_string()]);
        let checked = plan(&backend, case, &mut script)
            .and_then(|judge| driven(&driver, &script, &judge))
            .and_then(|()| stored(&backend, case, &cache));
        if let Err(why) = checked {
            failures.push(format!("{id}: {why}"));
        }
    }
    writeln!(
        std::io::stderr().lock(),
        "C door: total={} selected={selected_count} pass={} fail={} not_run={not_run} unselected={}",
        cases.len(),
        ran - failures.len(),
        failures.len(),
        cases.len() - selected_count
    )
    .expect("write case counts to stderr");
    assert_eq!(ran + not_run, selected_count);
    assert!(failures.is_empty(), "{failures:#?}");
}

/// ADR 0111: a record function's case runs on the question store, which
/// holds one row per good answer.
fn stored(backend: &Backend, case: &Members, folder: &Path) -> Checked {
    let expect = member(case, "expect");
    let verb = string(case, "verb");
    if expect.get("error").is_some() || matches!(verb.as_str(), "find" | "recognize" | "relate") {
        return Ok(());
    }
    let served = format!(
        "{}/case/{}/v1/systemone",
        backend.origin(),
        string(case, "id")
    );
    let mut wanted = BTreeSet::new();
    for exchange in member(case, "exchanges").as_array().into_iter().flatten() {
        wanted.extend(keys(
            &served,
            exchange["request"].as_str().unwrap_or_default().as_bytes(),
        )?);
    }
    let failed = expect["success"]["failed_questions"].as_u64().unwrap_or(0);
    let want = i64::try_from(wanted.len()).map_err(|error| error.to_string())?
        - i64::try_from(failed).map_err(|error| error.to_string())?;
    let store = folder.join("thinkthen.sqlite");
    let held: i64 = if store.is_file() {
        rusqlite::Connection::open(store)
            .and_then(|db| db.query_row("SELECT count(*) FROM answers", [], |row| row.get(0)))
            .map_err(|error| error.to_string())?
    } else {
        0
    };
    same("stored answers", &json!(held), &json!(want))
}

/// Read one optional absolute ID list, and refuse duplicate or unknown IDs.
fn selected_ids(cases: &[Members]) -> Checked<Option<BTreeSet<String>>> {
    let mut available = BTreeSet::new();
    for case in cases {
        let id = string(case, "id");
        if id.is_empty() {
            return Err("a shared case has no ID".to_owned());
        }
        if !available.insert(id.clone()) {
            return Err(format!("duplicate shared case `{id}`"));
        }
    }
    let Some(path) = std::env::var_os("THINKTHEN_CONFORMANCE_IDS") else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return Err("THINKTHEN_CONFORMANCE_IDS takes an absolute path".to_owned());
    }
    let text =
        std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut selected = BTreeSet::new();
    for id in text
        .lines()
        .map(str::trim)
        .filter(|id| !id.is_empty() && !id.starts_with('#'))
    {
        if !selected.insert(id.to_owned()) {
            return Err(format!("duplicate selected case `{id}`"));
        }
    }
    if selected.is_empty() {
        return Err("the selected case list is empty".to_owned());
    }
    for id in &selected {
        if !available.contains(id) {
            return Err(format!(
                "selected case `{id}` is absent from the shared corpus"
            ));
        }
    }
    Ok(Some(selected))
}

/// The retry signal after a failure: 1 for a status the engine retries,
/// 0 for a refused key.
#[test]
fn a_retried_status_is_retryable_and_a_refused_key_is_not() {
    let backend = Backend::start().expect("the conformance backend");
    let asked = r#"{"decide":"Does this need attention?"}"#;
    let mut script = Script::default();
    let folder = scratch("retryable");
    for arm in ["503", "401"] {
        // A cache folder answers one backend address, so each arm gets its own.
        let cache = folder.join(arm).display().to_string();
        script.ask("env", &["THINKTHEN_CACHE", &cache]);
        let path = if arm == "401" { "status/401" } else { arm };
        let base = format!("{}/arm/{path}/v1", backend.origin());
        script.ask("retryable", &[&base, asked, "Is this urgent?"]);
    }
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    let want = [(0, "2 1".to_owned()), (0, "2 0".to_owned())];
    assert_eq!(got, want);
}

/// Run one case's requests through the driver and judge its replies.
fn driven(driver: &Path, script: &Script, judge: &Judge<'_>) -> Checked {
    let output = run(driver, "", &script.0);
    if !output.status.success() {
        return Err(format!("the driver failed: {}", text(&output.stderr)));
    }
    judge(&replies(&output.stdout)?)
}

/// Write one case's requests and return the check of their replies.
fn plan<'a>(backend: &'a Backend, case: &Members, script: &mut Script) -> Checked<Judge<'a>> {
    let id = string(case, "id");
    let verb = string(case, "verb");
    let expect = member(case, "expect");
    if let Some(kind) = expect["error"]["kind"].as_str() {
        return refused(backend, &id, case, kind, script);
    }
    let base = format!("{}/case/{id}/v1", backend.origin());
    let exchanges = member(case, "exchanges")
        .as_array()
        .cloned()
        .unwrap_or_default();
    let served = format!("{base}/systemone");
    // `find`, `recognize` and `relate` keep request digests until slice 4.
    let keyed = !matches!(verb.as_str(), "find" | "recognize" | "relate");
    let mut renamed = BTreeMap::new();
    for exchange in &exchanges {
        let request = exchange["request"].as_str().unwrap_or_default().as_bytes();
        let now = if keyed {
            json!(keys(&served, request)?)
        } else {
            json!(digest(&served, request))
        };
        renamed.insert(digest(CANONICAL, request), now);
    }
    let success = swap(&expect["success"], &renamed);
    let texts: Vec<String> = exchanges
        .iter()
        .map(|exchange| exchange["evidence"].as_str().unwrap_or_default().to_owned())
        .collect();
    let records = json!(texts).to_string();
    let question = case.get("question").map_or("{}", |raw| raw.get());
    let first = texts.first().cloned().unwrap_or_default();
    match (verb.as_str(), success["kind"].as_str().unwrap_or_default()) {
        ("recognize", _) => Ok(recognized(script, &base, case, question, &success)),
        ("relate", _) => related(script, &base, case, question, &success),
        ("annotate", _) => Ok(annotating(script, &base, case, &records, success)),
        ("find", _) => {
            script.ask("call", &[&base, question]);
            let units: Vec<String> = serde_json::from_str::<Value>(question)
                .ok()
                .and_then(|asked| serde_json::from_value(asked["units"].clone()).ok())
                .unwrap_or_default();
            let want = picked(&units, &success["operation"]["selected"]);
            Ok(Box::new(move |got| same("find", &parsed(&got[0])?, &want)))
        }
        ("rank", _) => {
            let asked =
                serde_json::from_str::<Value>(question).map_err(|error| error.to_string())?;
            let request = json!({"rank": asked["decide"], "records": texts}).to_string();
            script.ask("call", &[&base, &request]);
            let ranking = success["operation"]["ranking"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            let want = ranking
                .iter()
                .map(|row| picked(&texts, &row["index"]))
                .collect();
            Ok(Box::new(move |got| same("rank", &parsed(&got[0])?, &want)))
        }
        (_, "filter") => {
            let request = with(&renamed_verb(question, "filter")?, "records", &records);
            script.ask("call", &[&base, &request]);
            let indexes = success["operation"]["indexes"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            let want = indexes.iter().map(|index| picked(&texts, index)).collect();
            Ok(Box::new(move |got| {
                same("filter", &parsed(&got[0])?, &want)
            }))
        }
        (_, "decide_many") => {
            let mut fields = vec![base.as_str(), question];
            fields.extend(texts.iter().map(String::as_str));
            script.ask("many", &fields);
            let answers = success["answers"].as_array().cloned().unwrap_or_default();
            let want: Vec<Value> = answers
                .iter()
                .map(|answer| answer["bare"].clone())
                .collect();
            Ok(Box::new(move |got| {
                let rows = judged(&got[0])?;
                let outcomes = rows.iter().map(|(outcome, _)| bare(*outcome)).collect();
                same("decide_many", &outcomes, &json!(want))
            }))
        }
        _ => Ok(one_answer(script, &base, &verb, question, &first, &success)),
    }
}

/// Annotate the case's record, when it names one, or its evidence texts.
fn annotating<'a>(
    script: &mut Script,
    base: &str,
    case: &Members,
    records: &str,
    success: Value,
) -> Judge<'a> {
    let set = case.get("question_set").map_or("{}", |raw| raw.get());
    let whole = case
        .get("record")
        .map(|record| json!([record.get()]).to_string());
    let records = whole.as_deref().unwrap_or(records);
    let request = format!(r#"{{"annotate":{set},"records":{records}}}"#);
    script.ask("call", &[base, &request]);
    let one = whole.is_some();
    Box::new(move |got| annotated(&parsed(&got[0])?, &success, one))
}

/// Recognize through the JSON door and the typed door.
fn recognized<'a>(
    script: &mut Script,
    base: &str,
    case: &Members,
    question: &str,
    success: &Value,
) -> Judge<'a> {
    let evidence = string(case, "text");
    let request = with(question, "evidence", &json!(evidence).to_string());
    script.ask("call", &[base, &request]);
    script.ask("recognize", &[base, question, &evidence]);
    let want = success["answers"][0]["bare"].clone();
    Box::new(move |got| {
        same("call", &parsed(&got[0])?, &want)?;
        same("recognize", &parsed(&got[1])?, &want)
    })
}

/// Relate through the JSON door and the typed door.
fn related<'a>(
    script: &mut Script,
    base: &str,
    case: &Members,
    question: &str,
    success: &Value,
) -> Checked<Judge<'a>> {
    let entities = case.get("entities").map_or("[]", |raw| raw.get());
    script.ask("call", &[base, &with(question, "records", entities)]);
    let each: Vec<Box<RawValue>> =
        serde_json::from_str(entities).map_err(|error| error.to_string())?;
    let mut fields = vec![base, question];
    fields.extend(each.iter().map(|one| one.get()));
    script.ask("relate", &fields);
    let want = json!({"edges": success["answers"][0]["bare"]});
    Ok(Box::new(move |got| {
        same("call", &parsed(&got[0])?, &want)?;
        same("relate", &parsed(&got[1])?, &want)
    }))
}

/// A single-answer verb: its bare and detailed replies, the typed decide
/// where it fits, and the usage counters the case names.
fn one_answer<'a>(
    script: &mut Script,
    base: &str,
    verb: &str,
    question: &str,
    first: &str,
    success: &Value,
) -> Judge<'a> {
    let evidence = json!(first).to_string();
    let bare_request = with(question, "evidence", &evidence);
    // Details come first, so they are the text's first send.
    script.ask("call", &[base, &with(&bare_request, "details", "true")]);
    script.ask("call", &[base, &bare_request]);
    let decide = verb == "decide";
    if decide {
        script.ask("decide", &[base, question, first]);
    }
    // The counters run last, on an empty cache folder of their own.
    let counters = success.get("counters").cloned();
    let calls = counters
        .as_ref()
        .and_then(|counters| counters["calls"].as_u64())
        .unwrap_or_default();
    if counters.is_some() {
        let folder = scratch("counters").display().to_string();
        script.ask("env", &["THINKTHEN_CACHE", &folder]);
        script.ask("call", &[base, r#"{"usage":true}"#]);
        for _ in 0..calls {
            script.ask("call", &[base, &bare_request]);
        }
        script.ask("call", &[base, r#"{"usage":true}"#]);
    }
    let expected = success["answers"][0].clone();
    let served = format!("{base}/systemone");
    Box::new(move |got| {
        single(&got[1], &got[0], &expected, &served)?;
        if decide {
            let rows = judged(&got[2])?;
            let (outcome, probability) = rows.first().copied().ok_or("no judgment")?;
            same("decide", &bare(outcome), &expected["bare"])?;
            let yes = &expected["details"]["answer"]["probability"];
            same("probability", &json!(probability), yes)?;
        }
        if let Some(counters) = &counters {
            let before = parsed(&got[2 + usize::from(decide)])?;
            let after = parsed(got.last().ok_or("no usage reply")?)?;
            let moved = |name: &str| {
                after[name]
                    .as_u64()
                    .zip(before[name].as_u64())
                    .map(|(a, b)| a - b)
            };
            let seen = json!({
                "calls": counters["calls"],
                "requests": moved("requests_sent"),
                "cache_answers": moved("cache_answers"),
            });
            same("counters", &seen, counters)?;
        }
        Ok(())
    })
}

/// A single judgment's bare reply and its `details` reply.
fn single(plain: &Reply, detailed: &Reply, expected: &Value, served: &str) -> Checked {
    same("bare", &parsed(plain)?, &expected["bare"])?;
    let details = parsed(detailed)?;
    let wanted = &expected["details"];
    same("value", &details["value"], &expected["bare"])?;
    for (name, value) in wanted["answer"].as_object().into_iter().flatten() {
        same(name, &details["answer"][name], value)?;
    }
    // Indexing a missing key reads null, so an absent field reads as the word "absent".
    let absent = json!("absent");
    let at = |from: &Value, name| from.get(name).unwrap_or(&absent).clone();
    let confidence = |from: &Value| at(&from["answer"], "confidence");
    same("confidence", &confidence(&details), &confidence(wanted))?;
    for name in "model question_sha256 usage requests_sent cached".split(' ') {
        same(name, &at(&details["meta"], name), &at(wanted, name))?;
    }
    // A keyed request stands for the list of its keys, so a row of one
    // request's questions reads the flattened list.
    let requests = match &wanted["requests"] {
        Value::Array(held) => held
            .iter()
            .flat_map(|one| one.as_array().cloned().unwrap_or_else(|| vec![one.clone()]))
            .collect(),
        other => other.clone(),
    };
    same("requests", &at(&details["meta"], "requests"), &requests)?;
    same("url", &details["meta"]["url"], &json!(served))
}

/// Each expected annotate value against its record's value object.
fn annotated(rows: &Value, success: &Value, one: bool) -> Checked {
    for expected in success["answers"].as_array().into_iter().flatten() {
        let record = if one {
            Some(0)
        } else {
            expected["exchange"]
                .as_u64()
                .and_then(|at| usize::try_from(at).ok())
        };
        let row = record
            .and_then(|at| rows.get(at))
            .ok_or("a record is missing")?;
        let name = expected["name"].as_str().unwrap_or_default();
        same(name, &row[name], &expected["bare"])?;
    }
    Ok(())
}

/// A fault case's request, and the code its kind maps to.
fn refused<'a>(
    backend: &'a Backend,
    id: &str,
    case: &Members,
    kind: &str,
    script: &mut Script,
) -> Checked<Judge<'a>> {
    let generic = format!("{}/generic/v1", backend.origin());
    let asked = r#"{"decide":"Does this need attention?"}"#;
    let urgent = "Is this urgent?";
    match id {
        "20-usage-fault" => script.ask("decide", &[&generic, asked, "   "]),
        "21-backend-fault" => {
            script.ask(
                "decide",
                &[
                    &format!("{}/arm/refuse/v1", backend.origin()),
                    asked,
                    urgent,
                ],
            );
        }
        // The engine opens its cache while it is built, so the null engine
        // names this failure.
        "22-local-fault" => {
            let file = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("cases-not-a-folder");
            std::fs::write(&file, "not a folder").map_err(|error| error.to_string())?;
            script.ask("env", &["THINKTHEN_CACHE", &file.display().to_string()]);
            script.ask("decide", &[&generic, asked, urgent]);
            // A good folder builds an engine, which clears the thread's slot.
            let good = scratch("cases-good-cache").display().to_string();
            script.ask("env", &["THINKTHEN_CACHE", &good]);
            script.ask("nullcode", &[&generic, "-"]);
        }
        "23-cancelled-fault" => script.ask("cancelled", &[&generic, asked, urgent]),
        "24-deadline-fault" => script.ask("expired", &[&generic, asked, urgent]),
        "29-usage-json-text" => {
            let question = case.get("question").map_or("{}", |raw| raw.get());
            let evidence = string(case, "evidence");
            script.ask(
                "call",
                &[
                    &generic,
                    &with(question, "evidence", &json!(evidence).to_string()),
                ],
            );
        }
        "30-local-question-file" => {
            let path = scratch("case-30-question-file").join("question.json");
            std::fs::write(&path, case["question"].get()).map_err(|error| error.to_string())?;
            let name = path.to_string_lossy();
            script.ask("file", &[&generic, &name, &string(case, "evidence")]);
        }
        "31-usage-rank-blank-question" => {
            script.ask(
                "call",
                &[&generic, r#"{"rank":"   ","records":["one","two"]}"#],
            );
        }
        other => return Err(format!("no door boundary is written for {other}")),
    }
    let code = [
        "usage",
        "backend",
        "deadline",
        "local",
        "cancelled",
        "defect",
    ]
    .iter()
    .position(|named| *named == kind)
    .and_then(|at| i32::try_from(at + 1).ok())
    .ok_or("an unknown kind")?;
    let sends = matches!(id, "21-backend-fault");
    let file_case = id == "30-local-question-file";
    // After 22's good build, a null engine's code is the usage code again.
    let after: Vec<Reply> = if id == "22-local-fault" {
        vec![(0, "1".to_owned())]
    } else {
        Vec::new()
    };
    let before = backend.count();
    Ok(Box::new(move |got| {
        let (said, message) = got.first().ok_or("no reply")?;
        if *said != code {
            return Err(format!("code {said} ({message}), expected {code}"));
        }
        if file_case && !message.starts_with("0 ") {
            return Err(format!("the named-file refusal was retryable: {message}"));
        }
        if !sends && backend.count() != before {
            return Err("a refusal sent a request".to_owned());
        }
        let rest = got.get(1..).unwrap_or_default();
        if rest != after.as_slice() {
            return Err(format!(
                "after the refusal came {rest:?}, expected {after:?}"
            ));
        }
        Ok(())
    }))
}

/// Parse the driver's `CODE LENGTH` framed replies.
pub(crate) fn replies(output: &[u8]) -> Checked<Vec<Reply>> {
    let mut rest = output;
    let mut all = Vec::new();
    while !rest.is_empty() {
        let line = rest
            .iter()
            .position(|byte| *byte == b'\n')
            .ok_or("a reply has no header")?;
        let header = String::from_utf8_lossy(&rest[..line]).into_owned();
        let (code, length) = header
            .split_once(' ')
            .ok_or("a reply header is malformed")?;
        let code: i32 = code.parse().map_err(|_| "a reply code is not a number")?;
        let length: usize = length
            .parse()
            .map_err(|_| "a reply length is not a number")?;
        let body = rest
            .get(line + 1..line + 1 + length)
            .ok_or("a reply is short")?;
        all.push((code, String::from_utf8_lossy(body).into_owned()));
        rest = rest.get(line + 2 + length..).unwrap_or_default();
    }
    Ok(all)
}

fn parsed((code, body): &Reply) -> Checked<Value> {
    if *code != 0 {
        return Err(format!("code {code}: {body}"));
    }
    let value: Value = serde_json::from_str(body).map_err(|error| format!("{error}: {body}"))?;
    if value.get("facts").is_some() {
        return value
            .get("value")
            .cloned()
            .ok_or("a call wrapper has no value".to_owned());
    }
    Ok(value)
}

/// A judgment reply's `OUTCOME PROBABILITY` pairs.
fn judged(reply: &Reply) -> Checked<Vec<(i32, f64)>> {
    let (code, body) = reply;
    if *code != 0 {
        return Err(format!("code {code}: {body}"));
    }
    let words: Vec<&str> = body.split(' ').collect();
    words
        .chunks(2)
        .map(|pair| match pair {
            [outcome, probability] => Ok((
                outcome.parse().map_err(|_| "an outcome is not a number")?,
                probability
                    .parse()
                    .map_err(|_| "a probability is not a number")?,
            )),
            _ => Err("a judgment is half written".to_owned()),
        })
        .collect()
}

/// The shared cases' bare value for a C outcome.
fn bare(outcome: i32) -> Value {
    match outcome {
        1 => json!(true),
        0 => json!(false),
        _ => Value::Null,
    }
}

fn picked(texts: &[String], index: &Value) -> Value {
    let at = index.as_u64().and_then(|at| usize::try_from(at).ok());
    at.and_then(|at| texts.get(at))
        .map_or(Value::Null, |text| json!(text))
}

/// A question object's written bytes with one more member at the end.
pub(crate) fn with(object: &str, key: &str, value: &str) -> String {
    let open = object.trim_end().strip_suffix('}').unwrap_or(object);
    format!("{open},\"{key}\":{value}}}")
}

/// A `decide` question file asked under another verb's key.
pub(crate) fn renamed_verb(question: &str, verb: &str) -> Checked<String> {
    question
        .trim_start()
        .strip_prefix('{')
        .and_then(|rest| rest.trim_start().strip_prefix(r#""decide""#))
        .map(|rest| format!(r#"{{"{verb}"{rest}"#))
        .ok_or_else(|| format!("the question does not lead with decide: {question}"))
}

/// One member of a case; a missing member reads as null.
pub(crate) fn member(case: &Members, key: &str) -> Value {
    case.get(key)
        .and_then(|raw| serde_json::from_str(raw.get()).ok())
        .unwrap_or_default()
}

pub(crate) fn string(case: &Members, key: &str) -> String {
    member(case, key).as_str().unwrap_or_default().to_owned()
}

fn digest(url: &str, request: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"systemone\n");
    hasher.update(url.as_bytes());
    hasher.update(b"\n");
    hasher.update(request);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Every question key of one request body, in wire order, by ADR 0111
/// section 2: the SHA-256 of the adapter, the URL, the model, the state and
/// one question as the body carries them, joined by line feeds.
pub(crate) fn keys(url: &str, body: &[u8]) -> Checked<Vec<String>> {
    let parts: Members = serde_json::from_slice(body).map_err(|error| error.to_string())?;
    let part = |name: &str| {
        parts
            .get(name)
            .map(|raw| raw.get())
            .ok_or(format!("no {name}"))
    };
    let (model, state) = (part("model")?, part("state")?);
    let questions: Members =
        serde_json::from_str(part("questions")?).map_err(|error| error.to_string())?;
    let mut placed = Vec::new();
    for (name, question) in &questions {
        let place: usize = name[1..]
            .parse()
            .map_err(|_| format!("no qN name: {name}"))?;
        placed.push((place, question.get()));
    }
    placed.sort_by_key(|(place, _)| *place);
    Ok(placed
        .into_iter()
        .map(|(_, question)| {
            let joined = ["systemone", url, model, state, question].join("\n");
            Sha256::digest(joined.as_bytes())
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .collect())
}

fn swap(value: &Value, renamed: &BTreeMap<String, Value>) -> Value {
    match value {
        Value::String(held) => renamed.get(held).cloned().unwrap_or_else(|| json!(held)),
        Value::Array(items) => items.iter().map(|item| swap(item, renamed)).collect(),
        Value::Object(fields) => fields
            .iter()
            .map(|(name, field)| (name.clone(), swap(field, renamed)))
            .collect(),
        other => other.clone(),
    }
}

/// Compare two values, holding numbers to a rounding tolerance.
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
                        .all(|(name, value)| other.get(name).is_some_and(|held| close(value, held)))
            }
            (one, other) => one == other,
        }
    }
    if close(actual, expected) {
        Ok(())
    } else {
        Err(format!("{what}: got {actual}, expected {expected}"))
    }
}

/// ADR 0056: a name `recognize` found carries `text` in place of `name`, and
/// relate reads it as the name. `name` wins when a record holds both.
#[test]
fn relate_reads_what_recognize_found() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("the conformance backend");
    let base = format!("{}/generic/v1", backend.origin());
    let recognize = r#"{"version":1,"recognize":{"kinds":{"person":null}}}"#;
    let relate = r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#;
    let mut script = Script::default();
    script.ask("recognize", &[&base, recognize, "Maria Chen arrived."]);
    let found = replies(&run(&driver, &base, &script.0).stdout).expect("a reply");
    let found = parsed(&found[0]).expect("names");
    let records: Vec<String> = found["entities"]
        .as_array()
        .expect("entities")
        .iter()
        .map(ToString::to_string)
        .collect();
    let named = [
        r#"{"name":"Maria Chen","text":"not this","kind":"person"}"#,
        r#"{"name":"arrived.","kind":"person"}"#,
    ];
    let mut script = Script::default();
    for each in [records.iter().map(String::as_str).collect(), named.to_vec()] {
        script.ask("relate", &[vec![base.as_str(), relate], each].concat());
    }
    let got = replies(&run(&driver, &base, &script.0).stdout).expect("replies");
    let by_text = parsed(&got[0]).expect("edges");
    assert_eq!(by_text, parsed(&got[1]).expect("edges"));
    assert_eq!(by_text["edges"].as_array().map(Vec::len), Some(2));
}
