//! Question, set, and spec arguments: JSON text or a file named `'@path'`,
//! read under the privilege gate, the 1 MiB cap, the regular-file rule, and
//! confinement beneath `thinkthen.file_directory` (decision 8).

use std::path::{Component, Path, PathBuf};

use thinkthen::ErrorKind;

use crate::call::Refusal;
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
pub(crate) fn arg_form<'a>(text: &'a str, what: &str) -> Result<ArgForm<'a>, Refusal> {
    if let Some(path) = text.strip_prefix('@') {
        return Ok(ArgForm::File(path));
    }
    if text.trim_start().starts_with('{') {
        return Ok(ArgForm::Json(text));
    }
    Err(Refusal::usage(format!(
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

/// Read a file a caller named with `@`. A role without
/// `pg_read_server_files` reads only inside `thinkthen.file_directory`.
fn read_named(what: &str, path: &str, directory: Option<&str>) -> Result<String, Refusal> {
    let confined = if ffi::may_read_files() {
        None
    } else {
        match directory.filter(|held| !held.trim().is_empty()) {
            Some(held) => Some(PathBuf::from(held)),
            None => {
                return Err(Refusal::usage(
                    "a named file needs pg_read_server_files, or an administrator's thinkthen.file_directory",
                ));
            }
        }
    };
    read_within(Path::new(path), FILE_CAP, confined.as_deref()).map_err(|error| match error {
        CheckedReadError::Refused => Refusal::of(
            ErrorKind::Local,
            format!(
                "the {what} file '@{path}' did not read: it must be a regular file at most {FILE_CAP} bytes, \
                 and inside thinkthen.file_directory with one link when one is set"
            ),
        ),
        CheckedReadError::OverCap => {
            Refusal::of(ErrorKind::Local, format!("the {what} file '@{path}' is over the {FILE_CAP} byte cap"))
        }
    })
}

/// One argument's text, and the file it came from.
pub(crate) struct Given {
    json: String,
    file: Option<String>,
    what: String,
}

impl Given {
    /// Read an argument: its JSON, or the named file's text.
    pub(crate) fn read(
        arg: Option<&str>,
        what: &str,
        directory: Option<&str>,
    ) -> Result<Self, Refusal> {
        let text = arg.unwrap_or_default();
        if text.trim().is_empty() {
            return Err(Refusal::usage(format!("the {what} is empty")));
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
    ) -> Result<T, Refusal> {
        let what = &self.what;
        parse(&self.json).map_err(|error| match &self.file {
            None => error.into(),
            Some(path) => Refusal::of(
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
    ) -> Result<Self, Refusal> {
        let Some(members) = members else {
            return Ok(self);
        };
        if members.is_empty() {
            return Err(Refusal::usage("the members array is empty"));
        }
        let mut value: serde_json::Value = serde_json::from_str(&self.json)
            .map_err(|_| Refusal::usage("the question is not a JSON object"))?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| Refusal::usage("the question is not a JSON object"))?;
        if object.contains_key(key) {
            return Err(Refusal::usage(format!(
                "the question already names its {key}; pass NULL for the array"
            )));
        }
        object.insert(key.to_owned(), serde_json::Value::from(members));
        self.json = value.to_string();
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
            arg_form("@refund.json", "question"),
            Ok(ArgForm::File("refund.json"))
        );
        assert_eq!(
            arg_form(" {\"decide\": \"x?\"}", "question"),
            Ok(ArgForm::Json(" {\"decide\": \"x?\"}"))
        );
        assert_eq!(
            arg_form("names.json", "recognize spec"),
            Err(Refusal::usage(
                "a recognize spec file is named with the @ spelling: '@names.json'; bare text is never a path"
            ))
        );
    }

    fn scratch(name: &str) -> PathBuf {
        let held = std::env::temp_dir().join(format!("thinkthen-pg-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&held);
        std::fs::create_dir_all(held.join("base")).expect("the base directory creates");
        std::fs::create_dir_all(held.join("outside")).expect("the outside directory creates");
        held
    }

    fn mkfifo(path: &Path) {
        let status = std::process::Command::new("mkfifo")
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
}
