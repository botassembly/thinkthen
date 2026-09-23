//! Failures are scoped to the engine that recorded them: a failure on one
//! engine never makes another engine read "no failure", and a freed
//! engine's failure never appears on a new engine that reuses its address.
//!
//! On the old one-slot-per-thread table both broke: a second engine's
//! failure erased the first engine's (so the first read "no failure yet"),
//! and a thread's entry for a freed engine survived the free, so a new
//! engine at the same address inherited the dead engine's message. The
//! table now lives in the engine, keyed by the recording thread.
//!
//! Every call here is refused before anything reaches the wire, so the
//! suite needs no stub and no `THINKTHEN_NULL`.

use std::ffi::{CStr, CString};

use thinkthen::thinkthen_engine;

/// A judgment that was never written, so a refusal is visible in the out.
const UNWRITTEN: thinkthen::thinkthen_answer = thinkthen::thinkthen_answer {
    outcome: 7,
    probability: -1.0,
};

/// The message the calling thread recorded on `engine`, copied out so the
/// assertion cannot hold a pointer past its promise.
unsafe fn message(engine: *const thinkthen_engine) -> String {
    unsafe { CStr::from_ptr(thinkthen::thinkthen_error_message(engine)) }
        .to_string_lossy()
        .into_owned()
}

/// Record one usage failure on `engine` through the public door: a null
/// question is refused before anything is sent.
unsafe fn record_failure(engine: *const thinkthen_engine) {
    let evidence = CString::new("i want a refund").expect("static");
    let mut answer = UNWRITTEN;
    let code = unsafe {
        thinkthen::thinkthen_decide(
            engine,
            std::ptr::null(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        )
    };
    assert_eq!(code, 1, "a null question is the usage kind");
    assert_eq!(answer.outcome, 7, "a refusal writes nothing");
}

/// A failure on one engine does not hide another engine's: each engine
/// keeps its own failures, so both read back what they recorded.
#[test]
fn a_failure_on_one_engine_does_not_hide_another() {
    unsafe {
        let first = thinkthen::thinkthen_engine_new();
        let second = thinkthen::thinkthen_engine_new();
        assert!(
            !first.is_null() && !second.is_null(),
            "both engines build without a wire"
        );
        record_failure(first);
        record_failure(second);

        assert_eq!(
            thinkthen::thinkthen_error_code(first),
            1,
            "the first engine still reports its own failure"
        );
        assert_eq!(
            thinkthen::thinkthen_error_code(second),
            1,
            "the second engine reports its own failure"
        );
        assert!(
            message(first).contains("a null question"),
            "the first engine's own message: {}",
            message(first)
        );
        assert!(
            message(second).contains("a null question"),
            "the second engine's own message: {}",
            message(second)
        );

        // Now clear the second engine's failure by succeeding on it? A
        // success never clears it (the header's rule); free both instead.
        thinkthen::thinkthen_engine_free(first);
        thinkthen::thinkthen_engine_free(second);
    }
}

/// A freed engine's failure does not ride its reused address: the reader
/// thread recorded a failure on the first engine, another thread frees it
/// and builds a second engine at the same address, and the reader must
/// read no failure from the second engine.
#[test]
fn a_freed_engines_failure_does_not_ride_its_reused_address() {
    let (address_tx, address_rx) = std::sync::mpsc::channel::<usize>();
    let (reused_tx, reused_rx) = std::sync::mpsc::channel::<usize>();

    std::thread::scope(|scope| {
        let reader = scope.spawn(move || {
            let engine = unsafe { thinkthen::thinkthen_engine_new() };
            assert!(!engine.is_null(), "the reader's engine builds");
            unsafe { record_failure(engine) };
            address_tx.send(engine as usize).expect("the address goes");

            let reused = reused_rx.recv().expect("the reused address comes");
            let fresh = reused as *mut thinkthen_engine;
            assert_eq!(
                unsafe { thinkthen::thinkthen_error_code(fresh) },
                0,
                "a new engine at the old address starts with no failure"
            );
            assert_eq!(
                unsafe { message(fresh) },
                "no failure yet",
                "the dead engine's message must not appear here"
            );
        });

        let old = address_rx.recv().expect("the old address comes");
        unsafe { thinkthen::thinkthen_engine_free(old as *mut thinkthen_engine) };

        // The same size class reuses the block at once; loop only as a
        // guard against allocator noise from other suites.
        let mut fresh = std::ptr::null_mut();
        for _ in 0..16 {
            fresh = unsafe { thinkthen::thinkthen_engine_new() };
            assert!(!fresh.is_null(), "the fresh engine builds");
            if fresh as usize == old {
                break;
            }
            unsafe { thinkthen::thinkthen_engine_free(fresh) };
        }
        assert_eq!(
            fresh as usize, old,
            "the allocator reused the freed engine's address"
        );
        reused_tx.send(fresh as usize).expect("the address goes");

        reader.join().expect("the reader finishes");
        unsafe { thinkthen::thinkthen_engine_free(fresh) };
    });
}
