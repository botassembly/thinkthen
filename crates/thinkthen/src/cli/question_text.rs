//! The text of a question file or question set, read under the libraries' cap.
//!
//! The command reads through the crate's one capped reader, so a file such as
//! `/dev/zero` is refused before memory or a request is spent on it.

use std::io;
use std::path::Path;

use crate::cli::failure::Failure;
use crate::{QuestionFileError, read_question_file};

/// Read the named file's text, or fail with `unopened` when it cannot be read
/// and with the too-large refusal when it holds more than 1 MiB.
pub(crate) fn read(path: &Path, unopened: fn(io::Error) -> Failure) -> Result<String, Failure> {
    read_question_file(path).map_err(|reason| match reason {
        QuestionFileError::Unreadable(error) => unopened(error),
        QuestionFileError::TooLarge => Failure::QuestionFileTooLarge,
        // The same error `fs::read_to_string` gives, so that message is unchanged.
        QuestionFileError::NotUtf8 => unopened(io::Error::new(
            io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8",
        )),
    })
}
