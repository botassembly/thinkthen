//! Judgment questions accept plain text, JSON, or a file named `'@path'`.
//! Sets and specs accept JSON or `@path`. Named files use the privilege gate,
//! 1 MiB cap, regular-file rule, and `thinkthen.file_directory` confinement.

use std::path::{Component, Path, PathBuf};

use thinkthen::{Error, ErrorKind, For, Settings};

use crate::call;
use crate::ffi;

/// The largest file a named argument reads, in bytes.
pub(crate) const FILE_CAP: u64 = 1024 * 1024;

/// What an argument names: JSON text, or the path after `@`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ArgForm<'a> {
    File(&'a str),
    Json(&'a str),
}

/// Bare text that is not JSON never names a file (R2-4).
pub(crate) fn arg_form<'a>(text: &'a str, what: &str) -> Result<ArgForm<'a>, Error> {
    if let Some(path) = text.strip_prefix('@') {
        return Ok(ArgForm::File(path));
    }
    if text.trim_start().starts_with('{') {
        return Ok(ArgForm::Json(text));
    }
    Err(call::usage(format!(
        "a {what} file is named with the @ spelling: '@{text}'; bare text is never a path"
    )))
}

/// Every unreadable cause is one class, so a caller learns nothing about
/// the filesystem. `OverCap` names the cap for a file the caller may read.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CheckedReadError {
    Refused,
    OverCap,
}

/// The part of `path` inside the directory, judged by spelling alone.
fn beneath(path: &Path, base: &Path, real_base: &Path) -> Option<PathBuf> {
    let absolute = |spelled: &Path| -> Option<PathBuf> {
        if spelled.is_absolute() {
            Some(spelled.to_path_buf())
        } else {
            Some(std::env::current_dir().ok()?.join(spelled))
        }
    };
    let path = absolute(path)?;
    if path.components().any(|part| part == Component::ParentDir) {
        return None;
    }
    let rel = path
        .strip_prefix(absolute(base)?)
        .or_else(|_| path.strip_prefix(real_base))
        .ok()?;
    (!rel.as_os_str().is_empty()).then(|| rel.to_path_buf())
}

/// Read `path` as a regular file of at most `cap` bytes, opened once and
/// judged by its descriptor. A confined read refuses an outside spelling
/// before any open, opens beneath the base, and takes a file with one link
/// whose descriptor path sits inside the base, before it reads the size.
pub(crate) fn read_within(
    path: &Path,
    cap: u64,
    confined: Option<&Path>,
) -> Result<String, CheckedReadError> {
    read_checked(path, cap, confined, None)
}

fn read_checked(
    path: &Path,
    cap: u64,
    confined: Option<&Path>,
    named_root: Option<&Path>,
) -> Result<String, CheckedReadError> {
    use std::io::Read;
    use std::os::unix::fs::MetadataExt;
    let real_base = match confined {
        None => None,
        Some(base) => Some(std::fs::canonicalize(base).map_err(|_| CheckedReadError::Refused)?),
    };
    let file = match (confined, &real_base) {
        (Some(base), Some(real_base)) => {
            let rel = beneath(path, base, real_base).ok_or(CheckedReadError::Refused)?;
            ffi::open_beneath(real_base, &rel)
        }
        _ => ffi::open_plain(path),
    }
    .map_err(|_| CheckedReadError::Refused)?;
    let meta = file.metadata().map_err(|_| CheckedReadError::Refused)?;
    if !meta.is_file() {
        return Err(CheckedReadError::Refused);
    }
    if let Some(real_base) = &real_base {
        let real = ffi::path_of(&file).ok_or(CheckedReadError::Refused)?;
        if meta.nlink() != 1 || !real.starts_with(real_base) {
            return Err(CheckedReadError::Refused);
        }
    }
    if let Some(root) = named_root {
        let real = ffi::path_of(&file).ok_or(CheckedReadError::Refused)?;
        if !real.starts_with(root) {
            return Err(CheckedReadError::Refused);
        }
    }
    if meta.len() > cap {
        return Err(CheckedReadError::OverCap);
    }
    let mut text = String::new();
    // One byte past the cap tells a file that grew after the check.
    file.take(cap + 1)
        .read_to_string(&mut text)
        .map_err(|_| CheckedReadError::Refused)?;
    if text.len() as u64 > cap {
        return Err(CheckedReadError::OverCap);
    }
    Ok(text)
}

