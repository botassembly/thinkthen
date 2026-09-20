//! Shared live-authority acceptance tests use only temporary Git repositories.

#[cfg(test)]
mod live_support;

use live_support::*;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn linked_worktrees_share_one_allowance() {
    let (main, linked) = temporary_repository("live-shared-authority-red");
    let first = live(&main)
        .current_dir(&main)
        .args(["--max-tokens", "7", "hold.sh"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("first live job");
    wait_for(&main.join("ready"));

    let second = live(&linked)
        .current_dir(&linked)
        .args(["--max-tokens", "7", "quick.sh", "from", "linked"])
        .output()
        .expect("second live job");
    assert_eq!(second.status.code(), Some(1), "{second:?}");

    fs::write(main.join("release"), "go\n").expect("release");
    let first = first.wait_with_output().expect("first output");
    assert_eq!(first.status.code(), Some(0), "{first:?}");

    let refused = live(&linked)
        .current_dir(&linked)
        .args(["--max-tokens", "7", "quick.sh"])
        .output()
        .expect("over allowance");
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    let final_run = live(&linked)
        .current_dir(&linked)
        .args(["--max-tokens", "3", "quick.sh", "from", "linked"])
        .output()
        .expect("exact fit");
    assert_eq!(final_run.status.code(), Some(0), "{final_run:?}");
    assert_eq!(
        fs::read_to_string(linked.join("quick")).expect("arguments"),
        "from linked\n"
    );
    for tree in [&main, &linked] {
        let report = status(tree);
        assert!(report.contains("limit_tokens 10\n"), "{report}");
        assert!(report.contains("charged_tokens 10\n"), "{report}");
        assert!(report.contains("pending none\n"), "{report}");
    }
}

#[test]
fn historical_launchers_refuse_the_checkpoint_before_job_or_build() {
    for revision in ["2c32524", "a556b97"] {
        let (main, _linked) = temporary_repository(&format!("live-history-{revision}"));
        let fake_bin = main
            .parent()
            .expect("root")
            .join(format!("fake-bin-{revision}"));
        fs::create_dir(&fake_bin).expect("fake bin");
        executable(
            &fake_bin.join("cargo"),
            &format!(
                "#!/bin/sh\necho invoked >{}/build-invoked\nexit 99\n",
                main.display()
            ),
        );
        let old = Command::new("/usr/bin/git")
            .current_dir(repo())
            .args(["show", &format!("{revision}:sdlc/scripts/live")])
            .output()
            .expect("historical launcher");
        assert!(old.status.success());
        executable(
            &main.join("sdlc/scripts/live"),
            &String::from_utf8(old.stdout).expect("launcher text"),
        );
        let mut command = Command::new(main.join("sdlc/scripts/live"));
        command
            .current_dir(&main)
            .env_clear()
            .env("PATH", format!("{}:/usr/bin:/bin", fake_bin.display()))
            .env("THINKTHEN_API_KEY", "historical-key");
        if revision == "2c32524" {
            command.arg(main.join("quick.sh"));
        } else {
            command
                .args(["--max-tokens", "1"])
                .arg(main.join("quick.sh"));
        }
        let output = command.output().expect("historical refusal");
        assert_ne!(output.status.code(), Some(0), "{revision}: {output:?}");
        assert!(!main.join("quick").exists(), "{revision} ran");
        assert!(!main.join("build-invoked").exists(), "{revision} built");
    }
}

#[test]
fn migration_refuses_an_actually_running_2c32524_wrapper() {
    let (main, linked) = temporary_repository("live-migrate-active-2c32524");
    let old = Command::new("/usr/bin/git")
        .current_dir(repo())
        .args(["show", "2c32524:sdlc/scripts/live"])
        .output()
        .expect("historical launcher");
    assert!(old.status.success());
    executable(
        &linked.join("sdlc/scripts/live"),
        &String::from_utf8(old.stdout).unwrap(),
    );
    fs::write(
        linked.join("sdlc/live-tokens"),
        b"limit_tokens 10\nspent_tokens 0\n",
    )
    .unwrap();
    run(&linked, &["add", "sdlc/scripts/live", "sdlc/live-tokens"]);
    run(&linked, &["commit", "-qm", "actual historical launcher"]);

    let fake_bin = main.parent().unwrap().join("active-2c-bin");
    fs::create_dir(&fake_bin).unwrap();
    executable(
        &fake_bin.join("cargo"),
        &format!(
            "#!/bin/sh\necho ready >{0}/cargo-ready\nwhile [ ! -e {0}/cargo-release ]; do sleep 0.02; done\nexit 99\n",
            linked.display()
        ),
    );
    let mut historical = Command::new(linked.join("sdlc/scripts/live"))
        .current_dir(&linked)
        .env_clear()
        .env("PATH", format!("{}:/usr/bin:/bin", fake_bin.display()))
        .env("THINKTHEN_API_KEY", "historical-key")
        .arg(linked.join("quick.sh"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("historical wrapper");
    let _historical_cleanup = ProcessCleanup::new(historical.id());
    wait_for(&linked.join("cargo-ready"));
    let refused = migrate(&main, &["--verify"]);
    fs::write(linked.join("cargo-release"), b"go\n").unwrap();
    let _status = historical.wait().expect("historical status");
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("active legacy live wrapper"),
        "{refused:?}"
    );
    assert!(!linked.join("quick").exists());
}

#[test]
fn key_refusal_precedes_repository_resolution() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("live-key-first");
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("sdlc/scripts")).expect("scripts");
    fs::copy(
        repo().join("sdlc/scripts/live"),
        root.join("sdlc/scripts/live"),
    )
    .expect("live");
    for (case, key, sentence) in [
        ("blank", " \t", "THINKTHEN_API_KEY holds nothing"),
        (
            "line",
            "first\nsecond",
            "THINKTHEN_API_KEY contains a line break",
        ),
    ] {
        let output = Command::new(root.join("sdlc/scripts/live"))
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("THINKTHEN_API_KEY", key)
            .args(["--max-tokens", "1", "job.sh"])
            .output()
            .expect("key refusal");
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains(sentence));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("Git"));
    }
}

