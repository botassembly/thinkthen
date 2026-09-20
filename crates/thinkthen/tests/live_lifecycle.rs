//! Lifecycle tests for durable charging, key confinement, paths, and signals.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const KEY: &str = "review-secret-sentinel";

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn executable(path: &Path, text: &str) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::write(path, text)?;
    let mut mode = fs::metadata(path)?.permissions();
    mode.set_mode(0o755);
    fs::set_permissions(path, mode)
}

fn fixture(name: &str) -> io::Result<PathBuf> {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("sdlc/scripts"))?;
    fs::create_dir_all(root.join("fake-bin"))?;
    fs::create_dir_all(root.join("target/debug"))?;
    fs::copy(
        repo().join("sdlc/scripts/live"),
        root.join("sdlc/scripts/live"),
    )?;
    fs::write(
        root.join("sdlc/live-tokens"),
        "limit_tokens 100\nspent_tokens 10\n",
    )?;
    executable(
        &root.join("fake-bin/cargo"),
        "#!/bin/sh\n[ -z \"${THINKTHEN_API_KEY+x}\" ] && [ -z \"${live_api_key+x}\" ] || echo cargo >>\"$LEAK_LOG\"\necho build >>\"$ORDER_LOG\"\nif [ \"${BUILD_HOLD:-}\" = yes ]; then echo ready >\"$BUILD_READY\"; while [ ! -e \"$BUILD_RELEASE\" ]; do sleep 0.02; done; fi\nexit \"${BUILD_STATUS:-0}\"\n",
    )?;
    executable(
        &root.join("fake-bin/sync"),
        "#!/bin/sh\n[ -z \"${THINKTHEN_API_KEY+x}\" ] && [ -z \"${live_api_key+x}\" ] || echo sync >>\"$LEAK_LOG\"\ncount=0; [ ! -f \"$SYNC_COUNT\" ] || count=$(cat \"$SYNC_COUNT\"); count=$((count + 1)); echo \"$count\" >\"$SYNC_COUNT\"\nif [ \"$count\" -eq 1 ]; then grep -q '^spent_tokens 17$' \"$LEDGER_CHECK\" && grep -q '^state charged$' \"$OWNER_CHECK\" && echo durable >>\"$ORDER_LOG\"; fi\nif [ \"$count\" -eq 2 ]; then grep -Eq '^child_pid [1-9][0-9]*$' \"$OWNER_CHECK\" && echo child-durable >>\"$ORDER_LOG\"; if [ \"${CHILD_HOLD:-}\" = yes ]; then echo ready >\"$CHILD_READY\"; while [ ! -e \"$CHILD_RELEASE\" ]; do sleep 0.02; done; fi; fi\n[ \"${SYNC_FAIL_AT:-}\" != \"$count\" ] || exit 9\n",
    )?;
    Ok(root)
}

fn job(root: &Path, body: &str) -> io::Result<PathBuf> {
    let path = root.join("job.sh");
    executable(&path, &format!("#!/bin/sh\nset -eu\n{body}\n"))?;
    Ok(path)
}

fn command(root: &Path) -> Command {
    let mut run = Command::new("sh");
    run.env_clear()
        .env(
            "PATH",
            format!("{}:/usr/bin:/bin", root.join("fake-bin").display()),
        )
        .env("THINKTHEN_API_KEY", KEY)
        .env("live_api_key", "preexisting-export")
        .env("LEAK_LOG", root.join("leak"))
        .env("ORDER_LOG", root.join("order"))
        .env("SYNC_COUNT", root.join("sync-count"))
        .env("LEDGER_CHECK", root.join("sdlc/live-tokens"))
        .env("OWNER_CHECK", root.join("sdlc/live-tokens.lock/owner"))
        .env("BUILD_READY", root.join("build-ready"))
        .env("BUILD_RELEASE", root.join("build-release"))
        .env("CHILD_READY", root.join("child-ready"))
        .env("CHILD_RELEASE", root.join("child-release"))
        .arg(root.join("sdlc/scripts/live"));
    run
}

fn live(root: &Path, job: impl AsRef<Path>) -> io::Result<Output> {
    command(root)
        .args(["--max-tokens", "7"])
        .arg(job.as_ref())
        .output()
}

fn spawn(root: &Path, job: impl AsRef<Path>) -> io::Result<Child> {
    command(root)
        .args(["--max-tokens", "7"])
        .arg(job.as_ref())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "{} was not written",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn signal(pid: u32, name: &str) -> io::Result<()> {
    let status = Command::new("kill")
        .args([name, &pid.to_string()])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other("signal command failed"))
    }
}

