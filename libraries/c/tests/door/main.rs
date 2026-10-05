#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a fixture that fails should stop this proof"
)]
//! The C door, driven from C.
//!
//! Every program here links the built door the way a host does: through
//! `include/thinkthen.h`, `-lthinkthen`, and the soname, under
//! AddressSanitizer with leak checking. Each runs in its own process with a
//! cleared environment, a loopback address, a fake key, and a fresh cache.
//!
//! Unix consumers run under ASan; Windows consumers use MSVC and the public DLL.

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
#[cfg(unix)]
use unix::compile;
#[cfg(windows)]
use windows::compile;
mod header;
mod native_tool;
fn declared(input: &str) -> Vec<String> {
    header::declarations(input).expect("header declarations")
}

mod backends;
mod bytes;
mod cases;
#[path = "../../../../crates/thinkthen/src/test_deadline/child.rs"]
mod child;
mod golden;
mod plan;
mod question_file;
mod request_width;
mod settings;
mod source_controls;
mod sources;
mod usage;

use crate::child::ChildEnvironment as _;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use conformance_backend::Backend;

const KEY: &str = "sk-c-door-loopback";

fn crate_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// This process's own scratch folder. Nextest runs each test in its own
/// process, so no process deletes or overwrites another's archive, drivers or
/// caches (Debt 025). Each process holds a lock on `door/<pid>.lock` for its
/// whole life. The first call deletes each other folder whose lock it can
/// take, since that process has exited, so a run's copies of the archive, about
/// 90 MB a process, last only until the next run. The empty lock files stay,
/// at most one for each process ID the system hands out.
fn root() -> &'static Path {
    static ROOT: OnceLock<(PathBuf, File)> = OnceLock::new();
    let (folder, _held) = ROOT.get_or_init(|| {
        let parent = Path::new(env!("CARGO_TARGET_TMPDIR")).join("door");
        std::fs::create_dir_all(&parent).expect("the door's scratch folder");
        let own = std::process::id().to_string();
        let held = lock_file(&parent, &own);
        held.lock().expect("this process's lock");
        sweep(&parent, &own);
        let folder = parent.join(&own);
        let _stale = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("this process's scratch folder");
        (folder, held)
    });
    folder
}

/// Delete each other process's folder whose lock this process can take.
fn sweep(parent: &Path, own: &str) {
    let names: Vec<String> = std::fs::read_dir(parent)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter_map(|name| name.strip_suffix(".lock").map(str::to_owned))
        .filter(|name| name != own && parent.join(name).exists())
        .collect();
    for name in names {
        // The lock stays held while the folder goes, so a new process with
        // the same ID waits for it.
        let other = lock_file(parent, &name);
        if other.try_lock().is_ok() {
            let _gone = std::fs::remove_dir_all(parent.join(&name));
        }
    }
}

fn lock_file(parent: &Path, name: &str) -> File {
    File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(parent.join(format!("{name}.lock")))
        .expect("a lock file")
}

fn scratch(name: &str) -> PathBuf {
    let folder = root().join(name);
    let _absent = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("a scratch folder");
    folder
}

/// Run one program with only the door's settings in its environment, and
/// kill it after a minute.
fn run(binary: &Path, base: &str, input: &[u8]) -> Output {
    run_with(binary, base, input, &[])
}

/// [`run`] with these variables added to the environment.
fn run_with(binary: &Path, base: &str, input: &[u8], extra: &[(&str, &Path)]) -> Output {
    let mut child = start_with(binary, base, extra);
    let mut stdin = child.stdin.take().expect("its input");
    stdin.write_all(input).expect("its input was written");
    drop(stdin);
    finished(child)
}

/// Start one program with only the door's settings and a fresh cache.
fn start(binary: &Path, base: &str) -> Child {
    start_with(binary, base, &[])
}

