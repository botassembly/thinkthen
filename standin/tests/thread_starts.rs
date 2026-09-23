//! What the engine does with threads: a request starts none of its own,
//! and a thread that cannot start fails the call instead of panicking.
//!
//! This binary defines `pthread_create` itself and forwards to the C
//! library's, so it counts every thread the process starts and can refuse
//! chosen starts on the calling thread. The tests share that state, so
//! they take one lock.
//!
//! A request to a numeric address starts no thread (R4-1, R7-1). ureq's
//! default resolver starts one lookup thread a request whenever a timeout
//! is set, even for an IP literal, and drops its handle. Dropping the
//! handle detaches a thread that is often exiting at that moment, and
//! glibc's `pthread_detach` then reads the thread's record after the
//! exiting thread may have released its stack. Two cores of the C door's
//! churn probe fault on that read. The stand-in's own resolver parses a
//! numeric address on the calling thread.
//!
//! No request detaches a thread either. The binary also defines
//! `pthread_detach` and counts the detaches made on the test's own thread
//! and on the engine's `ttb-` threads. The batch path used to drop its
//! scoped worker and feeder handles, and a scoped handle dropped unjoined
//! detaches its thread, often one that is already exiting.
//!
//! A refused thread start is an error (R4-12). The batch path used to
//! `expect` its worker and feeder starts.

#![cfg(target_os = "linux")]

mod common;

use std::cell::Cell;
use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use thinkthen_contract::{Connector, EngineConfig, ErrorKind, Question};
use thinkthen_standin::StandinConnector;

/// Every thread this process has asked the C library to create.
static CREATED: AtomicUsize = AtomicUsize::new(0);

/// One test at a time: they share the counter and the process.
static SERIAL: Mutex<()> = Mutex::new(());

thread_local! {
    /// The creates made on this thread so far, and the one to refuse.
    static REFUSE: Cell<(usize, Option<usize>)> = const { Cell::new((0, None)) };
}

/// Every detach made on a watched thread.
static DETACHED: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    /// Whether this thread's detaches count: the test's calling thread.
    static WATCHED: Cell<bool> = const { Cell::new(false) };
}

/// Whether the calling thread's detaches count: a watched thread, or one
/// of the engine's own threads, all named `ttb-`.
fn watched_caller() -> bool {
    if WATCHED.try_with(Cell::get).unwrap_or(false) {
        return true;
    }
    let mut name = [0 as libc::c_char; 16];
    // Sound: the buffer holds the 16 bytes Linux allows a thread name.
    let named = unsafe { libc::pthread_getname_np(libc::pthread_self(), name.as_mut_ptr(), 16) };
    named == 0 && name[..4].iter().map(|&byte| byte as u8).eq(*b"ttb-")
}

/// Count a watched detach, then hand it to the C library's own function.
///
/// # Safety
///
/// The caller passes what `pthread_detach` takes; it goes on unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_detach(thread: libc::pthread_t) -> libc::c_int {
    if watched_caller() {
        DETACHED.fetch_add(1, Ordering::SeqCst);
    }
    // Sound: RTLD_NEXT names the C library's definition, whose signature
    // this is, and the argument is passed through unchanged.
    unsafe {
        let real = libc::dlsym(libc::RTLD_NEXT, c"pthread_detach".as_ptr());
        assert!(!real.is_null(), "the C library defines pthread_detach");
        let real: unsafe extern "C" fn(libc::pthread_t) -> libc::c_int = std::mem::transmute(real);
        real(thread)
    }
}

/// The C library's `pthread_create` signature.
type Create = unsafe extern "C" fn(
    *mut libc::pthread_t,
    *const libc::pthread_attr_t,
    extern "C" fn(*mut c_void) -> *mut c_void,
    *mut c_void,
) -> libc::c_int;

