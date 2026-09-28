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

mod bytes;
mod cases;
#[path = "../../../../crates/thinkthen/src/test_deadline/child.rs"]
mod child;
mod settings;

use std::collections::BTreeMap;
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

fn scratch(name: &str) -> PathBuf {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("a scratch folder");
    folder
}

/// The built door as a release archive lays it out: `libthinkthen.so`, its
/// soname link `libthinkthen.so.0`, and `libthinkthen.a`, in one folder.
fn archive() -> &'static Path {
    static FOLDER: OnceLock<PathBuf> = OnceLock::new();
    FOLDER.get_or_init(|| {
        let built = child::command(env!("CARGO"), child::CARGO)
            .args(["build", "--locked", "--offline", "--lib"])
            .arg("--message-format=json-render-diagnostics")
            .current_dir(crate_dir())
            .output()
            .expect("cargo ran");
        let said = String::from_utf8_lossy(&built.stderr);
        assert!(built.status.success(), "{said}");
        assert!(!said.contains("collision"), "the build warned: {said}");
        // Only `cargo doc` warns when the door's library shares the engine's
        // crate name, so the build's own report of its files is checked.
        let files: Vec<PathBuf> = text(&built.stdout)
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter(|message| message["target"]["name"] == "thinkthen_c")
            .flat_map(|message| message["filenames"].as_array().cloned().unwrap_or_default())
            .filter_map(|file| file.as_str().map(PathBuf::from))
            .collect();
        let folder = scratch("archive");
        for (from, to) in [
            ("libthinkthen_c.so", "libthinkthen.so"),
            ("libthinkthen_c.a", "libthinkthen.a"),
        ] {
            let file = files
                .iter()
                .find(|file| file.file_name().is_some_and(|name| name == from))
                .expect("the build reported the door's library under its own name");
            std::fs::copy(file, folder.join(to)).expect("a built library");
        }
        std::os::unix::fs::symlink("libthinkthen.so", folder.join("libthinkthen.so.0"))
            .expect("the soname link");
        folder
    })
}

/// Compile one C program against the archive, under AddressSanitizer, once.
/// Three tests share `driver.c`, and a relink fails another test's launch.
fn compile(source: &Path) -> PathBuf {
    static BUILT: Mutex<BTreeMap<PathBuf, PathBuf>> = Mutex::new(BTreeMap::new());
    let mut built = BUILT.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(binary) = built.get(source) {
        return binary.clone();
    }
    let name = source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("a name");
    let binary = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("c-{name}"));
    let folder = archive();
    let linked = child::command("cc", &[])
        .args([
            "-std=c11",
            "-D_GNU_SOURCE",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pthread",
            "-g",
        ])
        .args(["-fsanitize=address", "-fno-omit-frame-pointer", "-I"])
        .arg(crate_dir().join("include"))
        .arg(source)
        .arg("-o")
        .arg(&binary)
        .arg("-L")
        .arg(folder)
        .arg("-lthinkthen")
        .arg(format!("-Wl,-rpath,{}", folder.display()))
        .output()
        .expect("cc ran");
    assert!(
        linked.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&linked.stderr)
    );
    built.insert(source.to_owned(), binary.clone());
    binary
}

/// Run one program with only the door's settings in its environment, and
/// kill it after a minute.
fn run(binary: &Path, base: &str, input: &[u8]) -> Output {
    let mut child = start(binary, base);
    let mut stdin = child.stdin.take().expect("its input");
    stdin.write_all(input).expect("its input was written");
    drop(stdin);
    finished(child)
}

/// Start one program with only the door's settings and a fresh cache.
fn start(binary: &Path, base: &str) -> Child {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let cache = scratch(&format!("cache-{}", RUNS.fetch_add(1, Ordering::Relaxed)));
    Command::new(binary)
        .env_clear()
        .env("THINKTHEN_BASE_URL", base)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_CACHE", &cache)
        .env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the program started")
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

/// R2-26 and ADR 0047 item 6: the soname, the exported symbols, and the
/// version macros match the header.
#[test]
fn the_library_carries_its_soname_and_exactly_the_header_symbols() {
    let library = archive().join("libthinkthen.so");
    let dynamic = child::command("readelf", &[])
        .arg("-d")
        .arg(&library)
        .output()
        .expect("readelf");
    assert!(
        text(&dynamic.stdout).contains("Library soname: [libthinkthen.so.0]"),
        "{}",
        text(&dynamic.stdout)
    );
    let header =
        std::fs::read_to_string(crate_dir().join("include/thinkthen.h")).expect("the header");
    let exported = child::command("nm", &[])
        .args(["-D", "--defined-only"])
        .arg(&library)
        .output()
        .expect("nm");
    let mut symbols: Vec<String> = text(&exported.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().nth(2))
        .filter(|name| name.starts_with("thinkthen_"))
        .map(str::to_owned)
        .collect();
    symbols.sort();
    assert_eq!(symbols, declared(&header));
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

/// Every function the header declares: a `thinkthen_` name followed by `(`
/// outside a comment.
fn declared(header: &str) -> Vec<String> {
    let mut code = String::new();
    let mut rest = header;
    while let Some(open) = rest.find("/*") {
        code.push_str(&rest[..open]);
        rest = rest[open..].split_once("*/").map_or("", |(_, after)| after);
    }
    code.push_str(rest);
    let mut names: Vec<String> = code
        .match_indices('(')
        .filter_map(|(at, _)| {
            let name = code.get(..at)?.trim_end();
            let start = name
                .rfind(|letter: char| !(letter.is_alphanumeric() || letter == '_'))
                .map_or(0, |at| at + 1);
            name.get(start..)
        })
        .filter(|name| name.starts_with("thinkthen_"))
        .map(str::to_owned)
        .collect();
    names.sort();
    names.dedup();
    names
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
/// nothing when it passes; ASan and LSan report on standard error.
#[test]
fn every_c_row_holds_under_the_sanitizer() {
    for name in ["nulls", "opts", "threads", "engines", "atexit", "fork"] {
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
    for arrived in [1, 2, 6] {
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
    assert_eq!(backend.count(), 6, "nothing was sent after a fire");
}