fn start_with(binary: &Path, base: &str, extra: &[(&str, &Path)]) -> Child {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let cache = scratch(&format!("cache-{}", RUNS.fetch_add(1, Ordering::Relaxed)));
    let mut command = Command::new(binary);
    command
        .clear_environment()
        .home(cache.join("home"))
        .env("THINKTHEN_BASE_URL", base)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_CACHE", &cache)
        .envs(extra.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    command.env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=0");
    #[cfg(windows)]
    if let Some(path) = std::env::var_os("PATH") {
        command.env("PATH", path);
    }
    command.spawn().expect("the program started")
}

/// Wait for a started program, killing it after a minute, and check that
/// the key never reached its output.
fn finished(mut child: Child) -> Output {
    let started = Instant::now();
    while child.try_wait().expect("a status").is_none() {
        if started.elapsed() > Duration::from_secs(60) {
            let _ = child.kill();
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let output = child.wait_with_output().expect("its output");
    for (stream, bytes) in [("output", &output.stdout), ("error", &output.stderr)] {
        assert!(
            !text(bytes).contains(KEY),
            "the key reached standard {stream}"
        );
    }
    output
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// The slide as drawn and one call of each function print their pinned text
/// on the generic arm.
#[test]
fn the_examples_print_their_pinned_answers() {
    let backend = Backend::start().expect("a loopback backend");
    let base = format!("{}/generic/v1", backend.origin());
    for name in ["slide", "functions"] {
        let source = crate_dir().join("examples").join(format!("{name}.c"));
        let output = run(&compile(&source), &base, b"");
        assert!(output.status.success(), "{name}: {}", text(&output.stderr));
        assert_eq!(text(&output.stderr), "", "{name}");
        let pinned = std::fs::read_to_string(source.with_extension("txt")).expect("a pinned text");
        assert_eq!(text(&output.stdout), pinned, "{name}");
    }
    assert!(
        backend.count() > 0,
        "the examples answered without the backend"
    );
}

/// The C rows: the argument rules, the `_opts` twins, two threads' messages
/// (R1-9), engine-owned failures (R2-16), a call after thread-local
/// teardown (R2-7), and a forked child. Each program checks itself and says
/// nothing when it passes; Unix ASan and LSan report on standard error.
#[test]
fn every_c_row_holds_under_the_sanitizer() {
    for name in ["nulls", "opts", "threads", "engines", "atexit", "fork"] {
        if cfg!(windows) && name == "fork" {
            continue;
        }
        let backend = Backend::start().expect("a loopback backend");
        let base = format!("{}/generic/v1", backend.origin());
        let source = crate_dir().join("tests/c").join(format!("{name}.c"));
        let output = run(&compile(&source), &base, b"");
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new()),
            "{name}"
        );
        match name {
            "nulls" => assert_eq!(backend.count(), 0, "a refusal sent a request"),
            // The parent's first call and the child's; the parent's last is cached.
            "fork" => assert_eq!(backend.count(), 2, "the child's call crossed the wire"),
            _ => {}
        }
    }
}

/// Ticket 0255: one installed C driver exercises all owned-facts typed exports.
#[test]
fn typed_facts_are_owned_and_refusals_leave_outputs_untouched() {
    let backend = Backend::start().expect("a loopback backend");
    let base = format!("{}/generic/v1", backend.origin());
    let source = crate_dir().join("tests/c/typed_facts.c");
    let output = run(&compile(&source), &base, b"");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(
        backend.count(),
        9,
        "the driver counts actual scalar, bulk, recognition and relation sends"
    );
    let refusal = Backend::start().expect("a loopback backend");
    let status_base = format!("{}/arm/status/401/v1", refusal.origin());
    let output = run(&compile(&source), &status_base, b"E");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(refusal.count(), 1, "only the started failure sent");
}

/// Ticket 0166: a token fired while the reply is held cancels the typed
/// scalar, the JSON scalar, and the bulk door, and nothing new is sent. The
/// program says when each token has fired, so each release follows its fire.
#[test]
fn a_token_fired_during_a_held_reply_cancels_the_call() {
    let backend = Backend::start().expect("a loopback backend");
    let base = format!("{}/arm/held/v1", backend.origin());
    let mut child = start(&compile(&crate_dir().join("tests/c/cancel.c")), &base);
    let mut stdin = child.stdin.take().expect("its input");
    let mut said = BufReader::new(child.stdout.take().expect("its output"));
    let mut fired = Vec::new();
    // The five bulk texts share one default packed request. Hold that third
    // arrival before firing token C, as with the two scalar arrivals.
    for arrived in [1, 2, 3] {
        assert_eq!(backend.wait(arrived), arrived, "the requests are held");
        stdin.write_all(b"fire\n").expect("the fire line");
        let mut line = String::new();
        said.read_line(&mut line).expect("the fired line");
        fired.push(line);
        std::thread::sleep(Duration::from_millis(250));
        backend.round();
    }
    backend.release();
    drop(stdin);
    let output = finished(child);
    let mut rest = String::new();
    std::io::Read::read_to_string(&mut said, &mut rest).expect("the rest of its output");
    assert_eq!(
        (
            output.status.code(),
            fired.concat() + &rest,
            text(&output.stderr)
        ),
        (
            Some(0),
            "fired A\nfired B\nfired C\n".to_owned(),
            String::new()
        )
    );
    assert_eq!(backend.count(), 3, "nothing was sent after a fire");
}

/// Version macros are checked on every host alongside the unchanged C ABI inventory.
#[test]
fn version_macros_match_the_package() {
    let header = std::fs::read_to_string(crate_dir().join("include/thinkthen.h")).expect("header");
    let version: Vec<String> = ["MAJOR", "MINOR", "PATCH"]
        .iter()
        .map(|part| {
            let line = format!("#define THINKTHEN_VERSION_{part} ");
            let at = header.find(&line).expect("a version macro") + line.len();
            header[at..]
                .lines()
                .next()
                .unwrap_or_default()
                .trim()
                .to_owned()
        })
        .collect();
    assert_eq!(version.join("."), env!("CARGO_PKG_VERSION"));
}