fn wait_gone(pid: u32) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Command::new("kill")
        .args(["-0", &pid.to_string()])
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
    {
        assert!(Instant::now() < deadline, "process {pid} stayed alive");
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn durable_charge_precedes_job_and_sync_failure_starts_nothing() {
    let root = fixture("live-durable-charge").expect("fixture");
    let work = job(&root, "echo job >>\"$ORDER_LOG\"").expect("job");
    let output = live(&root, &work).expect("run");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        fs::read_to_string(root.join("order")).expect("order"),
        "build\ndurable\nchild-durable\njob\n"
    );

    let root = fixture("live-sync-failure").expect("fixture");
    let work = job(&root, "echo ran >ran").expect("job");
    let output = command(&root)
        .env("SYNC_FAIL_AT", "1")
        .args(["--max-tokens", "7"])
        .arg(&work)
        .output()
        .expect("run");
    assert_ne!(output.status.code(), Some(0));
    assert!(!root.join("ran").exists());
    assert!(root.join("sdlc/live-tokens.lock/owner").exists());
    assert_eq!(
        fs::read_to_string(root.join("sdlc/live-tokens")).expect("ledger"),
        "limit_tokens 100\nspent_tokens 17\n"
    );

    let root = fixture("live-child-sync-failure").expect("fixture");
    let work = job(&root, "echo ran >ran").expect("job");
    let output = command(&root)
        .env("SYNC_FAIL_AT", "2")
        .args(["--max-tokens", "7"])
        .arg(&work)
        .output()
        .expect("run");
    assert_ne!(output.status.code(), Some(0));
    assert!(!root.join("ran").exists());
    let owner = fs::read_to_string(root.join("sdlc/live-tokens.lock/owner")).expect("owner");
    assert!(owner.contains("state charged\n"), "{owner}");
    assert!(!owner.contains("child_pid 0\n"), "{owner}");
}

#[test]
fn relative_job_is_resolved_against_repository_once() {
    let root = fixture("live-relative-job").expect("fixture");
    job(
        &root,
        "echo repo >repo-ran; printf '%s' \"$0\" >invoked-path",
    )
    .expect("repo job");
    let outside = root.join("outside");
    fs::create_dir(&outside).expect("outside");
    executable(
        &outside.join("job.sh"),
        "#!/bin/sh\necho outside >outside-ran\n",
    )
    .expect("conflicting job");
    let output = command(&root)
        .current_dir(&outside)
        .args(["--max-tokens", "7", "job.sh"])
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(root.join("repo-ran").exists());
    assert!(!outside.join("outside-ran").exists());
    assert_eq!(
        fs::read_to_string(root.join("invoked-path")).expect("invoked path"),
        root.join("job.sh").to_string_lossy()
    );
}

#[test]
fn relative_job_preserves_trailing_newlines_in_directory_and_name() {
    let root = fixture("live-newline-job").expect("fixture");
    let relative = PathBuf::from("trailing directory\n").join("job\n");
    fs::create_dir(root.join("trailing directory\n")).expect("job directory");
    executable(
        &root.join(&relative),
        "#!/bin/sh\nprintf '%s' \"$0\" >\"${0%/*}/invoked-path\"\n",
    )
    .expect("job");
    let outside = root.join("outside");
    fs::create_dir(&outside).expect("outside");
    let output = command(&root)
        .current_dir(&outside)
        .args(["--max-tokens", "7"])
        .arg(&relative)
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        fs::read_to_string(root.join("trailing directory\n/invoked-path")).expect("invoked path"),
        root.join(relative).to_string_lossy()
    );
}

#[test]
fn initial_owner_failure_removes_uncharged_lock() {
    let root = fixture("live-owner-failure").expect("fixture");
    executable(
        &root.join("fake-bin/mv"),
        "#!/bin/sh\ncase \"$3\" in */live-tokens.lock/owner) exit 9;; esac\nexec /usr/bin/mv \"$@\"\n",
    )
    .expect("mv");
    let work = job(&root, "echo ran >ran").expect("job");
    let output = live(&root, &work).expect("run");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "live: the initial lock owner could not be written; the uncharged lock was removed\n"
    );
    assert!(!root.join("ran").exists());
    assert!(!root.join("sdlc/live-tokens.lock").exists());
    assert_eq!(
        fs::read_to_string(root.join("sdlc/live-tokens")).expect("ledger"),
        "limit_tokens 100\nspent_tokens 10\n"
    );
}

