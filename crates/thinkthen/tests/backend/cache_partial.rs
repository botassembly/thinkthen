//! A cache keeps no reply that failed a question, and reads one as a miss.
//!
//! ADR 0053 item 6 and its amendment. `--record` alone and `--replay` alone
//! keep a partial reply as the live run gave it.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use conformance_backend::Backend;

use crate::harness::spawn;

/// A decide question, then a choice, so the arm below fails the choice alone.
const SET: &str = r#"{"version":1,"questions":{"ready":{"decide":"Is it ready?"},"kind":{"choose":"Which kind?","options":["bug","other"]}}}"#;
const ARM: &str = "/arm/malformed/missing_answer/v1";
const GENERIC: &str = "/generic/v1";
type Printed = (&'static str, Option<i32>);
type Ran = io::Result<()>;
const FAILED: Printed = (
    "{\"ready\":true,\"kind\":{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_answer\"}}}\n",
    Some(6),
);
const ANSWERED: Printed = ("{\"ready\":true,\"kind\":\"bug\"}\n", Some(0));
const REFUSED: Printed = ("", Some(4));
const RESPONSE: &str = "  \"response\": ";
const KEY: (&str, &str) = ("THINKTHEN_API_KEY", "sk-cache-partial");

/// An empty folder under the test's own scratch space.
fn scratch(name: &str) -> String {
    let path = format!("{}/cache-partial/{name}", env!("CARGO_TARGET_TMPDIR"));
    let _absent = fs::remove_dir_all(&path);
    path
}

/// One conformance backend, and a private cache home for every run against it.
struct Runs {
    backend: Backend,
    home: String,
    set: String,
}

impl Runs {
    fn new(name: &str) -> io::Result<Self> {
        let home = scratch(name);
        let set = format!("{home}.json");
        fs::create_dir_all(&home)?;
        fs::write(&set, SET)?;
        let backend = Backend::start()?;
        Ok(Self { backend, home, set })
    }

    fn default_cache(&self) -> String {
        format!("{}/thinkthen", self.home)
    }

    /// Run once and pin standard output, the exit code and every request so far.
    fn run(&self, arm: &str, verb: &[&str], folder: &[&str], printed: Printed, sent: usize) -> Ran {
        let url = format!("{}{arm}", self.backend.origin());
        let environment = [KEY, ("XDG_CACHE_HOME", self.home.as_str())];
        let arguments = [verb, &["--url", url.as_str()], folder].concat();
        let output = spawn(&arguments, &environment, b"hi")?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let ran = (stdout.as_ref(), output.status.code());
        assert_eq!(ran, printed, "{arguments:?}");
        assert_eq!(self.backend.count(), sent, "{arguments:?}");
        Ok(())
    }

    fn annotate(&self, arm: &str, folder: &[&str], printed: Printed, sent: usize) -> Ran {
        self.run(arm, &["annotate", &self.set], folder, printed, sent)
    }

    fn decide(&self, arm: &str, folder: &[&str], printed: Printed, sent: usize) -> Ran {
        self.run(arm, &["decide", "Is it?"], folder, printed, sent)
    }
}

/// The recording entries in a folder: its files named for a digest.
fn entries(folder: &str) -> Vec<PathBuf> {
    let listed = fs::read_dir(folder).into_iter().flatten().flatten();
    let digest = |path: &PathBuf| path.file_name().is_some_and(|name| name.len() == 69);
    listed.map(|entry| entry.path()).filter(digest).collect()
}

fn only_entry(folder: &str) -> io::Result<PathBuf> {
    let mut found = entries(folder);
    assert_eq!(found.len(), 1, "{folder}");
    found.pop().ok_or_else(|| io::Error::other("no entry"))
}

/// Replace an entry's response line, keep its request line, and give back the
/// old line and the new text.
fn set_response(entry: &Path, to: impl FnOnce(&str) -> String) -> io::Result<(String, String)> {
    let text = fs::read_to_string(entry)?;
    let old = text.lines().find(|kept| kept.starts_with(RESPONSE));
    let old = old.ok_or_else(|| io::Error::other("no response line"))?;
    let written = text.replacen(old, &to(old), 1);
    fs::write(entry, &written)?;
    Ok((old.to_owned(), written))
}

#[test]
fn a_cache_keeps_no_failed_question() -> io::Result<()> {
    let runs = Runs::new("keeps")?;
    for sent in [1, 2] {
        runs.annotate(ARM, &[], FAILED, sent)?;
        assert!(entries(&runs.default_cache()).is_empty(), "default cache");
    }
    let (cache, both, decided) = (scratch("cache"), scratch("both"), scratch("decided"));
    for sent in [3, 4] {
        runs.annotate(ARM, &["--cache", &cache], FAILED, sent)?;
        assert!(entries(&cache).is_empty(), "--cache keeps none");
    }
    for sent in [5, 6] {
        runs.annotate(ARM, &["--record", &both, "--replay", &both], FAILED, sent)?;
        assert!(entries(&both).is_empty(), "one folder for both keeps none");
    }
    let recorded = scratch("recorded");
    runs.annotate(ARM, &["--record", &recorded], FAILED, 7)?;
    runs.annotate(ARM, &["--replay", &recorded], FAILED, 7)?;
    for sent in [8, 9] {
        runs.decide(ARM, &["--cache", &decided], REFUSED, sent)?;
        assert!(entries(&decided).is_empty(), "every question failed");
    }
    let whole = Runs::new("whole")?;
    whole.annotate(GENERIC, &[], ANSWERED, 1)?;
    whole.annotate(GENERIC, &[], ANSWERED, 1)?;
    Ok(())
}

#[test]
fn a_cache_reads_a_partial_entry_as_a_miss() -> io::Result<()> {
    let runs = Runs::new("miss")?;
    let armed = scratch("armed");
    runs.annotate(ARM, &["--record", &armed], FAILED, 1)?;
    let entry = only_entry(&armed)?;
    let (partial, planted) = set_response(&entry, |old| old.replacen("0.9", "0.8", 1))?;
    for sent in [2, 3] {
        runs.annotate(ARM, &["--cache", &armed], FAILED, sent)?;
        assert_eq!(fs::read_to_string(&entry)?, planted, "the old entry stays");
    }
    runs.annotate(ARM, &["--replay", &armed], FAILED, 3)?;

    let fixed = scratch("fixed");
    runs.annotate(GENERIC, &["--record", &fixed], ANSWERED, 4)?;
    set_response(&only_entry(&fixed)?, |_| partial.clone())?;
    runs.annotate(GENERIC, &["--replay", &fixed], FAILED, 4)?;
    runs.annotate(GENERIC, &["--cache", &fixed], ANSWERED, 5)?;
    runs.annotate(GENERIC, &["--cache", &fixed], ANSWERED, 5)?;

    runs.annotate(GENERIC, &[], ANSWERED, 6)?;
    set_response(&only_entry(&runs.default_cache())?, |_| partial.clone())?;
    runs.annotate(GENERIC, &[], ANSWERED, 7)?;
    runs.annotate(GENERIC, &[], ANSWERED, 7)?;

    let broken = scratch("broken");
    runs.decide(GENERIC, &["--record", &broken], ("true\n", Some(0)), 8)?;
    let entry = only_entry(&broken)?;
    let answers = format!("{RESPONSE}{{\"model\":\"jev-latest\",\"answers\":{{}}}}");
    let (_, planted) = set_response(&entry, |_| answers)?;
    runs.decide(GENERIC, &["--cache", &broken], REFUSED, 8)?;
    assert_eq!(fs::read_to_string(&entry)?, planted, "the entry stays");
    Ok(())
}
