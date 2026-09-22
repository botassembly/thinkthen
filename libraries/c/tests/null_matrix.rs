//! The null and length matrix the header promises: one test a row of the
//! argument rules, each with the code and the "nothing was sent" proof.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`. Serialized with
//! `--test-threads=1`, because the refusal rows compare the stand-in's
//! process-global request counter before and after their call.

use std::ffi::{c_char, CStr, CString};

use thinkthen::thinkthen_engine;

/// A judgment that was never written, so a refusal is visible in the out.
const UNWRITTEN: thinkthen::thinkthen_answer = thinkthen::thinkthen_answer {
    outcome: 7,
    probability: -1.0,
};

unsafe fn engine() -> *mut thinkthen_engine {
    thinkthen::thinkthen_engine_new()
}

unsafe fn message(engine: *const thinkthen_engine) -> String {
    CStr::from_ptr(thinkthen::thinkthen_error_message(engine))
        .to_string_lossy()
        .into_owned()
}

/// The door's own request counter through the JSON door; reading the
/// counters sends nothing.
unsafe fn usage_of(engine: *const thinkthen_engine) -> u64 {
    let request = CString::new(r#"{"usage": true}"#).expect("static");
    let reply = thinkthen::thinkthen_call(engine, request.as_ptr());
    assert!(!reply.is_null(), "the counters answer");
    let text = CStr::from_ptr(reply).to_string_lossy().into_owned();
    thinkthen::thinkthen_free_string(reply);
    let value: serde_json::Value = serde_json::from_str(&text).expect("the counters are JSON");
    value["requests"].as_u64().unwrap_or(0)
}

/// A null engine is the usage code everywhere, and the error surface
/// answers without one. No message is available, because no engine holds
/// one.
#[test]
fn a_null_engine_is_the_usage_code_everywhere() {
    unsafe {
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let spec = CString::new(r#"{"kinds": ["person"]}"#).expect("static");
        let mut answer = UNWRITTEN;
        let mut out: *mut c_char = std::ptr::null_mut();
        let mut out_len = 0_usize;

        let code = thinkthen::thinkthen_decide(
            std::ptr::null(),
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 1, "a null engine is the usage code");
        assert_eq!(answer.outcome, 7, "the out holds what it held");

        let code = thinkthen::thinkthen_decide_opts(
            std::ptr::null(),
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            -1,
            std::ptr::null(),
            &mut answer,
        );
        assert_eq!(code, 1);

        let code = thinkthen::thinkthen_decide_many(
            std::ptr::null(),
            question.as_ptr(),
            &std::ptr::null(),
            &0,
            0,
            std::ptr::null_mut(),
        );
        assert_eq!(code, 1);

        let reply = thinkthen::thinkthen_call(std::ptr::null(), question.as_ptr());
        assert!(reply.is_null());
        let reply = thinkthen::thinkthen_call_opts(std::ptr::null(), question.as_ptr(), -1, std::ptr::null());
        assert!(reply.is_null());

        let code = thinkthen::thinkthen_recognize(
            std::ptr::null(),
            spec.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, 1);
        let code = thinkthen::thinkthen_recognize_opts(
            std::ptr::null(),
            spec.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            -1,
            std::ptr::null(),
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, 1);

        let texts = [evidence.as_ptr()];
        let lengths = [evidence.as_bytes().len()];
        let code = thinkthen::thinkthen_relate(
            std::ptr::null(),
            spec.as_ptr(),
            texts.as_ptr(),
            lengths.as_ptr(),
            1,
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, 1);
        let code = thinkthen::thinkthen_relate_opts(
            std::ptr::null(),
            spec.as_ptr(),
            texts.as_ptr(),
            lengths.as_ptr(),
            1,
            -1,
            std::ptr::null(),
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, 1);

        assert_eq!(thinkthen::thinkthen_error_code(std::ptr::null()), 1);
        assert_eq!(thinkthen::thinkthen_error_retryable(std::ptr::null()), 0);
        assert!(!thinkthen::thinkthen_error_message(std::ptr::null()).is_null());

        // The null-tolerant values: free and fire accept null, as the
        // header says.
        thinkthen::thinkthen_engine_free(std::ptr::null_mut());
        thinkthen::thinkthen_cancel_token_free(std::ptr::null_mut());
        thinkthen::thinkthen_cancel(std::ptr::null_mut());
    }
}

/// A null or non-UTF-8 required string is the usage kind, and the door
/// checks it before it asks the engine.
#[test]
fn a_missing_string_is_refused_and_sends_nothing() {
    unsafe {
        let engine = engine();
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let spec = CString::new(r#"{"kinds": ["person"]}"#).expect("static");
        let before = usage_of(engine);

        let mut answer = UNWRITTEN;
        let code = thinkthen::thinkthen_decide(
            engine,
            std::ptr::null(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null question"), "{}", message(engine));
        assert_eq!(answer.outcome, 7);

        let broken = CString::new(vec![0xFF_u8, 0xFE]).expect("no NUL");
        let code = thinkthen::thinkthen_decide(
            engine,
            broken.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("not UTF-8"), "{}", message(engine));

        let reply = thinkthen::thinkthen_call(engine, std::ptr::null());
        assert!(reply.is_null());
        assert!(message(engine).contains("null request"), "{}", message(engine));

        let mut out: *mut c_char = std::ptr::null_mut();
        let mut out_len = 0_usize;
        let code = thinkthen::thinkthen_recognize(
            engine,
            std::ptr::null(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null spec"), "{}", message(engine));

        let texts = [spec.as_ptr()];
        let lengths = [spec.as_bytes().len()];
        let code = thinkthen::thinkthen_relate(
            engine,
            std::ptr::null(),
            texts.as_ptr(),
            lengths.as_ptr(),
            1,
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null spec"), "{}", message(engine));

        assert_eq!(usage_of(engine), before, "every refusal sent nothing");
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The text rule: null needs a zero length, and the two spellings of the
/// empty text agree, with the engine's own blank-evidence rule deciding
/// the code.
#[test]
fn a_null_text_needs_a_zero_length() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let before = usage_of(engine);

        let mut answer = UNWRITTEN;
        let code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            std::ptr::null(),
            5,
            &mut answer,
        );
        assert_eq!(code, 1);
        assert!(
            message(engine).contains("null text with a nonzero length"),
            "{}",
            message(engine)
        );
        assert_eq!(answer.outcome, 7);
        assert_eq!(usage_of(engine), before, "the refusal sent nothing");

        let mut from_null = UNWRITTEN;
        let mut from_empty = UNWRITTEN;
        let null_code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            std::ptr::null(),
            0,
            &mut from_null,
        );
        let empty = CString::new("").expect("static");
        let empty_code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            empty.as_ptr(),
            0,
            &mut from_empty,
        );
        assert_eq!(null_code, 1, "the blank evidence is a usage refusal: {}", message(engine));
        assert_eq!(empty_code, null_code, "the two empty spellings agree");
        assert!(
            message(engine).contains("evidence is text, not white space"),
            "{}",
            message(engine)
        );
        assert_eq!(from_null.outcome, 7);
        assert_eq!(from_empty.outcome, 7);

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The out pointers are written on success, so null is refused before the
/// engine is asked.
#[test]
fn a_null_out_pointer_is_refused_before_the_engine() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let spec = CString::new(r#"{"kinds": ["person"]}"#).expect("static");
        let before = usage_of(engine);

        let code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            std::ptr::null_mut(),
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null out pointer"), "{}", message(engine));

        let texts: Vec<CString> = ["one", "two"]
            .iter()
            .map(|text| CString::new(*text).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
        let code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            2,
            std::ptr::null_mut(),
        );
        assert_eq!(code, 1);
        assert!(
            message(engine).contains("null out array"),
            "{}",
            message(engine)
        );

        let mut out: *mut c_char = std::ptr::null_mut();
        let mut out_len = 0_usize;
        let code = thinkthen::thinkthen_recognize(
            engine,
            spec.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            std::ptr::null_mut(),
            &mut out_len,
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null out pointer"), "{}", message(engine));
        let code = thinkthen::thinkthen_recognize(
            engine,
            spec.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut out,
            std::ptr::null_mut(),
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null out_len"), "{}", message(engine));

        let texts = [evidence.as_ptr()];
        let lengths = [evidence.as_bytes().len()];
        let code = thinkthen::thinkthen_relate(
            engine,
            spec.as_ptr(),
            texts.as_ptr(),
            lengths.as_ptr(),
            1,
            std::ptr::null_mut(),
            &mut out_len,
        );
        assert_eq!(code, 1);
        assert!(message(engine).contains("null out pointer"), "{}", message(engine));

        assert_eq!(usage_of(engine), before, "every refusal sent nothing");
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// A null array is refused only when its count is nonzero; a count of zero
/// reads and writes nothing, so null arrays and a null out are accepted.
#[test]
fn a_null_array_is_refused_only_with_a_nonzero_count() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let before = usage_of(engine);

        let mut held = [UNWRITTEN; 3];
        let code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            3,
            held.as_mut_ptr(),
        );
        assert_eq!(code, 1);
        assert!(
            message(engine).contains("null texts array"),
            "{}",
            message(engine)
        );
        assert_eq!(held[0].outcome, 7);

        let texts: Vec<CString> = ["one", "two", "three"]
            .iter()
            .map(|text| CString::new(*text).expect("no NUL"))
            .collect();
        let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
        let code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            std::ptr::null(),
            3,
            held.as_mut_ptr(),
        );
        assert_eq!(code, 1);
        assert!(
            message(engine).contains("null lengths array"),
            "{}",
            message(engine)
        );

        // The engine's own empty rule: nothing is read or written, and the
        // call succeeds without sending.
        let code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
        );
        assert_eq!(code, 0, "{}", message(engine));
        assert_eq!(usage_of(engine), before, "a zero-count call sent nothing");

        thinkthen::thinkthen_engine_free(engine);
    }
}
