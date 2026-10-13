//! Immediate facts project immutable core values, including aggregate ID absence.
use crate::session::views::*;
use crate::session::views_generated::*;
use std::ffi::{CStr, c_char};
fn text(value: thinkthen_complete_utf8_v1) -> String {
    if value.len == 0 {
        return String::new();
    }
    // SAFETY: fixtures retain the failure slot that owns this counted UTF-8.
    String::from_utf8(unsafe { std::slice::from_raw_parts(value.data.cast(), value.len) }.to_vec())
        .expect("view UTF-8")
}
fn optional_text(value: thinkthen_complete_utf8_presence_v1) -> Option<String> {
    (value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1).then(|| text(value.value))
}
pub(super) fn check(
    expected: &thinkthen::Facts,
    pointer: *const thinkthen_error_view_v1,
    legacy: *const c_char,
) {
    assert!(!pointer.is_null(), "last typed error is present");
    // SAFETY: owned native diagnostics remain live until this thread fails again.
    let error = unsafe { &*pointer };
    assert!(!error.facts.is_null(), "last typed facts are present");
    let view = unsafe { &*error.facts };
    assert_eq!(
        (view.records, view.requests_sent, view.cache_answers),
        (
            expected.records(),
            expected.requests_sent(),
            expected.cache_answers()
        )
    );
    assert_eq!(view.seconds, expected.seconds());
    assert_eq!(optional_text(view.model).as_deref(), expected.model());
    assert_eq!(
        optional_text(view.estimated_cost_usd).as_deref(),
        expected.estimated_cost_usd()
    );
    for (value, expected) in [
        (view.input_tokens, expected.input_tokens()),
        (view.output_tokens, expected.output_tokens()),
    ] {
        assert_eq!(
            (value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1).then_some(value.value),
            expected
        );
    }
    let raw: serde_json::Value =
        serde_json::from_slice(unsafe { CStr::from_ptr(legacy) }.to_bytes()).expect("legacy facts");
    assert_eq!(
        Some(view.largest_request_bytes),
        raw["largest_request_bytes"].as_u64()
    );
    assert_eq!(
        (view.largest_request_estimated_input_tokens.presence
            == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1)
            .then_some(view.largest_request_estimated_input_tokens.value),
        raw["largest_request_estimated_input_tokens"].as_u64()
    );
    assert_eq!(
        text(view.token_estimate_method),
        raw["token_estimate_method"]
            .as_str()
            .expect("estimate method")
    );
    assert_eq!(
        view.call_id.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
        expected.call_id().is_some()
    );
    if let Some(id) = expected.call_id() {
        assert_eq!(text(unsafe { &*view.call_id.value }.value), id.to_string());
    }
    assert_eq!(
        view.attempts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
        expected.attempts().is_some()
    );
    if let Some(attempts) = expected.attempts() {
        let observed = &view.attempts.value;
        assert_eq!(observed.len, attempts.len());
    }
    assert_eq!(
        view.usage_persistence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
        expected.usage_persistence().is_some()
    );
    assert!(view.extensions.len == 0);
}
