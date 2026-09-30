//! Private diagnostic boundary regressions.

use std::panic::{AssertUnwindSafe, catch_unwind, panic_any};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

use super::{CallOptions, Stop, guarded};
use crate::engine::{Cancel, workers};
use crate::public::error::{Error, ErrorKind};

/// A payload whose disposal panics again with its own marker.
struct Exploding;

impl Drop for Exploding {
    fn drop(&mut self) {
        panic_any("drop key evidence secret");
    }
}

fn wait_for_host_check(held: mpsc::Receiver<()>, joined: Arc<AtomicBool>) {
    held.recv().expect("host check releases worker");
    joined.store(true, Ordering::Release);
}

#[test]
fn diagnostic_boundary_child() {
    if std::env::var_os("THINKTHEN_TEST_DIAGNOSTIC_CHILD").is_none() {
        return;
    }
    std::panic::set_hook(Box::new(|info| {
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or("unknown panic");
        let _written = std::io::Write::write_all(
            &mut std::io::stderr(),
            format!("prior hook: {message}\n").as_bytes(),
        );
    }));

    let worker: Result<(), Error> = guarded(|| {
        workers::on_worker(&Cancel::default(), || {
            panic_any("worker key evidence secret")
        });
        Ok(())
    });
    let error = worker.expect_err("worker panic becomes a defect");
    assert_eq!(error.kind(), ErrorKind::Defect);
    assert!(!error.retryable());
    assert_eq!(
        error.to_string(),
        "defect: the engine panicked below the public door"
    );

    let scoped: Result<(), Error> = guarded(|| {
        let (results, _received) = mpsc::channel::<()>();
        workers::scoped_observed(
            1,
            results,
            &|_: ()| (),
            &|| panic_any("scoped key evidence secret"),
            |queue| {
                let _sent = queue.send(());
            },
        );
        Ok(())
    });
    let error = scoped.expect_err("scoped worker panic becomes a defect");
    assert_eq!(error.kind(), ErrorKind::Defect);
    assert!(!error.retryable());

    let dropped: Result<(), Error> = guarded(|| panic_any(Exploding));
    let error = dropped.expect_err("a payload that panics on drop is a defect");
    assert_eq!(error.kind(), ErrorKind::Defect);

    let (release, held) = mpsc::channel();
    let joined = Arc::new(AtomicBool::new(false));
    let check = || -> bool {
        let _sent = release.send(());
        panic_any("host stop marker");
    };
    let stop = Stop::begin(CallOptions::new().interrupt(&check)).expect("valid stop");
    let joined_worker = Arc::clone(&joined);
    let resumed = catch_unwind(AssertUnwindSafe(|| {
        let _: Result<(), Error> = stop.run(|cancel| {
            workers::on_worker(cancel, move || wait_for_host_check(held, joined_worker));
            Ok(())
        });
    }))
    .expect_err("host check panic resumes");
    assert_eq!(resumed.downcast_ref::<&str>(), Some(&"host stop marker"));
    assert!(joined.load(Ordering::Acquire));

    let (release, held) = mpsc::channel();
    let joined = Arc::new(AtomicBool::new(false));
    let check = || -> bool {
        let _sent = release.send(());
        panic_any("host cancel marker");
    };
    let cancel = Cancel::default().with_check(&check);
    let joined_worker = Arc::clone(&joined);
    let resumed = catch_unwind(AssertUnwindSafe(|| {
        workers::with_engine_diagnostics(|| {
            workers::on_worker(&cancel, move || wait_for_host_check(held, joined_worker));
        });
    }))
    .expect_err("direct host check panic resumes");
    assert_eq!(resumed.downcast_ref::<&str>(), Some(&"host cancel marker"));
    assert!(joined.load(Ordering::Acquire));

    let _unrelated = std::thread::spawn(|| panic_any("unrelated host marker")).join();
    assert_eq!(guarded(|| Ok(7)).ok(), Some(7));
}

#[test]
fn engine_diagnostics_hide_worker_payloads_and_preserve_host_hook() {
    let output = Command::new(std::env::current_exe().expect("test binary"))
        .env_clear()
        .args([
            "--exact",
            "public::options::tests::diagnostic_boundary_child",
            "--nocapture",
        ])
        .env("THINKTHEN_TEST_DIAGNOSTIC_CHILD", "1")
        .output()
        .expect("run diagnostic child");
    assert!(
        output.status.success(),
        "diagnostic child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    for stream in [&*stdout, &*stderr] {
        assert!(!stream.contains("worker key evidence secret"));
        assert!(!stream.contains("scoped key evidence secret"));
        assert!(!stream.contains("drop key evidence secret"));
    }
    assert!(stderr.contains("prior hook: host stop marker"));
    assert!(stderr.contains("prior hook: host cancel marker"));
    assert!(stderr.contains("prior hook: unrelated host marker"));
}
