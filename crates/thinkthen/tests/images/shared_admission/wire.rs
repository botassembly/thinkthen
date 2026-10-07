//! Independent expected image wire construction.
use super::{QuestionInput, Value, json, questions, url};

#[expect(
    clippy::panic,
    reason = "this independent fixture expects an image input"
)]
pub(super) fn expected_body(profile: &Value, role: &str, evidence: &QuestionInput) -> Value {
    let QuestionInput::Images(evidence) = evidence else {
        panic!("image evidence")
    };
    let urls: Vec<_> = evidence
        .images()
        .iter()
        .map(|image| url(image.bytes(), image.media().mime()))
        .collect();
    let state = evidence.text().unwrap_or("");
    if profile["model"] == "pplx-decider-v1-27b" {
        let mut parts = vec![];
        if !state.is_empty() {
            parts.push(json!(state));
        }
        parts.extend(
            urls.into_iter()
                .map(|url| json!({"type":"image_url","image_url":{"url":url}})),
        );
        json!({"state":parts,"model":profile["model"],"questions":questions(role)})
    } else {
        json!({"state":state,"model":profile["model"],"questions":questions(role),"images":urls})
    }
}
