//! Explicit native image execution against independent saved bodies and loopback.

use conformance_backend::{Canned, Listener};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{
    Answer, CallOptions, Engine, ErrorKind, ImageEvidence, ImageInput, ImageMedia, InputFunction,
    Question, QuestionInput,
};

#[path = "images/replay.rs"]
mod replay;

const RED: &[u8] = include_bytes!("../../../specification/fixtures/images/red.png");
const BLUE: &[u8] = include_bytes!("../../../specification/fixtures/images/blue.png");
const LARGE: &[u8] = include_bytes!("../../../specification/fixtures/images/above-spike.png");
const JPEG: &[u8] = include_bytes!("../../../specification/fixtures/images/red.jpg");
const LIQUID_DECIDE: &str =
    include_str!("../../../specification/fixtures/images/liquid-decide-reply.json");
const PPLX_DECIDE: &str =
    include_str!("../../../specification/fixtures/images/perplexity-decide-reply.json");

thinkthen::choices! { enum Color { Red => "red", Blue => "blue" } }

#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn fixture(bytes: &[u8], media: ImageMedia) -> ImageInput {
    ImageInput::new(media, bytes.to_vec()).expect("valid saved fixture or loopback configuration")
}
#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn input(text: Option<&str>, images: Vec<ImageInput>) -> QuestionInput {
    QuestionInput::Images(
        ImageEvidence::new(text.map(str::to_owned), images)
            .expect("valid saved fixture or loopback configuration"),
    )
}
fn pair() -> QuestionInput {
    input(
        Some("Compare originals."),
        vec![
            fixture(RED, ImageMedia::Png),
            fixture(BLUE, ImageMedia::Png),
            fixture(RED, ImageMedia::Png),
        ],
    )
}
#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn question() -> Question {
    Question::decide("Is red visible?")
        .expect("valid saved fixture or loopback configuration")
        .cut()
}
#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn engine(listener: &Listener, backend: &str, model: &str) -> Engine {
    Engine::builder()
        .backend(backend)
        .expect("valid saved fixture or loopback configuration")
        .base_url(listener.base())
        .expect("valid saved fixture or loopback configuration")
        .model(model)
        .expect("valid saved fixture or loopback configuration")
        .api_key("fake-image-key")
        .expect("valid saved fixture or loopback configuration")
        .no_cache()
        .max_retries(0)
        .build()
        .expect("valid saved fixture or loopback configuration")
}
fn url(bytes: &[u8], media: &str) -> String {
    use base64::Engine as _;
    format!(
        "data:{media};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct OrderedBody {
    state: Value,
    model: String,
    questions: Value,
    images: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct PartsBody {
    state: Vec<Value>,
    model: String,
    questions: Value,
}
fn ordered_expected(questions: Value) -> OrderedBody {
    OrderedBody {
        state: json!("Compare originals."),
        model: "d1".into(),
        questions,
        images: vec![
            url(RED, "image/png"),
            url(BLUE, "image/png"),
            url(RED, "image/png"),
        ],
    }
}
fn questions(kind: &str) -> Value {
    match kind {
        "decide" => json!({"q1":{"type":"noul","instructions":"Is red visible?"}}),
        "choose" => {
            json!({"q1":{"type":"choice","instructions":"Which color?","criteria":{"red":null,"blue":null}}})
        }
        "score" => {
            json!({"q1":{"type":"score","instructions":"How red?","criteria":["none","all"]}})
        }
        _ => unreachable!(),
    }
}

#[test]
fn scalar_doors_send_original_order_duplicates_criteria_and_return_typed_facts() {
    let decide = question();
    let choose = Question::choose::<Color>("Which color?")
        .unwrap()
        .option(Color::Red, None)
        .unwrap()
        .option(Color::Blue, None)
        .unwrap()
        .build()
        .unwrap();
    let score = Question::score("How red?")
        .unwrap()
        .level("none", None)
        .unwrap()
        .level("all", None)
        .unwrap()
        .build()
        .unwrap();
    for (route, model, responses) in [
        (
            "liquid",
            "d1",
            [
                LIQUID_DECIDE,
                include_str!("../../../specification/fixtures/images/liquid-choose-reply.json"),
                include_str!("../../../specification/fixtures/images/liquid-score-reply.json"),
            ],
        ),
        (
            "perplexity",
            "pplx-decider-v1-27b",
            [
                PPLX_DECIDE,
                include_str!("../../../specification/fixtures/images/perplexity-choose-reply.json"),
                include_str!("../../../specification/fixtures/images/perplexity-score-reply.json"),
            ],
        ),
    ] {
        let at = AtomicUsize::new(0);
        let listener = Listener::answering(move |_| {
            Canned::ok(responses[at.fetch_add(1, Ordering::Relaxed) % 3])
        })
        .unwrap();
        let engine = engine(&listener, route, model);
        let answer = engine.decide_input(&decide, &pair()).unwrap();
        assert_eq!(*answer.value(), Answer::Yes);
        assert_eq!(answer.facts().input_tokens(), Some(123));
        assert_eq!(
            *engine.choose_input(&choose, &pair()).unwrap().value(),
            Some(Color::Red)
        );
        assert_eq!(*engine.score_input(&score, &pair()).unwrap().value(), 0.8);
        let requests = listener.requests();
        assert_eq!(requests.len(), 3);
        for (request, kind) in requests.iter().zip(["decide", "choose", "score"]) {
            if route == "liquid" {
                assert_eq!(
                    serde_json::from_slice::<OrderedBody>(&request.body).unwrap(),
                    ordered_expected(questions(kind))
                );
            } else {
                let mut state = vec![json!("Compare originals.")];
                state.extend([RED, BLUE, RED].map(
                    |bytes| json!({"type":"image_url","image_url":{"url":url(bytes,"image/png")}}),
                ));
                assert_eq!(
                    serde_json::from_slice::<PartsBody>(&request.body).unwrap(),
                    PartsBody {
                        state,
                        model: model.into(),
                        questions: questions(kind)
                    }
                );
            }
        }
    }
}

#[test]
fn image_only_state_has_no_invented_caption_and_large_originals_exceed_spike_caps() {
    for (route, model, reply) in [
        ("liquid", "d1", LIQUID_DECIDE),
        ("perplexity", "pplx-decider-v1-27b", PPLX_DECIDE),
    ] {
        let listener = Listener::answering(move |_| Canned::ok(reply)).unwrap();
        let engine = engine(&listener, route, model);
        assert!(LARGE.len() > 32_768);
        engine
            .decide_input(
                &question(),
                &input(None, vec![fixture(LARGE, ImageMedia::Png)]),
            )
            .unwrap();
        let body = &listener.requests()[0].body;
        assert!(body.len() > 65_536);
        let body: Value = serde_json::from_slice(body).unwrap();
        if route == "liquid" {
            assert_eq!(body["state"], "");
            assert_eq!(body["images"], json!([url(LARGE, "image/png")]));
        } else {
            assert_eq!(
                body["state"],
                json!([{"type":"image_url","image_url":{"url":url(LARGE,"image/png")}}])
            );
            assert!(body.get("images").is_none());
        }
    }
}

#[test]
fn every_text_only_function_and_unknown_route_refuses_with_zero_connections() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    for function in [
        InputFunction::Tag,
        InputFunction::Filter,
        InputFunction::Rank,
        InputFunction::Annotate,
        InputFunction::Find,
        InputFunction::Recognize,
        InputFunction::Relate,
    ] {
        let error = engine
            .input_details(function, &question(), &pair(), CallOptions::new())
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.to_string(),
            format!(
                "{} accepts text only; images are unsupported",
                function.name()
            )
        );
    }
    for (route, model) in [
        ("liquid", "d1:free"),
        ("perplexity", "unknown"),
        ("llamacpp", "local"),
        ("openrouter", "liquid/lfm-2.5-clef"),
        ("typesafe", "jev-1.13.0"),
        ("mlx", "local"),
        ("ollama", "local"),
    ] {
        let error = self::engine(&listener, route, model)
            .decide_input(&question(), &pair())
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(error.to_string().contains("images are unsupported"));
    }
    let unnamed = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    assert_eq!(
        unnamed
            .decide_input(&question(), &pair())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 0);
}

