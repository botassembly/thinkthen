//! Bounds admitted with a backend's selected model.

use std::num::NonZeroUsize;

/// Backend bounds used by functions and request packing.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Limits {
    /// Maximum encoded request bytes.
    pub(crate) request_bytes: NonZeroUsize,
    /// Maximum questions per request, or no additional cap.
    pub(crate) questions: Option<NonZeroUsize>,
    /// Maximum options per choice, including a none option.
    pub(crate) options: Option<NonZeroUsize>,
}
