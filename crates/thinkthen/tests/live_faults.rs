//! Failure-path proofs for the durable live supervisor.

#[cfg(test)]
#[path = "live_support/mod.rs"]
#[allow(
    dead_code,
    reason = "the shared fixture exposes helpers used by sibling suites"
)]
mod live_support;

use live_support::*;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn stopped_gate_and_large_environment_keep_term_bounded() {
    let (main, _linked) = temporary_repository("live-stopped-large-delivery");
    let _cleanup = SessionCleanup::new(&main);
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).expect("hooks");
    let mut command = live(&main);
    command
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "after-charge")
        .args(["--max-tokens", "1", "quick.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let large = "x".repeat(50_000);
    for number in 0..24 {
        command.env(format!("LARGE_{number}"), &large);
    }
    let mut wrapper = command.spawn().expect("wrapper");
    let _wrapper_cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&hooks.join("after-charge.ready"));
    let gate = json_number(&authority_state(&main), "pid");
    signal(gate as u32, "-STOP");
    fs::write(hooks.join("after-charge.release"), b"go\n").expect("release hook");
    thread::sleep(Duration::from_millis(100));
    let started = Instant::now();
    signal(wrapper.id(), "-TERM");
    let result = wrapper.wait().expect("wrapper status");
    let elapsed = started.elapsed();
    assert_eq!(result.code(), Some(143), "{result:?}");
    assert!(elapsed >= Duration::from_millis(4_500), "{elapsed:?}");
    assert!(elapsed < Duration::from_millis(6_500), "{elapsed:?}");
    assert!(!main.join("quick").exists());
    assert!(authority_state(&main).contains("\"phase\":\"unresolved\""));
}