#[test]
fn malformed_pixels_media_counts_and_debug_are_safe_and_send_nothing() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    for (bytes, media) in [
        (
            include_bytes!("../../../specification/fixtures/images/malformed-pixels.png")
                .as_slice(),
            ImageMedia::Png,
        ),
        (
            include_bytes!("../../../specification/fixtures/images/truncated.png").as_slice(),
            ImageMedia::Png,
        ),
        (
            include_bytes!("../../../specification/fixtures/images/truncated.jpg").as_slice(),
            ImageMedia::Jpeg,
        ),
        (
            include_bytes!(
                "../../../specification/fixtures/images/progressive-app14-one-component.jpg"
            )
            .as_slice(),
            ImageMedia::Jpeg,
        ),
        (
            include_bytes!("../../../specification/fixtures/images/malformed-scan.jpg").as_slice(),
            ImageMedia::Jpeg,
        ),
        (RED, ImageMedia::Jpeg),
        (JPEG, ImageMedia::Png),
        (b"PRIVATE_IMAGE_MARKER", ImageMedia::Png),
    ] {
        let error = ImageInput::new(media, bytes.to_vec()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(!format!("{error:?}").contains("PRIVATE_IMAGE_MARKER"));
    }
    fixture(JPEG, ImageMedia::Jpeg);
    assert!(ImageEvidence::new(None, vec![]).is_err());
    assert!(ImageEvidence::new(None, vec![fixture(RED, ImageMedia::Png); 9]).is_err());
    assert!(ImageEvidence::new(None, vec![fixture(RED, ImageMedia::Png); 8]).is_ok());
    let secret = input(
        Some("PRIVATE_IMAGE_MARKER"),
        vec![fixture(RED, ImageMedia::Png)],
    );
    let debug = format!("{secret:?}");
    assert!(!debug.contains("PRIVATE_IMAGE_MARKER"));
    assert!(!debug.contains(&url(RED, "image/png")));
    assert_eq!(listener.count(), 0);
}

