//! The native C door's error and hook boundaries.

use super::{DEFECT, Failure, Held, USAGE, built, guard, lock};
use std::io::Write;

const CHILD: &str = "THINKTHEN_C_PANIC_CHILD";
const STRING_MARKER: &str = "c-owned-string-payload-marker";
const DROP_MARKER: &str = "c-owned-drop-payload-marker";

fn held() -> Held {
    let engine = thinkthen::Engine::builder()
        .base_url("http://127.0.0.1:9/v1")
        .map(thinkthen::EngineBuilder::no_cache)
        .and_then(thinkthen::EngineBuilder::build)
        .expect("an engine that sends nothing");
    Held::new(engine)
}

fn message_of(engine: &Held) -> String {
    engine
        .read(|last| last.message.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// R3-24: each finished thread leaves its per-engine error entry.
#[test]
fn a_threads_failure_leaves_the_table_when_the_thread_exits() {
    let engine = std::sync::Arc::new(held());
    for _ in 0..200 {
        let shared = std::sync::Arc::clone(&engine);
        let _ = std::thread::spawn(move || shared.fail(Failure::usage("a short thread"))).join();
    }
    assert_eq!(lock(&engine.failures).len(), 0);
    engine.fail(Failure::usage("this thread"));
    assert_eq!(lock(&engine.failures).len(), 1);
}

#[test]
fn a_panic_behind_the_door_is_the_fixed_nonretryable_defect() {
    let engine = held();
    let code = guard(Some(&engine), DEFECT, || -> i32 {
        panic!("{STRING_MARKER}")
    });
    assert_eq!(
        (
            code,
            super::code(Some(&engine)),
            super::retryable(Some(&engine))
        ),
        (DEFECT, DEFECT, 0)
    );
    assert_eq!(message_of(&engine), "defect: a panic crossed the C door");
    let result = crate::complete::failure(super::snapshot(Some(&engine)).expect("snapshot"))
        .expect("complete failure");
    assert_eq!(result.summary.state, 2);
    assert_eq!(result.summary.error.value.code, DEFECT);
    assert_eq!(result.summary.facts.present, 0);
    assert_eq!(result.summary.meta.present, 0);
    assert_eq!(guard(Some(&engine), DEFECT, || USAGE), USAGE);
}

/// The hook is process-global within this library, so the secrecy proof runs
/// alone in a child and captures its two output streams.
#[test]
fn a_caught_payload_never_reaches_native_diagnostics() {
    if std::env::var_os(CHILD).is_some() {
        native_panic_child();
    } else {
        let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .env_clear()
            .args([
                "--exact",
                "failures::tests::a_caught_payload_never_reaches_native_diagnostics",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .output()
            .expect("isolated native proof");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        for stream in [&output.stdout, &output.stderr] {
            let text = String::from_utf8_lossy(stream);
            assert!(!text.contains(STRING_MARKER), "{text}");
            assert!(!text.contains(DROP_MARKER), "{text}");
        }
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "host-thread-marker\n"
        );
    }
}

struct Exploding;

impl Drop for Exploding {
    fn drop(&mut self) {
        panic!("{DROP_MARKER}");
    }
}

fn native_panic_child() {
    std::panic::set_hook(Box::new(|info| {
        let text = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or(if info.payload().is::<Exploding>() {
                DROP_MARKER
            } else {
                "other panic"
            });
        let _ = std::io::stderr().write_all(format!("{text}\n").as_bytes());
    }));
    let engine = held();
    assert_eq!(
        guard(Some(&engine), DEFECT, || -> i32 {
            panic!("{STRING_MARKER}")
        }),
        DEFECT
    );
    assert_eq!(message_of(&engine), "defect: a panic crossed the C door");
    assert_eq!(
        guard(Some(&engine), DEFECT, || -> i32 {
            std::panic::panic_any(Exploding)
        }),
        DEFECT
    );
    assert_eq!(message_of(&engine), "defect: a panic crossed the C door");
    let none = built(|| -> Result<thinkthen::Engine, Failure> { std::panic::panic_any(Exploding) });
    assert!(none.is_none());
    assert_eq!(super::code(None), DEFECT);
    assert_eq!(super::retryable(None), 0);
    let null_message = super::unbuilt(|last| last.message.to_string_lossy().into_owned());
    assert_eq!(
        null_message.as_deref(),
        Some("defect: a panic built no engine")
    );
    assert_eq!(guard(Some(&engine), DEFECT, || USAGE), USAGE);
    let _ = std::thread::spawn(|| panic!("host-thread-marker")).join();
}

#[test]
fn owned_failure_snapshots_keep_sticky_prestart_errors_after_replacement_and_engine_drop() {
    let engine = held();
    assert!(super::snapshot(Some(&engine)).is_none());
    engine.fail(Failure::usage("first refusal"));
    let saved = super::snapshot(Some(&engine)).expect("owned snapshot");
    assert_eq!(
        (saved.code, saved.retryable, saved.facts.is_none()),
        (USAGE, false, true)
    );
    assert_eq!(saved.message, "first refusal");
    assert_eq!(engine.settle(Ok::<_, Failure>(())), Ok(()));
    assert_eq!(
        super::snapshot(Some(&engine)).expect("sticky").message,
        "first refusal"
    );
    engine.fail(Failure::local("later refusal"));
    drop(engine);
    assert_eq!(saved.message, "first refusal");
    assert!(saved.facts.is_none());
}
#[test]
fn failed_build_and_engine_snapshots_stay_in_the_calling_threads_own_error_slots() {
    let engine = std::sync::Arc::new(held());
    engine.fail(Failure::usage("parent refusal"));
    let shared = engine.clone();
    let saved = std::thread::spawn(move || {
        assert!(super::snapshot(Some(&shared)).is_none());
        shared.fail(Failure::local("child refusal"));
        super::snapshot(Some(&shared)).expect("child snapshot")
    })
    .join()
    .expect("thread");
    assert_eq!(saved.message, "child refusal");
    assert_eq!(
        super::snapshot(Some(&engine))
            .expect("parent snapshot")
            .message,
        "parent refusal"
    );
    assert!(built(|| Err::<thinkthen::Engine, _>(Failure::usage("failed build"))).is_none());
    let failed_build = super::snapshot(None).expect("failed build snapshot");
    assert_eq!(failed_build.message, "failed build");
    assert!(failed_build.facts.is_none());
    assert!(built(|| Ok::<_, Failure>(held().engine)).is_some());
    assert!(super::snapshot(None).is_none());
    assert_eq!(failed_build.message, "failed build");
}
