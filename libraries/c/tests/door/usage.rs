//! ADR 0113: a C engine built from the environment adds its sends and its
//! cache answers to the command's usage totals when the host frees it.

use super::*;
use cases::{Script, replies};

/// Run the driver once: build an engine with `settings`, decide one refund
/// request, and free the engine. Return both replies.
fn decide_replies(driver: &Path, base: &str, settings: &str, home: &Path) -> Vec<(i32, String)> {
    let mut script = Script::default();
    script.ask("settings", &[base, settings]);
    script.ask(
        "decide",
        &[base, r#"{"decide":"asks for a refund"}"#, "Refund me."],
    );
    let output = run_with(
        driver,
        base,
        &script.0,
        &[(
            child::Folder::Usage.variable(home).0,
            Path::new(&child::Folder::Usage.variable(home).1),
        )],
    );
    replies(&output.stdout).expect("replies")
}

/// Both reply codes of `decide_replies`.
fn decide_once(driver: &Path, base: &str, settings: &str, home: &Path) -> (i32, i32) {
    let said = decide_replies(driver, base, settings, home);
    (said[0].0, said[1].0)
}

/// This home's usage folder (ticket 0360).
fn usage_folder(home: &Path) -> std::path::PathBuf {
    child::Folder::Usage.under(home)
}

/// The one month file under this home's usage folder.
fn month_counts(home: &Path) -> serde_json::Value {
    let usage = usage_folder(home);
    let month = std::fs::read_dir(&usage)
        .expect("the usage folder")
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_string_lossy().ends_with(".json"))
        .expect("a month file");
    serde_json::from_slice(&std::fs::read(month.path()).expect("the month")).expect("JSON")
}

#[test]
fn a_freed_c_engine_adds_its_send_to_the_usage_totals() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-home");
    let said = decide_once(&driver, &base, r#"{"cache":false}"#, &home);
    assert_eq!(said, (0, 0));
    assert_eq!(backend.count(), 1);
    let counts = month_counts(&home);
    assert_eq!(counts["requests_sent"], 1, "{counts}");
}

#[test]
fn a_cached_rerun_sends_nothing_and_adds_a_cache_answer() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-cached-home");
    let cache = scratch("usage-cached-folder");
    let settings = serde_json::json!({"cache": cache}).to_string();
    for _ in 0..2 {
        assert_eq!(decide_once(&driver, &base, &settings, &home), (0, 0));
    }
    assert_eq!(backend.count(), 1);
    let counts = month_counts(&home);
    assert_eq!(
        (&counts["requests_sent"], &counts["cache_answers"]),
        (&1.into(), &1.into()),
        "{counts}"
    );
}

/// Main answered over a malformed month and lost the count without a word.
/// The C door now returns the local code with the sentence and sends
/// nothing, and every native and SQL host over the door gets the same.
#[cfg(unix)]
#[test]
fn a_malformed_month_refuses_the_c_call_before_any_send() {
    use std::os::unix::fs::PermissionsExt as _;

    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-malformed-home");
    let usage = usage_folder(&home);
    std::fs::create_dir_all(&usage).expect("usage folder");
    std::fs::set_permissions(&usage, std::fs::Permissions::from_mode(0o700)).expect("private");
    for (name, bytes) in [(".lock", &b""[..]), ("2026-08.json", b"not JSON")] {
        std::fs::write(usage.join(name), bytes).expect("usage file");
        std::fs::set_permissions(usage.join(name), std::fs::Permissions::from_mode(0o600))
            .expect("private");
    }
    let said = decide_replies(&driver, &base, r#"{"cache":false}"#, &home);
    assert_eq!(said[0].0, 0, "{said:?}");
    assert_eq!(said[1].0, 4, "{said:?}");
    assert!(
        said[1].1.contains("cannot read the usage totals: 2026-08.json has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again."),
        "{said:?}"
    );
    assert!(!said[1].1.contains(&home.display().to_string()), "{said:?}");
    assert_eq!(backend.count(), 0);
}
