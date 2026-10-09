//! G8: each library JSON method writes the bytes the command prints.
//!
//! Every shared case that asks one whole text with a single judgment, an
//! `annotate` set, `recognize`, or `relate` runs through the compiled command
//! and through the public API on the same case arm. `Details::to_json` must
//! equal the shared judgment members of the `--details` line. `value_json`, `Recognized::to_json`, and each
//! `Edge::to_json` must equal the bare output. Other tests check what the
//! command prints; this one holds the library to the same bytes.

use std::fs;
use std::path::{Path, PathBuf};

use conformance_backend::Backend;
use serde::Deserialize;
use serde_json::value::RawValue;
use thinkthen::{Edge, Engine, Entity, LoadedQuestion, QuestionSet, Recognize, Relate};

use crate::harness::spawn;

const CASES: &str = include_str!("../../../../conformance/cases.json");
const KEY: &str = "sk-public-json";

#[derive(Deserialize)]
struct Cases {
    cases: Vec<Case>,
}

/// One shared case, with the question members kept in written order.
#[derive(Deserialize)]
struct Case {
    id: String,
    verb: String,
    question: Option<Box<RawValue>>,
    question_set: Option<Box<RawValue>>,
    text: Option<String>,
    entities: Option<Box<RawValue>>,
    #[serde(default)]
    exchanges: Vec<Exchange>,
    expect: serde_json::Value,
}

#[derive(Deserialize)]
struct Exchange {
    evidence: Option<String>,
}

#[derive(Deserialize)]
struct Named {
    name: String,
    kind: String,
}

type Compared = Result<usize, String>;

