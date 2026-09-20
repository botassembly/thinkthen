//! Black-box tests for the paid-call door. All jobs and build commands are local.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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

fn folder(name: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(path.join("sdlc/scripts"))?;
    fs::create_dir_all(path.join("fake-bin"))?;
    fs::create_dir_all(path.join("target/debug"))?;
    fs::copy(repo().join("sdlc/scripts/live"), script(&path))?;
    executable(
        &path.join("fake-bin/cargo"),
        "#!/bin/sh\nprintf 'build\\n' >>\"$FAKE_BUILD_LOG\"\nexit \"${FAKE_BUILD_STATUS:-0}\"\n",
    )?;
    Ok(path)
}

fn script(folder: &Path) -> PathBuf {
    folder.join("sdlc/scripts/live")
}

fn ledger_text(folder: &Path, text: &str) -> io::Result<PathBuf> {
    let path = folder.join("sdlc/live-tokens");
    fs::write(&path, text)?;
    Ok(path)
}

fn ledger(folder: &Path, limit: u64, spent: u64) -> io::Result<PathBuf> {
    ledger_text(
        folder,
        &format!("# kept comment\nlimit_tokens {limit}\nspent_tokens {spent}\n"),
    )
}

fn job(folder: &Path, name: &str, body: &str) -> io::Result<PathBuf> {
    let path = folder.join(name);
    executable(
        &path,
        &format!("#!/bin/sh\nset -eu\ncd -- \"$(dirname -- \"$0\")\"\n{body}\n"),
    )?;
    Ok(path)
}

fn command(folder: &Path) -> Command {
    let mut command = Command::new("sh");
    command
        .env_clear()
        .env(
            "PATH",
            format!("{}:/usr/bin:/bin", folder.join("fake-bin").display()),
        )
        .env("FAKE_BUILD_LOG", folder.join("builds"))
        .env("THINKTHEN_API_KEY", "sk-not-read")
        .arg(script(folder));
    command
}

fn live(folder: &Path, reservation: &str, job: &Path) -> io::Result<Output> {
    command(folder)
        .args(["--max-tokens", reservation])
        .arg(job)
        .output()
}

fn spent(folder: &Path) -> io::Result<String> {
    fs::read_to_string(folder.join("sdlc/live-tokens"))?
        .lines()
        .find_map(|line| line.strip_prefix("spent_tokens "))
        .map(str::to_owned)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing spent row"))
}

fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "{} was never written",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn finish(child: Child) -> io::Result<Output> {
    child.wait_with_output()
}

#[test]
fn reservations_are_strict_canonical_positive_decimals() {
    let cases = [
        ("missing", vec![]),
        ("zero", vec!["--max-tokens", "0", "job.sh"]),
        ("leading", vec!["--max-tokens", "01", "job.sh"]),
        ("signed", vec!["--max-tokens", "+1", "job.sh"]),
        ("point", vec!["--max-tokens", "1.0", "job.sh"]),
        ("large", vec!["--max-tokens", "1000000000", "job.sh"]),
        (
            "long",
            vec!["--max-tokens", "999999999999999999999999999999", "job.sh"],
        ),
        ("old", vec!["job.sh"]),
    ];
    for (case, args) in cases {
        let folder = folder(&format!("live-reservation-{case}")).expect("folder");
        ledger(&folder, 9, 0).expect("ledger");
        job(&folder, "job.sh", "echo ran >ran").expect("job");
        let output = command(&folder).args(args).output().expect("run");
        assert_eq!(output.status.code(), Some(2), "{case}");
        assert!(!folder.join("builds").exists(), "{case} built");
        assert!(!folder.join("ran").exists(), "{case} ran");
    }
}

