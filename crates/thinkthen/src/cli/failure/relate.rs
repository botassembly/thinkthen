use super::Failure;
use crate::core::{EntitySetError, RelateConfigError};

/// What stopped `relate` after its shared checks passed.
#[derive(Debug)]
pub(crate) enum Error {
    /// The inline rules or the question file are refused.
    Config {
        file: bool,
        error: RelateConfigError,
    },
    /// The complete entity set is refused before any request.
    Entities(EntitySetError),
    /// The command saw the complete oversized set; the public core error remains count-free.
    TooMany { count: usize },
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
        Failure::Relate(Error::Entities(error)) => Some((2, error.to_string())),
        Failure::Relate(Error::TooMany { count }) => {
            let count = *count as u128;
            let possible = count * (count - 1) / 2;
            Some((
                2,
                format!(
                    "relate takes at most 255 entities; this set has {count}. If all {count} were distinct, an unordered all-kind rule would have {possible} candidate pairs; split the set or narrow by kind"
                ),
            ))
        }
        _ => None,
    }
}
