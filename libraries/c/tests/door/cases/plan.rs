//! How each shared case asks the door, and what its replies must hold.

use conformance_backend::Backend;
use serde_json::value::RawValue;
use serde_json::{Value, json};

use super::wire::{fixture_keys, judged, member, parsed, renamed_verb, same, string, swap, with};
use super::{CANONICAL, Checked, Judge, Members, Reply, Script};
use crate::scratch;

/// Write one case's requests and return the check of their replies.
pub(super) fn plan<'a>(
    backend: &'a Backend,
    case: &Members,
    script: &mut Script,
) -> Checked<Judge<'a>> {
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
    // Every row lists question keys, by ADR 0111.
    let renamed = fixture_keys(CANONICAL, &served, &exchanges)?;
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
            let want = found(&units, &success["operation"]);
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
                .map(|row| {
                    json!({
                        "index": row["index"],
                        "record": picked(&texts, &row["index"]),
                        "probability": row["probability"],
                    })
                })
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
            let file = crate::root().join("cases-not-a-folder");
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

/// The door's `find` value: the selected unit with its place and its
/// probability from the case's candidate list, or null.
fn found(units: &[String], operation: &Value) -> Value {
    let selected = &operation["selected"];
    let chosen = operation["probabilities"]
        .as_array()
        .and_then(|rows| rows.iter().find(|row| &row["index"] == selected));
    match (picked(units, selected), chosen) {
        (Value::Null, _) | (_, None) => Value::Null,
        (unit, Some(row)) => {
            json!({"index": selected, "unit": unit, "probability": row["probability"]})
        }
    }
}
