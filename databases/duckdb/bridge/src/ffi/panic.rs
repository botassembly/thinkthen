//! The C++ bridge's panic guard: the engine's one shared guard.

/// Run `call`; a panic in it returns `Err` with no trace of its payload.
pub(super) fn caught<T>(call: impl FnOnce() -> T) -> Result<T, ()> {
    thinkthen::contained(call).ok_or(())
}