/// The file a named path means. A relative name sits inside a set
/// `thinkthen.file_directory`; with no folder it keeps the backend's working
/// folder. The confined read still judges the joined path.
fn resolve(path: &str, directory: Option<&str>) -> PathBuf {
    match directory {
        Some(held) if Path::new(path).is_relative() => Path::new(held).join(path),
        _ => PathBuf::from(path),
    }
}

/// Read a file a caller named with `@`. A role without
/// `pg_read_server_files` reads only inside `thinkthen.file_directory`.
pub(crate) fn read_named(what: &str, path: &str, directory: Option<&str>) -> Result<String, Error> {
    let directory = directory.filter(|held| !held.trim().is_empty());
    let confined = authorize(directory)?;
    read_within(&resolve(path, directory), FILE_CAP, confined.as_deref()).map_err(|error| match error {
        CheckedReadError::Refused => Error::new(
            ErrorKind::Local,
            format!(
                "the {what} file '@{path}' did not read: it must be a regular file at most {FILE_CAP} bytes, \
                 and inside thinkthen.file_directory with one link when one is set"
            ),
        ),
        CheckedReadError::OverCap => {
            Error::new(ErrorKind::Local, format!("the {what} file '@{path}' is over the {FILE_CAP} byte cap"))
        }
    })
}

/// The same privilege decision precedes native metadata selection and content opening.
fn authorize(directory: Option<&str>) -> Result<Option<PathBuf>, Error> {
    let confined = if ffi::may_read_files() {
        None
    } else {
        match directory {
            Some(held) => Some(PathBuf::from(held)),
            None => {
                return Err(call::usage(
                    "a named file needs pg_read_server_files, or an administrator's thinkthen.file_directory",
                ));
            }
        }
    };
    Ok(confined)
}

pub(crate) fn read_resolved_question(
    source: &str,
    directory: Option<&str>,
) -> Result<(thinkthen::QuestionFileReference, String), Error> {
    let directory = directory.filter(|held| !held.trim().is_empty());
    let confined = authorize(directory)?;
    let reference = if let Some(name) = source.strip_prefix("@@") {
        thinkthen::QuestionFileReference::named(name)?
    } else if let Some(directory) = directory {
        thinkthen::QuestionFileReference::reference_in(source, Path::new(directory))?
    } else {
        thinkthen::QuestionFileReference::reference(source)?
    };
    let text = read_checked(reference.path(), FILE_CAP, confined.as_deref(), reference.named_root())
        .map_err(|error| Error::new(ErrorKind::Local, match error {
            CheckedReadError::Refused => "the question file was not read: the descriptor is outside its admitted roots or is not a permitted regular file",
            CheckedReadError::OverCap => "the question file is over the 1048576 byte cap",
        }))?;
    Ok((reference, text))
}

/// One argument's text, and the file it came from.
pub(crate) struct Given {
    json: String,
    file: Option<String>,
    what: String,
}

