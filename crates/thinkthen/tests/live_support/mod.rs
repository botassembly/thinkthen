use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) struct SessionCleanup {
    main: PathBuf,
}

pub(crate) struct ProcessCleanup {
    pid: u32,
}

impl ProcessCleanup {
    pub(crate) fn new(pid: u32) -> Self {
        Self { pid }
    }
}

impl Drop for ProcessCleanup {
    fn drop(&mut self) {
        let _status = Command::new("/bin/kill")
            .args(["-KILL", &self.pid.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
}

impl SessionCleanup {
    pub(crate) fn new(main: &Path) -> Self {
        Self {
            main: main.to_owned(),
        }
    }
}

impl Drop for SessionCleanup {
    fn drop(&mut self) {
        let Ok(state) = fs::read_to_string(self.main.join(".git/thinkthen-live/state.json")) else {
            return;
        };
        if state.contains("\"pending\":null") {
            return;
        }
        let session = json_number(&state, "sid");
        let _status = Command::new("/bin/kill")
            .args(["-KILL", "--", &format!("-{session}")])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        let Ok(entries) = fs::read_dir("/proc") else {
            return;
        };
        for entry in entries.flatten() {
            let Ok(pid) = entry.file_name().to_string_lossy().parse::<i64>() else {
                continue;
            };
            let Ok(stat) = fs::read_to_string(entry.path().join("stat")) else {
                continue;
            };
            let Some(close) = stat.rfind(')') else {
                continue;
            };
            let fields: Vec<_> = stat[close + 1..].split_whitespace().collect();
            if fields.get(3).and_then(|field| field.parse::<i64>().ok()) == Some(session) {
                let _status = Command::new("/bin/kill")
                    .args(["-KILL", &pid.to_string()])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
            }
        }
    }
}

pub(crate) fn executable(path: &Path, text: &str) {
    fs::write(path, text).expect("write executable");
    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("chmod");
}

pub(crate) fn run(directory: &Path, arguments: &[&str]) {
    let output = Command::new("/usr/bin/git")
        .arg("-C")
        .arg(directory)
        .args(arguments)
        .output()
        .expect("git");
    assert!(output.status.success(), "git failed: {output:?}");
}

pub(crate) fn temporary_repository(name: &str) -> (PathBuf, PathBuf) {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&root);
    let main = root.join("main");
    let linked = root.join("linked");
    fs::create_dir_all(main.join("sdlc/scripts")).expect("scripts");
    fs::create_dir_all(main.join("target/debug")).expect("target");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../sdlc/scripts/live"),
        main.join("sdlc/scripts/live"),
    )
    .expect("copy live");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../sdlc/scripts/live-state.py"),
        main.join("sdlc/scripts/live-state.py"),
    )
    .expect("copy live state module");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../sdlc/scripts/live-migrate"),
        main.join("sdlc/scripts/live-migrate"),
    )
    .expect("copy migration tool");
    fs::write(
        main.join("sdlc/live-tokens"),
        "checkpoint_limit_tokens 10\ncheckpoint_charged_tokens 0\ncheckpoint_authority_id not-activated\n",
    )
    .expect("checkpoint");
    executable(&main.join("target/debug/thinkthen"), "#!/bin/sh\nexit 0\n");
    executable(
        &main.join("hold.sh"),
        "#!/bin/sh\necho ready >ready\nwhile [ ! -e release ]; do sleep 0.02; done\n",
    );
    executable(&main.join("quick.sh"), "#!/bin/sh\necho \"$@\" >quick\n");
    run(&main, &["init", "-q"]);
    run(&main, &["config", "user.email", "test@example.invalid"]);
    run(&main, &["config", "user.name", "Test"]);
    run(&main, &["add", "."]);
    run(&main, &["commit", "-qm", "fixture"]);
    run(
        &main,
        &[
            "worktree",
            "add",
            "-q",
            linked.to_str().expect("path"),
            "-b",
            "linked",
        ],
    );
    fs::create_dir_all(linked.join("target/debug")).expect("linked target");
    executable(
        &linked.join("target/debug/thinkthen"),
        "#!/bin/sh\nexit 0\n",
    );
    let activated = Command::new(main.join("sdlc/scripts/live-migrate"))
        .current_dir(&main)
        .args(["--activate", "10", "0"])
        .output()
        .expect("activate authority");
    assert!(activated.status.success(), "{activated:?}");
    (main, linked)
}

