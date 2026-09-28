//! One command's selected replacement trust roots.

use crate::engine::facade::{Roots, RootsError};
use crate::failure::Failure;

use super::Environment;

impl Environment {
    /// Read selected roots before the transport key reader or any send.
    pub(crate) fn roots(&self) -> Result<Option<Roots>, Failure> {
        self.ca_bundle
            .as_deref()
            .map(Roots::load)
            .transpose()
            .map_err(|error| match error {
                RootsError::Usage(message) => Failure::Usage(message),
                RootsError::Local(message) => Failure::Configuration(message),
            })
    }
}
