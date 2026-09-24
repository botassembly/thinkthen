use super::Failure;
use crate::core::RelateConfigError;

/// What stopped `relate` after its shared checks passed.
#[derive(Debug)]
pub(crate) enum Error {
    /// The inline rules or the question file are refused.
    Config {
        file: bool,
        error: RelateConfigError,
    },
    /// No logical relation question has a usable answer.
    Logical,
}

pub(super) fn message(failure: &Failure) -> Option<(u8, String)> {
    match failure {
        Failure::Relate(Error::Config { file, error }) => {
            let code = if *file && *error != RelateConfigError::WrongVerb {
                5
            } else {
                2
            };
            Some((code, error.to_string()))
        }
        Failure::Relate(Error::Logical) => Some((
            4,
            "the backend returned no usable relation answer".to_owned(),
        )),
        _ => None,
    }
}
