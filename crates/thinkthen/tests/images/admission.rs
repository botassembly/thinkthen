//! Final bodies, caller limits, relocation, and image-aware estimates.
use super::{LIQUID_DECIDE, RED, engine, fixture, input, question};
use conformance_backend::{Canned, Listener};
use std::io::Cursor;
use thinkthen::{
    Engine, ErrorKind, InputEvidence, InputFileReader, InputReaderOptions, ReaderMedia,
    ReaderOptions, SourceUnit,
};

#[test]
fn explicit_caller_limits_narrow_images() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = engine(&listener, "liquid", "d1");
    let image = fixture(RED, thinkthen::ImageMedia::Png);
    engine
        .decide_input(&question(), &input(None, vec![image.clone()]))
        .unwrap();
    let base = listener.requests()[0].body.len();
    let narrowed = Engine::builder()
        .backend("liquid")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .api_key("sk-fake")
        .unwrap()
        .no_cache()
        .max_request_bytes(base - 1)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        narrowed
            .decide_input(&question(), &input(None, vec![image]))
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 1);
}
#[test]
fn moved_file_names_do_not_change_existing_identity_and_source_has_no_lines() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let place = super::folder();
    let engine = Engine::builder()
        .backend("liquid")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .api_key("fake-key")
        .unwrap()
        .cache_at(&place)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let options = InputReaderOptions {
        media: ReaderMedia::Image,
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    };
    let source = |name| {
        InputFileReader::new(name, Cursor::new(RED), options)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
    };
    let old = source("original.png");
    let moved = source("elsewhere/image.dat");
    let first = engine
        .details_input(&question(), &old.question_input())
        .unwrap();
    let second = engine
        .details_input(&question(), &moved.question_input())
        .unwrap();
    assert_eq!(first.value().requests(), second.value().requests());
    let first_complete = engine
        .decide_input_complete_with(
            &question(),
            &old.question_input(),
            thinkthen::CallOptions::new(),
        )
        .unwrap();
    let second_complete = engine
        .decide_input_complete_with(
            &question(),
            &moved.question_input(),
            thinkthen::CallOptions::new(),
        )
        .unwrap();
    assert_eq!(
        first_complete.value().answer_id(),
        second_complete.value().answer_id()
    );
    assert_eq!(first_complete.facts().requests_sent(), 0);
    assert_eq!(second_complete.facts().requests_sent(), 0);
    let first_doc = serde_json::to_value(first_complete.complete().unwrap()).unwrap();
    assert_eq!(
        first_doc["value"]["source"],
        serde_json::json!({"file":"original.png"})
    );
    let complete = serde_json::to_value(second_complete.complete().unwrap()).unwrap();
    assert_eq!(
        complete["value"]["source"],
        serde_json::json!({"file":"elsewhere/image.dat"})
    );
    assert_eq!(listener.count(), 1);
    let located = serde_json::to_value(moved).unwrap();
    assert_eq!(located["file"], "elsewhere/image.dat");
    assert!(located.get("first_line").is_none());
    assert!(located.get("last_line").is_none());
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = std::str::from_utf8(&requests[0].body).unwrap();
    assert!(!body.contains("original.png"));
    assert!(!body.contains("elsewhere/image.dat"));
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
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
    let mut batch = engine.try_decide_input_many_with(
        &question,
        source,
        thinkthen::CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::MIN)),
    );
    let first = batch.next().unwrap().unwrap();
    let source = serde_json::to_value(first.input()).unwrap();
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
        .decide_input(
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
        .decide_input(&question(), &input(None, vec![image.clone()]))
        .unwrap();
    let base = listener.requests()[0].body.len();
    engine
        .decide_input(
            &question(),
            &input(Some(&"x".repeat(4_499_999 - base)), vec![image.clone()]),
        )
        .unwrap();
    assert_eq!(listener.requests()[0].body.len(), 4_499_999);
    let error = engine
        .decide_input(
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
