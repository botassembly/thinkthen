//! The one door for a paid call, at the two gates it refuses at.
//!
//! Nothing here reaches a network. Each case is a refusal, and a refusal
//! happens before the job runs, so the job is a script that would leave a file
//! behind and never does.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The repository the script lives in.
fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

/// A tree of this test's own, holding a copy of the script and its ledger.
///
/// The script reads the ledger beside itself and no variable moves it, so a
/// case that wants another spend copies the script rather than redirecting it.
/// Every case here is a refusal, which happens before the script builds
/// anything, so the copy needs nothing else of the repository.
fn folder(name: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(path.join("sdlc").join("scripts"))?;
    fs::copy(
        repo().join("sdlc").join("scripts").join("live"),
        script(&path),
    )?;
    Ok(path)
}

/// The copy of the live script this tree holds.
fn script(folder: &Path) -> PathBuf {
    folder.join("sdlc").join("scripts").join("live")
}

/// Write the ledger the copied script reads, holding this limit and this spend.
fn ledger(folder: &Path, limit: u64, spent: u64) -> io::Result<PathBuf> {
    let path = folder.join("sdlc").join("live-tokens");
    fs::write(
        &path,
        format!("limit_tokens {limit}\nspent_tokens {spent}\n"),
    )?;
    Ok(path)
}

/// Write a job that leaves a file behind, so a run that happened is visible.
fn job(folder: &Path) -> io::Result<(PathBuf, PathBuf)> {
    let ran = folder.join("ran");
    let path = folder.join("job.sh");
    fs::write(&path, format!("#!/bin/sh\necho ran > {}\n", ran.display()))?;
    Ok((path, ran))
}

/// Run this tree's copy of the live script over one job, with no environment
/// but what the case names.
fn live(folder: &Path, job: &Path, key: Option<&str>) -> io::Result<Output> {
    let mut command = Command::new("sh");
    command
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .arg(script(folder))
        .arg(job);
    if let Some(key) = key {
        command.env("THINKTHEN_API_KEY", key);
    }
    command.output()
}

#[test]
fn a_ledger_at_its_limit_refuses_the_job_and_leaves_the_spend_alone() {
    let folder = folder("live-at-the-limit").expect("a folder for this case");
    let (job, ran) = job(&folder).expect("a job to run");

    for (limit, spent) in [(476_000_000_u64, 476_000_000_u64), (1_000, 1_185)] {
        let file = ledger(&folder, limit, spent).expect("a ledger");

        let output = live(&folder, &job, Some("sk-not-read")).expect("the script runs");

        assert_eq!(output.status.code(), Some(1), "{spent} of {limit}");
        assert!(!ran.exists(), "the job ran at the limit");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains(&spent.to_string()), "{message}");
        assert!(message.contains(&limit.to_string()), "{message}");
        assert_eq!(
            fs::read_to_string(&file).expect("the ledger is text"),
            format!("limit_tokens {limit}\nspent_tokens {spent}\n"),
            "a refusal changes no spend"
        );
    }
}

#[test]
fn a_key_that_holds_nothing_refuses_the_job_and_never_shows_a_value() {
    let folder = folder("live-with-no-key").expect("a folder for this case");
    let (job, ran) = job(&folder).expect("a job to run");
    ledger(&folder, 476_000_000, 1_185).expect("a ledger");

    for key in [None, Some(""), Some("   ")] {
        let output = live(&folder, &job, key).expect("the script runs");

        assert_eq!(output.status.code(), Some(1), "{key:?}");
        assert!(!ran.exists(), "the job ran with no key");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("THINKTHEN_API_KEY"), "{message}");
        assert!(output.stdout.is_empty(), "{key:?}");
    }
}

#[test]
fn a_job_the_repository_does_not_hold_is_refused_before_anything_else() {
    let folder = folder("live-with-no-job").expect("a folder for this case");
    ledger(&folder, 476_000_000, 1_185).expect("a ledger");

    let output = live(&folder, &folder.join("absent.sh"), Some("sk-not-read")).expect("it runs");

    assert_eq!(output.status.code(), Some(2));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("absent.sh"), "{message}");
}
