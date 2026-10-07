//! Real native control/error serialization checks, not complete-result parity.
use super::super::output::{NativeObject, native_error, tool_result};
use crate::{CallOptions, CancelToken, Engine, ErrorKind, Question};
use conformance_backend::Backend;

fn engine(base: &str) -> Engine {
    Engine::builder()
        .base_url(base)
        .unwrap()
        .api_key("sk-mcp-fake-only")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap()
}

#[test]
fn started_backend_errors_preserve_observed_facts_and_secrecy() {
    let backend = Backend::start().unwrap();
    let engine = engine(&format!("{}/arm/refuse/v1", backend.origin()));
    let question = Question::decide("Question contains private payload")
        .unwrap()
        .cut();
    let failure = engine.decide(&question, "private-evidence").unwrap_err();
    assert_eq!(backend.count(), 1);
    let expected =
        serde_json::to_value(failure.facts().expect("started facts").complete().unwrap()).unwrap();
    let reply = serde_json::to_value(tool_result(
        NativeObject::new(&native_error(&failure)).unwrap(),
        true,
    ))
    .unwrap();
    assert_eq!(reply["structuredContent"]["facts"], expected);
    assert_eq!(reply["structuredContent"]["error"]["kind"], "backend");
    assert_eq!(reply["structuredContent"]["facts"]["requests_sent"], 1);
    for secret in ["sk-mcp-fake-only", "private-evidence", "private payload"] {
        assert!(!reply.to_string().contains(secret));
        assert!(!format!("{engine:?} {failure:?}").contains(secret));
    }
}

#[test]
fn pre_cancel_and_zero_deadline_send_nothing_and_do_not_fabricate_success() {
    let backend = Backend::start().unwrap();
    let engine = engine(&format!("{}/generic/v1", backend.origin()));
    let question = Question::decide("q").unwrap().cut();
    let token = CancelToken::new();
    token.cancel();
    let cancelled = engine
        .decide_with(&question, "x", CallOptions::new().cancel(&token))
        .unwrap_err();
    assert_eq!(cancelled.kind(), ErrorKind::Cancelled);
    let deadline = engine
        .decide_with(&question, "x", CallOptions::new().deadline_ms(0).unwrap())
        .unwrap_err();
    assert_eq!(deadline.kind(), ErrorKind::Deadline);
    assert_eq!(
        serde_json::to_value(tool_result(
            NativeObject::new(&native_error(&deadline)).unwrap(),
            true
        ))
        .unwrap()["isError"],
        true
    );
    assert_eq!(backend.count(), 0);
}

#[test]
fn a_cancelled_sent_attempt_finishes_before_native_facts_are_serialized() {
    let backend = Backend::start().unwrap();
    let engine = engine(&format!("{}/arm/held/v1", backend.origin()));
    let question = Question::decide("q").unwrap().cut();
    let token = CancelToken::new();
    let failure = std::thread::scope(|scope| {
        let call =
            scope.spawn(|| engine.decide_with(&question, "x", CallOptions::new().cancel(&token)));
        backend.wait(1);
        token.cancel();
        backend.release();
        call.join().unwrap().unwrap_err()
    });
    assert_eq!(backend.count(), 1);
    assert_eq!(failure.kind(), ErrorKind::Cancelled);
    assert_eq!(
        serde_json::to_value(native_error(&failure)).unwrap()["facts"]["requests_sent"],
        1
    );
}

#[test]
fn failed_calls_never_become_cache_answers_and_replay_misses_send_nothing() {
    let backend = Backend::start().unwrap();
    let folder = std::env::temp_dir().join(format!("thinkthen-mcp-cache-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let engine = Engine::builder()
        .base_url(&format!("{}/arm/refuse/v1", backend.origin()))
        .unwrap()
        .api_key("sk-mcp-fake-only")
        .unwrap()
        .cache_at(&folder)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let question = Question::decide("q").unwrap().cut();
    for expected in 1..=2 {
        let failure = engine.decide(&question, "x").unwrap_err();
        assert_eq!(failure.kind(), ErrorKind::Backend);
        assert_eq!(
            serde_json::to_value(native_error(&failure)).unwrap()["facts"]["requests_sent"],
            1
        );
        assert_eq!(backend.count(), expected);
    }
    let replay = Engine::builder()
        .base_url(&format!("{}/generic/v1", backend.origin()))
        .unwrap()
        .api_key("sk-mcp-fake-only")
        .unwrap()
        .no_cache()
        .replay(&folder)
        .unwrap()
        .build()
        .unwrap();
    let failure = replay.decide(&question, "x").unwrap_err();
    assert_eq!(failure.kind(), ErrorKind::Local);
    assert_eq!(
        serde_json::to_value(native_error(&failure)).unwrap()["facts"]["requests_sent"],
        0
    );
    assert_eq!(backend.count(), 2, "strict replay miss sends no request");
    drop(engine);
    drop(replay);
    std::fs::remove_dir_all(folder).unwrap();
}

#[cfg(unix)]
#[test]
fn cancelling_a_partial_stdio_frame_retires_the_polling_input_reader() {
    use std::io::{BufReader, Write};
    let (read, mut write) = std::os::unix::net::UnixStream::pair().unwrap();
    let stop = CancelToken::new();
    write.write_all(b"{\"unfinished\":").unwrap();
    let (sent, done) = std::sync::mpsc::channel();
    let token = stop.clone();
    let reader = std::thread::spawn(move || {
        let mut input = BufReader::new(super::super::input::PollInput::new(read, token));
        let _frame = super::super::protocol::read_line(&mut input);
        sent.send(()).unwrap();
    });
    stop.cancel();
    done.recv_timeout(std::time::Duration::from_secs(3))
        .expect("polling read retired while peer remains open");
    reader.join().unwrap();
}

#[cfg(unix)]
#[test]
fn full_stdio_output_pipe_retires_on_stop_without_a_reading_peer() {
    use nix::poll::{PollFd, PollFlags, poll};
    use std::io::Write;
    use std::os::fd::AsFd;
    let (read, write) = nix::unistd::pipe().unwrap();
    let _unread = std::fs::File::from(read);
    let mut output = std::fs::File::from(write);
    let mut full = false;
    for _ in 0..256 {
        if poll(&mut [PollFd::new(output.as_fd(), PollFlags::POLLOUT)], 0u16).unwrap() == 0 {
            full = true;
            break;
        }
        output.write_all(&[b'x'; 512]).unwrap();
    }
    assert!(full, "pipe is under backpressure before the write begins");
    let stop = CancelToken::new();
    let token = stop.clone();
    let (started, ready) = std::sync::mpsc::channel();
    let (sent, done) = std::sync::mpsc::channel();
    let writer = std::thread::spawn(move || {
        let mut output = super::super::input::PollOutput::new(output, token);
        started.send(()).unwrap();
        let result = output.write_all(&[b'y'; 512]);
        sent.send(result).unwrap();
    });
    ready
        .recv_timeout(std::time::Duration::from_secs(3))
        .unwrap();
    stop.cancel();
    assert_eq!(
        done.recv_timeout(std::time::Duration::from_secs(3))
            .expect("output joined without draining peer")
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::BrokenPipe
    );
    writer.join().unwrap();
}
