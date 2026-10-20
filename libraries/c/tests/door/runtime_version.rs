//! An installed C consumer refuses an incompatible header before creating an engine.
use super::*;

#[test]
fn loaded_interface_must_match_before_any_other_operation() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let source = crate_dir().join("tests/c/runtime_version.c");
    let output = run(&compile(&source), &base, b"");
    successful(&output);
    assert_eq!(
        text(&output.stdout),
        format!("{}\n", env!("CARGO_PKG_VERSION"))
    );

    let header =
        std::fs::read_to_string(crate_dir().join("include/thinkthen.h")).expect("installed header");
    let prefix = "#define THINKTHEN_INTERFACE_NUMBER ";
    let number = header
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .expect("interface number");
    let loaded: u32 = number
        .trim_end_matches('u')
        .parse()
        .expect("uint32 interface");
    let required = loaded + 1;
    let incompatible = scratch("incompatible-header");
    std::fs::write(
        incompatible.join("thinkthen.h"),
        header.replace(&format!("{prefix}{number}"), &format!("{prefix}{required}")),
    )
    .expect("incompatible installed header");
    let copied = incompatible.join("runtime_version.c");
    std::fs::copy(source, &copied).expect("installed C consumer");
    let output = run(&compile(&copied), &base, b"");
    assert_eq!(output.status.code(), Some(78));
    assert!(output.stdout.is_empty());
    assert_eq!(
        text(&output.stderr),
        format!(
            "ThinkThen C interface mismatch: header requires {required}, loaded library {} provides {loaded}\n",
            env!("CARGO_PKG_VERSION")
        )
    );
    assert_eq!(backend.count(), 0, "version checks must send nothing");
}
