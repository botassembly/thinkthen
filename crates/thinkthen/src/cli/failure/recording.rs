//! Fixed local recording diagnostics.

use super::Failure;

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
        Failure::RecordingBackendMismatch => (
            5,
            "the recording folder belongs to another backend interface or address; \
             restore its backend settings or choose another folder"
                .to_owned(),
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
