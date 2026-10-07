//! Typed image/text columns and physical locations use the native input door.
use super::common;
use conformance_backend::{Canned, Listener};
use thinkthen::{
    CallOptions, ErrorKind, ImageEvidence, ImageInput, ImageMedia, InputReaderOptions,
    PolarsCallOptions, PolarsEngine, Question, QuestionInput, ReaderMedia, ReaderOptions,
    SourceUnit,
};

#[expect(
    clippy::expect_used,
    reason = "a failed saved fixture stops the consumer"
)]
fn images() -> QuestionInput {
    let red = ImageInput::new(
        ImageMedia::Png,
        include_bytes!("../../../../specification/fixtures/images/red.png").to_vec(),
    )
    .expect("red image");
    let blue = ImageInput::new(
        ImageMedia::Png,
        include_bytes!("../../../../specification/fixtures/images/blue.png").to_vec(),
    )
    .expect("blue image");
    QuestionInput::Images(
        ImageEvidence::new(
            Some("Compare originals.".to_owned()),
            vec![red.clone(), blue, red],
        )
        .expect("ordered comparison"),
    )
}
#[expect(
    clippy::expect_used,
    reason = "a failed saved fixture stops the consumer"
)]
fn question() -> Question {
    Question::decide("Is red visible?").expect("decide").cut()
}
#[expect(
    clippy::expect_used,
    reason = "a failed saved fixture stops the consumer"
)]
fn engine(listener: &Listener) -> thinkthen::Engine {
    common::builder(listener.base())
        .backend("liquid")
        .expect("backend")
        .model("d1")
        .expect("model")
        .no_cache()
        .build()
        .expect("engine")
}
const REPLY: &str =
    include_str!("../../../../specification/fixtures/images/liquid-decide-reply.json");

#[test]
fn explicit_image_columns_preserve_order_duplicates_nulls_and_probabilities() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).expect("listener");
    let engine = engine(&listener);
    let question = question();
    let inputs = [Some(images()), None, Some(images())];
    let values = engine
        .input_column(
            &question,
            &inputs,
            PolarsCallOptions::new().probability(true),
        )
        .expect("image column");
    assert_eq!(
        values
            .value()
            .column("value")
            .expect("value")
            .bool()
            .expect("booleans")
            .iter()
            .collect::<Vec<_>>(),
        [Some(true), None, Some(true)]
    );
    assert_eq!(
        values
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("probabilities")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.9), None, Some(0.9)]
    );
    assert_eq!(values.facts().records(), 2);
    assert_eq!(values.facts().requests_sent(), 1);
    let requests = listener.requests();
    for request in &requests {
        let body: serde_json::Value = serde_json::from_slice(&request.body).expect("body");
        assert_eq!(body["state"], "Compare originals.");
        let images = body["images"].as_array().expect("images");
        assert_eq!(images.len(), 3);
        assert_eq!(images[0], images[2]);
        assert_ne!(images[0], images[1]);
    }
    let details = engine
        .input_details_column(&question, &inputs[..1], CallOptions::new())
        .expect("typed details");
    assert_eq!(
        details.value()[0]
            .as_ref()
            .expect("details")
            .probabilities(),
        &thinkthen::Probabilities::YesNo { yes: 0.9 }
    );
    assert_eq!(listener.count(), 2);
    assert!(!format!("{inputs:?}").contains("Compare originals."));
}

#[test]
fn located_image_columns_have_no_invented_text_coordinates() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).expect("listener");
    let engine = engine(&listener);
    let question = question();
    let file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images/red.png");
    let located = engine
        .source_column(
            &question,
            std::slice::from_ref(&file),
            InputReaderOptions {
                media: ReaderMedia::Image,
                reading: ReaderOptions {
                    unit: SourceUnit::File,
                    ..ReaderOptions::default()
                },
            },
            PolarsCallOptions::new(),
        )
        .expect("located image");
    assert_eq!(located.value().height(), 1);
    assert!(
        located
            .value()
            .column("file")
            .expect("file")
            .str()
            .expect("source name")
            .get(0)
            .expect("location")
            .ends_with("red.png")
    );
    assert_eq!(
        located
            .value()
            .column("first_line")
            .expect("first")
            .u64()
            .expect("line")
            .get(0),
        None
    );
    assert_eq!(
        located
            .value()
            .column("last_line")
            .expect("last")
            .u64()
            .expect("line")
            .get(0),
        None
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn text_only_image_refusals_and_expired_image_columns_send_nothing() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).expect("listener");
    let engine = engine(&listener);
    let tag = Question::tag_labels("Which?")
        .expect("tag")
        .label("red", None)
        .expect("red")
        .label("blue", None)
        .expect("blue")
        .build()
        .expect("tags");
    let mixed = [
        Some(QuestionInput::Text("ordinary text".to_owned())),
        Some(images()),
    ];
    let error = engine
        .input_column(&tag, &mixed, PolarsCallOptions::new())
        .expect_err("text-only");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.to_string(),
        "tag accepts text only; images are unsupported"
    );
    let error = engine
        .input_column(
            &question(),
            &[Some(images())],
            PolarsCallOptions::new().call(CallOptions::new().deadline_ms(0).expect("spent")),
        )
        .expect_err("deadline");
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(listener.count(), 0);
}

#[test]
fn ordered_image_choose_and_score_keep_native_probability_and_rubric_values() {
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("low") {
            Canned::ok(r#"{"model":"d1","answers":{"q1":{"type":"score","probabilities":{"0":0.25,"1":0.75}}}}"#)
        } else {
            Canned::ok(r#"{"model":"d1","answers":{"q1":{"type":"choice","probabilities":{"red":0.8,"blue":0.2}}}}"#)
        }
    }).expect("listener");
    let engine = engine(&listener);
    let choose = Question::choose_labels("Which color?")
        .expect("choose")
        .label("red", None)
        .expect("red")
        .label("blue", None)
        .expect("blue")
        .build()
        .expect("question");
    let inputs = [Some(images()), None];
    let chosen = engine
        .input_column(&choose, &inputs, PolarsCallOptions::new().probability(true))
        .expect("choose column");
    assert_eq!(
        chosen
            .value()
            .column("value")
            .expect("value")
            .str()
            .expect("labels")
            .iter()
            .collect::<Vec<_>>(),
        [Some("red"), None]
    );
    assert_eq!(
        chosen
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("probabilities")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.8), None]
    );
    let score = Question::score("How much?")
        .expect("score")
        .level("low", None)
        .expect("low")
        .level("high", None)
        .expect("high")
        .build()
        .expect("question");
    let scored = engine
        .input_column(&score, &inputs, PolarsCallOptions::new())
        .expect("score column");
    assert_eq!(
        scored
            .value()
            .column("value")
            .expect("value")
            .f64()
            .expect("scores")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.75), None]
    );
    assert_eq!(listener.count(), 2);
}
