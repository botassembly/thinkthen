//! The decision band's missing value asks a person to read the text.

use std::path::Path;
use std::process::Command;

use conformance_backend::{Canned, Listener};

#[test]
fn slide_handles_yes_no_and_no_value() {
    for (probability, expected) in [(0.9, "Yes\n"), (0.1, "No\n"), (0.5, "a person reads it\n")] {
        let backend = Listener::answering(move |_| {
            Canned::ok(&format!(
                r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{probability}}}}}}}"#
            ))
        })
        .expect("a decision-band loopback backend");
        let binary = std::env::current_exe().expect("the test binary");
        let built = binary
            .parent()
            .and_then(Path::parent)
            .expect("the profile folder");
        let cache =
            Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("cache-slide-band-{probability}"));
        let _absent = std::fs::remove_dir_all(&cache);
        let output = Command::new(
            built
                .join("examples")
                .join(format!("slide{}", std::env::consts::EXE_SUFFIX)),
        )
        .env_clear()
        .env("THINKTHEN_BASE_URL", backend.base())
        .env("THINKTHEN_API_KEY", "sk-examples-loopback")
        .env("THINKTHEN_CACHE", &cache)
        .output()
        .expect("the slide example ran");
        assert!(
            output.status.success() && output.stderr.is_empty(),
            "slide: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        assert_eq!(backend.count(), 1, "slide sends one decision");
    }
}