#[test]
fn key_reaches_only_job_environment_and_never_an_error() {
    let root = fixture("live-key-scope").expect("fixture");
    for tool in ["awk", "find", "mkfifo", "mv", "sed"] {
        executable(
            &root.join("fake-bin").join(tool),
            &format!("#!/bin/sh\n[ -z \"${{THINKTHEN_API_KEY+x}}\" ] && [ -z \"${{live_api_key+x}}\" ] || echo {tool} >>\"$LEAK_LOG\"\nexec /usr/bin/{tool} \"$@\"\n"),
        )
        .expect("bookkeeping wrapper");
    }
    let work = job(
        &root,
        "[ \"$THINKTHEN_API_KEY\" = review-secret-sentinel ]; printf x >usage.json",
    )
    .expect("job");
    let output = live(&root, &work).expect("run");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(!root.join("leak").exists());
    assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));
    assert!(!String::from_utf8_lossy(&output.stdout).contains(KEY));

    let root = fixture("live-key-error").expect("fixture");
    let work = job(&root, "echo ran >ran").expect("job");
    let output = command(&root)
        .env("BUILD_STATUS", "8")
        .args(["--max-tokens", "7"])
        .arg(&work)
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(8));
    assert!(!root.join("leak").exists());
    assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));

    let root = fixture("live-key-newline").expect("fixture");
    let work = job(&root, "echo ran >ran").expect("job");
    let output = command(&root)
        .env("THINKTHEN_API_KEY", "first\nsecond")
        .args(["--max-tokens", "7"])
        .arg(&work)
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "live: THINKTHEN_API_KEY contains a line break, so no call goes out\n"
    );
    assert!(!root.join("order").exists());
    assert!(!root.join("sdlc/live-tokens.lock").exists());
}

#[test]
fn fifo_preserves_every_accepted_key_byte() {
    for (case, key) in [
        ("spaces", " leading and trailing "),
        ("backslash", r"two\parts"),
        ("tab", "left\tright"),
        ("carriage-return", "left\rright"),
    ] {
        let root = fixture(&format!("live-key-bytes-{case}")).expect("fixture");
        let work = job(&root, "printf '%s' \"$THINKTHEN_API_KEY\" >received").expect("job");
        let output = command(&root)
            .env("THINKTHEN_API_KEY", key)
            .args(["--max-tokens", "7"])
            .arg(&work)
            .output()
            .expect("run");
        assert_eq!(output.status.code(), Some(0), "{case}: {output:?}");
        assert_eq!(
            fs::read(root.join("received")).expect("received key"),
            key.as_bytes()
        );
    }
}

#[test]
fn signals_during_build_and_child_start_stop_before_paid_job() {
    for (case, signal_name, status) in [("hup", "-HUP", 129), ("int", "-INT", 130)] {
        let root = fixture(&format!("live-signal-build-{case}")).expect("fixture");
        let work = job(&root, "echo ran >ran").expect("job");
        let child = command(&root)
            .env("BUILD_HOLD", "yes")
            .args(["--max-tokens", "7"])
            .arg(&work)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("wrapper");
        wait_for(&root.join("build-ready"));
        signal(child.id(), signal_name).expect("signal");
        fs::write(root.join("build-release"), "go").expect("release build");
        let output = child.wait_with_output().expect("output");
        assert_eq!(output.status.code(), Some(status), "{case}: {output:?}");
        assert!(!root.join("ran").exists());
        assert_eq!(
            fs::read_to_string(root.join("sdlc/live-tokens")).expect("ledger"),
            "limit_tokens 100\nspent_tokens 10\n"
        );
    }

    for (case, signal_name, status) in [("int", "-INT", 130), ("term", "-TERM", 143)] {
        let root = fixture(&format!("live-signal-child-start-{case}")).expect("fixture");
        let work = job(&root, "echo ran >ran").expect("job");
        let child = command(&root)
            .env("CHILD_HOLD", "yes")
            .args(["--max-tokens", "7"])
            .arg(&work)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("wrapper");
        wait_for(&root.join("child-ready"));
        signal(child.id(), signal_name).expect("signal");
        fs::write(root.join("child-release"), "go").expect("release sync");
        let output = child.wait_with_output().expect("output");
        assert_eq!(output.status.code(), Some(status), "{case}: {output:?}");
        assert!(!root.join("ran").exists());
    }
}

