//! The door's wire suite, against the stub on this surface's port (8216).
//!
//! The stub is the in-repo tools/wire-stub running with
//! `STUB_PORT=8216 STUB_DELAY_MS=300`. Run through `./check.sh`, which
//! starts nothing itself: when no stub is up the suite is skipped. The
//! dead-address test needs no stub, and it clears the null switch itself,
//! so a null run cannot answer it from the in-process backend (R2-28).

use std::ffi::CString;


/// A dead loopback port: the backend kind with no retry, the shape a
/// caller with a fallback needs to tell from a busy backend.
const DEAD: &str = "http://127.0.0.1:9/v1";

#[test]
fn a_dead_address_is_the_backend_code_and_not_retryable() {
    unsafe {
        std::env::remove_var("THINKTHEN_NULL");
        std::env::remove_var("ENGINE_NULL");
        std::env::set_var("ENGINE_BASE_URL", DEAD);
        let engine = thinkthen::thinkthen_engine_new();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let mut answer =
            thinkthen::thinkthen_answer { outcome: 0, probability: 0.0 };

        let code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );

        assert_eq!(code, 2, "backend is the second code");
        let message = std::ffi::CStr::from_ptr(
            thinkthen::thinkthen_error_message(engine),
        )
        .to_string_lossy()
        .into_owned();
        assert!(!message.is_empty(), "the failure carries a message");
        assert_eq!(
            thinkthen::thinkthen_error_retryable(engine),
            0,
            "a refused address is not retryable"
        );
        thinkthen::thinkthen_engine_free(engine);
        std::env::remove_var("ENGINE_BASE_URL");
    }
}
