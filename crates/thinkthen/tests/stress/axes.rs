#![cfg(test)]
//! The same event-driven axes run with bounded inputs and full-volume inputs.
use super::{records, rust_rows, stress_engine};
use crate::loopback::{HeldReply, StressFixture};
use serde_json::Value;
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::time::{Duration, Instant};
use thinkthen::{Cancel, Decide};

const GUARD: Duration = Duration::from_secs(30);

fn healthy(table: &Value) -> &Value {
    table["scenarios"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == "healthy")
        .unwrap()
}
fn server(fixture: &StressFixture) -> &crate::loopback::Loopback {
    let StressFixture::Server(server) = fixture else {
        panic!("healthy server required")
    };
    server
}
// Each call has a different question. A worker pool within one call cannot satisfy
// this barrier. Neither caller receives its first reply until both have sent.
fn overlap(fixture: &StressFixture) -> Arc<(Mutex<u8>, Condvar)> {
    let state = Arc::new((Mutex::new(0_u8), Condvar::new()));
    let observed = Arc::clone(&state);
    server(fixture).delay(Duration::ZERO, move |request| {
        let bit = if request.body.windows(6).any(|part| part == b"pair-0") {
            1
        } else {
            2
        };
        let (seen, wake) = &*observed;
        let mut seen = seen.lock().unwrap();
        *seen |= bit;
        wake.notify_all();
        let (seen, guard) = wake
            .wait_timeout_while(seen, GUARD, |seen| *seen != 3)
            .unwrap();
        assert!(!guard.timed_out(), "calls never overlapped: {seen}");
        false
    });
    state
}
fn rust_pair(table: &Value, backend: &str, count: usize) {
    let fixture = StressFixture::open("healthy").unwrap();
    let overlap = overlap(&fixture);
    let engine = stress_engine(table, backend, fixture.base());
    std::thread::scope(|scope| {
        let calls: Vec<_> = ["pair-0", "pair-1"]
            .into_iter()
            .map(|question| {
                let engine = engine.clone().cancel(Cancel::default());
                scope.spawn(move || rust_rows(&engine, healthy(table), count, question))
            })
            .collect();
        for call in calls {
            call.join().unwrap();
        }
    });
    assert_eq!(*overlap.0.lock().unwrap(), 3);
}
fn assert_closed(ending: std::io::Result<usize>) {
    assert!(
        matches!(ending, Ok(0))
            || matches!(ending, Err(ref error)
        if error.kind() == std::io::ErrorKind::ConnectionReset),
        "connection stayed open: {ending:?}"
    );
}
fn rust_cancel(table: &Value, backend: &str, count: usize, stage: HeldReply) {
    let fixture = StressFixture::open("healthy").unwrap();
    let server = server(&fixture);
    let engine = stress_engine(table, backend, fixture.base());
    let (arrived, closed) = server.hold_matching(stage, |request| {
        request.body.windows(9).any(|part| part == b"cancelled")
    });
    let cancel = Cancel::default();
    let stopped = engine.clone().cancel(cancel.clone());
    let worker = std::thread::spawn(move || {
        let question = Decide::new("cancelled").unwrap();
        let mut rows = stopped.rows(&question, records(count)).unwrap();
        assert_eq!(
            rows.next().unwrap().unwrap_err().code().as_str(),
            "local.cancelled"
        );
        assert!(rows.next().is_none());
    });
    let batch = table["client"]["batch"].as_u64().unwrap() as usize;
    let workers = crate::loopback::limits()["workers"].as_u64().unwrap() as usize;
    let frontier = count.div_ceil(batch).min(workers);
    for _ in 0..frontier {
        arrived.recv_timeout(GUARD).unwrap();
    }
    // Keep the other call inside an actual backend exchange while cancelling.
    let (seen, received) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let released = Mutex::new(released);
    let first = std::sync::atomic::AtomicBool::new(true);
    server.delay(Duration::ZERO, move |_| {
        if first.swap(false, std::sync::atomic::Ordering::Relaxed) {
            seen.send(()).unwrap();
            released.lock().unwrap().recv_timeout(GUARD).unwrap();
        }
        false
    });
    std::thread::scope(|scope| {
        let survivor = engine.clone().cancel(Cancel::default());
        // One held survivor batch cannot add unrelated requests after the snapshot.
        let survivor_count = count.min(batch);
        let survivor =
            scope.spawn(move || rust_rows(&survivor, healthy(table), survivor_count, "survivor"));
        received.recv_timeout(GUARD).unwrap();
        let paid = server.count();
        let deadline = Instant::now() + Duration::from_secs(2);
        cancel.cancel();
        for _ in 0..frontier {
            assert_closed(
                closed
                    .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                    .unwrap(),
            );
        }
        worker.join().unwrap();
        assert_eq!(
            server.count(),
            paid,
            "cancelled call sent another paid request"
        );
        release.send(()).unwrap();
        survivor.join().unwrap();
    });
    rust_rows(
        &engine.cancel(Cancel::default()),
        healthy(table),
        count,
        "reuse",
    );
}

