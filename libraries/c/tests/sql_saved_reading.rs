//! SQL preparation retains saved selections through the shared native request.
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use thinkthen::*;
use thinkthen_host::{complete_native::Prepared, sql_request};

#[allow(
    clippy::unwrap_used,
    reason = "invalid loopback fixtures stop the regression"
)]
fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let answers = request
        .get("questions")
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, q)| {
            let criteria = q.get("criteria").unwrap().as_object().unwrap();
            let winner = if criteria.contains_key("OUT") {
                "OUT"
            } else {
                criteria.keys().next().unwrap()
            };
            let probabilities = criteria
                .keys()
                .map(|key| (key.clone(), json!(u8::from(key == winner))))
                .collect::<serde_json::Map<_, _>>();
            (
                name.clone(),
                json!({"type":"choice","probabilities":probabilities}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}
fn inline(original: &str) -> Result<RequestInput, Error> {
    Ok(RequestInput::Records {
        items: vec![RequestItem {
            original: Some(RequestOriginal::Json {
                value: RawRecord::json(original)?,
            }),
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
            images: vec![],
        }],
    })
}
fn request(
    verb: &str,
    definition: RequestDefinition,
    input: RequestInput,
) -> Result<AdmittedRequest, Error> {
    Request::named(
        verb,
        RequestArguments {
            question: RequestQuestion::Definition { value: definition },
            input,
            options: RequestOptions {
                none: verb == "find",
                ..RequestOptions::default()
            },
        },
    )?
    .admit()
}
#[test]
fn sql_saved_find_keeps_selected_evidence_originals_and_sources()
-> Result<(), Box<dyn std::error::Error>> {
    exercise("find", r#"{"find":"Fits?","on":["/body"]}"#)
}
#[test]
fn sql_saved_recognize_keeps_selected_evidence_and_refuses_decoded_sources()
-> Result<(), Box<dyn std::error::Error>> {
    exercise(
        "recognize",
        r#"{"version":1,"recognize":{"kinds":{"person":null}},"on":["/body"]}"#,
    )
}
fn exercise(verb: &str, authored: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(answer)?;
    let engine = Engine::builder()
        .base_url(listener.base())?
        .api_key("saved-reading-fixture")?
        .model("fixed")?
        .no_cache()
        .max_retries(0)
        .build()?;
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("sql-saved-reading-{verb}-{}", std::process::id()));
    std::fs::create_dir_all(&folder)?;
    let path = folder.join("records.jsonl");
    let original = r#"{"body":"Alpha.","private":"Excluded."}"#;
    std::fs::write(&path, format!("\n{original}\n"))?;
    let prepared = Prepared::content(verb, authored)?;
    for source in [false, true] {
        let admitted = if source {
            request(
                verb,
                sql_request::definition(&prepared),
                RequestInput::Source {
                    source: RequestSource {
                        paths: vec![path.clone()],
                        framing: Some(RequestFraming::Jsonl),
                        reading: ReaderOptions::default(),
                        media: ReaderMedia::Text,
                    },
                },
            )?
        } else {
            request(verb, sql_request::definition(&prepared), inline(original)?)?
        };
        let before = listener.count();
        if source && verb == "recognize" {
            refuse_decoded_source(&engine, &admitted)?;
            assert_eq!(listener.count(), before);
            continue;
        }
        let definition = if verb == "find" {
            Question::find("Fits?")?.into()
        } else {
            Recognize::builder()
                .kind(Kind::new("person", None)?)?
                .build()?
                .into()
        };
        let expected = request(verb, definition, inline(r#""Alpha.""#)?)?;
        assert_eq!(listener.count(), before);
        let RequestOutcome::Complete(done) =
            engine.execute_request(&admitted, RequestEnvironment::default())?
        else {
            return Err("saved request failed".into());
        };
        check_original(done.value(), source.then_some(path.as_path()))?;
        assert_eq!(listener.count(), before + 1);
        assert!(matches!(
            engine.execute_request(&expected, RequestEnvironment::default())?,
            RequestOutcome::Complete(_)
        ));
        let requests = listener.requests();
        assert_eq!(
            requests.first().ok_or("saved body")?.body,
            requests.get(1).ok_or("baseline body")?.body,
            "saved {verb} must send selected evidence only"
        );
        assert_eq!(listener.count(), before + 2);
    }
    let before = listener.count();
    let invalid = request(
        verb,
        sql_request::definition(&prepared),
        inline(r#"{"private":"Excluded."}"#)?,
    )
    .and_then(|missing| engine.execute_request(&missing, RequestEnvironment::default()));
    assert_eq!(
        invalid.err().ok_or("missing field accepted")?.kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), before);
    std::fs::remove_dir_all(folder)?;
    Ok(())
}

fn check_original(
    value: &RequestValue,
    source: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = match value {
        RequestValue::Found(found) => found
            .candidates()
            .first()
            .and_then(|candidate| candidate.input()),
        RequestValue::Recognized(rows) => rows.first().map(CompleteRecord::original),
        _ => None,
    }
    .ok_or("missing original")?;
    let QuestionInput::Record(record) = input else {
        return Err("lost record original".into());
    };
    assert_eq!(
        serde_json::to_value(record.original())?,
        json!({"body":"Alpha.","private":"Excluded."})
    );
    if let Some(path) = source {
        let location = record.location().ok_or("lost source")?;
        assert_eq!(
            (location.file(), location.first_line(), location.last_line()),
            (path.to_str().ok_or("path")?, Some(2), Some(2))
        );
    } else {
        assert!(record.location().is_none());
    }
    Ok(())
}

fn refuse_decoded_source(
    engine: &Engine,
    admitted: &AdmittedRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let error = engine
        .execute_request(admitted, RequestEnvironment::default())
        .err()
        .ok_or("decoded source accepted")?;
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "located recognize takes literal text units, not decoded JSON fields"
    );
    assert_eq!(
        error
            .facts()
            .ok_or("missing refusal facts")?
            .requests_sent(),
        0
    );
    Ok(())
}
