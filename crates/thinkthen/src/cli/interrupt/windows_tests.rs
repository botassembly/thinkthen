//! Native console proof; subprocess helpers are excluded from routine selection.
use super::{Environment, Routing, State};
use crate::test_deadline::child;
use crate::windows::test_support::{ffi, process};
use child::ChildEnvironment as _;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "thinkthen-console-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).expect("scratch");
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.0);
    }
}
fn child(scratch: &Scratch, mode: &str) -> process::Owned {
    let mut command = Command::new(std::env::current_exe().expect("test executable"));
    command
        .clear_environment()
        .args([
            "--exact",
            "cli::interrupt::windows_tests::signal_child",
            "--ignored",
            "--nocapture",
        ])
        .env("THINKTHEN_WINDOWS_SIGNAL_MODE", mode)
        .env("THINKTHEN_WINDOWS_SIGNAL_MARKERS", &scratch.0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let child = process::Owned::console(&mut command).expect("isolated child");
    process::wait(&scratch.0.join("ready"));
    child
}
#[test]
fn native_ctrl_c_exits_130_at_rest_and_after_finish() {
    for mode in ["idle", "finished"] {
        let scratch = Scratch::new();
        let child = child(&scratch, mode);
        process::inject(
            child.id(),
            "cli::interrupt::windows_tests::console_injector",
            None,
        );
        let output = child.finish().expect("Ctrl-C exit");
        assert_eq!(output.status.code(), Some(130), "{mode}: {output:?}");
    }
}
#[test]
fn native_ctrl_c_cancels_then_finishes_or_exits_on_an_acknowledged_second_event() {
    for mode in ["active", "second", "second-term"] {
        let scratch = Scratch::new();
        let mut child = child(&scratch, mode);
        process::inject(
            child.id(),
            "cli::interrupt::windows_tests::console_injector",
            Some(&scratch.0.join("ack")),
        );
        if !child.alive() {
            let output = child.finish().expect("early signal exit");
            panic!("first active signal must remain cooperative: {mode}: {output:?}");
        }
        process::wait(&scratch.0.join("ack"));
        if mode == "second" {
            process::inject(
                child.id(),
                "cli::interrupt::windows_tests::console_injector",
                None,
            );
        } else {
            fs::write(scratch.0.join("release"), b"1").expect("release");
        }
        let expected = if mode == "second-term" { 143 } else { 130 };
        let output = child.finish().expect("signal exit");
        assert_eq!(output.status.code(), Some(expected), "{mode}: {output:?}");
    }
}
#[test]
fn sigterm_exits_143_at_rest_and_after_cooperative_finish() {
    for mode in ["term-idle", "term-active"] {
        let scratch = Scratch::new();
        let child = child(&scratch, mode);
        fs::write(scratch.0.join("release"), b"1").expect("release");
        assert_eq!(
            child.finish().expect("SIGTERM exit").status.code(),
            Some(143)
        );
    }
}
#[test]
#[ignore = "subprocess-only native console injector"]
fn console_injector() {
    let process = std::env::var("THINKTHEN_CONSOLE_INJECT_PID")
        .expect("owned PID")
        .parse()
        .expect("PID");
    let acknowledgment = std::env::var_os("THINKTHEN_CONSOLE_INJECT_ACK").map(PathBuf::from);
    ffi::inject(process, acknowledgment.as_deref()).expect("native console injection");
}
#[test]
#[ignore = "subprocess-only synchronized signal state"]
fn signal_child() {
    let mode = std::env::var("THINKTHEN_WINDOWS_SIGNAL_MODE").expect("mode");
    let root =
        PathBuf::from(std::env::var_os("THINKTHEN_WINDOWS_SIGNAL_MARKERS").expect("markers"));
    let state = State::install(super::register, super::emulate);
    let mut environment = Environment::default();
    let guard = if mode == "idle" || mode == "term-idle" {
        None
    } else {
        Some(
            state
                .activate_with(&mut environment, || {
                    Routing::start(state.cancel.clone(), None)
                })
                .expect("active"),
        )
    };
    let guard = if mode == "finished" {
        assert_eq!(
            guard
                .expect("guard")
                .finish(ExitCode::SUCCESS)
                .expect("finish"),
            ExitCode::SUCCESS
        );
        None
    } else {
        guard
    };
    fs::write(root.join("ready"), b"1").expect("ready");
    if mode.starts_with("term-") {
        process::wait(&root.join("release"));
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTERM).expect("SIGTERM");
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !state.cancel.fired() {
        assert!(std::time::Instant::now() < deadline, "signal not delivered");
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    fs::write(root.join("ack"), b"1").expect("ack");
    if mode != "term-active" {
        process::wait(&root.join("release"));
    }
    if mode == "second-term" {
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTERM)
            .expect("second SIGTERM");
        panic!("second SIGTERM returned");
    }
    let code = guard
        .expect("active guard")
        .finish(ExitCode::SUCCESS)
        .expect("finish");
    std::process::exit(if code == ExitCode::from(143) {
        143
    } else {
        130
    });
}
