//! The door's null suite: the typed doors, the error codes a host sees,
//! and the JSON door's replies, all against the in-process backend.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`.

use std::ffi::{c_char, CString};

use thinkthen::thinkthen_engine;

const YES: i32 = 1;
const NO: i32 = 0;
const UNSURE: i32 = 2;

unsafe fn engine() -> *mut thinkthen_engine {
    thinkthen::thinkthen_engine_new()
}

unsafe fn message(engine: *const thinkthen_engine) -> String {
    std::ffi::CStr::from_ptr(thinkthen::thinkthen_error_message(engine))
        .to_string_lossy()
        .into_owned()
}

#[test]
fn the_bare_question_and_the_grammar_settle_the_same() {
    unsafe {
        let engine = engine();
        let bare = CString::new("Does the customer ask for a refund?").expect("static");
        let grammar = CString::new(
            r#"{"decide": "Does the customer ask for a refund?", "threshold": 0.5}"#,
        )
        .expect("static");
        let evidence =
            CString::new("I want a refund for order 9").expect("static");
        let text_len = evidence.as_bytes().len();

        let mut from_bare = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };
        let mut from_grammar = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };

        let bare_code = thinkthen::thinkthen_decide(
            engine,
            bare.as_ptr(),
            evidence.as_ptr(),
            text_len,
            &mut from_bare,
        );
        let grammar_code = thinkthen::thinkthen_decide(
            engine,
            grammar.as_ptr(),
            evidence.as_ptr(),
            text_len,
            &mut from_grammar,
        );

        assert_eq!(bare_code, 0, "the bare question asks");
        assert_eq!(grammar_code, 0, "the grammar asks");
        assert_eq!(from_bare.outcome, from_grammar.outcome);
        assert_eq!(from_bare.probability, from_grammar.probability);
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn a_band_reaches_the_unsure_code() {
    unsafe {
        let engine = engine();
        let question = CString::new(
            r#"{"decide": "Is this a maybe?", "threshold": "0.2:0.8"}"#,
        )
        .expect("static");
        let evidence = CString::new("maybe a discount").expect("static");
        let mut answer = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };
        let code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 0);
        assert_eq!(answer.outcome, UNSURE, "the band calls the middle answer unsure");
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn decide_many_keeps_the_input_order() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let texts: Vec<CString> = ["I want a refund for order 9", "just saying hi", "maybe a discount"]
            .iter()
            .map(|text| CString::new(*text).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> =
            texts.iter().map(|text| text.as_ptr()).collect();
        let mut out = [
            thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 },
            thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 },
            thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 },
        ];
        let code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            3,
            out.as_mut_ptr(),
        );
        assert_eq!(code, 0);
        assert_eq!(out[0].outcome, YES);
        assert_eq!(out[1].outcome, NO);
        assert_eq!(out[2].outcome, YES);
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn a_broken_question_is_the_usage_code_with_nothing_sent() {
    unsafe {
        let engine = engine();
        let broken = CString::new(r#"{"choose": "Pick.", "options": []}"#).expect("static");
        let evidence = CString::new("anything").expect("static");
        let usage_before = usage_of(engine);
        let mut answer = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };
        let code = thinkthen::thinkthen_decide(
            engine,
            broken.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 1, "usage is the first code");
        assert!(!message(engine).is_empty());
        assert_eq!(
            thinkthen::thinkthen_error_retryable(engine),
            0,
            "a usage failure is not retryable"
        );
        assert_eq!(
            usage_of(engine),
            usage_before,
            "nothing was sent for a usage failure"
        );
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn the_json_door_answers_every_verb_it_carries() {
    unsafe {
        let engine = engine();

        let decide = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9"}"#,
        ));
        assert_eq!(decide["answer"], serde_json::json!(true), "decide answers");

        let band = take(json(
            engine,
            r#"{"decide": "Is this a maybe?", "threshold": "0.2:0.8", "evidence": "maybe a discount"}"#,
        ));
        assert_eq!(band["answer"], serde_json::json!(null), "unsure is null");

        let choice = take(json(
            engine,
            r#"{"choose": "Which team owns this?", "options": ["the refund desk", "the maybe desk", "anywhere else"], "threshold": 0.5, "evidence": "please route this ticket"}"#,
        ));
        assert_eq!(
            choice["answer"],
            serde_json::json!("the refund desk"),
            "choose names the winning option"
        );

        let scored = take(json(
            engine,
            r#"{"score": "How strong?", "levels": ["low", "mid", "high"], "evidence": "I want a refund for order 9"}"#,
        ));
        assert!(scored["answer"].is_number(), "score gives the position");
        assert!(scored["nearest"].is_string(), "the nearest level rides beside");

        let tags = take(json(
            engine,
            r#"{"tag": "Name the labels.", "labels": ["refund", "complaint"], "evidence": "I want a refund for order 9"}"#,
        ));
        assert!(tags["answer"].is_array(), "tag gives the labels that held");

        let filtered = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "threshold": 0.5, "records": ["I want a refund for order 9", "just saying hi", "maybe a discount"]}"#,
        ));
        assert_eq!(
            filtered["indexes"],
            serde_json::json!([0, 2]),
            "filter keeps the refund records"
        );

        let audit = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9", "details": true}"#,
        ));
        assert_eq!(audit["answer"], serde_json::json!(true));
        assert!(audit["model"].is_string());
        assert!(audit["digest"].is_string());

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// One JSON-door call through the door's own entry point.
unsafe fn json(engine: *const thinkthen_engine, text: &str) -> *mut c_char {
    let request = CString::new(text).expect("no NUL");
    thinkthen::thinkthen_call(engine, request.as_ptr())
}

/// Run one JSON-door call, parse the reply, and free the string.
unsafe fn take(pointer: *mut c_char) -> serde_json::Value {
    assert!(!pointer.is_null(), "the call succeeded");
    let text = std::ffi::CStr::from_ptr(pointer).to_string_lossy().into_owned();
    thinkthen::thinkthen_free_string(pointer);
    serde_json::from_str(&text).expect("the reply is JSON")
}

/// The door's own counters through the JSON door.
unsafe fn usage_of(engine: *const thinkthen_engine) -> u64 {
    let request = CString::new(r#"{"usage": true}"#).expect("static");
    let reply = thinkthen::thinkthen_call(engine, request.as_ptr());
    let usage = take(reply);
    usage["requests"].as_u64().unwrap_or(0)
}