#[test]
fn first_signal_wins_and_post_child_signal_survives_bookkeeping() {
    let root = fixture("live-repeated-signal").expect("fixture");
    let work = job(
        &root,
        "trap 'echo hup >>signals' HUP; trap 'echo term >>signals; exit 0' TERM; echo ready >ready; while :; do sleep 0.1; done",
    )
    .expect("job");
    let child = spawn(&root, &work).expect("wrapper");
    wait_for(&root.join("ready"));
    signal(child.id(), "-HUP").expect("signal");
    wait_for(&root.join("signals"));
    signal(child.id(), "-TERM").expect("signal");
    let output = child.wait_with_output().expect("output");
    assert_eq!(output.status.code(), Some(129), "{output:?}");
    let signals = fs::read_to_string(root.join("signals")).expect("signals");
    assert!(signals.contains("hup\n"), "{signals}");
    assert!(signals.contains("term\n"), "{signals}");

    let root = fixture("live-running-int").expect("fixture");
    let work = job(
        &root,
        "trap 'echo term >term-received; exit 0' TERM; echo ready >ready; while :; do sleep 0.1; done",
    )
    .expect("job");
    let child = spawn(&root, &work).expect("wrapper");
    wait_for(&root.join("ready"));
    signal(child.id(), "-INT").expect("signal");
    let output = child.wait_with_output().expect("output");
    assert_eq!(output.status.code(), Some(130), "{output:?}");
    assert_eq!(
        fs::read_to_string(root.join("term-received")).expect("TERM receipt"),
        "term\n"
    );

    let root = fixture("live-post-child-signal").expect("fixture");
    executable(
        &root.join("fake-bin/find"),
        "#!/bin/sh\nfor arg do if [ \"$arg\" = -newer ]; then echo ready >\"$FIND_READY\"; while [ ! -e \"$FIND_RELEASE\" ]; do sleep 0.02; done; fi; done\nexec /usr/bin/find \"$@\"\n",
    )
    .expect("find");
    let work = job(&root, "echo done >done; printf x >usage.json").expect("job");
    let child = command(&root)
        .env("FIND_READY", root.join("find-ready"))
        .env("FIND_RELEASE", root.join("find-release"))
        .args(["--max-tokens", "7"])
        .arg(&work)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("wrapper");
    wait_for(&root.join("find-ready"));
    signal(child.id(), "-TERM").expect("signal");
    fs::write(root.join("find-release"), "go").expect("release find");
    let output = child.wait_with_output().expect("output");
    assert_eq!(output.status.code(), Some(143), "{output:?}");
    assert!(!root.join("sdlc/live-tokens.lock").exists());
}

#[test]
fn killed_wrapper_leaves_only_a_keyless_gate_that_recovery_stops() {
    let root = fixture("live-killed-before-child-owner").expect("fixture");
    executable(
        &root.join("fake-bin/mv"),
        "#!/bin/sh\nif [ \"$3\" = \"$OWNER_CHECK\" ] && grep -Eq '^child_pid [1-9][0-9]*$' \"$2\"; then echo ready >\"$OWNER_HOLD_READY\"; while [ ! -e \"$OWNER_HOLD_RELEASE\" ]; do sleep 0.02; done; fi\nexec /usr/bin/mv \"$@\"\n",
    )
    .expect("mv");
    let work = job(&root, "echo ran >ran").expect("job");
    let wrapper = command(&root)
        .env("OWNER_HOLD_READY", root.join("owner-hold-ready"))
        .env("OWNER_HOLD_RELEASE", root.join("owner-hold-release"))
        .args(["--max-tokens", "7"])
        .arg(&work)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("wrapper");
    wait_for(&root.join("sdlc/live-tokens.lock/gate-ready"));
    wait_for(&root.join("owner-hold-ready"));
    let processes = Command::new("ps")
        .args(["-o", "pid=,args=", "--ppid", &wrapper.id().to_string()])
        .output()
        .expect("child processes");
    let listing = String::from_utf8_lossy(&processes.stdout);
    let gate_pid = listing
        .lines()
        .find(|line| line.contains("live-gate"))
        .and_then(|line| line.split_whitespace().next())
        .and_then(|pid| pid.parse::<u32>().ok())
        .expect("gate PID");
    let mv_pid = listing
        .lines()
        .find(|line| line.contains("fake-bin/mv"))
        .and_then(|line| line.split_whitespace().next())
        .and_then(|pid| pid.parse::<u32>().ok())
        .expect("mv PID");
    let environ = fs::read(format!("/proc/{gate_pid}/environ")).expect("gate environment");
    assert!(
        !environ
            .windows(KEY.len())
            .any(|bytes| bytes == KEY.as_bytes())
    );
    signal(wrapper.id(), "-KILL").expect("kill wrapper");
    signal(mv_pid, "-KILL").expect("kill held owner write");
    let lock = root.join("sdlc/live-tokens.lock");
    for name in ["gate-ready", "key", "owner.new", "owner", "stamp", "usage"] {
        let path = lock.join(name);
        if path.exists() {
            fs::remove_file(path).expect("remove recovery file");
        }
    }
    fs::remove_dir(&lock).expect("remove recovered lock");
    wait_gone(gate_pid);
    let _status = wrapper.wait_with_output().expect("wrapper output");
    assert!(!root.join("ran").exists());
}
