//! Deterministic signal checks at each FIFO handoff checkpoint.

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn executable(path: &Path, text: &str) -> io::Result<()> {
    fs::write(path, text)?;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

fn fixture(name: &str) -> io::Result<PathBuf> {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("sdlc/scripts"))?;
    fs::create_dir_all(root.join("fake-bin"))?;
    fs::create_dir_all(root.join("target/debug"))?;
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../sdlc/scripts/live"),
        root.join("sdlc/scripts/live"),
    )?;
    fs::write(
        root.join("sdlc/live-tokens"),
        "limit_tokens 100\nspent_tokens 10\n",
    )?;
    executable(&root.join("fake-bin/cargo"), "#!/bin/sh\nexit 0\n")?;
    executable(&root.join("fake-bin/sync"), "#!/bin/sh\nexit 0\n")?;
    executable(&root.join("job.sh"), "#!/bin/sh\necho ran >ran\n")?;
    Ok(root)
}

fn spawn(root: &Path) -> io::Result<Child> {
    Command::new("sh")
        .env_clear()
        .env(
            "PATH",
            format!("{}:/usr/bin:/bin", root.join("fake-bin").display()),
        )
        .env("THINKTHEN_API_KEY", "fifo-signal-key")
        .arg(root.join("sdlc/scripts/live"))
        .args(["--max-tokens", "7"])
        .arg(root.join("job.sh"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(Instant::now() < deadline, "{} missing", path.display());
        thread::sleep(Duration::from_millis(2));
    }
}

fn signal(pid: u32, name: &str) -> io::Result<()> {
    if Command::new("kill")
        .args([name, &pid.to_string()])
        .status()?
        .success()
    {
        Ok(())
    } else {
        Err(io::Error::other("signal command failed"))
    }
}

fn finish(mut child: Child) -> io::Result<Output> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait()?.is_none() {
        if Instant::now() >= deadline {
            let _stopped = child.kill();
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "live wrapper hung after a catchable signal",
            ));
        }
        thread::sleep(Duration::from_millis(5));
    }
    child.wait_with_output()
}

fn assert_stopped(root: &Path, output: &Output) -> io::Result<()> {
    assert_eq!(output.status.code(), Some(143), "{output:?}");
    assert!(!root.join("ran").exists());
    assert!(!root.join("sdlc/live-tokens.lock").exists());
    assert_eq!(
        fs::read_to_string(root.join("sdlc/live-tokens"))?,
        "limit_tokens 100\nspent_tokens 17\n"
    );
    Ok(())
}

fn hold_second_sync(root: &Path) -> io::Result<()> {
    executable(
        &root.join("fake-bin/sync"),
        "#!/bin/sh\nroot=${0%/fake-bin/sync}\ncount=0; [ ! -f \"$root/sync-count\" ] || count=$(cat \"$root/sync-count\"); count=$((count + 1)); echo $count >\"$root/sync-count\"\nif [ $count -eq 2 ]; then echo ready >\"$root/sync-ready\"; while [ ! -e \"$root/sync-release\" ]; do sleep 0.02; done; fi\n",
    )
}

fn recorded_gate(root: &Path) -> io::Result<u32> {
    let owner = root.join("sdlc/live-tokens.lock/owner");
    loop {
        wait_for(&owner);
        let text = fs::read_to_string(&owner)?;
        if let Some(pid) = text
            .lines()
            .find_map(|line| line.strip_prefix("child_pid "))
            .filter(|pid| *pid != "0")
        {
            return pid
                .parse::<u32>()
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error));
        }
        thread::sleep(Duration::from_millis(2));
    }
}

#[test]
fn term_during_fifo_creation_and_reader_open_never_hangs() {
    for mode in ["before", "after"] {
        let root = fixture(&format!("live-fifo-{mode}")).expect("fixture");
        executable(
            &root.join("fake-bin/mkfifo"),
            "#!/bin/sh\nif [ \"$FIFO_MODE\" = after ]; then /usr/bin/mkfifo \"$@\" || exit; fi\necho ready >\"$FIFO_READY\"\nwhile [ ! -e \"$FIFO_RELEASE\" ]; do sleep 0.02; done\n[ \"$FIFO_MODE\" = after ] || exec /usr/bin/mkfifo \"$@\"\n",
        )
        .expect("mkfifo");
        // These variables are inherited only by the fake keyless mkfifo command.
        let child = Command::new("sh")
            .env_clear()
            .env(
                "PATH",
                format!("{}:/usr/bin:/bin", root.join("fake-bin").display()),
            )
            .env("THINKTHEN_API_KEY", "fifo-signal-key")
            .env("FIFO_MODE", mode)
            .env("FIFO_READY", root.join("fifo-ready"))
            .env("FIFO_RELEASE", root.join("fifo-release"))
            .arg(root.join("sdlc/scripts/live"))
            .args(["--max-tokens", "7"])
            .arg(root.join("job.sh"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("wrapper");
        wait_for(&root.join("fifo-ready"));
        signal(child.id(), "-TERM").expect("TERM wrapper");
        fs::write(root.join("fifo-release"), "go").expect("release mkfifo");
        let output = finish(child).expect("wrapper output");
        assert_stopped(&root, &output).expect("stopped state");
    }
}

#[test]
fn repeated_signal_while_waiting_keeps_lock_until_stopped_gate_exits() {
    let root = fixture("live-fifo-reader-stopped").expect("fixture");
    hold_second_sync(&root).expect("sync");
    let child = spawn(&root).expect("wrapper");
    wait_for(&root.join("sync-ready"));
    let gate_pid = recorded_gate(&root).expect("gate PID");
    signal(gate_pid, "-STOP").expect("stop gate before reader open");
    fs::write(root.join("sync-release"), "go").expect("release sync");
    wait_for(&root.join("sdlc/live-tokens.lock/deliver"));
    signal(child.id(), "-TERM").expect("TERM wrapper without a reader");
    thread::sleep(Duration::from_millis(50));
    signal(child.id(), "-HUP").expect("HUP waiting wrapper");
    thread::sleep(Duration::from_millis(50));
    assert!(root.join("sdlc/live-tokens.lock").exists());
    signal(child.id(), "-0").expect("wrapper remains alive");
    signal(gate_pid, "-0").expect("stopped gate remains alive");
    signal(gate_pid, "-CONT").expect("continue stopped gate");
    let output = finish(child).expect("wrapper output");
    assert_stopped(&root, &output).expect("stopped state");
}

#[test]
fn term_after_key_receipt_cleans_the_lock() {
    let root = fixture("live-fifo-key-received").expect("fixture");
    hold_second_sync(&root).expect("sync");
    let child = spawn(&root).expect("wrapper");
    wait_for(&root.join("sync-ready"));
    let gate_pid = recorded_gate(&root).expect("gate PID");
    signal(gate_pid, "-STOP").expect("stop gate before receipt");
    fs::write(root.join("sync-release"), "go").expect("release sync");
    wait_for(&root.join("sdlc/live-tokens.lock/deliver"));
    signal(child.id(), "-STOP").expect("stop wrapper before receipt");
    signal(gate_pid, "-CONT").expect("let gate receive key");
    wait_for(&root.join("sdlc/live-tokens.lock/key-received"));
    signal(child.id(), "-TERM").expect("TERM wrapper");
    signal(child.id(), "-CONT").expect("continue wrapper");
    let output = finish(child).expect("wrapper output");
    assert_stopped(&root, &output).expect("stopped state");
}
