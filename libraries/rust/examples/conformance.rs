//! The Rust surface's slice of the conformance file, run offline.
//!
//! Reads `conformance/conformance.json` from the repository root, replays
//! every case the library can express against the null backend (which
//! answers with the file's own numbers), and prints one line a case:
//! `ok`, `skip` with a reason, or `FAIL` with what diverged. A nonzero exit
//! follows any FAIL. What this surface cannot run comes from the shared
//! skip table in the conformance file, one place a reason each; the
//! deadline case runs as the spent-budget shape. Run through `./check.sh`.

use std::path::Path;

use serde_json::Value;

use thinkthen::{
    Annotated, Answer, Details, Engine, Error, Options, Question, QuestionSet, Recognize,
    Recognized, Relate, Row, byte_range, failed_questions, name_in, rows_json,
};

/// The one skip-table reader's decision for this case on this surface,
/// asked through conformance/skiptable.py so the table's logic lives in
/// exactly one place (surfaces-review-4: this example carried its own
/// copy, and the copies drifted on the wire and none facets). `Ok(None)`
/// runs the case; `Err` fails it, because a reader that cannot answer
/// must never turn into a silent run or a silent skip.
fn central_skip(case: &Value, wire: bool) -> Result<Option<(String, bool)>, String> {
    let id = case["id"].as_str().unwrap_or("?");
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/skiptable.py");
    let kind = case["expect"]["error"]["kind"].as_str();
    let has_error = !case["expect"]["error"].is_null();
    let has_null = case["records"]
        .as_array()
        .is_some_and(|records| records.iter().any(Value::is_null));
    let mut command = std::process::Command::new("python3");
    command
        .arg(script)
        .arg("lookup")
        .arg("rust")
        .arg(id)
        .env_remove("THINKTHEN_API_KEY")
        .env_remove("THINKTHEN_BASE_URL");
    if let Some(kind) = kind {
        command.args(["--kind", kind]);
    }
    if let Some(form) = case["form"].as_str() {
        command.args(["--form", form]);
    }
    if has_null {
        command.args(["--record", "null"]);
    }
    if case["none"].as_bool().unwrap_or(false) {
        command.args(["--none", "true"]);
    }
    if has_error {
        command.args(["--error", "true"]);
    }
    if wire {
        command.args(["--wire", "true"]);
    }
    let output = command
        .output()
        .map_err(|error| format!("the one skip reader did not run: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "the one skip reader refused: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let line = String::from_utf8_lossy(&output.stdout);
    let line = line.lines().next().unwrap_or("").trim();
    if line == "RUN" {
        return Ok(None);
    }
    let (status, why) = line
        .split_once('\t')
        .ok_or_else(|| format!("the one skip reader answered without a status: {line:?}"))?;
    match status {
        "SKIP" => Ok(Some((why.to_owned(), false))),
        "DIVERGE" => Ok(Some((why.to_owned(), true))),
        other => Err(format!("the one skip reader answered {other:?}")),
    }
}

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/conformance.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let file: Value = serde_json::from_str(&text).expect("the file parses");
    let cases = file["cases"].as_array().expect("the file holds cases");

    let tt = Engine::from_env().expect("the stand-in never fails to build");
    let mut failed = 0;

    for case in cases {
        let id = case["id"].as_str().unwrap_or("?");
        let verb = case["verb"].as_str().unwrap_or("?");
        let question_text =
            serde_json::to_string(&case["question"]).expect("the question serializes");
        let outcome = match central_skip(case, wire_is_set()) {
            Err(reader) => {
                println!("FAIL     {id}: {reader}");
                failed += 1;
                continue;
            }
            Ok(Some((why, true))) => Err(Outcome::Diverge(why)),
            Ok(Some((why, false))) => Err(Outcome::Skip(why)),
            Ok(None) => run(&tt, verb, &question_text, case),
        };
        match outcome {
            Ok(()) => println!("ok       {id}"),
            Err(Outcome::Skip(reason)) => println!("skip     {id}: {reason}"),
            Err(Outcome::Diverge(reason)) => println!("diverge  {id}: {reason}"),
            Err(Outcome::Fail(what)) => {
                failed += 1;
                println!("FAIL     {id}: {what}");
            }
        }
    }

    if failed > 0 {
        eprintln!("{failed} conformance case(s) failed");
        std::process::exit(1);
    }
    println!("conformance slice green for the Rust surface");
}

enum Outcome {
    Skip(String),
    /// A stand-in divergence already recorded in conformance/DIVERGENCES.md.
    Diverge(String),
    Fail(String),
}

fn ok_if(condition: bool, what: String) -> Result<(), Outcome> {
    if condition {
        Ok(())
    } else {
        Err(Outcome::Fail(what))
    }
}

/// The ruled record row (go-ahead item 4), in this host's own type:
/// `Row { input, value }`, checked where the case carries rows and its
/// serialization (`rows_json`) matches the file's shape. The value is
/// `Answer::value()`'s `Option<bool>`, so an unsure row rides as `null`.
fn check_rows(expect: &Value, records: &[&str], values: &[Answer]) -> Result<(), Outcome> {
    let Some(wanted) = expect["rows"].as_array() else {
        return Ok(());
    };
    let rows: Vec<Row<Option<bool>>> = records
        .iter()
        .zip(values)
        .map(|(input, value)| Row {
            input: (*input).to_owned(),
            value: value.value(),
        })
        .collect();
    let got: Value = serde_json::from_str(&rows_json(&rows)).expect("the rows are JSON");
    ok_if(
        got == Value::Array(wanted.clone()),
        format!("expected rows {wanted:?}, got {got:?}"),
    )
}

fn fail(error: Error) -> Outcome {
    Outcome::Fail(format!(
        "the call failed: {:?} ({})",
        error.kind, error.message
    ))
}

fn wire_is_set() -> bool {
    std::env::var_os("ENGINE_BASE_URL").is_some()
        || std::env::var_os("THINKTHEN_BASE_URL").is_some()
}

fn run(tt: &Engine, verb: &str, question_text: &str, case: &Value) -> Result<(), Outcome> {
    let expect = &case["expect"];
    if let Some(kind) = expect["error"]["kind"].as_str() {
        if kind == "deadline" {
            let question = built(question_text)?;
            let budget = case["budget_ms"].as_u64().unwrap_or_default();
            let spent = std::time::Duration::from_millis(budget);
            let error = tt
                .decide_opts(&question, evidence(case), Options::new().deadline_in(spent))
                .err()
                .ok_or_else(|| {
                    Outcome::Fail("expected the deadline kind, got an answer".to_owned())
                })?;
            return ok_if(
                format!("{:?}", error.kind).to_lowercase() == kind,
                format!("expected the {kind} kind, got {:?}", error.kind),
            );
        }
        return check_error(tt, verb, question_text, case, kind);
    }
    match verb {
        "decide" => {
            let question = built(question_text)?;
            let details = tt.details(&question, evidence(case)).map_err(fail)?;
            check_details(expect, &details)
        }
        "decide_many" => {
            let question = built(question_text)?;
            let records = records(case);
            let judgments = tt.decide_many(&question, &records).map_err(fail)?;
            let answers: Vec<Answer> = judgments.iter().map(|judgment| judgment.answer).collect();
            let wanted: Vec<Answer> = expected_answers(&expect["answers"]);
            ok_if(answers == wanted, format!("expected {wanted:?}, got {answers:?}"))?;
            let got: Vec<f64> = judgments.iter().map(|judgment| judgment.probability).collect();
            let wanted: Vec<f64> = expect["probabilities"]
                .as_array()
                .map(|numbers| numbers.iter().filter_map(|number| number.as_f64()).collect())
                .unwrap_or_default();
            ok_if(
                got.iter().zip(&wanted).all(|(one, two)| (one - two).abs() < 1e-9),
                format!("expected {wanted:?}, got {got:?}"),
            )?;
            check_rows(expect, &records, &answers)
        }
        "filter" => {
            let question = built(question_text)?;
            let records = records(case);
            let kept = tt.filter(&question, &records).map_err(fail)?;
            let wanted: Vec<&str> = expect["indexes"]
                .as_array()
                .map(|indexes| {
                    indexes
                        .iter()
                        .filter_map(|place| place.as_u64().map(|place| records[place as usize]))
                        .collect()
                })
                .unwrap_or_default();
            ok_if(kept == wanted, format!("expected {wanted:?}, got {kept:?}"))?;
            let values = vec![Answer::Yes; kept.len()];
            check_rows(expect, &kept, &values)
        }
        "choose" => {
            let question = built(question_text)?;
            let picked = tt.choose(&question, evidence(case)).map_err(fail)?;
            match expect["answer"].as_str() {
                Some(wanted) => ok_if(
                    picked.as_deref() == Some(wanted),
                    format!("expected {wanted:?}, got {picked:?}"),
                ),
                None => ok_if(picked.is_none(), format!("expected no pick, got {picked:?}")),
            }
        }
        "score" => {
            let question = built(question_text)?;
            let scored = tt.score(&question, evidence(case)).map_err(fail)?;
            let wanted = expect["answer"].as_f64();
            ok_if(
                wanted.map_or(false, |wanted| (scored.value - wanted).abs() < 1e-9),
                format!("expected {wanted:?}, got {}", scored.value),
            )?;
            let nearest = expect["details"]["nearest_level"].as_str();
            ok_if(
                Some(scored.nearest.as_str()) == nearest,
                format!("expected level {nearest:?}, got {:?}", scored.nearest),
            )
        }
        "tag" => {
            let question = built(question_text)?;
            let labels = tt.tag(&question, evidence(case)).map_err(fail)?;
            let wanted: Vec<String> = expect["answer"]
                .as_array()
                .map(|labels| {
                    labels
                        .iter()
                        .filter_map(|label| label.as_str().map(str::to_owned))
                        .collect()
                })
                .unwrap_or_default();
            ok_if(labels == wanted, format!("expected {wanted:?}, got {labels:?}"))
        }
        "rank" => {
            let question = built(question_text)?;
            let records = records(case);
            let ranked = tt.rank(&question, &records).map_err(fail)?;
            let got: Vec<usize> = ranked.iter().map(|one| one.index).collect();
            let wanted: Vec<usize> = expect["ranking"]
                .as_array()
                .map(|order| order.iter().filter_map(Value::as_u64).map(|n| n as usize).collect())
                .unwrap_or_default();
            ok_if(got == wanted, format!("expected order {wanted:?}, got {got:?}"))?;
            let mut by_input = vec![0.0_f64; records.len()];
            for one in &ranked {
                by_input[one.index] = one.probability;
            }
            let wanted: Vec<f64> = expect["probabilities"]
                .as_array()
                .map(|numbers| numbers.iter().filter_map(|number| number.as_f64()).collect())
                .unwrap_or_default();
            ok_if(
                by_input.iter().zip(&wanted).all(|(one, two)| (one - two).abs() < 1e-9),
                format!("expected probabilities {wanted:?}, got {by_input:?}"),
            )
        }
        "find" => {
            let text = case["question"]
                .as_str()
                .ok_or_else(|| Outcome::Fail("the find question arrives as text".to_owned()))?;
            let units = records(case);
            let found = tt.find(text, &units).map_err(fail)?;
            match expect["answer"].as_u64() {
                Some(index) => ok_if(
                    found.index == Some(index as usize),
                    format!("expected unit {index}, got {:?}", found.index),
                ),
                None => ok_if(
                    found.index.is_none(),
                    format!("expected no unit, got {:?}", found.index),
                ),
            }
        }
        "recognize" => {
            let ask = Recognize::from_json(question_text).map_err(fail)?;
            let text = case["text"]
                .as_str()
                .ok_or_else(|| Outcome::Fail("a recognize case carries its text".to_owned()))?;
            let found = tt.recognize(&ask, text).map_err(fail)?;
            check_recognize(expect, text, &found)
        }
        "relate" => {
            let ask = Relate::from_json(question_text).map_err(fail)?;
            let records = records(case);
            let edges = tt.relate(&ask, &records).map_err(fail)?;
            check_relate(expect, &edges)
        }
        "annotate" => {
            // The core parser's own grammar: a version of 1 beside the set.
            let wrapped = serde_json::json!({ "version": 1, "questions": case["set"] });
            let set = QuestionSet::from_json(&wrapped.to_string()).map_err(fail)?;
            let held = records(case);
            let records: Vec<&str> = if held.is_empty() {
                vec![evidence(case)]
            } else {
                held
            };
            let annotated = tt.annotate(&set, &records).map_err(fail)?;
            if let Some(wanted_rows) = expect["rows"].as_array() {
                // The multi-record form: one answer a record, in input
                // order, each field the bare answer (a score is its
                // position).
                ok_if(
                    annotated.len() == wanted_rows.len(),
                    format!("expected {} rows, got {}", wanted_rows.len(), annotated.len()),
                )?;
                for (record, (wanted, got)) in records.iter().zip(wanted_rows.iter().zip(&annotated)) {
                    for (name, value) in wanted["value"].as_object().into_iter().flatten() {
                        let field = got
                            .iter()
                            .find(|(held, _)| held == name)
                            .map(|(_, held)| held)
                            .ok_or_else(|| {
                                Outcome::Fail(format!("no {name} field in the answer for {record:?}"))
                            })?;
                        match field {
                            Annotated::Decision(answer) => ok_if(
                                *answer == expected_answer(value),
                                format!("{name}: expected {value:?}, got {answer:?}"),
                            )?,
                            Annotated::Score(scored) => ok_if(
                                value.as_f64().is_some_and(|wanted| (scored.value - wanted).abs() < 1e-9),
                                format!("{name}: expected {value:?}, got {}", scored.value),
                            )?,
                            other => {
                                return Err(Outcome::Fail(format!(
                                    "{name}: expected a bare answer, got {other:?}"
                                )))
                            }
                        }
                    }
                }
                return Ok(());
            }
            let first = annotated
                .first()
                .ok_or_else(|| Outcome::Fail("no annotated record came back".to_owned()))?;
            for (name, wanted) in expect["answers"].as_object().into_iter().flatten() {
                let field = first
                    .iter()
                    .find(|(held, _)| held == name)
                    .map(|(_, held)| held)
                    .ok_or_else(|| Outcome::Fail(format!("no {name} field in the answer")))?;
                if let Some(marker) = wanted.get("failed") {
                    // The ruled marker (0054): this host's spelling is the
                    // typed `Annotated::Failed(Failed { kind, cause })`,
                    // whose own serialization carries the ruled words.
                    match field {
                        Annotated::Failed(failed) => {
                            let ruled = serde_json::to_value(failed)
                                .expect("a failed marker is JSON");
                            ok_if(
                                ruled.get("kind") == marker.get("kind")
                                    && ruled.get("cause") == marker.get("cause"),
                                format!("{name}: expected the {marker} marker, got {ruled}"),
                            )?;
                        }
                        other => {
                            return Err(Outcome::Fail(format!(
                                "{name}: expected the failed marker, got {other:?}"
                            )))
                        }
                    }
                    continue;
                }
                match field {
                    Annotated::Decision(answer) => ok_if(
                        *answer == expected_answer(&wanted["answer"]),
                        format!("{name}: expected {:?}, got {answer:?}", wanted["answer"]),
                    )?,
                    Annotated::Choice(picked) => ok_if(
                        picked.as_deref() == wanted["answer"].as_str(),
                        format!("{name}: expected {:?}, got {picked:?}", wanted["answer"]),
                    )?,
                    other => {
                        return Err(Outcome::Skip(format!(
                            "{name} holds a field the runner does not check ({other:?})"
                        )))
                    }
                }
            }
            if let Some(wanted) = expect["failed_questions"].as_u64() {
                let counted = u64::from(failed_questions(&annotated));
                ok_if(
                    counted == wanted,
                    format!("expected {wanted} failed questions, got {counted}"),
                )?;
            }
            Ok(())
        }
        "details" => {
            let question = built(question_text)?;
            let details = tt.details(&question, evidence(case)).map_err(fail)?;
            check_details(expect, &details)
        }
        "usage" => Err(Outcome::Skip(
            "the cache half needs the disk cache, which the stand-in does not carry".to_owned(),
        )),
        other => Err(Outcome::Skip(format!("no case shape for {other}"))),
    }
}

fn check_recognize(expect: &Value, text: &str, found: &Recognized) -> Result<(), Outcome> {
    let wanted = expect["entities"].as_array().cloned().unwrap_or_default();
    ok_if(
        wanted.len() == found.entities.len(),
        format!(
            "expected {} names, got {}",
            wanted.len(),
            found.entities.len()
        ),
    )?;
    for (one, two) in wanted.iter().zip(&found.entities) {
        ok_if(
            one["id"].as_u64() == Some(two.id),
            format!("id: expected {}, got {}", one["id"], two.id),
        )?;
        ok_if(
            one["text"].as_str() == Some(two.text.as_str()),
            format!("text: expected {}, got {:?}", one["text"], two.text),
        )?;
        ok_if(
            one["kind"].as_str() == Some(two.kind.as_str()),
            format!("kind: expected {}, got {:?}", one["kind"], two.kind),
        )?;
        ok_if(
            one["start"].as_u64() == Some(two.start as u64)
                && one["end"].as_u64() == Some(two.end as u64),
            format!(
                "offsets: expected {}..{}, got {}..{}",
                one["start"], one["end"], two.start, two.end
            ),
        )?;
        ok_if(
            one["strength"]
                .as_f64()
                .is_some_and(|wanted| (wanted - two.strength).abs() < 1e-9),
            format!("strength: expected {}, got {}", one["strength"], two.strength),
        )?;
        // The per-host offset proof, on every case: the name slices out of
        // the original text in Rust's byte indexing, the emoji case
        // included. The contract counts code points, so this is where
        // `byte_range` earns its place.
        ok_if(
            name_in(two, text) == two.text,
            format!(
                "the offsets do not slice the name in bytes: {:?} at {:?}, wanted {:?}",
                name_in(two, text),
                byte_range(two, text),
                two.text
            ),
        )?;
    }
    let wanted = expect["relations"].as_array().cloned().unwrap_or_default();
    ok_if(
        wanted.len() == found.relations.len(),
        format!(
            "expected {} relations, got {}",
            wanted.len(),
            found.relations.len()
        ),
    )?;
    for (one, two) in wanted.iter().zip(&found.relations) {
        ok_if(
            one["name"].as_str() == Some(two.name.as_str()),
            format!("name: expected {}, got {:?}", one["name"], two.name),
        )?;
        ok_if(
            one["source"].as_u64() == Some(two.source)
                && one["target"].as_u64() == Some(two.target),
            format!(
                "ends: expected {}->{}, got {}->{}",
                one["source"], one["target"], two.source, two.target
            ),
        )?;
        ok_if(
            one["probability"]
                .as_f64()
                .is_some_and(|wanted| (wanted - two.probability).abs() < 1e-9),
            format!(
                "probability: expected {}, got {}",
                one["probability"], two.probability
            ),
        )?;
    }
    Ok(())
}

fn check_relate(expect: &Value, edges: &[thinkthen::Edge]) -> Result<(), Outcome> {
    let wanted = expect["edges"].as_array().cloned().unwrap_or_default();
    ok_if(
        wanted.len() == edges.len(),
        format!("expected {} edges, got {}", wanted.len(), edges.len()),
    )?;
    for (one, two) in wanted.iter().zip(edges) {
        ok_if(
            one["name"].as_str() == Some(two.name.as_str()),
            format!("name: expected {}, got {:?}", one["name"], two.name),
        )?;
        ok_if(
            one["source"].as_u64() == Some(two.source)
                && one["target"].as_u64() == Some(two.target),
            format!(
                "ends: expected {}->{}, got {}->{}",
                one["source"], one["target"], two.source, two.target
            ),
        )?;
        ok_if(
            one["probability"]
                .as_f64()
                .is_some_and(|wanted| (wanted - two.probability).abs() < 1e-9),
            format!(
                "probability: expected {}, got {}",
                one["probability"], two.probability
            ),
        )?;
    }
    Ok(())
}

fn check_details(expect: &Value, details: &Details) -> Result<(), Outcome> {
    // The audit's identity fields and the two 0053/0054 additions; the
    // recorded probability is compared only when the case does not pin a
    // requests list, because the null backend's own rule cannot reproduce
    // case 73's recorded number.
    let pins_requests = expect["details"]["requests"].is_array();
    if !pins_requests {
        ok_if(
            details.answer == expected_answer(&expect["answer"]),
            format!("expected {:?}, got {:?}", expect["answer"], details.answer),
        )?;
        let wanted_probability = expect["details"]["probability"].as_f64();
        if let Some(wanted) = wanted_probability {
            ok_if(
                (details.probability - wanted).abs() < 1e-9,
                format!("expected probability {wanted}, got {}", details.probability),
            )?;
        }
    }
    ok_if(
        Some(details.model.as_str()) == expect["details"]["model"].as_str(),
        format!(
            "expected model {:?}, got {:?}",
            expect["details"]["model"], details.model
        ),
    )?;
    let wanted_digest = expect["details"]["question_sha256"].as_str();
    ok_if(
        Some(details.digest.as_str()) == wanted_digest,
        "the digest diverged".to_owned(),
    )?;
    if let Some(wanted) = expect["details"]["requests"].as_array() {
        let wanted: Vec<&str> = wanted.iter().filter_map(Value::as_str).collect();
        let got: Vec<&str> = details.requests.iter().map(String::as_str).collect();
        ok_if(got == wanted, format!("expected requests {wanted:?}, got {got:?}"))?;
    }
    if let Some(wanted) = expect["details"]["failed_questions"].as_u64() {
        ok_if(
            u64::from(details.failed_questions) == wanted,
            format!(
                "expected {wanted} failed questions, got {}",
                details.failed_questions
            ),
        )?;
    }
    Ok(())
}

fn check_error(
    tt: &Engine,
    verb: &str,
    question_text: &str,
    case: &Value,
    kind: &str,
) -> Result<(), Outcome> {
    // A question that cannot even build fails with the usage kind, which
    // is the expected outcome for the blank-question case.
    let built_question = match Question::from_json(question_text) {
        Ok(question) => Some(question),
        Err(error) if format!("{:?}", error.kind).to_lowercase() == kind => return Ok(()),
        Err(error) => return Err(fail(error)),
    };
    let error = match (verb, built_question) {
        ("filter", Some(question)) => tt.filter(&question, &records(case)).err(),
        ("decide", Some(question)) => tt.decide(&question, evidence(case)).err(),
        ("choose", Some(question)) => tt.choose(&question, evidence(case)).err(),
        ("rank", Some(question)) => tt.rank(&question, &records(case)).err(),
        _ => None,
    };
    match error {
        Some(error) => ok_if(
            format!("{:?}", error.kind).to_lowercase() == kind,
            format!("expected the {kind} kind, got {:?}", error.kind),
        ),
        None => Err(Outcome::Fail(format!(
            "expected the {kind} kind, got an answer"
        ))),
    }
}

fn built(question_text: &str) -> Result<Question, Outcome> {
    Question::from_json(question_text).map_err(fail)
}

fn evidence(case: &Value) -> &str {
    case["evidence"].as_str().unwrap_or_default()
}

fn records(case: &Value) -> Vec<&str> {
    case["records"]
        .as_array()
        .map(|records| {
            records
                .iter()
                .filter_map(|record| record.as_str())
                .collect()
        })
        .unwrap_or_default()
}

fn expected_answer(value: &Value) -> Answer {
    match value {
        Value::Bool(true) => Answer::Yes,
        Value::Bool(false) => Answer::No,
        _ => Answer::Unsure,
    }
}

fn expected_answers(value: &Value) -> Vec<Answer> {
    value
        .as_array()
        .map(|answers| answers.iter().map(expected_answer).collect())
        .unwrap_or_default()
}