#[test]
fn invalid_ledgers_are_refused_at_every_numeric_edge() {
    let cases = [
        ("missing-limit", "spent_tokens 0\n"),
        ("missing-spend", "limit_tokens 1\n"),
        (
            "duplicate",
            "limit_tokens 1\nlimit_tokens 1\nspent_tokens 0\n",
        ),
        ("unknown", "limit_tokens 1\nspent_tokens 0\nother 1\n"),
        ("zero-limit", "limit_tokens 0\nspent_tokens 0\n"),
        ("large-limit", "limit_tokens 1000000000\nspent_tokens 0\n"),
        ("leading-limit", "limit_tokens 01\nspent_tokens 0\n"),
        ("leading-spend", "limit_tokens 9\nspent_tokens 00\n"),
        (
            "large-spend",
            "limit_tokens 999999999\nspent_tokens 1000000000\n",
        ),
        ("over-limit", "limit_tokens 8\nspent_tokens 9\n"),
    ];
    for (case, contents) in cases {
        let folder = folder(&format!("live-ledger-{case}")).expect("folder");
        let file = ledger_text(&folder, contents).expect("ledger");
        let job = job(&folder, "job.sh", "echo ran >ran").expect("job");
        let output = live(&folder, "1", &job).expect("run");
        assert_eq!(output.status.code(), Some(2), "{case}");
        assert_eq!(fs::read_to_string(file).expect("text"), contents);
        assert!(!folder.join("builds").exists(), "{case} built");
        assert!(!folder.join("ran").exists(), "{case} ran");
    }
}

#[test]
fn exact_fit_precharges_and_a_larger_reservation_does_nothing() {
    let folder = folder("live-exact-fit").expect("folder");
    ledger(&folder, 10, 9).expect("ledger");
    let job = job(&folder, "job.sh", "echo ran >ran").expect("job");
    let refused = live(&folder, "2", &job).expect("refusal");
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(spent(&folder).expect("spent"), "9");
    assert!(!folder.join("builds").exists());
    let accepted = live(&folder, "1", &job).expect("accepted");
    assert_eq!(accepted.status.code(), Some(0), "{accepted:?}");
    assert_eq!(spent(&folder).expect("spent"), "10");
    assert!(folder.join("ran").exists());
    assert!(!folder.join("sdlc/live-tokens.lock").exists());
}

#[test]
fn measured_files_never_refund_a_reservation() {
    let cases = [
        ("absent", "echo ran >ran"),
        ("repeated", "echo ran >ran"),
        (
            "below",
            "printf '%s\\n' '{\"usage\":{\"input_tokens\":2}}' >usage.json",
        ),
        ("deleted", "printf x >usage.json; rm usage.json"),
        (
            "backdated",
            "printf x >usage.json; touch -t 200001010000 usage.json",
        ),
    ];
    for (case, body) in cases {
        let folder = folder(&format!("live-scan-{case}")).expect("folder");
        ledger(&folder, 100, 10).expect("ledger");
        if case == "repeated" {
            fs::write(
                folder.join("usage.json"),
                "{\"usage\":{\"input_tokens\":99}}\n",
            )
            .expect("old usage file");
        }
        let job = job(&folder, "job.sh", body).expect("job");
        let output = live(&folder, "7", &job).expect("run");
        assert_eq!(output.status.code(), Some(0), "{case}: {output:?}");
        assert_eq!(spent(&folder).expect("spent"), "17", "{case}");
    }
}

#[test]
fn status_precedence_keeps_failure_and_flags_successful_overuse() {
    for (case, job_status, expected) in [("success", 0, 1), ("failure", 7, 7)] {
        let folder = folder(&format!("live-overuse-{case}")).expect("folder");
        ledger(&folder, 100, 10).expect("ledger");
        let body = format!(
            "printf '%s\\n' '{{\"usage\":{{\"input_tokens\":8}}}}' >usage.json; exit {job_status}"
        );
        let job = job(&folder, "job.sh", &body).expect("job");
        let output = live(&folder, "7", &job).expect("run");
        assert_eq!(output.status.code(), Some(expected), "{case}");
        assert_eq!(spent(&folder).expect("spent"), "17");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("reservation 7"), "{message}");
        assert!(message.contains("measured 8"), "{message}");
    }
}

#[test]
fn failed_job_keeps_its_full_charge() {
    let folder = folder("live-failed-job").expect("folder");
    ledger(&folder, 100, 10).expect("ledger");
    let job = job(&folder, "job.sh", "exit 6").expect("job");
    let output = live(&folder, "7", &job).expect("run");
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(spent(&folder).expect("spent"), "17");
}