/// Count the create, refuse it when this thread asked, and otherwise hand
/// it to the C library's own function.
///
/// # Safety
///
/// The caller passes what `pthread_create` takes; the arguments go on
/// unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_create(
    thread: *mut libc::pthread_t,
    attr: *const libc::pthread_attr_t,
    start: extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> libc::c_int {
    let refused = REFUSE
        .try_with(|plan| {
            let (made, refuse) = plan.get();
            plan.set((made + 1, refuse));
            refuse == Some(made)
        })
        .unwrap_or(false);
    if refused {
        return libc::EAGAIN;
    }
    CREATED.fetch_add(1, Ordering::SeqCst);
    // Sound: RTLD_NEXT names the C library's definition, whose signature
    // is `Create`, and the arguments are passed through unchanged.
    unsafe {
        let real = libc::dlsym(libc::RTLD_NEXT, c"pthread_create".as_ptr());
        assert!(!real.is_null(), "the C library defines pthread_create");
        let real: Create = std::mem::transmute(real);
        real(thread, attr, start, arg)
    }
}

/// An engine on the answering backend, with a timeout set.
fn engine(base: &str, width: usize) -> std::sync::Arc<dyn thinkthen_contract::Engine> {
    StandinConnector
        .connect(&EngineConfig {
            address: Some(base.to_owned()),
            // A timeout is what sent ureq's resolver to its thread.
            timeout: Some(Duration::from_secs(30)),
            max_retries: Some(0),
            width: Some(width),
            ..EngineConfig::default()
        })
        .expect("the engine connects")
}

/// Notes to judge.
fn notes(count: usize) -> Vec<String> {
    (0..count).map(|n| format!("please refund {n}")).collect()
}

fn question() -> Question {
    Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("the question parses")
}

#[test]
fn a_thousand_requests_to_a_numeric_address_start_no_lookup_thread() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    common::wire_only();
    let engine = engine(&common::answering_backend(), 1);
    let notes = notes(1000);
    let records: Vec<&str> = notes.iter().map(String::as_str).collect();

    let before = CREATED.load(Ordering::SeqCst);
    let judged = engine
        .decide_many(&question(), &records, None)
        .expect("the backend answers");
    let started = CREATED.load(Ordering::SeqCst) - before;

    assert_eq!(judged.len(), 1000, "every record was judged");
    // One worker, one feeder, and the backend's one keep-alive connection.
    assert!(
        started <= 3,
        "1000 requests started {started} threads; each request must start none"
    );
}

#[test]
fn a_refused_thread_start_fails_the_call_or_leaves_fewer_workers() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    common::wire_only();
    let base = common::answering_backend();
    let notes = notes(8);
    let records: Vec<&str> = notes.iter().map(String::as_str).collect();
    let refused = "defect: the engine could not start a thread: \
                   Resource temporarily unavailable (os error 11)";

    // Each case: the width, and which start on this thread to refuse. The
    // batch starts its workers first and its feeder last.
    for (width, refuse, answers) in [(4, 0, false), (4, 1, true), (1, 1, false)] {
        let engine = engine(&base, width);
        REFUSE.with(|plan| plan.set((0, Some(refuse))));
        let outcome = engine.decide_many(&question(), &records, None);
        let made = REFUSE.with(|plan| plan.replace((0, None)).0);
        assert!(
            made > refuse,
            "width {width}: start {refuse} was reached and refused"
        );
        match outcome {
            Ok(judged) => {
                assert!(
                    answers,
                    "width {width}, start {refuse} refused: the call answered"
                );
                assert_eq!(judged.len(), 8, "the remaining workers judged every record");
            }
            Err(error) => {
                assert!(!answers, "width {width}, start {refuse} refused: {error}");
                assert_eq!(error.to_string(), refused);
                assert_eq!(error.kind, ErrorKind::Defect);
                assert!(error.retryable, "a later try may find a thread");
            }
        }
    }
}

#[test]
fn no_request_to_a_numeric_address_detaches_a_thread() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    common::wire_only();
    let engine = engine(&common::answering_backend(), 4);
    let notes = notes(1000);
    let records: Vec<&str> = notes.iter().map(String::as_str).collect();

    WATCHED.with(|watched| watched.set(true));
    let before = DETACHED.load(Ordering::SeqCst);
    let judged = engine
        .decide_many(&question(), &records, None)
        .expect("the backend answers");
    // The C door's shape: a batch of one a call.
    for record in &records[..200] {
        engine
            .decide_many(&question(), &[record], None)
            .expect("the backend answers");
    }
    let detached = DETACHED.load(Ordering::SeqCst) - before;
    WATCHED.with(|watched| watched.set(false));

    assert_eq!(judged.len(), 1000, "every record was judged");
    assert_eq!(
        detached, 0,
        "1,200 requests in 201 calls detached {detached} threads"
    );
}
