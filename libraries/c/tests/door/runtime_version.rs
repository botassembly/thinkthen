//! An installed C consumer refuses an incompatible library before creating an engine.
use super::*;

#[test]
fn loaded_interface_must_match_before_any_other_operation() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let source = crate_dir().join("tests/c/runtime_version.c");
    let binary = compile(&source);
    let output = run(&binary, &base, b"");
    successful(&output);
    assert_eq!(text(&output.stdout), format!("{}\n", env!("CARGO_PKG_VERSION")));

    let (library, required, loaded) = incompatible_library();
    let output = run_with(&binary, &base, b"", &[("THINKTHEN_TEST_LIBRARY", &library)]);
    assert_eq!(output.status.code(), Some(78));
    assert!(output.stdout.is_empty());
    assert_eq!(text(&output.stderr), format!("ThinkThen C interface mismatch: header requires {required}, loaded library different-interface provides {loaded}\n"));
    assert_eq!(backend.count(), 0, "version checks must send nothing");
}

fn incompatible_library() -> (PathBuf, u32, u32) {
    let incompatible = scratch("incompatible-library");
    let header = crate_dir().join("include/thinkthen.h");
    // Generate a second interface from a changed declaration. Its bootstrap is
    // compiled into a real shared library; the caller keeps the installed header.
    let generated = child::command(if cfg!(windows) { "python" } else { "python3" }, &[])
        .args(["-c", "import runpy,sys; from pathlib import Path; g=runpy.run_path(sys.argv[1]); h=Path(sys.argv[2]).read_bytes(); h=b'\\n'.join(l for l in h.split(b'\\n') if not l.startswith(b'#define THINKTHEN_INTERFACE_NUMBER ')); Path(sys.argv[3]).write_bytes(g['interface_header'](h+b'\\nvoid thinkthen_different_interface(void);\\n'))"])
        .arg(crate_dir().join("../../sdlc/scripts/generate-c-header.py"))
        .arg(&header)
        .arg(incompatible.join("thinkthen.h"))
        .output().expect("generate incompatible interface");
    successful(&generated);
    let number = |path| -> u32 {
        std::fs::read_to_string(path).expect("generated header").lines()
            .find_map(|line| line.strip_prefix("#define THINKTHEN_INTERFACE_NUMBER "))
            .expect("digest").trim_end_matches('u').parse().expect("uint32 digest")
    };
    let required = number(header);
    let loaded = number(incompatible.join("thinkthen.h"));
    assert_ne!(loaded, required, "a declaration changes the interface digest");
    let fixture = incompatible.join("library.c");
    std::fs::write(&fixture, "#include \"thinkthen.h\"\n#ifdef _WIN32\n__declspec(dllexport)\n#endif\nconst char *thinkthen_version(uint32_t *number) { *number = THINKTHEN_INTERFACE_NUMBER; return \"different-interface\"; }\n").expect("library source");
    let library = incompatible.join(if cfg!(windows) { "different.dll" } else if cfg!(target_os = "macos") { "different.dylib" } else { "different.so" });
    let mut compiler = child::command(if cfg!(windows) { "cl.exe" } else { "cc" }, &["INCLUDE", "LIB", "LIBPATH"]);
    if cfg!(windows) {
        compiler.args(["/nologo", "/LD"]).arg(&fixture).arg(format!("/Fe{}", library.display()));
    } else {
        compiler.args([if cfg!(target_os = "macos") { "-dynamiclib" } else { "-shared" }, "-fPIC"]).arg(&fixture).arg("-o").arg(&library);
    }
    successful(&compiler.current_dir(&incompatible).output().expect("compile library"));
    (library, required, loaded)
}
