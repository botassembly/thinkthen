//! Two threads over one engine, each recording its own failure: each
//! reads its own message back, and neither thread's failure crosses into
//! the other's slot.
//!
//! On the old last-writer-wins slot this test failed: the second failure
//! replaced the first thread's message, so one thread read the other
//! thread's text, and the main thread saw a failure it never made. The
//! sanitizer twin is `tests/error_threads.c`, which check.sh runs under
//! AddressSanitizer with the reviewer's saved-pointer read, where the
//! replaced message was a use-after-free.
//!
//! Run through `./check.sh`, which sets `THINKTHEN_NULL=1`.

use std::ffi::{CStr, CString};
use std::sync::Barrier;

use thinkthen::thinkthen_engine;

/// The null backend, as `check.sh` sets it: a wire run would send real
/// requests, and these tests need the fast, silent backend.
fn null_backend() {
    assert_eq!(
        std::env::var("THINKTHEN_NULL").as_deref(),
        Ok("1"),
        "run this through check.sh, which sets THINKTHEN_NULL=1"
    );
}

/// The message the calling thread recorded on `engine`, copied out so the
/// assertion cannot hold a pointer past its promise.
unsafe fn message(engine: *const thinkthen_engine) -> String {
    unsafe {
        CStr::from_ptr(thinkthen::thinkthen_error_message(engine))
            .to_string_lossy()
            .into_owned()
    }
}

#[test]
fn two_threads_read_their_own_messages() {
    null_backend();
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();
        assert!(!engine.is_null(), "the engine builds");
        let barrier = Barrier::new(2);
        let requests = [
            r#"{"nope": true}"#,
            r#"{"choose": "Pick.", "options": ["a", "b"]}"#,
        ];
        let own = ["no verb the door knows", "no evidence string"];
        let other = [own[1], own[0]];

        // A raw pointer is not `Send`; each thread gets its address and
        // rebuilds it, the same shape `tests/concurrency.rs` uses.
        let key = engine as usize;
        let barrier = &barrier;
        std::thread::scope(|scope| {
            let mut handles = Vec::new();
            for index in 0..2 {
                handles.push(scope.spawn(move || {
                    let engine = key as *mut thinkthen_engine;
                    let request = CString::new(requests[index]).expect("static");
                    let reply = thinkthen::thinkthen_call_opts(
                        engine,
                        request.as_ptr(),
                        -1,
                        std::ptr::null(),
                    );
                    assert!(reply.is_null(), "a broken request answers nothing");
                    assert_eq!(
                        thinkthen::thinkthen_error_code(engine),
                        1,
                        "the failure is the usage kind"
                    );
                    let mine = message(engine);
                    assert!(mine.contains(own[index]), "this thread's own message: {mine}");

                    // The other thread fails too, then this thread reads
                    // again: its own message is untouched.
                    barrier.wait();
                    let again = message(engine);
                    assert!(
                        again.contains(own[index]),
                        "the message after the other thread's failure: {again}"
                    );
                    assert!(
                        !again.contains(other[index]),
                        "the other thread's text crossed into this slot: {again}"
                    );
                }));
            }
            for handle in handles {
                handle.join().expect("the thread finishes");
            }
        });

        // The main thread recorded no failure, so it reads none: failures
        // belong to the thread that made them.
        assert_eq!(message(engine), "no failure yet");
        thinkthen::thinkthen_engine_free(engine);
    }
}
