//! A partial reply stores its good answers and never a failed one, so the
//! next run asks only the failed question, by ADR 0111 section 6.

use std::fs;
use std::io;
use std::path::Path;

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
/// A replay of the question whose live answer failed is a miss.
const MISSED: Printed = ("", Some(5));
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

/// The answers a folder's live store holds, or none when it has no store.
fn answers(folder: &str) -> io::Result<usize> {
    let folder = Path::new(folder);
    if !folder.join("thinkthen.sqlite").exists() {
        return Ok(0);
    }
    Ok(crate::support::stored(folder)?.len())
}

/// The arm drops the last answer of every request. The first run's request
/// asks both questions and stores the decide answer. The rerun asks the
/// choice alone, whose reply then holds no answer at all, so it is refused.
// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn a_cache_stores_the_good_answers_and_asks_only_the_failed_question_again() -> io::Result<()> {
    let runs = Runs::new("keeps")?;
    runs.annotate(ARM, &[], FAILED, 1)?;
    assert_eq!(answers(&runs.default_cache())?, 1, "default cache");
    runs.annotate(ARM, &[], REFUSED, 2)?;
    assert_eq!(answers(&runs.default_cache())?, 1, "default cache");
    let (cache, both, decided) = (scratch("cache"), scratch("both"), scratch("decided"));
    runs.annotate(ARM, &["--cache", &cache], FAILED, 3)?;
    runs.annotate(ARM, &["--cache", &cache], REFUSED, 4)?;
    assert_eq!(answers(&cache)?, 1, "--cache keeps the good answer");
    runs.annotate(ARM, &["--record", &both, "--replay", &both], FAILED, 5)?;
    runs.annotate(ARM, &["--record", &both, "--replay", &both], REFUSED, 6)?;
    assert_eq!(
        answers(&both)?,
        1,
        "one folder for both keeps the good answer"
    );
    let recorded = scratch("recorded");
    runs.annotate(ARM, &["--record", &recorded], FAILED, 7)?;
    assert_eq!(answers(&recorded)?, 1, "--record keeps the good answer");
    runs.annotate(ARM, &["--replay", &recorded], MISSED, 7)?;
    for sent in [8, 9] {
        runs.decide(ARM, &["--cache", &decided], REFUSED, sent)?;
        assert_eq!(answers(&decided)?, 0, "every question failed");
    }
    let whole = Runs::new("whole")?;
    whole.annotate(GENERIC, &[], ANSWERED, 1)?;
    whole.annotate(GENERIC, &[], ANSWERED, 1)?;
    Ok(())
}