static NEXT: AtomicUsize = AtomicUsize::new(0);
#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn folder() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "thinkthen-native-images-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&path).expect("valid saved fixture or loopback configuration");
    path
}

#[test]
fn cache_and_strict_replay_use_original_bytes_order_and_state_without_filenames() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let folder = folder();
    let build = || {
        Engine::builder()
            .backend("liquid")
            .unwrap()
            .base_url(listener.base())
            .unwrap()
            .model("d1")
            .unwrap()
            .api_key("fake-key")
            .unwrap()
    };
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let first = engine.details_input(&question(), &pair()).unwrap();
    let again = engine.details_input(&question(), &pair()).unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(again.facts().requests_sent(), 0);
    assert_eq!(
        first.value().probabilities(),
        &thinkthen::Probabilities::YesNo { yes: 0.9 }
    );
    let changed = input(
        Some("Compare originals."),
        vec![
            fixture(BLUE, ImageMedia::Png),
            fixture(RED, ImageMedia::Png),
            fixture(RED, ImageMedia::Png),
        ],
    );
    engine.decide_input(&question(), &changed).unwrap();
    assert_eq!(listener.count(), 2);
    let replay = build().no_cache().replay(&folder).unwrap().build().unwrap();
    replay.decide_input(&question(), &pair()).unwrap();
    assert_eq!(listener.count(), 2);
    let miss = input(Some("changed context"), vec![fixture(RED, ImageMedia::Png)]);
    assert_eq!(
        replay.decide_input(&question(), &miss).unwrap_err().kind(),
        ErrorKind::Local
    );
    assert_eq!(listener.count(), 2);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn batches_retain_inputs_and_context_and_retry_keeps_complete_images() {
    let sends = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if sends.fetch_add(1, Ordering::Relaxed) == 0 {
            Canned::status(503, "retry")
        } else {
            Canned::ok(LIQUID_DECIDE)
        }
    })
    .unwrap();
    let engine = Engine::builder()
        .backend("liquid")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .no_cache()
        .max_retries(1)
        .build()
        .unwrap();
    let inputs = vec![pair(), pair()];
    let rows = engine
        .decide_input_many_with(
            &question(),
            inputs.clone(),
            CallOptions::new().context("Batch context."),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].input(), &inputs[0]);
    assert_eq!(rows[1].value(), &Answer::Yes);
    let bodies = listener.requests();
    assert_eq!(bodies.len(), 2);
    assert_eq!(bodies[0].body, bodies[1].body);
    let actual: OrderedBody = serde_json::from_slice(&bodies[0].body).unwrap();
    let mut expected = ordered_expected(questions("decide"));
    expected.state = json!({"context":"Batch context.","text":"Compare originals."});
    assert_eq!(actual, expected);
}

#[test]
fn text_input_door_preserves_existing_request_bytes() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    engine.decide(&question(), "Original text.").unwrap();
    engine
        .decide_input(&question(), &QuestionInput::Text("Original text.".into()))
        .unwrap();
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, requests[1].body);
}

#[path = "images/admission.rs"]
mod admission;

#[test]
fn saved_0034_liquid_one_image_exchange_keeps_original_wire_and_usage() {
    let exchange: Value = serde_json::from_str(include_str!(
        "../../../specification/fixtures/images/0034-liquid-decide-exchange.json"
    ))
    .unwrap();
    let response = exchange["response"].to_string();
    let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    let request = &exchange["request"];
    let words = request["questions"]["q1"]["instructions"].as_str().unwrap();
    let question = Question::decide(words).unwrap().cut();
    let image = fixture(
        include_bytes!("../../../specification/fixtures/images/0034-earth.jpg"),
        ImageMedia::Jpeg,
    );
    let call = engine
        .details_input(&question, &input(request["state"].as_str(), vec![image]))
        .unwrap();
    assert_eq!(
        call.value().probabilities(),
        &thinkthen::Probabilities::YesNo {
            yes: 0.9987551569665462
        }
    );
    assert_eq!(call.facts().input_tokens(), Some(148));
    let actual: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(&actual, request);
}
