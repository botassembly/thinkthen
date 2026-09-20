//! Migration and retirement proofs for the guarded live supervisor.

#[cfg(test)]
#[path = "live_support/mod.rs"]
#[allow(
    dead_code,
    reason = "the shared fixture exposes helpers used by sibling suites"
)]
mod live_support;

use live_support::*;
use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

#[test]
fn migration_refuses_dirty_locked_and_active_trees() {
    let (main, _linked) = temporary_repository("live-migrate-dirty");
    fs::write(main.join("quick.sh"), b"changed\n").expect("dirty file");
    let dirty = migrate(&main, &["--verify"]);
    assert_eq!(dirty.status.code(), Some(1), "{dirty:?}");

    let (main, linked) = temporary_repository("live-migrate-locked");
    run(&main, &["worktree", "lock", linked.to_str().expect("path")]);
    let locked = migrate(&main, &["--verify"]);
    assert_eq!(locked.status.code(), Some(1), "{locked:?}");

    let (main, _linked) = temporary_repository("live-migrate-active");
    let lock = main.join(".git/thinkthen-live/lock");
    let mut holder = Command::new("/usr/bin/flock")
        .args([lock.to_str().expect("lock"), "/bin/sleep", "30"])
        .spawn()
        .expect("lock holder");
    let _cleanup = ProcessCleanup::new(holder.id());
    thread::sleep(Duration::from_millis(50));
    let active = migrate(&main, &["--verify"]);
    assert_eq!(active.status.code(), Some(1), "{active:?}");
    holder.kill().expect("stop holder");
    holder.wait().expect("holder status");
}

