//! Stays: native handle cancellation and foreign pointer ownership need direct concurrency checks.
//! Session diagnostics remain independent from the engine constructor's slot.
use super::{
    thinkthen_session_error_facts_json as facts, thinkthen_session_error_message as message,
    thinkthen_session_error_retryable as retryable,
};
use crate::failures;
#[path = "views/ffi.rs"]
mod typed_errors;

#[test]
fn immediate_native_diagnostics_keep_retry_and_owned_facts() {
    let backend = conformance_backend::Backend::start().expect("loopback");
    let engine = thinkthen::Engine::builder()
        .base_url(&format!("{}/arm/503/v1", backend.origin()))
        .expect("base")
        .api_key("fake-session-key")
        .expect("key")
        .max_retries(0)
        .no_cache()
        .build()
        .expect("engine");
    let question = thinkthen::Question::decide("Is it?")
        .expect("question")
        .cut();
    let error = engine
        .decide_with(
            &question,
            "text",
            thinkthen::CallOptions::default().attempts(true),
        )
        .expect_err("backend fails");
    let expected = error.facts().expect("started failure facts").clone();
    assert!(error.retryable());
    assert_eq!(
        failures::session_native_call(|| Err(error.into())),
        crate::ffi::values::THINKTHEN_EBACKEND
    );
    drop(engine);
    typed_errors::check(&expected, crate::failures::session_view(), facts());
    assert_eq!(retryable(), 1);
    let saved_message = message();
    let saved_facts = facts();
    assert!(!saved_facts.is_null());
    // SAFETY: the session owns this NUL-terminated string until the next failure.
    let json = unsafe { std::ffi::CStr::from_ptr(saved_facts) }.to_owned();
    let value: serde_json::Value = serde_json::from_slice(json.to_bytes()).expect("facts JSON");
    assert_eq!(value["requests_sent"], 1);
    assert_eq!(
        failures::session_call(|| Ok(())),
        crate::ffi::values::THINKTHEN_OK
    );
    let constructor = crate::failures::built(|| crate::settings::build("{}"));
    assert!(constructor.is_some());
    drop(constructor);
    assert!(crate::failures::built(|| crate::settings::build("{\"bad\":1}")).is_none());
    assert_eq!(message(), saved_message);
    assert_eq!(facts(), saved_facts);
    assert_eq!(retryable(), 1);
    // SAFETY: constructor calls and success preserve the session-owned string.
    assert_eq!(
        unsafe { std::ffi::CStr::from_ptr(facts()) },
        json.as_c_str()
    );
    assert_eq!(crate::failures::retryable(None), 0);
    assert_eq!(
        failures::session_call(|| Err(thinkthen::ErrorKind::Usage)),
        crate::ffi::values::THINKTHEN_EUSAGE
    );
    assert_eq!(retryable(), 0);
    assert!(facts().is_null());
}

#[test]
fn aggregate_immediate_facts_keep_absent_invocation_and_legacy_json() {
    let expected = thinkthen::Tally::new().facts();
    let legacy = serde_json::to_string(&expected).expect("count-only facts");
    failures::session_native_call(|| {
        Err(
            thinkthen::Error::new(thinkthen::ErrorKind::Local, "safe aggregate")
                .with_facts(expected.clone())
                .into(),
        )
    });
    typed_errors::check(&expected, crate::failures::session_view(), facts());
    // SAFETY: the calling thread preserves its independent failure slot.
    assert_eq!(
        unsafe { std::ffi::CStr::from_ptr(facts()) }
            .to_str()
            .expect("facts UTF-8"),
        legacy
    );
}
