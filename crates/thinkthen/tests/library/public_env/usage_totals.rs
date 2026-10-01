//! ADR 0113: an engine built from the environment adds to the command's
//! usage totals, and one built by hand writes none.

use super::*;

pub(super) fn run_usage(case: &str, argument: &str) -> Vec<String> {
    let engine = match case {
        "usage-seeded" => EngineBuilder::from_env().and_then(|seed| seed.no_cache().build()),
        "usage-cached" => EngineBuilder::from_env().and_then(EngineBuilder::build),
        "usage-refused" => {
            let engine = EngineBuilder::from_env()
                .and_then(|seed| seed.no_cache().build())
                .expect("the engine builds; the check waits for a send");
            let question = Question::decide("asks for a refund").expect("a question").cut();
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
    let question = Question::decide("asks for a refund")
        .expect("a question")
        .cut();
    let call = engine.decide(&question, EVIDENCE).expect("an answer");
    vec![format!(
        "{:?} sent {} cached {}",
        call.value(),
        call.facts().requests_sent(),
        call.facts().cache_answers()
    )]
}

#[cfg(feature = "cli")]
/// The command's own `status --json` under this home's state folder.
fn status(home: &Path) -> serde_json::Value {
    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(["status", "--json"])
            .env_clear()
            .env("XDG_STATE_HOME", home.join("state"))
            .env("HOME", home),
    )
    .expect("status runs");
    assert!(output.status.success());
    serde_json::from_slice::<serde_json::Value>(&output.stdout).expect("status JSON")["usage"]
        ["total"]
        .clone()
}

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

#[cfg(feature = "cli")]
#[test]
fn the_command_and_a_seeded_engine_add_to_one_total() {
    let listener = failing_at(&[1]);
    let home = folder("usage-one-total");
    let state = home.join("state");
    let evidence = folder("usage-one-total-evidence");
    fs::write(&evidence, format!("{EVIDENCE}\n")).expect("evidence");
    let child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["decide", "asks for a refund"])
        .env_clear()
        .env("XDG_CACHE_HOME", &home)
        .env("XDG_STATE_HOME", home.join("state"))
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
            ("XDG_CACHE_HOME", home.to_str().expect("home")),
            ("XDG_STATE_HOME", state.to_str().expect("state")),
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

#[cfg(feature = "cli")]
#[test]
fn a_cached_rerun_sends_nothing_and_adds_a_cache_answer() {
    let listener = listener();
    let home = folder("usage-cached-rerun");
    let state = home.join("state");
    let environment = [
        ("XDG_CACHE_HOME", home.to_str().expect("home")),
        ("XDG_STATE_HOME", state.to_str().expect("state")),
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
    let state = home.join("state");
    let built = in_child(
        "usage-builder",
        &[
            ("XDG_CACHE_HOME", home.to_str().expect("home")),
            ("XDG_STATE_HOME", state.to_str().expect("state")),
            (ARGUMENT, listener.base()),
        ],
    );
    assert_eq!(built, "Yes sent 1 cached 0");
    assert_eq!(listener.count(), 1);
    assert!(!state.join("thinkthen").exists());
}

/// Main answered over a shared usage folder and lost the count without a
/// word. The engine now refuses its first send, and again on a second call,
/// with the sentence that names no path (ticket 0360).
#[cfg(unix)]
#[test]
fn a_shared_usage_folder_refuses_every_send_and_names_no_path() {
    use std::os::unix::fs::PermissionsExt as _;

    let listener = listener();
    let home = folder("usage-shared-folder");
    let usage = home.join("state/thinkthen");
    fs::create_dir_all(&usage).expect("usage folder");
    fs::set_permissions(&usage, fs::Permissions::from_mode(0o755)).expect("shared mode");
    let refused = in_child(
        "usage-refused",
        &[
            ("XDG_STATE_HOME", home.join("state").to_str().expect("state")),
            ("THINKTHEN_BASE_URL", listener.base()),
        ],
    );
    let sentence = "Local: cannot read the usage totals: the usage folder that thinkthen status names has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside.";
    assert_eq!(refused, format!("{sentence}\n{sentence}"));
    assert_eq!(listener.count(), 0);
    assert_eq!(entries(&usage), 0);
}