#[test]
#[ignore = "release-only full shared-case parity; run sdlc/scripts/test-full-cases --run"]
fn release_only_each_json_method_prints_the_commands_bytes_on_the_shared_cases() {
    let cases: Cases = serde_json::from_str(CASES).expect("the shared cases");
    let backend = Backend::start().expect("backend");
    let (mut compared, mut failures) = (0, Vec::new());
    for case in cases
        .cases
        .iter()
        .filter(|case| case.expect["error"].is_null())
    {
        match compare(&backend, case) {
            Ok(count) => compared += count,
            Err(why) => failures.push(format!("{}: {why}", case.id)),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
    assert_eq!(compared, 64);
}

/// Compare every text of one case; the count of texts compared.
fn compare(backend: &Backend, case: &Case) -> Compared {
    let base = format!("{}/case/{}/v1", backend.origin(), case.id);
    let engine = Engine::builder()
        .base_url(&base)
        .and_then(|builder| builder.api_key(KEY))
        .map(thinkthen::EngineBuilder::no_cache)
        .and_then(thinkthen::EngineBuilder::build)
        .map_err(|error| error.to_string())?;
    let folder =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("public-json-{}", case.id));
    fs::create_dir_all(&folder).map_err(|error| error.to_string())?;
    let path = folder.join("question.json");
    let asked = case.question.as_ref().or(case.question_set.as_ref());
    let asked = asked.map_or("", |raw| raw.get());
    fs::write(&path, asked).map_err(|error| error.to_string())?;
    let texts = case
        .exchanges
        .iter()
        .filter_map(|held| held.evidence.as_deref());
    match case.verb.as_str() {
        "decide" | "choose" | "tag" | "score" => {
            let question = thinkthen::Question::load(&path).map_err(|error| error.to_string())?;
            texts
                .map(|text| details(&engine, &question, case, &path, &base, text))
                .sum()
        }
        "annotate" if !asked.contains(r#""on""#) => {
            let set = QuestionSet::load(&path).map_err(|error| error.to_string())?;
            texts
                .map(|text| {
                    let mut rows = engine.annotate(&set, [text]);
                    let row = rows.next().ok_or("no row")?.map_err(|e| e.to_string())?;
                    same(
                        row.value_json() + "\n",
                        &command(case, &path, &base, text, false)?,
                    )
                })
                .sum()
        }
        "recognize" => {
            let ask = Recognize::load(&path).map_err(|error| error.to_string())?;
            let text = case.text.as_deref().unwrap_or_default();
            let found = engine.recognize(&ask, text).map_err(|e| e.to_string())?;
            let flags = found.value().relations().unwrap_or_default();
            flagged(
                flags.iter().map(|one| one.either()),
                &found.value().to_json(),
            )?;
            same(
                found.value().to_json() + "\n",
                &command(case, &path, &base, text, false)?,
            )
        }
        "relate" => {
            let ask = Relate::load(&path).map_err(|error| error.to_string())?;
            let given = case.entities.as_ref().map_or("[]", |raw| raw.get());
            let named: Vec<Named> = serde_json::from_str(given).map_err(|e| e.to_string())?;
            let entities = named
                .iter()
                .map(|held| Entity::new(&held.name, &held.kind))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?;
            let edges = engine.relate(&ask, entities).map_err(|e| e.to_string())?;
            let lines: String = edges
                .value()
                .iter()
                .map(|edge| edge.to_json() + "\n")
                .collect();
            flagged(edges.value().iter().map(Edge::either), &lines)?;
            // Each edge counts, so a case whose edges all vanished shows in the total.
            same(lines, &command(case, &path, &base, given, false)?).map(|_| edges.value().len())
        }
        _ => Ok(0),
    }
}

/// Ticket 0344: the typed `either` flags count the JSON's `"either":true`
/// members, so the accessor and the printed edge agree.
fn flagged(flags: impl Iterator<Item = bool>, json: &str) -> Result<(), String> {
    let typed = flags.filter(|flag| *flag).count();
    let printed = json.matches(r#""either":true}"#).count();
    if typed == printed {
        Ok(())
    } else {
        Err(format!(
            "{typed} typed both-ways edges, {printed} printed in\n{json}"
        ))
    }
}

/// Compare every judgment byte after checking the CLI-only input position.
fn details(
    engine: &Engine,
    question: &LoadedQuestion,
    case: &Case,
    path: &Path,
    base: &str,
    text: &str,
) -> Compared {
    let details = match question {
        LoadedQuestion::Question(question) => engine.details(question, text),
        LoadedQuestion::Banded(question) => engine.details(question, text),
    };
    let details = details.map_err(|error| error.to_string())?;
    let legacy: serde_json::Value =
        serde_json::from_str(&details.value().to_json()).map_err(|error| error.to_string())?;
    let printed = command(case, path, base, text, true)?;
    let complete: serde_json::Value =
        serde_json::from_str(&printed).map_err(|error| error.to_string())?;
    if legacy.get("schema").and_then(serde_json::Value::as_str) != Some("thinkthen.result/1")
        || complete.get("schema").and_then(serde_json::Value::as_str) != Some("thinkthen.result/2")
    {
        return Err("the released and complete detail versions were not retained".into());
    }
    thinkthen::AnswerId::new(
        complete
            .get("answer_id")
            .and_then(serde_json::Value::as_str)
            .ok_or("no complete answer ID")?,
    )
    .map_err(|error| error.to_string())?;
    let last = text.bytes().filter(|&b| b == b'\n').count()
        + usize::from(!text.is_empty() && !text.ends_with('\n'));
    if complete.get("position")
        != Some(&serde_json::json!({"file":null,"first":1,"last":last.max(1)}))
    {
        return Err("the complete command has no matching original position".into());
    }
    // Correlation IDs identify distinct live observations. Compare the retained
    // legacy judgment fields, not the independent answers' transient identities.
    versioned_values(case, text, &legacy, &complete)?;
    for key in ["question", "answer", "threshold"] {
        if legacy.get(key) != complete.get(key) {
            return Err(format!("the retained {key} differs"));
        }
    }
    let legacy_meta = legacy
        .get("meta")
        .and_then(serde_json::Value::as_object)
        .ok_or("no legacy meta")?;
    let complete_meta = complete
        .get("meta")
        .and_then(serde_json::Value::as_object)
        .ok_or("no complete meta")?;
    for (key, value) in legacy_meta {
        if complete_meta.get(key) != Some(value) {
            return Err(format!("the retained meta.{key} differs"));
        }
    }
    if complete_meta
        .get("origin")
        .and_then(serde_json::Value::as_str)
        != Some("live")
        || complete_meta
            .get("requests")
            .and_then(serde_json::Value::as_array)
            .ok_or("no requests")?
            .len()
            != complete_meta
                .get("observations")
                .and_then(serde_json::Value::as_array)
                .ok_or("no observations")?
                .len()
    {
        return Err("the actual complete observation alignment differs".into());
    }
    Ok(1)
}

/// Pin released boolean values and complete authored values independently.
fn versioned_values(
    case: &Case,
    text: &str,
    legacy: &serde_json::Value,
    complete: &serde_json::Value,
) -> Result<(), String> {
    let exchange = case
        .exchanges
        .iter()
        .position(|held| held.evidence.as_deref() == Some(text))
        .ok_or("no canonical exchange")?;
    let answers = case
        .expect
        .get("success")
        .and_then(|success| success.get("answers"))
        .and_then(serde_json::Value::as_array)
        .ok_or("no canonical answers")?;
    let answer = answers
        .iter()
        .find(|answer| answer["exchange"].as_u64() == Some(exchange as u64))
        .ok_or("no canonical answer")?;
    let bare = &answer["bare"];
    if &legacy["value"] != bare {
        return Err("the released value differs from the canonical bare answer".into());
    }
    let question: serde_json::Value =
        serde_json::from_str(case.question.as_ref().map_or("{}", |raw| raw.get()))
            .map_err(|error| error.to_string())?;
    let expected = if case.verb == "decide" {
        bare.as_bool()
            .and_then(|yes| question.get(if yes { "true" } else { "false" }))
            .unwrap_or(bare)
    } else {
        bare
    };
    if &complete["value"] != expected {
        return Err(
            "the complete value differs from the independently expected authored answer".into(),
        );
    }
    Ok(())
}

/// The command's standard output for one input.
fn command(
    case: &Case,
    path: &Path,
    base: &str,
    input: &str,
    detailed: bool,
) -> Result<String, String> {
    // `annotate` names its set's path; the other verbs read `@FILE`.
    let at = if case.verb == "annotate" { "" } else { "@" };
    let question = format!("{at}{}", path.display());
    let mut arguments = vec![case.verb.as_str(), &question, "--url", base, "--no-cache"];
    if detailed {
        arguments.push("--details");
    }
    let output = spawn(&arguments, &[("THINKTHEN_API_KEY", KEY)], input.as_bytes())
        .map_err(|error| error.to_string())?;
    if !matches!(output.status.code(), Some(0 | 1 | 3 | 6)) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("exit {:?}: {stderr}", output.status.code()));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn same(library: String, command: &str) -> Compared {
    if library == command {
        Ok(1)
    } else {
        Err(format!(
            "the library wrote\n{library}\nand the command printed\n{command}"
        ))
    }
}
