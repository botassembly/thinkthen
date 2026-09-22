//! The cancel token the header promises: one handle, fired from any
//! thread, ends every call that carries it with the cancelled code and no
//! results.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`. Serialized with
//! `--test-threads=1`, because the "nothing was sent" rows compare the
//! stand-in's process-global request counter.

use std::ffi::{c_char, CStr, CString};
use std::time::{Duration, Instant};

use thinkthen::thinkthen_cancel_token;
use thinkthen::thinkthen_engine;

const CANCELLED: i32 = 5;
const NO_DEADLINE: std::ffi::c_long = -1;

/// A judgment that was never written, so a cancel is visible in the out.
const UNWRITTEN: thinkthen::thinkthen_answer = thinkthen::thinkthen_answer {
    outcome: 7,
    probability: -1.0,
};

unsafe fn engine() -> *mut thinkthen_engine {
    thinkthen::thinkthen_engine_new()
}


/// The null backend, as `check.sh` sets it: a wire run would send real
/// requests, and these tests need the fast, silent backend.
fn null_backend() {
    assert_eq!(
        std::env::var("ENGINE_NULL").as_deref(),
        Ok("1"),
        "run this through check.sh, which sets ENGINE_NULL=1"
    );
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

/// A fired token refuses every call that carries it, before anything is
/// sent, and lands no results.
#[test]
fn a_fired_token_refuses_every_call_that_carries_it() {
    unsafe {
        let engine = engine();
        let token = thinkthen::thinkthen_cancel_token_new();
        assert!(!token.is_null(), "the token builds");
        thinkthen::thinkthen_cancel(token);
        let before = usage_of(engine);

        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");

        let mut answer = UNWRITTEN;
        let code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            token,
            &mut answer,
        );
        assert_eq!(code, CANCELLED, "{}", message(engine));
        assert!(message(engine).contains("cancelled"), "{}", message(engine));
        assert_eq!(
            thinkthen::thinkthen_error_retryable(engine),
            0,
            "a cancel is not retryable"
        );
        assert_eq!(answer.outcome, 7, "no result lands on a cancel");

        let texts: Vec<CString> = ["one", "two"]
            .iter()
            .map(|text| CString::new(*text).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
        let mut held = [UNWRITTEN; 2];
        let code = thinkthen::thinkthen_decide_many_opts(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            2,
            NO_DEADLINE,
            token,
            held.as_mut_ptr(),
        );
        assert_eq!(code, CANCELLED);
        assert_eq!(held[0].outcome, 7, "no row lands on a cancel");

        let request = CString::new(
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9"}"#,
        )
        .expect("static");
        let reply = thinkthen::thinkthen_call_opts(engine, request.as_ptr(), NO_DEADLINE, token);
        assert!(reply.is_null());
        assert_eq!(thinkthen::thinkthen_error_code(engine), CANCELLED);

        let spec = CString::new(r#"{"kinds": ["person"]}"#).expect("static");
        let mut out: *mut c_char = std::ptr::null_mut();
        let mut out_len = 0_usize;
        let code = thinkthen::thinkthen_recognize_opts(
            engine,
            spec.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            token,
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, CANCELLED);

        let texts = [evidence.as_ptr()];
        let lengths = [evidence.as_bytes().len()];
        let code = thinkthen::thinkthen_relate_opts(
            engine,
            spec.as_ptr(),
            texts.as_ptr(),
            lengths.as_ptr(),
            1,
            NO_DEADLINE,
            token,
            &mut out,
            &mut out_len,
        );
        assert_eq!(code, CANCELLED);

        assert_eq!(usage_of(engine), before, "a fired token sends nothing");
        thinkthen::thinkthen_cancel_token_free(token);
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// One-shot: an unfired token changes nothing, a second fire is ignored,
/// the token stays fired, and a null token is no token.
#[test]
fn a_token_is_one_shot_and_a_null_token_is_none() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let token = thinkthen::thinkthen_cancel_token_new();

        let mut plain = UNWRITTEN;
        let mut tokenized = UNWRITTEN;
        let plain_code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut plain,
        );
        let tokenized_code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            token,
            &mut tokenized,
        );
        assert_eq!(plain_code, 0, "{}", message(engine));
        assert_eq!(tokenized_code, plain_code, "an unfired token changes nothing");
        assert_eq!(tokenized.outcome, plain.outcome);

        thinkthen::thinkthen_cancel(token);
        thinkthen::thinkthen_cancel(token);
        let mut after = UNWRITTEN;
        let first = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            token,
            &mut after,
        );
        let second = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            token,
            &mut after,
        );
        assert_eq!(first, CANCELLED);
        assert_eq!(second, CANCELLED, "a second fire is ignored, the token stays fired");

        let mut untokenized = UNWRITTEN;
        let code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            std::ptr::null(),
            &mut untokenized,
        );
        assert_eq!(code, 0, "a null token is no token: {}", message(engine));

        thinkthen::thinkthen_cancel_token_free(token);
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The cross-thread shape a C host uses for its own interrupt: fire the
/// token from another thread while a two-million-record batch runs. The
/// batch finishes in about 5.6 s without the token; the cancel must end it
/// within about a tick, with no rows.
#[test]
fn a_cancel_from_another_thread_ends_a_running_batch() {
    null_backend();
    /// The raw pointer as a thread-safe value; the token outlives the
    /// firer, which the join below proves.
    #[derive(Clone, Copy)]
    struct Shared(*mut thinkthen_cancel_token);
    unsafe impl Send for Shared {}

    impl Shared {
        /// The token pointer, for the fire in hand.
        fn token(self) -> *mut thinkthen_cancel_token {
            self.0
        }
    }

    unsafe {
        let engine = engine();
        let token = thinkthen::thinkthen_cancel_token_new();
        let question = CString::new("Is this a complaint?").expect("static");
        let texts: Vec<CString> = (0..2_000_000)
            .map(|index| CString::new(format!("record {index}")).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
        let mut out: Vec<_> = (0..texts.len()).map(|_| UNWRITTEN).collect();

        let firer = std::thread::spawn({
            let shared = Shared(token);
            move || {
                std::thread::sleep(Duration::from_millis(50));
                thinkthen::thinkthen_cancel(shared.token());
            }
        });

        let started = Instant::now();
        let code = thinkthen::thinkthen_decide_many_opts(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            texts.len(),
            NO_DEADLINE,
            token,
            out.as_mut_ptr(),
        );
        let took = started.elapsed();
        firer.join().expect("the firer thread joins");

        assert_eq!(code, CANCELLED, "{}", message(engine));
        assert!(
            took < Duration::from_millis(1_500),
            "the cancel landed at {took:?}; the whole batch runs about 5.6 s"
        );
        assert!(
            out.iter().take(4).all(|answer| answer.outcome == 7),
            "no rows land on a cancel"
        );
        assert_eq!(out[out.len() - 1].outcome, 7);

        thinkthen::thinkthen_cancel_token_free(token);
        thinkthen::thinkthen_engine_free(engine);
    }
}
