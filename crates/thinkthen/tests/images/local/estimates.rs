use super::{CLEF, REPLY, build, pair, question};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use thinkthen::ErrorKind;

#[test]
fn local_images_use_approximate_body_admission_and_refuse_a_small_explicit_cap_without_sends() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let small = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .max_estimated_input_tokens_total(Some(1))
        .build()
        .unwrap();
    let error = small.decide_input(&question(), &pair()).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.to_string(),
        "max_estimated_input_tokens_total=1 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"
    );
    assert_eq!(listener.count(), 0);
    assert!(!format!("{error:?}").contains("PRIVATE_LOCAL_KEY"));
    let sufficient = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .max_estimated_input_tokens_total(Some(u64::MAX))
        .build()
        .unwrap();
    sufficient.decide_input(&question(), &pair()).unwrap();
    assert_eq!(listener.count(), 1);
    let request: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        request,
        json!({
            "state": "Compare originals.", "model": "clef-local-0036",
            "questions": crate::questions("decide"),
            "images": [crate::url(crate::RED, "image/png"), crate::url(crate::BLUE, "image/png")]
        })
    );
}
