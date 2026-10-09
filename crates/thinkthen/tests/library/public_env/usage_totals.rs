//! ADR 0113: an engine built from the environment adds to the command's
//! usage totals, and one built by hand writes none.

use super::*;
#[cfg(unix)]
use crate::child::ChildEnvironment as _;
use crate::child::Folder;

fn hold_usage_writer(case: &str, argument: &str) -> Option<fs::File> {
    (case == "usage-held").then(|| {
        let usage = Path::new(argument);
        fs::create_dir_all(usage).expect("usage folder");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(usage, fs::Permissions::from_mode(0o700)).expect("private folder");
        }
        let lock = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(usage.join(".lock"))
            .expect("usage lock");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            lock.set_permissions(fs::Permissions::from_mode(0o600))
                .expect("private lock");
        }
        lock.lock().expect("hold writer");
        lock
    })
}

pub(super) fn run_usage(case: &str, argument: &str) -> Vec<String> {
    let engine = match case {
        "usage-seeded" | "usage-held" => {
            EngineBuilder::from_env().and_then(|seed| seed.no_cache().build())
        }
        "usage-cached" => EngineBuilder::from_env().and_then(EngineBuilder::build),
        "usage-refused" => {
            let engine = EngineBuilder::from_env()
                .and_then(|seed| seed.no_cache().build())
                .expect("the engine builds; the check waits for a send");
            let question = Question::decide("asks for a refund")
                .expect("a question")
                .cut();
            return (0..2)
                .map(|_| match engine.decide(&question, EVIDENCE) {
                    Err(error) => format!("{:?}: {error}", error.kind()),
                    Ok(call) => format!("answered {:?}", call.value()),
                })
                .collect();
        }
        "usage-builder" => Engine::builder()
            .base_url(argument)
            .and_then(|builder| builder.no_cache().build()),
        _ => panic!("no child case {case}"),
    }
    .expect("the engine");
    let held = hold_usage_writer(case, argument);
    let question = Question::decide("asks for a refund")
        .expect("a question")
        .cut();
    let call = engine.decide(&question, EVIDENCE).expect("an answer");
    let observed = call
        .facts()
        .usage_persistence()
        .expect("engine facts observation");
    let complete: serde_json::Value =
        serde_json::to_value(call.facts().complete().expect("complete facts")).expect("facts JSON");
    assert_eq!(
        complete["usage_persistence"]["observed_at"],
        "facts_snapshot"
    );
    let legacy = serde_json::to_value(call.facts()).expect("legacy facts");
    assert!(legacy.get("usage_persistence").is_none());
    let status = engine.finish_usage_status();
    if case == "usage-held" {
        assert_eq!(observed, thinkthen::UsagePersistence::Pending);
        assert_eq!(complete["usage_persistence"]["state"], "pending");
        assert_eq!(status, thinkthen::UsagePersistence::Failed);
        assert_eq!(engine.usage_persistence(), status);
        assert_eq!(
            status.advice(),
            Some("check the usage folder permissions and free space")
        );
        assert_eq!(call.facts().usage_persistence(), Some(observed));
        assert_eq!(engine.usage().requests_sent(), 1);
        drop(held);
        let next = engine
            .decide(&question, EVIDENCE)
            .expect("answer after writer failure");
        assert_eq!(next.facts().requests_sent(), 1);
        assert_eq!(next.facts().input_tokens(), call.facts().input_tokens());
        let failed =
            serde_json::to_value(next.facts().complete().expect("failed persistence facts"))
                .expect("JSON");
        assert_eq!(
            failed["usage_persistence"],
            serde_json::json!({"state":"failed", "observed_at":"facts_snapshot", "advice":"check the usage folder permissions and free space"})
        );
        assert!(!failed.to_string().contains(argument));
        assert_eq!(engine.usage().requests_sent(), 2);
        assert_eq!(engine.finish_usage_status(), status);
    } else {
        assert_eq!(
            status,
            if case == "usage-builder" {
                thinkthen::UsagePersistence::Disabled
            } else {
                thinkthen::UsagePersistence::Written
            }
        );
        assert!(status.advice().is_none());
    }
    engine.finish_usage();
    vec![format!(
        "{:?} sent {} cached {}",
        call.value(),
        call.facts().requests_sent(),
        call.facts().cache_answers()
    )]
}

#[cfg(unix)]
#[cfg(feature = "cli")]
/// The command's own `status --json` under this root's usage folder.
fn status(home: &Path) -> serde_json::Value {
    let state = Folder::Usage.variable(home);
    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(["status", "--json"])
            .clear_environment()
            .home(home)
            .env(state.0, state.1),
    )
    .expect("status runs");
    assert!(output.status.success());
    serde_json::from_slice::<serde_json::Value>(&output.stdout).expect("status JSON")["usage"]
        ["total"]
        .clone()
}

