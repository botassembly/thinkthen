//! Execute the shared original assets and deterministic admission constructions.
#![expect(
    clippy::unwrap_used,
    reason = "malformed saved fixtures or loopback setup fail this test, never production input"
)]
use super::{Color, engine, fixture, input, questions, url};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use std::io::Cursor;
use thinkthen::{
    BatchSetting, CallOptions, Engine, Error, ErrorKind, ImageInput, ImageMedia, InputFileReader,
    InputReaderOptions, Question, QuestionInput, ReaderMedia, ReaderOptions, SourceUnit,
};

fn corpus() -> Value {
    serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap()
}
fn bytes(path: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path),
    )
    .unwrap()
}
fn setup(listener: &Listener, profile: &Value) -> Engine {
    if let Some(path) = profile["profile_file"].as_str() {
        return Engine::builder()
            .backend("llamacpp")
            .unwrap()
            .base_url(listener.base())
            .unwrap()
            .model(profile["model"].as_str().unwrap())
            .unwrap()
            .profile_json(&String::from_utf8(bytes(path)).unwrap())
            .unwrap()
            .api_key("fake-image-key")
            .unwrap()
            .no_cache()
            .max_retries(0)
            .build()
            .unwrap();
    }
    let backend = if profile["model"] == "d1" {
        "liquid"
    } else {
        "perplexity"
    };
    engine(listener, backend, profile["model"].as_str().unwrap())
}
fn saved_reply(profile: &str, role: &str) -> String {
    let route = if profile == "perplexity-decider" {
        "perplexity"
    } else {
        "liquid"
    };
    String::from_utf8(bytes(&format!(
        "specification/fixtures/images/{route}-{role}-reply.json"
    )))
    .unwrap()
}
fn text(construction: &Value, role: &str) -> Option<String> {
    let recipe = construction.get("text_recipe")?;
    let counts = recipe["by_function"].get(role).unwrap();
    Some(format!(
        "{}{}{}",
        recipe["prefix"].as_str().unwrap(),
        recipe["repeat_token"]
            .as_str()
            .unwrap()
            .repeat(counts["repeat_count"].as_u64().unwrap() as usize),
        "x".repeat(counts["ascii_tail_bytes"].as_u64().unwrap() as usize)
    ))
}
fn originals(construction: &Value) -> Result<Vec<ImageInput>, Error> {
    if let Some(paths) = construction["images"].as_array() {
        return Ok(paths
            .iter()
            .map(|path| fixture(&bytes(path.as_str().unwrap()), ImageMedia::Png))
            .collect());
    }
    let raw = bytes(construction["existing_image"].as_str().unwrap());
    let media = construction["declared_media"]
        .as_str()
        .or_else(|| construction["media"].as_str())
        .unwrap_or("image/png");
    let media = match media {
        "image/png" => ImageMedia::Png,
        "image/jpeg" => ImageMedia::Jpeg,
        _ => {
            // Unsupported formats enter through the actual native image reader, whose type is closed.
            let mut reader = InputFileReader::new(
                "fixture",
                Cursor::new(raw),
                InputReaderOptions {
                    media: ReaderMedia::Image,
                    reading: ReaderOptions {
                        unit: SourceUnit::File,
                        window: None,
                    },
                },
            )?;
            return reader
                .next()
                .unwrap()
                .map(|_| unreachable!("unsupported format must refuse"));
        }
    };
    let image = ImageInput::new(media, raw)?;
    if let Some(dimensions) = construction["dimensions"].as_array() {
        assert_eq!(
            [u64::from(image.width()), u64::from(image.height())],
            [
                dimensions.first().unwrap().as_u64().unwrap(),
                dimensions.get(1).unwrap().as_u64().unwrap()
            ]
        );
    }
    Ok(vec![
        image;
        construction["repeat"].as_u64().unwrap_or(1) as usize
    ])
}
fn evidence(construction: &Value, role: &str) -> Result<QuestionInput, Error> {
    let caption = text(construction, role);
    Ok(QuestionInput::Images(thinkthen::ImageEvidence::new(
        caption,
        originals(construction)?,
    )?))
}
fn scalar(engine: &Engine, role: &str, evidence: &QuestionInput) -> Result<(), Error> {
    match role {
        "decide" => {
            assert_eq!(
                engine.decide_input(&super::question(), evidence)?.value(),
                &thinkthen::Answer::Yes
            );
        }
        "choose" => {
            let question = Question::choose::<Color>("Which color?")
                .unwrap()
                .option(Color::Red, None)
                .unwrap()
                .option(Color::Blue, None)
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(
                engine.choose_input(&question, evidence)?.value(),
                &Some(Color::Red)
            );
        }
        "score" => {
            let question = Question::score("How red?")
                .unwrap()
                .level("none", None)
                .unwrap()
                .level("all", None)
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(*engine.score_input(&question, evidence)?.value(), 0.8);
        }
        _ => unreachable!(),
    }
    Ok(())
}
fn typed_many(
    engine: &Engine,
    role: &str,
    inputs: [QuestionInput; 2],
) -> Result<Vec<QuestionInput>, Error> {
    let options = CallOptions::new().batch(BatchSetting::Max);
    match role {
        "decide" => engine
            .decide_input_many_with(&super::question(), inputs, options)
            .map(|row| {
                let row = row?;
                assert_eq!(row.value(), &thinkthen::Answer::Yes);
                Ok(row.input().clone())
            })
            .collect(),
        "choose" => {
            let question = Question::choose::<Color>("Which color?")
                .unwrap()
                .option(Color::Red, None)
                .unwrap()
                .option(Color::Blue, None)
                .unwrap()
                .build()
                .unwrap();
            engine
                .choose_input_many_with(&question, inputs, options)
                .map(|row| {
                    let row = row?;
                    assert_eq!(row.value(), &Some(Color::Red));
                    Ok(row.input().clone())
                })
                .collect()
        }
        "score" => {
            let question = Question::score("How red?")
                .unwrap()
                .level("none", None)
                .unwrap()
                .level("all", None)
                .unwrap()
                .build()
                .unwrap();
            engine
                .score_input_many_with(&question, inputs, options)
                .map(|row| {
                    let row = row?;
                    assert_eq!(*row.value(), 0.8);
                    Ok(row.input().clone())
                })
                .collect()
        }
        _ => unreachable!(),
    }
}
#[path = "shared_admission/wire.rs"]
mod wire;
use wire::expected_body;

