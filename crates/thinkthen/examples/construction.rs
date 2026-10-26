//! Generate finite stub construction cases from admitted canonical requests.
#![allow(
    clippy::disallowed_types,
    reason = "fixture generation carries invalid admission inputs as JSON"
)]
use serde_json::{Value, json};
use std::{collections::BTreeSet, path::Path};
use thinkthen::{BackendName, OffsetUnit, Request, Settings};
fn build(request: Value) -> Result<serde_json::Map<String, Value>, Box<dyn std::error::Error>> {
    let settings = Settings::default().with_backend(BackendName::Stub);
    let mut result = serde_json::Map::from_iter([
        ("client".into(), serde_json::to_value(&settings)?),
        ("request".into(), request.clone()),
        ("requests".into(), json!(0)),
    ]);
    match Request::from_json(&request.to_string()) {
        Ok(request) => {
            let engine = settings.engine(|_| None)?;
            let call = engine.open_request(&request, OffsetUnit::CodePoints)?;
            call.sender().close();
            let rows = call.rows().collect::<Result<Vec<_>, _>>()?;
            result.insert("expected".into(), serde_json::to_value(rows)?);
        }
        Err(error) => {
            result.insert("expected_error".into(), json!(error.code()));
        }
    }
    Ok(result)
}
fn unnamed_variant(
    named: &serde_json::Map<String, Value>,
    name: &str,
) -> Result<serde_json::Map<String, Value>, Box<dyn std::error::Error>> {
    let mut case = named.clone();
    case.insert("name".into(), json!(name));
    for field in ["request", "equivalent_request"] {
        case.get_mut(field)
            .and_then(Value::as_object_mut)
            .ok_or("a named case needs its request")?
            .remove("name");
    }
    // Missing input obtains the canonical source without producing a semantic answer.
    // Every reviewed answer, detail, status and position remains copied from the named case.
    let mut probe = case
        .get("request")
        .ok_or("a case needs its request")?
        .clone();
    probe
        .as_object_mut()
        .ok_or("a request must be an object")?
        .insert("records".into(), json!([null]));
    let probe = build(probe)?;
    let source = probe
        .get("expected")
        .and_then(Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("source"))
        .ok_or("missing input must retain canonical source")?;
    for row in case
        .get_mut("expected")
        .and_then(Value::as_array_mut)
        .ok_or("a construction case needs reviewed answers")?
    {
        row.as_object_mut()
            .ok_or("an expected row must be an object")?
            .insert("source".into(), source.clone());
    }
    Ok(case)
}
fn emit(path: &Path, value: &Value, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = format!("{}\n", serde_json::to_string_pretty(value)?);
    if check {
        if std::fs::read_to_string(path)? != bytes {
            return Err(std::io::Error::other(
                "regenerate canonical construction fixtures with the construction example",
            )
            .into());
        }
    } else {
        std::fs::write(path, bytes)?;
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let check = std::env::args().nth(1).as_deref() == Some("--check");
    let directory = Path::new("conformance/typed");
    let manifest: Value = serde_json::from_slice(&std::fs::read(directory.join("manifest.json"))?)?;
    let mut functions = BTreeSet::new();
    let mut cases = Vec::new();
    for name in manifest
        .get("cases")
        .and_then(Value::as_array)
        .ok_or("the manifest needs cases")?
    {
        let case: Value = serde_json::from_slice(&std::fs::read(
            directory.join(name.as_str().ok_or("a case name must be text")?),
        )?)?;
        let request = case
            .get("request")
            .ok_or("a case needs its request")?
            .clone();
        if functions.insert(
            request
                .get("question")
                .and_then(|question| question.get("function"))
                .and_then(Value::as_str)
                .ok_or("a canonical request needs its function")?
                .to_owned(),
        ) {
            cases.push(build(request)?);
        }
    }
    if functions.len() != thinkthen::FunctionName::ALL.len() {
        return Err("the corpus must cover every function".into());
    }
    emit(Path::new("construction/stub.json"), &json!(cases), check)?;
    let inputs = [
        (
            "named-choose-domain",
            json!({"name":"saved","question":{"function":"choose","question":"Which fits?","choices":["red","blue"]},"records":["alpha","beta"]}),
        ),
        (
            "named-rank-levels",
            json!({"name":"priority","question":{"function":"rank","question":"How much?","levels":["low","high"],"top":1},"records":["alpha","beta"]}),
        ),
        (
            "choose-missing-domain",
            json!({"question":{"function":"choose","question":"Which fits?"}}),
        ),
        (
            "choose-small-domain",
            json!({"question":{"function":"choose","question":"Which fits?","choices":["one"]}}),
        ),
        (
            "tag-rejects-band",
            json!({"question":{"function":"tag","question":"Ready?","labels":["one"],"threshold":{"low":0.2,"high":0.8}}}),
        ),
        (
            "annotation-rejects-rank",
            json!({"question":{"function":"annotate","questions":{"rank":{"function":"rank","question":"How much?","levels":["low","high"]}}}}),
        ),
        (
            "evidence-needs-pointer",
            json!({"question":{"function":"decide","question":"Ready?","evidence":"body"}}),
        ),
    ];
    let mut options = Vec::new();
    let mut unnamed = Vec::new();
    for (name, request) in inputs {
        let mut case = build(request.clone())?;
        case.insert("name".into(), json!(name));
        if case.get("expected_error").is_none() {
            case.insert("equivalent_request".into(), request);
        }
        if let Some(suffix) = name.strip_prefix("named-") {
            unnamed.push(unnamed_variant(&case, &format!("unnamed-{suffix}"))?);
        }
        options.push(case);
    }
    options.extend(unnamed);
    for (function, field) in [
        ("choose", "choices"),
        ("tag", "labels"),
        ("score", "levels"),
        ("rank", "levels"),
        ("recognize", "kinds"),
    ] {
        let mut question = serde_json::Map::from_iter([
            ("function".into(), json!(function)),
            (field.into(), json!(["é", "e\u{301}"])),
        ]);
        if function != "recognize" {
            question.insert("question".into(), json!("Which fits?"));
        }
        let mut case = build(json!({"question": question}))?;
        case.insert(
            "name".into(),
            json!(format!("{function}-rejects-canonically-equal-labels")),
        );
        options.push(case);
    }
    for (name, rules) in [
        (
            "relate-rejects-canonically-equal-rules",
            json!([
                {"relation": "é", "source": "person", "target": "person"},
                {"relation": "e\u{301}", "source": "person", "target": "person"},
            ]),
        ),
        (
            "relate-rejects-canonically-equal-kinds",
            json!([
                {"relation": "knows", "source": "é", "target": "e\u{301}"},
            ]),
        ),
    ] {
        let mut case = build(json!({"question": {"function": "relate", "rules": rules}}))?;
        case.insert("name".into(), json!(name));
        options.push(case);
    }

    emit(
        Path::new("construction/options.json"),
        &json!(options),
        check,
    )
}