#[test]
fn migration_refuses_a_surviving_actual_2c32524_job() {
    let (main, linked) = temporary_repository("live-migrate-active-2c-job");
    let old = Command::new("/usr/bin/git")
        .current_dir(repo())
        .args(["show", "2c32524:sdlc/scripts/live"])
        .output()
        .unwrap();
    executable(
        &linked.join("sdlc/scripts/live"),
        &String::from_utf8(old.stdout).unwrap(),
    );
    fs::write(
        linked.join("sdlc/live-tokens"),
        b"limit_tokens 10\nspent_tokens 0\n",
    )
    .unwrap();
    executable(
        &linked.join("active.sh"),
        "#!/bin/sh\necho $$ >\"$1/pid\"\necho ready >\"$1/ready\"\ntrap '' TERM HUP INT\nwhile :; do sleep 1; done\n",
    );
    run(
        &linked,
        &["add", "sdlc/scripts/live", "sdlc/live-tokens", "active.sh"],
    );
    run(&linked, &["commit", "-qm", "actual historical active job"]);
    let fake_bin = main.parent().unwrap().join("active-job-bin");
    let signals = main.parent().unwrap().join("active-job-signals");
    fs::create_dir(&fake_bin).unwrap();
    fs::create_dir(&signals).unwrap();
    executable(&fake_bin.join("cargo"), "#!/bin/sh\nexit 0\n");
    let mut wrapper = Command::new(linked.join("sdlc/scripts/live"))
        .current_dir(&linked)
        .env_clear()
        .env("PATH", format!("{}:/usr/bin:/bin", fake_bin.display()))
        .env("THINKTHEN_API_KEY", "historical-key")
        .arg(linked.join("active.sh"))
        .arg(&signals)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _wrapper_cleanup = ProcessCleanup::new(wrapper.id());
    wait_for(&signals.join("ready"));
    let job_pid: u32 = fs::read_to_string(signals.join("pid"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let _job_cleanup = ProcessCleanup::new(job_pid);
    signal(wrapper.id(), "-KILL");
    let _status = wrapper.wait().unwrap();
    let refused = migrate(&main, &["--verify"]);
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    assert!(String::from_utf8_lossy(&refused.stderr).contains("active legacy live job"));
}

#[test]
fn failed_activation_fsync_leaves_runtime_disabled() {
    let (main, _linked) = temporary_repository("live-failed-activation");
    let authority = main.join(".git/thinkthen-live");
    fs::remove_dir_all(&authority).unwrap();
    let failed = Command::new(main.join("sdlc/scripts/live-migrate"))
        .current_dir(&main)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("THINKTHEN_LIVE_FAIL_FSYNC", "authority")
        .args(["--activate", "10", "0"])
        .output()
        .unwrap();
    assert_eq!(failed.status.code(), Some(1), "{failed:?}");
    for arguments in [vec!["--status"], vec!["--max-tokens", "1", "quick.sh"]] {
        let output = Command::new(main.join("sdlc/scripts/live"))
            .current_dir(&main)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("THINKTHEN_API_KEY", "test-key")
            .args(arguments)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(!main.join("quick").exists());
    }
}

#[test]
fn persistent_failure_after_active_publication_keeps_the_fence_closed() {
    let (main, _linked) = temporary_repository("live-persistent-activation-failure");
    let authority = main.join(".git/thinkthen-live");
    fs::remove_dir_all(&authority).unwrap();
    let failed = Command::new(main.join("sdlc/scripts/live-migrate"))
        .current_dir(&main)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("THINKTHEN_LIVE_FAIL_FSYNC_FROM", "6")
        .args(["--activate", "10", "0"])
        .output()
        .unwrap();
    assert_eq!(failed.status.code(), Some(1), "{failed:?}");
    assert!(authority.join("activation-fence").exists());
    for arguments in [vec!["--status"], vec!["--max-tokens", "1", "quick.sh"]] {
        let output = Command::new(main.join("sdlc/scripts/live"))
            .current_dir(&main)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("THINKTHEN_API_KEY", "test-key")
            .args(arguments)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(!main.join("quick").exists());
    }
}

#[test]
fn migration_refuses_relative_2c32524_wrapper_with_execed_job() {
    let (main, linked) = temporary_repository("live-relative-2c-wrapper");
    let old = Command::new("/usr/bin/git")
        .current_dir(repo())
        .args(["show", "2c32524:sdlc/scripts/live"])
        .output()
        .unwrap();
    executable(
        &linked.join("sdlc/scripts/live"),
        &String::from_utf8(old.stdout).unwrap(),
    );
    fs::write(
        linked.join("sdlc/live-tokens"),
        b"limit_tokens 10\nspent_tokens 0\n",
    )
    .unwrap();
    executable(
        &linked.join("exec-sleep.sh"),
        "#!/bin/sh\nexec /bin/sleep 30\n",
    );
    run(
        &linked,
        &[
            "add",
            "sdlc/scripts/live",
            "sdlc/live-tokens",
            "exec-sleep.sh",
        ],
    );
    run(&linked, &["commit", "-qm", "relative historical wrapper"]);
    let fake_bin = main.parent().unwrap().join("relative-wrapper-bin");
    fs::create_dir(&fake_bin).unwrap();
    executable(&fake_bin.join("cargo"), "#!/bin/sh\nexit 0\n");
    let mut wrapper = Command::new("sdlc/scripts/live")
        .current_dir(&linked)
        .env_clear()
        .env("PATH", format!("{}:/usr/bin:/bin", fake_bin.display()))
        .env("THINKTHEN_API_KEY", "historical-key")
        .arg("exec-sleep.sh")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _cleanup = ProcessCleanup::new(wrapper.id());
    let children = format!("/proc/{0}/task/{0}/children", wrapper.id());
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    let job_pid = loop {
        if let Some(pid) = fs::read_to_string(&children)
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .and_then(|value| value.parse::<u32>().ok())
        {
            break pid;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "historical job missing"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    let _job_cleanup = ProcessCleanup::new(job_pid);
    let refused = migrate(&main, &["--verify"]);
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    assert!(String::from_utf8_lossy(&refused.stderr).contains("active legacy"));
    wrapper.kill().unwrap();
    wrapper.wait().unwrap();
}

#[test]
fn retirement_uses_primary_tree_and_preserves_branch_tip_and_directory() {
    let (main, linked) = temporary_repository("live-migrate-retire");
    let old = Command::new("/usr/bin/git")
        .current_dir(repo())
        .args(["show", "a556b97:sdlc/scripts/live"])
        .output()
        .expect("old launcher");
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
    run(&linked, &["commit", "-qm", "historical launcher"]);
    let branch_tip = Command::new("/usr/bin/git")
        .current_dir(&linked)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap()
        .stdout;
    let landed = Command::new("/usr/bin/git")
        .current_dir(&main)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let landed = String::from_utf8(landed.stdout).unwrap();

    let retired = migrate(&linked, &["--retire", landed.trim()]);
    assert!(retired.status.success(), "{retired:?}");
    assert!(main.is_dir());
    assert!(linked.is_dir());
    let primary_branch = Command::new("/usr/bin/git")
        .current_dir(&main)
        .args(["symbolic-ref", "--short", "HEAD"])
        .output()
        .unwrap();
    assert!(primary_branch.status.success(), "{primary_branch:?}");
    let preserved = Command::new("/usr/bin/git")
        .current_dir(&main)
        .args(["rev-parse", "refs/heads/linked"])
        .output()
        .unwrap();
    assert_eq!(preserved.stdout, branch_tip);

    let historical = main.parent().unwrap().join("historical");
    run(
        &main,
        &[
            "worktree",
            "add",
            "-q",
            historical.to_str().unwrap(),
            "refs/heads/linked",
        ],
    );
    fs::create_dir_all(historical.join("target/debug")).unwrap();
    executable(
        &historical.join("target/debug/thinkthen"),
        "#!/bin/sh\nexit 0\n",
    );
    let refused = live(&main)
        .current_dir(&main)
        .args(["--max-tokens", "1", "quick.sh"])
        .output()
        .expect("historical refusal");
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
}