fn assert_request(
    request: &conformance_backend::Recorded,
    profile: &Value,
    role: &str,
    construction: &Value,
    evidence: &QuestionInput,
) {
    assert_eq!(
        serde_json::from_slice::<Value>(&request.body).unwrap(),
        expected_body(profile, role, evidence)
    );
    if let Some(target) = construction["final_serialized_body_bytes"].as_u64() {
        assert_eq!(request.body.len(), target as usize);
    }
    if let Some(min) = construction["final_body_bytes_min"].as_u64() {
        assert!(request.body.len() >= min as usize);
        assert!(
            bytes(construction["existing_image"].as_str().unwrap()).len()
                >= construction["compressed_bytes_min"].as_u64().unwrap() as usize
        );
    }
}
#[test]
fn shared_image_assets_execute_all_scalar_admission_categories_with_counted_sends() {
    let corpus = corpus();
    let parity = &corpus["parity"];
    for (category, scenario) in parity["image_admission_scenarios"]
        .as_object()
        .unwrap()
        .iter()
        .flat_map(|(category, scenarios)| {
            scenarios
                .as_array()
                .unwrap()
                .iter()
                .map(move |scenario| (category, scenario))
        })
    {
        if scenario["construction"]["mode"] == "many inputs" {
            continue;
        }
        for role in ["decide", "choose", "score"] {
            let reply = saved_reply(scenario["profile_ref"].as_str().unwrap(), role);
            let listener = Listener::answering(move |_| Canned::ok(&reply)).unwrap();
            let profile = &parity["image_profiles"][scenario["profile_ref"].as_str().unwrap()];
            let engine = setup(&listener, profile);
            let construction = &scenario["construction"];
            let evidence = evidence(construction, role);
            let result = evidence.as_ref().map_or_else(
                |error| Err(error.kind()),
                |input| scalar(&engine, role, input).map_err(|error| error.kind()),
            );
            if scenario["expect"]["requests"] == 1 {
                assert_eq!(result, Ok(()), "{category}/{} {role}", scenario["id"]);
                assert_eq!(listener.count(), 1);
                assert_request(
                    &listener.requests()[0],
                    profile,
                    role,
                    construction,
                    evidence.as_ref().unwrap(),
                );
            } else {
                assert_eq!(
                    result.unwrap_err(),
                    ErrorKind::Usage,
                    "{category}/{} {role}",
                    scenario["id"]
                );
                assert_eq!(listener.count(), 0, "{category}/{} {role}", scenario["id"]);
            }
        }
    }
}