#[test]
fn child_wait_deadline_precedes_delayed_unresolved_sync() {
    let (main, _linked) = temporary_repository("live-child-wait-before-sync");
    let _session_cleanup = SessionCleanup::new(&main);
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).unwrap();
    executable(
        &main.join("ignore.sh"),
        "#!/bin/sh\ntrap '' TERM HUP INT\necho ready >ready\nwhile :; do sleep 1; done\n",
    );
    let mut wrapper = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "before-unresolved-sync")
        .args(["--max-tokens", "1", "ignore.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _wrapper_cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&main.join("ready"));
    let started = Instant::now();
    signal(wrapper.id(), "-TERM");
    let sync_ready = hooks.join("before-unresolved-sync.ready");
    while !sync_ready.exists() {
        assert!(
            started.elapsed() < Duration::from_secs(7),
            "sync hook missing"
        );
        thread::sleep(Duration::from_millis(10));
    }
    let child_wait = started.elapsed();
    assert!(child_wait >= Duration::from_millis(4_500), "{child_wait:?}");
    assert!(child_wait < Duration::from_millis(5_700), "{child_wait:?}");
    thread::sleep(Duration::from_millis(1_200));
    fs::write(hooks.join("before-unresolved-sync.release"), b"go\n").unwrap();
    let status = wrapper.wait().unwrap();
    assert_eq!(status.code(), Some(143), "{status:?}");
    assert!(started.elapsed() >= Duration::from_millis(5_700));
}

#[test]
fn launch_faults_refuse_without_tracebacks_or_early_execution() {
    for (fault, expected_charge, expected_status) in [
        ("prerequisite", 0, 1),
        ("socket", 0, 1),
        ("spawn", 0, 1),
        ("readiness", 0, 1),
        ("setsid", 0, 1),
        ("identity", 0, 1),
        ("exec", 1, 125),
    ] {
        let (main, _linked) = temporary_repository(&format!("live-fault-{fault}"));
        let output = live(&main)
            .current_dir(&main)
            .env("THINKTHEN_LIVE_TEST_FAULT", fault)
            .args(["--max-tokens", "1", "quick.sh"])
            .output()
            .expect("fault result");
        assert_eq!(
            output.status.code(),
            Some(expected_status),
            "{fault}: {output:?}"
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("Traceback"));
        assert!(!main.join("quick").exists(), "{fault} executed the job");
        assert_eq!(
            json_number(&authority_state(&main), "charged_tokens"),
            expected_charge
        );
    }
}

#[test]
fn checkout_cwd_selects_the_matching_worktree_binary() {
    let (main, linked) = temporary_repository("live-cwd-and-binary");
    for (tree, label) in [(&main, "main"), (&linked, "linked")] {
        executable(
            &tree.join("target/debug/thinkthen"),
            &format!("#!/bin/sh\necho {label}\n"),
        );
        executable(
            &tree.join("which.sh"),
            "#!/bin/sh\npwd >calling-cwd\nthinkthen >called-binary\n",
        );
        let outside = tree.parent().expect("outside");
        let output = live(tree)
            .current_dir(outside)
            .args(["--max-tokens", "1", "which.sh"])
            .output()
            .expect("outside invocation");
        assert_eq!(output.status.code(), Some(0), "{label}: {output:?}");
        assert_eq!(
            fs::read_to_string(tree.join("calling-cwd")).unwrap().trim(),
            tree.display().to_string()
        );
        assert_eq!(
            fs::read_to_string(tree.join("called-binary")).unwrap(),
            format!("{label}\n")
        );
    }
}

#[test]
fn signaled_job_maps_to_shell_status() {
    let (main, _linked) = temporary_repository("live-signaled-status");
    executable(&main.join("term.sh"), "#!/bin/sh\nkill -TERM $$\n");
    let output = live(&main)
        .current_dir(&main)
        .args(["--max-tokens", "1", "term.sh"])
        .output()
        .expect("signaled job");
    assert_eq!(output.status.code(), Some(143), "{output:?}");
    assert!(authority_state(&main).contains("\"pending\":null"));
}

#[test]
fn signal_before_verified_readiness_stops_only_the_owned_child() {
    let (main, _linked) = temporary_repository("live-signal-before-ready");
    let mut wrapper = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_FAULT", "readiness")
        .args(["--max-tokens", "1", "quick.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("wrapper");
    let _cleanup = ProcessCleanup::new(wrapper.id());
    let children = PathBuf::from(format!("/proc/{0}/task/{0}/children", wrapper.id()));
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let gate_exists = fs::read_to_string(&children)
            .unwrap_or_default()
            .split_whitespace()
            .filter_map(|pid| fs::read(format!("/proc/{pid}/cmdline")).ok())
            .any(|command| {
                command
                    .windows(b"--gate".len())
                    .any(|part| part == b"--gate")
            });
        if gate_exists {
            break;
        }
        assert!(Instant::now() < deadline, "gate was not spawned");
        thread::sleep(Duration::from_millis(10));
    }
    let started = Instant::now();
    signal(wrapper.id(), "-TERM");
    let result = wrapper.wait().expect("wrapper status");
    assert_eq!(result.code(), Some(143), "{result:?}");
    assert!(started.elapsed() < Duration::from_millis(1_500));
    assert_eq!(json_number(&authority_state(&main), "charged_tokens"), 0);
}

#[test]
fn exited_leader_with_a_member_in_another_group_becomes_unresolved() {
    let (main, _linked) = temporary_repository("live-surviving-session-member");
    let _cleanup = SessionCleanup::new(&main);
    executable(
        &main.join("fork.sh"),
        "#!/bin/sh\nexec /usr/bin/python3 -c 'import os,signal,time; p=os.fork(); os._exit(0) if p else None; os.setpgid(0,0); signal.signal(signal.SIGTERM, signal.SIG_IGN); open(\"ready\",\"w\").close(); time.sleep(30)'\n",
    );
    let started = Instant::now();
    let output = live(&main)
        .current_dir(&main)
        .args(["--max-tokens", "1", "fork.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("leader-exit result");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(started.elapsed() >= Duration::from_millis(4_500));
    assert!(authority_state(&main).contains("\"phase\":\"unresolved\""));
    assert!(main.join("ready").exists());
}

#[test]
fn sigkill_after_release_preserves_an_active_job_for_recovery() {
    let (main, _linked) = temporary_repository("live-active-job-sigkill");
    let _session_cleanup = SessionCleanup::new(&main);
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).unwrap();
    let mut wrapper = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "after-release")
        .args(["--max-tokens", "1", "hold.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _wrapper_cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&hooks.join("after-release.ready"));
    fs::write(hooks.join("after-release.release"), b"go\n").unwrap();
    wait_for(&main.join("ready"));
    signal(wrapper.id(), "-KILL");
    let _status = wrapper.wait().unwrap();
    let refused = recover(&main);
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    let session = json_number(&authority_state(&main), "sid");
    Command::new("/bin/kill")
        .args(["-KILL", "--", &format!("-{session}")])
        .status()
        .unwrap();
    recover_until_clear(&main, "active job SIGKILL");
}

#[test]
fn changed_boot_accepts_large_process_ids_without_inspection() {
    let (main, _linked) = temporary_repository("live-old-boot-large-pids");
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).unwrap();
    let mut wrapper = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "after-replace")
        .args(["--max-tokens", "1", "quick.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&hooks.join("after-replace.ready"));
    signal(wrapper.id(), "-KILL");
    let _status = wrapper.wait().unwrap();
    let state = main.join(".git/thinkthen-live/state.json");
    let script = r#"import json,sys
p=sys.argv[1]
s=json.load(open(p))
s['pending']['wrapper'].update(boot_id='00000000-0000-0000-0000-000000000000',pid=1500000000,start_ticks=1500000001)
s['pending']['gate'].update(boot_id='00000000-0000-0000-0000-000000000000',pid=1600000000,pgid=1600000000,sid=1600000000,start_ticks=1600000001)
open(p,'w').write(json.dumps(s,separators=(',',':'),sort_keys=True)+'\n')
"#;
    assert!(
        Command::new("/usr/bin/python3")
            .args(["-c", script])
            .arg(&state)
            .status()
            .unwrap()
            .success()
    );
    let recovered = recover(&main);
    assert_eq!(recovered.status.code(), Some(0), "{recovered:?}");
    assert!(authority_state(&main).contains("\"pending\":null"));
}

#[test]
fn a_reused_pid_with_another_start_identity_does_not_block_recovery() {
    let (main, _linked) = temporary_repository("live-reused-pid");
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).unwrap();
    let mut wrapper = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "after-replace")
        .args(["--max-tokens", "1", "quick.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&hooks.join("after-replace.ready"));
    signal(wrapper.id(), "-KILL");
    let _status = wrapper.wait().unwrap();
    let state = main.join(".git/thinkthen-live/state.json");
    let reused = std::process::id().to_string();
    let mutate = "import json,sys;p=sys.argv[1];n=int(sys.argv[2]);s=json.load(open(p));s['pending']['gate'].update(pid=n,pgid=n,sid=n,start_ticks=0);open(p,'w').write(json.dumps(s,separators=(',',':'),sort_keys=True)+'\\n')";
    assert!(
        Command::new("/usr/bin/python3")
            .args(["-c", mutate])
            .arg(&state)
            .arg(reused)
            .status()
            .unwrap()
            .success()
    );
    let recovered = recover(&main);
    assert_eq!(recovered.status.code(), Some(0), "{recovered:?}");
    assert!(authority_state(&main).contains("\"pending\":null"));
}

#[test]
fn runtime_never_creates_a_missing_permanent_lock() {
    let (main, _linked) = temporary_repository("live-missing-permanent-lock");
    let lock = main.join(".git/thinkthen-live/lock");
    fs::remove_file(&lock).expect("remove lock");
    for arguments in [
        vec!["--status"],
        vec!["--recover"],
        vec!["--max-tokens", "1", "quick.sh"],
    ] {
        let output = Command::new(main.join("sdlc/scripts/live"))
            .current_dir(&main)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("THINKTHEN_API_KEY", "test-key")
            .args(arguments)
            .output()
            .expect("missing lock refusal");
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(!lock.exists());
    }
    let activate = migrate(&main, &["--activate", "10", "0"]);
    assert_eq!(activate.status.code(), Some(1), "{activate:?}");
    assert!(!lock.exists());
}

#[test]
fn malformed_state_types_and_ids_fail_with_safe_diagnostics() {
    for (case, transform) in [
        (
            "authority-type",
            (
                "\"authority_id\":\"",
                "\"authority_id\":false,\"discard\":\"",
            ),
        ),
        (
            "authority-uuid",
            ("\"authority_id\":\"", "\"authority_id\":\"bad"),
        ),
        ("pending-type", ("\"pending\":null", "\"pending\":[]")),
        (
            "numeric-bool",
            ("\"charged_tokens\":0", "\"charged_tokens\":false"),
        ),
        (
            "numeric-negative",
            ("\"charged_tokens\":0", "\"charged_tokens\":-1"),
        ),
        (
            "numeric-large",
            ("\"limit_tokens\":10", "\"limit_tokens\":1000000000"),
        ),
    ] {
        let (main, _linked) = temporary_repository(&format!("live-malformed-{case}"));
        let path = main.join(".git/thinkthen-live/state.json");
        let before = fs::read_to_string(&path).expect("state");
        fs::write(&path, before.replacen(transform.0, transform.1, 1)).expect("mutated state");
        let output = live(&main)
            .current_dir(&main)
            .args(["--max-tokens", "1", "quick.sh"])
            .output()
            .expect("malformed refusal");
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.starts_with("live: "), "{case}: {error}");
        assert!(!error.contains("Traceback"), "{case}: {error}");
        assert!(!main.join("quick").exists());
    }
}

#[test]
fn every_top_level_state_type_refuses_without_a_traceback() {
    for (field, value) in [
        ("version", "true"),
        ("authority_id", "false"),
        ("status", "[]"),
        ("machine_id", "{}"),
        ("common_dir_hex", "1"),
    ] {
        let (main, _linked) = temporary_repository(&format!("live-type-{field}"));
        let path = main.join(".git/thinkthen-live/state.json");
        let mutate = "import json,sys;p=sys.argv[1];s=json.load(open(p));s[sys.argv[2]]=json.loads(sys.argv[3]);open(p,'w').write(json.dumps(s,separators=(',',':'),sort_keys=True)+'\\n')";
        assert!(
            Command::new("/usr/bin/python3")
                .args(["-c", mutate])
                .arg(&path)
                .args([field, value])
                .status()
                .unwrap()
                .success()
        );
        let output = live(&main)
            .current_dir(&main)
            .args(["--max-tokens", "1", "quick.sh"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{field}: {output:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "live: the live authority state is malformed\n"
        );
    }
}

#[test]
fn unknown_schema_machine_binding_and_retirement_refuse() {
    for case in ["unknown", "machine", "retired"] {
        let (main, _linked) = temporary_repository(&format!("live-state-{case}"));
        let path = main.join(".git/thinkthen-live/state.json");
        let before = fs::read_to_string(&path).unwrap();
        let changed = match case {
            "unknown" => before.replacen("{\"authority_id\"", "{\"extra\":0,\"authority_id\"", 1),
            "machine" => before.replacen("\"machine_id\":\"", "\"machine_id\":\"0", 1),
            "retired" => before.replacen("\"status\":\"active\"", "\"status\":\"retired\"", 1),
            _ => unreachable!(),
        };
        fs::write(&path, changed).unwrap();
        let output = live(&main)
            .current_dir(&main)
            .args(["--max-tokens", "1", "quick.sh"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("Traceback"));
    }
}

#[test]
fn unsafe_authority_files_are_refused() {
    for case in [
        "directory-permission",
        "state-permission",
        "state-symlink",
        "lock-symlink",
    ] {
        let (main, _linked) = temporary_repository(&format!("live-unsafe-{case}"));
        let authority = main.join(".git/thinkthen-live");
        let state = authority.join("state.json");
        let lock = authority.join("lock");
        match case {
            "directory-permission" => set_mode(&authority, 0o755).unwrap(),
            "state-permission" => set_mode(&state, 0o644).unwrap(),
            "state-symlink" => {
                let copy = authority.join("copy");
                fs::rename(&state, &copy).unwrap();
                std::os::unix::fs::symlink(&copy, &state).unwrap();
            }
            "lock-symlink" => {
                fs::remove_file(&lock).unwrap();
                std::os::unix::fs::symlink("state.json", &lock).unwrap();
            }
            _ => unreachable!(),
        }
        let output = live(&main)
            .current_dir(&main)
            .arg("--status")
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("Traceback"));
    }
}

fn set_mode(path: &std::path::Path, mode: u32) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(mode);
    fs::set_permissions(path, permissions)
}