#[test]
fn missing_job_and_blank_keys_stop_before_build() {
    for (case, key, path) in [
        ("missing-job", Some("key"), "absent.sh"),
        ("missing-key", None, "job.sh"),
        ("empty-key", Some(""), "job.sh"),
        ("space-key", Some(" \t"), "job.sh"),
    ] {
        let folder = folder(&format!("live-input-{case}")).expect("folder");
        ledger(&folder, 100, 10).expect("ledger");
        job(&folder, "job.sh", "echo ran >ran").expect("job");
        let mut run = command(&folder);
        run.args(["--max-tokens", "7"]).arg(folder.join(path));
        match key {
            Some(value) => {
                run.env("THINKTHEN_API_KEY", value);
            }
            None => {
                run.env_remove("THINKTHEN_API_KEY");
            }
        }
        let output = run.output().expect("run");
        assert_ne!(output.status.code(), Some(0), "{case}");
        assert!(!folder.join("builds").exists(), "{case} built");
        assert!(!folder.join("ran").exists(), "{case} ran");
        assert_eq!(spent(&folder).expect("spent"), "10", "{case}");
    }
}

#[test]
fn scan_failure_is_informational_but_obeys_job_status_precedence() {
    for (tool, fake) in [
        (
            "find",
            "#!/bin/sh\nfor arg do [ \"$arg\" = -newer ] && exit 9; done\nexec /usr/bin/find \"$@\"\n",
        ),
        ("sed", "#!/bin/sh\nexit 9\n"),
    ] {
        for (case, job_status, expected) in [("success", 0, 1), ("failure", 12, 12)] {
            let folder = folder(&format!("live-{tool}-failure-{case}")).expect("folder");
            ledger(&folder, 100, 10).expect("ledger");
            executable(&folder.join("fake-bin").join(tool), fake).expect("fake scan tool");
            let job = job(
                &folder,
                "job.sh",
                &format!("printf x >new.json; exit {job_status}"),
            )
            .expect("job");
            let output = live(&folder, "7", &job).expect("run");
            assert_eq!(
                output.status.code(),
                Some(expected),
                "{tool} {case}: {output:?}"
            );
            assert_eq!(spent(&folder).expect("spent"), "17");
            assert!(!folder.join("sdlc/live-tokens.lock").exists());
        }
    }
}

#[test]
fn cleanup_keeps_owner_when_an_unexpected_lock_entry_exists() {
    let folder = folder("live-safe-cleanup").expect("folder");
    ledger(&folder, 100, 10).expect("ledger");
    let job = job(&folder, "job.sh", ": >sdlc/live-tokens.lock/unexpected").expect("job");
    let output = live(&folder, "7", &job).expect("run");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert_eq!(spent(&folder).expect("spent"), "17");
    assert!(folder.join("sdlc/live-tokens.lock/owner").exists());
    assert!(folder.join("sdlc/live-tokens.lock/unexpected").exists());
}

#[test]
fn one_wrapper_holds_authority_until_its_child_finishes() {
    let folder = folder("live-two-processes").expect("folder");
    ledger(&folder, 15, 0).expect("ledger");
    let first_job = job(
        &folder,
        "first job\nwith newline.sh",
        "echo started >started; while [ ! -f release ]; do sleep 0.02; done",
    )
    .expect("job");
    let second_job = job(&folder, "second.sh", "echo second >second").expect("job");
    let first = command(&folder)
        .args(["--max-tokens", "10"])
        .arg(&first_job)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("first wrapper");
    wait_for(&folder.join("started"));
    let owner = fs::read_to_string(folder.join("sdlc/live-tokens.lock/owner")).expect("owner");
    assert!(owner.contains("state charged\n"), "{owner}");
    assert!(
        owner
            .lines()
            .any(|line| line.starts_with("child_pid ") && line != "child_pid 0"),
        "{owner}"
    );
    assert!(owner.contains("job_path_hex "), "{owner}");
    assert!(!owner.contains("with newline"), "{owner}");
    let second = live(&folder, "5", &second_job).expect("second");
    assert_eq!(second.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&second.stderr),
        "live: the live spend lock is held, so no call goes out\n"
    );
    assert!(!folder.join("second").exists());
    assert_eq!(
        fs::read_to_string(folder.join("builds")).expect("builds"),
        "build\n"
    );
    fs::write(folder.join("release"), "go\n").expect("release");
    let output = finish(first).expect("wrapper output");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(spent(&folder).expect("spent"), "10");
    assert!(!folder.join("sdlc/live-tokens.lock").exists());
}