impl Given {
    /// Plain judgment text has the same public question grammar as JSON;
    /// sets and specs still use `read` and never take this branch.
    pub(crate) fn read_question(
        arg: Option<&str>,
        key: &str,
        directory: Option<&str>,
    ) -> Result<Self, Error> {
        let text = arg.unwrap_or_default();
        if text.trim().is_empty() || text.starts_with('@') || text.trim_start().starts_with('{') {
            return Self::read(arg, "question", directory);
        }
        let verb = match key {
            "" => "decide",
            "options" => "choose",
            "levels" => "score",
            "labels" => "tag",
            _ => return Err(call::defect("unknown judgment members")),
        };
        let mut object = serde_json::Map::new();
        object.insert(verb.to_owned(), serde_json::Value::from(text));
        Ok(Self {
            json: serde_json::Value::Object(object).to_string(),
            file: None,
            what: "question".to_owned(),
        })
    }

    /// Read an argument: its JSON, or the named file's text.
    pub(crate) fn read(
        arg: Option<&str>,
        what: &str,
        directory: Option<&str>,
    ) -> Result<Self, Error> {
        let text = arg.unwrap_or_default();
        if text.trim().is_empty() {
            return Err(call::usage(format!("the {what} is empty")));
        }
        let (json, file) = match arg_form(text, what)? {
            ArgForm::File(path) => (read_named(what, path, directory)?, Some(path.to_owned())),
            ArgForm::Json(json) => (json.to_owned(), None),
        };
        let what = what.to_owned();
        Ok(Self { json, file, what })
    }

    /// Parse the text. A file's parse failure is `local` and names the file (0095).
    pub(crate) fn parse<T>(
        &self,
        parse: impl FnOnce(&str) -> Result<T, thinkthen::Error>,
    ) -> Result<T, Error> {
        let what = &self.what;
        parse(&self.json).map_err(|error| match &self.file {
            None => error,
            Some(path) => Error::new(
                ErrorKind::Local,
                format!(
                    "the {what} file '@{path}' does not parse: {}",
                    error.detail().message()
                ),
            ),
        })
    }

    /// Join a members array to the question's JSON object under `key`.
    pub(crate) fn with_members(
        mut self,
        key: &str,
        members: Option<Vec<String>>,
    ) -> Result<Self, Error> {
        let Some(members) = members else {
            return Ok(self);
        };
        if members.is_empty() {
            return Err(call::usage("the members array is empty"));
        }
        let value: serde_json::Value = serde_json::from_str(&self.json)
            .map_err(|_| call::usage("the question is not a JSON object"))?;
        let object = value
            .as_object()
            .ok_or_else(|| call::usage("the question is not a JSON object"))?;
        if object.contains_key(key) {
            return Err(call::usage(format!(
                "the question already names its {key}; pass NULL for the array"
            )));
        }
        // Preserve the source bytes and their member order. The final core
        // question parse must still see duplicate names in the original JSON.
        let source = self
            .json
            .trim_end()
            .strip_suffix('}')
            .ok_or_else(|| call::usage("the question is not a JSON object"))?;
        let separator = if object.is_empty() { "" } else { "," };
        let encoded = serde_json::to_string(&members)
            .map_err(|_| call::defect("members could not be written as JSON"))?;
        self.json = format!("{source}{separator}\"{key}\":{encoded}}}");
        Ok(self)
    }

