//! Native MSVC source consumers, using the same public import-library producer as packing.
use super::*;

const MSVC: &[&str] = &["INCLUDE", "LIB", "LIBPATH"];

fn archive() -> &'static Path {
    static FOLDER: OnceLock<PathBuf> = OnceLock::new();
    FOLDER.get_or_init(|| {
        let keep: Vec<&str> = child::CARGO.iter().chain(MSVC).copied().collect();
        let built = child::command(env!("CARGO"), &keep)
            .args([
                "build",
                "--locked",
                "--offline",
                "--lib",
                "--message-format=json-render-diagnostics",
            ])
            .current_dir(crate_dir())
            .output()
            .expect("Cargo ran");
        assert!(built.status.success(), "{}", text(&built.stderr));
        let binary = text(&built.stdout)
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter(|message| message["target"]["name"] == "thinkthen_c")
            .flat_map(|message| message["filenames"].as_array().cloned().unwrap_or_default())
            .filter_map(|file| file.as_str().map(PathBuf::from))
            .find(|path| {
                path.file_name()
                    .is_some_and(|name| name == "thinkthen_c.dll")
            })
            .expect("the produced DLL");
        let folder = scratch("archive");
        let staged = child::command("python", MSVC)
            .arg(crate_dir().join("../../sdlc/scripts/release-windows-c.py"))
            .arg("stage")
            .arg(binary)
            .arg(crate_dir().join("include/thinkthen.h"))
            .arg(&folder)
            .output()
            .expect("public import library producer");
        assert!(staged.status.success(), "{}", text(&staged.stderr));
        std::fs::copy(
            folder.join("bin/thinkthen.dll"),
            root().join("thinkthen.dll"),
        )
        .expect("the public DLL beside its consumers");
        folder
    })
}

pub(super) fn compile(source: &Path) -> PathBuf {
    compile_mode(source, false)
}

fn compile_mode(source: &Path, sanitizer: bool) -> PathBuf {
    static BUILT: Mutex<BTreeMap<(PathBuf, bool), PathBuf>> = Mutex::new(BTreeMap::new());
    let mut built = BUILT.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(binary) = built.get(&(source.to_owned(), sanitizer)) {
        return binary.clone();
    }
    let name = source
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("a name");
    let binary = root().join(format!(
        "c-{name}{}.exe",
        if sanitizer { "-asan" } else { "" }
    ));
    let folder = archive();
    let mut compiler = child::command("cl.exe", MSVC);
    if sanitizer {
        compiler.arg("/fsanitize=address");
    }
    let linked = compiler
        .args([
            "/nologo",
            "/TC",
            "/std:c11",
            "/W4",
            "/WX",
            "/MD",
            "/D_CRT_SECURE_NO_WARNINGS",
        ])
        .arg(format!("/I{}", folder.join("include").display()))
        .arg(source)
        .arg(format!("/Fo{}", binary.with_extension("obj").display()))
        .arg(format!("/Fe{}", binary.display()))
        .args(["/link"])
        .arg(folder.join("lib/thinkthen.dll.lib"))
        .current_dir(root())
        .output()
        .expect("MSVC ran");
    assert!(
        linked.status.success(),
        "{}{}",
        text(&linked.stdout),
        text(&linked.stderr)
    );
    let inspected = child::command("python", MSVC)
        .arg(crate_dir().join("../../sdlc/scripts/release-windows-c.py"))
        .arg("inspect")
        .arg(folder)
        .arg(&binary)
        .output()
        .expect("native inspection");
    assert!(inspected.status.success(), "{}", text(&inspected.stderr));
    built.insert((source.to_owned(), sanitizer), binary.clone());
    binary
}

#[test]
fn complete_dll_exports_and_imports_match_the_header() {
    let inspected = child::command("python", MSVC)
        .arg(crate_dir().join("../../sdlc/scripts/release-windows-c.py"))
        .arg("inspect")
        .arg(archive())
        .output()
        .expect("dumpbin inspection");
    assert!(inspected.status.success(), "{}", text(&inspected.stderr));
    let input = std::fs::read_to_string(crate_dir().join("include/thinkthen.h")).expect("header");
    assert!(!declared(&input).is_empty());
}

/// MSVC's runtime availability is proved by an actual owned negative first.
/// This instruments C consumers only; the Rust DLL and Unix LSan are outside coverage.
#[test]
fn msvc_asan_proves_instrumentation_before_ownership_checks() {
    let probe = compile_mode(&crate_dir().join("tests/c/asan_probe.c"), true);
    let output = run(&probe, "", b"");
    assert!(!output.status.success());
    assert!(
        text(&output.stderr).contains("AddressSanitizer: heap-use-after-free"),
        "MSVC ASan instrumentation/runtime not established: {}",
        text(&output.stderr)
    );
    std::fs::remove_file(probe.with_extension("obj")).expect("remove negative object");
    std::fs::remove_file(probe).expect("remove negative executable");
    for name in [
        "nulls",
        "opts",
        "threads",
        "engines",
        "typed_facts",
        "atexit",
    ] {
        let backend = Backend::start().expect("backend");
        let source = crate_dir().join("tests/c").join(format!("{name}.c"));
        let binary = compile_mode(&source, true);
        let output = run(&binary, &format!("{}/generic/v1", backend.origin()), b"");
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new()),
            "{name}"
        );
    }
}