#[cfg(unix)]
#[cfg(feature = "cli")]
/// A listener that answers `503` to the arrivals named, and `ANSWERED` otherwise.
fn failing_at(busy: &'static [usize]) -> Listener {
    let arrivals = AtomicUsize::new(0);
    Listener::answering(move |_| {
        if busy.contains(&arrivals.fetch_add(1, Ordering::SeqCst)) {
            Canned::status(503, "busy").asking("retry-after-ms", "0")
        } else {
            Canned::ok(ANSWERED)
        }
    })
    .expect("a loopback listener")
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[cfg(feature = "cli")]
#[test]
fn the_command_and_a_seeded_engine_add_to_one_total() {
    let listener = failing_at(&[1]);
    let home = folder("usage-one-total");
    let (cache, state) = (Folder::Cache.variable(&home), Folder::Usage.variable(&home));
    let evidence = folder("usage-one-total-evidence");
    fs::write(&evidence, format!("{EVIDENCE}\n")).expect("evidence");
    let child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["decide", "asks for a refund"])
        .clear_environment()
        .env(cache.0, &cache.1)
        .env(state.0, &state.1)
        .env("THINKTHEN_BASE_URL", listener.base())
        .stdin(fs::File::open(&evidence).expect("evidence"))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("the command starts");
    let command = crate::wait::finish(child, "thinkthen decide").expect("the command runs");
    assert!(
        command.status.success(),
        "{}",
        String::from_utf8_lossy(&command.stderr)
    );
    let seeded = in_child(
        "usage-seeded",
        &[
            (cache.0, cache.1.as_str()),
            (state.0, state.1.as_str()),
            ("THINKTHEN_BASE_URL", listener.base()),
        ],
    );
    assert_eq!(seeded, "Yes sent 2 cached 0");
    assert_eq!(listener.count(), 3);
    assert_eq!(
        status(&home),
        serde_json::json!({"requests_sent": 3, "retries": 1, "input_tokens": 624, "output_tokens": 96, "cache_answers": 0})
    );
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[cfg(feature = "cli")]
#[test]
fn a_cached_rerun_sends_nothing_and_adds_a_cache_answer() {
    let listener = listener();
    let home = folder("usage-cached-rerun");
    let (cache, state) = (Folder::Cache.variable(&home), Folder::Usage.variable(&home));
    let environment = [
        (cache.0, cache.1.as_str()),
        (state.0, state.1.as_str()),
        ("THINKTHEN_BASE_URL", listener.base()),
    ];
    assert_eq!(
        in_child("usage-cached", &environment),
        "Yes sent 1 cached 0"
    );
    assert_eq!(
        in_child("usage-cached", &environment),
        "Yes sent 0 cached 1"
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(
        status(&home),
        serde_json::json!({"requests_sent": 1, "retries": 0, "input_tokens": 312, "output_tokens": 48, "cache_answers": 1})
    );
}

#[test]
fn an_engine_built_by_hand_writes_no_usage() {
    let listener = listener();
    let home = folder("usage-by-hand");
    let (cache, state) = (Folder::Cache.variable(&home), Folder::Usage.variable(&home));
    let built = in_child(
        "usage-builder",
        &[
            (cache.0, cache.1.as_str()),
            (state.0, state.1.as_str()),
            (ARGUMENT, listener.base()),
        ],
    );
    assert_eq!(built, "Yes sent 1 cached 0");
    assert_eq!(listener.count(), 1);
    assert!(!Folder::Usage.under(&home).exists());
}

/// Main answered over a shared usage folder or a malformed month and lost
/// the count without a word. The engine now refuses its first send, and again
/// on a second call, with a sentence that names no path (ticket 0360).
#[cfg(unix)]
#[test]
fn an_unreadable_usage_folder_refuses_every_send_and_names_no_path() {
    use std::os::unix::fs::PermissionsExt as _;

    let shared = "the usage folder that thinkthen status names has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside.";
    let malformed = "2026-08.json has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again.";
    for (label, mode, month, problem) in [
        ("usage-shared-folder", 0o755, None, shared),
        ("usage-malformed-month", 0o700, Some(&b"{"[..]), malformed),
        (
            "usage-malformed-month-no-lock",
            0o700,
            Some(&b"garbage"[..]),
            malformed,
        ),
    ] {
        let listener = listener();
        let home = folder(label);
        let usage = Folder::Usage.under(&home);
        fs::create_dir_all(&usage).expect("usage folder");
        fs::set_permissions(&usage, fs::Permissions::from_mode(mode)).expect("folder mode");
        if let Some(bytes) = month {
            let lock = !label.ends_with("no-lock");
            let files = [(".lock", &b""[..]), ("2026-08.json", bytes)];
            for (name, contents) in files.into_iter().skip(usize::from(!lock)) {
                fs::write(usage.join(name), contents).expect("usage file");
                fs::set_permissions(usage.join(name), fs::Permissions::from_mode(0o600))
                    .expect("private file");
            }
        }
        let before = entries(&usage);
        let state = Folder::Usage.variable(&home);
        let refused = in_child(
            "usage-refused",
            &[
                (state.0, state.1.as_str()),
                ("THINKTHEN_BASE_URL", listener.base()),
            ],
        );
        let sentence = format!("Local: cannot read the usage totals: {problem}");
        assert_eq!(refused, format!("{sentence}\n{sentence}"), "{label}");
        assert_eq!(listener.count(), 0, "{label}");
        assert_eq!(entries(&usage), before, "{label}");
        if let Some(bytes) = month {
            assert_eq!(fs::read(usage.join("2026-08.json")).expect("month"), bytes);
        }
    }
}

#[cfg(unix)]
#[test]
fn failed_finalization_preserves_a_good_answer_and_its_pending_facts() {
    let listener = listener();
    let home = folder("usage-held-sdk");
    let state = Folder::Usage.variable(&home);
    let usage = Folder::Usage.under(&home);
    let result = in_child(
        "usage-held",
        &[
            (state.0, state.1.as_str()),
            ("THINKTHEN_BASE_URL", listener.base()),
            (ARGUMENT, usage.to_str().expect("usage path")),
        ],
    );
    assert_eq!(result, "Yes sent 1 cached 0");
    assert_eq!(listener.count(), 2);
}
