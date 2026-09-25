//! Fixed local recording diagnostics.

use super::Failure;
use crate::core::RecordError;

impl Failure {
    /// Name invalid text by its framing while preserving every other record error.
    pub(crate) fn record(error: RecordError, streamed: bool) -> Self {
        match error {
            RecordError::NotUtf8 => Self::InvalidUtf8 { record: streamed },
            other => Self::Record(other),
        }
    }
}

pub(super) fn message(failure: &Failure) -> Option<(u8, String)> {
    Some(match failure {
        Failure::ReplayMiss(name) => (
            5,
            format!(
                "the replay folder holds no entry named `{name}`; \
                 the entry name covers the backend interface, address, and request"
            ),
        ),
        Failure::Entry(name, why) => (5, format!("the entry `{name}` was refused: {why}")),
        Failure::RecordingConflict(name) => (
            5,
            format!("the entry `{name}` already records a different response"),
        ),
        Failure::RecordingStorage => (
            5,
            "the recording folder could not be read or written; check its permissions and free space"
                .to_owned(),
        ),
        Failure::RecordingPathIsFile => (
            5,
            "the recording directory is a file; choose another path or remove the file".to_owned(),
        ),
        Failure::RecordingBackendMismatch(url, true) => (
            5,
            format!(
                "the default cache is bound to a backend address other than `{url}`; \
                 go back to that address, use --no-cache, or set THINKTHEN_CACHE to another folder"
            ),
        ),
        Failure::RecordingBackendMismatch(url, false) => (
            5,
            format!(
                "the recording folder is bound to a backend address other than `{url}`; \
                 restore its backend settings or choose another folder"
            ),
        ),
        Failure::RecordingFolderLegacy => (
            5,
            "the recording folder predates backend binding; \
             replay it read-only or choose a new folder"
                .to_owned(),
        ),
        _ => return None,
    })
}
