use super::Failure;

#[derive(Debug)]
#[rustfmt::skip]
pub(crate) enum Error { Config { file: bool, error: crate::core::RelateConfigError }, Logical }

#[rustfmt::skip]
pub(super) fn message(failure: &Failure) -> Option<(u8, String)> { Some(match failure { Failure::Relate(Error::Config { file, error }) => (if *file && !matches!(error, crate::core::RelateConfigError::WrongVerb) { 5 } else { 2 }, error.to_string()), Failure::Relate(Error::Logical) => (4, "the backend returned no usable relation answer".to_owned()), _ => return None }) }
