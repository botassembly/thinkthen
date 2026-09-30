//! The one capped reader for question files, question sets and plan files.
//!
//! The command, the public loaders and every library read a question file
//! through [`read_question_file`] and map its reason to their own sentence.

use std::fs::File;
use std::io::{self, Read as _};
use std::path::Path;

use super::Error;

/// The most bytes a question file may hold: 1 MiB.
const LIMIT: u64 = 1_048_576;

/// Why a question file gave no text.
#[derive(Debug)]
pub enum QuestionFileError {
    /// The file could not be opened or read.
    Unreadable(io::Error),
    /// The file holds more than 1 MiB.
    TooLarge,
    /// The file's bytes are not UTF-8.
    NotUtf8,
}

/// Read a question file's text, reading at most one byte past 1 MiB, so a file
/// such as `/dev/zero` is refused before memory is spent on it.
///
/// # Errors
///
/// Returns the [`QuestionFileError`] that says why no text was read.
pub fn read_question_file(path: impl AsRef<Path>) -> Result<String, QuestionFileError> {
    let mut bytes = Vec::new();
    File::open(path)
        .and_then(|file| file.take(LIMIT + 1).read_to_end(&mut bytes))
        .map_err(QuestionFileError::Unreadable)?;
    if bytes.len() as u64 > LIMIT {
        return Err(QuestionFileError::TooLarge);
    }
    String::from_utf8(bytes).map_err(|_| QuestionFileError::NotUtf8)
}

/// The public loaders' reading: `the {role} is too large` over the cap, and
/// `the {role} could not be read` otherwise, both local.
pub(crate) fn load_text(path: &Path, role: &str) -> Result<String, Error> {
    read_question_file(path).map_err(|reason| match reason {
        QuestionFileError::TooLarge => Error::local(format!("the {role} is too large")),
        QuestionFileError::Unreadable(_) | QuestionFileError::NotUtf8 => {
            Error::local(format!("the {role} could not be read"))
        }
    })
}