#[cfg(feature = "command")]
fn input(count: usize) -> tempfile::NamedTempFile {
    use std::io::Write;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    for i in 0..count {
        writeln!(
            file,
            "{}",
            serde_json::json!({"id":super::identifier(i),"body":format!("record {}", super::record_number(i))})
        )
        .unwrap();
    }
    file.flush().unwrap();
    file
}
#[cfg(feature = "command")]
struct CommandCall {
    child: std::process::Child,
    diagnostics: Option<std::thread::JoinHandle<Vec<u8>>>,
}
#[cfg(feature = "command")]
impl CommandCall {
    fn new(call: (std::process::Child, std::thread::JoinHandle<Vec<u8>>)) -> Self {
        Self {
            child: call.0,
            diagnostics: Some(call.1),
        }
    }
    fn diagnostics(&mut self) -> Vec<u8> {
        self.diagnostics.take().unwrap().join().unwrap()
    }
}
#[cfg(feature = "command")]
impl Drop for CommandCall {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(diagnostics) = self.diagnostics.take() {
            let _ = diagnostics.join();
        }
    }
}
#[cfg(feature = "command")]
fn command_result(mut call: CommandCall, scenario: &Value, count: usize) {
    use std::io::{BufRead, BufReader};
    let mut received = 0;
    for line in BufReader::new(call.child.stdout.take().unwrap()).lines() {
        received += 1;
        super::assert_row(
            &serde_json::from_str::<Value>(&line.unwrap()).unwrap(),
            received,
            received - 1,
            scenario,
        );
    }
    let status = call.child.wait().unwrap();
    let diagnostic = call.diagnostics();
    assert!(status.success(), "{}", String::from_utf8_lossy(&diagnostic));
    assert_eq!(received, count);
}
#[cfg(feature = "command")]
fn command_pair(table: &Value, backend: &str, count: usize) {
    let fixture = StressFixture::open("healthy").unwrap();
    let overlap = overlap(&fixture);
    let input = input(count);
    let calls: Vec<_> = ["pair-0", "pair-1"]
        .into_iter()
        .map(|question| {
            CommandCall::new(super::spawn_command(
                table,
                backend,
                fixture.base(),
                question,
                input.reopen().unwrap().into(),
            ))
        })
        .collect();
    // Observe both requests before reading output. A failed barrier must reap the
    // commands immediately rather than wait for the command's network timeout.
    let seen = overlap.0.lock().unwrap();
    let (seen, guard) = overlap
        .1
        .wait_timeout_while(seen, GUARD, |seen| *seen != 3)
        .unwrap();
    assert!(!guard.timed_out(), "commands never overlapped: {seen}");
    drop(seen);
    for call in calls {
        command_result(call, healthy(table), count);
    }
}
#[cfg(all(feature = "command", unix))]
fn command_interrupt(table: &Value, backend: &str, count: usize, stage: HeldReply) {
    use std::io::Read;
    let fixture = StressFixture::open("healthy").unwrap();
    let (arrived, closed) = server(&fixture).hold(stage);
    let input = input(count);
    let mut call = CommandCall::new(super::spawn_command(
        table,
        backend,
        fixture.base(),
        "Ready?",
        input.reopen().unwrap().into(),
    ));
    arrived.recv_timeout(GUARD).unwrap();
    assert!(
        std::process::Command::new("kill")
            .args(["-INT", &call.child.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    assert_closed(closed.recv_timeout(Duration::from_secs(2)).unwrap());
    let mut output = Vec::new();
    call.child
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut output)
        .unwrap();
    assert_eq!(call.child.wait().unwrap().code(), Some(130));
    let diagnostic = call.diagnostics();
    assert!(output.is_empty(), "interrupted held call emitted rows");
    assert!(!diagnostic.is_empty(), "interrupt was not observable");
}

#[derive(Clone, Copy)]
enum Axis {
    RustPair,
    RustCancel,
    #[cfg(feature = "command")]
    CommandPair,
    #[cfg(all(feature = "command", unix))]
    CommandInterrupt,
}
fn run(table: &Value, count: usize, axis: Axis) {
    for backend in super::backends() {
        match axis {
            Axis::RustPair => rust_pair(table, backend, count),
            #[cfg(feature = "command")]
            Axis::CommandPair => command_pair(table, backend, count),
            Axis::RustCancel => {
                for stage in [HeldReply::Headers, HeldReply::Body, HeldReply::RefusalBody] {
                    rust_cancel(table, backend, count, stage);
                }
            }
            #[cfg(all(feature = "command", unix))]
            Axis::CommandInterrupt => {
                for stage in [HeldReply::Headers, HeldReply::Body, HeldReply::RefusalBody] {
                    command_interrupt(table, backend, count, stage);
                }
            }
        }
    }
}
pub(super) fn cases(table: &Value, count: usize) {
    for axis in [
        Axis::RustPair,
        Axis::RustCancel,
        #[cfg(feature = "command")]
        Axis::CommandPair,
        #[cfg(all(feature = "command", unix))]
        Axis::CommandInterrupt,
    ] {
        run(table, count, axis);
    }
}
fn bounded(axis: Axis) {
    let table = crate::loopback::stress_table().unwrap();
    let count = table["records"]["smoke"].as_u64().unwrap() as usize;
    run(&table, count, axis);
}
#[test]
fn rust_pair_overlaps_and_preserves_each_call() {
    bounded(Axis::RustPair);
}
#[test]
fn rust_cancel_closes_only_its_call_and_client_remains_usable() {
    bounded(Axis::RustCancel);
}
#[cfg(feature = "command")]
#[test]
fn command_pair_overlaps_as_independent_processes() {
    bounded(Axis::CommandPair);
}
#[cfg(all(feature = "command", unix))]
#[test]
fn command_interrupt_closes_the_exchange_and_exits_130() {
    bounded(Axis::CommandInterrupt);
}