#[test]
fn status_and_recovery_never_create_or_rebind_authority() {
    let (main, _linked) = temporary_repository("live-status-recovery");
    let git_dir = main.join(".git");
    let state = git_dir.join("thinkthen-live/state.json");
    let original = fs::read(&state).expect("state");
    assert!(status(&main).contains("status active\n"));
    let recovered = Command::new(main.join("sdlc/scripts/live"))
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .arg("--recover")
        .output()
        .expect("recover");
    assert!(recovered.status.success(), "{recovered:?}");
    assert_eq!(fs::read(&state).expect("state"), original);

    fs::remove_file(&state).expect("remove state");
    for action in ["--status", "--recover"] {
        let output = Command::new(main.join("sdlc/scripts/live"))
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg(action)
            .output()
            .expect("missing authority refusal");
        assert_eq!(output.status.code(), Some(1), "{action}: {output:?}");
        assert!(!state.exists());
    }
}

#[test]
fn corrupt_copied_and_legacy_authority_grant_nothing() {
    for case in ["corrupt", "binding", "legacy-lock", "checkpoint"] {
        let (main, _linked) = temporary_repository(&format!("live-refusal-{case}"));
        let state = main.join(".git/thinkthen-live/state.json");
        match case {
            "corrupt" => fs::write(&state, b"{\"version\":1,\"version\":1}\n").expect("corrupt"),
            "binding" => {
                let text = fs::read_to_string(&state).expect("state");
                fs::write(
                    &state,
                    text.replace("common_dir_hex\":\"", "common_dir_hex\":\"00"),
                )
                .expect("binding");
            }
            "legacy-lock" => {
                fs::create_dir(main.join("sdlc/live-tokens.lock")).expect("legacy lock")
            }
            "checkpoint" => fs::write(
                main.join("sdlc/live-tokens"),
                b"limit_tokens 10\nspent_tokens 0\n",
            )
            .expect("reverted checkpoint"),
            _ => unreachable!(),
        }
        let output = live(&main)
            .current_dir(&main)
            .args(["--max-tokens", "1", "quick.sh"])
            .output()
            .expect("refusal");
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert!(!main.join("quick").exists(), "{case} ran");
    }
}

