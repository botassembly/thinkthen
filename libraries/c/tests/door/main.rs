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

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
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
        let built = Command::new(env!("CARGO"))
            .args(["build", "--locked", "--offline", "--lib"])
            .current_dir(crate_dir())
            .output()
            .expect("cargo ran");
        let said = String::from_utf8_lossy(&built.stderr);
        assert!(built.status.success(), "{said}");
        assert!(!said.contains("collision"), "the build warned: {said}");
        let profile = std::env::current_exe().expect("this test");
        let profile = profile
            .parent()
            .and_then(Path::parent)
            .expect("the profile folder");
        let folder = scratch("archive");
        for (from, to) in [
            ("libthinkthen_c.so", "libthinkthen.so"),
            ("libthinkthen_c.a", "libthinkthen.a"),
        ] {
            std::fs::copy(profile.join(from), folder.join(to)).expect("a built library");
        }
        std::os::unix::fs::symlink("libthinkthen.so", folder.join("libthinkthen.so.0"))
            .expect("the soname link");
        folder
    })
}

/// Compile one C program against the archive, under AddressSanitizer.
fn compile(source: &Path) -> PathBuf {
    let name = source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("a name");
    let binary = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("c-{name}"));
    let folder = archive();
    let built = Command::new("cc")
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
        built.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    binary
}

/// Run one program with only the door's settings in its environment, and
/// kill it after a minute.
fn run(binary: &Path, base: &str, input: &[u8]) -> Output {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let cache = scratch(&format!("cache-{}", RUNS.fetch_add(1, Ordering::Relaxed)));
    let mut child = Command::new(binary)
        .env_clear()
        .env("THINKTHEN_BASE_URL", base)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_CACHE", &cache)
        .env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the program started");
    let mut stdin = child.stdin.take().expect("its input");
    std::io::Write::write_all(&mut stdin, input).expect("its input was written");
    drop(stdin);
    let started = Instant::now();
    while child.try_wait().expect("a status").is_none() {
        if started.elapsed() > Duration::from_secs(60) {
            let _ = child.kill();
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    child.wait_with_output().expect("its output")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// R2-26 and ADR 0047 item 6: the soname, the exported symbols, and the
/// version macros match the header.
#[test]
fn the_library_carries_its_soname_and_exactly_the_header_symbols() {
    let library = archive().join("libthinkthen.so");
    let dynamic = Command::new("readelf")
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
    let exported = Command::new("nm")
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
    assert_eq!(symbols.len(), 19);
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
        if name == "nulls" {
            assert_eq!(backend.count(), 0, "a refusal sent a request");
        }
    }
}
