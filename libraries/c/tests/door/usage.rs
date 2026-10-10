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

#[test]
fn status_utilities_reject_invalid_flags_without_sending() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let mut script = Script::default();
    for flag in ["usage_status", "finish_usage_status"] {
        for value in ["false", "null", "1", "\"private-evidence\""] {
            script.ask("call", &[&base, &format!(r#"{{"{flag}":{value}}}"#)]);
        }
        for extra in [
            r#""usage":true"#,
            r#""decide":"private-evidence","evidence":"private-evidence""#,
            r#""usage_status":true,"finish_usage_status":true"#,
        ] {
            script.ask("call", &[&base, &format!(r#"{{"{flag}":true,{extra}}}"#)]);
        }
    }
    script.ask(
        "call",
        &[
            &base,
            r#"{"schema":"thinkthen.request/1","usage_status":true}"#,
        ],
    );
    let output = run(&driver, &base, &script.0);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = replies(&output.stdout).expect("replies");
    assert_eq!(said.len(), 15);
    for (code, message) in said {
        assert_eq!(code, 1, "{message}");
        assert!(!message.contains("private-evidence"), "{message}");
        assert!(!message.contains(KEY), "{message}");
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn explicit_status_finalization_preserves_answers_and_direct_totals() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-status-home");
    let mut script = Script::default();
    script.ask("settings", &[&base, r#"{"cache":false}"#]);
    script.ask("call", &[&base, r#"{"usage_status":true}"#]);
    script.ask(
        "call",
        &[
            &base,
            r#"{"decide":"asks for a refund","evidence":"Refund me."}"#,
        ],
    );
    script.ask("finish_native", &[&base, ""]);
    script.ask("call", &[&base, r#"{"finish_usage_status":true}"#]);
    script.ask("call", &[&base, r#"{"usage_status":true}"#]);
    script.ask("call", &[&base, r#"{"usage":true}"#]);
    script.ask("usage_native", &[&base, ""]);
    let variable = child::Folder::Usage.variable(&home);
    let output = run_with(
        &driver,
        &base,
        &script.0,
        &[(variable.0, Path::new(&variable.1))],
    );
    let said = replies(&output.stdout).expect("replies");
    assert!(said.iter().all(|reply| reply.0 == 0), "{said:?}");
    for index in [1, 4, 5] {
        assert_eq!(said[index].1, r#"{"state":"written"}"#);
    }
    let answer: serde_json::Value = serde_json::from_str(&said[2].1).expect("answer");
    assert_eq!(answer["value"], true);
    assert!(
        answer["facts"].get("usage_persistence").is_none(),
        "legacy facts"
    );
    let totals: serde_json::Value = serde_json::from_str(&said[6].1).expect("totals");
    assert_eq!(totals["requests_sent"], 1);
    assert!(totals.get("state").is_none());
    assert_eq!(said[3].1, "3 ");
    assert_eq!(said[7].1, "3 ");
    assert_eq!(month_counts(&home)["requests_sent"], 1);
    assert_eq!(backend.count(), 1);
}

#[cfg(unix)]
#[test]
fn held_writer_reports_pending_then_latched_safe_failure_without_losing_answer() {
    use std::os::unix::fs::PermissionsExt as _;
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-held-private-home");
    let usage = usage_folder(&home);
    std::fs::create_dir_all(&usage).expect("usage folder");
    std::fs::set_permissions(&usage, std::fs::Permissions::from_mode(0o700))
        .expect("private folder");
    let lock = File::options()
        .write(true)
        .create_new(true)
        .open(usage.join(".lock"))
        .expect("lock");
    lock.set_permissions(std::fs::Permissions::from_mode(0o600))
        .expect("private lock");
    lock.lock().expect("hold writer");
    let mut script = Script::default();
    script.ask("settings", &[&base, r#"{"cache":false}"#]);
    script.ask("call", &[&base, r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"asks for a refund"},"input":{"kind":"text","text":"Refund me."}}}"#]);
    script.ask("usage_native", &[&base, ""]);
    script.ask("call", &[&base, r#"{"usage_status":true}"#]);
    script.ask("finish_native", &[&base, ""]);
    script.ask("call", &[&base, r#"{"finish_usage_status":true}"#]);
    script.ask("usage_native", &[&base, ""]);
    script.ask("call", &[&base, r#"{"usage_status":true}"#]);
    script.ask("call", &[&base, r#"{"usage":true}"#]);
    script.ask("free_native", &[&base, ""]);
    let variable = child::Folder::Usage.variable(&home);
    let output = run_with(
        &driver,
        &base,
        &script.0,
        &[(variable.0, Path::new(&variable.1))],
    );
    let said = replies(&output.stdout).expect("replies");
    assert!(said.iter().all(|reply| reply.0 == 0), "{said:?}");
    assert_eq!(said[2].1, "2 ");
    let failure = "4 check the usage folder permissions and free space";
    assert_eq!(said[3].1, r#"{"state":"pending"}"#);
    assert_eq!(said[4].1, failure);
    assert_eq!(said[6].1, failure);
    assert_eq!(said[9].1, failure);
    for index in [5, 7] {
        assert_eq!(
            said[index].1,
            r#"{"state":"failed","advice":"check the usage folder permissions and free space"}"#
        );
    }
    let answer: serde_json::Value = serde_json::from_str(&said[1].1).expect("answer");
    assert_eq!(answer["facts"]["usage_persistence"]["state"], "pending");
    assert_eq!(
        answer["facts"]["usage_persistence"]["observed_at"],
        "facts_snapshot"
    );
    let totals: serde_json::Value = serde_json::from_str(&said[8].1).expect("totals");
    assert_eq!(totals["requests_sent"], 1);
    for index in [2, 3, 4, 5, 6, 7, 9] {
        assert!(!said[index].1.contains(&home.display().to_string()));
        assert!(!said[index].1.contains("Refund me."));
        assert!(!said[index].1.contains(KEY));
    }
    assert_eq!(backend.count(), 1);
}

#[test]
fn replay_status_utilities_add_no_model_send() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-status-replay-home");
    let record = scratch("usage-status-record");
    for mode in ["record", "replay"] {
        let settings = serde_json::json!({"cache":false, mode:record}).to_string();
        let mut script = Script::default();
        script.ask("settings", &[&base, &settings]);
        for request in [
            r#"{"decide":"asks for a refund","evidence":"Refund me."}"#,
            r#"{"usage_status":true}"#,
            r#"{"finish_usage_status":true}"#,
            r#"{"usage_status":true}"#,
            r#"{"usage":true}"#,
        ] {
            script.ask("call", &[&base, request]);
        }
        let variable = child::Folder::Usage.variable(&home);
        let output = run_with(
            &driver,
            &base,
            &script.0,
            &[(variable.0, Path::new(&variable.1))],
        );
        let said = replies(&output.stdout).expect("replies");
        assert!(said.iter().all(|reply| reply.0 == 0), "{said:?}");
        assert_eq!(said[3].1, r#"{"state":"written"}"#);
        assert_eq!(said[4].1, said[3].1);
        let totals: serde_json::Value = serde_json::from_str(&said[5].1).expect("totals");
        assert_eq!(totals["requests_sent"], u64::from(mode == "record"));
    }
    assert_eq!(backend.count(), 1);
    assert_eq!(month_counts(&home)["requests_sent"], 1);
}
