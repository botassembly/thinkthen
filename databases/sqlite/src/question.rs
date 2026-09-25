//! Arguments: text read from SQL, questions and question sets parsed once,
//! and the `'@name'` file door.

use std::collections::{HashMap, VecDeque};
use std::io::Read as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::sync::{Arc, LazyLock, Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::types::ValueRef;
use thinkthen::{ErrorKind, LoadedQuestion, Question, QuestionSet};

use crate::Failure;

/// The most parses each cache holds, oldest out first.
const CACHE_CAP: usize = 4096;

/// The largest file `'@name'` reads, in bytes.
const FILE_CAP: u64 = 1024 * 1024;

/// A named file's modified time and size, read from the descriptor its bytes came through.
type Stamp = (SystemTime, u64);

/// How SQL gave a value, for a message that names it.
pub(crate) fn shown(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => "NULL".to_owned(),
        ValueRef::Integer(whole) => whole.to_string(),
        ValueRef::Real(real) => format!("{real:?}"),
        ValueRef::Text(bytes) => format!("'{}'", String::from_utf8_lossy(bytes)),
        ValueRef::Blob(_) => "a BLOB".to_owned(),
    }
}

/// Text from one SQL argument: `None` for NULL, `usage` for a BLOB, a
/// number, a NUL byte, or bytes that are not UTF-8 (G11).
pub(crate) fn text(value: ValueRef<'_>, what: &str) -> Result<Option<String>, Failure> {
    let bytes = match value {
        ValueRef::Null => return Ok(None),
        ValueRef::Text(bytes) => bytes,
        ValueRef::Blob(_) => return Err(Failure::usage(format!("{what} is a BLOB; pass text"))),
        ValueRef::Integer(_) | ValueRef::Real(_) => {
            return Err(Failure::usage(format!("{what} is a number; pass text")));
        }
    };
    if bytes.contains(&0) {
        return Err(Failure::usage(format!("{what} holds a NUL byte")));
    }
    String::from_utf8(bytes.to_vec())
        .map(Some)
        .map_err(|_| Failure::usage(format!("{what} is not UTF-8")))
}

/// Read the file `'@name'` names: one `O_NONBLOCK` open, a regular file of
/// at most 1 MiB, symlinks followed, and one sentence for every cause.
pub(crate) fn named_file(argument: &str) -> Result<(String, Stamp), Failure> {
    let refused = || {
        Failure::of(
            ErrorKind::Local,
            format!(
                "the question file '{argument}' did not read: it must be a regular file at most {FILE_CAP} bytes"
            ),
        )
    };
    let path = argument.strip_prefix('@').ok_or_else(refused)?;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| refused())?;
    let meta = file.metadata().map_err(|_| refused())?;
    if !meta.is_file() || meta.len() > FILE_CAP {
        return Err(refused());
    }
    let mut held = String::new();
    file.take(FILE_CAP + 1)
        .read_to_string(&mut held)
        .map_err(|_| refused())?;
    if held.len() as u64 > FILE_CAP {
        return Err(refused());
    }
    Ok((held, (meta.modified().unwrap_or(UNIX_EPOCH), meta.len())))
}

/// Whether a cached parse still stands: an inline argument always does, and
/// a named file does while its stamp matches the file on disk.
fn fresh(argument: &str, stamp: Option<Stamp>) -> bool {
    let Some(path) = argument.strip_prefix('@') else {
        return true;
    };
    let now = std::fs::metadata(path)
        .ok()
        .and_then(|meta| Some((meta.modified().ok()?, meta.len())));
    stamp.is_some() && now == stamp
}

/// One held parse and, for a named file, its stamp.
type Entry<T> = (Arc<T>, Option<Stamp>);

/// Parses by argument text, bounded, oldest out first.
#[derive(Debug)]
pub(crate) struct Parsed<T> {
    map: HashMap<String, Entry<T>>,
    order: VecDeque<String>,
}

impl<T> Parsed<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn get(&self, key: &str) -> Option<(Arc<T>, Option<Stamp>)> {
        self.map
            .get(key)
            .map(|(held, stamp)| (Arc::clone(held), *stamp))
    }

    fn insert(&mut self, key: String, held: Arc<T>, stamp: Option<Stamp>) {
        if self.map.insert(key.clone(), (held, stamp)).is_none() {
            self.order.push_back(key);
        }
        while self.order.len() > CACHE_CAP {
            if let Some(oldest) = self.order.pop_front() {
                self.map.remove(&oldest);
            }
        }
    }
}

/// Look up one argument, or parse it and hold the parse. A stat runs with the lock released.
fn cached<T>(
    cache: &Mutex<Parsed<T>>,
    argument: &str,
    parse: impl FnOnce(&str, bool) -> Result<T, Failure>,
) -> Result<Arc<T>, Failure> {
    let held = cache
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get(argument);
    if let Some((held, stamp)) = held
        && fresh(argument, stamp)
    {
        return Ok(held);
    }
    let (source, stamp) = if argument.starts_with('@') {
        let (source, stamp) = named_file(argument)?;
        (source, Some(stamp))
    } else {
        (argument.to_owned(), None)
    };
    let held = Arc::new(parse(&source, stamp.is_some())?);
    cache.lock().unwrap_or_else(PoisonError::into_inner).insert(
        argument.to_owned(),
        Arc::clone(&held),
        stamp,
    );
    Ok(held)
}

/// A broken rule in a file is `local`; in an argument it stays `usage` (0095, Q16).
fn from_file(error: thinkthen::Error, file: bool) -> Failure {
    let mut failure = Failure::from(error);
    if file && failure.kind == ErrorKind::Usage {
        failure.kind = ErrorKind::Local;
    }
    failure
}

static QUESTIONS: LazyLock<Mutex<Parsed<LoadedQuestion>>> =
    LazyLock::new(|| Mutex::new(Parsed::new()));
static SETS: LazyLock<Mutex<Parsed<QuestionSet>>> = LazyLock::new(|| Mutex::new(Parsed::new()));

/// The question one argument names: `'@name'`, JSON that starts with `{`, or
/// plain text asked as a decide question at the default cut.
pub(crate) fn question(argument: &str) -> Result<Arc<LoadedQuestion>, Failure> {
    cached(&QUESTIONS, argument, |source, file| {
        if file || source.starts_with('{') {
            Question::from_json(source).map_err(|error| from_file(error, file))
        } else {
            Ok(LoadedQuestion::Question(Question::decide(source)?.cut()))
        }
    })
}

/// The question set one argument names, inline JSON or `'@name'`.
pub(crate) fn set(argument: &str) -> Result<Arc<QuestionSet>, Failure> {
    cached(&SETS, argument, |source, file| {
        QuestionSet::from_json(source).map_err(|error| from_file(error, file))
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{CACHE_CAP, Parsed};

    /// R4-17 recognize half: the question cache holds its bound, newest kept.
    #[test]
    fn the_parse_cache_keeps_its_bound() {
        let mut cache = Parsed::new();
        for at in 0..5_000 {
            cache.insert(format!("question {at}"), Arc::new(at), None);
        }
        assert_eq!((cache.map.len(), cache.order.len()), (CACHE_CAP, CACHE_CAP));
        assert!(cache.get("question 4999").is_some());
        assert!(cache.get("question 903").is_none());
    }
}
