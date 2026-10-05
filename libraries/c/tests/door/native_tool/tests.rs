//! Ticket 0381 regressions execute the same bounded runner on portable children.
use super::*;

fn python(script: &str) -> Command {
    let mut command = child::command(if cfg!(windows) { "python" } else { "python3" }, &[]);
    command.args(["-c", script]);
    command
}

#[test]
fn filled_streams_finish_and_nonzero_status_is_preserved() {
    for code in [0, 23] {
        let script = format!(
            "import os, sys\nos.write(1, b'o' * 524288)\nos.write(2, b'e' * 524288)\nsys.exit({code})"
        );
        let output = output(
            &mut python(&script),
            "portable compiler",
            Duration::from_secs(10),
        )
        .expect("filled streams finish before the deadline");
        assert_eq!(output.status.code(), Some(code));
        assert_eq!(output.stdout, vec![b'o'; 524288]);
        assert_eq!(output.stderr, vec![b'e'; 524288]);
    }
}

#[test]
fn timeout_reaps_only_the_owned_child_and_names_each_tool() {
    let mut survivor = python("import time; time.sleep(60)")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("independently owned sibling");
    let mut observations = Vec::new();
    for tool in ["Cargo", "MSVC (cl.exe)", "Python stage", "Python inspect"] {
        let mut stalled = python("import time; time.sleep(60)")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("owned stalled tool");
        let failure = wait(&mut stalled, tool, Duration::from_millis(100));
        let stopped = stalled
            .try_wait()
            .expect("independent stopped-child observation");
        let sibling = survivor
            .try_wait()
            .expect("independent sibling observation");
        // Clean our children even when a regression left the stalled one alive.
        let _ = stalled.kill();
        let _ = wait(
            &mut stalled,
            "owned stalled cleanup",
            Duration::from_secs(1),
        );
        observations.push((tool, failure, stopped, sibling));
    }
    survivor.kill().expect("clean independently owned sibling");
    wait(
        &mut survivor,
        "owned sibling cleanup",
        Duration::from_secs(1),
    )
    .expect("sibling stops");
    assert!(survivor.try_wait().expect("sibling reaped").is_some());
    for (tool, failure, stopped, sibling) in observations {
        assert!(stopped.is_some(), "timed-out owned tool is still running");
        assert!(sibling.is_none(), "another owned process was killed");
        let error = failure.expect_err("a deadline must fail, never return killed success");
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        assert_eq!(error.to_string(), format!("{tool} timed out after 100ms"));
    }
}

#[test]
fn timeout_of_the_capture_route_fails_instead_of_returning_a_killed_status() {
    let error = output(
        &mut python("import time; time.sleep(60)"),
        "Python inspect",
        Duration::from_millis(100),
    )
    .expect_err("capture deadline");
    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    assert_eq!(error.to_string(), "Python inspect timed out after 100ms");
}

/// Fault the join at the actual fixture boundary while a real worker stays alive.
/// Teardown bombs distinguish stopping the process from returning through cleanup.
#[cfg(unix)]
#[test]
fn failed_typed_facts_join_stops_before_worker_reads_or_teardown() {
    let folder = scratch("typed-facts-join-refusal");
    let input =
        std::fs::read_to_string(crate_dir().join("tests/c/typed_facts.c")).expect("fixture");
    let injection = r#"
#include <stdatomic.h>
#include <unistd.h>
static atomic_int live;
static void *held_caller(void *opaque) {
    (void)opaque;
    atomic_store(&live, 1);
    for (;;) sleep(1);
    return NULL;
}
static int failed_join(pthread_t thread) {
    (void)thread;
    while (!atomic_load(&live)) usleep(1000);
    return 1;
}
#undef fixture_start
#undef fixture_join
#define fixture_start(thread, entry, argument) ((void)(entry), pthread_create(thread, NULL, held_caller, argument))
#define fixture_join(thread) failed_join(thread)
#define thinkthen_engine_free(engine) ((void)(engine), _Exit(42))
int main(void) {"#;
    let source = folder.join("join_refusal.c");
    let input = input
        .replace(
            "\"platform.h\"",
            &format!("\"{}\"", crate_dir().join("tests/c/platform.h").display()),
        )
        .replace("int main(void) {", injection);
    std::fs::write(&source, input).expect("owned faulted fixture copy");
    let backend = Backend::start().expect("backend");
    let output = run(
        &compile(&source),
        &format!("{}/generic/v1", backend.origin()),
        b"S",
    );
    assert_eq!(
        output.status.code(),
        Some(1),
        "teardown bomb or continued fixture"
    );
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"FAIL second caller joins\n");
    assert_eq!(backend.count(), 1, "the held worker never sends");
}
