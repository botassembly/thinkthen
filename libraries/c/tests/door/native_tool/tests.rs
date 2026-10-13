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