pub(crate) fn live(tree: &Path) -> Command {
    let mut command = Command::new(tree.join("sdlc/scripts/live"));
    command
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("THINKTHEN_API_KEY", "test-key");
    command
}

pub(crate) fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub(crate) fn status(tree: &Path) -> String {
    let output = Command::new(tree.join("sdlc/scripts/live"))
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .arg("--status")
        .output()
        .expect("status");
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout).expect("UTF-8 status")
}

pub(crate) fn signal(pid: u32, name: &str) {
    assert!(
        Command::new("/bin/kill")
            .args([name, &pid.to_string()])
            .status()
            .expect("signal")
            .success()
    );
}

pub(crate) fn authority_state(main: &Path) -> String {
    fs::read_to_string(main.join(".git/thinkthen-live/state.json")).expect("state")
}

pub(crate) fn json_number(text: &str, name: &str) -> i64 {
    let start = text
        .find(&format!("\"{name}\":"))
        .map(|offset| offset + name.len() + 3)
        .expect("number field");
    text[start..]
        .split(|character: char| !character.is_ascii_digit())
        .next()
        .expect("number")
        .parse()
        .expect("integer")
}

pub(crate) fn recover(main: &Path) -> Output {
    Command::new(main.join("sdlc/scripts/live"))
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .arg("--recover")
        .output()
        .expect("recover")
}

pub(crate) fn migrate(main: &Path, arguments: &[&str]) -> Output {
    Command::new(main.join("sdlc/scripts/live-migrate"))
        .current_dir(main)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .args(arguments)
        .output()
        .expect("migration command")
}

pub(crate) fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(Instant::now() < deadline, "{} missing", path.display());
        thread::sleep(Duration::from_millis(10));
    }
}

pub(crate) fn inspect_and_release_git_helper(hooks: &Path, key: &[u8]) {
    wait_for(&hooks.join("git-helper.pid"));
    let helper = fs::read_to_string(hooks.join("git-helper.pid")).expect("helper PID");
    let environment =
        fs::read(format!("/proc/{}/environ", helper.trim())).expect("helper environment");
    assert!(!environment.windows(key.len()).any(|part| part == key));
    fs::write(hooks.join("git-helper.release"), b"go\n").expect("release helper");
}

pub(crate) fn recover_until_clear(main: &Path, context: &str) {
    if context == "after-replace" {
        let state_path = main.join(".git/thinkthen-live/state.json");
        let before = authority_state(main);
        let boot_start = before.find("\"boot_id\":\"").expect("boot ID") + 11;
        let boot_end = before[boot_start..].find('"').expect("boot end") + boot_start;
        let changed = before.replace(
            &before[boot_start..boot_end],
            "00000000-0000-0000-0000-000000000000",
        );
        fs::write(state_path, changed).expect("changed boot state");
        assert!(recover(main).status.success());
        return;
    }
    if context == "after-key-receipt" {
        let before = authority_state(main);
        let pid = json_number(&before, "pid").to_string();
        let unreadable = Command::new(main.join("sdlc/scripts/live"))
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("THINKTHEN_LIVE_TEST_UNREADABLE_PID", pid)
            .arg("--recover")
            .output()
            .expect("unreadable identity recovery");
        assert_eq!(unreadable.status.code(), Some(1));
        assert_eq!(authority_state(main), before);
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let recovered = recover(main);
        if recovered.status.success() {
            return;
        }
        assert!(Instant::now() < deadline, "{context}: {recovered:?}");
        thread::sleep(Duration::from_millis(20));
    }
}