#[test]
fn a_separate_clone_has_no_authority() {
    let (main, _linked) = temporary_repository("live-separate-source");
    let clone = main.parent().expect("root").join("clone");
    let output = Command::new("/usr/bin/git")
        .args(["clone", "-q", "--no-local"])
        .arg(&main)
        .arg(&clone)
        .output()
        .expect("clone");
    assert!(output.status.success(), "{output:?}");
    fs::create_dir_all(clone.join("target/debug")).expect("target");
    executable(&clone.join("target/debug/thinkthen"), "#!/bin/sh\nexit 0\n");
    let refused = live(&clone)
        .current_dir(&clone)
        .args(["--max-tokens", "1", "quick.sh"])
        .output()
        .expect("separate clone refusal");
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    assert!(!clone.join("quick").exists());
}

#[test]
fn key_bytes_and_job_path_bytes_reach_only_the_released_job() {
    let (main, _linked) = temporary_repository("live-key-path-bytes");
    let odd = main.join("odd directory\n/job\n.sh");
    fs::create_dir_all(odd.parent().expect("parent")).expect("odd directory");
    executable(
        &odd,
        "#!/bin/sh\nprintf '%s' \"$THINKTHEN_API_KEY\" >received-key\nprintf '%s\\n' \"$0\" \"$@\" >received-arguments\n",
    );
    let key = " leading\\key\twith\rbytes ";
    let output = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_API_KEY", key)
        .args(["--max-tokens", "1"])
        .arg(PathBuf::from("odd directory\n/job\n.sh"))
        .args(["one", "two words"])
        .output()
        .expect("odd bytes");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        fs::read(main.join("received-key")).expect("key"),
        key.as_bytes()
    );
    assert_eq!(
        fs::read_to_string(main.join("received-arguments")).expect("arguments"),
        format!("{}\none\ntwo words\n", odd.display())
    );
    assert!(
        !fs::read(main.join(".git/thinkthen-live/state.json"))
            .expect("state")
            .windows(key.len())
            .any(|part| part == key.as_bytes())
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains(key));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(key));
}

