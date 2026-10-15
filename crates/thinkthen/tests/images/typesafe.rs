//! Pinned TypeSafe asset wire, opt-in header and admission through native requests.
use super::{engine, fixture, input, question, questions, request::ImageRequests};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use thinkthen::{Answer, ImageMedia, Question};

const PNG: &[u8] = include_bytes!("../../../../specification/fixtures/images/local/edge-1024.png");
const JPEG: &[u8] = include_bytes!("../../../../specification/fixtures/images/0034-earth.jpg");
const DECIDE: &str =
    include_str!("../../../../specification/fixtures/images/typesafe-decide-reply.json");

fn evidence() -> thinkthen::QuestionInput {
    input(
        Some("Compare originals."),
        vec![
            fixture(PNG, ImageMedia::Png),
            fixture(JPEG, ImageMedia::Jpeg),
            fixture(PNG, ImageMedia::Png),
        ],
    )
}

#[test]
fn scalar_replays_send_original_assets_once_with_ordered_keys_and_opt_in() {
    use base64::Engine as _;
    let replies = [
        DECIDE,
        include_str!("../../../../specification/fixtures/images/typesafe-choose-reply.json"),
        include_str!("../../../../specification/fixtures/images/typesafe-score-reply.json"),
    ];
    let at = std::sync::atomic::AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        Canned::ok(replies[at.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % 3])
    })
    .unwrap();
    let engine = engine(&listener, "typesafe", "jev-1.13.0");
    assert_eq!(
        engine
            .request_decide(&question(), &evidence())
            .unwrap()
            .value()[0]
            .result()
            .value(),
        Answer::Yes
    );
    let choose = Question::choose_labels("Which color?")
        .unwrap()
        .label("red", None)
        .unwrap()
        .label("blue", None)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        engine.request_choose(&choose, &evidence()).unwrap().value()[0]
            .result()
            .value(),
        Some("red")
    );
    let score = Question::score("How red?")
        .unwrap()
        .level("none", None)
        .unwrap()
        .level("all", None)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        engine.request_score(&score, &evidence()).unwrap().value()[0]
            .result()
            .value(),
        0.8
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for (request, kind) in requests.iter().zip(["decide", "choose", "score"]) {
        assert_eq!(request.header("TypeSafe-Danger-Zone-Images"), Some("true"));
        let mut assets = serde_json::Map::new();
        let mut state = vec![json!("Compare originals.")];
        for (at, (bytes, format)) in [(PNG, "png"), (JPEG, "jpeg"), (PNG, "png")]
            .into_iter()
            .enumerate()
        {
            let hash = format!("{:x}", Sha256::digest(bytes));
            let key = format!("thinkthen-image-{}-{}", at + 1, &hash[..16]);
            state.push(json!(key));
            assets.insert(
                key,
                json!({"type":"image", "format":format,
                "data_base64":base64::engine::general_purpose::STANDARD.encode(bytes)}),
            );
        }
        assert_eq!(
            serde_json::from_slice::<Value>(&request.body).unwrap(),
            json!({"state":state,"model":"jev-1.13.0","questions":questions(kind),"assets":assets})
        );
    }
}

#[test]
fn text_only_bytes_and_headers_stay_unchanged_and_image_only_has_no_caption() {
    let listener = Listener::answering(|_| Canned::ok(DECIDE)).unwrap();
    let engine = engine(&listener, "typesafe", "jev-1.14.0");
    engine.decide(&question(), "Original text.").unwrap();
    engine
        .request_decide(
            &question(),
            &thinkthen::QuestionInput::Text("Original text.".into()),
        )
        .unwrap();
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    for request in &requests {
        assert_eq!(request.body, br#"{"state":"Original text.","model":"jev-1.14.0","questions":{"q1":{"type":"noul","instructions":"Is red visible?"}}}"#);
        assert_eq!(request.header("TypeSafe-Danger-Zone-Images"), None);
    }
    engine
        .request_decide(
            &question(),
            &input(None, vec![fixture(PNG, ImageMedia::Png)]),
        )
        .unwrap();
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["state"].as_array().unwrap().len(), 1);
    assert_eq!(
        requests[0].header("TypeSafe-Danger-Zone-Images"),
        Some("true")
    );
}

#[test]
fn oversized_and_undersized_images_count_zero_actual_sends() {
    let listener = Listener::answering(|_| Canned::ok(DECIDE)).unwrap();
    let engine = engine(&listener, "typesafe", "jev-1.13.0");
    for (bytes, sentence) in [
        (super::RED, "image 1 is below width: limit 256, actual 1"),
        (
            include_bytes!("../../../../specification/fixtures/images/boundary-2048x1024.png")
                .as_slice(),
            "image 1 exceeds width: limit 1568, actual 2048",
        ),
    ] {
        let error = engine
            .request_decide(
                &question(),
                &input(None, vec![fixture(bytes, ImageMedia::Png)]),
            )
            .unwrap_err();
        assert!(error.to_string().contains(sentence), "{error}");
        assert_eq!(listener.count(), 0);
    }
}
