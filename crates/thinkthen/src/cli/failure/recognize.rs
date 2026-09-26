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
        _ => return None,
    })
}