#[test]
fn ready_gate_uses_exact_isolated_interpreter_and_has_no_key() {
    let (main, _linked) = temporary_repository("live-gate-environment");
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).expect("hooks");
    let child = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_API_KEY", "marker-key-for-proc")
        .env("THINKTHEN_LIVE_TEST_GIT_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "after-ready")
        .args(["--max-tokens", "1", "quick.sh"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("wrapper");
    inspect_and_release_git_helper(&hooks, b"marker-key-for-proc");
    wait_for(&hooks.join("after-ready.ready"));
    let children = fs::read_to_string(format!("/proc/{}/task/{}/children", child.id(), child.id()))
        .expect("children");
    let gate = children.split_whitespace().next().expect("gate PID");
    let command = fs::read(format!("/proc/{gate}/cmdline")).expect("cmdline");
    let expected = b"/usr/bin/python3\0-I\0-S\0";
    assert!(command.starts_with(expected), "{command:?}");
    let environment = fs::read(format!("/proc/{gate}/environ")).expect("environment");
    assert!(
        !environment
            .windows(b"marker-key-for-proc".len())
            .any(|part| part == b"marker-key-for-proc")
    );
    assert!(
        !environment
            .windows(b"THINKTHEN_API_KEY".len())
            .any(|part| part == b"THINKTHEN_API_KEY")
    );
    fs::write(hooks.join("after-ready.release"), b"go\n").expect("release hook");
    let output = child.wait_with_output().expect("output");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
}

#[test]
fn ignored_term_becomes_unresolved_after_one_five_second_grace() {
    let (main, _linked) = temporary_repository("live-unresolved-term");
    let _cleanup = SessionCleanup::new(&main);
    executable(
        &main.join("ignore.sh"),
        "#!/bin/sh\ntrap '' TERM HUP INT\nsh -c 'trap \"\" TERM HUP INT; while :; do sleep 1; done' &\necho ready >ready\nwhile :; do sleep 1; done\n",
    );
    let mut wrapper = live(&main)
        .current_dir(&main)
        .args(["--max-tokens", "2", "ignore.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("wrapper");
    let _wrapper_cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&main.join("ready"));
    let started = Instant::now();
    signal(wrapper.id(), "-TERM");
    thread::sleep(Duration::from_secs(1));
    signal(wrapper.id(), "-HUP");
    let output = wrapper.wait().expect("status");
    let elapsed = started.elapsed();
    assert_eq!(output.code(), Some(143), "{output:?}");
    assert!(elapsed >= Duration::from_millis(4_500), "{elapsed:?}");
    assert!(elapsed < Duration::from_millis(6_500), "{elapsed:?}");
    let state = authority_state(&main);
    assert_eq!(json_number(&state, "charged_tokens"), 2);
    assert!(state.contains("\"phase\":\"unresolved\""), "{state}");
    let refused = live(&main)
        .current_dir(&main)
        .args(["--max-tokens", "1", "quick.sh"])
        .output()
        .expect("pending refusal");
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(recover(&main).status.code(), Some(1));
    let session = json_number(&state, "sid");
    assert!(
        Command::new("/bin/kill")
            .args(["-KILL", "--", &format!("-{session}")])
            .status()
            .expect("kill session")
            .success()
    );
    recover_until_clear(&main, "TERM recovery");
    assert!(authority_state(&main).contains("\"pending\":null"));
}

#[test]
fn hup_and_int_keep_their_status_and_forward_to_the_session() {
    for (case, sent, trap_name, expected) in
        [("hup", "-HUP", "HUP", 129), ("int", "-INT", "TERM", 130)]
    {
        let (main, _linked) = temporary_repository(&format!("live-signal-{case}"));
        executable(
            &main.join("signal.sh"),
            &format!(
                "#!/bin/sh\ntrap 'echo received >received; exit 0' {trap_name}\necho ready >ready\nwhile :; do sleep 1; done\n"
            ),
        );
        let wrapper = live(&main)
            .current_dir(&main)
            .args(["--max-tokens", "1", "signal.sh"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("wrapper");
        wait_for(&main.join("ready"));
        signal(wrapper.id(), sent);
        let output = wrapper.wait_with_output().expect("output");
        assert_eq!(output.status.code(), Some(expected), "{case}: {output:?}");
        assert!(main.join("received").exists(), "{case}");
        assert!(authority_state(&main).contains("\"pending\":null"));
    }
}

#[test]
fn sigkill_boundaries_preserve_release_and_charge_order() {
    let boundaries = [
        ("before-readiness", false, false),
        ("after-ready", false, false),
        ("after-temp-sync", false, false),
        ("after-replace", false, true),
        ("after-directory-sync", false, true),
        ("after-charge", false, true),
        ("after-key-receipt", false, true),
        ("after-release", true, true),
        ("after-job-exit", true, true),
    ];
    for (point, may_run, charged) in boundaries {
        let (main, _linked) =
            temporary_repository(&format!("live-kill-{}", point.replace('-', "_")));
        let hooks = main.join("hooks");
        fs::create_dir(&hooks).expect("hooks");
        let wrapper = live(&main)
            .current_dir(&main)
            .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
            .env("THINKTHEN_LIVE_TEST_POINT", point)
            .args(["--max-tokens", "1", "quick.sh", point])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("wrapper");
        wait_for(&hooks.join(format!("{point}.ready")));
        signal(wrapper.id(), "-KILL");
        let _output = wrapper.wait_with_output().expect("killed output");
        if !may_run {
            assert!(!main.join("quick").exists(), "{point} ran before release");
        }
        let state = authority_state(&main);
        assert_eq!(
            json_number(&state, "charged_tokens"),
            if charged { 1 } else { 0 },
            "{point}"
        );
        if !state.contains("\"pending\":null") {
            recover_until_clear(&main, point);
        }
        assert_eq!(
            json_number(&authority_state(&main), "charged_tokens"),
            if charged { 1 } else { 0 }
        );
    }
}

#[test]
fn file_and_directory_sync_failures_never_release_the_job() {
    for (case, charged) in [("file", 0), ("directory", 1)] {
        let (main, _linked) = temporary_repository(&format!("live-sync-{case}"));
        let output = live(&main)
            .current_dir(&main)
            .env("THINKTHEN_LIVE_FAIL_FSYNC", case)
            .args(["--max-tokens", "1", "quick.sh"])
            .output()
            .expect("sync failure");
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert!(!main.join("quick").exists(), "{case} ran");
        assert_eq!(
            json_number(&authority_state(&main), "charged_tokens"),
            charged,
            "{case}"
        );
    }
}
