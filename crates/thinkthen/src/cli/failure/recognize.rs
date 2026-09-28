//! Fixed recognition diagnostics.

use super::Failure;

#[derive(Debug)]
pub(crate) enum Error {
    Config {
        file: bool,
        error: crate::core::RecognizeConfigError,
    },
    LogicalQuestion,
    /// A `--kind` entry holds no `=`.
    KindWithoutSign,
    /// The text passed `--max-text-bytes`. The message names sizes alone.
    TextTooLong {
        bytes: usize,
        limit: usize,
    },
    RelationLimit(String),
}

pub(super) fn message(failure: &Failure) -> Option<(u8, String)> {
    Some(match failure {
        Failure::Recognize(Error::Config { file, error }) => {
            (if *file { 5 } else { 2 }, error.to_string())
        }
        Failure::Recognize(Error::LogicalQuestion) => (
            4,
            "the backend failed one required recognition question".to_owned(),
        ),
        Failure::Recognize(Error::KindWithoutSign) => (
            2,
            "--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind"
                .to_owned(),
        ),
        Failure::Recognize(Error::TextTooLong { bytes, limit }) => (
            2,
            format!(
                "recognize: the text is {bytes} bytes, over the limit of {limit}; raise it with --max-text-bytes"
            ),
        ),
        Failure::Recognize(Error::RelationLimit(message)) => (2, format!("recognize: {message}")),
        _ => return None,
    })
}
