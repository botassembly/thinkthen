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

/// Resolve a path already stripped of its explicit @, through the native loader.
pub(crate) fn reference(
    path: &Path,
    unopened: fn(io::Error) -> Failure,
) -> Result<String, Failure> {
    let Some(value) = path.to_str() else { return read(path, unopened) };
    let reference = crate::QuestionFileReference::reference(&format!("@{value}"))
        .map_err(|error| Failure::NamedQuestion(error.detail().message().to_owned()))?;
    let text = selected_text(&reference, unopened)?;
    reference.parse(&text, crate::QuestionRole::Set, |text| Ok(text.to_owned()))
        .map_err(Failure::from)
}

/// Read the selected spelling once while retaining the native named-root check.
/// The caller then admits these exact bytes with the selected native file role.
pub(crate) fn selected_text(
    reference: &crate::QuestionFileReference,
    unopened: fn(io::Error) -> Failure,
) -> Result<String, Failure> {
    let Some(root) = reference.named_root() else { return read(reference.path(), unopened) };
    let unavailable = || Failure::NamedQuestion("the named question is unavailable".to_owned());
    let target = reference.path().canonicalize().map_err(|_| unavailable())?;
    if !target.starts_with(root) || !target.is_file() { return Err(unavailable()); }
    read(&target, unopened).map_err(|_| unavailable())
}
