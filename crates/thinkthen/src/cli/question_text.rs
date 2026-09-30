//! The text of a question file or question set, read under the libraries' cap.
//!
//! The command reads at most one byte past the cap, so a file such as
//! `/dev/zero` is refused before memory or a request is spent on it.

use std::fs::File;
use std::io::{self, Read as _};
use std::path::Path;

use crate::cli::failure::Failure;

/// The most bytes a question file or question set may hold: 1 MiB, the cap the
/// libraries' question-file loaders apply.
const LIMIT: u64 = 1_048_576;

/// Read the named file's text, or fail with `unopened` when it cannot be read
/// and with the too-large refusal when it holds more than [`LIMIT`] bytes.
pub(crate) fn read(path: &Path, unopened: fn(io::Error) -> Failure) -> Result<String, Failure> {
    let mut bytes = Vec::new();
    File::open(path)
        .and_then(|file| file.take(LIMIT + 1).read_to_end(&mut bytes))
        .map_err(unopened)?;
    if bytes.len() as u64 > LIMIT {
        return Err(Failure::QuestionFileTooLarge);
    }
    // The same error `fs::read_to_string` gives, so that message is unchanged.
    String::from_utf8(bytes).map_err(|_| {
        unopened(io::Error::new(
            io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8",
        ))
    })
}