#[test]
fn complete_image_questions_keep_distinct_states_and_coalesce_identical_questions() {
    let corpus = corpus();
    let parity = &corpus["parity"];
    for scenario in parity["image_admission_scenarios"]["packing-overflow"]
        .as_array()
        .unwrap()
    {
        if scenario["construction"]["mode"] != "many inputs" {
            continue;
        }
        for (role, distinct) in ["decide", "choose", "score"]
            .into_iter()
            .flat_map(|role| [false, true].map(move |distinct| (role, distinct)))
        {
            let reply = saved_reply(scenario["profile_ref"].as_str().unwrap(), role);
            let listener = Listener::answering(move |_| Canned::ok(&reply)).unwrap();
            let profile = &parity["image_profiles"][scenario["profile_ref"].as_str().unwrap()];
            let engine = setup(&listener, profile);
            let construction = &scenario["construction"];
            let first = evidence(construction, role).unwrap();
            let mut caption = text(construction, role).unwrap();
            if distinct {
                caption.replace_range(0..1, "I");
            }
            let second = input(Some(&caption), originals(construction).unwrap());
            let rows = typed_many(&engine, role, [first.clone(), second.clone()]).unwrap();
            assert_eq!(rows, [first.clone(), second.clone()]);
            assert_eq!(
                listener.count(),
                if distinct { 2 } else { 1 },
                "{} {role}, distinct={distinct}",
                scenario["id"]
            );
            let target = construction["text_recipe"]["by_function"][role]["each_final_body_bytes"]
                .as_u64()
                .unwrap() as usize;
            // Concurrent sends may arrive in either order; result rows retain input order.
            let mut requests = listener.requests();
            requests.sort_by(|left, right| left.body.cmp(&right.body));
            let mut expected = [&first, &second];
            expected.sort_by_key(|input| expected_body(profile, role, input).to_string());
            for (request, evidence) in requests.iter().zip(expected) {
                assert_eq!(request.body.len(), target);
                assert_request(request, profile, role, construction, evidence);
            }
        }
    }
}

