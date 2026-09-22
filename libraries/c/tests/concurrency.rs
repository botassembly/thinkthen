//! The thread promise: one engine serves four threads at once, each
//! caller sees the answers it would get alone, and every call reaches the
//! engine exactly once.
//!
//! Its own test process, so the counter assertion cannot see another
//! test's calls. Run through `./check.sh`, which sets `ENGINE_NULL=1`.

use std::ffi::{CStr, CString};

use thinkthen::thinkthen_engine;

/// Calls each thread makes.
const CALLS: usize = 25;

/// The engine pointer as a thread-safe value. The engine outlives every
/// thread here and the door promises concurrent calls, so the copy is
/// sound; the marker is needed only because the raw pointer type carries
/// no `Send`.
#[derive(Clone, Copy)]
struct Shared(*const thinkthen_engine);
unsafe impl Send for Shared {}

impl Shared {
    /// The engine pointer for the call in hand.
    fn engine(self) -> *const thinkthen_engine {
        self.0
    }
}

/// A judgment that was never written, so a refusal is visible.
const UNWRITTEN: thinkthen::thinkthen_answer = thinkthen::thinkthen_answer {
    outcome: 7,
    probability: -1.0,
};

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


/// The null backend, as `check.sh` sets it: a wire run would send real
/// requests, and these tests need the fast, silent backend.
fn null_backend() {
    assert_eq!(
        std::env::var("ENGINE_NULL").as_deref(),
        Ok("1"),
        "run this through check.sh, which sets ENGINE_NULL=1"
    );
}

#[test]
fn four_threads_share_one_engine() {
    null_backend();
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();
        assert!(!engine.is_null(), "the engine builds");
        let shared = Shared(engine);

        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let broken = CString::new(r#"{"choose": "Pick.", "options": []}"#).expect("static");
        let request = CString::new(
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9"}"#,
        )
        .expect("static");
        // The closures below copy these references, so every thread reads
        // the same strings.
        let (question, evidence, broken, request) = (&question, &evidence, &broken, &request);

        std::thread::scope(|scope| {
            // The plain typed call, over and over.
            scope.spawn(move || {
                for _ in 0..CALLS {
                    let mut answer = UNWRITTEN;
                    let code = thinkthen::thinkthen_decide(
                        shared.engine(),
                        question.as_ptr(),
                        evidence.as_ptr(),
                        evidence.as_bytes().len(),
                        &mut answer,
                    );
                    assert_eq!(code, 0, "the plain decide answers");
                    assert_eq!(answer.outcome, 1, "the refund answers yes");
                }
            });
            // The same call through its `_opts` twin.
            scope.spawn(move || {
                for _ in 0..CALLS {
                    let mut answer = UNWRITTEN;
                    let code = thinkthen::thinkthen_decide_opts(
                        shared.engine(),
                        question.as_ptr(),
                        evidence.as_ptr(),
                        evidence.as_bytes().len(),
                        -1,
                        std::ptr::null(),
                        &mut answer,
                    );
                    assert_eq!(code, 0, "the `_opts` decide answers");
                    assert_eq!(answer.outcome, 1);
                }
            });
            // The JSON door.
            scope.spawn(move || {
                for _ in 0..CALLS {
                    let reply = thinkthen::thinkthen_call(shared.engine(), request.as_ptr());
                    assert!(!reply.is_null(), "the JSON door answers");
                    let text = CStr::from_ptr(reply).to_string_lossy().into_owned();
                    thinkthen::thinkthen_free_string(reply);
                    let value: serde_json::Value =
                        serde_json::from_str(&text).expect("the reply is JSON");
                    assert_eq!(value["answer"], serde_json::json!(true));
                }
            });
            // A failing caller beside the answering ones: its own code and
            // its own message come back on every call.
            scope.spawn(move || {
                for _ in 0..CALLS {
                    let mut answer = UNWRITTEN;
                    let code = thinkthen::thinkthen_decide(
                        shared.engine(),
                        broken.as_ptr(),
                        evidence.as_ptr(),
                        evidence.as_bytes().len(),
                        &mut answer,
                    );
                    assert_eq!(code, 1, "the broken question is the usage code");
                    assert_eq!(
                        thinkthen::thinkthen_error_code(shared.engine()),
                        1,
                        "the failing caller reads its own code"
                    );
                    let message = CStr::from_ptr(thinkthen::thinkthen_error_message(
                        shared.engine(),
                    ))
                    .to_string_lossy()
                    .into_owned();
                    assert!(
                        message.contains("choose"),
                        "the failing caller reads its own message: {message}"
                    );
                    assert_eq!(answer.outcome, 7, "a refusal writes nothing");
                }
            });
        });

        // Every answering call reached the engine exactly once, and the
        // refusals sent nothing: three threads of CALLS sends each.
        assert_eq!(
            usage_of(engine),
            (3 * CALLS) as u64,
            "one send a call, none lost and none doubled"
        );
        // The main thread recorded no failure of its own, so the per-thread
        // slot has none for it: the failures live on the threads that made
        // them.
        assert_eq!(
            thinkthen::thinkthen_error_code(engine),
            0,
            "the main thread recorded no failure"
        );
        thinkthen::thinkthen_engine_free(engine);
    }
}