    /// Append the shared parser's question fields without changing their member order.
    pub(crate) fn with_settings(mut self, settings: &Settings, verb: For) -> Result<Self, Error> {
        if settings == &Settings::default() {
            return Ok(self);
        }
        // Keep the original JSON bytes; the final parse retains duplicate names
        // and maps a named file's failure to local after settings are appended.
        let explicit: serde_json::Value = serde_json::from_str(&self.json)
            .map_err(|_| call::usage("the question is one JSON object"))?;
        let fields = explicit
            .as_object()
            .ok_or_else(|| call::usage("the question is one JSON object"))?;
        let keys: Vec<_> = fields.keys().map(String::as_str).collect();
        settings
            .conflicts(&keys, false)
            .map_err(|error| call::usage(error.to_string()))?;
        let extra = settings
            .question_json(verb, "settings merge")
            .map_err(|error| call::usage(error.to_string()))?;
        let Some(first) = extra.find(',') else {
            return Ok(self);
        };
        let suffix = extra
            .get(first + 1..extra.len() - 1)
            .ok_or_else(|| call::defect("settings lost their fields"))?;
        let source = self
            .json
            .trim_end()
            .strip_suffix('}')
            .ok_or_else(|| call::usage("the question is one JSON object"))?;
        let separator = if fields.is_empty() { "" } else { "," };
        self.json = format!("{source}{separator}{suffix}}}");
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// R2-4: bare text is never a path, and the refusal names the `@` form.
    #[test]
    fn a_bare_path_is_refused_and_the_at_form_passes() {
        assert_eq!(
            call::shown(arg_form("@refund.json", "question")),
            Ok(ArgForm::File("refund.json"))
        );
        assert_eq!(
            call::shown(arg_form(" {\"decide\": \"x?\"}", "question")),
            Ok(ArgForm::Json(" {\"decide\": \"x?\"}"))
        );
        assert_eq!(
            call::shown(arg_form("names.json", "recognize spec")),
            Err("thinkthen usage: a recognize spec file is named with the @ spelling: '@names.json'; bare text is never a path (retryable: no)".to_owned())
        );
    }

    pub(super) fn scratch(name: &str) -> PathBuf {
        let held = std::env::temp_dir().join(format!("thinkthen-pg-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&held);
        std::fs::create_dir_all(held.join("base")).expect("the base directory creates");
        std::fs::create_dir_all(held.join("outside")).expect("the outside directory creates");
        held
    }

    fn mkfifo(path: &Path) {
        let status = std::process::Command::new("mkfifo")
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .arg(path)
            .status()
            .expect("mkfifo runs");
        assert!(status.success());
    }

    /// R3-2 and R4-6: only regular files within the cap read; symlinks out
    /// of the directory, a fifo, and a device take the one refusal.
    #[test]
    fn a_checked_read_takes_only_regular_files_within_the_cap_and_confinement() {
        let held = scratch("checked");
        let base = held.join("base");
        let inside = base.join("inside.json");
        std::fs::write(&inside, b"{}").expect("the inside file writes");
        std::fs::write(held.join("outside/secret.json"), b"{}").expect("the secret writes");
        std::os::unix::fs::symlink(held.join("outside/secret.json"), base.join("link-out.json"))
            .expect("link");
        std::fs::create_dir_all(base.join("sub")).expect("sub");
        std::os::unix::fs::symlink(held.join("outside"), base.join("hop")).expect("hop");
        assert_eq!(read_within(&inside, 1024, Some(&base)).as_deref(), Ok("{}"));
        assert_eq!(
            read_within(Path::new("/dev/zero"), 1024, None),
            Err(CheckedReadError::Refused)
        );
        assert_eq!(
            read_within(&base.join("link-out.json"), 1024, Some(&base)),
            Err(CheckedReadError::Refused)
        );
        // An intermediate symlink that leaves the directory refuses.
        assert_eq!(
            read_within(&base.join("hop/secret.json"), 1024, Some(&base)),
            Err(CheckedReadError::Refused)
        );
        let fifo = base.join("pipe.json");
        mkfifo(&fifo);
        let started = std::time::Instant::now();
        assert_eq!(
            read_within(&fifo, 1024, Some(&base)),
            Err(CheckedReadError::Refused)
        );
        assert!(started.elapsed() < std::time::Duration::from_millis(50));
        std::fs::write(base.join("big.json"), vec![b'x'; 2048]).expect("the big file writes");
        assert_eq!(
            read_within(&base.join("big.json"), 1024, None),
            Err(CheckedReadError::OverCap)
        );
        let _ = std::fs::remove_dir_all(&held);
    }

    /// R5-15 and R5-16: an outside file over the cap gives the one
    /// confinement refusal, and a hard link inside to an outside file refuses.
    #[test]
    fn a_confined_read_refuses_outside_files_by_one_rule() {
        let held = scratch("confined");
        let base = held.join("base");
        std::fs::write(held.join("outside/big.json"), vec![b'x'; 2048]).expect("the big file");
        std::fs::write(held.join("outside/secret.json"), b"{}").expect("the secret");
        std::fs::hard_link(held.join("outside/secret.json"), base.join("linked.json"))
            .expect("the hard link");
        assert_eq!(
            read_within(&held.join("outside/big.json"), 1024, Some(&base)),
            Err(CheckedReadError::Refused)
        );
        assert_eq!(
            read_within(&base.join("linked.json"), 1024, Some(&base)),
            Err(CheckedReadError::Refused)
        );
        assert_eq!(
            read_within(&base.join("linked.json"), 1024, None).as_deref(),
            Ok("{}")
        );
        let _ = std::fs::remove_dir_all(&held);
    }

    /// R6-8: an outside path refuses before any open, so a writer blocked
    /// on an outside fifo stays blocked through the confined read.
    #[test]
    fn an_outside_path_refuses_before_any_open() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};
        let held = scratch("unopened");
        let base = held.join("base");
        let fifo = held.join("pipe.json");
        mkfifo(&fifo);
        let opened = Arc::new(AtomicBool::new(false));
        let writer = {
            let (fifo, opened) = (fifo.clone(), Arc::clone(&opened));
            std::thread::spawn(move || {
                let _held = std::fs::OpenOptions::new().write(true).open(&fifo);
                opened.store(true, Ordering::SeqCst);
            })
        };
        std::thread::sleep(std::time::Duration::from_millis(100));
        for path in [
            fifo.clone(),
            base.join("../pipe.json"),
            base.join("./../pipe.json"),
        ] {
            assert_eq!(
                read_within(&path, 1024, Some(&base)),
                Err(CheckedReadError::Refused)
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
        let woke = opened.load(Ordering::SeqCst);
        let _reader = ffi::open_plain(&fifo);
        writer.join().expect("the writer joins");
        let _ = std::fs::remove_dir_all(&held);
        assert!(!woke, "a confined read opened a path outside its directory");
    }

    /// 0370: a relative name resolves inside the folder for every role, and
    /// the confined read still refuses each way out of it.
    #[test]
    fn a_relative_name_resolves_inside_the_directory() {
        use CheckedReadError::Refused;
        let held = scratch("relative");
        let base = held.join("base");
        std::fs::write(base.join("refund.json"), b"{}").expect("the inside file writes");
        let secret = held.join("outside/secret.json");
        std::fs::write(&secret, b"{}").expect("the secret writes");
        std::os::unix::fs::symlink(&secret, base.join("link-out.json")).expect("link");
        std::os::unix::fs::symlink(held.join("outside"), base.join("hop")).expect("hop");
        let folder = base.to_str().expect("the folder is text").to_owned();
        let inside = format!("{folder}/refund.json");
        let outside = secret.to_str().expect("the secret is text").to_owned();
        let table: [(&str, Result<&str, &CheckedReadError>); 6] = [
            ("refund.json", Ok("{}")),
            ("../outside/secret.json", Err(&Refused)),
            ("link-out.json", Err(&Refused)),
            ("hop/secret.json", Err(&Refused)),
            (&inside, Ok("{}")),
            (&outside, Err(&Refused)),
        ];
        for (name, want) in table {
            let got = read_within(&resolve(name, Some(&folder)), 1024, Some(&base));
            assert_eq!(got.as_deref(), want, "{name}");
        }
        // With no folder, a relative name keeps the backend's working folder.
        assert_eq!(resolve("refund.json", None), PathBuf::from("refund.json"));
        assert_eq!(resolve(&outside, Some(&folder)), secret);
        let _ = std::fs::remove_dir_all(&held);
    }
}

#[cfg(test)]
#[path = "files_complete_tests.rs"]
mod complete_tests;
