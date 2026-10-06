//! Image admission uses image tokens rather than treating encoded pixels as text.
use conformance_backend::{Canned, Listener};
use thinkthen::{
    Engine, ErrorKind, ImageEvidence, ImageInput, ImageMedia, Question, QuestionInput,
};
const LIQUID_DECIDE: &str =
    include_str!("../../../specification/fixtures/images/liquid-decide-reply.json");
const RED: &[u8] = include_bytes!("../../../specification/fixtures/images/red.png");
#[test]
fn estimated_image_admission_is_labelled_and_refuses_before_sending() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let engine = Engine::builder()
        .backend("liquid")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .api_key("sk-fake")
        .unwrap()
        .no_cache()
        .max_estimated_input_tokens_total(Some(0))
        .build()
        .unwrap();
    let question = Question::decide("Is red visible?").unwrap().cut();
    let image = ImageInput::new(ImageMedia::Png, RED.to_vec()).unwrap();
    let input = QuestionInput::Images(ImageEvidence::new(None, vec![image]).unwrap());
    let error = engine.decide_input(&question, &input).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.estimated_input_denial().is_some());
    assert!(
        error
            .to_string()
            .contains("text-bytes-908-plus-image-tiles-v1")
    );
    assert!(!error.to_string().contains("encoded-body-bytes-908-v1"));
    assert_eq!(listener.count(), 0);
}