#[test]
fn term_is_forwarded_and_wrapper_waits_before_cleanup() {
    let folder = folder("live-term").expect("folder");
    ledger(&folder, 100, 2).expect("ledger");
    let job = job(
        &folder,
        "job.sh",
        "trap 'echo term >received; exit 0' TERM; echo ready >ready; while :; do sleep 1; done",
    )
    .expect("job");
    let wrapper = command(&folder)
        .args(["--max-tokens", "9"])
        .arg(&job)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("wrapper");
    wait_for(&folder.join("ready"));
    assert!(
        Command::new("kill")
            .args(["-TERM", &wrapper.id().to_string()])
            .status()
            .expect("TERM")
            .success()
    );
    let output = finish(wrapper).expect("wrapper output");
    assert_eq!(output.status.code(), Some(143), "{output:?}");
    assert_eq!(spent(&folder).expect("spent"), "11");
    assert_eq!(
        fs::read_to_string(folder.join("received")).expect("receipt"),
        "term\n"
    );
    assert!(!folder.join("sdlc/live-tokens.lock").exists());
}

#[test]
fn stale_and_uncertain_locks_fail_closed_before_build() {
    for (case, state, ledger_spent, charged) in [
        ("charged", "charged", 17, 17),
        ("lower", "charged", 16, 17),
        ("uncertain", "charging", 10, 17),
    ] {
        let folder = folder(&format!("live-stale-{case}")).expect("folder");
        ledger(&folder, 100, ledger_spent).expect("ledger");
        let lock = folder.join("sdlc/live-tokens.lock");
        fs::create_dir(&lock).expect("lock");
        fs::write(
            lock.join("owner"),
            format!("pid 123\nchild_pid 0\njob_path_hex 2f78\nreservation 7\nprior_spend 10\ncharged_total {charged}\nstate {state}\n"),
        )
        .expect("owner");
        let job = job(&folder, "job.sh", "echo ran >ran").expect("job");
        let output = live(&folder, "1", &job).expect("run");
        assert_eq!(output.status.code(), Some(1), "{case}");
        assert!(lock.exists(), "{case}");
        assert!(!folder.join("builds").exists(), "{case} built");
    }
}

#[test]
fn key_build_and_precharge_failures_never_start_job() {
    for case in ["key", "build", "precharge"] {
        let folder = folder(&format!("live-failure-{case}")).expect("folder");
        let file = ledger(&folder, 100, 10).expect("ledger");
        let job = job(&folder, "job.sh", "echo ran >ran").expect("job");
        let mut run = command(&folder);
        run.args(["--max-tokens", "7"]).arg(&job);
        match case {
            "key" => {
                run.env_remove("THINKTHEN_API_KEY");
            }
            "build" => {
                run.env("FAKE_BUILD_STATUS", "8");
            }
            "precharge" => executable(
                &folder.join("fake-bin/mv"),
                "#!/bin/sh\ncase \"$3\" in */live-tokens) exit 9;; esac\nexec /bin/mv \"$@\"\n",
            )
            .expect("fake mv"),
            _ => unreachable!(),
        }
        let output = run.output().expect("run");
        assert_ne!(output.status.code(), Some(0), "{case}");
        assert!(!folder.join("ran").exists(), "{case} ran");
        assert_eq!(
            fs::read_to_string(&file).expect("ledger"),
            "# kept comment\nlimit_tokens 100\nspent_tokens 10\n"
        );
        assert_eq!(
            folder.join("sdlc/live-tokens.lock").exists(),
            case == "precharge"
        );
        if case == "precharge" {
            let entries = fs::read_dir(folder.join("sdlc/live-tokens.lock"))
                .expect("lock contents")
                .map(|entry| entry.expect("entry").file_name())
                .collect::<Vec<_>>();
            assert_eq!(entries, ["owner"]);
            assert!(
                fs::read_to_string(folder.join("sdlc/live-tokens.lock/owner"))
                    .expect("owner")
                    .contains("state charging\n")
            );
        }
    }
}

#[test]
fn exact_maximum_ledger_and_reservation_are_accepted() {
    let folder = folder("live-maximum-edge").expect("folder");
    ledger(&folder, 999_999_999, 0).expect("ledger");
    let job = job(&folder, "job.sh", "echo ran >ran").expect("job");
    let output = live(&folder, "999999999", &job).expect("run");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(spent(&folder).expect("spent"), "999999999");
    assert!(!repo().join("ran").exists());
    assert!(!repo().join("yes").exists());
}
