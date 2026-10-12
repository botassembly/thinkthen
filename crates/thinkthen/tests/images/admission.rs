//! Final bodies, caller limits, relocation, and image-aware estimates.
use super::request::ImageRequests;
use super::{LIQUID_DECIDE, RED, engine, fixture, input, question};
use conformance_backend::{Canned, Listener};
use thinkthen::{ErrorKind, InputReaderOptions, ReaderMedia, ReaderOptions, SourceUnit};

#[test]
fn explicit_caller_limits_narrow_images() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    let image = fixture(RED, thinkthen::ImageMedia::Png);
    engine
        .request_decide(&question(), &input(None, vec![image.clone()]))
        .unwrap();
    let base = listener.requests()[0].body.len();
    let narrowed = super::image_builder::builder(listener.base(), "liquid", "d1", Some("sk-fake"))
        .no_cache()
        .max_request_bytes(base - 1)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        narrowed
            .request_decide(&question(), &input(None, vec![image]))
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 1);
}
#[test]
fn image_reader_batches_keep_completed_rows_and_stop_after_later_decode_failure() {
    use std::num::NonZeroUsize;
    use thinkthen::{BatchSetting, read_inputs};
    let place = super::folder();
    std::fs::write(place.join("a.png"), RED).unwrap();
    std::fs::write(place.join("b.png"), b"PRIVATE_IMAGE_BYTES").unwrap();
    std::fs::write(place.join("c.png"), RED).unwrap();
    let options = InputReaderOptions {
        media: ReaderMedia::Image,
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    };
    let source = read_inputs([&place], options).unwrap();
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    let question = question();
    let mut batch = super::request::decisions(
        &engine,
        question.clone().into(),
        source,
        thinkthen::CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::MIN)),
    )
    .unwrap();
    let first = batch.next().unwrap().unwrap();
    let source = serde_json::to_value(first.original()).unwrap();
    assert!(source["file"].as_str().unwrap().ends_with("a.png"));
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(!format!("{error:?}").contains("PRIVATE_IMAGE_BYTES"));
    assert!(batch.next().is_none());
    assert_eq!(listener.count(), 1);
    drop(batch);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn direct_question_model_override_rechecks_images_without_changing_endpoint() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    let question = thinkthen::Question::decide("Is red visible?")
        .unwrap()
        .model("d1:free")
        .unwrap()
        .cut();
    let error = engine
        .request_decide(
            &question,
            &input(None, vec![fixture(RED, thinkthen::ImageMedia::Png)]),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
    engine.decide(&question, "Original text.").unwrap();
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].line, "POST /v1/systemone HTTP/1.1");
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["model"], "d1:free");
    assert!(body.get("images").is_none());
}

#[test]
#[ignore = "release-only large-input boundary; run sdlc/scripts/test-full-cases --run"]
fn release_only_final_image_body_limits_are_exact() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    let image = fixture(RED, thinkthen::ImageMedia::Png);
    engine
        .request_decide(&question(), &input(None, vec![image.clone()]))
        .unwrap();
    let base = listener.requests()[0].body.len();
    engine
        .request_decide(
            &question(),
            &input(Some(&"x".repeat(4_499_999 - base)), vec![image.clone()]),
        )
        .unwrap();
    assert_eq!(listener.requests()[0].body.len(), 4_499_999);
    let error = engine
        .request_decide(
            &question(),
            &input(Some(&"x".repeat(4_500_000 - base)), vec![image.clone()]),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 2);
}

#[test]
fn borrowed_image_lengths_refuse_before_image_allocation() {
    use thinkthen::{ImageAdmission, ImageInput, MAX_IMAGE_BYTES, MAX_IMAGES};
    for count in [0, MAX_IMAGES + 1, usize::MAX] {
        let error = ImageAdmission::new(count).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "image evidence requires 1 to 8 images"
        );
    }
    for length in [MAX_IMAGE_BYTES + 1, usize::MAX] {
        let error = ImageInput::admit_length(length).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "image exceeds the 25165824 compressed byte SDK limit"
        );
    }
    ImageInput::admit_length(MAX_IMAGE_BYTES).unwrap();
    for last in [1, usize::MAX] {
        let mut admission = ImageAdmission::new(2).unwrap();
        admission.push(MAX_IMAGE_BYTES).unwrap();
        let error = admission.push(last).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "image evidence exceeds the 25165824 compressed byte SDK limit"
        );
    }
    let mut admission = ImageAdmission::new(MAX_IMAGES).unwrap();
    for _ in 0..MAX_IMAGES {
        admission.push(0).unwrap();
    }
}
