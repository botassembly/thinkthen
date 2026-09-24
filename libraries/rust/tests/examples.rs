//! Every program under `examples/` prints what the `.txt` beside it says.
//!
//! Each program runs as a user runs it: in its own process, through
//! `Engine::from_env`, with nothing in its environment but a loopback
//! address, a fake key, and a fresh cache folder. The loopback backend's
//! generic arm answers every question, so each pinned text follows that
//! arm's rule. A model would answer differently.

use std::path::Path;
use std::process::Command;

use conformance_backend::Backend;

#[test]
fn every_example_prints_its_pinned_answer() {
    let sources = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    // Cargo builds each example beside the test binary's own `deps` folder.
    let deps = std::env::current_exe().expect("the test binary");
    let built = deps
        .parent()
        .and_then(Path::parent)
        .expect("the profile folder");
    let mut ran = 0;
    for entry in std::fs::read_dir(&sources).expect("the examples folder") {
        let source = entry.expect("an examples entry").path();
        if source.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let name = source
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("a name");
        let expected = std::fs::read_to_string(source.with_extension("txt"))
            .unwrap_or_else(|_| panic!("{name}.txt pins what {name} prints"));
        let cache = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("cache-{name}"));
        let _absent = std::fs::remove_dir_all(&cache);
        let backend = Backend::start().expect("a loopback backend");
        let program = built
            .join("examples")
            .join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
        let output = Command::new(&program)
            .env_clear()
            .env(
                "THINKTHEN_BASE_URL",
                format!("{}/generic/v1", backend.origin()),
            )
            .env("THINKTHEN_API_KEY", "sk-examples-loopback")
            .env("THINKTHEN_CACHE", &cache)
            .output()
            .unwrap_or_else(|error| panic!("{name} ran from {}: {error}", program.display()));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success() && stderr.is_empty(),
            "{name}: {stderr}"
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected, "{name}");
        assert!(backend.count() > 0, "{name} answered without the backend");
        ran += 1;
    }
    assert!(ran > 0, "the examples folder holds no program");
}