#[test]
fn different_candidate_orders_split_complete_questions_by_bytes_with_one_image_state() {
    let corpus = corpus();
    let parity = &corpus["parity"];
    for scenario in parity["image_admission_scenarios"]["packing-overflow"]
        .as_array()
        .unwrap()
    {
        if scenario["construction"]["mode"] != "many inputs" {
            continue;
        }
        let reply = saved_reply(scenario["profile_ref"].as_str().unwrap(), "choose");
        let listener = Listener::answering(move |_| Canned::ok(&reply)).unwrap();
        let profile = &parity["image_profiles"][scenario["profile_ref"].as_str().unwrap()];
        let engine = setup(&listener, profile);
        let construction = &scenario["construction"];
        let input = evidence(construction, "choose").unwrap();
        let question = Question::choose_labels("Which color?")
            .unwrap()
            .label("red", None)
            .unwrap()
            .label("blue", None)
            .unwrap()
            .build()
            .unwrap();
        let inputs = construction["same_state_choose_candidate_orders"]
            .as_array()
            .unwrap()
            .iter()
            .map(|names| thinkthen::RecordInput {
                examples: None,
                original: input.clone(),
                context: None,
                options: Some(
                    thinkthen::RecordOptions::new(
                        names
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|name| thinkthen::RecordOption {
                                name: name.as_str().unwrap().into(),
                                description: None,
                            })
                            .collect(),
                    )
                    .unwrap(),
                ),
            });
        let rows = engine
            .choose_records_complete_with(
                &question,
                inputs,
                CallOptions::new().batch(BatchSetting::Max),
            )
            .unwrap();
        assert_eq!(rows.value().len(), 2);
        assert_eq!(listener.count(), 2);
        let target = construction["text_recipe"]["by_function"]["choose"]["each_final_body_bytes"]
            .as_u64()
            .unwrap() as usize;
        let requests = listener.requests();
        for request in &requests {
            assert_eq!(request.body.len(), target);
            assert_request(request, profile, "choose", construction, &input);
        }
        for criteria in [
            r#""criteria":{"red":null,"blue":null}"#,
            r#""criteria":{"blue":null,"red":null}"#,
        ] {
            assert_eq!(
                requests
                    .iter()
                    .filter(|request| { String::from_utf8_lossy(&request.body).contains(criteria) })
                    .count(),
                1
            );
        }
        assert_eq!(rows.value()[0].original(), &input);
        assert_eq!(rows.value()[1].original(), &input);
        assert_eq!(rows.value()[0].result().value(), Some("red"));
        assert_eq!(rows.value()[1].result().value(), Some("red"));
    }
}

#[test]
fn details_many_retains_a_valid_large_original_after_one_successful_send() {
    let corpus = corpus();
    let parity = &corpus["parity"];
    let scenario = parity["image_admission_scenarios"]["packing-overflow"]
        .as_array()
        .unwrap()
        .iter()
        .find(|scenario| {
            scenario["profile_ref"] == "perplexity-decider"
                && scenario["construction"]["mode"] == "many inputs"
        })
        .unwrap();
    let reply = saved_reply("perplexity-decider", "decide");
    let listener = Listener::answering(move |_| Canned::ok(&reply)).unwrap();
    let profile = &parity["image_profiles"]["perplexity-decider"];
    let engine = setup(&listener, profile);
    let original = evidence(&scenario["construction"], "decide").unwrap();
    let encoded = serde_json::to_string(&original).unwrap();
    assert!(encoded.len() > 16 * 1024 * 1024);
    let call = engine
        .details_input_many_with(
            &super::question(),
            [original.clone()],
            CallOptions::new().batch(BatchSetting::Max),
        )
        .into_call()
        .unwrap();
    assert_eq!(listener.count(), 1);
    let requests = listener.requests();
    assert_eq!(requests[0].body.len(), 32 * 1024 * 1024);
    assert_request(
        &requests[0],
        profile,
        "decide",
        &scenario["construction"],
        &original,
    );
    assert_eq!(call.value().len(), 1);
    let row: Value = serde_json::from_str(&call.value()[0].value().to_json()).unwrap();
    assert_eq!(
        row["input"],
        serde_json::from_str::<Value>(&encoded).unwrap()
    );
    assert_eq!(row["value"], true);
    assert_eq!(row["answer"], json!({"kind":"yes_no","probability":0.9}));
    assert_eq!(row["meta"]["model"], "pplx-decider-v1-27b");
    assert_eq!(call.facts().records(), 1);
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(call.facts().input_tokens(), Some(123));
    assert_eq!(call.facts().output_tokens(), Some(0));
}
