//! Yield a terminal error once without pulling another input.
pub(crate) fn next<T, E>(
    stopped: &mut bool,
    next: impl FnOnce() -> Option<Result<T, E>>,
) -> Option<Result<T, E>> {
    if *stopped {
        return None;
    }
    let result = next()?;
    *stopped = result.is_err();
    Some(result)
}

pub(crate) fn until_error<T, E>(
    mut pull: impl FnMut() -> Option<Result<T, E>>,
) -> impl Iterator<Item = Result<T, E>> {
    let mut stopped = false;
    std::iter::from_fn(move || next(&mut stopped, &mut pull))
}
